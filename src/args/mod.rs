//! Command line arguments

use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
pub(super) struct Cli {
    /// Address and port to listen to connections
    #[arg(default_value = "0.0.0.0:3000")]
    pub address: SocketAddr
}
