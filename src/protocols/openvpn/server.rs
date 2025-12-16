use crate::{network::{device::get_default_tun, openssl::{SslRead, SslWrite, create_server_ctx}}, protocols::{fsm::FSM, openvpn::{packet::{MessageType, OpenVPNPacket, process_packet}, protcol::{self, OpenVPNState}}, util::{TunRead, TunWrite}}};
use std::{
    collections::linked_list,
    future,
    io::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    pin::Pin,
};

use dashmap::DashMap;
use openssl::ssl::{Ssl, SslContext};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf, split},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_openssl::SslStream;
use tun::AsyncDevice;

struct ClientConnectionState { 
    ssl_write: Arc<Mutex<SslWrite>>, 
    fsm: FSM<OpenVPNState, MessageType, OpenVPNPacket>,
}

type ClientTable = DashMap<SocketAddr, ClientConnectionState>;

// Only one send stream (one thread to read TUN packets)
async fn server_send_stream(mut tun_read: TunRead, table: Arc<ClientTable>) {
    loop {
        let mut buffer: Vec<u8> = Vec::new();
        let result = tun_read.read_buf(&mut buffer).await;

        let len = match result {
            Ok(0) => {println!("Send 0"); return;},
            Ok(n) => n,
            Err(_) =>  {println!("Send Err"); return;},
        };

        let client_addr = SocketAddrV4::from("127.0.0.1:3000".parse().unwrap());
        
        process_packet(buffer.clone());

        println!("Sent {:?}", &buffer[..len]);

        // Proceess packet here, get IP and port from higher level TCP/UDP
        if let Some(client_state) = table.get_mut(&SocketAddr::from(client_addr)) {
            _ = client_state.ssl_write.lock().await.write(&buffer);
        }
    }
}

// One recv stream for each client
async fn server_recv_stream(mut ssl_read: SslRead, tun_write: TunWrite) {
    loop {
        let mut buffer: Vec<u8> = Vec::new();

        let result = ssl_read.read_buf(&mut buffer).await;

        let len = match result {
            Ok(0) => {println!("Recv 0"); return;},
            Ok(n) => n,
            Err(_) =>  {println!("Recv Err"); return;},
        };

        println!("Received {:?}", &buffer[..len]);

        // let mut tun_write = tun_write.lock().await;

        // Don't enable writing yet
        // _ = tun_write.write(&buffer);
    }
}

// Accepts incoming client connections, then spawns the necessary threads
async fn acceptor(
    ctx: SslContext,
    state: Arc<ClientTable>,
    tun_write: TunWrite, // For adding to recv stream for each client
) -> Result<(SslRead, SslWrite), Error> {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    loop {
        let ssl = Ssl::new(&ctx)?;
        let (stream, client_addr) = listener.accept().await.unwrap();
        let mut ssl_stream = SslStream::new(ssl, stream)?;
        let handshake_result = Pin::new(&mut ssl_stream).accept().await;
        if handshake_result.is_err() {
            eprintln!("{}", handshake_result.unwrap_err());
            let _ = ssl_stream.shutdown().await;
            continue;
        }

        println!("Accepted new client");
        let (ssl_read, ssl_write) = split(ssl_stream);

        let ssl_write = Arc::new(Mutex::new(ssl_write));
        let fsm = protcol::build_server_fsm(ssl_write.clone());
        let client_state = ClientConnectionState { ssl_write, fsm };
        
        state.insert(client_addr, client_state);
        tokio::spawn(server_recv_stream(ssl_read, tun_write.clone()));
    }
}

pub async fn main() {
    let client_table = ClientTable::new();

    let client_table: Arc<ClientTable> = Arc::new(client_table);

    let tun = get_default_tun();

    let (tun_read, tun_write) = split(tun);
    let tun_write: TunWrite = Arc::from(Mutex::from(tun_write));

    let ctx = create_server_ctx();
    let ctx = ctx.unwrap();

    tokio::spawn(acceptor(ctx, client_table.clone(), tun_write));
    tokio::spawn(server_send_stream(tun_read, client_table.clone()));
}
