use futures::prelude::*;
use libp2p::core::{
    muxing::StreamMuxerBox, nodes::Substream, transport::dummy::DummyTransport, PeerId,
};
use libp2p::Swarm;
use libp2p_discv5::{enr, Discv5, Discv5Event};
use std::time::Duration;

type Libp2pStream = DummyTransport<(PeerId, StreamMuxerBox)>;

/// Starts a simple discv5 server which regularly queries for new peers and displays the results.
pub fn run_query_server(mut swarm: Swarm<Libp2pStream, Discv5<Substream<StreamMuxerBox>>>) {
    let target_random_node_id = enr::NodeId::random();
    swarm.find_node(target_random_node_id);

    // construct a 30 second interval to search for new peers.
    let mut query_interval = tokio::timer::Interval::new_interval(Duration::from_secs(10));

    // Kick it off!
    tokio::run(futures::future::poll_fn(move || -> Result<_, ()> {
        loop {
            // start a query if it's time to do so
            while let Ok(Async::Ready(_)) = query_interval.poll() {
                // pick a random node target
                let target_random_node_id = enr::NodeId::random();
                println!("Connected Peers: {}", swarm.connected_peers());
                println!("Searching for peers...");
                // execute a FINDNODE query
                swarm.find_node(target_random_node_id);
            }

            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(event)) => match event {
                    Discv5Event::FindNodeResult { closer_peers, .. } => {
                        if !closer_peers.is_empty() {
                            println!("Query Completed. Nodes found:");
                            for n in closer_peers {
                                println!("PeerId: {}", n);
                            }
                        } else {
                            println!("Query Completed. No peers found.")
                        }
                    }
                    _ => (),
                },
                Async::Ready(None) | Async::NotReady => break,
            }
        }

        Ok(Async::NotReady)
    }));
}
