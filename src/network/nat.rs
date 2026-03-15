use std::{
    collections::HashMap, net::{SocketAddr}, sync::{Arc, Mutex, mpsc::Sender}
};

use pnet::packet::
    ip::IpNextHeaderProtocol
;

use crate::network::packet::BasePacket;

// pub fn build_nat_entry(mut packet: &Ipv4Packet) -> Option<NatEntry> {
//     let dest_ip = p.get_source();

//     let protocol = p.get_next_level_protocol();

//     if protocol != IpNextHeaderProtocols::Tcp || protocol != IpNextHeaderProtocols::Udp {
//         return None;
//     }

//     // Flipped because on an incoming packet the order will be reversed compared to when
//     // the entry was put into the NAT table
//     let (dest_port, source_port) = match protocol {
//         IpNextHeaderProtocols::Tcp => {
//             let packet = TcpPacket::new(&mut buffer)?;
//             (packet.get_source(), packet.get_destination())
//         }
//         IpNextHeaderProtocols::Udp => {
//             let packet = UdpPacket::new(&mut buffer)?;
//             (packet.get_source(), packet.get_destination())
//         }
//         _ => return None,
//     };

//     let entry = NatEntry {
//         proto: protocol,
//         source_port: source_port,
//         dest: std::net::SocketAddr::V4(SocketAddrV4::new(dest_ip, dest_port)),
//     };

//     Some(entry)
// }

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NatEntry {
    pub proto: IpNextHeaderProtocol,
    pub source_port: u16,
    pub dest: SocketAddr,
}

#[derive(Clone)]
pub struct NatTable {
    table: Arc<Mutex<HashMap<NatEntry, Sender<BasePacket>>>>
}

impl NatTable {
    pub fn new() -> Self {
        Self {
            table: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn insert(&mut self, e: NatEntry, sender: Sender<BasePacket>) {

    }

    pub fn lookup(&self, e: NatEntry) -> Option<Sender<BasePacket>> {
        let guard = self.table.lock().unwrap();
        guard.get(&e).cloned()
    }
} 
