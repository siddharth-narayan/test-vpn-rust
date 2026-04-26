use core::{convert::{From, Into}, mem::transmute, todo};
use std::io::{Read, Seek, Write};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

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

impl TryFrom<u8> for MessageType {
    type Error = ();
    fn try_from(repr: u8) -> Result<Self, Self::Error> {
        if repr < 1 || repr > 10 {
            Err(())
        } else {
            Ok(unsafe { transmute::<u8, Self>(repr) })
        }
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

struct PacketAck(u32); // The packet number that is acknowledged

// ==========================
// ===== Packet structs =====
// ==========================

trait Packet {
    type Error = PacketConstructError;
    type ReadArgs;
    type WriteArgs;

    fn read<T: Read + Write + Seek>(buf: &mut T, args: Self::ReadArgs) -> Result<Self, Self::Error> where Self: Sized;
    fn write<T: Read + Write + Seek>(&self, buf: &mut T, args: Self::WriteArgs);

    fn len(&self) -> usize;
}

enum PacketConstructError {
    MagicBroken,
    IOError
}

pub struct OpenVPNPacket {
    packet_len: u16,

    message_type: MessageType, // (5 bits)
    key_id: u8, // (3 bits)

    payload: GenericPacket,
}

impl Packet for OpenVPNPacket {
    type WriteArgs = ();
    type ReadArgs = ();
    fn len(&self) -> usize {
        0
    }

    fn read<T: Read + Write + Seek>(buf: &mut T, args: Self::ReadArgs) -> Result<Self, Self::Error> {
        let packet_len = buf.read_u16::<LittleEndian>().map_err(|_| PacketConstructError::IOError)?;
        let message_type = buf.read_u8().map_err(|_| PacketConstructError::IOError)?;
        let key_id = message_type;
        let payload = GenericPacket::read(buf, (packet_len.into(), MessageType::try_from(message_type).unwrap()))?;

        Ok(Self {
            packet_len: packet_len,
            message_type: MessageType::try_from(message_type).unwrap(),
            key_id: key_id as u8,
            payload: payload
        })
    }

    fn write<T: Read + Write + Seek>(&self, buf: &mut T, args: Self::WriteArgs) {
        buf.write_u16::<LittleEndian>(self.packet_len);
    }
}

pub enum GenericPacket {
    CiphertextControlPacket(CiphertextControlPacket),
    PlaintextControlPacket(PlaintextControlPacket), // Obsolete?
    DataPacket(DataPacket),
}

impl Packet for GenericPacket {
    type ReadArgs = (usize, MessageType);
    type WriteArgs = MessageType;

    fn len(&self) -> usize {
        0
    }

    fn read<T: Read + Write + Seek>(buf: &mut T, args: Self::ReadArgs) -> Result<Self, Self::Error> {
        match args.1 {
            MessageType::P_ACK_V1 => {

            },

            _ => {

            }
        };

        Err(PacketConstructError::IOError)
    }

    fn write<T: Read + Write + Seek>(&self, buf: &mut T, _args: Self::WriteArgs) {
        match &self {
            &GenericPacket::CiphertextControlPacket(p) => p.write(buf, ()),
            &GenericPacket::PlaintextControlPacket(p) => p.write(buf, ()),
            &GenericPacket::DataPacket(p) => p.write(buf, ()),
        }
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
    type ReadArgs = usize;
    type WriteArgs = ();
    
    fn len(&self) -> usize {
        0
    }

    fn read<T: Read + Write + Seek>(buf: &mut T, args: Self::ReadArgs) -> Result<Self, Self::Error> {
        buf.read_u16::<BigEndian>();
        todo!()
    }

    fn write<T: Read + Write + Seek>(&self, buf: &mut T, args: Self::WriteArgs) {
        todo!()
    }
}

enum CiphertextControlPacketError {

}




pub struct PlaintextControlPacket {
    magic: u32,

    key_method: u8,
    key_source: u8,

    options_len: u16,
    options: Vec<u8>,

    user_len: u16,
    username: Vec<u8>,

    pass_len: u16,
    password: Vec<u8>,
}

impl Packet for PlaintextControlPacket {
    type ReadArgs = ();
    type WriteArgs = ();

    fn len(&self) -> usize {
        4 + 1
            + 1
            + 2
            + self.options_len as usize
            + 2
            + self.user_len as usize
            + 2
            + self.pass_len as usize
    }

    fn read<T: Read + Write + Seek>(buf: &mut T, args: Self::ReadArgs) -> Result<Self, Self::Error> {
        let magic = buf.read_u32::<LittleEndian>().map_err(|_| PacketConstructError::IOError)?;

        if magic != 0 {
            return PacketConstructError::MagicBroken;
        }



    }

    fn write<T: Read + Write + Seek>(&self, buf: &mut T, args: Self::WriteArgs) {
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

enum DataPacketError {

}

impl Packet for DataPacket {
    type ReadArgs = ();
    type WriteArgs = ();
    
    fn len(&self) -> usize {
        0
    }

    fn read<T: Read + Write + Seek>(buf: &mut T, args: Self::ReadArgs) -> Result<Self, Self::Error> {
        buf.read_u16::<BigEndian>();
        todo!()
    }

    fn write<T: Read + Write + Seek>(&self, buf: &mut T, args: Self::WriteArgs) {
        todo!()
    }
}