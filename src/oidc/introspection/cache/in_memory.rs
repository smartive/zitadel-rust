pub use moka::future::{Cache, CacheBuilder};
use std::time::Duration;

use openidconnect::TokenIntrospectionResponse;

type Response = super::super::ZitadelIntrospectionResponse;

#[derive(Debug)]
pub struct InMemoryIntrospectionCache {
    cache: Cache<String, (Response, i64)>,
}

impl InMemoryIntrospectionCache {
    /// Creates a new in memory cache, with setting a default `max_capacity` of `10_000` entries.
    /// and a TTL expiry of 15 minutes, which is much smaller than the default access token lifetime
    /// in Zitadel.
    /// A small cache expiry is desired to detect revoked tokens and changes in role assignments
    /// outside the token lifetime.
    ///
    /// For more fine-grained control use [new_from_cache].
    pub fn new() -> Self {
        Self {
            cache: CacheBuilder::new(10_000)
                .time_to_live(Duration::from_secs(15 * 60))
                .build(),
        }
    }

    /// Create a new in memory cache from a preconfigured Moka cache.
    /// Use the re-exposed CacheBuilder to set custom configuration values.
    ///
    /// ```
    /// use std::time::Duration;
    /// use zitadel::oidc::introspection::cache::in_memory::InMemoryIntrospectionCache;
    /// use zitadel::oidc::introspection::cache::in_memory::CacheBuilder;
    /// let cache = InMemoryIntrospectionCache::new_from_cache(
    ///                 CacheBuilder::new(10_000)
    ///                     // use short lifetime to make sure token invalidation can be detected
    ///                     .time_to_live(Duration::from_secs(60*15))
    ///                     .build()
    ///             );
    pub fn new_from_cache(cache: Cache<String, (Response, i64)>) -> Self {
        Self { cache }
    }
}

impl Default for InMemoryIntrospectionCache {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl super::IntrospectionCache for InMemoryIntrospectionCache {
    async fn get(&self, token: &str) -> Option<Response> {
        match self.cache.get(token).await {
            Some((_, expires_at))
                if expires_at < time::OffsetDateTime::now_utc().unix_timestamp() =>
            {
                self.cache.invalidate(token).await;
                None
            }
            Some((response, _)) => Some(response),
            None => None,
        }
    }

    async fn set(&self, token: &str, response: Response) {
        if !response.active() || response.exp().is_none() {
            return;
        }
        let expires_at = response.exp().unwrap().timestamp();
        self.cache
            .insert(token.to_string(), (response, expires_at))
            .await;
    }

    async fn clear(&self) {
        self.cache.invalidate_all();
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    use crate::oidc::introspection::cache::IntrospectionCache;
    use chrono::{TimeDelta, Utc};

    use super::*;

    #[tokio::test]
    async fn test_get_set() {
        let c = InMemoryIntrospectionCache::new();
        let t = &c as &dyn IntrospectionCache;

        let mut response = Response::new(true, Default::default());
        response.set_exp(Some(Utc::now()));

        t.set("token1", response.clone()).await;
        t.set("token2", response.clone()).await;

        assert!(t.get("token1").await.is_some());
        assert!(t.get("token2").await.is_some());
        assert!(t.get("token3").await.is_none());
    }

    #[tokio::test]
    async fn test_non_exp_response() {
        let c = InMemoryIntrospectionCache::new();
        let t = &c as &dyn IntrospectionCache;

        let response = Response::new(true, Default::default());

        t.set("token1", response.clone()).await;
        t.set("token2", response.clone()).await;

        assert!(t.get("token1").await.is_none());
        assert!(t.get("token2").await.is_none());
    }

    #[tokio::test]
    async fn test_clear() {
        let c = InMemoryIntrospectionCache::new();
        let t = &c as &dyn IntrospectionCache;

        let mut response = Response::new(true, Default::default());
        response.set_exp(Some(Utc::now()));

        t.set("token1", response.clone()).await;
        t.set("token2", response.clone()).await;

        t.clear().await;

        assert!(t.get("token1").await.is_none());
        assert!(t.get("token2").await.is_none());
    }

    #[tokio::test]
    async fn test_remove_expired_token() {
        let c = InMemoryIntrospectionCache::new();
        let t = &c as &dyn IntrospectionCache;

        let mut response = Response::new(true, Default::default());
        response.set_exp(Some(Utc::now() - TimeDelta::try_seconds(10).unwrap()));

        t.set("token1", response.clone()).await;
        t.set("token2", response.clone()).await;

        let _ = t.get("token1").await;
        let _ = t.get("token2").await;

        assert!(t.get("token1").await.is_none());
        assert!(t.get("token2").await.is_none());
    }
}
