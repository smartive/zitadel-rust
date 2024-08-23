//! Module that contains credentials for ZITADEL.
//! This module allows accessing the ZITADEL API or
//! using ZITADEL as an OIDC provider. Can be used in conjunction
//! with the gRPC service clients to access the API.

mod application;
mod jwt;
mod service_account;

pub use application::*;
pub use service_account::*;
