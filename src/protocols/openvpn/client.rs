use openssl::ssl::{Ssl, SslContext};
use pnet::packet::ipv4::MutableIpv4Packet;
use std::net::SocketAddr;
use std::pin::Pin;
use std::process::exit;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{ReadHalf, WriteHalf, split};
use tokio::net::TcpStream;
use tokio_openssl::SslStream;
use tun::AsyncDevice;

use crate::network::device;
use crate::network::openssl::{create_client_ctx, create_server_ctx};

pub async fn main() {
    println!("afafa");
    let ctx = create_client_ctx().unwrap();
    if let Ok((mut ssl_read, mut ssl_write)) =
        client_connect(&ctx, "127.0.0.1:8080".parse().unwrap()).await
    {
        let device = device::get_default_tun();
        let (tun_read, tun_write) = split(device);

        tokio::spawn(client_send_stream(tun_read, ssl_write));
        tokio::spawn(client_recv_stream(ssl_read, tun_write));
    } else {
        println!("Failed to start connection to server");
    }
}

pub async fn client_connect(
    ctx: &SslContext,
    addr: SocketAddr,
) -> Result<
    (
        ReadHalf<SslStream<TcpStream>>,
        WriteHalf<SslStream<TcpStream>>,
    ),
    std::io::Error,
> {
    let ssl = Ssl::new(ctx)?;

    let tcp_stream = TcpStream::connect(addr).await?;

    // ssl.connect(tcp_stream)?;
    let mut ssl_stream = SslStream::new(ssl, tcp_stream)?;
    
    let handshake_result = Pin::new(&mut ssl_stream).connect().await;

    if handshake_result.is_err() {
        eprintln!("{}", handshake_result.unwrap_err());
        let _ = ssl_stream.shutdown().await;
        exit(0);
    }

    return Ok(split(ssl_stream));
}

pub async fn client_send_stream(
    mut tun_read: ReadHalf<AsyncDevice>,
    mut ssl_write: WriteHalf<SslStream<TcpStream>>,
) {
    loop {
        let mut buffer: Vec<u8> = Vec::new();
        let result = tun_read.read_buf(&mut buffer).await;

        let len = match result {
            Ok(0) => {println!("Send 0"); return;},
            Ok(n) => n,
            Err(_) =>  {println!("Send Err"); return;},
        };

        println!("Sent {:?}", &buffer[..len]);

        _ = ssl_write.write(&buffer).await;
    }
}

pub async fn client_recv_stream(
    mut ssl_read: ReadHalf<SslStream<TcpStream>>,
    mut tun_write: WriteHalf<AsyncDevice>,
) {
    loop {
        let mut buffer: Vec<u8> = Vec::new();
        let result = ssl_read.read_buf(&mut buffer).await;

        let len = match result {
            Ok(0) => {println!("Recv 0"); return;},
            Ok(n) => n,
            Err(_) =>  {println!("Recv Err"); return;},
        };

        println!("Received {:?}", &buffer.as_slice()[0..len]);
        

        // println!("Received {:?}", &buffer[..len]);
        // _ = tun_write.write(&buffer);
    }
}