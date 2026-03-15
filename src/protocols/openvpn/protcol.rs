use std::{io::{Read, Write}, sync::{mpsc::{Receiver, Sender}}};

use openssl::ssl::{ErrorCode, SslStream};
use pnet::packet::ipv4::Ipv4Packet;

use crate::{
    network::packet::{BasePacket}, protocols::openvpn::packet::{OpenVPNPacket}
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ProtocolState {
    Unconnected,
    InHandshake,
    Connected,
    Errored,
}

pub struct OpenVPNConnection<T: Read + Write> {
    pub status: ProtocolState,
    pub session_id: u64,
    pub sent_bytes: u64,
    pub recv_bytes: u64,
    stream: SslStream<T>,

    ackd_packets: [u8; 16],
}

impl<T: Read + Write> OpenVPNConnection<T> {
    pub fn new(s: SslStream<T>) -> Self {
        Self {
            status: ProtocolState::Unconnected,
            session_id: 0, // Todo?
            sent_bytes: 0,
            recv_bytes: 0,
            stream: s,
            ackd_packets: [0; 16],
        }
    }

    pub fn send_packet(&mut self, packet: Box<BasePacket>) {
        // self.stream.ssl_write();
    }

    pub fn try_recv_packet(&mut self) -> Result<Box<BasePacket>, OpenVPNPacketRecvError> {
        let mut buf = [0u8; 256];
        
        match self.stream.ssl_read(&mut buf) {
            Err(e) => {
                if e.code() == ErrorCode::WANT_READ {
                    return Err(OpenVPNPacketRecvError::OpenSSLNoData)
                } else {
                    return Err(OpenVPNPacketRecvError::OpenSSLRealErr)
                }
            }
            _ => ()
        };

        let packet = match Ipv4Packet::new(&buf) {
            Some(p) => p,
            None => return Err(OpenVPNPacketRecvError::Ipv4PacketConstructionErr)
        };

        let openvpn_packet = OpenVPNPacket::try_from(packet);
        
        // let packet = BasePacket {
        //     layer: PacketLayer::L2,
        //     src: 
        //     payload: Box::new(buf)
        // };

        Ok(Box::new(openvpn_packet.unwrap().into()))
    }

    // to_openvpn_packet()
}

pub enum OpenVPNPacketRecvError {
    OpenSSLNoData,
    OpenSSLRealErr,
    Ipv4PacketConstructionErr
}

// Do it all in a single thread!
pub fn client_thread<T: Read + Write>(stream: SslStream<T>, tx: Sender<Box<BasePacket>>) {
    let mut connection = OpenVPNConnection::new(stream);

    loop {
        // Self -> Peer
        match connection.try_recv_packet() {
            Ok(p) => {
                tx.send(p);
            },
            Err(e) => {
                match e {
                    OpenVPNPacketRecvError::OpenSSLRealErr => todo!(),
                    _ => ()
                };
            }
        }

        // Peer -> Self


        
    }
}

// pub fn send_packet<T>(stream: &mut SslStream<T>, packet: Box<BasePacket>) {
//     // stream.send()
// }