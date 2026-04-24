// #[allow(unused)]

use std::{
    io::{self, BufReader, Cursor, ErrorKind, Read, Write},
    sync::mpsc::{self, Sender, TryRecvError},
};

use binrw::{BinRead, BinWrite};

use crate::{
    network::{hub::HubClientTable, openssl::BufferedSsl, packet::BasePacket, util::{Layer, PacketProtocol, TransportProtocol}},
    protocols::openvpn::packet::{DataPacket, GenericPacket, OpenVPNPacket},
};

pub enum OpenVPNPacketRecvError {
    OpenSSLNoData,
    OpenSSLRealErr(std::io::Error),
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
    pub connection_mode: TransportProtocol,
    pub layer: Layer,
    pub status: ProtocolState,
    pub session_id: u64,
    pub sent_bytes: u64,
    pub recv_bytes: u64,
    stream: BufferedSsl<T>,

    ackd_packets: [u8; 16],
}

impl<T: Read + Write> OpenVPNConnection<T> {
    pub fn new(s: BufferedSsl<T>, layer: Layer) -> Self {
        Self {
            connection_mode: TransportProtocol::TLS,
            layer: layer,
            status: ProtocolState::Unconnected,
            session_id: 0, // Todo?
            sent_bytes: 0,
            recv_bytes: 0,
            stream: s,
            ackd_packets: [0; 16],
        }
    }

    pub fn try_recv_packet(&mut self) -> Result<BasePacket, OpenVPNPacketRecvError> {
        let mut buf = [0u8; 512];

        if let Err(x) = self.stream.read(&mut buf) {
            if x.kind() == ErrorKind::WouldBlock {
                return Err(OpenVPNPacketRecvError::OpenSSLNoData)
            } else {
                return Err(OpenVPNPacketRecvError::OpenSSLRealErr(x))
            }
        };

        let mut reader  = BufReader::new(Cursor::new(buf));
        let openvpn_packet = match OpenVPNPacket::read(&mut reader) {
            Ok(p) => p,
            Err(e) => {
                return Err(OpenVPNPacketRecvError::PacketConstructionErr);
            }
        };

        // BasePacket::new(self.layer, openvpn_packet);

        todo!()
    }

    pub fn send_packet(&mut self, packet: BasePacket) {
        let inner  = DataPacket::new();
        OpenVPNPacket::new(MessageType::P_DATA_V2, packet.raw_ref())
    }

    // to_openvpn_packet()
}

// Do it all in a single thread!
pub fn connection_thread<T: Read + Write>(
    mut connection: OpenVPNConnection<T>,
    self_tx: Sender<BasePacket>,
    mut nat: HubClientTable,
) {
    // Each thread has its own tx/rx pair for _receiving_ base packets (after a NAT entry is matched)
    let (hub_tx, self_rx) = mpsc::channel::<BasePacket>();

    loop {
        // Peer -> Self
        match connection.try_recv_packet() {
            Ok(p) => {
                nat.insert(&p, hub_tx.clone());
                let _ = self_tx.send(p);
            }
            Err(e) => {
                match e {
                    OpenVPNPacketRecvError::OpenSSLRealErr(e) => {
                        println!("openssl real error: {}", e);
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
