use discv5::{enr, Discv5};
use log::info;

/// Starts a simple discv5 server which regularly queries for new peers and displays the results.
pub async fn run_query_server(mut discv5: Discv5) {
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
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        info!("Connected Peers: {}", discv5.connected_peers());
    }
}
