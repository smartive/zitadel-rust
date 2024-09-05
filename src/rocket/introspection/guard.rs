use custom_error::custom_error;
use openidconnect::TokenIntrospectionResponse;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Request};
use std::collections::HashMap;

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
    pub project_roles: Option<HashMap<String, HashMap<String, String>>>,
    pub metadata: Option<HashMap<String, String>>,
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
            project_roles: response.extra_fields().project_roles.clone(),
            metadata: response.extra_fields().metadata.clone(),
        }
    }
}

#[async_trait]
impl<'request> FromRequest<'request> for &'request IntrospectedUser {
    type Error = &'request IntrospectionGuardError;

    async fn from_request(request: &'request Request<'_>) -> Outcome<Self, Self::Error> {
        let auth: Vec<_> = request.headers().get("authorization").collect();
        if auth.len() > 1 {
            return Outcome::Error((Status::BadRequest, &IntrospectionGuardError::InvalidHeader));
        } else if auth.is_empty() {
            return Outcome::Error((Status::Unauthorized, &IntrospectionGuardError::Unauthorized));
        }

        let token = auth[0];
        if !token.starts_with("Bearer ") {
            return Outcome::Error((Status::Unauthorized, &IntrospectionGuardError::WrongScheme));
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

                #[cfg(feature = "introspection_cache")]
                let result = async {
                    if let Some(cache) = &config.cache {
                        if let Some(response) = cache.get(&token).await {
                            return Ok(response);
                        }
                    }

                    let response = introspect(
                        &config.introspection_uri,
                        &config.authority,
                        &config.authentication,
                        &token,
                    )
                    .await;

                    if let Some(cache) = &config.cache {
                        if let Ok(response) = &response {
                            cache.set(&token, response.clone()).await;
                        }
                    }

                    response
                }
                .await;

                #[cfg(not(feature = "introspection_cache"))]
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
            Err((status, error)) => Outcome::Error((*status, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    use std::thread;

    use rocket::http::Header;
    use rocket::{get, local::blocking::Client, uri};
    use tokio::runtime::Builder;

    use crate::{credentials::Application, rocket::introspection::IntrospectionConfigBuilder};

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

    fn get_config() -> IntrospectionConfig {
        let config = thread::spawn(move || {
            let rt = Builder::new_multi_thread().enable_all().build().unwrap();
            rt.block_on(async {
                IntrospectionConfigBuilder::new(ZITADEL_URL)
                    .with_jwt_profile(Application::load_from_json(APPLICATION).unwrap())
                    .build()
                    .await
                    .unwrap()
            })
        });

        config.join().unwrap()
    }

    #[get("/")]
    fn route(_user: &IntrospectedUser) -> &'static str {
        "Hello Rocket"
    }

    mod without_config {
        #![allow(clippy::all)]
        #![allow(dead_code)]

        use rocket::{launch, routes};

        #[launch]
        pub(super) fn rocket() -> _ {
            rocket::build().mount("/", routes![super::route])
        }
    }

    mod with_config {
        #![allow(clippy::all)]
        #![allow(dead_code)]

        use rocket::{launch, routes};

        #[launch]
        pub(super) fn rocket() -> _ {
            rocket::build()
                .mount("/", routes![super::route])
                .manage(super::get_config())
        }
    }

    #[test]
    fn guard_fails_if_no_auth_header_present() {
        let client = Client::tracked(with_config::rocket()).expect("valid rocket instance");
        let response = client.get(uri!(route)).dispatch();

        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn guard_fails_if_multiple_auth_header_present() {
        let client = Client::tracked(with_config::rocket()).expect("valid rocket instance");
        let mut request = client.get(uri!(route));
        request.add_header(Header::new("authorization", "foo"));
        request.add_header(Header::new("authorization", "bar"));
        let response = request.dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn guard_fails_if_non_bearer_auth_header_present() {
        let client = Client::tracked(with_config::rocket()).expect("valid rocket instance");
        let mut request = client.get(uri!(route));
        request.add_header(Header::new("authorization", "Basic Whatever"));
        let response = request.dispatch();

        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn guard_fails_if_no_config_present() {
        let client = Client::tracked(without_config::rocket()).expect("valid rocket instance");
        let mut request = client.get(uri!(route));
        request.add_header(Header::new(
            "authorization",
            format!("Bearer {}", PERSONAL_ACCESS_TOKEN),
        ));
        let response = request.dispatch();

        assert_eq!(response.status(), Status::InternalServerError);
    }

    #[test]
    fn authentication_fails_on_inactive_token() {
        let client = Client::tracked(with_config::rocket()).expect("valid rocket instance");
        let mut request = client.get(uri!(route));
        request.add_header(Header::new(
            "authorization",
            format!("Bearer {}", "PERSONAL_ACCESS_TOKEN"),
        ));
        let response = request.dispatch();

        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn authentication_succeeds_on_valid_token() {
        let client = Client::tracked(with_config::rocket()).expect("valid rocket instance");
        let mut request = client.get(uri!(route));
        request.add_header(Header::new(
            "authorization",
            format!("Bearer {}", PERSONAL_ACCESS_TOKEN),
        ));
        let response = request.dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}
