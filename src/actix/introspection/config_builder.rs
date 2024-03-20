use custom_error::custom_error;

use crate::actix::introspection::config::IntrospectionConfig;
use crate::credentials::Application;
use crate::oidc::discovery::{discover, DiscoveryError};
use crate::oidc::introspection::AuthorityAuthentication;

custom_error! {
    /// Error type for introspection config builder related errors.
    pub IntrospectionConfigBuilderError
        NoAuthSchema = "no authentication for authority defined",
        Discovery{source: DiscoveryError} = "could not fetch discovery document: {source}",
        NoIntrospectionUrl = "discovery document did not contain an introspection url",
}

/// Builder for [IntrospectionConfig]s.
/// The authority is mandatory when creating the builder.
/// Then, either one of the authentication mechanisms must be chosen or the
/// builder will throw an error during [build](IntrospectionConfigBuilder::build).
pub struct IntrospectionConfigBuilder {
    authority: String,
    authentication: Option<AuthorityAuthentication>,
}

impl IntrospectionConfigBuilder {
    /// Create a new config builder with the given authority.
    /// Returns the chainable config builder.
    pub fn new(authority: &str) -> Self {
        Self {
            authority: authority.to_string(),
            authentication: None,
        }
    }

    /// Set the authentication method to [AuthorityAuthentication::Basic].
    pub fn with_basic_auth(
        &mut self,
        client_id: &str,
        client_secret: &str,
    ) -> &mut IntrospectionConfigBuilder {
        self.authentication = Some(AuthorityAuthentication::Basic {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        });

        self
    }

    /// Set the authentication method to [AuthorityAuthentication::JWTProfile]
    /// by using the given [Application].
    pub fn with_jwt_profile(
        &mut self,
        application: Application,
    ) -> &mut IntrospectionConfigBuilder {
        self.authentication = Some(AuthorityAuthentication::JWTProfile { application });

        self
    }

