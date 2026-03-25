use std::{net::TcpStream, sync::mpsc::Sender, thread};

use openssl::ssl::{Ssl, SslConnector, SslMethod, SslStream, SslVerifyMode};

use crate::{
    network::{hub::Hub, openssl::create_client_ctx, packet::BasePacket},
    protocols::openvpn::protcol::client_thread,
};

#[allow(dead_code)]
pub fn openvpn_main_thread(hub: Hub) {
    let tcp_stream = TcpStream::connect("127.0.0.1:443").unwrap();

    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls_client()).unwrap();
    ssl_connector_builder.set_verify(SslVerifyMode::NONE);
    let ssl_connector = ssl_connector_builder.build();

    let ssl_stream = ssl_connector.connect("", tcp_stream).unwrap();

    let sender_clone = hub.tx();
    let nat_clone = hub.table();

    thread::spawn(move || client_thread(ssl_stream, sender_clone, nat_clone));
}
