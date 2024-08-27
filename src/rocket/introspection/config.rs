use openidconnect::IntrospectionUrl;

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
