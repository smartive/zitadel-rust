use custom_error::custom_error;
use openidconnect::url::{ParseError, Url};
use openidconnect::{
    core::CoreTokenType, ExtraTokenFields, StandardTokenIntrospectionResponse,
    TokenIntrospectionResponse,
};
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Header, TokenData, Validation};
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use std::str::FromStr;
use reqwest::{Client};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use crate::credentials::{Application, ApplicationError};
use crate::oidc::discovery::{discover, DiscoveryError};

#[cfg(feature = "introspection_cache")]
pub mod cache;
pub mod claims;

use self::claims::ZitadelClaims;

custom_error! {
    /// Error type for introspection related errors.
    pub IntrospectionError
        HttpClientError{source: reqwest::Error} = "could not create http client: {source}",
        RequestFailed{origin: String, source: openidconnect::reqwest::Error} = "{origin} request did fail: {source}",
        PayloadSerialization = "could not correctly serialize introspection payload",
        JWTProfile{source: ApplicationError} = "could not create signed jwt key: {source}",
        ParseUrl{source: ParseError} = "could not parse url: {source}",
        ParseResponse{source: serde_json::Error} = "could not parse introspection response: {source}",
        DecodeResponse{source: base64::DecodeError} = "could not decode base64 metadata: {source}",
        ResponseError{source: ZitadelResponseError} = "received error response from Zitadel: {source}",
        DiscoveryError{source: DiscoveryError} = "Discovery error during introspection: {source}",
        JWTUnsupportedAlgorithm = "unsupported algorithm in JWT",
        MissingJwksKey = "missing key in jwks",
        JsonWebTokenErrors{source: jsonwebtoken::errors::Error} = @{ match source.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid JWT string, missing the 3 . segments, JKWS validation won't work with opaque tokens, make sure you've switched to JWT tokens or use instead the instropect method",
            jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName => "Invalid Algorithm in JWKS",
            _ => "Other JWT error"
        }},
        TokenNotActive = "token is not active",
}

/// Introspection response information that is returned by the ZITADEL
/// introspection endpoint. Introspection returns the
/// [specified information](https://zitadel.com/docs/apis/openidoauth/endpoints#introspect-response)
/// by default and [additional claims](https://zitadel.com/docs/apis/openidoauth/claims)
/// if requested by scope:
/// - When scope contains `urn:zitadel:iam:user:resourceowner`, the fields prefixed with
///   `resource_owner_` are set.
/// - When scope contains `urn:zitadel:iam:user:metadata`, the metadata hashmap will be
///    filled with the user metadata.
///  - When scope contains `urn:zitadel:iam:org:projects:roles`, the project_roles hashmap will be
///    filled with the project roles.
///  - When using custom claims through Zitadel Actions, the custom_claims hashmap will be filled with
///    the custom claims. [custom claims](https://zitadel.com/docs/apis/openidoauth/claims#custom-claims)
///
/// It can be used as a basis for further customized authorization checks, for example:
/// ```
/// use std::collections::HashMap;
/// use zitadel::axum::introspection::IntrospectedUser;
/// use zitadel::oidc::introspection::{ZitadelIntrospectionExtraTokenFields, ZitadelIntrospectionResponse};
/// use serde_json::Error as SerdeError;
///
///
/// #[derive(Debug, serde::Deserialize, serde::Serialize)]
///  pub struct CustomClaims {
///     pub name: Option<String>,
///     pub given_name: Option<String>,
///     pub family_name: Option<String>,
///     pub preferred_username: Option<String>,
///     pub email: Option<String>,
///     pub email_verified: Option<bool>,
///     pub locale: Option<String>,
///     #[serde(rename = "urn:zitadel:iam:user:resourceowner:id")]
///     pub resource_owner_id: Option<String>,
///     #[serde(rename = "urn:zitadel:iam:user:resourceowner:name")]
///     pub resource_owner_name: Option<String>,
///     #[serde(rename = "urn:zitadel:iam:user:resourceowner:primary_domain")]
///     pub resource_owner_primary_domain: Option<String>,
///     #[serde(rename = "urn:zitadel:iam:user:metadata")]
///     pub metadata: Option<HashMap<String, String>>,
///     #[serde(rename = "my:zitadel:grants")]
///     pub flat_roles: Option<Vec<String>>,
///     #[serde(rename = "year")]
///     pub anum: Option<String>,
/// }
///
/// enum Role {
///   Admin,
///   Client
/// }
///
/// trait MyAuthorizationChecks {
///     fn has_role(&self, role: Role, org_id: &str) -> bool;
/// }
///
/// impl MyAuthorizationChecks for ZitadelIntrospectionExtraTokenFields {
///     fn has_role(&self, role: Role, org_id: &str) -> bool {
///         let role = match role {
///             Role::Admin => "Admin",
///             Role::Client => "Client",
///         };
///         self.project_roles.as_ref()
///             .and_then(|roles| roles.get(role))
///             .map(|org_ids| org_ids.contains_key(org_id))
///             .unwrap_or(false)
///     }
/// }
///
/// pub trait ExtIntrospectedUser {
///     fn custom_claims(&self) -> Result<CustomClaims, SerdeError>;
/// }
/// impl ExtIntrospectedUser for ZitadelIntrospectionResponse {
///     fn custom_claims(&self) -> Result<CustomClaims, SerdeError> {
///         let as_value = serde_json::to_value(self)?;
///         let custom_claims: CustomClaims = serde_json::from_value(as_value)?;
///         Ok(custom_claims)
///     }
/// }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZitadelIntrospectionExtraTokenFields {
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
    pub project_roles: Option<HashMap<String, HashMap<String, String>>>,
    #[serde(rename = "urn:zitadel:iam:user:metadata")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(flatten)]
    custom_claims: Option<HashMap<String, JsonValue>>
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
/// against a given ZITADEL authority (instance) and return simplified token claims.
///
/// ### Errors
///
/// The introspection may fail if:
/// - The introspection call fails (bad request, unauthorized, other http errors)
/// - The response of the introspection could not be parsed
/// - The token is not active
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
/// let claims = introspect(
///     metadata.additional_metadata().introspection_endpoint.as_ref().unwrap(),
///     authority,
///     &auth,
///     token,
/// ).await?;
///
/// println!("User ID: {}", claims.sub);
/// if claims.has_role("admin") {
///     println!("User is an admin");
/// }
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "oidc")]
pub async fn introspect(
    introspection_uri: &str,
    authority: &str,
    authentication: &AuthorityAuthentication,
    token: &str,
) -> Result<ZitadelClaims, IntrospectionError> {
    let response = introspect_raw(introspection_uri, authority, authentication, token).await?;
    
    // Check if token is active
    if !response.active() {
        return Err(IntrospectionError::TokenNotActive);
    }
    
    Ok(response.into())
}

