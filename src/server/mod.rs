use discv5::{enr, ConfigBuilder, Discv5, ListenConfig};
use std::{
    convert::TryInto,
    net::{IpAddr, SocketAddrV4, SocketAddrV6},
    process::exit,
    sync::Arc,
    time::Duration,
};

/// Services
pub mod services;

/// Bootstraps server peers from a file.
pub mod bootstrap;

/// ENR creation.
pub mod enr_build;

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
    let enr = enr_build::build(server, &enr_key).unwrap();

    let connect_enr = server.enr.as_ref().map(|enr| {
        enr.parse::<enr::Enr<enr::CombinedKey>>()
            .expect("Invalid base64 encoded ENR")
    });

    let mut ipv4_address = None;
    let mut ipv6_address = None;
    for address in server.listen_addresses.split(',') {
        match address
            .parse::<IpAddr>()
            .expect("Invalid listening address")
        {
            IpAddr::V4(ip) => ipv4_address = Some(ip),
            IpAddr::V6(ip) => ipv6_address = Some(ip),
        }
    }

    let listen_port = server.listen_port;
    let listen_port_v6 = server.listen_port_v6;

    let listen_config = ListenConfig::from_two_sockets(
        ipv4_address.map(|v| SocketAddrV4::new(v, listen_port)),
        ipv6_address.map(|v| SocketAddrV6::new(v, listen_port_v6.unwrap_or(listen_port), 0, 0)),
    );

    log::info!("Server listening on {:?}", listen_config);
    // Build the discv5 server using a default config
    let config = ConfigBuilder::new(listen_config)
        .request_timeout(Duration::from_secs(3))
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
    discv5
        .start()
        .await
        .expect("Should be able to start the server");

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
