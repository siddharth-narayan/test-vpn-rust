use std::sync::mpsc::Receiver;
use std::thread;
use std::{sync::mpsc};

use crate::network::nat::NatTable;
use crate::protocols::openvpn;
use crate::protocols::util::ProtocolThreadInfo;
use crate::{network::packet::BasePacket};
use crate::network::device::get_default_tun;

mod network;
mod protocols;

fn main() {
    let (tun_read, tun_write) = get_default_tun().split();
    let nat_table = NatTable::new();

    let (packet_sender, packet_receiver) = mpsc::channel::<Box<BasePacket>>();

    let thread_info = ProtocolThreadInfo {
        nat_table: nat_table.clone(),
        packet_write_stream: packet_sender.clone(),
    };

    thread::spawn(move || {
        spawn_packet_receiver(packet_receiver)
    });

    thread::spawn(move || {
        openvpn::server::openvpn_main_thread(thread_info)
    });
}

fn spawn_packet_receiver(receiver: Receiver<Box<BasePacket>>) {
    
}