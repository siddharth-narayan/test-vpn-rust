use std::net::IpAddr;

pub enum PacketLayer {
    L2,
    L3,
}

pub struct BasePacket {
    pub layer: PacketLayer,
    pub src: IpAddr,
    pub dst: IpAddr,
    pub payload: Box<[u8]>
}