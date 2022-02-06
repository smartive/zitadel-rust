use std::fs::read_to_string;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// A service account for [ZITADEL](https://zitadel.ch/). The service
/// account can be loaded from a valid JSON string or from a file containing the JSON string.
/// The account can be used to communicate with the ZITADEL API and may serve as access token
/// provider for a gRPC service client.
///
/// To create a service account json, head over to the [ZITADEL console](https://console.zitadel.ch/)
/// and execute the following steps:
/// - create a `Service User` in your organization
/// - Give the service user the relevant authorization (e.g. ORG_OWNER or access to a specific project)
/// - Create a "key" in the account detail page of the service user and download it
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    user_id: String,
    key_id: String,
    key: String,
}

#[derive(Debug, Serialize)]
struct JwtClaims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    aud: String,
}

#[derive(Debug, Serialize)]
struct JwtProfileAuthBody {
    grant_type: String,
    assertion: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
struct TokenAuthResponse {
    access_token: String,
}

impl ServiceAccount {
    /// Load a [`ServiceAccount`] from a JSON file at a specific filepath.
    ///
    /// # Errors
    ///
    /// This function may return an error when [`read_to_string`] returns an error.
    /// Further, an error may occur during the deserialization of
    /// [`load_from_json`][ServiceAccount::load_from_json].
    ///
    /// # Example
    ///
    /// ```
    /// use zitadel::credentials::ServiceAccount;
    /// let service_account = ServiceAccount::load_from_file("./my_json_key.json")?;
    /// println!("{}", service_account);
    /// ```
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = read_to_string(file_path)?;
        ServiceAccount::load_from_json(data.as_str())
    }

    /// Load a [`ServiceAccount`] from a JSON string.
    ///
    /// # Errors
    ///
    /// This method may fail if the [deserialization][serde_json::from_str] does fail.
    /// Such an error can occur if the JSON is not formatted properly.
    ///
    /// # Example
    ///
    /// ```
    /// use zitadel::credentials::ServiceAccount;
    /// let service_account = ServiceAccount::load_from_json(r#"{"keyId": "1337", "userId": "42", "key": "foobar"}"#)?;
    /// println!("{}", service_account);
    /// ```
    pub fn load_from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sa: ServiceAccount = serde_json::from_str(json)?;
        Ok(sa)
    }

    /// Authenticates the [`ServiceAccount`] against the [issuer][crate::ISSUER] to
    /// fetch an access token.
    ///
    /// The function returns an access token that can be sent
    /// to authenticate any request as the given service account. The access token
    /// is valid for ten minutes.
    ///
    /// # Errors
    ///
    /// This method may fail when:
    /// - The key in the service account is not a valid PEM encoded RSA private key.
    /// - When the [token endpoint][crate::TOKEN_ENDPOINT] is not reachable.
    /// - When any error in the request happens.
    /// - When the response status code is **not** 200 OK.
    /// - When the response cannot be parsed as valid JSON.
    ///
    /// # Example
    ///
    /// ```
    /// use zitadel::credentials::ServiceAccount;
    /// let service_account = ServiceAccount::load_from_file("./my_json_key.json")?;
    /// let access_token = service_account.authenticate().await?;
    /// println!("{}", access_token);
    /// ```
    pub async fn authenticate(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let jwt = self.signed_jwt()?;

        let response = client
            .post(crate::TOKEN_ENDPOINT)
            .body(serde_urlencoded::to_string(
                JwtProfileAuthBody::new_with_jwt(&jwt),
            )?)
            .header("content-type", "application/x-www-form-urlencoded")
            .header("accept", "application/json")
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            let result = &response.text().await?;
            return Err(Box::new(errors::TokenError(result.to_string())));
        }

        let result = response.json::<TokenAuthResponse>().await?;

        Ok(result.access_token)
    }

    fn get_claims(&self) -> JwtClaims {
        let now = chrono::offset::Utc::now().timestamp();
        JwtClaims {
            iss: self.user_id.to_string(),
            sub: self.user_id.to_string(),
            iat: now - 1,
            exp: now + 60,
            aud: crate::ISSUER.to_string(),
        }
    }

    fn signed_jwt(&self) -> Result<String, Box<dyn std::error::Error>> {
        let key = EncodingKey::from_rsa_pem(self.key.as_bytes())?;
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.key_id.to_string());
        let claims = self.get_claims();
        let jwt = encode(&header, &claims, &key)?;

        Ok(jwt)
    }
}

impl JwtProfileAuthBody {
    fn new_with_jwt(jwt: &str) -> Self {
        JwtProfileAuthBody {
            grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer".to_string(),
            assertion: jwt.to_string(),
            scope: format!("openid urn:zitadel:iam:org:project:id:{}:aud", crate::ZITADEL_API_ID),
        }
    }
}

mod errors {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct TokenError(pub String);

    impl std::fmt::Display for TokenError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Token-Response error: {}", self.0)
        }
    }

    impl Error for TokenError {}
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use super::*;

    const SERVICE_ACCOUNT: &'static str = r#"
    {
      "type": "serviceaccount",
      "keyId": "1337",
      "key": "my_rsa_key",
      "userId": "42"
    }
    "#;

    #[test]
    fn load_successfully_from_file() {
        let mut file = File::create("./temp").unwrap();
        file.write_all(SERVICE_ACCOUNT.as_bytes())
            .expect("Could not write temp.");

        let sa = ServiceAccount::load_from_file("./temp").unwrap();

        assert_eq!(sa.user_id, "42");
        assert_eq!(sa.key_id, "1337");
        assert_eq!(sa.key, "my_rsa_key");
    }

    #[test]
    fn load_successfully_from_json() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();

        assert_eq!(sa.user_id, "42");
        assert_eq!(sa.key_id, "1337");
        assert_eq!(sa.key, "my_rsa_key");
    }
}
