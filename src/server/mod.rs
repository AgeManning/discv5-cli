use discv5::{enr, enr::k256, enr::CombinedKey, Discv5, Discv5ConfigBuilder};
use std::{
    convert::TryInto,
    net::{IpAddr, SocketAddr},
    time::Duration,
};

mod query_server;

/// The [clap] cli command arguments for the server service.
pub mod command;
pub use command::*;

/// Run the query server
pub async fn run(server: &Server) {
    let listen_address = server
        .listen_address
        .parse::<IpAddr>()
        .expect("Invalid listening address");

    let listen_port = server.listen_port;
    let no_search = server.no_search;

    // The number of nodes required to come to consensus before our external IP is updated.
    let peer_update_min = server.peer_update_min;

    let time_between_searches = Duration::from_secs(server.break_time);

    let stats = server.stats;

    // Create the key pair
    let enr_key = if server.static_key {
        // A fixed key for testing
        let raw_key = vec![
            183, 28, 113, 166, 126, 17, 119, 173, 78, 144, 22, 149, 225, 180, 185, 238, 23, 174,
            22, 198, 102, 141, 49, 62, 172, 47, 150, 219, 205, 163, 242, 145,
        ];
        let secret_key = k256::ecdsa::SigningKey::from_bytes(&raw_key).unwrap();
        CombinedKey::from(secret_key)
    } else if let Some(string_key) = &server.secp256k1_key {
        let raw_key = hex::decode(string_key).expect("Invalid hex bytes for secp256k1 key");
        let secret_key =
            k256::ecdsa::SigningKey::from_bytes(&raw_key).expect("Invalid secp256k1 key");
        CombinedKey::from(secret_key)
    } else {
        CombinedKey::generate_secp256k1()
    };

    // Build the ENR
    let enr = {
        let mut builder = enr::EnrBuilder::new("v4");

        // if the -w switch is used, use the listen_address and port for the ENR
        if server.enr_default {
            builder.ip(listen_address);
            builder.udp4(listen_port);
        } else {
            if let Some(address_string) = &server.enr_address {
                let enr_address = address_string
                    .parse::<IpAddr>()
                    .expect("Invalid enr-address");
                builder.ip(enr_address);
            }
            if let Some(enr_port) = server.enr_port {
                builder.udp4(enr_port);
            }
        }

        if let Some(seq_no_string) = &server.enr_seq_no {
            let seq_no = seq_no_string
                .parse::<u64>()
                .expect("Invalid sequence number, must be a uint");
            builder.seq(seq_no);
        }

        if let Some(eth2_string) = &server.enr_eth2 {
            let ssz_bytes = hex::decode(eth2_string).expect("Invalid eth2 hex bytes");
            builder.add_value("eth2", &ssz_bytes);
        }

        builder.build(&enr_key).unwrap()
    };

    // If the ENR is useful print it
    log::info!("Node Id: {}", enr.node_id());
    if enr.udp4_socket().is_some() {
        log::info!("Base64 ENR: {}", enr.to_base64());
        log::info!(
            "ip: {}, udp port:{}",
            enr.ip4().unwrap(),
            enr.udp4().unwrap()
        );
    } else {
        log::warn!("ENR is not printed as no IP:PORT was specified");
    }

    let connect_enr = server.enr.as_ref().map(|enr| {
        enr.parse::<enr::Enr<enr::CombinedKey>>()
            .expect("Invalid base64 encoded ENR")
    });

    // default discv5 configuration
    let config = Discv5ConfigBuilder::new()
        .enr_peer_update_min(peer_update_min.try_into().unwrap())
        .build();
    // construct the discv5 service
    let mut discv5 = Discv5::new(enr, enr_key, config).unwrap();

    // try to connect to an ENR if specified
    if !no_search {
        if let Some(connect_enr) = connect_enr {
            log::info!(
                "Connecting to ENR. ip: {:?}, udp_port: {:?},  tcp_port: {:?}",
                connect_enr.ip4(),
                connect_enr.udp4(),
                connect_enr.tcp4()
            );
            if let Err(e) = discv5.add_enr(connect_enr) {
                log::warn!("ENR not added: {:?}", e);
            }
        }
    }

    // Start the discv5 server
    discv5
        .start(SocketAddr::new(listen_address, listen_port))
        .await
        .expect("Should be able to start the server");

    // start the query
    if !no_search {
        query_server::run_query_server(discv5, time_between_searches, stats).await;
    } else {
        log::info!("Server running...");
        let _ = tokio::signal::ctrl_c().await;
    }
}
