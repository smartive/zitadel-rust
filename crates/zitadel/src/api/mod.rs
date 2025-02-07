//! Module with API elements and definitions of ZITADEL.
//! Contains the compiled ZITADEL gRPC API as well
//! as the gRPC clients and constructors for clients.
//!
//! Further contains interceptors that may be used to
//! authenticate the clients to ZITADEL with credentials.


pub mod clients;
#[allow(clippy::all)]
#[cfg(feature = "api")]
pub use zitadel_gen::zitadel;
#[cfg(feature = "interceptors")]
pub mod interceptors;
