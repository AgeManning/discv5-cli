use discv5::{enr, Discv5};
use std::{sync::Arc, time::Duration};

/// Regularly queries for new peers.
pub async fn run(discv5: Arc<Discv5>, break_time: Duration) {
    loop {
        log::info!("Searching for peers...");
        let target_random_node_id = enr::NodeId::random();
        // let unlocked_server = discv5.lock().unwrap();
        match discv5.find_node(target_random_node_id).await {
            Err(e) => log::warn!("Find Node result failed: {e:?}"),
            Ok(found_enrs) => {
                log::info!("Query Completed. Nodes found: {}", found_enrs.len());
                for enr in found_enrs {
                    log::info!("Node: {}", enr.node_id());
                }
            }
        }
        log::info!("Connected Peers: {}", discv5.connected_peers());
        tokio::time::sleep(break_time).await;
    }
}
