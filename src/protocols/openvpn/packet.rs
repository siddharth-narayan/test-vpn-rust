use binrw::{Endian::Little, binrw, helpers::until_eof, meta::{EndianKind, ReadEndian}};
use pnet::packet::{Packet, ipv4::Ipv4Packet};

use std::{
    ffi::{CStr, CString},
    io::{Read, Write},
};

use crate::{
    network::{nat::NatEntry, packet::BasePacket},
    protocols::openvpn::protcol::{self, OpenVPNConnection},
};

#[binrw]
#[brw(repr=u8)]
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


#[binrw]
struct PacketAck {
    packet_num: u32, // The packet number that is acknowledged
}

// ==========================
// ===== Packet structs =====
// ==========================
#[binrw]
pub struct OpenVPNPacket {
    packet_len: u16,
    message_type: MessageType,
    key_id: u8, // key_id < 8 (3 bits)
    payload: GenericPacket,
}

impl ReadEndian for OpenVPNPacket {
    const ENDIAN: EndianKind = EndianKind::Endian(Little);
}

// impl  for OpenVPNPacket {
//     const ENDIAN: EndianKind = EndianKind::Endian(Little);
// }

#[binrw]
enum GenericPacket {
    CiphertextControlPacket(CiphertextControlPacket),
    PlaintextControlPacket(PlaintextControlPacket), // Obsolete?
    DataPacket(DataPacket),
}

#[binrw]
struct CiphertextControlPacket {
    session_id: u64,

    #[brw(if(true))] // Fix byte count
    hmac: [u8; 16], // Only if --tls-auth?

    replay_packet_id: u64,

    packet_ack_len: u32,
    #[br(count = packet_ack_len)]
    packet_acks: Vec<PacketAck>, // include peer_session_id if len > 0

    // TODO: This will read EVERYTHING, including possibly other packets. Make sure that this only reads to the 
    // End of the packet -- maybe read a packet from the stream into a n byte stream that can be transformed.
    #[br(parse_with = until_eof)] 
    payload: Vec<u8>,
}

#[binrw]
struct PlaintextControlPacket {
    #[brw(magic = 0u32)]
    magic: u32,

    key_method: u8,
    key_source: u8,

    options_len: u16,
    #[br(count = options_len)]
    options: Vec<u8>,

    user_len: u16,
    #[br(count = user_len)]
    username: Vec<u8>,

    pass_len: u16,
    #[br(count = pass_len)]
    password: Vec<u8>,
}

#[binrw]
struct DataPacket {}
