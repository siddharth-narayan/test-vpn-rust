use std::{
    collections::HashMap,
    net::{SocketAddr, SocketAddrV4},
    sync::{Arc, Mutex, mpsc::Sender},
};

use pnet::packet::{
    Packet,
    ip::{IpNextHeaderProtocol, IpNextHeaderProtocols},
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
};

use crate::network::packet::{BasePacket, PacketType};

// pub fn build_nat_entry(mut packet: &Ipv4Packet) -> Option<NatEntry> {
//     let dest_ip = p.get_source();

//     let protocol = p.get_next_level_protocol();

//     if protocol != IpNextHeaderProtocols::Tcp || protocol != IpNextHeaderProtocols::Udp {
//         return None;
//     }

//     // Flipped because on an incoming packet the order will be reversed compared to when
//     // the entry was put into the NAT table
//     let (dest_port, source_port) = match protocol {
//         IpNextHeaderProtocols::Tcp => {
//             let packet = TcpPacket::new(&mut buffer)?;
//             (packet.get_source(), packet.get_destination())
//         }
//         IpNextHeaderProtocols::Udp => {
//             let packet = UdpPacket::new(&mut buffer)?;
//             (packet.get_source(), packet.get_destination())
//         }
//         _ => return None,
//     };

//     let entry = NatEntry {
//         proto: protocol,
//         source_port: source_port,
//         dest: std::net::SocketAddr::V4(SocketAddrV4::new(dest_ip, dest_port)),
//     };

//     Some(entry)
// }

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

impl TryFrom<&Box<BasePacket>> for NatEntry {
    type Error = NatEntryError;
    fn try_from(p: &Box<BasePacket>) -> Result<Self, Self::Error> {
        if p.p_type != PacketType::IPv4 {
            return Err(NatEntryError::NotIPv4);
        }

        let packet = match Ipv4Packet::new(&p.payload) {
            Some(p) => p,
            None => return Err(NatEntryError::BufferSize),
        };

        // Flipped because on an incoming packet the order will be reversed compared to when
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
            _ => {
                return Err(NatEntryError::NotTCPorUDP)
            },
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
    table: Arc<Mutex<HashMap<NatEntry, Sender<Box<BasePacket>>>>>,
}

impl NatTable {
    pub fn new() -> Self {
        Self {
            table: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&mut self, e: NatEntry, sender: Sender<Box<BasePacket>>) {}

    pub fn lookup(&self, e: NatEntry) -> Option<Sender<Box<BasePacket>>> {
        let guard = self.table.lock().unwrap();
        guard.get(&e).cloned()
    }
}
