use clap::{Parser as ClapParser, Subcommand as ClapSubcommand};

use crate::utils::logging;

/// Discv5-cli Args
#[allow(missing_docs)]
#[derive(ClapParser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        short = 'v',
        long,
        default_value = "info",
        help = "Sets the logging verbosity level."
    )]
    pub log_level: logging::LogLevel,

    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

/// Discv5-cli Subcommand
#[derive(ClapSubcommand, Clone, Debug)]
#[allow(missing_docs)]
pub enum Subcommand {
    #[clap(name = "packet", about = "Performs packet operations")]
    Packet(crate::packet::Packet),
    #[clap(name = "request-enr", about = "Requests an ENR from a node")]
    RequestEnr(crate::request_enr::RequestEnr),
    #[clap(name = "server", about = "Runs a discv5 test server")]
    Server(crate::server::Server),
}
