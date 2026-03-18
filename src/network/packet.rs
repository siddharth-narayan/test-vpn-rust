use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use pnet::{
    packet::{
        FromPacket, Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocol, ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket
    },
    util::MacAddr,
};

pub type Layer = tun::Layer;

#[derive(PartialEq)]
pub enum PacketType {
    Ethernet,
    IPv4,
    IPv6,
}

#[derive(Eq, Hash, PartialEq)]
pub enum Address {
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
    MAC(MacAddr),
}

pub struct BasePacket {
    pub p_type: PacketType,
    pub payload: Box<[u8]>,
}

impl BasePacket {
    pub fn get_address(&self) -> Option<Address> {
        let address = match self.p_type {
            PacketType::Ethernet => {
                if let Some(packet) = EthernetPacket::new(self.payload.as_ref()) {
                    Address::MAC(packet.get_destination())
                } else {
                    return None;
                }
            }

            PacketType::IPv4 => {
                if let Some(packet) = Ipv4Packet::new(self.payload.as_ref()) {
                    Address::IPv4(packet.get_destination())
                } else {
                    return None;
                }
            }

            PacketType::IPv6 => {
                if let Some(packet) = Ipv6Packet::new(self.payload.as_ref()) {
                    Address::IPv6(packet.get_destination())
                } else {
                    return None;
                }
            }
        };

        Some(address)
    }
}

impl Into<Box<[u8]>> for Box<BasePacket> {
    fn into(self) -> Box<[u8]> {
        todo!()
    }
}

pub enum BasePacketError {
    NotEnoughBytes,
    ParseError,
}

impl TryFrom<Box<[u8]>> for Box<BasePacket> {
    type Error = BasePacketError;
    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        if value.len() < 1 {
            return Err(BasePacketError::NotEnoughBytes);
        }

        let ip_packet = match Ipv4Packet::new(value.as_ref()) {
            Some(p) => p,
            None => {
                println!("Failed to construct IPv4 packet");
                return Err(BasePacketError::ParseError);
            }
        };

        println!("{:?}", ip_packet);

        Err(BasePacketError::ParseError)
    }
}
