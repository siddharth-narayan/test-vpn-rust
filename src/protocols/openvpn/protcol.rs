use std::sync::Arc;

use pnet::packet::ipv4::Ipv4Packet;
use tokio::{
    net::TcpStream,
    sync::{Mutex, RwLock},
};
use tokio_openssl::SslStream;

use crate::{
    network::openssl::SslWrite,
    protocols::{
        fsm::{FSM, TransitionTable},
        openvpn::packet::{MessageType, OpenVPNPacket},
    },
};

pub struct ClientState {
    pub state: ProtocolState,
    pub session_id: u64,
    pub sent_bytes: u64,
    pub recv_bytes: u64,
}

impl ClientState {
    pub fn new(id: u64) -> Self {
        ClientState {
            state: ProtocolState::Unconnected,
            session_id: id,
            sent_bytes: 0,
            recv_bytes: 0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ProtocolState {
    Unconnected,
    InHandshake,
    Connected,
    Errored,
}

pub async fn build_openvpn_packet(state: RwLock<ClientState>, ip_packet: Ipv4Packet) {
    let mut total_size = 0;

    let guard = state.read().await;
    match guard.state {
        ProtocolState::Unconnected => {}
        ProtocolState::InHandshake => {}
        ProtocolState::Connected => {
            let vpn_packet = 
            _ = ssl_write.write(&buffer).await;
        }
        ProtocolState::Errored => {}
    }
    total_size += ip_packet.get_total_length();

    return;
}
