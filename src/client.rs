use clap::Parser;
mod protocols;

fn main() {
    let args = protocols::openvpn::config::ClientCli::parse();

    
}
