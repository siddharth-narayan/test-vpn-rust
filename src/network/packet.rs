use std::net::{Ipv4Addr, Ipv6Addr};

use pnet::{
    packet::{
        ethernet::EthernetPacket, ipv4::Ipv4Packet,
        ipv6::Ipv6Packet,
    },
    util::MacAddr,
};

use crate::network::util::{Layer, PacketProtocol};

#[derive(PartialEq)]
pub enum PacketRepr<'a> {
    Ethernet(EthernetPacket<'a>),
    IPv4(Ipv4Packet<'a>),
    IPv6(Ipv6Packet<'a>),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Address {
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
    MAC(MacAddr),
}

pub struct BasePacket {
    layer: Layer,
    proto: PacketProtocol,

    src: Address,
    dst: Address,

    raw: Box<[u8]>
}

impl BasePacket {
    pub fn new(layer: Layer, bytes: Vec<u8>) -> Option<Self> {
        let raw = bytes.into_boxed_slice();

        Some(Self {
            layer: layer,
            proto: packet_proto(layer, &raw)?,

            src: packet_src(layer, &raw)?,
            dst: packet_dst(layer, &raw)?,
            raw: raw
        })
    }

    pub fn raw_ref(&self) -> &Box<[u8]> {
        &self.raw
    }

    pub fn proto(&self) -> PacketProtocol {
        self.proto
    }

    pub fn dst(&self) -> Address {
        self.dst
    }


}

fn packet_proto(layer: Layer, packet: &Box<[u8]>) -> Option<PacketProtocol> {
    match layer {
        Layer::L2 => {
            return Some(PacketProtocol::Ethernet);
        },

        Layer::L3 => {
            let repr = Ipv4Packet::new(packet)?;

            if repr.get_version() == 4 {
                return Some(PacketProtocol::IPv4);
            } else {
                return Some(PacketProtocol::IPv6);
            }
        }
    }
}

fn packet_src(layer: Layer, packet: &Box<[u8]>) -> Option<Address> {
    match layer {
        Layer::L2 => {
            let repr = EthernetPacket::new(packet)?;
            return Some(Address::MAC(repr.get_source()));
        },

        Layer::L3 => {
            let repr = Ipv4Packet::new(packet)?;

            if repr.get_version() == 4 {
                return Some(Address::IPv4(repr.get_source()));
            } else {
                let repr_v6 = Ipv6Packet::new(packet)?;
                return Some(Address::IPv6(repr_v6.get_source()));
            }
        }
    }
}

fn packet_dst(layer: Layer, packet: &Box<[u8]>) -> Option<Address> {
    match layer {
        Layer::L2 => {
            let repr = EthernetPacket::new(packet)?;
            return Some(Address::MAC(repr.get_destination()));
        },

        Layer::L3 => {
            let repr = Ipv4Packet::new(packet)?;

            if repr.get_version() == 4 {
                return Some(Address::IPv4(repr.get_destination()));
            } else {
                let repr_v6 = Ipv6Packet::new(packet)?;
                return Some(Address::IPv6(repr_v6.get_destination()));
            }
        }
    }
}