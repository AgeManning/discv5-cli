use clap::{App, Arg};

pub fn start_cli<'a>() -> clap::ArgMatches<'a> {
    App::new("discv5-cli")
        .version("0.2.4")
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about(
            "Simple CLI tool for starting and debugging discv5 servers and packets. \
        This currently runs a discv5 server which regularly performs peer search queries.",
        )
        .arg(
            Arg::with_name("log-level")
                .value_name("level")
                .long("log-level")
                .short("v")
                .possible_values(&["trace", "debug", "info", "warn", "error"])
                .help("Specifies the listening address of the server.")
                .default_value("info")
                .takes_value(true),
        )
        .subcommand(server_cli())
        .subcommand(packet_cli())
        .subcommand(request_enr())
        .get_matches()
}

fn server_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("server")
        .about("Runs a discv5 test server")
        .arg(
            Arg::with_name("listen-address")
                .value_name("IP-ADDRESS")
                .long("listen-address")
                .short("l")
                .required(true)
                .help("Specifies the listening address of the server.")
                .default_value("0.0.0.0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("listen-port")
                .value_name("PORT")
                .long("listen-port")
                .short("p")
                .required(true)
                .help("Specifies the listening UDP port of the server.")
                .default_value("9000")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("enr-address")
                .value_name("IP-ADDRESS")
                .long("enr-address")
                .help("Specifies the IP address of the ENR record. Not specifying this results in an ENR with no IP field, unless the -w switch is used.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("enr-port")
                .value_name("PORT")
                .long("enr-port")
                .help("Specifies the UDP port of the ENR record. Not specifying this results in an ENR with no UDP field, unless the -w switch is used.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("enr-seq-no")
                .value_name("UINT")
                .long("enr-seq-no")
                .help("Specifies the ENR sequence number when creating the ENR.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("enr-eth2")
                .value_name("HEX_BYTES")
                .long("enr-eth2")
                .help("Specifies the Eth2 field as ssz encoded hex bytes.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("enr_default")
                .short("w")
                .help("The Enr IP address and port will be the same as the specified listening address and port.")
        )
        .arg(
            Arg::with_name("static-key")
            .long("static-key")
            .short("k")
                .help("Use a fixed static key (hard-coded). This is primarily for debugging.")
        )
        .arg(
            Arg::with_name("secp256k1-key")
            .long("secp256k1-key")
            .short("t")
            .help("Specify a secp256k1 private key (hex encoded) to use for the nodes identity.")
            .takes_value(true),
        )
        .arg(
            Arg::with_name("enr")
                .long("enr")
                .short("e")
                .value_name("BASE64-ENR")
                .allow_hyphen_values(true)
                .help("A base64 ENR that this node will initially connect to.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("peer-update-min")
                .long("update-nodes")
                .short("n")
                .value_name("INT")
                .default_value("2")
                .help("The minimum number of peers required to update the IP address. Cannot be less than 2.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("break-time")
                .long("break-time")
                .default_value("10")
                .help("The time to wait between successive searches. Default is 10 seconds.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stats")
                .long("stats")
                .default_value("10")
                .help("Displays statistics on the local routing table.")
        )
        .arg(
            Arg::with_name("no-search")
                .short("x")
                .help("Prevents the server from doing any peer searches.")
        )
}

fn packet_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("packet")
        .about("Performs various packet encoding/decoding functions")
        .subcommand(
            App::new("decode")
                .help("decodes packets")
                .arg(
                    Arg::with_name("packet")
                        .value_name("Packet")
                        .required(true)
                        .takes_value(true)
                        .help("The packet to be decoded as a hex string."),
                )
                .arg(
                    Arg::with_name("node_id")
                        .value_name("Node Id")
                        .takes_value(true)
                        .help("The node id of the destination of this packet to determine WHOAREYOU packets as a hex string."),
                ),
        )
}

fn request_enr<'a, 'b>() -> App<'a, 'b> {
    App::new("request-enr")
        .about("Requests the ENR of a multiaddr")
        .arg(
            Arg::with_name("multiaddr")
                .value_name("MULTIADDR")
                .takes_value(true)
                .help("The multiaddr of the node to request their ENR from"),
        )
}
