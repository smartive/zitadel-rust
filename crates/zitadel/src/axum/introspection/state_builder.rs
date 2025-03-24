use custom_error::custom_error;
use std::sync::Arc;

use crate::axum::introspection::state::IntrospectionConfig;
use crate::credentials::Application;
use crate::oidc::discovery::{discover, DiscoveryError};
use crate::oidc::introspection::AuthorityAuthentication;

#[cfg(feature = "introspection_cache")]
use crate::oidc::introspection::cache::IntrospectionCache;

use super::state::IntrospectionState;

custom_error! {
    /// Error type for introspection config builder related errors.
    pub IntrospectionStateBuilderError
        NoAuthSchema = "no authentication for authority defined",
        Discovery{source: DiscoveryError} = "could not fetch discovery document: {source}",
        NoIntrospectionUrl = "discovery document did not contain an introspection url",
}

pub struct IntrospectionStateBuilder {
    authority: String,
    authentication: Option<AuthorityAuthentication>,
    #[cfg(feature = "introspection_cache")]
    cache: Option<Box<dyn IntrospectionCache>>,
}

/// Builder for [IntrospectionConfig]
impl IntrospectionStateBuilder {
    pub fn new(authority: &str) -> Self {
        Self {
            authority: authority.to_string(),
            authentication: None,
            #[cfg(feature = "introspection_cache")]
            cache: None,
        }
    }

    pub fn with_basic_auth(
        &mut self,
        client_id: &str,
        client_secret: &str,
    ) -> &mut IntrospectionStateBuilder {
        self.authentication = Some(AuthorityAuthentication::Basic {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        });

        self
    }

    pub fn with_jwt_profile(&mut self, application: Application) -> &mut IntrospectionStateBuilder {
        self.authentication = Some(AuthorityAuthentication::JWTProfile { application });

        self
    }

    /// Set the [IntrospectionCache] to use for caching introspection responses.
    #[cfg(feature = "introspection_cache")]
    pub fn with_introspection_cache(
        &mut self,
        cache: impl IntrospectionCache + 'static,
    ) -> &mut IntrospectionStateBuilder {
        self.cache = Some(Box::new(cache));

        self
    }

    pub async fn build(&mut self) -> Result<IntrospectionState, IntrospectionStateBuilderError> {
        if self.authentication.is_none() {
            return Err(IntrospectionStateBuilderError::NoAuthSchema);
        }

        let metadata = discover(&self.authority)
            .await
            .map_err(|source| IntrospectionStateBuilderError::Discovery { source })?;

        let introspection_uri = metadata
            .additional_metadata()
            .introspection_endpoint
            .clone();

        if introspection_uri.is_none() {
            return Err(IntrospectionStateBuilderError::NoIntrospectionUrl);
        }

        Ok(IntrospectionState {
            config: Arc::new(IntrospectionConfig {
                authority: self.authority.clone(),
                introspection_uri: introspection_uri.unwrap(),
                authentication: self.authentication.as_ref().unwrap().clone(),
                #[cfg(feature = "introspection_cache")]
                cache: self.cache.take(),
            }),
        })
    }
}
