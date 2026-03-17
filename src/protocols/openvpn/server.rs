use std::{net::TcpListener, sync::mpsc::{self, Sender}, thread};

use openssl::ssl::{Ssl, SslStream};

use crate::{network::{hub::Hub, nat::NatTable, openssl::create_server_ctx, packet::BasePacket}, protocols::openvpn::protcol::client_thread};


pub fn openvpn_main_thread(hub: Hub) {
    let ctx = create_server_ctx().unwrap();

    let listener = TcpListener::bind("0.0.0.0:443").unwrap();

    loop {
        let (tcp_stream, addr) = match listener.accept() {
            Ok(x) => x,
            Err(e) => {
                continue
            }
        };

        match tcp_stream.set_nonblocking(true) {
            Err(_) => {
                println!("oops, failed to set nonblocking");
                continue;
            }
            _ => ()
        }
        let ssl = Ssl::new(&ctx).unwrap();
        let ssl_stream = SslStream::new(ssl, tcp_stream).unwrap();

        let sender_clone = hub.tx()
        .clone();
        let nat_clone = hub.table();

        thread::spawn(move || {
            client_thread(ssl_stream, sender_clone, nat_clone)
        });
    }
    
}
