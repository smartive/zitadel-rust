use std::fs::read_to_string;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const ISSUER: &'static str = "https://issuer.zitadel.ch";
const TOKEN_ENDPOINT: &'static str = "https://api.zitadel.ch/oauth/v2/token";

#[derive(Debug, Deserialize)]
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
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = read_to_string(file_path)?;
        ServiceAccount::load_from_json(data.as_str())
    }

    pub fn load_from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sa: ServiceAccount = serde_json::from_str(json)?;
        Ok(sa)
    }

    pub async fn authenticate(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let jwt = self.signed_jwt()?;

        let response = client
            .post(TOKEN_ENDPOINT)
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
            aud: ISSUER.to_string(),
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
            scope: "openid urn:zitadel:iam:org:project:id:69234237810729019:aud".to_string(),
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
