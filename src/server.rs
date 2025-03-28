use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use pnet::{
    datalink::{self, interfaces},
    packet::{ip::IpNextHeaderProtocol, ipv4::Ipv4Packet},
    transport::{self, TransportChannelType},
};

// fn client_handshake(stream: &mut SslStream<TcpStream>) -> Details {
//     let mut buffer: [u8; 1024] = [0; 1024];
//     stream.read(&mut buffer);
// }

fn handle_packet(buf: &[u8], len: usize) {
    let packet = Ipv4Packet::new(buf).unwrap();

    println!("Sending IPv4 Packet: {:?}", packet);
}

async fn handle_client(mut stream: SslStream<TcpStream>) {
    let mut buffer = [0; 1024];

    let interfaces = interfaces();
    println!("{:?}", interfaces);
    let default_interface = interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    let a = datalink::channel(default_interface.unwrap(), Default::default());
    println!("Handling client");
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                _ = stream.shutdown();
            }
            Ok(len) => handle_packet(&buffer, len),
            Err(e) => eprintln!("Failed to read from connection: {}", e),
        }
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

    loop {
        let stream = listener.accept();
        match stream {
            Ok(stream) => {
                let ssl_stream = acceptor.accept(stream.0).unwrap();
                tokio::spawn(handle_client(ssl_stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

// async fn other() {}

// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             // In a loop, read data from the socket and write the data back.
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     // socket closed
//                     Ok(0) => return,
//                     Ok(n) => n,
//                     Err(e) => {
//                         eprintln!("failed to read from socket; err = {:?}", e);
//                         return;
//                     }
//                 };

//                 // Write the data back
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }
//             }
//         });
//     }
// }
