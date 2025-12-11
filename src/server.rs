// use std::{future, pin::Pin};

// use openssl::ssl::{Ssl, SslAcceptor, SslMethod};
// use tokio::{io::{AsyncWriteExt, split}};
// use tokio_openssl::SslStream;

// use crate::protocols::openvpn::server;

// mod network;
// mod protocols;

// #[tokio::main]
fn main() {
    let sync_listener = std::net::TcpListener::bind("0.0.0.0:8080").unwrap();
    
    let (sync_stream, _) = sync_listener.accept().unwrap();

    let sync_acceptor = openssl::ssl::SslAcceptor::mozilla_intermediate(openssl::ssl::SslMethod::tls_server()).unwrap().build();
    let handshake_result = sync_acceptor.accept(sync_stream);
    if handshake_result.is_err() {
        eprintln!("{}", handshake_result.unwrap_err());
    }
}

    // println!("Finished first handshake");

    // let async_listener = tokio::net::TcpListener::bind("0.0.0.0:444").await.unwrap();

    // let (async_stream, _) = async_listener.accept().await.unwrap();

    // let ctx = SslAcceptor::mozilla_modern_v5(SslMethod::tls_server()).unwrap().build().into_context();
    // let ssl = Ssl::new(&ctx).unwrap();
    // let (stream, client_addr) = async_listener.accept().await.unwrap();

    // let mut ssl_stream = SslStream::new(ssl, async_stream).unwrap();
    
    // let handshake_result = Pin::new(&mut ssl_stream).accept().await;
    
    // if handshake_result.is_err() {
    //     eprintln!("{}", handshake_result.unwrap_err());
    //     let _ = ssl_stream.shutdown().await;
    // }

    // println!("abcd");
    // let (ssl_read, ssl_write) = split(ssl_stream);

// #[tokio::main]
// async fn main() {
//     tokio::spawn(server::main());

//     // Parse stuff commandline stuff here later
//     future::pending::<()>().await;
// }
