use clap::{App, Arg, SubCommand};

pub fn start_cli<'a>() -> clap::ArgMatches<'a> {
    App::new("discv5-cli")
        .version("0.1.0")
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about(
            "Simple CLI tool for starting and debugging discv5 servers and packets. \
        This currently runs a discv5 server which regularly performs peer search queries",
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
        .get_matches()
}

fn server_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("server")
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
}

fn packet_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("packet")
        .about("Performs various packet encoding/decoding functions")
        .subcommand(
            SubCommand::with_name("decode")
                .about("decodes packets")
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
