use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

use tun::Device;

use crate::network::{
    device::get_default_tun,
    nat::{NatEntry, NatTable},
    packet::{Address, BasePacket}, util::Layer,
};

#[derive(Clone)]
pub enum HubClientTable {
    // Maps NAT entries to send halves
    Nat(NatTable),

    // Maps IpAddr entries to send halves
    Base(Arc<Mutex<HashMap<Address, Sender<BasePacket>>>>),
}

impl HubClientTable {
    pub fn insert(&mut self, p: &BasePacket, tx: Sender<BasePacket>) {
        match self {
            HubClientTable::Nat(x) => {
                let entry = match NatEntry::try_from(p) {
                    Ok(e) => e,
                    Err(_) => {
                        return;
                    }
                };

                x.insert(entry, tx)
            }

            HubClientTable::Base(x) => {
                let mut guard = x.lock().unwrap();
                guard.insert(p.dst(), tx.clone());
            }
        }
    }

    pub fn lookup(&self, p: &BasePacket) -> Option<Sender<BasePacket>> {
        match self {
            HubClientTable::Nat(x) => {
                let entry = match NatEntry::try_from(p) {
                    Ok(e) => e,
                    Err(_) => return None,
                };

                x.lookup(entry)
            }

            _ => None, // HubClientTable::Base(x) => {
                       //     let guard = x.lock().unwrap();
                       //     let addr = match p.p_type {
                       //         PacketType::IPv4 => {

                       //         }
                       //     }
                       //     guard.get(&p.dst.ip()).cloned()
                       // }
        }
    }
}

#[derive(Clone)]
pub struct HubSettings {
    pub use_nat: bool,
    pub layer: Layer,
}

pub struct Hub {
    layer: Layer,
    settings: HubSettings,
    client_tx: Sender<BasePacket>,
    hub_client_table: HubClientTable,
}

impl Hub {
    pub fn new(settings: HubSettings) -> Self {
        let client_table = if settings.use_nat {
            HubClientTable::Nat(NatTable::new())
        } else {
            HubClientTable::Base(Arc::new(Mutex::new(HashMap::new())))
        };

        let tun = get_default_tun(settings.layer);
        tun.set_nonblock().unwrap();

        let (client_tx, hub_rx) = mpsc::channel::<BasePacket>();

        // For now just drop this so the receiver doesn't hang up when there are no clients
        drop(client_tx.clone());

        let table_clone = client_table.clone();
        thread::spawn(move || hub_packet_processor(hub_rx, tun, table_clone));

        Self {
            layer: Layer::L3,
            settings: settings,
            client_tx: client_tx,
            hub_client_table: client_table,
        }
    }

    pub fn table(&self) -> HubClientTable {
        self.hub_client_table.clone()
    }

    pub fn tx(&self) -> Sender<BasePacket> {
        self.client_tx.clone()
    }
}

fn hub_packet_processor(hub_rx: Receiver<BasePacket>, mut tun: Device, _table: HubClientTable) {
    loop {
        match hub_rx.try_recv() {
            Ok(p) => {
                let _ = tun.write(p.raw_ref());
            }

            Err(e) => match e {
                mpsc::TryRecvError::Empty => (),
                mpsc::TryRecvError::Disconnected => panic!(),
            },
        }

        let mut buf = [0u8; 512];

        match tun.read(buf.as_mut()) {
            Ok(_) => {
                println!("TUN received bytes: {:?}", buf);
                BasePacket::new(Layer::L3, buf.to_vec());
            },
            Err(e) => {
                if e.kind() != ErrorKind::WouldBlock {
                    println!("Failed to read from TUN: {}", e);
                }
            }
        };
    }
}
