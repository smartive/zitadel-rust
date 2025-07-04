use custom_error::custom_error;
use rocket::figment::Figment;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Request};
use std::collections::BTreeSet;

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{
        Object, Responses, SecurityRequirement, SecurityScheme, SecuritySchemeData, MediaType,
        RefOr, Response,
    },
    okapi::Map,
    request::{OpenApiFromRequest, RequestHeaderInput},
};
#[cfg(feature = "rocket_okapi")]
use schemars::schema::{InstanceType, ObjectValidation, Schema, SchemaObject};
#[cfg(feature = "rocket_okapi")]

use crate::oidc::introspection::{claims::ZitadelClaims, introspect, IntrospectionError};
use crate::rocket::introspection::IntrospectionConfig;

use super::config::IntrospectionRocketConfig;

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

/// Type alias for the extracted user.
/// 
/// The extracted user will always be valid when fetched in request function arguments.
/// If not, the API will return with an appropriate error.
///
/// # Example
///
/// ```
/// use rocket::{get, State};
/// use zitadel::rocket::introspection::IntrospectedUser;
///
/// #[get("/protected")]
/// async fn protected_route(user: &IntrospectedUser) -> &'static str {
///     if !user.has_role("admin") {
///        return "Admin access required";
///     }
///     
///     if user.has_role_in_project("project123", "editor") {
///         return "Hello Editor";
///     }
///     
///     "Hello Admin"
/// }
/// ```
pub type IntrospectedUser = ZitadelClaims;

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

                let claims = result.unwrap();
                
                if !claims.active {
                    return Err((Status::Unauthorized, IntrospectionGuardError::Inactive));
                }
                
                if claims.sub.is_empty() {
                    return Err((Status::Unauthorized, IntrospectionGuardError::NoUserId));
                }
                
                Ok(claims)
            })
            .await;

        match result {
            Ok(user) => Outcome::Success(user),
            Err((status, error)) => Outcome::Error((*status, error)),
        }
    }
}

#[cfg(feature = "rocket_okapi")]
impl<'a> OpenApiFromRequest<'a> for &'a IntrospectedUser {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let figment: Figment = rocket::Config::figment();
        let config: IntrospectionRocketConfig = figment
            .extract()
            .expect("authority must be set in Rocket.toml");

        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Use OpenID Connect to authenticate. (does not work in RapiDoc at all)".to_owned(),
            ),
            data: SecuritySchemeData::OpenIdConnect {
                open_id_connect_url: format!(
                    "{}/.well-known/openid-configuration",
                    config.authority
                ),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("OpenID".to_owned(), Vec::new());
        // These vvvv-------^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "OpenID".to_owned(),
            security_scheme,
            security_req,
        ))
    }

    fn get_responses(_gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut res = Responses::default();

        // Manually defining the error response schema
        let error_detail_schema = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                properties: {
                    let mut properties = Map::new();
                    properties.insert(
                        "code".to_owned(),
                        Schema::Object(SchemaObject {
                            instance_type: Some(InstanceType::Integer.into()),
                            ..Default::default()
                        }),
                    );
                    properties.insert(
                        "reason".to_owned(),
                        Schema::Object(SchemaObject {
                            instance_type: Some(InstanceType::String.into()),
                            ..Default::default()
                        }),
                    );
                    properties.insert(
                        "description".to_owned(),
                        Schema::Object(SchemaObject {
                            instance_type: Some(InstanceType::String.into()),
                            ..Default::default()
                        }),
                    );
                    properties
                },
                required: vec!["code".to_owned(), "reason".to_owned(), "description".to_owned()]
                    .into_iter()
                    .collect::<BTreeSet<_>>(), // Convert Vec to BTreeSet
                ..Default::default()
            })),
            ..Default::default()
        };

        let error_response_schema = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                properties: {
                    let mut properties = Map::new();
                    properties.insert("error".to_owned(), Schema::Object(error_detail_schema));
                    properties
                },
                required: vec!["error".to_owned()].into_iter().collect::<BTreeSet<_>>(), // Convert Vec to BTreeSet
                ..Default::default()
            })),
            ..Default::default()
        };

        // Create the content for the error response
        let mut content = Map::new();
        content.insert(
            "application/json".to_owned(),
            MediaType {
                schema: Some(Schema::Object(error_response_schema).into()),
                ..Default::default()
            },
        );

        // Adding 400 BadRequest response
        let bad_request_response = Response {
            description: "Bad Request - Multiple authorization headers found.".to_owned(),
            content: content.clone(),
            ..Default::default()
        };
        res.responses.insert("400".to_owned(), RefOr::Object(bad_request_response));

        // Adding 401 Unauthorized response
        let unauthorized_response = Response {
            description: "Unauthorized - The request requires user authentication.".to_owned(),
            content: content.clone(),
            ..Default::default()
        };
        res.responses.insert("401".to_owned(), RefOr::Object(unauthorized_response));

        Ok(res)
    }}

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
