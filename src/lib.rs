//! This is the [ZITADEL](https://zitadel.ch/) API package for rust.
//! It does provide the gRPC clients to access the ZITADEL API and contains other
//! elements around ZITADEL.

pub use zitadel::*;

pub mod grpc;
pub mod credentials;
mod zitadel;
