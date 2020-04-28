use discv5::{enr, Discv5, Discv5Event};
use futures::prelude::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
/// Starts a simple discv5 server which regularly queries for new peers and displays the results.
pub async fn run_query_server(mut discv5: Discv5) {
    // construct a 30 second interval to search for new peers.
    let mut query_interval = tokio::time::interval(Duration::from_secs(30));

    // Kick it off!
    future::poll_fn(move |cx: &mut Context| -> std::task::Poll<()> {
        loop {
            // start a query if it's time to do so
            if let Poll::Ready(Some(_)) = Pin::new(&mut query_interval).poll_next(cx) {
                // pick a random node target
                let target_random_node_id = enr::NodeId::random();
                println!("Connected Peers: {}", discv5.connected_peers());
                println!("Searching for peers...");
                // execute a FINDNODE query
                discv5.find_node(target_random_node_id);
            }

            match discv5.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => {
                    if let Discv5Event::FindNodeResult { closer_peers, .. } = event {
                        if !closer_peers.is_empty() {
                            println!("Query Completed. Nodes found:");
                            for n in closer_peers {
                                println!("Node: {}", n);
                            }
                        } else {
                            println!("Query Completed. No peers found.")
                        }
                    }
                }
                Poll::Ready(None) | Poll::Pending => return Poll::Pending,
            }
        }
    })
    .await
}
