use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use pnet::packet::{FromPacket, Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocol, ipv4::Ipv4Packet, tcp::TcpPacket};

pub enum PacketLayer {
    L2,
    L3,
}

pub struct BasePacket {
    pub layer: PacketLayer,
    pub proto: IpNextHeaderProtocol,
    pub src: SocketAddr, 
    pub dst: SocketAddr,
    pub payload: Box<[u8]>
}

impl Into<Box<[u8]>> for Box<BasePacket> {
    fn into(self) -> Box<[u8]> {
        todo!()
    }
}

pub enum BasePacketError {
    NotEnoughBytes,
    ParseError
}

impl TryFrom<Box<[u8]>> for Box<BasePacket> {
    type Error = BasePacketError;
    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        if value.len() < 1 {
            return Err(BasePacketError::NotEnoughBytes)
        }

        // EthernetPacket::new()

        // match value[]
        let ip_packet = match Ipv4Packet::new(value.as_ref()) {
            Some(p) => p,
            None => {
                println!("Failed to construct IPv4 packet");
                return Err(BasePacketError::ParseError)
            }
        };

        // TcpPacket::new(ip_packet.packet());

        println!("{:?}", value);

        Err(BasePacketError::ParseError)
        // Ok(Box::new(BasePacket {
        //     layer: PacketLayer::L2,

        //     src: IpAddr::V4(Ipv4Addr::LOCALHOST),
        //     dst: IpAddr::V4(Ipv4Addr::LOCALHOST),
        // }))
    }
}