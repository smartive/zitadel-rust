//! This module provides convenience functions and structs to interact with
//! ZITADEL within the [rocket framework](https://rocket.rs/).
//!
//! Rocket is a easy to use, fast, web framework for rust that allows
//! "middlewares" to intercept calls. To authenticate a user against ZITADEL,
//! "guards" are used to check the incoming request beforehand.
//!
//! Refer to the specific authentication method to see further documentation and
//! examples:
//!
//! - To use OAuth 2.0 Token Introspection, head over to the [introspection] module.
//! - More to come ;-)

pub mod introspection;
