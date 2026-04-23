use std::{net::TcpStream, thread};

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};

use crate::{
    network::{hub::Hub, openssl::BufferedSsl, util::Layer},
    protocols::openvpn::protcol::{OpenVPNConnection, connection_thread},
};

#[allow(dead_code)]
pub fn openvpn_main_thread(hub: Hub) {
    let tcp_stream = TcpStream::connect("127.0.0.1:443").unwrap();

    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls_client()).unwrap();
    ssl_connector_builder.set_verify(SslVerifyMode::NONE);
    let ssl_connector = ssl_connector_builder.build();

    let ssl_stream = ssl_connector.connect("127.0.0.1", tcp_stream).unwrap();

    let sender_clone = hub.tx();
    let nat_clone = hub.table();

    let connection = OpenVPNConnection::new(BufferedSsl::new(ssl_stream), Layer::L2);
    thread::spawn(move || connection_thread(connection, sender_clone, nat_clone));
}
