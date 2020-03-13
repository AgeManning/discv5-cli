//! # discv5-cli
//!
//! This is a simple CLI utility for creating and debugging discovery v5 servers.
//!
//! ## Install
//!
//! This can be installed via cargo:
//!
//! ```bash
//! $ cargo install discv5-cli --version 0.1.0-alpha
//! ```
//!
//! ## Usage
//!
//! ```bash
//! Sigma Prime <contact@sigmaprime.io>
//! Simple CLI tool for starting and debugging discv5 servers. This currently runs a discv5 server which regularly performs
//! peer search queries
//!
//! USAGE:
//!     discv5-cli [FLAGS] [OPTIONS] --listen-address <IP-ADDRESS> --listen-port <PORT>
//!
//! FLAGS:
//!     -w                  The Enr IP address and port will be the same as the specified listening address and port.
//!     -h, --help          Prints help information
//!     -k, --static-key    Use a fixed static key (hard-coded). This is primarily for debugging.
//!     -V, --version       Prints version information
//!
//! OPTIONS:
//!     -e, --enr <BASE64-ENR>               A base64 ENR that this node will initially connect to.
//!         --enr-address <IP-ADDRESS>       Specifies the IP address of the ENR record. Not specifying this results in an
//!                                          ENR with no IP field, unless the -w switch is used.
//!         --enr-port <PORT>                Specifies the UDP port of the ENR record. Not specifying this results in an ENR
//!                                          with no UDP field, unless the -w switch is used.
//!     -l, --listen-address <IP-ADDRESS>    Specifies the listening address of the server. [default: 127.0.0.1]
//!     -p, --listen-port <PORT>             Specifies the listening UDP port of the server. [default: 9000]
//! ```
//!
//! ## Example
//!
//! ```bash
//! $ discv5-cli -l 127.0.0.1 -p 9001 -w -e -IS4QCs0BSKEvnX8om4rAAi7D2p2lwQ7LVpAeESY2ikm1b5dBOqJC7istWMVg06dy-I09C8NuZdodEFNxIiiolWwSWkBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCIyg
//! Node Id: 0x98b6..42f9
//! Peer Id: QmQJbNYRuLJo87Q77ZAj246NfPtctKsBbWtnTvJDLrGcw1
//! Base64 ENR: enr:-IS4QKKLuzCDjV4hjA2-77cO0Xjx5vLGODQKe_Fl6Qim8qHBMN4chtaLqv6Xz6BWv5hfVvn0d2G0dt94ZAG9OHska44BgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQLHT5xE1ZhCXfrOv3D66tPfDZxV57TaX0rky3uwRhyBe4N1ZHCCIyk
//! ip: 127.0.0.1, udp port:9001
//! Connecting to ENR. ip: Some(127.0.0.1), udp_port: Some(9000),  tcp_port: None
//! Query Completed. Nodes found:
//! PeerId: QmdFQ2frpcyD3DiaMcM6XP37X3xCAV81GoB3jRusAa7kxu
//! Connected Peers: 1
//! Searching for peers...
//! Query Completed. Nodes found:
//! PeerId: QmdFQ2frpcyD3DiaMcM6XP37X3xCAV81GoB3jRusAa7kxu
//! ```

use libp2p::core::identity;
use libp2p_discv5::{enr, Discv5, Discv5Config};
use std::convert::TryInto;
use std::net::{IpAddr, SocketAddr};
mod cli;
mod query_server;

fn main() {
    let cli_matches = cli::start_cli();

    // Parse the CLI parameters.

    let listen_address = cli_matches
        .value_of("listen-address")
        .expect("required parameter")
        .parse::<IpAddr>()
        .expect("Invalid listening address");

    let listen_port = cli_matches
        .value_of("listen-port")
        .expect("required parameter")
        .parse::<u16>()
        .expect("Invalid listening port");

    // create the key pair
    let keypair = if cli_matches.is_present("static-key") {
        // A fixed key for testing
        let raw_key = vec![
            183, 28, 113, 166, 126, 17, 119, 173, 78, 144, 22, 149, 225, 180, 185, 238, 23, 174,
            22, 198, 102, 141, 49, 62, 172, 47, 150, 219, 205, 163, 242, 145,
        ];
        let secret_key = identity::secp256k1::SecretKey::from_bytes(raw_key).unwrap();
        identity::Keypair::Secp256k1(identity::secp256k1::Keypair::from(secret_key))
    } else {
        identity::Keypair::generate_secp256k1()
    };

    // build an enr key from the libp2p key
    let enr_key = keypair.clone().try_into().unwrap();

    // build the ENR
    let enr = {
        let mut builder = enr::EnrBuilder::new("v4");

        // if the -w switch is used, use the listen_address and port for the ENR
        if cli_matches.is_present("enr_default") {
            builder.ip(listen_address);
            builder.udp(listen_port);
        } else {
            if let Some(address_string) = cli_matches.value_of("enr-address") {
                let enr_address = address_string
                    .parse::<IpAddr>()
                    .expect("Invalid enr-address");
                builder.ip(enr_address);
            }
            if let Some(port_string) = cli_matches.value_of("enr-port") {
                let enr_port = port_string.parse::<u16>().expect("Invalid enr-port");
                builder.udp(enr_port);
            }
        }
        builder.build(&enr_key).unwrap()
    };

    // if the ENR is useful print it
    println!("Node Id: {}", enr.node_id());
    println!("Peer Id: {}", enr.peer_id());
    if enr.udp_socket().is_some() {
        println!("Base64 ENR: {}", enr.to_base64());
        println!("ip: {}, udp port:{}", enr.ip().unwrap(), enr.udp().unwrap());
    } else {
        println!("ENR is not printed as no IP:PORT was specified");
    }

    let connect_enr = cli_matches.value_of("enr").map(|enr| {
        enr.parse::<enr::Enr<enr::CombinedKey>>()
            .expect("Invalid base64 encoded ENR")
    });

    let listen_socket = SocketAddr::new(listen_address, listen_port);

    // unused transport for building a swarm
    let transport = libp2p::core::transport::dummy::DummyTransport::new();
    // default discv5 configuration
    let config = Discv5Config::default();
    // construct the discv5 swarm, initializing an unused transport layer
    let discv5 = Discv5::new(enr, keypair.clone(), config, listen_socket).unwrap();
    let mut swarm = libp2p::Swarm::new(transport, discv5, keypair.public().into_peer_id());

    // try to connect to an ENR if specified
    if let Some(connect_enr) = connect_enr {
        println!(
            "Connecting to ENR. ip: {:?}, udp_port: {:?},  tcp_port: {:?}",
            connect_enr.ip(),
            connect_enr.udp(),
            connect_enr.tcp()
        );
        swarm.add_enr(connect_enr);
    }

    // start the discv5 server
    query_server::run_query_server(swarm);
}
