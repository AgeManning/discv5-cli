#![doc=include_str!("../README.md")]

use clap::Parser;
use discv5_cli::{cli, packet, prelude::Packet};

#[tokio::main]
async fn main() {
    // Parse the command line arguments
    let cli = cli::Cli::parse();

    // Setup logging using the specified log level
    discv5_cli::utils::logging::construct_simple_logger(cli.log_level);

    // Run the appropriate command
    match cli.subcommand {
        Some(cli::Subcommand::Server(ref s)) => {
            discv5_cli::server::run(s).await;
        }
        Some(cli::Subcommand::RequestEnr(ref request_enr)) => {
            discv5_cli::request_enr::run(request_enr).await;
        }
        Some(cli::Subcommand::Packet(Packet { subcommand })) => match subcommand {
            packet::PacketSubcommand::Decode(ref decode) => {
                packet::decode(decode);
            }
        },
        _ => log::error!("Unable to parse command line arguments. See --help for options"),
    }
}
