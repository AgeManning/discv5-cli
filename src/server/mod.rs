pub mod query_server;
use clap::ArgMatches;
use discv5::{enr, enr::k256, enr::CombinedKey, Discv5, Discv5ConfigBuilder};
use log::{info, warn};
use std::net::{IpAddr, SocketAddr};

// handle a query server

pub async fn run(server_matches: &ArgMatches<'_>) {
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

    let no_search = server_matches.is_present("no-search");

    // the number of nodes required to come to consensus before our external IP is updated.
    let peer_update_min = server_matches
        .value_of("peer-update-min")
        .expect("There must be a value for nodes-to-update")
        .parse::<usize>()
        .expect("Nodes to implement must be an unsigned integer");

    let time_between_searches = std::time::Duration::from_secs(
        server_matches
            .value_of("break-time")
            .expect("This value must exist")
            .parse::<u64>()
            .expect("The break time must be a uint."),
    );

    let stats = server_matches.is_present("stats");

    // create the key pair
    let enr_key = if server_matches.is_present("static-key") {
        // A fixed key for testing
        let raw_key = vec![
            183, 28, 113, 166, 126, 17, 119, 173, 78, 144, 22, 149, 225, 180, 185, 238, 23, 174,
            22, 198, 102, 141, 49, 62, 172, 47, 150, 219, 205, 163, 242, 145,
        ];
        let secret_key = k256::ecdsa::SigningKey::from_bytes(&raw_key).unwrap();
        CombinedKey::from(secret_key)
    } else if let Some(string_key) = server_matches.value_of("secp256k1-key") {
        let raw_key = hex::decode(string_key).expect("Invalid hex bytes for secp256k1 key");
        let secret_key =
            k256::ecdsa::SigningKey::from_bytes(&raw_key).expect("Invalid secp256k1 key");
        CombinedKey::from(secret_key)
    } else {
        CombinedKey::generate_secp256k1()
    };

    // build the ENR
    let enr = {
        let mut builder = enr::EnrBuilder::new("v4");

        // if the -w switch is used, use the listen_address and port for the ENR
        if server_matches.is_present("enr_default") {
            builder.ip(listen_address);
            builder.udp4(listen_port);
        } else {
            if let Some(address_string) = server_matches.value_of("enr-address") {
                let enr_address = address_string
                    .parse::<IpAddr>()
                    .expect("Invalid enr-address");
                builder.ip(enr_address);
            }
            if let Some(port_string) = server_matches.value_of("enr-port") {
                let enr_port = port_string.parse::<u16>().expect("Invalid enr-port");
                builder.udp4(enr_port);
            }
        }

        if let Some(seq_no_string) = server_matches.value_of("enr-seq-no") {
            let seq_no = seq_no_string
                .parse::<u64>()
                .expect("Invalid sequence number, must be a uint");
            builder.seq(seq_no);
        }

        if let Some(eth2_string) = server_matches.value_of("enr-eth2") {
            let ssz_bytes = hex::decode(eth2_string).expect("Invalid eth2 hex bytes");
            builder.add_value("eth2", &ssz_bytes);
        }

        builder.build(&enr_key).unwrap()
    };

    // if the ENR is useful print it
    info!("Node Id: {}", enr.node_id());
    if enr.udp4_socket().is_some() {
        info!("Base64 ENR: {}", enr.to_base64());
        info!("ip: {}, udp port:{}", enr.ip4().unwrap(), enr.udp4().unwrap());
    } else {
        warn!("ENR is not printed as no IP:PORT was specified");
    }

    let connect_enr = server_matches.value_of("enr").map(|enr| {
        enr.parse::<enr::Enr<enr::CombinedKey>>()
            .expect("Invalid base64 encoded ENR")
    });

    // default discv5 configuration
    let config = Discv5ConfigBuilder::new()
        .enr_peer_update_min(peer_update_min)
        .build();
    // construct the discv5 service
    let mut discv5 = Discv5::new(enr, enr_key, config).unwrap();

    // try to connect to an ENR if specified
    if !no_search {
        if let Some(connect_enr) = connect_enr {
            info!(
                "Connecting to ENR. ip: {:?}, udp_port: {:?},  tcp_port: {:?}",
                connect_enr.ip4(),
                connect_enr.udp4(),
                connect_enr.tcp4()
            );
            if let Err(e) = discv5.add_enr(connect_enr) {
                warn!("ENR not added: {:?}", e);
            }
        }
    }

    // start the server
    discv5
        .start(SocketAddr::new(listen_address, listen_port))
        .await
        .expect("Should be able to start the server");

    // start the query
    if !no_search {
        query_server::run_query_server(discv5, time_between_searches, stats).await;
    } else {
        info!("Server running...");
        let _ = tokio::signal::ctrl_c().await;
    }
}
