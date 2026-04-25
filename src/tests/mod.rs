use std::{io::Cursor, time::SystemTime};

use binrw::BinRead;

use crate::protocols::openvpn::packet::DataPacket;

#[test]
pub fn packet() {
    let mut bytes = vec![1, 0, 0, 2, 2, 2, 2, 3, 3, 3, 3, 3];

    
    // let packet = DataPacket::read_args(&mut Cursor::new(bytes), (SystemTime::now(), Box::from([])));
    let packet = DataPacket::read(&mut Cursor::new(bytes));
    
    println!("TEST DATAPACKET: {:?}", packet)
}