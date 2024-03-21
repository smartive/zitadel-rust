//! This module provides convenience functions and structs to interact with
//! ZITADEL within the [actix-web framework](https://actix.rs/).
//!
//! Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
//! To authenticate a user against ZITADEL,
//! "extractors" are used to check the incoming request beforehand.
//!
//! Refer to the specific authentication method to see further documentation and
//! examples:
//!
//! - To use OAuth 2.0 Token Introspection, head over to the [introspection] module.

pub mod introspection;
