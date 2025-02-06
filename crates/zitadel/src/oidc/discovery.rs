use custom_error::custom_error;
use openidconnect::{
    core::{
        CoreAuthDisplay, CoreClaimName, CoreClaimType, CoreClientAuthMethod, CoreGrantType,
        CoreJsonWebKey, CoreJweContentEncryptionAlgorithm,
        CoreJweKeyManagementAlgorithm, CoreResponseMode, CoreResponseType,
        CoreSubjectIdentifierType,
    },
    url, AdditionalProviderMetadata, IntrospectionUrl, IssuerUrl, ProviderMetadata, RevocationUrl,
};
use serde::{Deserialize, Serialize};

custom_error! {
    /// Error type for discovery related errors.
    pub DiscoveryError
        IssuerUrl{source: url::ParseError} = "could not parse issuer url: {source}",
        DiscoveryDocument = "could not discover OIDC document",
        DiscoveryClientError{source: reqwest::Error} = "could not fetch discovery document: {source}",
}

/// Fetch the well-known [OIDC Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html)
/// document of a given `authority`. "Authority" is a synonym for "Issuer" and vice versa.
/// The discovery document contains information about various OIDC endpoints of the ZITADEL
/// instance. Note that the authority (issuer) must not contain the well-known url part
/// (`/.well-known/openid-configuration`).
///
/// The returned [metadata](ZitadelProviderMetadata) contains the parsed information of the
/// well-known OIDC configuration.
///
/// ### Errors
///
/// This method may fail if:
/// - The authority url cannot be parsed correctly
/// - The discovery call throws any kind of error
///
/// ### Example
///
/// #### Fetch the discovery document of the "ZITADEL Libraries" - Test Instance
///
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
/// use zitadel::oidc::discovery::discover;
/// let authority = "https://zitadel-libraries-l8boqa.zitadel.cloud";
/// let metadata = discover(authority).await?;
/// println!("{:?}", metadata.token_endpoint());
/// # Ok(())
/// # }
/// ```
pub async fn discover(authority: &str) -> Result<ZitadelProviderMetadata, DiscoveryError> {
    let issuer = IssuerUrl::new(authority.to_string())
        .map_err(|source| DiscoveryError::IssuerUrl { source })?;
    let async_http_client = reqwest::ClientBuilder::new().redirect(reqwest::redirect::Policy::none()).build()?;
    ZitadelProviderMetadata::discover_async(issuer, &async_http_client)
        .await
        .map_err(|_| DiscoveryError::DiscoveryDocument)
}

/// Definition of additional metadata that is not present in the
/// standard metadata of the openidconnect crate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZitadelAdditionalMetadata {
    pub introspection_endpoint: Option<IntrospectionUrl>,
    pub revocation_endpoint: Option<RevocationUrl>,
}

impl AdditionalProviderMetadata for ZitadelAdditionalMetadata {}

/// Type to the ZITADEL provider metadata. Essentially combines
/// the [ZitadelAdditionalMetadata] with the openid provider
/// metadata information.
pub type ZitadelProviderMetadata = ProviderMetadata<
    ZitadelAdditionalMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;

/*
pub type ZitadelProviderMetadata = ProviderMetadata<
    ZitadelAdditionalMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;
 */

#[cfg(test)]
mod tests {
    #![allow(clippy::all)]

    use super::*;

    const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";

    #[tokio::test]
    async fn discovery_fails_with_invalid_url() {
        let result = discover("foobar").await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DiscoveryError::IssuerUrl { .. }
        ));
    }

    #[tokio::test]
    async fn discovery_fails_with_invalid_discovery() {
        let result = discover("https://smartive.ch").await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DiscoveryError::DiscoveryDocument
        ));
    }

    #[tokio::test]
    async fn discovery_succeeds() {
        let result = discover(ZITADEL_URL).await.unwrap();

        assert_eq!(
            result.token_endpoint().unwrap().to_string(),
            "https://zitadel-libraries-l8boqa.zitadel.cloud/oauth/v2/token".to_string()
        );
    }
}
