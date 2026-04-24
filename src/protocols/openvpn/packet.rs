use binrw::{
    Endian::Little,
    binrw, binwrite,
    helpers::until_eof,
    meta::{EndianKind, ReadEndian, WriteEndian},
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
    #[br(temp)]
    #[bw(calc = (2 + 1 + 1 + payload.len()).try_into().unwrap())]
    packet_len: u16,

    #[brw(restore_position)]
    message_type: MessageType, // (5 bits)
    key_id: u8, // (3 bits)

    payload: GenericPacket,
}

impl ReadEndian for OpenVPNPacket {
    const ENDIAN: EndianKind = EndianKind::Endian(Little);
}

impl WriteEndian for OpenVPNPacket {
    const ENDIAN: EndianKind = EndianKind::Endian(Little);
}

impl OpenVPNPacket {
    pub fn new(_type: MessageType, payload: GenericPacket) -> Self {
        Self {
            message_type: _type,
            key_id: 0, // TODO
            payload: payload,
        }
    }
}

#[binrw]
pub enum GenericPacket {
    CiphertextControlPacket(CiphertextControlPacket),
    PlaintextControlPacket(PlaintextControlPacket), // Obsolete?
    DataPacket(DataPacket),
}

impl GenericPacket {
    pub fn len(&self) -> usize {
        match self {
            GenericPacket::CiphertextControlPacket(p) => p.len(),
            GenericPacket::PlaintextControlPacket(p) => p.len(),
            GenericPacket::DataPacket(p) => p.len(),
        }
    }
}

#[binrw]
pub struct CiphertextControlPacket {
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

impl CiphertextControlPacket {
    pub fn len(&self) -> usize {
        8 + if true { 16 } else { 0 }
            + 8
            + 4
            + size_of::<PacketAck>() * self.packet_ack_len as usize
            + self.payload.len()
    }
}

#[binrw]
pub struct PlaintextControlPacket {
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

impl PlaintextControlPacket {
    pub fn len(&self) -> usize {
        4 + 1
            + 1
            + 2
            + self.options_len as usize
            + 2
            + self.user_len as usize
            + 2
            + self.pass_len as usize
    }
}

#[binrw]
#[br(import(max_len: usize))]
pub struct DataPacket {
    // 24-bits
    #[br(parse_with = binrw::helpers::read_u24)]
    #[bw(write_with = binrw::helpers::write_u24)]
    peer_id: u32,
    auth_data: Vec<u8>,
    payload: Vec<u8>,
}

impl DataPacket {
    pub fn new() -> Self {
        Self {}
    }

    pub fn len(&self) -> usize {
        0
    }
}
