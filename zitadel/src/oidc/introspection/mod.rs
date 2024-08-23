use custom_error::custom_error;
use openidconnect::http::Method;
use openidconnect::reqwest::async_http_client;
use openidconnect::url::{ParseError, Url};
use openidconnect::{
    core::CoreTokenType, ExtraTokenFields, HttpRequest, StandardTokenIntrospectionResponse,
};

use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::credentials::{Application, ApplicationError};

#[cfg(feature = "introspection_cache")]
pub mod cache;

custom_error! {
    /// Error type for introspection related errors.
    pub IntrospectionError
        RequestFailed{source: openidconnect::reqwest::Error<reqwest::Error>} = "the introspection request did fail: {source}",
        PayloadSerialization = "could not correctly serialize introspection payload",
        JWTProfile{source: ApplicationError} = "could not create signed jwt key: {source}",
        ParseUrl{source: ParseError} = "could not parse url: {source}",
        ParseResponse{source: serde_json::Error} = "could not parse introspection response: {source}",
        DecodeResponse{source: base64::DecodeError} = "could not decode base64 metadata: {source}",
}

/// Introspection response information that is returned by the ZITADEL
/// introspection endpoint. Introspection returns the
/// [specified information](https://zitadel.com/docs/apis/openidoauth/endpoints#introspect-response)
/// by default and [additional claims](https://zitadel.com/docs/apis/openidoauth/claims)
/// if requested by scope:
/// - When scope contains `urn:zitadel:iam:user:resourceowner`, the fields prefixed with
///  `resource_owner_` are set.
/// - When scope contains `urn:zitadel:iam:user:metadata`, the metadata hashmap will be
///   filled with the user metadata.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZitadelIntrospectionExtraTokenFields<Role = String>
where
    Role: Hash + Eq + Clone,
{
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub locale: Option<String>,
    #[serde(rename = "urn:zitadel:iam:user:resourceowner:id")]
    pub resource_owner_id: Option<String>,
    #[serde(rename = "urn:zitadel:iam:user:resourceowner:name")]
    pub resource_owner_name: Option<String>,
    #[serde(rename = "urn:zitadel:iam:user:resourceowner:primary_domain")]
    pub resource_owner_primary_domain: Option<String>,
    #[serde(rename = "urn:zitadel:iam:org:project:roles")]
    pub project_roles: Option<HashMap<Role, HashMap<String, String>>>,
    #[serde(rename = "urn:zitadel:iam:user:metadata")]
    pub metadata: Option<HashMap<String, String>>,
}

impl<Role: Debug + Hash + Eq + DeserializeOwned + Serialize + Clone> ExtraTokenFields
    for ZitadelIntrospectionExtraTokenFields<Role>
{
}

/// Type alias for the ZITADEL introspection response.
pub type ZitadelIntrospectionResponse<Role = String> =
    StandardTokenIntrospectionResponse<ZitadelIntrospectionExtraTokenFields<Role>, CoreTokenType>;

/// Definition of the authentication scheme against the authority (or issuer). This authentication
/// is required when performing actions like introspection against any ZITADEL instance.
///
/// As an example, when a web API wants to check an incoming user token, it authenticates itself
/// against ZITADEL with basic credentials (client id/secret) and then checks if the presented
/// user token is valid.
///
/// [JWTProfile](https://www.rfc-editor.org/rfc/rfc9068) is the recommended way to authenticate
/// against ZITADEL as it provides better security and faster response times.
#[derive(Debug, Clone)]
pub enum AuthorityAuthentication {
    /// Basic authentication uses client id/secret as HTTP Basic authentication.
    Basic {
        client_id: String,
        client_secret: String,
    },
    /// JWTProfile uses an [Application] with a private RSA key to create and sign
    /// a JWT token. This token authenticates the application against ZITADEL.
    /// This is the recommended way since it provides better security and faster responses.
    JWTProfile { application: Application },
}

fn headers(auth: &AuthorityAuthentication) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(ACCEPT, "application/json".parse().unwrap());
    headers.append(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    match auth {
        AuthorityAuthentication::Basic {
            client_id,
            client_secret,
        } => {
            headers.append(
                AUTHORIZATION,
                format!(
                    "Basic {}",
                    base64::encode(&format!("{}:{}", client_id, client_secret))
                )
                .parse()
                .unwrap(),
            );
            headers
        }
        AuthorityAuthentication::JWTProfile { .. } => headers,
    }
}

fn payload(
    authority: &str,
    auth: &AuthorityAuthentication,
    token: &str,
) -> Result<String, IntrospectionError> {
    match auth {
        AuthorityAuthentication::Basic { .. } => serde_urlencoded::to_string([("token", token)])
            .map_err(|_| IntrospectionError::PayloadSerialization),
        AuthorityAuthentication::JWTProfile { application } => {
            let jwt = application
                .create_signed_jwt(authority)
                .map_err(|source| IntrospectionError::JWTProfile { source })?;

            serde_urlencoded::to_string([
                (
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                ),
                ("client_assertion", &jwt),
                ("token", token),
            ])
            .map_err(|_| IntrospectionError::PayloadSerialization)
        }
    }
}

