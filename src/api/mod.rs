//! Module with API elements and definitions of ZITADEL.
//! Contains the compiled ZITADEL gRPC API as well
//! as the gRPC clients and constructors for clients.
//!
//! Further contains interceptors that may be used to
//! authenticate the clients to ZITADEL with credentials.

#[cfg(feature = "api")]
pub use api::zitadel;

#[cfg(feature = "api")]
mod api;
#[cfg(any(feature = "api", feature = "interceptors", feature = "credentials"))]
pub mod clients;
#[cfg(feature = "interceptors")]
pub mod interceptors;
