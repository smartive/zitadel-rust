//! The introspection module allows
//! [OAuth 2.0 Token Introspection](https://www.rfc-editor.org/rfc/rfc7662) to be used
//! to authenticate a user against ZITADEL. The typical use-case is a web API that
//! receives calls from any source (maybe a Single Page Application) and needs to verify
//! if the given access token is valid and still active.
//!
//! In case of [actix-web](https://actix.rs/), ["extractors"](https://actix.rs/docs/extractors) are used to
//! extract the introspected user from the request. The request must contain a valid
//! authorization header (bearer token) and is then checked against the configured
//! authority.
//!
//! ### How to use the introspection extractor
//!
//! To use the introspection extractor, the following steps must be performed:
//! - Inject the (introspection-)config into the actix app
//! - Use the extractor in any route that need to be protected
//!
//! #### Configure Actix
//!
//! To enable the introspection, an [IntrospectionConfig] must be injected into
//! the [state](https://actix.rs/docs/application#state) of actix.
//! First, construct the configuration with the async builder method, and then
//! inject it into rocket.
//!
//! ```no_run
//! # use actix_web::{App, HttpServer};
//! # use zitadel::credentials::Application;
//! # use zitadel::actix::introspection::IntrospectionConfigBuilder;
//! # const APPLICATION: &str = r#"
//! #     {
//! #         "type": "application",
//! #         "keyId": "181963758610940161",
//! #         "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAwT2YZJytkkZ1DDM3dcu1OA8YPzHu6XR8HotdMNRnV75GhOT4\nB7zDtdtoP8w/1NHHPEJ859e0kYhrrnKikOKLS6fS1KRsmqR5ZvTq8SlZ2mq3RcX2\nebZx5dQt36INij/WXdsBmjM/yfWvqqWBSb0L/186DaWwmmIxoXWe873vxRmlzblg\nGd8Nu07s9YTREbGPbtFVHEUM6xI4oIe8HJ0e1+JBkiGqk31Cogo0FoAxrOAg0Sf4\n5XiUMYIjzqh8673F9SC4IpVxG22mpFk3vDFuAITaStWYbiH2hPJNKWyX9HDCZb1D\nDqa3wZBDiLqWxh22hNZ6ZIe+3UoSGWsPBH+E1wIDAQABAoIBAD2v5QsRPRN57HmF\njAnNir8nimz6CrN53Pl/MbOZypenBSn9UfReXPeb3+6lzCarBPgGnYsBQAJJU16v\n95daym7PVy1Mg+Ll6F9mhe2Qbr+b23+pj2IRTNC6aB6Aw+PDNzJk7GEGRTG6fWZz\nSQ96Cu9tvcGHiBXwjLlnK+PRWU5IsCiLsjT4xBXsMLMw3YOdMK5z58sqr+SnNEyq\nRHoEvi9aC94WrargVB45Yx+81YNW8uQ5rMDmYaJC5a7ENz522SlAuf4T+fAGJ/HE\n/qbZGD4YwlLqAFDgewQ+5tEWEus3zgY2MIR7vN2zXU1Ptk+mQkXZl/Pxdp7q1xU+\nvr/kcykCgYEAy7MiIAzc1ctQDvkk3HiespzdQ/sC7+CGsBzkyubRc9Oq/YR7GfVK\nGTuDEDlWwx92VAvJGDWRa3T426YDyqiPj66uo836sgL15Uigg5afZun2bqGC78le\nBhSy9b+0YDHPa87GxtKt9UmMoB6WdmoPzOkLEEGS7eesmk2DDgY+QSUCgYEA8tr/\n3PawigL1cxuFpcO1lH6XUspGeAo5yB8FXvfW5g50e37LgooIvOFgUlYuchxwr6uh\nW+CUAWmm4farsgvMBMPYw+PbkCTi/xemiiDmMHUYd7sJkTl0JXApq3pZsNMg4Fw/\n29RynmcG8TGe2dkwrWp1aBYjvIHwEHuNHHTTA0sCgYBtSUFAwsXkaj0cm2y8YHZ8\nS46mv1AXFHYOnKHffjDXnLN7ao2FIsXLfdNWa/zxmLqqYtxUAcFwToSJi6szGnZT\nVxvZRFSBFveIOQvtLW1+EH4nYr3WGko4pvhQwrZqea7YH0skNrogBILPEToWc9bg\nUBOgeB31R7uh2X47kvvphQKBgQDWc60dYnniZVp5mwQZrQjbaC4YXaZ8ugrsPPhx\nNEoAPSN/KihrzZiJsjtsec3p1lNrzRNgHqCT3sgPIdPcFa7DRm5UDRIF54zL1gaq\nUwLyJ3TDxdZc928o4DLryc8J5mZRuSRq6t+MIU5wDnFHzhK+EBQ9Jc/I1rU22ONz\nDXaIoQKBgH14Apggo0o4Eo+OnEBRFbbDulaOfVLPTK9rktikbwO1vzDch8kdcwCU\nsvtRXHjDQL93Ih/8S9aDJZoSDulwr3VUsuDiDEb4jfYmP2sbNO4nIJt+SBMhVOXV\nt7E/uWK28X0GL/bIUzSMMgTfdjhXEtJW+s6hQU1fG+9U1qVTQ2R/\n-----END RSA PRIVATE KEY-----\n",
//! #         "appId": "181963751145079041",
//! #         "clientId": "181963751145144577@zitadel_rust_test"
//! #     }"#;
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     let config = IntrospectionConfigBuilder::new("https://zitadel-libraries-l8boqa.zitadel.cloud")
//!         .with_jwt_profile(Application::load_from_json(APPLICATION).unwrap())
//!         .build()
//!         .await
//!         .unwrap();
//!
//!     HttpServer::new(move || {
//!         App::new()
//!             .app_data(config.clone())
//!             // .service(the services here...)
//!     })
//!     .bind(("0.0.0.0", 8080))?
//!     .run()
//!     .await
//! }
//! ```
//!
//! #### Use the extractor
//!
//! Simply inject the [IntrospectedUser] into any route that should be protected by
//! OAuth introspection.
//!
//! ```no_run
//! # use zitadel::actix::introspection::IntrospectedUser;
//! #[actix_web::get("/authed")]
//! async fn authed(user: IntrospectedUser) -> impl actix_web::Responder {
//!     format!(
//!         "Hello Authorized {:?} with id {}",
//!         user.username, user.user_id
//!     )
//! }
//! ```
//!
//! ### Full working example with JWT Profile
//!
//! ```no_run
//! use actix_web::{App, HttpServer};
//! use zitadel::{
//!     credentials::Application,
//!     actix::introspection::{IntrospectedUser, IntrospectionConfigBuilder},
//! };
//!
//! const APPLICATION: &str = r#"
//!     {
//!         "type": "application",
//!         "keyId": "181963758610940161",
//!         "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAwT2YZJytkkZ1DDM3dcu1OA8YPzHu6XR8HotdMNRnV75GhOT4\nB7zDtdtoP8w/1NHHPEJ859e0kYhrrnKikOKLS6fS1KRsmqR5ZvTq8SlZ2mq3RcX2\nebZx5dQt36INij/WXdsBmjM/yfWvqqWBSb0L/186DaWwmmIxoXWe873vxRmlzblg\nGd8Nu07s9YTREbGPbtFVHEUM6xI4oIe8HJ0e1+JBkiGqk31Cogo0FoAxrOAg0Sf4\n5XiUMYIjzqh8673F9SC4IpVxG22mpFk3vDFuAITaStWYbiH2hPJNKWyX9HDCZb1D\nDqa3wZBDiLqWxh22hNZ6ZIe+3UoSGWsPBH+E1wIDAQABAoIBAD2v5QsRPRN57HmF\njAnNir8nimz6CrN53Pl/MbOZypenBSn9UfReXPeb3+6lzCarBPgGnYsBQAJJU16v\n95daym7PVy1Mg+Ll6F9mhe2Qbr+b23+pj2IRTNC6aB6Aw+PDNzJk7GEGRTG6fWZz\nSQ96Cu9tvcGHiBXwjLlnK+PRWU5IsCiLsjT4xBXsMLMw3YOdMK5z58sqr+SnNEyq\nRHoEvi9aC94WrargVB45Yx+81YNW8uQ5rMDmYaJC5a7ENz522SlAuf4T+fAGJ/HE\n/qbZGD4YwlLqAFDgewQ+5tEWEus3zgY2MIR7vN2zXU1Ptk+mQkXZl/Pxdp7q1xU+\nvr/kcykCgYEAy7MiIAzc1ctQDvkk3HiespzdQ/sC7+CGsBzkyubRc9Oq/YR7GfVK\nGTuDEDlWwx92VAvJGDWRa3T426YDyqiPj66uo836sgL15Uigg5afZun2bqGC78le\nBhSy9b+0YDHPa87GxtKt9UmMoB6WdmoPzOkLEEGS7eesmk2DDgY+QSUCgYEA8tr/\n3PawigL1cxuFpcO1lH6XUspGeAo5yB8FXvfW5g50e37LgooIvOFgUlYuchxwr6uh\nW+CUAWmm4farsgvMBMPYw+PbkCTi/xemiiDmMHUYd7sJkTl0JXApq3pZsNMg4Fw/\n29RynmcG8TGe2dkwrWp1aBYjvIHwEHuNHHTTA0sCgYBtSUFAwsXkaj0cm2y8YHZ8\nS46mv1AXFHYOnKHffjDXnLN7ao2FIsXLfdNWa/zxmLqqYtxUAcFwToSJi6szGnZT\nVxvZRFSBFveIOQvtLW1+EH4nYr3WGko4pvhQwrZqea7YH0skNrogBILPEToWc9bg\nUBOgeB31R7uh2X47kvvphQKBgQDWc60dYnniZVp5mwQZrQjbaC4YXaZ8ugrsPPhx\nNEoAPSN/KihrzZiJsjtsec3p1lNrzRNgHqCT3sgPIdPcFa7DRm5UDRIF54zL1gaq\nUwLyJ3TDxdZc928o4DLryc8J5mZRuSRq6t+MIU5wDnFHzhK+EBQ9Jc/I1rU22ONz\nDXaIoQKBgH14Apggo0o4Eo+OnEBRFbbDulaOfVLPTK9rktikbwO1vzDch8kdcwCU\nsvtRXHjDQL93Ih/8S9aDJZoSDulwr3VUsuDiDEb4jfYmP2sbNO4nIJt+SBMhVOXV\nt7E/uWK28X0GL/bIUzSMMgTfdjhXEtJW+s6hQU1fG+9U1qVTQ2R/\n-----END RSA PRIVATE KEY-----\n",
//!         "appId": "181963751145079041",
//!         "clientId": "181963751145144577@zitadel_rust_test"
//!     }"#;
//!
//! #[actix_web::get("/unauthed")]
//! async fn unauthed() -> impl actix_web::Responder {
//!     "Hello Unauthorized User"
//! }
//!
//! #[actix_web::get("/authed")]
//! async fn authed(user: IntrospectedUser) -> impl actix_web::Responder {
//!     format!(
//!         "Hello Authorized {:?} with id {}",
//!         user.username, user.user_id
//!     )
//! }
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     println!("Starting server.");
//!     let auth = IntrospectionConfigBuilder::new("https://zitadel-libraries-l8boqa.zitadel.cloud")
//!         .with_jwt_profile(Application::load_from_json(APPLICATION).unwrap())
//!         .build()
//!         .await
//!         .unwrap();
//!     HttpServer::new(move || {
//!         App::new()
//!             .app_data(auth.clone())
//!             .service(unauthed)
//!             .service(authed)
//!     })
//!     .bind(("0.0.0.0", 8080))?
//!     .run()
//!     .await
//! }
//! ```

pub use config::IntrospectionConfig;
pub use config_builder::{IntrospectionConfigBuilder, IntrospectionConfigBuilderError};
pub use extractor::{IntrospectedUser, IntrospectionExtractorError};

mod config;
mod config_builder;
mod extractor;
