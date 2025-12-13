use pnet::packet::ipv4::Ipv4Packet;

use crate::protocols::{fsm::{FSM, FSMAction, TransitionTable}, openvpn::packet::MessageType};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum OpenVPNState {
    Unconnected,
    InHandshake,
    Connected,
    Errored,

}

enum OpenVPNAction {
    
}



impl FSMAction<OpenVPNState, MessageType> for OpenVPNAction {
    fn execute<Ipv4Packet>(&self, state: OpenVPNState, input: MessageType, data: Ipv4Packet) -> bool {

        // Only thing that needs to be verified here is that the packet is valid, and any side effects
        // the transition table takes care of everything else
        match state {
            OpenVPNState::Unconnected => {
                return true;
            }

            OpenVPNState::InHandshake => {

            }

            OpenVPNState::Connected => {

            }

            OpenVPNState::Errored => {

            }
        }
        true
    }
}


pub fn build_server_fsm() -> FSM<OpenVPNState, MessageType, OpenVPNAction> {
    let mut transitions = TransitionTable::<OpenVPNState, MessageType, OpenVPNAction>::new();
    transitions.insert(
        (OpenVPNState::Unconnected, MessageType::P_CONTROL_HARD_RESET_CLIENT_V2),
        
    );

    FSM::new(OpenVPNState::Unconnected, transitions)

}