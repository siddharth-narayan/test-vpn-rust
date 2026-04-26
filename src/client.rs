#![feature(negative_impls)]
#![feature(associated_type_defaults)]

use std::thread;

use crate::network::hub::{Hub, HubSettings};
use crate::network::util::Layer;
use crate::protocols::openvpn;

mod network;
mod protocols;

fn main() {
    let hub_settings = HubSettings {
        layer: Layer::L3,
        use_nat: false,
    };

    let hub = Hub::new(hub_settings);

    thread::spawn(move || openvpn::client::openvpn_main_thread(hub));

    loop {}
}
