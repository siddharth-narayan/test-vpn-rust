use pnet::packet::{Packet, ipv4::Ipv4Packet};

use std::{
    ffi::{CStr, CString},
    io::{Read, Write},
};

use crate::{
    network::{
        nat::NatEntry,
        packet::{BasePacket},
    },
    protocols::openvpn::protcol::{self, OpenVPNConnection},
};

#[repr(u8)]
#[allow(non_camel_case_types)]
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

// ==========================
// ===== Packet structs =====
// ==========================
pub struct OpenVPNPacket {
    packet_len: u16,
    message_type: MessageType,
    key_id: u8, // key_id < 8 (3 bits)
    payload: GenericPacket,
}

enum GenericPacket {
    CiphertextControlPacket(CiphertextControlPacket),
    PlaintextControlPacket(PlaintextControlPacket), // Obsolete?
    DataPacket(DataPacket),
}

struct CiphertextControlPacket {
    session_id: u64,
    // #[br(count = 1)] // Fix byte count
    hmac: Option<Vec<u8>>, // Only if --tls-auth?
    replay_packet_id: u64,
    packet_acks: Vec<PacketAck>, // include peer_session_id if len > 0
    packet_id: u32,
    // #[br(count)]
    payload: Vec<u8>,
}

struct PlaintextControlPacket {
    key_method: u8,
    key_source: u8,
    // options: Option<CStr>,
    // username: Option<CStr>,
    // password: Option<CStr>,
}

struct DataPacket {}

