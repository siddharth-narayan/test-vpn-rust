// LLM Generated
use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Toml},
    error::Error,
};
use serde::{Serialize, Deserialize};
use std::net::IpAddr;

// ── Client ────────────────────────────────────────────────────────────────────

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct ClientConfig {
    /// Remote VPN server IP
    #[arg(long, default_value = "127.0.0.1")]
    pub server_ip: IpAddr,

    /// Remote VPN server port
    #[arg(long, default_value_t = 51820)]
    pub server_port: u16,

    /// Private key path (PEM)
    #[arg(long, default_value = "client.key")]
    pub private_key: String,

    /// Server public key path (PEM)
    #[arg(long, default_value = "server.pub")]
    pub server_public_key: String,
}

pub fn get_client_config() -> Result<ClientConfig, Error> {
    Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("VPN_"))
        .extract()
}

// ── Server ────────────────────────────────────────────────────────────────────

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct ServerConfig {
    // ── Listener ──────────────────────────────────────────────────────────────

    /// IP to bind the UDP/TCP listener on
    #[arg(long, default_value = "0.0.0.0")]
    pub bind_ip: IpAddr,

    /// Port to listen on
    #[arg(long, default_value_t = 51820)]
    pub bind_port: u16,

    // ── TUN device ────────────────────────────────────────────────────────────

    /// Name of the TUN interface to create
    #[arg(long, default_value = "tun0")]
    pub tun_name: String,

    /// IP address assigned to the TUN interface (server-side tunnel IP)
    #[arg(long, default_value = "10.0.0.1")]
    pub tun_ip: IpAddr,

    /// Subnet mask for the TUN interface, e.g. 255.255.255.0
    #[arg(long, default_value = "255.255.255.0")]
    pub tun_netmask: IpAddr,

    /// CIDR prefix for the VPN subnet, e.g. 24
    #[arg(long, default_value_t = 24)]
    pub tun_prefix: u8,

    /// MTU for the TUN device
    #[arg(long, default_value_t = 1420)]
    pub tun_mtu: u16,

    // ── IP pool (for assigning addresses to clients) ───────────────────────────

    /// First assignable client IP in the pool
    #[arg(long, default_value = "10.0.0.2")]
    pub pool_start: IpAddr,

    /// Last assignable client IP in the pool
    #[arg(long, default_value = "10.0.0.254")]
    pub pool_end: IpAddr,

    // ── DNS ───────────────────────────────────────────────────────────────────

    /// DNS server(s) to push to clients (comma-separated)
    #[arg(long, default_value = "1.1.1.1,8.8.8.8")]
    pub dns_servers: String,

    // ── Crypto ────────────────────────────────────────────────────────────────

    /// Server private key path (PEM)
    #[arg(long, default_value = "server.key")]
    pub private_key: String,

    /// Directory of trusted client public keys
    #[arg(long, default_value = "clients/")]
    pub client_keys_dir: String,

    // ── Routing ───────────────────────────────────────────────────────────────

    /// Enable NAT / IP masquerade for client traffic
    #[arg(long, default_value_t = true)]
    pub nat_enabled: bool,

    /// Routes to push to clients (comma-separated CIDRs)
    #[arg(long, default_value = "0.0.0.0/0")]
    pub push_routes: String,

    // ── Keepalive ─────────────────────────────────────────────────────────────

    /// Keepalive interval in seconds (0 = disabled)
    #[arg(long, default_value_t = 25)]
    pub keepalive_secs: u64,

    /// Peer timeout in seconds
    #[arg(long, default_value_t = 120)]
    pub timeout_secs: u64,

    // ── Logging ───────────────────────────────────────────────────────────────

    /// Log level: error | warn | info | debug | trace
    #[arg(long, default_value = "info")]
    pub log_level: String,
}

pub fn get_server_config() -> Result<ServerConfig, Error> {
    Figment::new()
        .merge(Toml::file("Config.toml"))
        .merge(Env::prefixed("VPN_"))
        .extract()
}