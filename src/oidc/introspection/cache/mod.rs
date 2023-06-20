//! Module that enables and supports caching of introspection results.
//! Caching the returned introspection results can be useful to reduce the load on the
//! ZITADEL server. Depending on the enabled features, the cache can be persisted
//! or only be kept in memory.

pub mod in_memory;

type Response = super::ZitadelIntrospectionResponse;

/// Trait that needs to be implemented by a cache.
/// Enables caching the introspection results done by
/// [introspect](crate::oidc::introspection::introspect).
///
/// The cache MUST respect the `expires_in` field of the introspection result.
/// If, by any means, the `exp` field is not set, the cache MUST NOT cache the result.
///
/// ZITADEL will always set the `exp` field, if the token is "active".
///
/// Non-active tokens SHOULD not be cached.
#[async_trait::async_trait]
pub trait IntrospectionCache: Send + Sync + std::fmt::Debug {
    async fn get(&self, token: &str) -> Option<Response>;

    async fn set(&self, token: &str, response: Response);

    async fn clear(&self);
}
