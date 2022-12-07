use std::net::IpAddr;

use discv5::enr;

/// Builds an Enr from Server input cli args.
pub fn build(
    server: &super::command::Server,
    enr_key: &enr::CombinedKey,
) -> eyre::Result<enr::Enr<enr::CombinedKey>> {
    let mut builder = enr::EnrBuilder::new("v4");

    // Extract params from server config
    let listen_address = server
        .listen_address
        .parse::<IpAddr>()
        .expect("Invalid listening address");
    let listen_port = server.listen_port;

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

    // Set the server sequence number.
    if let Some(seq_no_string) = &server.enr_seq_no {
        let seq_no = seq_no_string
            .parse::<u64>()
            .expect("Invalid sequence number, must be a uint");
        builder.seq(seq_no);
    }

    // Set the eth2 enr field.
    if let Some(eth2_string) = &server.enr_eth2 {
        let ssz_bytes = hex::decode(eth2_string).expect("Invalid eth2 hex bytes");
        builder.add_value("eth2", &ssz_bytes);
    }

    // Build
    let enr = builder.build(enr_key)?;

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

    Ok(enr)
}
