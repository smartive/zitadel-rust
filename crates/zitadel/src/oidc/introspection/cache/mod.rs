//! Module that enables and supports caching of introspection results.
//! Caching the returned introspection results can be useful to reduce the load on the
//! ZITADEL server. Depending on the enabled features, the cache can be persisted
//! or only be kept in memory.

use async_trait::async_trait;
use std::fmt::Debug;
use std::ops::Deref;

pub mod in_memory;

type Response = super::ZitadelIntrospectionResponse;

/// Implementation of an introspection cache.
/// Enables caching the introspection results done by
/// [introspect](crate::oidc::introspection::introspect).
///
/// The cache MUST respect the `expires_in` field of the introspection result.
/// If, by any means, the `exp` field is not set, the cache MUST NOT cache the result.
///
/// ZITADEL will always set the `exp` field, if the token is "active".
///
/// Non-active tokens SHOULD not be cached.
#[async_trait]
pub trait IntrospectionCache: Send + Sync + std::fmt::Debug {
    /// Retrieves the cached introspection result for the given token, if it exists.
    async fn get(&self, token: &str) -> Option<Response>;

    /// Caches the given introspection result for the given token.
    /// If the token is not active, the result is not cached.
    async fn set(&self, token: &str, response: Response);

    /// Clears the cache.
    async fn clear(&self);
}

#[async_trait]
impl<T, V> IntrospectionCache for T
where
    T: Deref<Target = V> + Send + Sync + Debug,
    V: IntrospectionCache,
{
    async fn get(&self, token: &str) -> Option<Response> {
        self.deref().get(token).await
    }

    async fn set(&self, token: &str, response: Response) {
        self.deref().set(token, response).await
    }

    async fn clear(&self) {
        self.deref().clear().await
    }
}
