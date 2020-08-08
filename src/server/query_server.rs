use discv5::{enr, Discv5};
use log::info;

/// Starts a simple discv5 server which regularly queries for new peers and displays the results.
pub async fn run_query_server(mut discv5: Discv5) {
    loop {
        info!("Searching for peers...");
        // pick a random node target
        let target_random_node_id = enr::NodeId::random();
        // execute a FINDNODE query
        match discv5.find_node(target_random_node_id).await {
            Ok(found_nodes) => {
                if found_nodes.is_empty() {
                    info!("Query Completed. No peers found.")
                } else {
                    info!("Query Completed. Nodes found:");
                    for enr in found_nodes {
                        info!("Node: {}", enr)
                    }
                }
            }
            Err(error) => {
                info!("Error: {}", error);
            }
        };

        tokio::time::delay_for(std::time::Duration::from_secs(5)).await;
        info!("Connected Peers: {}", discv5.connected_peers());
    }
}
