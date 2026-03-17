use std::sync::mpsc::{Sender};
use std::thread;

use crate::network::hub::{Hub, HubSettings};
use crate::network::nat::NatTable;
use crate::protocols::openvpn;
use crate::{network::packet::BasePacket};

mod network;
mod protocols;

pub struct ClientMainThreadInfo {
    pub nat_table: NatTable,
    pub packet_write_stream: Sender<Box<BasePacket>>,
}

fn main() {

    let hub_settings = HubSettings {
        use_nat: false
    };
    
    let hub = Hub::new(hub_settings);

    thread::spawn(move || {
        openvpn::server::openvpn_main_thread(hub)
    });
}