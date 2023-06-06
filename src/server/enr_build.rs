use std::net::IpAddr;

use discv5::enr;

/// Builds an Enr from Server input cli args.
pub fn build(
    server: &super::command::Server,
    enr_key: &enr::CombinedKey,
) -> eyre::Result<enr::Enr<enr::CombinedKey>> {
    let mut builder = enr::EnrBuilder::new("v4");

    // Extract params from server config
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

    // if the -w switch is used, use the listen_address and port for the ENR
    if server.enr_default {
        if let Some(listen_address) = ipv4_address {
            builder.ip4(listen_address);
            builder.udp4(listen_port);
        }
        if let Some(listen_address) = ipv6_address {
            builder.ip6(listen_address);
            builder.udp6(listen_port_v6.unwrap_or(listen_port));
        }
    } else {
        // Logic for the ports. If the enr-port field is not set, use the listening port for both
        // v4 and v6 addresses.
        // If the enr-v4-port is set, use that for v4
        // If the enr-v6-port is set, use that for v6.
        if let Some(address_string) = &server.enr_addresses {
            for address in address_string.split(',') {
                match address
                    .parse::<IpAddr>()
                    .expect("Invalid listening address")
                {
                    IpAddr::V4(ip) => {
                        builder.ip4(ip);
                        builder.udp4(listen_port);
                    }
                    IpAddr::V6(ip) => {
                        builder.ip6(ip);
                        builder.udp6(listen_port_v6.unwrap_or(listen_port));
                    }
                }
            }
        }

        if let Some(udp4) = &server.enr_v4_port {
            builder.udp4(*udp4);
        }

        if let Some(udp6) = &server.enr_v6_port {
            builder.udp6(*udp6);
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
