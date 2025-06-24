use std::collections::HashMap;

use crate::axum::introspection::IntrospectionState;
use axum::http::StatusCode;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::IntoResponse,
    Json, RequestPartsExt,
};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use custom_error::custom_error;
use serde_json::json;

use crate::oidc::introspection::{claims::ZitadelClaims, introspect, IntrospectionError};

custom_error! {
    /// Error type for guard related errors.
    pub IntrospectionGuardError
        MissingConfig = "no introspection config given to rocket managed state",
        Unauthorized = "no HTTP authorization header found",
        InvalidHeader = "authorization header is invalid",
        WrongScheme = "Authorization header is not a bearer token",
        Introspection{source: IntrospectionError} = "introspection returned an error: {source}",
        Inactive = "access token is inactive",
        NoUserId = "introspection result contained no user id",
}

impl IntoResponse for IntrospectionGuardError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            IntrospectionGuardError::MissingConfig => {
                (StatusCode::INTERNAL_SERVER_ERROR, "missing config")
            }
            IntrospectionGuardError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            IntrospectionGuardError::InvalidHeader => (StatusCode::BAD_REQUEST, "invalid header"),
            IntrospectionGuardError::WrongScheme => (StatusCode::BAD_REQUEST, "invalid schema"),
            IntrospectionGuardError::Introspection { source: _ } => {
                (StatusCode::BAD_REQUEST, "introspection error")
            }
            IntrospectionGuardError::Inactive => (StatusCode::FORBIDDEN, "user is inactive"),
            IntrospectionGuardError::NoUserId => (StatusCode::NOT_FOUND, "user was not found"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

/// Type alias for the extracted user.
/// 
/// The extracted user will always be valid when fetched in request function arguments.
/// If not, the API will return with an appropriate error.
///
/// # Example
///
/// ```
/// use axum::http::StatusCode;
/// use axum::response::IntoResponse;
/// use zitadel::axum::introspection::IntrospectedUser;
///
/// async fn my_handler(user: IntrospectedUser) -> impl IntoResponse {
///     if !user.has_role("admin") {
///        return StatusCode::FORBIDDEN.into_response();
///     }
///     
///     if user.has_role_in_project("project123", "editor") {
///         return "Hello Editor".into_response();
///     }
///     
///     "Hello Admin".into_response()
/// }
/// ```
pub type IntrospectedUser = ZitadelClaims;

impl<S> FromRequestParts<S> for IntrospectedUser
where
    IntrospectionState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = IntrospectionGuardError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| IntrospectionGuardError::InvalidHeader)?;

        let state = IntrospectionState::from_ref(state);
        let config = &state.config;

        #[cfg(feature = "introspection_cache")]
        let res = {
            match state.config.cache.as_deref() {
                None => {
                    introspect(
                        &config.introspection_uri,
                        &config.authority,
                        &config.authentication,
                        bearer.token(),
                    )
                    .await
                }
                Some(cache) => match cache.get(bearer.token()).await {
                    Some(cached_response) => Ok(cached_response),
                    None => {
                        let res = introspect(
                            &config.introspection_uri,
                            &config.authority,
                            &config.authentication,
                            bearer.token(),
                        )
                        .await;
                        if let Ok(res) = &res {
                            cache.set(bearer.token(), res.clone()).await;
                        }
                        res
                    }
                },
            }
        };

        #[cfg(not(feature = "introspection_cache"))]
        let res = introspect(
            &config.introspection_uri,
            &config.authority,
            &config.authentication,
            bearer.token(),
        )
        .await;

        let claims = res.map_err(|source| IntrospectionGuardError::Introspection { source })?;

        if claims.sub.is_empty() {
            return Err(IntrospectionGuardError::NoUserId);
        }

        Ok(claims)
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    use axum::body::Body;
    use axum::http::Request;
    use axum::response::IntoResponse;
    use axum::routing::get;
    use axum::Router;

    use tower::ServiceExt;

    use crate::axum::introspection::{IntrospectionState, IntrospectionStateBuilder};
    use crate::credentials::Application;

    use super::*;

    const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    const PERSONAL_ACCESS_TOKEN: &str =
        "dEnGhIFs3VnqcQU5D2zRSeiarB1nwH6goIKY0J8MWZbsnWcTuu1C59lW9DgCq1y096GYdXA";
    const APPLICATION: &str = r#"
    {
        "type": "application",
        "keyId": "181963758610940161",
        "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAwT2YZJytkkZ1DDM3dcu1OA8YPzHu6XR8HotdMNRnV75GhOT4\nB7zDtdtoP8w/1NHHPEJ859e0kYhrrnKikOKLS6fS1KRsmqR5ZvTq8SlZ2mq3RcX2\nebZx5dQt36INij/WXdsBmjM/yfWvqqWBSb0L/186DaWwmmIxoXWe873vxRmlzblg\nGd8Nu07s9YTREbGPbtFVHEUM6xI4oIe8HJ0e1+JBkiGqk31Cogo0FoAxrOAg0Sf4\n5XiUMYIjzqh8673F9SC4IpVxG22mpFk3vDFuAITaStWYbiH2hPJNKWyX9HDCZb1D\nDqa3wZBDiLqWxh22hNZ6ZIe+3UoSGWsPBH+E1wIDAQABAoIBAD2v5QsRPRN57HmF\njAnNir8nimz6CrN53Pl/MbOZypenBSn9UfReXPeb3+6lzCarBPgGnYsBQAJJU16v\n95daym7PVy1Mg+Ll6F9mhe2Qbr+b23+pj2IRTNC6aB6Aw+PDNzJk7GEGRTG6fWZz\nSQ96Cu9tvcGHiBXwjLlnK+PRWU5IsCiLsjT4xBXsMLMw3YOdMK5z58sqr+SnNEyq\nRHoEvi9aC94WrargVB45Yx+81YNW8uQ5rMDmYaJC5a7ENz522SlAuf4T+fAGJ/HE\n/qbZGD4YwlLqAFDgewQ+5tEWEus3zgY2MIR7vN2zXU1Ptk+mQkXZl/Pxdp7q1xU+\nvr/kcykCgYEAy7MiIAzc1ctQDvkk3HiespzdQ/sC7+CGsBzkyubRc9Oq/YR7GfVK\nGTuDEDlWwx92VAvJGDWRa3T426YDyqiPj66uo836sgL15Uigg5afZun2bqGC78le\nBhSy9b+0YDHPa87GxtKt9UmMoB6WdmoPzOkLEEGS7eesmk2DDgY+QSUCgYEA8tr/\n3PawigL1cxuFpcO1lH6XUspGeAo5yB8FXvfW5g50e37LgooIvOFgUlYuchxwr6uh\nW+CUAWmm4farsgvMBMPYw+PbkCTi/xemiiDmMHUYd7sJkTl0JXApq3pZsNMg4Fw/\n29RynmcG8TGe2dkwrWp1aBYjvIHwEHuNHHTTA0sCgYBtSUFAwsXkaj0cm2y8YHZ8\nS46mv1AXFHYOnKHffjDXnLN7ao2FIsXLfdNWa/zxmLqqYtxUAcFwToSJi6szGnZT\nVxvZRFSBFveIOQvtLW1+EH4nYr3WGko4pvhQwrZqea7YH0skNrogBILPEToWc9bg\nUBOgeB31R7uh2X47kvvphQKBgQDWc60dYnniZVp5mwQZrQjbaC4YXaZ8ugrsPPhx\nNEoAPSN/KihrzZiJsjtsec3p1lNrzRNgHqCT3sgPIdPcFa7DRm5UDRIF54zL1gaq\nUwLyJ3TDxdZc928o4DLryc8J5mZRuSRq6t+MIU5wDnFHzhK+EBQ9Jc/I1rU22ONz\nDXaIoQKBgH14Apggo0o4Eo+OnEBRFbbDulaOfVLPTK9rktikbwO1vzDch8kdcwCU\nsvtRXHjDQL93Ih/8S9aDJZoSDulwr3VUsuDiDEb4jfYmP2sbNO4nIJt+SBMhVOXV\nt7E/uWK28X0GL/bIUzSMMgTfdjhXEtJW+s6hQU1fG+9U1qVTQ2R/\n-----END RSA PRIVATE KEY-----\n",
        "appId": "181963751145079041",
        "clientId": "181963751145144577@zitadel_rust_test"
    }"#;

    async fn authed(user: IntrospectedUser) -> impl IntoResponse {
        format!(
            "Hello authorized user: {:?} with id {}",
            user.username, user.sub
        )
    }

    async fn unauthed() -> impl IntoResponse {
        "Hello unauthorized"
    }

    #[derive(Clone)]
    struct SomeUserState {
        introspection_state: IntrospectionState,
    }

    impl FromRef<SomeUserState> for IntrospectionState {
        fn from_ref(input: &SomeUserState) -> Self {
            input.introspection_state.clone()
        }
    }

    async fn app() -> Router {
        let introspection_state = IntrospectionStateBuilder::new(ZITADEL_URL)
            .with_jwt_profile(Application::load_from_json(APPLICATION).unwrap())
            .build()
            .await
            .unwrap();

        let state = SomeUserState {
            introspection_state,
        };

        let app = Router::new()
            .route("/unauthed", get(unauthed))
            .route("/authed", get(authed))
            .with_state(state);

        return app;
    }

    #[tokio::test]
    async fn can_guard() {
        let app = app().await;

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/authed")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn guard_protects_if_non_bearer_present() {
        let app = app().await;

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/authed")
                    .header("authorization", "Basic Something")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn guard_protects_if_multiple_auth_headers_present() {
        let app = app().await;

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/authed")
                    .header("authorization", "something one")
                    .header("authorization", "something two")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn guard_protects_if_invalid_token() {
        let app = app().await;

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/authed")
                    .header("authorization", "Bearer something")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn guard_allows_valid_token() {
        let app = app().await;

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/authed")
                    .header("authorization", format!("Bearer {PERSONAL_ACCESS_TOKEN}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[cfg(feature = "introspection_cache")]
    mod introspection_cache {
        use super::*;
        use crate::oidc::introspection::cache::in_memory::InMemoryIntrospectionCache;
        use crate::oidc::introspection::cache::IntrospectionCache;
        use chrono::{TimeDelta, Utc};
        use http_body_util::BodyExt;
        use std::ops::Add;
        use std::sync::Arc;

        async fn app_witch_cache(cache: impl IntrospectionCache + 'static) -> Router {
            let introspection_state = IntrospectionStateBuilder::new(ZITADEL_URL)
                .with_jwt_profile(Application::load_from_json(APPLICATION).unwrap())
                .with_introspection_cache(cache)
                .build()
                .await
                .unwrap();

            let state = SomeUserState {
                introspection_state,
            };

            let app = Router::new()
                .route("/unauthed", get(unauthed))
                .route("/authed", get(authed))
                .with_state(state);

            return app;
        }

        #[tokio::test]
        async fn guard_uses_cached_response() {
            let cache = Arc::new(InMemoryIntrospectionCache::default());
            let app = app_witch_cache(cache.clone()).await;

            use crate::oidc::introspection::claims::ZitadelClaims;
            let res = ZitadelClaims {
                sub: "cached_sub".to_string(),
                iss: "https://test.zitadel.cloud".to_string(),
                aud: vec!["test".to_string()],
                username: Some("cached_user".to_string()),
                exp: Utc::now().add(TimeDelta::days(1)).timestamp(),
                iat: Utc::now().timestamp(),
                active: true,
                ..Default::default()
            };
            cache.set(PERSONAL_ACCESS_TOKEN, res).await;

            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/authed")
                        .header("authorization", format!("Bearer {PERSONAL_ACCESS_TOKEN}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            let text = String::from_utf8(
                response
                    .into_body()
                    .collect()
                    .await
                    .unwrap()
                    .to_bytes()
                    .to_vec(),
            )
            .unwrap();
            assert!(text.contains("cached_sub"));
        }

        #[tokio::test]
        async fn guard_caches_response() {
            let cache = Arc::new(InMemoryIntrospectionCache::default());
            let app = app_witch_cache(cache.clone()).await;

            let response = app
                .oneshot(
                    Request::builder()
                        .uri("/authed")
                        .header("authorization", format!("Bearer {PERSONAL_ACCESS_TOKEN}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            let text = String::from_utf8(
                response
                    .into_body()
                    .collect()
                    .await
                    .unwrap()
                    .to_bytes()
                    .to_vec(),
            )
            .unwrap();

            let cached_response = cache.get(PERSONAL_ACCESS_TOKEN).await.unwrap();

            assert!(text.contains(&cached_response.username.unwrap()));
        }
    }
}
