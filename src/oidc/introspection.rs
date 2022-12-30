use custom_error::custom_error;
use openidconnect::http::Method;
use openidconnect::reqwest::async_http_client;
use openidconnect::url::{ParseError, Url};
use openidconnect::{
    core::CoreTokenType, ExtraTokenFields, HttpRequest, StandardTokenIntrospectionResponse,
};
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use crate::credentials::{Application, ApplicationError};

custom_error! {
    pub IntrospectionError
        RequestFailed{source: openidconnect::reqwest::Error<reqwest::Error>} = "the introspection request did fail: {source}",
        PayloadSerialization = "could not correctly serialize introspection payload",
        JWTProfile{source: ApplicationError} = "could not create signed jwt key: {source}",
        ParseUrl{source: ParseError} = "could not parse url: {source}",
        ParseResponse{source: serde_json::Error} = "could not parse introspection response: {source}",
}

/// Introspection response information that is returned by the ZITADEL
/// introspection endpoint. Introspection returns the
/// [specified information](https://zitadel.com/docs/apis/openidoauth/endpoints#introspect-response)
/// by default and [additional claims](https://zitadel.com/docs/apis/openidoauth/claims)
/// if requested by scope.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZitadelIntrospectionExtraTokenFields {
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub locale: Option<String>,
}

impl ExtraTokenFields for ZitadelIntrospectionExtraTokenFields {}

/// Type alias for the ZITADEL introspection response.
pub type ZitadelIntrospectionResponse =
    StandardTokenIntrospectionResponse<ZitadelIntrospectionExtraTokenFields, CoreTokenType>;

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
        AuthorityAuthentication::Basic { .. } => serde_urlencoded::to_string(&[("token", token)])
            .map_err(|_| IntrospectionError::PayloadSerialization),
        AuthorityAuthentication::JWTProfile { application } => {
            let jwt = application
                .create_signed_jwt(authority)
                .map_err(|source| IntrospectionError::JWTProfile { source })?;

            serde_urlencoded::to_string(&[
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
/// let result = introspect(
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
pub async fn introspect(
    introspection_uri: &str,
    authority: &str,
    authentication: &AuthorityAuthentication,
    token: &str,
) -> Result<ZitadelIntrospectionResponse, IntrospectionError> {
    let response = async_http_client(HttpRequest {
        url: Url::parse(introspection_uri)
            .map_err(|source| IntrospectionError::ParseUrl { source })?,
        method: Method::POST,
        headers: headers(authentication),
        body: payload(authority, authentication, token)?.into_bytes(),
    })
    .await
    .map_err(|source| IntrospectionError::RequestFailed { source })?;

    serde_json::from_slice(response.body.as_slice())
        .map_err(|source| IntrospectionError::ParseResponse { source })
}