/// Perform an [OAuth 2.0 Token Introspection](https://www.rfc-editor.org/rfc/rfc7662)
/// against a given ZITADEL authority (instance). The function does not interpret the result
/// of the call but only returns the introspection result.
///
/// ### Errors
///
/// The introspection may fail if:
/// - The introspection call fails (bad request, unauthorized, other http errors)
/// - The response of the introspection could not be parsed
///
/// ### Example
///
/// #### Introspect the Personal Access Token of the Library Test ZITADEL Instance:
///
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
/// # use zitadel::oidc::discovery::discover;
/// # use zitadel::oidc::introspection::{AuthorityAuthentication, introspect};
/// let auth = AuthorityAuthentication::Basic {
///     client_id: "194339055499018497@zitadel_rust_test".to_string(),
///     client_secret: "Ip56oGzxKL1rJ8JaleUVKL7qUlpZ1tqHQYRSd6JE1mTlTJ3pDkDzoObHdZsOg88B".to_string(),
/// };
/// let authority = "https://zitadel-libraries-l8boqa.zitadel.cloud";
/// let token = "dEnGhIFs3VnqcQU5D2zRSeiarB1nwH6goIKY0J8MWZbsnWcTuu1C59lW9DgCq1y096GYdXA";
/// let metadata = discover(authority).await?;
///
/// let result = introspect::<String>(
///     metadata.additional_metadata().introspection_endpoint.as_ref().unwrap(),
///     authority,
///     &auth,
///     token,
/// ).await?;
///
/// println!("{:?}", result);
/// # Ok(())
/// # }
/// ```
pub async fn introspect<Role: Hash + Debug + Eq + DeserializeOwned + Serialize + Clone>(
    introspection_uri: &str,
    authority: &str,
    authentication: &AuthorityAuthentication,
    token: &str,
) -> Result<ZitadelIntrospectionResponse<Role>, IntrospectionError> {
    let response = async_http_client(HttpRequest {
        url: Url::parse(introspection_uri)
            .map_err(|source| IntrospectionError::ParseUrl { source })?,
        method: Method::POST,
        headers: headers(authentication),
        body: payload(authority, authentication, token)?.into_bytes(),
    })
    .await
    .map_err(|source| IntrospectionError::RequestFailed { source })?;

    let mut response: ZitadelIntrospectionResponse<Role> =
        serde_json::from_slice(response.body.as_slice())
            .map_err(|source| IntrospectionError::ParseResponse { source })?;
    decode_metadata::<Role>(&mut response)?;
    Ok(response)
}

// Metadata values are base64 encoded.
fn decode_metadata<Role: Hash + Debug + Eq + DeserializeOwned + Serialize + Clone>(
    response: &mut ZitadelIntrospectionResponse<Role>,
) -> Result<(), IntrospectionError> {
    if let Some(h) = &response.extra_fields().metadata {
        let mut extra: ZitadelIntrospectionExtraTokenFields<Role> = response.extra_fields().clone();
        let mut metadata = HashMap::new();
        for (k, v) in h {
            let decoded_v = base64::decode(v)
                .map_err(|source| IntrospectionError::DecodeResponse { source })?;
            let decoded_v = String::from_utf8_lossy(&decoded_v).into_owned();
            metadata.insert(k.clone(), decoded_v);
        }
        extra.metadata.replace(metadata);
        response.set_extra_fields(extra)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    use crate::oidc::discovery::discover;
    use openidconnect::TokenIntrospectionResponse;

    use super::*;

    const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    const PERSONAL_ACCESS_TOKEN: &str =
        "dEnGhIFs3VnqcQU5D2zRSeiarB1nwH6goIKY0J8MWZbsnWcTuu1C59lW9DgCq1y096GYdXA";

    #[tokio::test]
    async fn introspect_fails_with_invalid_url() {
        let result = introspect::<String>(
            "foobar",
            "foobar",
            &AuthorityAuthentication::Basic {
                client_id: "".to_string(),
                client_secret: "".to_string(),
            },
            "token",
        )
        .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IntrospectionError::ParseUrl { .. }
        ));
    }

    #[tokio::test]
    async fn introspect_fails_with_invalid_endpoint() {
        let meta = discover(ZITADEL_URL).await.unwrap();
        let result = introspect::<String>(
            &meta.token_endpoint().unwrap().to_string(),
            ZITADEL_URL,
            &AuthorityAuthentication::Basic {
                client_id: "".to_string(),
                client_secret: "".to_string(),
            },
            "token",
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn introspect_succeeds() {
        let meta = discover(ZITADEL_URL).await.unwrap();
        let result = introspect::<String>(
            &meta
                .additional_metadata()
                .introspection_endpoint
                .as_ref()
                .unwrap()
                .to_string(),
            ZITADEL_URL,
            &AuthorityAuthentication::Basic {
                client_id: "194339055499018497@zitadel_rust_test".to_string(),
                client_secret: "Ip56oGzxKL1rJ8JaleUVKL7qUlpZ1tqHQYRSd6JE1mTlTJ3pDkDzoObHdZsOg88B"
                    .to_string(),
            },
            PERSONAL_ACCESS_TOKEN,
        )
        .await
        .unwrap();

        assert!(result.active());
    }
}
