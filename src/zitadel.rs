/// The default issuer for ZITADEL. Can be used with OIDC packages.
pub const ISSUER: &'static str = "https://issuer.zitadel.ch";

/// The token endpoint of the ZITADEL API. This can be used to fetch an access/refresh
/// token from the API.
pub const TOKEN_ENDPOINT: &'static str = "https://api.zitadel.ch/oauth/v2/token";

/// API endpoint URI of ZITADEL.
pub const API_ENDPOINT: &'static str = "https://api.zitadel.ch";

/// The project ID of the ZITADEL API. This ID is needed to request the audience
/// of the ZITADEL API in an access token. The project audience is required to access
/// the API via gRPC.
pub const ZITADEL_API_ID: &'static str = "69234237810729019";
