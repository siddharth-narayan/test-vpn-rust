pub type Layer = tun::Layer;

pub enum TransportProtocol {
    TLS,
    UDP,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PacketProtocol {
    Ethernet,
    IPv4,
    IPv6
}