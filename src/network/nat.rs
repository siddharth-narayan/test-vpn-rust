use std::{
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
};

use dashmap::DashMap;
use pnet::
    packet::{
        ip::{IpNextHeaderProtocol, IpNextHeaderProtocols},
        ipv4::Ipv4Packet,
        tcp::TcpPacket,
        udp::UdpPacket,
    }
;

pub fn process_incoming_packet(mut buffer: Vec<u8>) -> Option<NatEntry> {
    let p = Ipv4Packet::new(&mut buffer)?;

    let dest_ip = p.get_source();

    let protocol = p.get_next_level_protocol();

    if protocol != IpNextHeaderProtocols::Tcp || protocol != IpNextHeaderProtocols::Udp {
        return None;
    }

    // Flipped because on an incoming packet the order will be reversed compared to when
    // the entry was put into the NAT table
    let (dest_port, source_port) = match protocol {
        IpNextHeaderProtocols::Tcp => {
            let packet = TcpPacket::new(&mut buffer)?;
            (packet.get_source(), packet.get_destination())
        }
        IpNextHeaderProtocols::Udp => {
            let packet = UdpPacket::new(&mut buffer)?;
            (packet.get_source(), packet.get_destination())
        }
        _ => return None,
    };

    let entry = NatEntry {
        proto: protocol,
        source_port: source_port,
        dest: std::net::SocketAddr::V4(SocketAddrV4::new(dest_ip, dest_port)),
    };

    Some(entry)
}

pub struct NatEntry {
    pub proto: IpNextHeaderProtocol,
    pub source_port: u16,
    pub dest: SocketAddr,
}

type NatTable = Arc<DashMap<NatEntry, SocketAddr>>;
