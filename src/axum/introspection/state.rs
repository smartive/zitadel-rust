use axum::extract::FromRef;
use openidconnect::IntrospectionUrl;

use crate::oidc::introspection::AuthorityAuthentication;

#[derive(Clone, Debug)]
pub struct IntrospectionState {
    pub(crate) config: IntrospectionConfig,
}

#[derive(Debug, Clone)]
pub struct IntrospectionConfig {
    pub(crate) authority: String,
    pub(crate) authentication: AuthorityAuthentication,
    pub(crate) introspection_uri: IntrospectionUrl,
}

impl FromRef<IntrospectionState> for IntrospectionConfig {
    fn from_ref(input: &IntrospectionState) -> Self {
        input.config.clone()
    }
}