/// Perform an [OAuth 2.0 Token Introspection](https://www.rfc-editor.org/rfc/rfc7662)
/// and return the raw introspection response.
/// 
/// This function is for advanced use cases where you need access to the full response.
/// For most use cases, use [`introspect`] which returns simplified token claims.
pub async fn introspect_raw(
    introspection_uri: &str,
    authority: &str,
    authentication: &AuthorityAuthentication,
    token: &str,
) -> Result<ZitadelIntrospectionResponse, IntrospectionError> {
    let async_http_client = reqwest::ClientBuilder::new().redirect(reqwest::redirect::Policy::none()).build()?;

    let url= Url::parse(introspection_uri)
            .map_err(|source| IntrospectionError::ParseUrl { source })?;
    let response = async_http_client
        .post(url)
        .headers(headers(authentication))
        .body(payload(authority, authentication, token)?)
        .send()
        .await
        .map_err(|source| IntrospectionError::RequestFailed {origin: "The introspection".to_string(),  source })?;

    if !response.status().is_success() {
        let status = response.status();
        let body_bytes = response.bytes().await?;

        return Err(IntrospectionError::ResponseError {
            source: ZitadelResponseError::new(status, &body_bytes),
        });
    }

    let mut response: ZitadelIntrospectionResponse =
        serde_json::from_slice(response.bytes().await?.to_vec().as_slice())
            .map_err(|source| IntrospectionError::ParseResponse { source })?;
    decode_metadata(&mut response)?;
    Ok(response)
}

#[derive(Debug)]
struct ZitadelResponseError {
    status_code: String,
    body: String,
}
impl ZitadelResponseError {
        fn new(status_code: reqwest::StatusCode, body: &[u8]) -> Self {
            Self {
                status_code: status_code.to_string(),
                body: String::from_utf8_lossy(body).to_string(),
            }
        }
}

impl Display for ZitadelResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status code: {}, body: {}", self.status_code, self.body)
    }
}
impl Error for ZitadelResponseError {}

