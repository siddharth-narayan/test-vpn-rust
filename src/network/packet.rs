use std::net::{IpAddr, SocketAddr};

use pnet::packet::ip::IpNextHeaderProtocol;

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

impl TryFrom<Box<[u8]>> for Box<BasePacket> {
    type Error = ();
    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        todo!()
    }
}