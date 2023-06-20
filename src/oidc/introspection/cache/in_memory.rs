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
        let mut cache = self.cache.lock().unwrap();
        let expires_at = response.exp().unwrap().timestamp();
        cache.insert(token.to_string(), (response, expires_at));
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    // TODO.

    // use crate::oidc::discovery::discover;
    // use openidconnect::TokenIntrospectionResponse;

    // use super::*;

    // const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    // const PERSONAL_ACCESS_TOKEN: &str =
    //     "dEnGhIFs3VnqcQU5D2zRSeiarB1nwH6goIKY0J8MWZbsnWcTuu1C59lW9DgCq1y096GYdXA";

    // #[tokio::test]
    // async fn introspect_fails_with_invalid_url() {
    //     let result = introspect(
    //         "foobar",
    //         "foobar",
    //         &AuthorityAuthentication::Basic {
    //             client_id: "".to_string(),
    //             client_secret: "".to_string(),
    //         },
    //         "token",
    //     )
    //     .await;

    //     assert!(result.is_err());
    //     assert!(matches!(
    //         result.unwrap_err(),
    //         IntrospectionError::ParseUrl { .. }
    //     ));
    // }

    // #[tokio::test]
    // async fn introspect_fails_with_invalid_endpoint() {
    //     let meta = discover(ZITADEL_URL).await.unwrap();
    //     let result = introspect(
    //         &meta.token_endpoint().unwrap().to_string(),
    //         ZITADEL_URL,
    //         &AuthorityAuthentication::Basic {
    //             client_id: "".to_string(),
    //             client_secret: "".to_string(),
    //         },
    //         "token",
    //     )
    //     .await;

    //     assert!(result.is_err());
    // }

    // #[tokio::test]
    // async fn introspect_succeeds() {
    //     let meta = discover(ZITADEL_URL).await.unwrap();
    //     let result = introspect(
    //         &meta
    //             .additional_metadata()
    //             .introspection_endpoint
    //             .as_ref()
    //             .unwrap()
    //             .to_string(),
    //         ZITADEL_URL,
    //         &AuthorityAuthentication::Basic {
    //             client_id: "194339055499018497@zitadel_rust_test".to_string(),
    //             client_secret: "Ip56oGzxKL1rJ8JaleUVKL7qUlpZ1tqHQYRSd6JE1mTlTJ3pDkDzoObHdZsOg88B"
    //                 .to_string(),
    //         },
    //         PERSONAL_ACCESS_TOKEN,
    //     )
    //     .await
    //     .unwrap();

    //     assert!(result.active());
    // }
}
