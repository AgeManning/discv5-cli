use discv5::{enr, ListenConfig};
use libp2p_core::Multiaddr;

mod enr_ext;
use enr_ext::EnrExt;
use std::net::Ipv4Addr;

/// The [clap] cli command arguments for the request-enr service.
pub mod command;
pub use command::*;

/// Runs the request ENR command.
pub async fn run(req: &RequestEnr) {
    // Obtain the multiaddr
    let multiaddr = req
        .multiaddr
        .parse::<Multiaddr>()
        .expect("Invalid Multiaddr provided");

    // Set up a server to receive the response
    let listen_address = req
        .listen_address
        .parse::<Ipv4Addr>()
        .expect("Invalid listening address");

    let listen_port = req.listen_port;
    let listen_config = ListenConfig::from_ip(listen_address.into(), listen_port);
    let enr_key = enr::CombinedKey::generate_secp256k1();

    // Build a local ENR
    let enr = enr::Builder::default()
        .ip4(listen_address)
        .udp4(listen_port)
        .build(&enr_key)
        .unwrap();

    // Construct the discv5 service
    let config = discv5::ConfigBuilder::new(listen_config).build();
    let mut discv5: discv5::Discv5 = discv5::Discv5::new(enr, enr_key, config).unwrap();

    // Start the server
    discv5.start().await.unwrap();

    // Request the ENR
    log::info!("Requesting ENR for: {}", multiaddr);
    match discv5.request_enr(multiaddr.to_string()).await {
        Ok(enr) => print_enr(enr),
        Err(e) => log::error!("Failed to obtain ENR. Error: {}", e),
    }
}

// Print various information about the obtained ENR.
fn print_enr(enr: enr::Enr<enr::CombinedKey>) {
    log::info!("ENR Found:");
    log::info!("Sequence No:{}", enr.seq());
    log::info!("NodeId:{}", enr.node_id());
    log::info!("Libp2p PeerId:{}", enr.peer_id());
    if let Some(ip) = enr.ip4() {
        log::info!("IP:{:?}", ip);
    }
    if let Some(tcp) = enr.tcp4() {
        log::info!("TCP Port:{}", tcp);
    }
    if let Some(udp) = enr.udp4() {
        log::info!("UDP Port:{}", udp);
    }

    let multiaddrs = enr.multiaddr();
    if !multiaddrs.is_empty() {
        log::info!("Known multiaddrs:");
        for multiaddr in multiaddrs {
            log::info!("{}", multiaddr);
        }
    }
}
