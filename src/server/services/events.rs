use std::sync::Arc;

use discv5::{Discv5, Event};

/// Streams the discv5 server event stream.
pub async fn run(discv5: Arc<Discv5>) {
    let mut event_stream = discv5.event_stream().await.unwrap();
    loop {
        match event_stream.recv().await {
            Some(Event::SocketUpdated(addr)) => {
                log::info!("Nodes ENR socket address has been updated to: {:?}", addr);
            }
            Some(Event::Discovered(enr)) => {
                log::info!("A peer has been discovered: {}", enr.node_id());
            }
            Some(Event::UnverifiableEnr { enr, .. }) => {
                log::info!(
                    "A peer has been added to the routing table with enr: {}",
                    enr
                );
            }
            Some(Event::NodeInserted { node_id, .. }) => {
                log::info!(
                    "A peer has been added to the routing table with node_id: {}",
                    node_id
                );
            }
            Some(Event::SessionEstablished(enr, addr)) => {
                log::info!(
                    "A session has been established with peer: {} at address: {}",
                    enr,
                    addr
                );
            }
            Some(Event::TalkRequest(talk_request)) => {
                log::info!(
                    "A talk request has been received from peer: {}",
                    talk_request.node_id()
                );
            }
            _ => {}
        }
    }
}
