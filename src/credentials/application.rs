use custom_error::custom_error;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use crate::credentials::jwt::JwtClaims;

/// Application for [ZITADEL](https://zitadel.ch/). An application is an OIDC application type
/// that allows a backend (for example an API for some single page application) to
/// check if sent credentials from a client are valid or not.
///
/// When using ZITADEL to authenticate a user against some backend, the application
/// provides the means to access the introspection endpoint of ZITADEL.
/// It uses OIDC Introspection, defined in [RFC7662](https://tools.ietf.org/html/rfc7662).
///
/// To create an application json, head over to your ZITADEL console
/// and execute the following steps:
/// - Create a project
/// - Create an API application
/// - Create a "key" inside the application to create and download the JWT profile
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    client_id: String,
    app_id: String,
    key_id: String,
    key: String,
}

custom_error! {
    /// Error type for application credential related errors.
    pub ApplicationError
        Io{source: std::io::Error} = "unable to read from file: {source}",
        Json{source: serde_json::Error} = "could not parse json: {source}",
        Key{source: jsonwebtoken::errors::Error} = "could not parse RSA key: {source}",
}

impl Application {
    /// Load an [`Application`] from a JSON file at a specific filepath.
    ///
    /// ### Errors
    ///
    /// This function may return an error when [`read_to_string`] returns an error.
    /// Further, an error may occur during the deserialization of
    /// [`load_from_json`][Application::load_from_json].
    ///
    /// ### Example
    ///
    /// ```no_run
    /// use zitadel::credentials::Application;
    /// let application = Application::load_from_file("./my_json_key.json")?;
    /// println!("{:#?}", application);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load_from_file(file_path: &str) -> Result<Self, ApplicationError> {
        let data = read_to_string(file_path).map_err(|e| ApplicationError::Io { source: e })?;
        Application::load_from_json(data.as_str())
    }

    /// Load an [`Application`] from a JSON string.
    ///
    /// ### Errors
    ///
    /// This method may fail if the [deserialization][serde_json::from_str] does fail.
    /// Such an error can occur if the JSON is not formatted properly.
    ///
    /// ### Example
    ///
    /// ```
    /// use zitadel::credentials::Application;
    /// let application = Application::load_from_json(r#"{"keyId": "1337", "clientId": "testing", "userId": "42", "key": "foobar"}"#)?;
    /// println!("{:#?}", application);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load_from_json(json: &str) -> Result<Self, ApplicationError> {
        let sa: Application =
            serde_json::from_str(json).map_err(|e| ApplicationError::Json { source: e })?;
        Ok(sa)
    }

    pub fn create_signed_jwt(&self, audience: &str) -> Result<String, ApplicationError> {
        let key = EncodingKey::from_rsa_pem(self.key.as_bytes())
            .map_err(|e| ApplicationError::Key { source: e })?;
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.key_id.to_string());
        let claims = JwtClaims::new(&self.client_id, audience);
        let jwt = encode(&header, &claims, &key)?;

        Ok(jwt)
    }
}
