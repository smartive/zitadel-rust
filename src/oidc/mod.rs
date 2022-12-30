//! Module with OIDC related functionality.
//! ZITADEL is built upon [OIDC](https://openid.net/developers/specs/) which
//! enables standardized access to some functionality. This module provides the
//! base functionality for OIDC related calls.
//!
//! As an example, the `introspection` module contains the [`introspection::introspect`] function
//! which allows [OAuth 2.0 Introspection](https://www.rfc-editor.org/rfc/rfc7662).
//! The user can send a token to ZITADEL to check whether the token is still active and
//! valid.

pub mod discovery;
pub mod introspection;
