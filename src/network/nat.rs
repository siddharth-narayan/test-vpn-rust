use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex, mpsc::Sender},
};

use pnet::packet::{
    Packet, ip::{IpNextHeaderProtocol, IpNextHeaderProtocols}, ipv4::Ipv4Packet, tcp::TcpPacket, udp::UdpPacket
};

use crate::network::{packet::{BasePacket, PacketRepr}, util::PacketProtocol};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NatEntry {
    pub proto: IpNextHeaderProtocol,
    pub source_port: u16,
    pub dest: SocketAddr,
}

pub enum NatEntryError {
    NotIPv4,
    // The underlying buffer is not long enough
    BufferSize,
    NotTCPorUDP,
}

impl TryFrom<&BasePacket> for NatEntry {
    type Error = NatEntryError;
    fn try_from(p: &BasePacket) -> Result<Self, Self::Error> {
        if p.proto() != PacketProtocol::IPv4 {
            return Err(NatEntryError::NotIPv4);
        };

        let packet = Ipv4Packet::new(p.raw_ref()).unwrap();

        // Flipped because on an incoming packet the order will be reversed comparaw to when
        // the entry was put into the NAT table
        let proto = packet.get_next_level_protocol();
        let (dest_port, source_port) = match proto {
            IpNextHeaderProtocols::Tcp => {
                if let Some(packet) = TcpPacket::new(packet.packet()) {
                    (packet.get_source(), packet.get_destination())
                } else {
                    return Err(NatEntryError::BufferSize);
                }
            }

            IpNextHeaderProtocols::Udp => {
                if let Some(packet) = UdpPacket::new(packet.packet()) {
                    (packet.get_source(), packet.get_destination())
                } else {
                    return Err(NatEntryError::BufferSize);
                }
            }
            _ => return Err(NatEntryError::NotTCPorUDP),
        };

        Ok(NatEntry {
            proto: proto,
            source_port: source_port,
            dest: (packet.get_destination(), dest_port).into(),
        })
    }
}

#[derive(Clone)]
pub struct NatTable {
    table: Arc<Mutex<HashMap<NatEntry, Sender<BasePacket>>>>,
}

impl NatTable {
    pub fn new() -> Self {
        Self {
            table: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&mut self, e: NatEntry, sender: Sender<BasePacket>) {}

    pub fn lookup(&self, e: NatEntry) -> Option<Sender<BasePacket>> {
        let guard = self.table.lock().unwrap();
        guard.get(&e).cloned()
    }
}