// Metadata values are base64 encoded.
fn decode_metadata(response: &mut ZitadelIntrospectionResponse) -> Result<(), IntrospectionError> {
    if let Some(h) = &response.extra_fields().metadata {
        let mut extra: ZitadelIntrospectionExtraTokenFields = response.extra_fields().clone();
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


/// Fetch the JSON Web Key Set (JWKS) from a ZITADEL instance.
/// 
/// This function retrieves the public keys used by ZITADEL to sign JWT tokens.
/// These keys can be used for local validation of tokens without making introspection calls.
/// 
/// # Arguments
/// 
/// * `idm_url` - The base URL of the ZITADEL instance (e.g., "https://my-instance.zitadel.cloud")
/// 
/// # Returns
/// 
/// Returns a `JwkSet` containing the public keys.
/// 
/// # Example
/// 
/// ```no_run
/// # use zitadel::oidc::introspection::fetch_jwks;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let jwks = fetch_jwks("https://my-instance.zitadel.cloud").await?;
/// println!("Fetched {} keys", jwks.keys.len());
/// # Ok(())
/// # }
/// ```
pub async fn fetch_jwks(idm_url: &str) -> Result<JwkSet, IntrospectionError> {
    let client: Client = Client::new();
    let openid_config = discover(idm_url).await.map_err(|err| {
        IntrospectionError::DiscoveryError { source: err }
    })?;
    let jwks_url = openid_config.jwks_uri().url().as_ref();
    let response = client
        .get(jwks_url)
        .send()
        .await?;
    let jwks_keys: JwkSet = response.json::<JwkSet>().await.map_err(|err| IntrospectionError::RequestFailed {origin: "Could not fetch jwks keys because ".to_string(), source: err })?;
    Ok(jwks_keys)
}


/// Validate a JWT token locally using JWKS keys.
/// 
/// This function validates a JWT token without making a network call to the introspection endpoint.
/// It uses the provided JWKS keys to verify the token signature and validates the token claims.
/// 
/// # Arguments
/// 
/// * `issuers` - List of valid issuers to check against the token's "iss" claim
/// * `audiences` - List of valid audiences to check against the token's "aud" claim
/// * `jwks_keys` - The JWKS keys to use for signature validation
/// * `token` - The JWT token to validate
/// 
/// # Returns
/// 
/// Returns the deserialized token claims if validation succeeds.
/// 
/// # Example
/// 
/// ```no_run
/// # use zitadel::oidc::introspection::{fetch_jwks, local_jwt_validation};
/// # use zitadel::oidc::introspection::claims::ZitadelClaims;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let jwks = fetch_jwks("https://my-instance.zitadel.cloud").await?;
/// let claims: ZitadelClaims = local_jwt_validation(
///     &["https://my-instance.zitadel.cloud"],
///     &["my-client-id"],
///     jwks,
///     "eyJ..."
/// ).await?;
/// println!("User ID: {}", claims.sub);
/// # Ok(())
/// # }
/// ```
pub async fn local_jwt_validation<U>(issuers: &[&str],
                                  audiences: &[&str],
                                  jwks_keys: JwkSet,
                                  token: &str, ) -> Result<U, IntrospectionError>

where
    U: DeserializeOwned,
{

    let unverified_token_header: Header = decode_header(token).map_err(|source| IntrospectionError::JsonWebTokenErrors { source })?;
    let user_kid = match unverified_token_header.kid {
        Some(k) => k,
        None => return Err(IntrospectionError::MissingJwksKey),
    };
    if let Some(j) = jwks_keys.find(&user_kid) {
        match &j.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?;
                let algorithm_key = j.common.key_algorithm.ok_or(IntrospectionError::JWTUnsupportedAlgorithm)?;
                let algorithm_str = format!("{}", algorithm_key);
                let algorithm = Algorithm::from_str(&algorithm_str).map_err(|source| IntrospectionError::JsonWebTokenErrors { source })?;
                let mut validation = Validation::new(algorithm);
                validation.set_audience(audiences);
                validation.leeway = 5;
                validation.set_issuer(issuers);
                validation.validate_exp = true;

                let decoded_token: TokenData<U> = decode::<U>(token, &decoding_key, &validation).map_err(|source| IntrospectionError::JsonWebTokenErrors { source })?;
                Ok(decoded_token.claims)
            }
            _ => unreachable!("Not yet Implemented or supported by Zitadel"),
        }
    } else {
        Err(IntrospectionError::MissingJwksKey)
    }
}


