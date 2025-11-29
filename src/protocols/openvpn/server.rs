use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    io::Read,
    iter::Map,
    net::{Ipv4Addr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use pnet::{
    datalink::{self, interfaces},
    packet::{
        Packet,
        ip::{IpNextHeaderProtocol, IpNextHeaderProtocols},
        ipv4::{Ipv4, Ipv4Packet, MutableIpv4Packet},
    },
    transport::{self, TransportChannelType},
};

struct ServerState {
    nat_table: Mutex<HashMap<NatEntry, Ipv4Addr>>,
    client_connection_map: Mutex<HashMap<Ipv4Addr, Arc<Mutex<SslStream<TcpStream>>>>>,
}

impl ServerState {
    fn new() -> ServerState {
        ServerState {
            nat_table: Mutex::new(HashMap::<NatEntry, Ipv4Addr>::new()),
            client_connection_map: Mutex::new(
                HashMap::<Ipv4Addr, Arc<Mutex<SslStream<TcpStream>>>>::new(),
            ),
        }
    }

    fn add_nat_entry(&self, packet: &Ipv4Packet) {
        let mut nat_table = self.nat_table.lock().unwrap();
        let mut new_entry = NatEntry {
            src: packet.get_source(),
            dst: packet.get_destination(),
            proto: packet.get_next_level_protocol(),
            port: None,
        };

        match new_entry.proto {
            IpNextHeaderProtocols::Tcp => {
                new_entry.port = Some(
                    pnet::packet::tcp::TcpPacket::new(packet.packet())
                        .unwrap()
                        .get_source(),
                )
            }
            IpNextHeaderProtocols::Udp => {
                new_entry.port = Some(
                    pnet::packet::udp::UdpPacket::new(packet.packet())
                        .unwrap()
                        .get_source(),
                )
            }
            _ => {}
        }

        nat_table.insert(new_entry, packet.get_source());
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct NatEntry {
    src: Ipv4Addr,
    dst: Ipv4Addr,
    proto: IpNextHeaderProtocol,
    port: Option<u16>,
}

fn packet_send(server_state: Arc<ServerState>, buf: &mut [u8], len: usize) {
    if buf[0] >> 4 != 4 {
        return;
    }

    let mut packet = MutableIpv4Packet::new(buf).unwrap();
    server_state.add_nat_entry(&packet.to_immutable());
    println!("Sending IPv4 Packet: {:?}", packet);

    packet.set_source("192.168.1.197".parse().unwrap());
    unsafe { forward_packet_ipv4(buf.as_mut_ptr(), len as u32) };
}

async fn handle_client(
    mut stream: SslStream<TcpStream>,
    server_state: Arc<ServerState>
) {
    let mut buffer = [0; 1024];

    // let interfaces = interfaces();
    // println!("{:?}", interfaces);
    // let default_interface = interfaces
    //     .iter()
    //     .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    println!("Handling client");
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                _ = stream.shutdown();
            }
            Ok(len) => packet_send(server_state.clone(), &mut buffer, len),
            Err(e) => eprintln!("Failed to read from connection: {}", e),
        }
    }
}

async fn reciever(server_state: Arc<ServerState>) {
    let mut socket: i32 = -1;
    tokio::task::block_in_place(|| unsafe { socket = ip_socket() });
    println!("after c");

    println!("Reciever attenpwting to recieve");

    loop {
        let mut buf: [u8; 4096] = [0; 4096];
        let bytes: i32;

        (unsafe { bytes = socket_read(socket, buf.as_mut_ptr(), 4096) });

        if bytes == -1 {
            continue;
        }
        println!("Recieved {bytes} bytes from somewhere!!: {:?}", buf);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    acceptor.set_certificate_chain_file("cert.pem").unwrap();
    acceptor.check_private_key().unwrap();

    let acceptor = acceptor.build();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080");
    let server_state = Arc::new(ServerState::new());
    tokio::spawn(reciever(server_state.clone()));
    loop {
        let stream = listener.accept();
        match stream {
            Ok(stream) => {
                let ssl_stream = acceptor.accept(stream.0).unwrap();
                tokio::spawn(handle_client(ssl_stream, server_state.clone()));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
