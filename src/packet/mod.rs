//! Handles the packet-based logic functions

use clap::ArgMatches;
use discv5::{enr, packet};
use log::{error, info};
use sha2::{Digest, Sha256};

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
        .unwrap_or_else(|| enr::NodeId::parse(&vec![0; 32]).expect("Valid 0 node-id"));

    info!("Using decoding node id: {}", node_id);

    let mut hasher = Sha256::new();
    hasher.input(node_id.raw());
    hasher.input(b"WHOAREYOU");
    let mut magic: packet::Magic = Default::default();
    magic.copy_from_slice(&hasher.result());

    match discv5::packet::Packet::decode(&packet_bytes, &magic) {
        Ok(p) => info!("Packet decoded: {:?}", p),
        Err(e) => error!("Packet failed to be decoded. Error: {:?}", e),
    }
}