#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    use crate::oidc::discovery::discover;
    use openidconnect::TokenIntrospectionResponse;
    use crate::credentials::{AuthenticationOptions, ServiceAccount};
    use super::*;

    const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    const ZITADEL_URL_ALTER: &str = "https://ferris-hk3otq.us1.zitadel.cloud";
    const ZITADEL_ISSUERS: [&str; 1] = ["https://ferris-hk3otq.us1.zitadel.cloud"];
    const ZITADEL_AUDIENCES: [&str; 1] = ["service_user"];
    const SERVICE_ACCOUNT: &str = r#"{
    "type":"serviceaccount",
    "keyId":"305100363495643757",
    "key":"-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQEAqqeKZxqy0upui0Ez7YDmm+hTtoSMAlTtFKjtkqn+AOKuicrj\ngzsZ9w/axLc0yv5JamG4iABRioGWi1x6lK09nNa9CcWod51SgihQc5Vpgi2grOBf\nk1FFb1JB3UcVJ7T6ix39cdEpK134FpfHAaef1Uzca/uX+Mj807Dkper0CZ/DV+2U\n97NzzqSK7F2jlZP/qHfCewjR9+uCT2sPqlrUxlyuz0f+Bl5K7ATA9RAMUVofdt20\nZNIRrKlRZJ9xnNUcZP8Gj5zd07OXuzS0kD+iL4a1rctGz1YIsPwxYglL0/rgzPzl\nYyoQUkbR+5dGfJZSKo3dUoWnrIleHkhNAnALzwIDAQABAoIBAC2c/HxUgYmoiYWF\nFwkbVAhGD+IQCZAx/PBxupZiA3dfH4HLDgasjGMiBLphsaW0VBEwL2+Cjkj0HDYB\nsO3+ZCKAryRmhYH7Net+NQq/2+Skp3atvj4VEfcQSHSJpFjpobH/gRej7VofEsmP\nJe0GTc1obt4Z8GPZ7OH0PRkv9KyBbEtVmM88H4/yXFL3N5HI4EpnZektW2yYKqzq\nkiwaqnT9ZUs/kjcJW1Vli5CzcMCExKab9zILrKDTmS4Bqz47osPEVVeCSj32PhCw\ny/F8Z22dU8lCsmSqRbxXCQLHI4F9nLorZfToyk/tGNH+LbGC7qiQEqX50IHkKzx6\nms/4LhkCgYEA4utIE6HDWaH+X/1kj4N/+WoSxLg4+3mVSCLvNTHZeVKcOOp8Mp/s\nN6gOaYetAbDdqT8sfyMT+7SIDTmYv5u8wuu3LRMXt9QSyQQGVGBaHC1++Raiz6tb\ncRd8T0ebYZd9JSPYciYL+t1bqOmaVNiLlMvY61oS8KmpAS9o/pOBnksCgYEAwIZX\nFRboNqRmgOezoz9kEPO0eWeg/XEHc6peUQM97tQbRSU33G6BH2YlphfGmkWt8Y6u\nvbYH7+B7BxX0VgXYt/MmBAQ8DQxSKsl+t19QrWS1xf5X1eyQ6OewlnoBU3cX0uaY\nEAgoodT5D/nf1i9WR2j8Ef1pBLwajjuG6hUnxg0CgYBwDjXClBAutAM0jaHaCNrq\nZIouILbq4Ahq3e14PEyjT7sblBeOvFBez5uGW1yAyEE9sZeclMrqciT5OucGP7bA\nHryPAq2ktpIsN9OUWRxGa+UWxinSGVGHkExvrfG6CJ/g9kmNXOJvmF4KFImEuoZ7\nDQrqdcmClJWDo1Da3iaU7wKBgQCDpFgvJ3aoxkkAo24FlfbKUJl62g0OvxalVD0h\nj+HtSENNSGGl7DmGSsY7h85Y9oQ1w6ZgOfO7Zfc5pR1pJJ5HSY8Y9/xHv8D/WL+4\niwgTR+Wy+HL+578+Qg5RFiOJ+sjjgKFBdRKzdXjIH8eaIMwSEAssEeaZQjW2Q6XA\nsa58kQKBgQCQ/6AeNZSfHpgnm4MKc7YCLK2oxsOwOlrHMrif6Yd7gPiaA1Ap+R6c\nHOpTkwT+3GsP+tnh2l7T7370mP+imU2K+3Tz1V/crgKqeOiPuQtq6dITdff0vBUH\niUCrltxb5I3uh8K+rE+zuLsBjrSrxwUP1trcXHhfEMSHYzxJ6Hz9Xw==\n-----END RSA PRIVATE KEY-----\n",
    "expirationDate":"9999-12-31T23:59:59Z",
    "userId":"305100342993885805"
    }"#;

    const PERSONAL_ACCESS_TOKEN: &str =
        "dEnGhIFs3VnqcQU5D2zRSeiarB1nwH6goIKY0J8MWZbsnWcTuu1C59lW9DgCq1y096GYdXA";

    const PERSONAL_ACCESS_TOKEN_ALTER : &str =
        "KyX1Pw1bVfYFSE0g6s3Io12I4sC-feEtkaShWstZJ0h34JHfE29q4oIOJFF0PZlfMDvaCvk";

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    pub struct CustomClaims {
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
        #[serde(rename = "urn:zitadel:iam:user:metadata")]
        pub metadata: Option<HashMap<String, String>>,
        #[serde(rename = "music")]
        pub taste: Option<String>,
        #[serde(rename = "year")]
        pub anum: Option<i32>,
     }

     pub trait ExtIntrospectedUser {
        fn custom_claims(&self) -> Result<CustomClaims, serde_json::Error>;
     }
     impl ExtIntrospectedUser for ZitadelIntrospectionResponse {
         fn custom_claims(&self) -> Result<CustomClaims, serde_json::Error> {
            let as_value = serde_json::to_value(self)?;
             let custom_claims: CustomClaims = serde_json::from_value(as_value)?;
            Ok(custom_claims)
        }
     }

    #[tokio::test]
    async fn introspect_fails_with_invalid_url() {
        let result = introspect(
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
        let result = introspect(
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
        let result = introspect(
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

    #[tokio::test]
    async fn fetch_jwks_succeeds() {
        let jwks: JwkSet = fetch_jwks(ZITADEL_URL_ALTER).await.unwrap();
        assert!(!jwks.keys.is_empty());
    }

    #[tokio::test]
    async fn local_jwt_validation_succeeds() {
        //
        //  this use the following Action definition
        // /**
        //  * Add an additional claim to the token / userinfo, if it's not already present
        //  *
        //  * Flow: Complement token, Triggers: Pre Userinfo creation, Pre access token creation
        //  *
        //  * @param ctx
        //  * @param api
        //  */
        // function addClaim(ctx, api) {
        //   api.v1.claims.setClaim('year', 2025)
        //   api.v1.claims.setClaim('music', 'funk')
        // }
        //

        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let access_token = sa.authenticate_with_options(ZITADEL_URL_ALTER, &AuthenticationOptions {
          scopes: vec!["profile".to_string(), "email".to_string(), "urn:zitadel:iam:user:resourceowner".to_string()],
           ..Default::default()
         }).await.unwrap();
        // move fetch_jwks after login has jwks can be purged after 30 hours of no login
        let jwks: JwkSet = fetch_jwks(ZITADEL_URL_ALTER).await.unwrap();
        let result: CustomClaims = local_jwt_validation::<CustomClaims>(&ZITADEL_ISSUERS, &ZITADEL_AUDIENCES, jwks, &access_token).await.unwrap();
        assert_eq!(result.taste.unwrap(), "funk");
        assert_eq!(result.anum.unwrap(), 2025);
    }

    #[tokio::test]
    async fn introspect_custom_claims_succeeds() {
        let meta = discover(ZITADEL_URL_ALTER).await.unwrap();
        let result = introspect(
            &meta
                .additional_metadata()
                .introspection_endpoint
                .as_ref()
                .unwrap()
                .to_string(),
            ZITADEL_URL_ALTER,
            &AuthorityAuthentication::Basic {
                client_id: "305507367414524812".to_string(),
                client_secret: "hmXmOdeviMGkGYaiIh4PepElr8jgrCdwewfvnkfy1TugbwCodiflqFdH7Yfs7Ody"
                    .to_string(),
            },
            PERSONAL_ACCESS_TOKEN_ALTER,
        )
            .await
            .unwrap();

        let custom_claims = result.custom_claims().unwrap();

        assert!(result.active());
        assert_eq!(custom_claims.taste.unwrap(), "funk");
        assert_eq!(custom_claims.anum.unwrap(), 2025);
    }
}
