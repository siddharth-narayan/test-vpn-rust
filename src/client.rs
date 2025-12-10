use clap::Parser;

mod protocols;
mod  network;

fn main() {
    let args = protocols::openvpn::config::ClientCli::parse();

    
}
