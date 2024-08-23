//! Module with API elements and definitions of ZITADEL.
//! Contains the compiled ZITADEL gRPC API as well
//! as the gRPC clients and constructors for clients.
//!
//! Further contains interceptors that may be used to
//! authenticate the clients to ZITADEL with credentials.

pub use generated::zitadel;

pub mod clients;
#[allow(clippy::all)]
mod generated;
#[cfg(feature = "interceptors")]
pub mod interceptors;
