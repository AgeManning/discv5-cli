#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

//! Core library for the discv5 cli.

/// Cli Handlers
pub mod cli;

/// Packet Handlers
pub mod packet;

/// Request ENR Handlers
pub mod request_enr;

/// Server Handlers
pub mod server;

/// Utility functions
pub mod utils;

/// A prelude for re-exporting commonly used items.
pub mod prelude {
    pub use crate::packet::*;
    pub use crate::request_enr::*;
    pub use crate::server::*;
}
