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
}
