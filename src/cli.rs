use clap::{App, Arg};

pub fn start_cli<'a>() -> clap::ArgMatches<'a> {
    App::new("discv5-cli")
        .version("0.1.0")
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about("Simple CLI tool for starting and debugging discv5 servers. \
        This currently runs a discv5 server which regularly performs peer search queries")
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
        .get_matches()
}
