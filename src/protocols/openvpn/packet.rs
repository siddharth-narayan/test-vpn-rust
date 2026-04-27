use binrw::{
    Endian::Big,
    binrw,
    helpers::until_eof,
    meta::{EndianKind, ReadEndian, WriteEndian},
};

use crate::protocols::openvpn::packet::{control_channel::ControlChannelPacket, data_channel::DataChannelPacket};

pub mod control_channel;
pub mod data_channel;

#[binrw]
#[brw(repr=u8)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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

impl From<u8> for MessageType {
    fn from(a: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(a) }
    }
}

pub enum AuthType {
    CBC
}

impl AuthType {
    pub fn auth_len(_type: Self) -> usize {
        match _type {
            Self::CBC => 4
        }
    }
}

#[binrw]
#[derive(Debug)]
struct PacketAck(u32); // The packet number that is acknowledged
pub struct PacketLen(u16);

impl Into<u16> for PacketLen {
    fn into(self) -> u16 {
        self.0
    }
}

// ==========================
// ===== Packet structs =====
// ==========================
#[binrw]
#[derive(Debug)]
pub struct OpenVPNPacket {
    #[br(temp)]
    #[bw(calc = 2)]
    packet_len: u16,

    #[br(temp)]
    #[bw(calc = ((*message_type as u8) << 3) + key_id)]
    type_key_tuple: u8,

    #[br(calc = MessageType::from(type_key_tuple >> 3))]
    #[bw(ignore)]
    message_type: MessageType, // (5 bits)
    #[br(calc = type_key_tuple &  0b0000_0111u8)]
    #[bw(ignore)]
    key_id: u8, // (3 bits)

    #[br(args { opcode: message_type })]
    payload: GenericPacket,
}


#[binrw]
#[br(import {opcode: MessageType })]
#[derive(Debug)]
pub enum GenericPacket {
    ControlChannelPacket(
        #[br(args { opcode })]
        ControlChannelPacket
    ),
    DataPacket(
        #[br(args { opcode })]
        DataChannelPacket
    ),
}

impl ReadEndian for OpenVPNPacket {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl WriteEndian for OpenVPNPacket {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}