use clap::{Args, Subcommand as ClapSubcommand};

/// Server Subcommand
#[derive(ClapSubcommand, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ServerSubcommand {
    /// Queries random node ids.
    #[default]
    Query,
    /// Prints the event stream.
    Events,
}

/// Server Command
#[derive(Args, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Server {
    /// The service to run once the server is started.
    #[clap(subcommand)]
    pub service: ServerSubcommand,
    /// Specifies the listening address of the server.
    #[clap(
        short = 'l',
        long = "listen-address",
        help = "Specifies the listening address of the server.",
        default_value = "0.0.0.0"
    )]
    pub listen_address: String,
    /// Specifies the listening UDP port of the server.
    #[clap(
        short = 'p',
        long = "listen-port",
        help = "Specifies the listening UDP port of the server.",
        default_value = "9000"
    )]
    pub listen_port: u16,
    /// Specifies the IP address of the ENR record. Not specifying this results in an ENR with no IP field, unless the -w switch is used.
    #[clap(
        short = 'i',
        long = "enr-address",
        help = "Specifies the IP address of the ENR record. Not specifying this results in an ENR with no IP field, unless the -w switch is used."
    )]
    pub enr_address: Option<String>,
    /// Specifies the UDP port of the ENR record. Not specifying this results in an ENR with no UDP field, unless the -w switch is used.
    #[clap(
        short = 'u',
        long = "enr-port",
        help = "Specifies the UDP port of the ENR record. Not specifying this results in an ENR with no UDP field, unless the -w switch is used."
    )]
    pub enr_port: Option<u16>,
    /// Specifies the ENR sequence number when creating the ENR.
    #[clap(
        short = 'q',
        long = "enr-seq-no",
        help = "Specifies the ENR sequence number when creating the ENR."
    )]
    pub enr_seq_no: Option<String>,
    /// Specifies the Eth2 field as ssz encoded hex bytes.
    #[clap(
        short = 'd',
        long = "enr-eth2",
        help = "Specifies the Eth2 field as ssz encoded hex bytes."
    )]
    pub enr_eth2: Option<String>,
    /// The Enr IP address and port will be the same as the specified listening address and port.
    #[clap(
        short = 'w',
        long = "enr-default",
        help = "The Enr IP address and port will be the same as the specified listening address and port."
    )]
    pub enr_default: bool,
    /// Use a fixed static key (hard-coded). This is primarily for debugging.
    #[clap(
        short = 'k',
        long = "static-key",
        help = "Use a fixed static key (hard-coded). This is primarily for debugging."
    )]
    pub static_key: bool,
    /// Specify a secp256k1 private key (hex encoded) to use for the nodes identity.
    #[clap(
        short = 't',
        long = "secp256k1-key",
        help = "Specify a secp256k1 private key (hex encoded) to use for the nodes identity."
    )]
    pub secp256k1_key: Option<String>,
    /// A base64 ENR that this node will initially connect to.
    #[clap(
        short = 'e',
        long = "enr",
        allow_hyphen_values = true,
        help = "A base64 ENR that this node will initially connect to."
    )]
    pub enr: Option<String>,
    /// The minimum number of peers required to update the IP address. Cannot be less than 2.
    #[clap(
        short = 'n',
        long = "peer-update-min",
        help = "The minimum number of peers required to update the IP address. Cannot be less than 2.",
        default_value = "2"
    )]
    pub peer_update_min: u64,
    /// The time to wait between successive searches. Default is 10 seconds.
    #[clap(
        short = 'b',
        long = "break-time",
        help = "The time to wait between successive searches. Default is 10 seconds.",
        default_value = "10"
    )]
    pub break_time: u64,
    /// Displays statistics on the local routing table.
    #[clap(
        short = 's',
        long = "stats",
        help = "Displays statistics on the local routing table.",
        default_value = "10"
    )]
    pub stats: u64,
    /// Prevents the server from doing any peer searches.
    #[clap(
        short = 'x',
        long = "no-search",
        help = "Prevents the server from doing any peer searches."
    )]
    pub no_search: bool,
    /// Bootstraps the server peers
    #[clap(
        short = 'o',
        long = "bootstrap",
        help = "Bootstraps the server peers from a specified file."
    )]
    pub bootstrap: Option<String>,
}
