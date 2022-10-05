//! Module for various gRPC interceptors that can be used with the
//! gRPC service clients for ZITADEL. The primary use-case of these
//! interceptors is to authenticate the clients to ZITADEL with
//! provided credentials.

use std::thread;

use tokio::runtime::Builder;
use tonic::{service::Interceptor, Request, Status};

use crate::credentials::{AuthenticationOptions, ServiceAccount};

/// Simple gRPC `Interceptor` that attaches a given access token to any request
/// a client sends. The token is attached with the `Bearer` auth-scheme.
///
/// The access token may be any valid access token for ZITADEL. A token
/// can be fetched with [`ServiceAccount`][crate::credentials::ServiceAccount]
/// credentials or you may create a `Personal Access Token` for a service account
/// in the ZITADEL console. Also, you could also use access tokens that are
/// passed from users.
///
/// The interceptor does not insert the access token if the intercepted call
/// already has an `Authorization` header.
///
/// ### Example
///
/// The following example shows how to use the `AccessTokenInterceptor` with
/// a personal access token to fetch the user profile of the service account.
///
/// ```
/// ```
pub struct AccessTokenInterceptor {
    access_token: String,
}

impl AccessTokenInterceptor {
    /// Create a new [`AccessTokenInterceptor`].
    /// The provided token can be any valid access token for ZITADEL.
    /// This includes:
    /// - Personal Access Tokens for service accounts
    /// - Access tokens that are passed from users
    /// - Access tokens that are fetched with a [`ServiceAccount`][crate::credentials::ServiceAccount]
    ///   and the corresponding [`authenticate`][crate::credentials::ServiceAccount::authenticate] method
    pub fn new(token: &str) -> Self {
        AccessTokenInterceptor {
            access_token: token.to_string(),
        }
    }
}

impl Interceptor for AccessTokenInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let meta = request.metadata_mut();
        if !meta.contains_key("authorization") {
            meta.insert(
                "authorization",
                format!("Bearer {}", self.access_token).parse().unwrap(),
            );
        }
        Ok(request)
    }
}

/// gRPC `Interceptor` that authenticates the service client calls
/// with the given [`ServiceAccount`][crate::credentials::ServiceAccount].
///
/// When no access token is available, the interceptor will fetch a new
/// token from the given audience (sometimes also called issuer) with
/// the - optionally - provided [`AuthenticationOptions`]. If the options
/// are set to `None`, the default options will be used.
///
/// When a token was fetched, the interceptor will only fetch a new token
/// when the lifetime of the token has expired (default 60 minutes).
///
/// **Note on async**: The interceptor does work in sync and async contexts.
/// However, in both cases, the interceptor spawns a new thread which then
/// runs a tokio runtime. This is necessary because the interceptor trait
/// only manages sync calls.
///
/// ### Example
///
/// The following example shows how to use the `ServiceAccountInterceptor`
/// to fetch the user profile of a service account.
///
/// ```
/// ```
pub struct ServiceAccountInterceptor {
    audience: String,
    service_account: ServiceAccount,
    auth_options: AuthenticationOptions,
    token: Option<String>,
    token_expiry: Option<time::OffsetDateTime>,
}

impl ServiceAccountInterceptor {
    /// Create a new [`ServiceAccountInterceptor`].
    /// The interceptor uses the provided service account with
    /// the given authentication options to fetch an access token
    /// and attach it to the intercepted calls.
    pub fn new(
        audience: &str,
        service_account: &ServiceAccount,
        auth_options: Option<AuthenticationOptions>,
    ) -> Self {
        Self {
            audience: audience.to_string(),
            service_account: service_account.clone(),
            auth_options: match auth_options {
                Some(options) => options,
                None => AuthenticationOptions::default(),
            },
            token: None,
            token_expiry: None,
        }
    }
}

