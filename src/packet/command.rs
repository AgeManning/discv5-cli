use clap::{Args, Subcommand as ClapSubcommand};

/// Packet Command
#[derive(Args, Clone, Debug)]
pub struct Packet {
    /// Packet Subcommand
    #[clap(subcommand)]
    pub subcommand: PacketSubcommand,
}

/// Packet Subcommand
#[derive(ClapSubcommand, Clone, Debug)]
pub enum PacketSubcommand {
    /// Decodes a packet.
    Decode(Decode),
}

/// Decode Options
#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decode {
    /// The packet to be decoded as a hex string.
    #[clap(
        short = 'p',
        long = "packet",
        help = "The packet to be decoded as a hex string."
    )]
    pub packet: String,
    /// The node id of the destination of this packet to determine WHOAREYOU packets as a hex string.
    #[clap(
        short = 'n',
        long = "nodeid",
        help = "The node id of the destination of this packet to determine WHOAREYOU packets as a hex string."
    )]
    pub node_id: String,
}