    /// Build the [IntrospectionConfig]. This asynchronous method fetches the discovery document
    /// of the ZITADEL instance and gets the introspection endpoint.
    ///
    /// ### Errors
    ///
    /// The construction may fail if:
    /// - No authentication ([IntrospectionConfigBuilder::with_basic_auth] or
    ///   [IntrospectionConfigBuilder::with_jwt_profile]) was set for the config.
    /// - The [discover] call throws an error.
    /// - No introspection endpoint is defined in the discovery document.
    ///
    /// ### Examples
    ///
    /// #### Build config with JWT Profile (recommended)
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// # use zitadel::credentials::Application;
    /// # use zitadel::rocket::introspection::IntrospectionConfigBuilder;
    /// # const APPLICATION: &str = r#"
    /// #     {
    /// #         "type": "application",
    /// #         "keyId": "181963758610940161",
    /// #         "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAwT2YZJytkkZ1DDM3dcu1OA8YPzHu6XR8HotdMNRnV75GhOT4\nB7zDtdtoP8w/1NHHPEJ859e0kYhrrnKikOKLS6fS1KRsmqR5ZvTq8SlZ2mq3RcX2\nebZx5dQt36INij/WXdsBmjM/yfWvqqWBSb0L/186DaWwmmIxoXWe873vxRmlzblg\nGd8Nu07s9YTREbGPbtFVHEUM6xI4oIe8HJ0e1+JBkiGqk31Cogo0FoAxrOAg0Sf4\n5XiUMYIjzqh8673F9SC4IpVxG22mpFk3vDFuAITaStWYbiH2hPJNKWyX9HDCZb1D\nDqa3wZBDiLqWxh22hNZ6ZIe+3UoSGWsPBH+E1wIDAQABAoIBAD2v5QsRPRN57HmF\njAnNir8nimz6CrN53Pl/MbOZypenBSn9UfReXPeb3+6lzCarBPgGnYsBQAJJU16v\n95daym7PVy1Mg+Ll6F9mhe2Qbr+b23+pj2IRTNC6aB6Aw+PDNzJk7GEGRTG6fWZz\nSQ96Cu9tvcGHiBXwjLlnK+PRWU5IsCiLsjT4xBXsMLMw3YOdMK5z58sqr+SnNEyq\nRHoEvi9aC94WrargVB45Yx+81YNW8uQ5rMDmYaJC5a7ENz522SlAuf4T+fAGJ/HE\n/qbZGD4YwlLqAFDgewQ+5tEWEus3zgY2MIR7vN2zXU1Ptk+mQkXZl/Pxdp7q1xU+\nvr/kcykCgYEAy7MiIAzc1ctQDvkk3HiespzdQ/sC7+CGsBzkyubRc9Oq/YR7GfVK\nGTuDEDlWwx92VAvJGDWRa3T426YDyqiPj66uo836sgL15Uigg5afZun2bqGC78le\nBhSy9b+0YDHPa87GxtKt9UmMoB6WdmoPzOkLEEGS7eesmk2DDgY+QSUCgYEA8tr/\n3PawigL1cxuFpcO1lH6XUspGeAo5yB8FXvfW5g50e37LgooIvOFgUlYuchxwr6uh\nW+CUAWmm4farsgvMBMPYw+PbkCTi/xemiiDmMHUYd7sJkTl0JXApq3pZsNMg4Fw/\n29RynmcG8TGe2dkwrWp1aBYjvIHwEHuNHHTTA0sCgYBtSUFAwsXkaj0cm2y8YHZ8\nS46mv1AXFHYOnKHffjDXnLN7ao2FIsXLfdNWa/zxmLqqYtxUAcFwToSJi6szGnZT\nVxvZRFSBFveIOQvtLW1+EH4nYr3WGko4pvhQwrZqea7YH0skNrogBILPEToWc9bg\nUBOgeB31R7uh2X47kvvphQKBgQDWc60dYnniZVp5mwQZrQjbaC4YXaZ8ugrsPPhx\nNEoAPSN/KihrzZiJsjtsec3p1lNrzRNgHqCT3sgPIdPcFa7DRm5UDRIF54zL1gaq\nUwLyJ3TDxdZc928o4DLryc8J5mZRuSRq6t+MIU5wDnFHzhK+EBQ9Jc/I1rU22ONz\nDXaIoQKBgH14Apggo0o4Eo+OnEBRFbbDulaOfVLPTK9rktikbwO1vzDch8kdcwCU\nsvtRXHjDQL93Ih/8S9aDJZoSDulwr3VUsuDiDEb4jfYmP2sbNO4nIJt+SBMhVOXV\nt7E/uWK28X0GL/bIUzSMMgTfdjhXEtJW+s6hQU1fG+9U1qVTQ2R/\n-----END RSA PRIVATE KEY-----\n",
    /// #         "appId": "181963751145079041",
    /// #         "clientId": "181963751145144577@zitadel_rust_test"
    /// #     }"#;
    /// let config = IntrospectionConfigBuilder::new("https://zitadel-libraries-l8boqa.zitadel.cloud")
    ///                 .with_jwt_profile(Application::load_from_json(APPLICATION).unwrap())
    ///                 .build()
    ///                 .await?;
    ///
    /// println!("{:?}", config);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// #### Build config with Basic Auth
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// # use zitadel::rocket::introspection::IntrospectionConfigBuilder;
    /// let config = IntrospectionConfigBuilder::new("https://zitadel-libraries-l8boqa.zitadel.cloud")
    ///                 .with_basic_auth(
    ///                     "194339055499018497@zitadel_rust_test",
    ///                     "Ip56oGzxKL1rJ8JaleUVKL7qUlpZ1tqHQYRSd6JE1mTlTJ3pDkDzoObHdZsOg88B",
    ///                 )
    ///                 .build()
    ///                 .await?;
    ///
    /// println!("{:?}", config);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn build(&mut self) -> Result<IntrospectionConfig, IntrospectionConfigBuilderError> {
        if self.authentication.is_none() {
            return Err(IntrospectionConfigBuilderError::NoAuthSchema);
        }

        let metadata = discover(&self.authority)
            .await
            .map_err(|source| IntrospectionConfigBuilderError::Discovery { source })?;

        let introspection_uri = metadata
            .additional_metadata()
            .introspection_endpoint
            .clone();

        if introspection_uri.is_none() {
            return Err(IntrospectionConfigBuilderError::NoIntrospectionUrl);
        }

        Ok(IntrospectionConfig {
            authority: self.authority.clone(),
            introspection_uri: introspection_uri.unwrap(),
            authentication: self.authentication.as_ref().unwrap().clone(),
        })
    }
}
