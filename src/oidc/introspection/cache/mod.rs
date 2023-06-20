//! Module that enables and supports caching of introspection results.
//! Caching the returned introspection results can be useful to reduce the load on the
//! ZITADEL server. Depending on the enabled features, the cache can be persisted
//! or only be kept in memory.

pub mod in_memory;
// mod pgsql;

/// Trait that needs to be implemented by a cache.
/// Enables caching the introspection results done by
/// [introspect](crate::oidc::introspection::introspect).
///
/// The cache MUST respect the `expires_in` field of the introspection result.
#[async_trait::async_trait]
pub trait IntrospectionCache: Send + Sync {
    async fn get(&self, token: &str) -> Option<super::ZitadelIntrospectionResponse>;

    async fn set(&self, token: &str, response: super::ZitadelIntrospectionResponse);
}
