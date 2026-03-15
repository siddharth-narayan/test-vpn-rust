// use clap::Parser;
// use figment::{
//     Figment,
//     providers::{Env, Format, Serialized, Toml},
// };
// use serde::{Deserialize, Serialize};

// #[derive(Parser, Debug, Serialize, Deserialize)]
// #[command(version, about, long_about = None)]
// pub struct ClientConfig {
//     #[arg(long, default_value_t = String::from("localhost"))]
//     pub host: String,

//     #[arg(long, default_value_t = 443)]
//     pub port: u16,
// }

// #[derive(Parser, Debug, Serialize, Deserialize)]
// #[command(version, about, long_about = None)]
// pub struct ServerConfig {
//     #[arg(long, default_value_t = String::from("localhost"))]
//     host: String,

//     #[arg(long, default_value_t = 443)]
//     port: u16,
// }

// pub fn get_client_config() -> ClientConfig {
//     // Parse CLI arguments. Override CLI config values with those in
//     // `Config.toml` and `APP_`-prefixed environment variables.
//     Figment::new()
//         .merge(Serialized::defaults(ClientConfig::parse()))
//         .merge(Toml::file("Config.toml"))
//         // .merge(Env::prefixed("APP_"))
//         .extract()?
// }

// pub fn get_server_config() -> ServerConfig {
//     // Parse CLI arguments. Override CLI config values with those in
//     // `Config.toml` and `APP_`-prefixed environment variables.
//     Figment::new()
//         .merge(Serialized::defaults(ServerConfig::parse()))
//         .merge(Toml::file("Config.toml"))
//         // .merge(Env::prefixed("APP_"))
//         .extract()?
// }
