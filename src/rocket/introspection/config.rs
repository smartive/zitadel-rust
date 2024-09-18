use openidconnect::IntrospectionUrl;
use serde::Deserialize;

#[cfg(feature = "introspection_cache")]
use crate::oidc::introspection::cache::IntrospectionCache;
use crate::oidc::introspection::AuthorityAuthentication;

/// Configuration that must be injected into
/// [the managed global state](https://rocket.rs/v0.5-rc/guide/state/#managed-state) of the
/// rocket instance to enable the OAuth token introspection authentication method.
///
/// Use the [IntrospectionConfigBuilder](super::IntrospectionConfigBuilder)
/// to construct a config.
#[derive(Debug)]
pub struct IntrospectionConfig {
    pub(crate) authority: String,
    pub(crate) authentication: AuthorityAuthentication,
    pub(crate) introspection_uri: IntrospectionUrl,
    #[cfg(feature = "introspection_cache")]
    pub(crate) cache: Option<Box<dyn IntrospectionCache>>,
}

#[cfg(feature = "rocket_okapi")]
/// Configuration for OAuth token introspection read from a Rocket.toml file.
///
/// # Fields
/// - `authority`: A string representing the authority URL used for introspection. This is typically
///   the base URL of the OAuth provider that will validate the tokens.
///
/// # Example
/// ```toml
/// [default]
/// authority = "https://auth.example.com/"
/// ```
///
/// # Features
/// This struct is only available when the `rocket_okapi` feature is enabled.
#[derive(Debug, Deserialize)]
pub struct IntrospectionRocketConfig {
    pub(crate) authority: String,
}
