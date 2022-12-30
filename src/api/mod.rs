//! Module with API elements and definitions of ZITADEL.
//! Contains the compiled ZITADEL gRPC API as well
//! as the gRPC clients and constructors for clients.
//!
//! Further contains interceptors that may be used to
//! authenticate the clients to ZITADEL with credentials.

pub use api::zitadel;

mod api;
pub mod clients;
#[cfg(feature = "interceptors")]
pub mod interceptors;
