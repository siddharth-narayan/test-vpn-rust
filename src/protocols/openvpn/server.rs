use crate::network::{device::get_default_tun, openssl::create_server_ctx};
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

type ClientTable = Arc<DashMap<SocketAddr, WriteHalf<SslStream<TcpStream>>>>;
type SslRead = ReadHalf<SslStream<TcpStream>>;
type SslWrite = WriteHalf<SslStream<TcpStream>>;
type TunReadDevice = ReadHalf<AsyncDevice>;
type TunWriteDevice = Arc<Mutex<WriteHalf<AsyncDevice>>>; // Wrap with Arc<Mutex<>> so that multiple server recv threads can write to it

// Only one send stream (one thread to read TUN packets)
async fn server_send_stream(mut tun_read: TunReadDevice, table: ClientTable) {
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        let result = tun_read.read(&mut buffer).await;

        let len = match result {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("Sending {:?}", &buffer[..len]);

        let client_addr = SocketAddrV4::from("127.0.0.1:3000".parse().unwrap());

        // Proceess packet here, get IP and port from higher level TCP/UDP
        if let Some(mut ssl_write) = table.get_mut(&SocketAddr::from(client_addr)) {
            _ = ssl_write.write(&buffer);
        }
    }
}

// One recv stream for each client
async fn server_recv_stream(mut ssl_read: SslRead, tun_write: TunWriteDevice) {
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        println!("recvc");
        let result = ssl_read.read(&mut buffer).await;

        let len = match result {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        println!("Received {:?}", &buffer[..len]);

        // let mut tun_write = tun_write.lock().await;

        // Don't enable writing yet
        // _ = tun_write.write(&buffer);
    }
}

// Accepts incoming client connections
async fn acceptor(
    ctx: SslContext,
    table: ClientTable,
    tun_write: TunWriteDevice, // For adding to recv stream for each client
) -> Result<(SslRead, SslWrite), Error> {
    let listener = TcpListener::bind("0.0.0.0:443").await.unwrap();

    loop {
        let ssl = Ssl::new(&ctx)?;
        let (stream, client_addr) = listener.accept().await.unwrap();
        println!("accepted");
        let mut ssl_stream = SslStream::new(ssl, stream)?;
        println!("before handshake");
        let handshake_result = Pin::new(&mut ssl_stream).accept().await;
        println!("aafter handshake");
        if handshake_result.is_err() {
            eprintln!("{}", handshake_result.unwrap_err());
            let _ = ssl_stream.shutdown().await;
            continue;
        }

        println!("abcd");
        let (ssl_read, ssl_write) = split(ssl_stream);

        table.insert(client_addr, ssl_write);
        tokio::spawn(server_recv_stream(ssl_read, tun_write.clone()));
    }
}

pub async fn main() {
    let client_table = DashMap::<SocketAddr, WriteHalf<SslStream<TcpStream>>>::new();
    let client_table: ClientTable = Arc::new(client_table);

    let tun = get_default_tun();

    let (tun_read, tun_write) = split(tun);
    let tun_write: TunWriteDevice = Arc::from(Mutex::from(tun_write));

    let ctx = create_server_ctx();
    let ctx = ctx.unwrap();

    tokio::spawn(acceptor(ctx, client_table.clone(), tun_write));
    tokio::spawn(server_send_stream(tun_read, client_table.clone()));
}
