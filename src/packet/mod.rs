//! Handles the packet-based logic functions

/// The [clap] cli command arguments for the packet service.
pub mod command;
pub use command::*;

/// Decodes a packet based on the CLI options.
pub fn decode(decode: &Decode) {
    let packet_bytes = hex::decode(&decode.packet).expect("Packet bytes must be valid hex");

    let node_id = discv5::enr::NodeId::parse(
        &hex::decode(&decode.node_id).expect("Node Id must be valid hex bytes"),
    )
    .expect("Must be a valid node-id");

    log::info!("Using decoding node id: {}", node_id);

    match discv5::packet::Packet::decode::<discv5::DefaultProtocolId>(&node_id, &packet_bytes) {
        Ok(p) => log::info!("Packet decoded: {:?}", p),
        Err(e) => log::error!("Packet failed to be decoded. Error: {:?}", e),
    }
}
