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

use discv5::{enr, enr::CombinedKey, Discv5, Discv5Config};
use std::net::{IpAddr, SocketAddr};
mod cli;
mod packet;
mod server;

#[tokio::main]
async fn main() {
    let cli_matches = cli::start_cli();

    // Parse the CLI parameters.
    if let Some(server_matches) = cli_matches.subcommand_matches("server") {
        let listen_address = server_matches
            .value_of("listen-address")
            .expect("required parameter")
            .parse::<IpAddr>()
            .expect("Invalid listening address");

        let listen_port = server_matches
            .value_of("listen-port")
            .expect("required parameter")
            .parse::<u16>()
            .expect("Invalid listening port");

        // create the key pair
        let enr_key = if server_matches.is_present("static-key") {
            // A fixed key for testing
            let raw_key = vec![
                183, 28, 113, 166, 126, 17, 119, 173, 78, 144, 22, 149, 225, 180, 185, 238, 23,
                174, 22, 198, 102, 141, 49, 62, 172, 47, 150, 219, 205, 163, 242, 145,
            ];
            CombinedKey::from(secp256k1::SecretKey::parse_slice(&raw_key).unwrap())
        } else {
            CombinedKey::generate_secp256k1()
        };

        // build the ENR
        let enr = {
            let mut builder = enr::EnrBuilder::new("v4");

            // if the -w switch is used, use the listen_address and port for the ENR
            if server_matches.is_present("enr_default") {
                builder.ip(listen_address);
                builder.udp(listen_port);
            } else {
                if let Some(address_string) = server_matches.value_of("enr-address") {
                    let enr_address = address_string
                        .parse::<IpAddr>()
                        .expect("Invalid enr-address");
                    builder.ip(enr_address);
                }
                if let Some(port_string) = server_matches.value_of("enr-port") {
                    let enr_port = port_string.parse::<u16>().expect("Invalid enr-port");
                    builder.udp(enr_port);
                }
            }
            builder.build(&enr_key).unwrap()
        };

        // if the ENR is useful print it
        println!("Node Id: {}", enr.node_id());
        if enr.udp_socket().is_some() {
            println!("Base64 ENR: {}", enr.to_base64());
            println!("ip: {}, udp port:{}", enr.ip().unwrap(), enr.udp().unwrap());
        } else {
            println!("ENR is not printed as no IP:PORT was specified");
        }

        let connect_enr = server_matches.value_of("enr").map(|enr| {
            enr.parse::<enr::Enr<enr::CombinedKey>>()
                .expect("Invalid base64 encoded ENR")
        });

        let listen_socket = SocketAddr::new(listen_address, listen_port);

        // default discv5 configuration
        let config = Discv5Config::default();
        // construct the discv5 service
        let mut discv5 = Discv5::new(enr, enr_key, config, listen_socket).unwrap();

        // try to connect to an ENR if specified
        if let Some(connect_enr) = connect_enr {
            println!(
                "Connecting to ENR. ip: {:?}, udp_port: {:?},  tcp_port: {:?}",
                connect_enr.ip(),
                connect_enr.udp(),
                connect_enr.tcp()
            );
            if let Err(e) = discv5.add_enr(connect_enr) {
                println!("ENR not added: {:?}", e);
            }
        }

        // start the query
        server::query_server::run_query_server(discv5).await;
    } else if let Some(packet_matches) = cli_matches.subcommand_matches("packet") {
        if let Some(decode_matches) = packet_matches.subcommand_matches("decode") {
            packet::decode(decode_matches);
        } else {
            // Currently no encode sub command
            println!("A packet subcommand must be supplied. See --help for options");
            return;
        }
    } else {
        // No subcommand supplied
        println!("A subcommand must be supplied. See --help for options");
        return;
    }
}
