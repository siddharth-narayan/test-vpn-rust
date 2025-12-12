use pnet::packet::ipv4::MutableIpv4Packet;

pub fn process_packet(mut buffer: Vec<u8>) {
    let p = MutableIpv4Packet::new(&mut buffer);

    match p {
        Some(p) => {
            println!("hAHA:{:?}", p);
        },
        None => {
            println!("Got none packet");
        }
    }
}