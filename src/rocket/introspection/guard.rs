use custom_error::custom_error;
use openidconnect::TokenIntrospectionResponse;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Request};

use crate::oidc::introspection::{introspect, IntrospectionError, ZitadelIntrospectionResponse};
use crate::rocket::introspection::IntrospectionConfig;

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

/// Struct for the injected route guard that requires an authenticated user.
/// Contains various information about the given token. The fields are optional
/// since a machine user does not have a profile or (varying by scope) not all
/// fields are returned from the introspection endpoint.
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
            email_verified: response.extra_fields().email_verified.clone(),
            locale: response.extra_fields().locale.clone(),
        }
    }
}

#[async_trait]
impl<'request> FromRequest<'request> for &'request IntrospectedUser {
    type Error = &'request IntrospectionGuardError;

    async fn from_request(request: &'request Request<'_>) -> Outcome<Self, Self::Error> {
        let auth: Vec<_> = request.headers().get("authorization").collect();
        if auth.len() > 1 {
            return Outcome::Failure((Status::BadRequest, &IntrospectionGuardError::InvalidHeader));
        } else if auth.len() == 0 {
            return Outcome::Failure((
                Status::Unauthorized,
                &IntrospectionGuardError::Unauthorized,
            ));
        }

        let token = auth[0];
        if !token.starts_with("Bearer ") {
            return Outcome::Failure((Status::Unauthorized, &IntrospectionGuardError::WrongScheme));
        }

        let result = request
            .local_cache_async(async {
                let token = token.replace("Bearer ", "");

                let config = request.rocket().state::<IntrospectionConfig>();
                if config.is_none() {
                    return Err((
                        Status::InternalServerError,
                        IntrospectionGuardError::MissingConfig,
                    ));
                }

                let config = config.unwrap();
                let result = introspect(
                    &config.introspection_uri,
                    &config.authority,
                    &config.authentication,
                    &token,
                )
                .await;

                if let Err(source) = result {
                    return Err((
                        Status::InternalServerError,
                        IntrospectionGuardError::Introspection { source },
                    ));
                }

                let result = result.unwrap();
                match result.active() {
                    true if result.sub().is_some() => Ok(result.into()),
                    false => Err((Status::Unauthorized, IntrospectionGuardError::Inactive)),
                    _ => Err((Status::Unauthorized, IntrospectionGuardError::NoUserId)),
                }
            })
            .await;

        match result {
            Ok(user) => Outcome::Success(user),
            Err((status, error)) => Outcome::Failure((*status, error)),
        }
    }
}