impl Interceptor for ServiceAccountInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let meta = request.metadata_mut();
        if !meta.contains_key("authorization") {
            if let Some(token) = &self.token {
                if let Some(expiry) = self.token_expiry {
                    if expiry > time::OffsetDateTime::now_utc() {
                        meta.insert(
                            "authorization",
                            format!("Bearer {}", token).parse().unwrap(),
                        );

                        return Ok(request);
                    }
                }
            }

            let aud = self.audience.clone();
            let auth = self.auth_options.clone();
            let sa = self.service_account.clone();

            let token = thread::spawn(move || {
                let rt = Builder::new_multi_thread().enable_all().build().unwrap();
                rt.block_on(async {
                    sa.authenticate_with_options(&aud, &auth)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))
                })
            });

            let token = token
                .join()
                .map_err(|_| Status::internal("could not fetch token"))??;

            self.token = Some(token.clone());
            self.token_expiry = Some(time::OffsetDateTime::now_utc() + time::Duration::minutes(59));

            meta.insert(
                "authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    const SERVICE_ACCOUNT: &str = r#"
    {
        "type": "serviceaccount",
        "keyId": "181828078849229057",
        "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEpQIBAAKCAQEA9VIWALQqzx1ypi42t7MG4KSOMldD10brsEUjTcjqxhl6TJrP\nsjaNKWArnV/XH+6ZKRd55mUEFFx9VflqdwQtMVPjZKXpV4cFDiPwf1Z1h1DS6im4\nSo7eKR7OGb7TLBhwt7i2UPF4WnxBhTp/M6pG5kCJ1t8glIo5yRbrILXObRmvNWMz\nVIFAyw68NDZGYNhnR8AT43zjeJTFXG/suuEoXO/mMmMjsYY8kS0BbiQeq5t5hIrr\na/odswkDPn5Zd4P91iJHDnYlgfJuo3oRmgpOj/dDsl+vTol+vveeMO4TXPwZcl36\ngUNPok7nd6BA3gqmOS+fMImzmZB42trghARXXwIDAQABAoIBAQCbMOGQcml+ep+T\ntzqQPWYFaLQ37nKRVmE1Mpeh1o+G4Ik4utrXX6EvYpJUzVN29ObZUuufr5nEE7qK\nT+1k+zRntyzr9/VElLrC9kNnGtfg0WWMEvZt3DF4i+9P5CMNCy0LXIOhcxBzFZYR\nZS8hDQArGvrX/nFK5qKlrqTyHXFIHDFa6z59ErhXEnsTgRvx/Mo+6UkdBkHsKnlJ\nAbXqXFbfz6nDsF1DgRra5ODn1k8nZqnC/YcssE7/dlbuByz10ECkOSzqYcfufnsb\n9N1Ld4Xlj3yzsqPFzEJyHHm9eEHQXsPavaXiM64/+zpsksLscEIE/0KtIy5tngpZ\nSCqZAcj5AoGBAPb1bQFWUBmmUuSTtSymsxgXghJiJ3r+jJgdGbkv2IsRTs4En5Sz\n0SbPE1YWmMDDgTacJlB4/XiaojQ/j1EEY17inxYomE72UL6/ET7ycsEw3e9ALuD5\np0y2Sdzes2biH30bw5jD8kJ+hV18T745KtzrwSH4I0lAjnkmiH+0S67VAoGBAP5N\nTtAp/Qdxh9GjNSw1J7KRLtJrrr0pPrJ9av4GoFoWlz+Qw2X3dl8rjG3Bqz9LPV7A\ngiHMel8WTmdIM/S3F4Q3ufEfE+VzG+gncWd9SJfX5/LVhatPzTGLNsY7AYGEpSwT\n5/0anS1mHrLwsVcPrZnigekr5A5mfZl6nxtOnE9jAoGBALACqacbUkmFrmy1DZp+\nUQSptI3PoR3bEG9VxkCjZi1vr3/L8cS1CCslyT1BK6uva4d1cSVHpjfv1g1xA38V\nppE46XOMiUk16sSYPv1jJQCmCHd9givcIy3cefZOTwTTwueTAyv888wKipjfgaIs\n8my0JllEljmeJi0Ylo6V/J7lAoGBAIFqRlmZhLNtC3mcXUsKIhG14OYk9uA9RTMA\nsJpmNOSj6oTm3wndTdhRCT4x+TxUxf6aaZ9ZuEz7xRq6m/ZF1ynqUi5ramyyj9kt\neYD5OSBNODVUhJoSGpLEDjQDg1iucIBmAQHFsYeRGL5nz1hHGkneA87uDzlk3zZk\nOORktReRAoGAGUfU2UfaniAlqrZsSma3ZTlvJWs1x8cbVDyKTYMX5ShHhp+cA86H\nYjSSol6GI2wQPP+qIvZ1E8XyzD2miMJabl92/WY0tHejNNBEHwD8uBZKrtMoFWM7\nWJNl+Xneu/sT8s4pP2ng6QE7jpHXi2TUNmSlgQry9JN2AmA9TuSTW2Y=\n-----END RSA PRIVATE KEY-----\n",
        "userId": "181828061098934529"
    }"#;

    #[test]
    fn token_interceptor_attach_auth_metadata() {
        let mut interceptor = AccessTokenInterceptor::new("token");
        let request = Request::new(());

        assert!(request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().contains_key("authorization"));
        assert_eq!(
            request.metadata().get("authorization").unwrap(),
            "Bearer token"
        );
    }

    #[test]
    fn token_interceptor_ignore_existing_auth_metadata() {
        let mut interceptor = AccessTokenInterceptor::new("token");
        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("authorization", "Bearer existing".parse().unwrap());

        assert!(!request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().contains_key("authorization"));
        assert_eq!(
            request.metadata().get("authorization").unwrap(),
            "Bearer existing"
        );
    }

    #[test]
    fn service_account_interceptor_attach_auth_metadata_sync_context() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let mut interceptor = ServiceAccountInterceptor::new(ZITADEL_URL, &sa, None);
        let request = Request::new(());

        assert!(request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().contains_key("authorization"));
        assert!(!request
            .metadata()
            .get("authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn service_account_interceptor_attach_auth_metadata_async_context() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let mut interceptor = ServiceAccountInterceptor::new(ZITADEL_URL, &sa, None);
        let request = Request::new(());

        assert!(request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().contains_key("authorization"));
        assert!(!request
            .metadata()
            .get("authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn service_account_interceptor_ignore_existing_auth_metadata_sync_context() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let mut interceptor = ServiceAccountInterceptor::new(ZITADEL_URL, &sa, None);
        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("authorization", "Bearer existing".parse().unwrap());

        assert!(!request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().contains_key("authorization"));
        assert_eq!(
            request.metadata().get("authorization").unwrap(),
            "Bearer existing"
        );
    }

    #[tokio::test]
    async fn service_account_interceptor_ignore_existing_auth_metadata_async_context() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let mut interceptor = ServiceAccountInterceptor::new(ZITADEL_URL, &sa, None);
        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("authorization", "Bearer existing".parse().unwrap());

        assert!(!request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().contains_key("authorization"));
        assert_eq!(
            request.metadata().get("authorization").unwrap(),
            "Bearer existing"
        );
    }

    #[test]
    fn service_account_interceptor_should_respect_token_lifetime_sync() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let mut interceptor = ServiceAccountInterceptor::new(ZITADEL_URL, &sa, None);
        interceptor.call(Request::new(())).unwrap();
        let token = interceptor.token.clone().unwrap();
        interceptor.call(Request::new(())).unwrap();

        assert_eq!(token, interceptor.token.unwrap());
    }

    #[tokio::test]
    async fn service_account_interceptor_should_respect_token_lifetime_async() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let mut interceptor = ServiceAccountInterceptor::new(ZITADEL_URL, &sa, None);
        interceptor.call(Request::new(())).unwrap();
        let token = interceptor.token.clone().unwrap();
        interceptor.call(Request::new(())).unwrap();

        assert_eq!(token, interceptor.token.unwrap());
    }
}
