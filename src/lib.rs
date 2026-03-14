#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/caos/zitadel/main/console/src/favicon.ico",
    html_logo_url = "https://raw.githubusercontent.com/caos/zitadel/main/console/src/assets/images/zitadel-logo-solo-light.svg",
    issue_tracker_base_url = "https://github.com/smartive/zitadel-rust/issues/"
)]
//! This is the [ZITADEL](https://zitadel.com/) API and authentication crate for rust.
//! It does provide the gRPC service clients to access the ZITADEL API and contains other
//! utilities and helpers for ZITADEL.
//!
//! # ZITADEL
//!
//! [ZITADEL](https://zitadel.com/) is an open source IAM (identity and access management) system.
//! With this crate, it is possible to access the API of ZITADEL itself
//! on the SaaS cloud instance or any self-hosted variant of ZITADEL.
//!
//! To create a ZITADEL (v2) instance, head over to the [ZITADEL customer portal](https://zitadel.cloud)
//! and create an account and an instance. The newly created instance can then be used to secure
//! your applications and APIs. This crate offers the necessary tools to access the API of the
//! instance, to authenticate users, and to validate incoming requests.
//!
//! To get started with ZITADEL, check out the
//! [getting started guide in the docs](https://docs.zitadel.com/docs/guides/start/quickstart)!
//!
//! # API Access
//!
//! To use this crate for accessing the ZITADEL API, ensure that the `api` feature is enabled.
//! You may use the provided
//! [clients][crate::api::clients] and their convenience functions to create a client that directly authenticates
//! itself against the API. The `*.proto` files are directly fetched from
//! [the ZITADEL repository](https://github.com/zitadel/zitadel).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

#[cfg(feature = "actix")]
pub mod actix;
#[cfg(feature = "api-common")]
pub mod api;
#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "credentials")]
pub mod credentials;
#[cfg(feature = "oidc")]
pub mod oidc;
#[cfg(feature = "rocket")]
pub mod rocket;
