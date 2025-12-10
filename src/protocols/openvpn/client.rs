use openssl::ssl::{Ssl, SslContext};
use std::net::SocketAddr;
use tokio::io::split;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_openssl::SslStream;
use tun::AsyncDevice;

use crate::network::openssl::create_ctx;
use crate::{network::device, protocols::openvpn::config::ClientCli};

pub async fn start_client(config: ClientCli) {
    let ctx = create_ctx().unwrap();
    if let Ok((ssl_read, ssl_write)) = client_connect(&ctx, "localhost:7000".parse().unwrap()).await
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
        tokio::io::ReadHalf<SslStream<TcpStream>>,
        tokio::io::WriteHalf<SslStream<TcpStream>>,
    ),
    std::io::Error,
> {
    let ssl = Ssl::new(ctx)?;

    let tcp_stream = TcpStream::connect(addr).await?;

    // ssl.connect(tcp_stream)?;
    let ssl_stream = SslStream::new(ssl, tcp_stream)?;

    return Ok(split(ssl_stream));
}

pub async fn client_send_stream(
    mut tun_read: tokio::io::ReadHalf<AsyncDevice>,
    mut ssl_write: tokio::io::WriteHalf<SslStream<TcpStream>>,
) {
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        let result = tun_read.read(&mut buffer).await;

        if result.is_err() {
            continue;
        }

        if buffer[0] >> 4 != 4 {
            continue; // Not IPv4
        }

        _ = ssl_write.write(&buffer);
    }
}

pub async fn client_recv_stream(
    mut ssl_read: tokio::io::ReadHalf<SslStream<TcpStream>>,
    mut tun_write: tokio::io::WriteHalf<AsyncDevice>,
) {
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        let result = ssl_read.read(&mut buffer).await;

        if result.is_err() {
            continue;
        }

        if buffer[0] >> 4 != 4 {
            continue; // Not IPv4
        }

        _ = tun_write.write(&buffer);
    }
}