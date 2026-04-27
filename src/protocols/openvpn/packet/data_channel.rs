#![allow(non_camel_case_types)]

use binrw::{
    Endian::Big,
    binrw,
    helpers::{until_eof},
    meta::{EndianKind, ReadEndian, WriteEndian},
};

use crate::protocols::openvpn::packet::MessageType;

#[binrw]
#[derive(Debug)]
#[br(import { opcode: MessageType })]
pub enum DataChannelPacket {
    P_DATA_V1(
        #[br(pre_assert(opcode == MessageType::P_DATA_V2))]
        P_DATA_V1
    ),
    P_DATA_V2(
        #[br(pre_assert(opcode == MessageType::P_DATA_V2))]
        P_DATA_V2
    )
}

#[binrw]
#[derive(Debug)]
pub struct P_DATA_V1 {
    #[br(parse_with = until_eof)]
    payload: Vec<u8>,
}

#[binrw]
#[derive(Debug)]
pub struct P_DATA_V2 {
    #[br(parse_with = binrw::helpers::read_u24)]
    #[bw(write_with = binrw::helpers::write_u24)]
    peer_id: u32, // 24-bits

    #[br(parse_with = until_eof)]
    payload: Vec<u8>,
}

impl ReadEndian for P_DATA_V1 {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl WriteEndian for P_DATA_V1 {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl ReadEndian for P_DATA_V2 {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}

impl WriteEndian for P_DATA_V2 {
    const ENDIAN: EndianKind = EndianKind::Endian(Big);
}