use std::{
    collections::HashMap,
    io::Write,
    net::{IpAddr, SocketAddr},
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
    packet::BasePacket,
};

#[derive(Clone)]
pub enum HubClientTable {
    // Maps NAT entries to send halves
    Nat(NatTable),

    // Maps IpAddr entries to send halves
    Base(Arc<Mutex<HashMap<IpAddr, Sender<Box<BasePacket>>>>>),
}

impl HubClientTable {
    pub fn insert(&mut self, p: &Box<BasePacket>, tx: Sender<Box<BasePacket>>) {
        match self {
            HubClientTable::Nat(x) => x.insert(NatEntry::from(p), tx),

            HubClientTable::Base(x) => {
                let mut guard = x.lock().unwrap();
                guard.insert(p.dst.ip(), tx.clone());
            }
        }
    }

    pub fn lookup(&self, e: &Box<BasePacket>) -> Option<Sender<Box<BasePacket>>> {
        match self {
            HubClientTable::Nat(x) => x.lookup(NatEntry::from(e)),

            HubClientTable::Base(x) => {
                let guard = x.lock().unwrap();
                guard.get(&e.dst.ip()).cloned()
            }
        }
    }
}

#[derive(Clone)]
pub struct HubSettings {
    pub use_nat: bool,
}

pub struct Hub {
    settings: HubSettings,
    client_tx: Sender<Box<BasePacket>>,
    hub_client_table: HubClientTable,
}

impl Hub {
    pub fn new(settings: HubSettings) -> Self {
        let client_table = if settings.use_nat {
            HubClientTable::Nat(NatTable::new())
        } else {
            HubClientTable::Base(Arc::new(Mutex::new(HashMap::new())))
        };

        let tun = get_default_tun();
        tun.set_nonblock().unwrap();

        let (client_tx, hub_rx) = mpsc::channel::<Box<BasePacket>>();

        let table_clone = client_table.clone();
        thread::spawn(move || hub_packet_processor(hub_rx, tun, table_clone));

        Self {
            settings: settings,
            client_tx: client_tx,
            hub_client_table: client_table,
        }
    }

    pub fn table(&self) -> HubClientTable {
        self.hub_client_table.clone()
    }

    pub fn tx(&self) -> Sender<Box<BasePacket>> {
        self.client_tx.clone()
    }
}

fn hub_packet_processor(hub_rx: Receiver<Box<BasePacket>>, mut tun: Device, table: HubClientTable) {
    loop {
        match hub_rx.try_recv() {
            Ok(p) => {
                let buf: Box<[u8]> = p.into();

                tun.write(buf.as_ref());
            }

            Err(e) => match e {
                mpsc::TryRecvError::Empty => (),
                mpsc::TryRecvError::Disconnected => panic!(),
            },
        }

        let mut buf: Box<[u8]> = Box::from([0u8; 256]);

        match tun.recv(buf.as_mut()) {
            Ok(x) => match Box::<BasePacket>::try_from(buf) {
                Ok(p) => {
                    let p = Box::new(p);
                    table.lookup(&p);
                },
                Err(e) => {
                    ()
                }
            },
            Err(e) => {
                todo!()
            }
        }
    }
}
