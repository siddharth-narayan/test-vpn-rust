use std::future;

use crate::protocols::openvpn::client;

mod protocols;
mod network;

#[tokio::main]
async fn main() {
    tokio::spawn(client::main());
    
    // Parse stuff commandline stuff here later
    future::pending::<()>().await;
}