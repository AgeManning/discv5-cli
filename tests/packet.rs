use discv5::enr as discv5_enr;
use discv5::packet as discv5_packet;

#[test]
pub fn test_packet_decoding() {
    let source_node_id = discv5_enr::NodeId::random();
    let hex_source_node_id = hex::encode(source_node_id.raw());
    println!("Source node id: {hex_source_node_id:?}");

    let dest_node_id = discv5_enr::NodeId::random();
    let hex_dest_node_id = hex::encode(dest_node_id.raw());
    println!("Destination node id: {hex_dest_node_id:?}");

    let packet = discv5_packet::Packet::new_random(&source_node_id).unwrap();
    let encoded = hex::encode(packet.encode::<discv5::DefaultProtocolId>(&dest_node_id));
    println!("Packet as hex: 0x{encoded:?}");
}
