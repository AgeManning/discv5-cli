use discv5::{enr, Discv5, Discv5ConfigBuilder};
use std::{
    convert::TryInto,
    net::{IpAddr, SocketAddr},
    process::exit,
    sync::Arc,
    time::Duration,
};

/// Services
pub mod services;

/// Bootstraps server peers from a file.
pub mod bootstrap;

/// ENR creation.
pub mod node;

/// Key construction for the server.
pub mod keys;

/// The [clap] cli command arguments for the server service.
pub mod command;
pub use command::*;

/// Run the query server
pub async fn run(server: &Server) {
    // The number of nodes required to come to consensus before our external IP is updated.
    let peer_update_min = server.peer_update_min;

    // Build the ENR
    let enr_key = keys::generate(server).unwrap();
    let enr = node::build(server, &enr_key).unwrap();

    let connect_enr = server.enr.as_ref().map(|enr| {
        enr.parse::<enr::Enr<enr::CombinedKey>>()
            .expect("Invalid base64 encoded ENR")
    });

    // Build the discv5 server using a default config
    let config = Discv5ConfigBuilder::new()
        .enr_peer_update_min(peer_update_min.try_into().unwrap())
        .build();
    let mut discv5 = Discv5::new(enr, enr_key, config).unwrap();

    // Connect to an ENR if allowed to search for p2p connections
    if !server.no_search {
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

    // Bootstrap the server peers
    if bootstrap::boostrap(&mut discv5, server.bootstrap.clone())
        .await
        .is_err()
    {
        log::error!("Failed to bootstrap discv5 server with bootstrap file")
    }

    // Start the discv5 server
    let listen_address = server
        .listen_address
        .parse::<IpAddr>()
        .expect("Invalid listening address");
    let listen_port = server.listen_port;
    discv5
        .start(SocketAddr::new(listen_address, listen_port))
        .await
        .expect("Should be able to start the server");
    log::info!("Server listening on {listen_address}:{listen_port}");

    let server_ref = Arc::new(discv5);
    if server.stats > 0 {
        services::stats::run(Arc::clone(&server_ref), None, server.stats);
    }

    if server.no_search {
        log::info!("Running without query service, press CTRL-C to exit.");
        let _ = tokio::signal::ctrl_c().await;
        exit(0);
    }

    // Match on the subcommand and run the appropriate service
    match server.service {
        ServerSubcommand::Query => {
            log::info!("Query service running...");
            services::query::run(server_ref, Duration::from_secs(server.break_time)).await;
        }
        ServerSubcommand::Events => {
            log::info!("Events service running...");
            services::events::run(server_ref).await;
        }
    }
}
