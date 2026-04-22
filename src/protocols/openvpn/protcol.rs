// #[allow(unused)]

use std::{
    io::{Cursor, BufReader, Read, Write},
    sync::mpsc::{self, Sender, TryRecvError},
};

use binrw::BinRead;
use openssl::ssl::{ErrorCode, SslStream};
use pnet::packet::{Packet, ipv4::Ipv4Packet};

use crate::{
    network::{hub::HubClientTable, openssl::BufferedSsl, packet::BasePacket},
    protocols::openvpn::packet::OpenVPNPacket,
};

pub enum ProcolMode {
    TLS,
    UDP,
}

pub enum OpenVPNPacketRecvError {
    OpenSSLNoData,
    OpenSSLRealErr,
    PacketConstructionErr,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ProtocolState {
    Unconnected,
    InHandshake,
    Connected,
    Errored,
}

pub struct OpenVPNConnection<T: Read + Write> {
    pub connection_mode: ProcolMode,
    pub status: ProtocolState,
    pub session_id: u64,
    pub sent_bytes: u64,
    pub recv_bytes: u64,
    stream: BufferedSsl<T>,

    ackd_packets: [u8; 16],
}

impl<T: Read + Write> OpenVPNConnection<T> {
    pub fn new(s: BufferedSsl<T>) -> Self {
        Self {
            connection_mode: ProcolMode::TLS,
            status: ProtocolState::Unconnected,
            session_id: 0, // Todo?
            sent_bytes: 0,
            recv_bytes: 0,
            stream: s,
            ackd_packets: [0; 16],
        }
    }

    pub fn try_recv_packet(&mut self) -> Result<Box<BasePacket>, OpenVPNPacketRecvError> {
        let mut buf = Vec::new();
        self.stream.read_to_end(&mut buf);

        let mut reader  = BufReader::new(Cursor::new(buf));
        let openvpn_packet = match OpenVPNPacket::read(&mut reader) {
            Ok(p) => p,
            Err(_) => {
                return Err(OpenVPNPacketRecvError::PacketConstructionErr);
            }
        };

        todo!()
    }

    pub fn send_packet(&mut self, packet: Box<BasePacket>) {
        // let openvpn_packet = OpenVPNPacket::try_from(packet).unwrap();
        // let bytes = Into::<Box<[u8]>>::into(openvpn_packet);
        // self.stream.ssl_write(bytes.as_ref());
    }

    // to_openvpn_packet()
}

// Do it all in a single thread!
pub fn connection_thread<T: Read + Write>(
    mut connection: OpenVPNConnection<T>,
    self_tx: Sender<Box<BasePacket>>,
    mut nat: HubClientTable,
) {
    // Each thread has its own tx/rx pair for _receiving_ base packets (after a NAT entry is matched)
    let (hub_tx, self_rx) = mpsc::channel::<Box<BasePacket>>();

    loop {
        // Peer -> Self
        match connection.try_recv_packet() {
            Ok(p) => {
                nat.insert(&p, hub_tx.clone());
                let _ = self_tx.send(p);
            }
            Err(e) => {
                match e {
                    OpenVPNPacketRecvError::OpenSSLRealErr => {
                        println!("openssl real error");
                    },
                    _ => (),
                };
            }
        }

        // Peer -> Self
        match self_rx.try_recv() {
            Ok(p) => {
                connection.send_packet(p);
            }
            Err(e) => match e {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => {
                    panic!()
                }
            },
        }
    }
}
