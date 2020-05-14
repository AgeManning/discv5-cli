use discv5::{enr, Discv5, Discv5Event};
use futures::prelude::*;
use log::info;
use std::time::Duration;
/// Starts a simple discv5 server which regularly queries for new peers and displays the results.
pub async fn run_query_server(mut discv5: Discv5) {
    // construct a 30 second interval to search for new peers.
    let mut query_interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        tokio::select! {
            _ = query_interval.next() => {
                // pick a random node target
                let target_random_node_id = enr::NodeId::random();
                println!("Connected Peers: {}", discv5.connected_peers());
                println!("Searching for peers...");
                // execute a FINDNODE query
                discv5.find_node(target_random_node_id);
            }
            Some(event) = discv5.next() => {
                    if let Discv5Event::FindNodeResult { closer_peers, .. } = event {
                        if !closer_peers.is_empty() {
                            info!("Query Completed. Nodes found:");
                            for n in closer_peers {
                                info!("Node: {}", n);
                            }
                        } else {
                            info!("Query Completed. No peers found.")
                        }
                    }
                }
        }
    }
}
