use std::{collections::HashMap, sync::Mutex};

use openidconnect::TokenIntrospectionResponse;

type Response = super::super::ZitadelIntrospectionResponse;

#[derive(Debug)]
pub struct InMemoryIntrospectionCache {
    cache: Mutex<HashMap<String, (Response, i64)>>,
}

impl InMemoryIntrospectionCache {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
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
        let mut cache = self.cache.lock().unwrap();

        match cache.get(token) {
            Some((_, expires_at))
                if expires_at < &time::OffsetDateTime::now_utc().unix_timestamp() =>
            {
                cache.remove(token);
                None
            }
            Some((response, _)) => Some(response.clone()),
            None => None,
        }
    }

    async fn set(&self, token: &str, response: Response) {
        if !response.active() || response.exp().is_none() {
            return;
        }

        let mut cache = self.cache.lock().unwrap();
        let expires_at = response.exp().unwrap().timestamp();
        cache.insert(token.to_string(), (response, expires_at));
    }

    async fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    use crate::oidc::introspection::cache::IntrospectionCache;
    use chrono::{Duration, Utc};

    use super::*;

    #[tokio::test]
    async fn test_get_set() {
        let c = InMemoryIntrospectionCache::new();
        let t = &c as &dyn IntrospectionCache;

        let mut response = Response::new(true, Default::default());
        response.set_exp(Some(Utc::now()));

        t.set("token1", response.clone()).await;
        t.set("token2", response.clone()).await;

        assert_eq!(c.cache.lock().unwrap().len(), 2);

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

        assert_eq!(c.cache.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_clear() {
        let c = InMemoryIntrospectionCache::new();
        let t = &c as &dyn IntrospectionCache;

        let mut response = Response::new(true, Default::default());
        response.set_exp(Some(Utc::now()));

        t.set("token1", response.clone()).await;
        t.set("token2", response.clone()).await;

        assert_eq!(c.cache.lock().unwrap().len(), 2);

        t.clear().await;

        assert_eq!(c.cache.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_remove_expired_token() {
        let c = InMemoryIntrospectionCache::new();
        let t = &c as &dyn IntrospectionCache;

        let mut response = Response::new(true, Default::default());
        response.set_exp(Some(Utc::now() - Duration::seconds(10)));

        t.set("token1", response.clone()).await;
        t.set("token2", response.clone()).await;

        assert_eq!(c.cache.lock().unwrap().len(), 2);

        let _ = t.get("token1").await;
        let _ = t.get("token2").await;

        assert_eq!(c.cache.lock().unwrap().len(), 0);
    }
}
