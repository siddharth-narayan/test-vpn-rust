use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use std::{
    io::{Read, Write},
    net::TcpStream, process::exit,
};

fn client_handshake(stream: &mut SslStream<TcpStream>) -> String{
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer);

    String::from_utf8(buffer.to_vec()).unwrap_or("72.100.100.100".to_string())
}

fn main() {
    let tcp_stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server.");
    let mut ssl_connector =
        SslConnector::builder(SslMethod::tls()).expect("Failed to create SSLConnector");
    ssl_connector.set_verify(SslVerifyMode::NONE);
    let ssl_connector = ssl_connector.build();

    let mut ssl_stream = ssl_connector
        .connect("127.0.0.1", tcp_stream)
        .expect("Failed to establish a TLS connection with the server");


    let mut tun_config = tun::Configuration::default();
        tun_config.address("72.100.100.100");
        // tun_config.layer(tun::Layer::L2);
        tun_config.up();

    let mut device;
    match tun::create(&tun_config) {
        Ok(d) => device = d,
        Err(e) => {eprintln!("Failed to create device: {e}"); exit(-1)}
    }


    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        let len = device.read(&mut buffer);
        if len.is_err() {
            break;
        }

        if buffer[0] >> 4 != 4 {
            continue;
        }

        _ = ssl_stream.write(&buffer);
    }

    match ssl_stream.shutdown() {
        Ok(..) => {
            _ = ssl_stream.shutdown();
        }
        Err(e) => eprintln!("Failed to shutdown TLS connection {}", e),
    }
}
