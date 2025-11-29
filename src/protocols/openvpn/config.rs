use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ClientCli {

    #[arg(long, default_value_t = String::from("localhost"))]
    pub host: String,

    #[arg(long, default_value_t = 443)]
    pub port: u16,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ServerCli {

    #[arg(long, default_value_t = String::from("localhost"))]
    host: String,

    #[arg(long, default_value_t = 443)]
    port: u16
}