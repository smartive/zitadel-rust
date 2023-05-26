//! This module provides convenience functions and structs to interact with ZITADEL within the [axum framework](https://docs.rs/axum/latest/axum/)
//!
//! Axum is a simple and easy to use web framework for rust that allows "extracters" and "middlewares" to intercept calls. To authenticate a user against ZITADEL.
//!
//! Refer to the specific authentication method to see further documentation and examples:
//!
//! - To use OAuth 2.0 Token Introspection, head over to the [introspection] module.
pub mod introspection;
