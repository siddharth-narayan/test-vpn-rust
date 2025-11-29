use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ClientCli {

    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    port: Option<u16>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ServerCli {

    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    port: Option<u16>,
}