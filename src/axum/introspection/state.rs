use axum::extract::FromRef;
use openidconnect::IntrospectionUrl;

use crate::oidc::introspection::AuthorityAuthentication;

#[derive(Clone, Debug)]
pub struct IntrospectionState {
    pub(crate) config: IntrospectionConfig,
}

impl IntrospectionState {
    pub fn config(&self) -> &IntrospectionConfig {
        &self.config
    }
}

/// Configuration that must be inject into the axum application state. Used by the
/// [IntrospectionStateBuilder](super::IntrospectionStateBuilder). This struct is also used to create the [IntrospectionState](IntrospectionState)
#[derive(Debug, Clone)]
pub struct IntrospectionConfig {
    pub authority: String,
    pub authentication: AuthorityAuthentication,
    pub introspection_uri: IntrospectionUrl,
}

impl FromRef<IntrospectionState> for IntrospectionConfig {
    fn from_ref(input: &IntrospectionState) -> Self {
        input.config.clone()
    }
}
