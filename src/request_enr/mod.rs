use discv5::enr;
use libp2p_core::Multiaddr;

mod enr_ext;
use enr_ext::EnrExt;

pub async fn run(matches: &clap::ArgMatches<'_>) {
    // Obtain the multiaddr
    let multiaddr = matches
        .value_of("multiaddr")
        .map(|m_addr| {
            m_addr
                .parse::<Multiaddr>()
                .expect("Invalid Multiaddr provided")
        })
        .expect("Multiaddr must be provided");

    // Set up a server to receive the response
    let listen_address = "0.0.0.0"
        .parse::<std::net::IpAddr>()
        .expect("This is a valid address");
    let listen_port = 9001;
    let enr_key = enr::CombinedKey::generate_secp256k1();

    // Build a local ENR
    let enr = enr::EnrBuilder::new("v4")
        .ip(listen_address)
        .udp4(listen_port)
        .build(&enr_key)
        .unwrap();

    // Construct the discv5 service
    let listen_socket = std::net::SocketAddr::new(listen_address, listen_port);
    let config = discv5::Discv5ConfigBuilder::new().build();
    let mut discv5 = discv5::Discv5::new(enr, enr_key, config).unwrap();

    // Start the server
    discv5.start(listen_socket).await.unwrap();

    // Request the ENR
    log::info!("Requesting ENR for: {}", multiaddr);
    match discv5.request_enr(multiaddr.to_string()).await {
        Ok(enr) => print_enr(enr),
        Err(e) => log::error!("Failed to obtain ENR. Error: {}", e),
    }
}

// Print various information about the obtained ENR.
<<<<<<< HEAD
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
=======
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
>>>>>>> 868b693f7c98eebcd461f7f1d8ed584a0c46ac47
    }

    let multiaddrs = enr.multiaddr();
    if !multiaddrs.is_empty() {
        log::info!("Known multiaddrs:");
        for multiaddr in multiaddrs {
            log::info!("{}", multiaddr);
        }
    }
}
