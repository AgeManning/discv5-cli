use clap::Args;

/// RequestEnr
#[derive(Args, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequestEnr {
    /// Multiaddr of the node to request the ENR from.
    #[clap(
        short = 'm',
        long = "multiaddr",
        help = "The multiaddr of the node to request their ENR from"
    )]
    pub multiaddr: String,
    /// Specifies the listening address of the server.
    #[clap(
        short = 'l',
        long = "listen-address",
        help = "Specifies the IPv4 listening address of the server.",
        default_value = "0.0.0.0"
    )]
    pub listen_address: String,
    /// Specifies the listening UDP port of the server.
    #[clap(
        short = 'p',
        long = "listen-port",
        help = "Specifies the listening UDP port of the server.",
        default_value = "9001"
    )]
    pub listen_port: u16,
}
