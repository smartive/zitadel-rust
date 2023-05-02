use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::IntoResponse,
    Json, RequestPartsExt, TypedHeader,
};
use custom_error::custom_error;
use openidconnect::TokenIntrospectionResponse;
use reqwest::StatusCode;
use serde_json::json;

use crate::oidc::introspection::{introspect, IntrospectionError, ZitadelIntrospectionResponse};

use super::state::IntrospectionConfig;

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
            IntrospectionGuardError::Introspection { source } => {
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

#[derive(Debug)]
pub struct IntrospectedUser {
    /// UserID of the introspected user (OIDC Field "sub").
    pub user_id: String,
    pub username: Option<String>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub locale: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for IntrospectedUser
where
    IntrospectionConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = IntrospectionGuardError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| IntrospectionGuardError::InvalidHeader)?;

        let config = IntrospectionConfig::from_ref(state);

        let res = introspect(
            &config.introspection_uri,
            &config.authority,
            &config.authentication,
            &bearer.token(),
        )
        .await;

        let user: Result<IntrospectedUser, IntrospectionGuardError> = match res {
            Ok(res) => match res.active() {
                true if res.sub().is_some() => Ok(res.into()),
                false => Err(IntrospectionGuardError::Inactive),
                _ => Err(IntrospectionGuardError::NoUserId),
            },
            Err(source) => return Err(IntrospectionGuardError::Introspection { source }),
        };

        user
    }
}

impl From<ZitadelIntrospectionResponse> for IntrospectedUser {
    fn from(response: ZitadelIntrospectionResponse) -> Self {
        Self {
            user_id: response.sub().unwrap().to_string(),
            username: response.username().map(|s| s.to_string()),
            name: response.extra_fields().name.clone(),
            given_name: response.extra_fields().given_name.clone(),
            family_name: response.extra_fields().family_name.clone(),
            preferred_username: response.extra_fields().preferred_username.clone(),
            email: response.extra_fields().email.clone(),
            email_verified: response.extra_fields().email_verified,
            locale: response.extra_fields().locale.clone(),
        }
    }
}
