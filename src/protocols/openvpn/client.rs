use std::{net::TcpStream, sync::mpsc::Sender, thread};

use openssl::ssl::{Ssl, SslStream};

use crate::{network::{hub::Hub, openssl::create_client_ctx, packet::BasePacket}, protocols::openvpn::protcol::client_thread};

pub fn openvpn_main_thread(hub: Hub) {
    let ctx = create_client_ctx().unwrap();

    let tcp_stream = TcpStream::connect("127.0.0.1:443").unwrap();
    match tcp_stream.set_nonblocking(true) {
        Err(_) => {
            println!("oops, failed to set nonblocking");
            return;
        }
        _ => (),
    }
    
    let ssl = Ssl::new(&ctx).unwrap();
    let ssl_stream = SslStream::new(ssl, tcp_stream).unwrap();

    let sender_clone = hub.tx();
    let nat_clone = hub.table();

    thread::spawn(move || client_thread(ssl_stream, sender_clone, nat_clone));
}
