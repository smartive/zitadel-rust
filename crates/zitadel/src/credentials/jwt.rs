use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JwtClaims {
    pub(super) iss: String,
    pub(super) sub: String,
    pub(super) iat: i64,
    pub(super) exp: i64,
    pub(super) aud: String,
}

impl JwtClaims {
    pub(super) fn new(sub_and_iss: &str, audience: &str) -> Self {
        let iat = time::OffsetDateTime::now_utc();
        let exp = iat + time::Duration::hours(1);
        Self {
            iss: sub_and_iss.to_string(),
            sub: sub_and_iss.to_string(),
            iat: iat.unix_timestamp(),
            exp: exp.unix_timestamp(),
            aud: audience.to_string(),
        }
    }

    pub(super) fn new_with_expiry(
        sub_and_iss: &str,
        audience: &str,
        exp: time::OffsetDateTime,
    ) -> Self {
        let iat = time::OffsetDateTime::now_utc();
        Self {
            iss: sub_and_iss.to_string(),
            sub: sub_and_iss.to_string(),
            iat: iat.unix_timestamp(),
            exp: exp.unix_timestamp(),
            aud: audience.to_string(),
        }
    }
}
