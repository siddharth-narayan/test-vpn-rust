use std::sync::mpsc::Sender;

use crate::network::{nat::NatTable, packet::BasePacket};

pub struct ProtocolThreadInfo {
    pub nat_table: NatTable,
    pub packet_write_stream: Sender<Box<BasePacket>>

}

// impl ProtocolThreaadInfo {
//     pub fn new(tx: Sender<BasePacket>) {

//     }
// }