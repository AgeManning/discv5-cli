use std::sync::Arc;

use discv5::{Discv5, Discv5Event};

/// Streams the discv5 server event stream.
pub async fn run(discv5: Arc<Discv5>) {
    let mut event_stream = discv5.event_stream().await.unwrap();
    loop {
        match event_stream.recv().await {
            Some(Discv5Event::SocketUpdated(addr)) => {
                log::info!("Nodes ENR socket address has been updated to: {:?}", addr);
            }
            Some(Discv5Event::Discovered(enr)) => {
                log::info!("A peer has been discovered: {}", enr.node_id());
            }
            Some(discv5::Discv5Event::EnrAdded { enr, .. }) => {
                log::info!(
                    "A peer has been added to the routing table with enr: {}",
                    enr
                );
            }
            Some(discv5::Discv5Event::NodeInserted { node_id, .. }) => {
                log::info!(
                    "A peer has been added to the routing table with node_id: {}",
                    node_id
                );
            }
            Some(discv5::Discv5Event::SessionEstablished(enr, addr)) => {
                log::info!(
                    "A session has been established with peer: {} at address: {}",
                    enr,
                    addr
                );
            }
            Some(discv5::Discv5Event::TalkRequest(talk_request)) => {
                log::info!(
                    "A talk request has been received from peer: {}",
                    talk_request.node_id()
                );
            }
            _ => {}
        }
    }
}
