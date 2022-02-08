//! Contains elements and modules to interact with the ZITADEL API via gRPC.

pub use api::*;

mod api;
pub mod interceptors;
pub mod clients;
