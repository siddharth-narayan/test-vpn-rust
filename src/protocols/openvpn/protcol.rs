use std::sync::Arc;

use pnet::packet::ipv4::Ipv4Packet;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_openssl::SslStream;

use crate::{network::openssl::SslWrite, protocols::{
    fsm::{FSM, TransitionTable},
    openvpn::packet::{MessageType, OpenVPNPacket},
}};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ProtocolState {
    Unconnected,
    InHandshake,
    Connected,
    Errored,
}

pub fn build_openvpn_packet(ip_packet: Ipv4Packet) {
    let mut total_size = 0;

    total_size += ip_packet.get_total_length()
}