use pnet::packet::
    ipv4::Ipv4Packet
;

use std::ffi::CString;

use crate::{network::{nat::NatEntry, packet::BasePacket}, protocols::openvpn::protcol::{self, OpenVPNConnection}};

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]

pub enum MessageType {
    P_CONTROL_HARD_RESET_CLIENT_V1 = 1,
    P_CONTROL_HARD_RESET_SERVER_V1,
    P_CONTROL_SOFT_RESET_V1,
    P_CONTROL_V1,
    P_ACK_V1,
    P_DATA_V1,
    P_CONTROL_HARD_RESET_CLIENT_V2,
    P_CONTROL_HARD_RESET_SERVER_V2,
    P_DATA_V2,
    P_CONTROL_HARD_RESET_CLIENT_V3,
}

struct PacketAck {}

impl PacketAck {
    pub fn len(&self) -> usize {
        return 0;
    }
}

pub struct OpenVPNPacket {
    packet_len: u16,
    message_type: MessageType,
    key_id: u8, // key_id < 8 (3 bits)
    payload: GenericPacket,
}

impl OpenVPNPacket {
    pub fn new(p_type: MessageType, payload: GenericPacket) -> Self {
        OpenVPNPacket {
            packet_len: 0,
            message_type: p_type,
            key_id: 0,
            payload: payload,
        }
    }
}

#[derive(Debug)]
pub enum OpenVPNPacketConvErr {

}

impl TryFrom<Box<BasePacket>> for OpenVPNPacket {
    type Error = OpenVPNPacketConvErr;
    fn try_from(value: Box<BasePacket>) -> Result<Self, Self::Error> {
        // OpenVPNPacket {
            
        // }
        todo!()
    }
}

impl<'a> TryFrom<Ipv4Packet<'a>> for OpenVPNPacket {
    type Error = OpenVPNPacketConvErr;
    fn try_from(value: Ipv4Packet) -> Result<Self, Self::Error> {
        // OpenVPNPacket {
            
        // }
        todo!()
    }
}

impl Into<BasePacket> for OpenVPNPacket {
    fn into(self) -> BasePacket {
        // BasePacket { layer: PacketLayer::L2, src: (), dst: (), payload: () }
        todo!()
    }
}

impl Into<Box<[u8]>> for OpenVPNPacket {
    fn into(self) -> Box<[u8]> {
        todo!()
    }
}

enum GenericPacket {
    CiphertextControlPacket(CiphertextControlPacket),
    PlaintextControlPacket(PlaintextControlPacket), // Obsolete?
    DataPacket(DataPacket),
}

impl GenericPacket {
    pub fn len(&self) -> usize {
        match self {
            GenericPacket::CiphertextControlPacket(ciphertext_control_packet) => {
                ciphertext_control_packet.payload.len()
            }
            GenericPacket::PlaintextControlPacket(plaintext_control_packet) => todo!(),
            GenericPacket::DataPacket(data_packet) => todo!(),
        }
    }
}

struct CiphertextControlPacket {
    session_id: u64,
    hmac: Option<Vec<u8>>, // Only if --tls-auth?
    replay_packet_id: u64,
    packet_acks: Vec<PacketAck>, // include peer_session_id if len > 0
    packet_id: u32,
    payload: Vec<u8>,
}

impl CiphertextControlPacket {
    pub fn new(session: u64, payload: Vec<u8>) -> Self {
        CiphertextControlPacket {
            session_id: session,
            hmac: None,
            replay_packet_id: 0,
            packet_acks: Vec::new(),
            packet_id: 0,
            payload: payload,
        }
    }

    pub fn len(&self) {
        let mut len: usize = 24; // 3 x u64 = 24 bytes

        if self.hmac.is_some() {
            len += self.hmac.clone().unwrap().len();
        }

        len += self
            .packet_acks
            .iter()
            .map(|packet_ack| packet_ack.len())
            .sum::<usize>();

        len += self.payload.len()
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        out.extend(self.session_id.to_le_bytes());

        if self.hmac.is_some() {
            out.append(&mut self.hmac.unwrap())
        }

        out
    }
}

struct PlaintextControlPacket {
    key_method: u8,
    key_source: u8,

    options: Option<CString>,
    username: Option<CString>,
    password: Option<CString>,
}

impl PlaintextControlPacket {
    fn new(method: u8) -> PlaintextControlPacket {
        PlaintextControlPacket {
            key_method: 0,
            key_source: 0,
            options: None,
            username: None,
            password: None,
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        let mut prefix = vec![0u8; 4];
        out.append(&mut prefix); // Literal 0 as first 4 bytes

        out.push(self.key_method);
        out.push(self.key_source);

        if self.options.is_some() {
            out.append(self.options.unwrap().into_bytes().as_mut());
        }

        if self.username.is_some() {
            out.append(self.username.unwrap().into_bytes().as_mut());

            if self.password.is_some() {
                out.append(self.password.unwrap().into_bytes().as_mut()); // Might need null terminator?
            }
        }

        return out;
    }
}

struct DataPacket {}

impl DataPacket {}
