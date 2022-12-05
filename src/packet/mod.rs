//! Handles the packet-based logic functions

use clap::ArgMatches;
use discv5::{enr, packet::Packet};
use log::{error, info};

/// Decodes a packet based on the CLI options.
pub fn decode(matches: &ArgMatches) {
    let packet_bytes_string = matches
        .value_of("packet")
        .expect("A <packet> must be supplied");

    let packet_bytes = hex::decode(packet_bytes_string).expect("Packet bytes must be valid hex");

    let node_id = matches
        .value_of("node-id")
        .map(|bytes| {
            enr::NodeId::parse(&hex::decode(bytes).expect("Node Id must be valid hex bytes"))
                .expect("Must be a valid node-id")
        })
        .unwrap_or_else(|| enr::NodeId::parse(&[0; 32]).expect("Valid 0 node-id"));

    info!("Using decoding node id: {}", node_id);

    match Packet::decode(&node_id, &packet_bytes) {
        Ok(p) => info!("Packet decoded: {:?}", p),
        Err(e) => error!("Packet failed to be decoded. Error: {:?}", e),
    }
}
