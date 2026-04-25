use std::io::{Read, Seek, Write};
use byteorder::{BigEndian, ReadBytesExt};

#[allow(non_camel_case_types)]
#[repr(u8)]
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

struct PacketAck {
    packet_num: u32, // The packet number that is acknowledged
}

// ==========================
// ===== Packet structs =====
// ==========================

trait Packet {
    type Error;
    fn read<T: Read + Write + Seek>(buf: T) -> Result<Self, Self::Error> where Self: Sized;
    fn write<T: Read + Write + Seek>(&self, buf: T);

    fn len(&self) -> usize;
}

pub struct OpenVPNPacket {
    packet_len: u16,

    message_type: MessageType, // (5 bits)
    key_id: u8, // (3 bits)

    payload: GenericPacket,
}

impl Packet for OpenVPNPacket {
    type Error = ();
    
    fn len(&self) -> usize {
        0
    }

    fn read<T: Read + Write + Seek>(buf: T) -> Result<Self, Self::Error> {
        buf.read_u16::<BigEndian>()
    }

    fn write<T: Read + Write + Seek>(&self, buf: T) {
        todo!()
    }
}

pub enum GenericPacket {
    CiphertextControlPacket(CiphertextControlPacket),
    PlaintextControlPacket(PlaintextControlPacket), // Obsolete?
    DataPacket(DataPacket),
}

impl Packet for GenericPacket {
    type Error = ();
    
    fn len(&self) -> usize {
        0
    }

    fn read<T: Read + Write + Seek>(buf: T) -> Result<Self, Self::Error> {
        buf.read_u16::<BigEndian>()
    }

    fn write<T: Read + Write + Seek>(&self, buf: T) {
        todo!()
    }
}

pub struct CiphertextControlPacket {
    session_id: u64,

    hmac: [u8; 16], // Only if --tls-auth?

    replay_packet_id: u64,

    packet_ack_len: usize,
    packet_acks: Vec<PacketAck>, // include peer_session_id if len > 0

    payload: Vec<u8>,
}

impl Packet for CiphertextControlPacket {
    type Error = ();
    
    fn len(&self) -> usize {
        0
    }

    fn read<T: Read + Write + Seek>(buf: T) -> Result<Self, Self::Error> {
        buf.read_u16::<BigEndian>()
    }

    fn write<T: Read + Write + Seek>(&self, buf: T) {
        todo!()
    }
}

pub struct PlaintextControlPacket {
    magic: u32,

    key_method: u8,
    key_source: u8,

    options_len: usize,
    options: Vec<u8>,

    user_len: usize,
    username: Vec<u8>,

    pass_len: usize,
    password: Vec<u8>,
}

impl Packet for PlaintextControlPacket {
    type Error = ();
    
    fn len(&self) -> usize {
        4 + 1
            + 1
            + 2
            + self.options_len
            + 2
            + self.user_len
            + 2
            + self.pass_len
    }

    fn read<T: Read + Write + Seek>(buf: T) -> Result<Self, Self::Error> {
        buf.read_u16::<BigEndian>()
    }

    fn write<T: Read + Write + Seek>(&self, buf: T) {
        todo!()
    }

}

#[derive(Debug)]
pub struct DataPacket {
    // 24-bits
    peer_id: u32,

    auth_data: Vec<u8>,

    payload: Vec<u8>,
}

impl Packet for DataPacket {
    type Error = ();
    
    fn len(&self) -> usize {
        0
    }

    fn read<T: Read + Write + Seek>(buf: T) -> Result<Self, Self::Error> {
        buf.read_u16::<BigEndian>()
    }

    fn write<T: Read + Write + Seek>(&self, buf: T) {
        todo!()
    }
}