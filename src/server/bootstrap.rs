use std::{fs::File, io::BufReader, str::FromStr};

use discv5::{Discv5, Enr};
use serde::{Deserialize, Serialize};

/// The top level bootstrap object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct BootstrapStore {
    /// The list of bootstrap nodes.
    pub data: Vec<BootstrapNode>,
}

/// A bootstrap node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct BootstrapNode {
    /// The node's peer id.
    pub peer_id: String,
    /// The node's ENR.
    pub enr: String,
    /// The last seen p2p address.
    pub last_seen_p2p_address: String,
    /// The node's state.
    pub state: State,
    /// The node's direction.
    pub direction: Direction,
}

/// The direction of node connection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Direction {
    /// An inbound connection
    #[serde(rename = "inbound")]
    Inbound,
    /// An outbound connection
    #[serde(rename = "outbound")]
    Outbound,
}

/// The connection state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum State {
    /// Connected state
    #[serde(rename = "connected")]
    Connected,
    /// Disconnected state
    #[serde(rename = "disconnected")]
    Disconnected,
}

/// Function to bootstrap peers using a JSON file.
pub async fn boostrap(discv5: &mut Discv5, file: Option<String>) -> eyre::Result<()> {
    if let Some(f) = file {
        // Read the JSON bootstrap file
        let file = File::open(f)?;
        let reader = BufReader::new(file);
        let bootstrap_store: BootstrapStore = serde_json::from_reader(reader)?;

        // For each bootstrap node, try to connect to it.
        for node in bootstrap_store.data {
            // Skip over invalid enrs
            if let Ok(enr) = Enr::from_str(&node.enr) {
                let node_id = enr.node_id();
                match discv5.add_enr(enr) {
                    Err(_) => { /* log::warn!("Failed to bootstrap node with id: {node_id}") */ }
                    Ok(_) => {
                        log::debug!("Bootstrapped node: {node_id}");
                    }
                }
            }
        }
    }

    Ok(())
}
