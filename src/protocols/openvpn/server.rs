use std::{net::TcpListener, sync::mpsc, thread};

use openssl::ssl::{Ssl, SslStream};

use crate::{network::openssl::create_server_ctx, protocols::{openvpn::protcol::{client_thread}, util::ProtocolThreadInfo}};

pub fn openvpn_main_thread(thread_info: ProtocolThreadInfo) {
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

        let sender = thread_info.packet_write_stream
        .clone();

        thread::spawn(move || {
            client_thread(ssl_stream, sender)
        });
    }
    
}
