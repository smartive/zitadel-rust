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
#[derive(Clone, Debug, Serialize, Deserialize)]
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
pub(crate) struct TokenAuthResponse {
    pub(crate) access_token: String,
    // pub(crate) expires_in: i64,
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
    /// ```no_run
    /// use zitadel::credentials::ServiceAccount;
    /// let service_account = ServiceAccount::load_from_file("./my_json_key.json")?;
    /// println!("{:#?}", service_account);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
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
    /// println!("{:#?}", service_account);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
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
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// let service_account = ServiceAccount::load_from_file("./my_json_key.json")?;
    /// let access_token = service_account.authenticate().await?;
    /// println!("{}", access_token);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn authenticate(&self) -> Result<String, Box<dyn std::error::Error>> {
        let result = self.token_auth().await?;
        Ok(result.access_token)
    }

    pub(crate) async fn token_auth(&self) -> Result<TokenAuthResponse, Box<dyn std::error::Error>> {
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
            let result = response.text().await?;
            return Err(Box::new(errors::TokenError(result.to_string())));
        }

        let result = response.json::<TokenAuthResponse>().await?;

        Ok(result)
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
            scope: format!(
                "openid urn:zitadel:iam:org:project:id:{}:aud",
                crate::ZITADEL_API_ID
            ),
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
        "keyId": "147183960294746926",
        "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAy+yyAONERzgj0SNKEc2KRHQKhh+ekh/IKy1gZBy5l7L5JiAv\nzTY6+tY1PyGbkB2EkH3d0Gki5rIa4UrvjXyuOlUu+8uBG+rykVayhngnPZ+qO07W\nT8FC4YKiJABRm6hkLumrq/nckOno5RISdGt77+s02eGL33m0lr2SdIqj+ZWIJICK\n+keQne1llC3wqvsNJQiDnvRLar+w0wtJ95tOD2xz3met+0oQIpEt1UrNYgQQDirh\n45ih+pMvPR0zdx99nPWv920UXCM/KLGMveTZ4fDcF5tn1/2ghI6xNgrEZSKFNtdh\n8jijXAQvz7immVhGnAkxGX4AtXYyNgxK76aB6QIDAQABAoIBAHC5R3BMMJr5wnrB\n+hi7OJo8VvDrG5mErf6IF8dfRYxAp47WrfXO621q6YYbSsWwO24v1WR2KY/Cli9B\nYAgjCqA+JDmVtam8BxgmB4tjcbWTw+MC4l614wWLU5t4/aOAwthX3Mi01qLYWh/+\nDGuEWr81kkJ6dfozaYsGAaYgWSIF9Hqi5b3Lr1J34+SoDTLjOsqcdl57iIJeOw1X\nA0Gpp8JBfuDPmgS03GRMMXpb1YE4WMHpwyIRAc06dJyJtBfqZwYCZKR3Ln2WS5qO\nvibmDOXysnfHVnrnrnKkgsdInSApMu8YOuiLtjnsAq7GEW0Ts3ebX0tttkW+z3tM\ncSF177ECgYEA2AFo6IzfFtDnPVE1OVV1OuhVoTUIr5yYTRw1E9kRrH2QmG2BSlyh\n7MoUEOtrSJASrUdafhtsyY7sXF46EazRFm6Yq6LOc/ubLouNFnPcXmxrKtHw8jHP\nVQNP4Zt1D0gnafqEO1+9/WR5lidVDV7N/980E7W5n1PkpdXR474G6N0CgYEA8a6n\ncXckju3E2krfUepTwcmMKOyYgYoj39zIoBIF/D+WlTRSc7q0bRWB/11J9pkoOsHD\ny/5bSolfK4VGKH/FloRcP8Z/lVrZ/HCbR/hw+EEgaWUVyqanNU5485oxgDi+OjTO\nCbpkx2iRueH9g4MbDVAJdurSsTkOL8SYTOguJn0CgYBvhJLP9OK8Wc/4pTNwTUF/\nzzFeUA4S9CrhLJ3uiFQKlK0RNP/aD2b94/pmHdS+mrs5wKvkjW4lxWcb7P3X4Dv2\nc9TYT+58jLq6VgvaOqjcCudtLQRTVgnvnw0fse3GnP7URST9rlldOAFZ1yafB4Id\nBvRQ9LJHor1aLMD27kWM9QKBgQDZpjxNRsq5nQ9Gt17eWnULdAKxaED/h/Q+yooy\n/Yg+XtWxkOkgJ+gMxO3Jl63gUpWUNKOrtmloesYmX2OLXWYH4zNgi9aiHqtpV8+/\nxNGYAK67u7kgQ20Z6I2sdBRYMMG/kYZr5FyV6Go0SH0STqOyHX4ohdkwmP1Zr2ao\n+/9z0QKBgF8G+sY3yqiqV10I8bfa4mX3El876dMdeZJJgpQigD8n9LXN3N44GEga\n2b2GCUvvMXo4qTqKhgIXe7Cc3wJ7LjGXGuhDZYwfQ0giHR5aNBRKYYnaibKQTOQ0\nQqPe2TmJZ3N+7YXUw6ZlnUqYgDCohh6idQbE6/WputEdyT/gmBlY\n-----END RSA PRIVATE KEY-----\n",
        "userId": "147183953818739366"
    }
    "#;

    #[test]
    fn load_successfully_from_file() {
        let mut file = File::create("./temp").unwrap();
        file.write_all(SERVICE_ACCOUNT.as_bytes())
            .expect("Could not write temp.");

        let sa = ServiceAccount::load_from_file("./temp").unwrap();

        assert_eq!(sa.user_id, "147183953818739366");
        assert_eq!(sa.key_id, "147183960294746926");
    }

    #[test]
    fn load_successfully_from_json() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();

        assert_eq!(sa.user_id, "147183953818739366");
        assert_eq!(sa.key_id, "147183960294746926");
    }

    #[test]
    fn creates_correct_claims() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let claims = sa.get_claims();

        assert_eq!(claims.aud, crate::ISSUER);
        assert_eq!(claims.sub, sa.user_id);
        assert_eq!(claims.iss, sa.user_id);
    }

    #[test]
    fn creates_a_signed_jwt() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let claims = sa.signed_jwt().unwrap();

        assert_eq!(&claims[0..5], "eyJ0e");
    }

    #[tokio::test]
    async fn fetch_a_token_auth() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let token = sa.token_auth().await.unwrap();
        assert_ne!(token.access_token, "".to_string());
        // assert_ne!(token.expires_in, 0);
    }

    #[tokio::test]
    async fn authenticate_a_service_account() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let token = sa.authenticate().await.unwrap();
        assert_ne!(token, "".to_string());
    }

    // TODO: test with mockito when server returns fail.
}
