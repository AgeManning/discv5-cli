use discv5::{enr, ConnectionDirection, ConnectionState, Discv5};
use log::info;
use std::collections::HashMap;
use std::time::Duration;

/// Starts a simple discv5 server which regularly queries for new peers and displays the results.
pub async fn run_query_server(mut discv5: Discv5, break_time: Duration, stats: bool) {
    loop {
        info!("Searching for peers...");
        // pick a random node target
        let target_random_node_id = enr::NodeId::random();
        match discv5.find_node(target_random_node_id).await {
            Err(e) => println!("Find Node result failed: {:?}", e),
            Ok(found_enrs) => {
                info!("Query Completed. Nodes found: {}", found_enrs.len());
                for enr in found_enrs {
                    info!("Node: {}", enr.node_id());
                }
            }
        }

        // If stats are requested, print some table stats.
        if stats {
            print_stats(&mut discv5);
        }

        tokio::time::sleep(break_time).await;
        info!("Connected Peers: {}", discv5.connected_peers());
    }
}

fn print_stats(discv5: &mut Discv5) {
    let table_entries = discv5.table_entries();
    let self_id: discv5::Key<_> = discv5.local_enr().node_id().into();

    let mut bucket_values = HashMap::new();

    // Reconstruct the buckets
    for (node_id, enr, status) in table_entries {
        let key: discv5::Key<_> = node_id.into();
        let bucket_no = key.log2_distance(&self_id);
        if let Some(bucket_no) = bucket_no {
            bucket_values
                .entry(bucket_no)
                .or_insert_with(|| Vec::new())
                .push((enr, status));
        }
    }

    // Build some stats
    for (bucket, entries) in bucket_values {
        let mut connected_peers = 0;
        let mut connected_incoming_peers = 0;
        let mut connected_outgoing_peers = 0;
        let mut disconnected_peers = 0;

        for (_enr, status) in entries {
            match (status.state, status.direction) {
                (ConnectionState::Connected, ConnectionDirection::Incoming) => {
                    connected_peers += 1;
                    connected_incoming_peers += 1;
                }
                (ConnectionState::Connected, ConnectionDirection::Outgoing) => {
                    connected_peers += 1;
                    connected_outgoing_peers += 1;
                }
                (ConnectionState::Disconnected, _) => {
                    disconnected_peers += 1;
                }
            }
        }

        info!("Bucket {} statistics: Connected peers: {} (Incoming: {}, Outgoing: {}), Disconnected Peers: {}", bucket, connected_peers, connected_incoming_peers, connected_outgoing_peers, disconnected_peers);
    }
}
