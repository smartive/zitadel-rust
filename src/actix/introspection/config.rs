use openidconnect::IntrospectionUrl;

use crate::oidc::introspection::AuthorityAuthentication;

/// Configuration that must be injected into
/// [state](https://actix.rs/docs/application#state) of actix
/// to enable the OAuth token introspection authentication method.
///
/// Use the [IntrospectionConfigBuilder](super::IntrospectionConfigBuilder)
/// to construct a config.
#[derive(Clone, Debug)]
pub struct IntrospectionConfig {
    pub(crate) authority: String,
    pub(crate) authentication: AuthorityAuthentication,
    pub(crate) introspection_uri: IntrospectionUrl,
}
