use std::{future::Future, pin::Pin};

use actix_web::dev::Payload;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, FromRequest, HttpRequest};
use custom_error::custom_error;

use crate::actix::introspection::config::IntrospectionConfig;
use crate::oidc::introspection::{claims::ZitadelClaims, introspect, IntrospectionError};

custom_error! {
    /// Error type for extractor related errors.
    pub IntrospectionExtractorError
        MissingConfig = "no introspection config given to actix app data",
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
/// use actix_web::{get, HttpResponse, Responder};
/// use zitadel::actix::introspection::IntrospectedUser;
///
/// #[get("/protected")]
/// async fn protected_route(user: IntrospectedUser) -> impl Responder {
///     if !user.has_role("admin") {
///        return HttpResponse::Forbidden().body("Admin access required");
///     }
///     
///     if user.has_role_in_project("project123", "editor") {
///         return HttpResponse::Ok().body("Hello Editor");
///     }
///     
///     HttpResponse::Ok().body("Hello Admin")
/// }
/// ```
pub type IntrospectedUser = ZitadelClaims;

impl FromRequest for IntrospectedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<IntrospectedUser, Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let config = req.app_data::<IntrospectionConfig>();
        if config.is_none() {
            return Box::pin(async {
                Err(ErrorInternalServerError(
                    IntrospectionExtractorError::MissingConfig,
                ))
            });
        }

        let auth = req.headers().get("authorization");
        if auth.is_none() {
            return Box::pin(async {
                Err(ErrorUnauthorized(IntrospectionExtractorError::Unauthorized))
            });
        }

        let auth = auth.unwrap().to_str();
        if auth.is_err() {
            return Box::pin(async {
                Err(ErrorUnauthorized(
                    IntrospectionExtractorError::InvalidHeader,
                ))
            });
        }

        let token = auth.unwrap();
        if !token.starts_with("Bearer ") {
            return Box::pin(async {
                Err(ErrorUnauthorized(IntrospectionExtractorError::WrongScheme))
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
                return Err(ErrorInternalServerError(
                    IntrospectionExtractorError::Introspection { source },
                ));
            }

            let claims = result.unwrap();
            
            if !claims.active {
                return Err(ErrorUnauthorized(IntrospectionExtractorError::Inactive));
            }
            
            if claims.sub.is_empty() {
                return Err(ErrorUnauthorized(IntrospectionExtractorError::NoUserId));
            }
            
            Ok(claims)
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    use actix_web::{get, test, App, Responder};

    use crate::{actix::introspection::IntrospectionConfigBuilder, credentials::Application};

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

    async fn get_config() -> IntrospectionConfig {
        IntrospectionConfigBuilder::new(ZITADEL_URL)
            .with_jwt_profile(Application::load_from_json(APPLICATION).unwrap())
            .build()
            .await
            .unwrap()
    }

    #[get("/")]
    async fn route(_user: IntrospectedUser) -> impl Responder {
        "Hello Actix"
    }

    #[tokio::test]
    async fn extractor_fails_if_no_config_present() {
        let app = test::init_service(App::new().service(route)).await;

        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 500);

        let body = test::read_body(resp).await.to_vec();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, IntrospectionExtractorError::MissingConfig.to_string());
    }

    #[tokio::test]
    async fn extractor_fails_if_no_auth_header_present() {
        let app = test::init_service(App::new().app_data(get_config().await).service(route)).await;

        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);

        let body = test::read_body(resp).await.to_vec();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, IntrospectionExtractorError::Unauthorized.to_string());
    }

    #[tokio::test]
    async fn extractor_fails_if_non_bearer_auth_header_present() {
        let app = test::init_service(App::new().app_data(get_config().await).service(route)).await;

        let req = test::TestRequest::default()
            .insert_header(("authorization", "Basic foobar"))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);

        let body = test::read_body(resp).await.to_vec();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, IntrospectionExtractorError::WrongScheme.to_string());
    }

    #[tokio::test]
    async fn authentication_fails_on_inactive_token() {
        let app = test::init_service(App::new().app_data(get_config().await).service(route)).await;

        let req = test::TestRequest::default()
            .insert_header((
                "authorization",
                format!("Bearer {}", "PERSONAL_ACCESS_TOKEN"),
            ))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);

        let body = test::read_body(resp).await.to_vec();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, IntrospectionExtractorError::Inactive.to_string());
    }

    #[tokio::test]
    async fn authentication_succeeds_on_valid_token() {
        let app = test::init_service(App::new().app_data(get_config().await).service(route)).await;

        let req = test::TestRequest::default()
            .insert_header(("authorization", format!("Bearer {}", PERSONAL_ACCESS_TOKEN)))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}
