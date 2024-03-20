use std::{future::Future, pin::Pin};

use actix_web::dev::Payload;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, FromRequest, HttpRequest};
use openidconnect::TokenIntrospectionResponse;

use crate::actix::introspection::config::IntrospectionConfig;
use crate::oidc::introspection::{introspect, ZitadelIntrospectionResponse};

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
            email_verified: response.extra_fields().email_verified,
            locale: response.extra_fields().locale.clone(),
        }
    }
}

impl FromRequest for IntrospectedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<IntrospectedUser, Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let config = req.app_data::<IntrospectionConfig>();
        if config.is_none() {
            return Box::pin(async {
                Err(ErrorInternalServerError("IntrospectionConfig missing"))
            });
        }

        let auth = req.headers().get("authorization");
        if auth.is_none() {
            return Box::pin(async { Err(ErrorUnauthorized("Authorization header missing")) });
        }

        let auth = auth.unwrap().to_str();
        if auth.is_err() {
            return Box::pin(async { Err(ErrorUnauthorized("Authorization header invalid")) });
        }

        let token = auth.unwrap();
        if !token.starts_with("Bearer ") {
            return Box::pin(async {
                Err(ErrorUnauthorized("Authorization header has wrong scheme"))
            });
        }

        let config = config.unwrap().clone();
        let token = token.replace("Bearer ", "");

        Box::pin(async move {
            let result = introspect(
                &config.introspection_uri,
                &config.authority,
                &config.authentication,
                &token,
            )
            .await;

            if let Err(source) = result {
                return Err(ErrorInternalServerError(source));
            }

            let result = result.unwrap();
            match result.active() {
                true if result.sub().is_some() => Ok(result.into()),
                false => Err(ErrorUnauthorized("User not active")),
                _ => Err(ErrorUnauthorized("User not found")),
            }
        })
    }
}
