use openidconnect::IntrospectionUrl;
use std::sync::Arc;

#[cfg(feature = "introspection_cache")]
use crate::oidc::introspection::cache::IntrospectionCache;
use crate::oidc::introspection::AuthorityAuthentication;

/// State which must be present for extractor to work,
/// compare [axum's official documentation](https://docs.rs/axum/0.6.4/axum/extract/struct.State.html#for-library-authors).
/// Use [IntrospectionStateBuilder](super::IntrospectionStateBuilder) to configure the respective parameters.
///
/// If a custom state is used, then [FromRef](axum::extract::FromRef) must be implemented,
/// to make the necessary state available.
///
/// ```
/// use axum::extract::FromRef;
/// use zitadel::axum::introspection::IntrospectionState;
/// struct UserState {
///     introspection_state: IntrospectionState
/// }
///
/// impl FromRef<UserState> for IntrospectionState {
///     fn from_ref(input: &UserState) -> Self {
///         input.introspection_state.clone()
///     }
/// }
#[derive(Clone, Debug)]
pub struct IntrospectionState {
    pub(crate) config: Arc<IntrospectionConfig>,
}

#[derive(Debug)]
pub(crate) struct IntrospectionConfig {
    pub(crate) authority: String,
    pub(crate) authentication: AuthorityAuthentication,
    pub(crate) introspection_uri: IntrospectionUrl,
    #[cfg(feature = "introspection_cache")]
    pub(crate) cache: Option<Box<dyn IntrospectionCache>>,
}
