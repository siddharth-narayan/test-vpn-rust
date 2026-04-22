use std::{net::TcpListener, thread};
use serde::{Serialize, Deserialize};
use openssl::ssl::{HandshakeError, Ssl, SslAcceptor, SslFiletype, SslMethod, SslStream};

use crate::{
    network::{hub::Hub, openssl::{BufferedSsl, create_server_ctx}},
    protocols::openvpn::protcol::{OpenVPNConnection, connection_thread},
};

#[allow(dead_code)]
pub fn openvpn_main_thread(hub: Hub) {
    let listener = TcpListener::bind("0.0.0.0:443").unwrap();
    let mut ssl_acceptor_builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls_server()).unwrap();
    ssl_acceptor_builder.set_certificate_file("cert.pem", SslFiletype::PEM);
    ssl_acceptor_builder.set_private_key_file("key.pem", SslFiletype::PEM);

    let ssl_acceptor = ssl_acceptor_builder.build();

    loop {
        let tcp_stream = match listener.accept() {
            Ok(x) => {
                // x.0.set_nonblocking(true).unwrap_or_else(|_| {
                //     println!("Failed to set nonblocking");
                // });

                x.0
            }
            Err(_e) => {
                println!("TCP Accept error");
                continue;
            }
        };

        let handshake = ssl_acceptor.accept(tcp_stream);
        let mut ssl_stream = match handshake {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        ssl_stream.get_ref().set_nonblocking(true);

        let sender_clone = hub.tx().clone();
        let nat_clone = hub.table();

        ssl_stream.accept();
        let connection = OpenVPNConnection::new(BufferedSsl::new(ssl_stream));
        thread::spawn(move || connection_thread(connection, sender_clone, nat_clone));
    }
}
