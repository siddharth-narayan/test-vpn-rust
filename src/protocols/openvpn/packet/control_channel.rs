#![allow(non_camel_case_types)]

use binrw::{
    Endian::Big,
    binrw,
    helpers::{until},
    meta::{EndianKind, ReadEndian, WriteEndian},
};

use crate::protocols::openvpn::packet::{PacketAck, MessageType};

fn until_null(byte: &u8) -> bool {
    *byte == 0
}

#[binrw]
#[derive(Debug)]
#[br(import { opcode: MessageType })]
pub struct ControlChannelPacket {
    session_id: u64,

    #[br(count = if true { 20 } else { 0 })] // Fix byte count
    hmac: Vec<u8>, // Only if --tls-auth?

    replay_packet_id: u32,

    packet_ack_len: u32,
    #[br(count = packet_ack_len)]
    packet_acks: Vec<PacketAck>, // include peer_session_id if len > 0

    net_time: u32,

    payload: ControlMethod
}

#[binrw]
#[derive(Debug)]
enum ControlMethod {
    CONTROL_METHOD_1(CONTROL_METHOD_1),
    CONTROL_METHOD_2(CONTROL_METHOD_2)
}

#[binrw]
#[derive(Debug)]
pub struct CONTROL_METHOD_1 {
    key_len: u8,
    #[br(count = key_len)]
    key: Vec<u8>,


    hmac_len: u8,
    #[br(count = hmac_len)]
    hmac: Vec<u8>,

    #[br(parse_with = until(until_null))]
    options: Vec<u8>
}

#[binrw]
#[derive(Debug)]
pub struct CONTROL_METHOD_2 {
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


impl ReadEndian for ControlChannelPacket {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl WriteEndian for ControlChannelPacket {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl ReadEndian for CONTROL_METHOD_1 {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl WriteEndian for CONTROL_METHOD_1 {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl ReadEndian for CONTROL_METHOD_2 {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl WriteEndian for CONTROL_METHOD_2 {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}
