use clap::ArgMatches;
use discv5::{enr, enr::CombinedKey, Discv5, Discv5ConfigBuilder};
use libp2p_core::Multiaddr;
use log::{error, info};
use std::net::{IpAddr, SocketAddr};

mod enr_ext;
use enr_ext::EnrExt;

pub async fn run(matches: &ArgMatches<'_>) {
    // Obtain the multiaddr
    let multiaddr = matches
        .value_of("multiaddr")
        .map(|m_addr| {
            m_addr
                .parse::<Multiaddr>()
                .expect("Invalid Multiaddr provided")
        })
        .expect("Multiaddr must be provided");

    // set up a server to receive the response
    let listen_address = "0.0.0.0"
        .parse::<IpAddr>()
        .expect("This is a valid address");
    let listen_port = 9001;
    let enr_key = CombinedKey::generate_secp256k1();

    // build a local ENR
    let enr = enr::EnrBuilder::new("v4")
        .ip(listen_address)
        .udp4(listen_port)
        .build(&enr_key)
        .unwrap();

    let listen_socket = SocketAddr::new(listen_address, listen_port);
    // default discv5 configuration
    let config = Discv5ConfigBuilder::new().build();
    // construct the discv5 service
    let mut discv5 = Discv5::new(enr, enr_key, config).unwrap();

    // start the server
    discv5.start(listen_socket).await.unwrap();

    // Request the ENR
    info!("Requesting ENR for: {}", multiaddr);

    match discv5.request_enr(multiaddr.to_string()).await {
        Ok(enr) => print_enr(enr),
        Err(e) => error!("Failed to obtain ENR. Error: {}", e),
    }
}

// Print various information about the obtained ENR.
fn print_enr(enr: enr::Enr<CombinedKey>) {
    info!("ENR Found:");
    info!("Sequence No:{}", enr.seq());
    info!("NodeId:{}", enr.node_id());
    info!("Libp2p PeerId:{}", enr.peer_id());
    if let Some(ip) = enr.ip4() {
        info!("IP:{:?}", ip);
    }
    if let Some(tcp) = enr.tcp4() {
        info!("TCP Port:{}", tcp);
    }
    if let Some(udp) = enr.udp4() {
        info!("UDP Port:{}", udp);
    }

    let multiaddrs = enr.multiaddr();
    if !multiaddrs.is_empty() {
        info!("Known multiaddrs:");
        for multiaddr in multiaddrs {
            info!("{}", multiaddr);
        }
    }
}
