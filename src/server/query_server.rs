use discv5::{enr, Discv5, Discv5Event};
use futures::prelude::*;
use log::info;
/// Starts a simple discv5 server which regularly queries for new peers and displays the results.
pub async fn run_query_server(mut discv5: Discv5) {
    info!("Searching for peers...");
    // pick a random node target
    let target_random_node_id = enr::NodeId::random();
    // execute a FINDNODE query
    discv5.find_node(target_random_node_id);

    loop {
        match discv5.next().await {
            Some(Discv5Event::FindNodeResult { closer_peers, .. }) => {
                if !closer_peers.is_empty() {
                    info!("Query Completed. Nodes found:");
                    for n in closer_peers {
                        info!("Node: {}", n);
                    }
                } else {
                    info!("Query Completed. No peers found.")
                }
                info!("Connected Peers: {}", discv5.connected_peers());
                info!("Searching for peers...");
                // pick a random node target
                let target_random_node_id = enr::NodeId::random();
                info!("Target NodeId: {}", target_random_node_id);
                // execute a FINDNODE query
                discv5.find_enr_predicate(target_random_node_id, |_| true, 16);
            }
            _ => {}
        }
    }
}
