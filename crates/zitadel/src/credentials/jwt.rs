use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct JwtClaims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    aud: String,
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
}
