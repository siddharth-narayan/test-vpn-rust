use std::sync::Arc;

use pnet::packet::ipv4::Ipv4Packet;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_openssl::SslStream;

use crate::{network::openssl::SslWrite, protocols::{
    fsm::{FSM, TransitionTable},
    openvpn::packet::{MessageType, OpenVPNPacket},
}};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum OpenVPNState {
    Unconnected,
    InHandshake,
    Connected,
    Errored,
}

pub fn build_server_fsm(mut ssl_write: Arc<Mutex<SslWrite>>) -> FSM<OpenVPNState, MessageType, OpenVPNPacket> {
    let mut transitions = TransitionTable::<OpenVPNState, MessageType, OpenVPNPacket>::new();

    transitions.insert(
        (
            OpenVPNState::Unconnected,
            MessageType::P_CONTROL_HARD_RESET_CLIENT_V2,
        ),
        (
            OpenVPNState::InHandshake,
            Box::new(|ssl_write, data| {
                
                return true;
            }),
        ),
    );

    transitions.insert(
        (
            OpenVPNState::InHandshake,
            MessageType::P_CONTROL_HARD_RESET_CLIENT_V2,
        ),
        (
            OpenVPNState::InHandshake,
            Box::new(|ssl_write, data| {
                return true;
            }),
        ),
    );

    transitions.insert(
        (
            OpenVPNState::Unconnected,
            MessageType::P_CONTROL_HARD_RESET_CLIENT_V2,
        ),
        (
            OpenVPNState::InHandshake,
            Box::new(|ssl_write, data| {
                return true;
            }),
        ),
    );

    transitions.insert(
        (
            OpenVPNState::Unconnected,
            MessageType::P_CONTROL_HARD_RESET_CLIENT_V2,
        ),
        (
            OpenVPNState::InHandshake,
            Box::new(|ssl_write, data| {
                return true;
            }),
        ),
    );

    transitions.insert(
        (
            OpenVPNState::Unconnected,
            MessageType::P_CONTROL_HARD_RESET_CLIENT_V2,
        ),
        (
            OpenVPNState::InHandshake,
            Box::new(|ssl_write, data| {
                return true;
            }),
        ),
    );

    FSM::new(OpenVPNState::Unconnected, transitions, ssl_write)
}
