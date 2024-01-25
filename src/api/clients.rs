//! Module with convenience functions to create clients for ZITADEL
//! API access. When the credentials feature is enabled, additional
//! modules provide access to functions that create clients with
//! specific interceptors for authentication.

use std::error::Error;

use custom_error::custom_error;
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};

use crate::api::interceptors::{AccessTokenInterceptor, ServiceAccountInterceptor};
use crate::api::zitadel::oidc::v2beta::oidc_service_client::OidcServiceClient;
use crate::api::zitadel::org::v2beta::organization_service_client::OrganizationServiceClient;
use crate::api::zitadel::session::v2beta::session_service_client::SessionServiceClient;
use crate::api::zitadel::settings::v2beta::settings_service_client::SettingsServiceClient;
use crate::api::zitadel::system::v1::system_service_client::SystemServiceClient;
use crate::api::zitadel::user::v2beta::user_service_client::UserServiceClient;
use crate::credentials::{AuthenticationOptions, ServiceAccount};

use super::zitadel::{
    admin::v1::admin_service_client::AdminServiceClient,
    auth::v1::auth_service_client::AuthServiceClient,
    management::v1::management_service_client::ManagementServiceClient,
};

custom_error! {
    /// Errors that may occur when creating a client.
    pub ClientError
        InvalidUrl = "the provided url is invalid",
        ConnectionError = "could not connect to provided endpoint",
}

enum AuthType {
    None,
    AccessToken(String),
    ServiceAccount(ServiceAccount, Option<AuthenticationOptions>),
}

/// Helper [Interceptor] that allows chaining of multiple interceptors.
/// This is used to help return the same type in all builder methods like
/// [ClientBuilder::build_management_client]. Otherwise, each interceptor
/// would create its own return type. With this interceptor, the return type
/// stays the same and is not dependent on the authentication type used.
/// The builder can always return `Client<InterceptedService<Channel, ChainedInterceptor>>`.
pub struct ChainedInterceptor {
    interceptors: Vec<Box<dyn Interceptor>>,
}

impl ChainedInterceptor {
    pub(crate) fn new() -> Self {
        Self {
            interceptors: Vec::new(),
        }
    }

    pub(crate) fn add_interceptor(mut self, interceptor: Box<dyn Interceptor>) -> Self {
        self.interceptors.push(interceptor);
        self
    }
}

impl Interceptor for ChainedInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let mut request = request;
        for interceptor in &mut self.interceptors {
            request = interceptor.call(request)?;
        }
        Ok(request)
    }
}

/// A builder to create configured gRPC clients for ZITADEL API access.
/// The builder accepts the api endpoint and (depending on activated features)
/// an authentication method.
pub struct ClientBuilder {
    api_endpoint: String,
    auth_type: AuthType,
}

impl ClientBuilder {
    /// Create a new client builder with the the provided endpoint.
    pub fn new(api_endpoint: &str) -> Self {
        Self {
            api_endpoint: api_endpoint.to_string(),
            auth_type: AuthType::None,
        }
    }

    /// Configure the client builder to use a provided access token.
    /// This can be a pre-fetched token from ZITADEL or some other form
    /// of a valid access token like a personal access token (PAT).
    ///
    /// Clients with this authentication method will have the [`AccessTokenInterceptor`]
    /// attached.
    #[cfg(feature = "interceptors")]
    pub fn with_access_token(mut self, access_token: &str) -> Self {
        self.auth_type = AuthType::AccessToken(access_token.to_string());
        self
    }

    /// Configure the client builder to use a [`ServiceAccount`][crate::credentials::ServiceAccount].
    /// The service account will be used to fetch a valid access token from ZITADEL.
    ///
    /// Clients with this authentication method will have the
    /// [`ServiceAccountInterceptor`][crate::api::interceptors::ServiceAccountInterceptor] attached
    /// that fetches an access token from ZITADEL and renewes it when it expires.
    #[cfg(feature = "interceptors")]
    pub fn with_service_account(
        mut self,
        service_account: &ServiceAccount,
        auth_options: Option<AuthenticationOptions>,
    ) -> Self {
        self.auth_type = AuthType::ServiceAccount(service_account.clone(), auth_options);
        self
    }

    /// Create a new [`AdminServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_admin_client(
        &self,
    ) -> Result<AdminServiceClient<InterceptedService<Channel, ChainedInterceptor>>, Box<dyn Error>>
    {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(AdminServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    /// Create a new [`AuthServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_auth_client(
        &self,
    ) -> Result<AuthServiceClient<InterceptedService<Channel, ChainedInterceptor>>, Box<dyn Error>>
    {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(AuthServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    /// Create a new [`ManagementServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_management_client(
        &self,
    ) -> Result<
        ManagementServiceClient<InterceptedService<Channel, ChainedInterceptor>>,
        Box<dyn Error>,
    > {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(ManagementServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    /// Create a new [`OidcServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_oidc_client(
        &self,
    ) -> Result<OidcServiceClient<InterceptedService<Channel, ChainedInterceptor>>, Box<dyn Error>>
    {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(OidcServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    /// Create a new [`OrganizationServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_organization_client(
        &self,
    ) -> Result<
        OrganizationServiceClient<InterceptedService<Channel, ChainedInterceptor>>,
        Box<dyn Error>,
    > {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(OrganizationServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    /// Create a new [`SessionServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_session_client(
        &self,
    ) -> Result<SessionServiceClient<InterceptedService<Channel, ChainedInterceptor>>, Box<dyn Error>>
    {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(SessionServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    /// Create a new [`SettingsServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_settings_client(
        &self,
    ) -> Result<
        SettingsServiceClient<InterceptedService<Channel, ChainedInterceptor>>,
        Box<dyn Error>,
    > {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(SettingsServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    /// Create a new [`SystemServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_system_client(
        &self,
    ) -> Result<SystemServiceClient<InterceptedService<Channel, ChainedInterceptor>>, Box<dyn Error>>
    {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(SystemServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    /// Create a new [`UserServiceClient`].
    ///
    /// Depending on the configured authentication method, the client has
    /// specialised interceptors attached.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    pub async fn build_user_client(
        &self,
    ) -> Result<UserServiceClient<InterceptedService<Channel, ChainedInterceptor>>, Box<dyn Error>>
    {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(UserServiceClient::with_interceptor(
            channel,
            self.get_chained_interceptor(),
        ))
    }

    fn get_chained_interceptor(&self) -> ChainedInterceptor {
        let mut interceptor = ChainedInterceptor::new();
        match &self.auth_type {
            AuthType::AccessToken(token) => {
                interceptor =
                    interceptor.add_interceptor(Box::new(AccessTokenInterceptor::new(token)));
            }
            AuthType::ServiceAccount(service_account, auth_options) => {
                interceptor =
                    interceptor.add_interceptor(Box::new(ServiceAccountInterceptor::new(
                        &self.api_endpoint,
                        service_account,
                        auth_options.clone(),
                    )));
            }
            _ => {}
        }

        interceptor
    }
}

async fn get_channel(api_endpoint: &str) -> Result<Channel, ClientError> {
    Endpoint::from_shared(api_endpoint.to_string())
        .map_err(|_| ClientError::InvalidUrl)?
        .connect()
        .await
        .map_err(|_| ClientError::ConnectionError)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    const SERVICE_ACCOUNT: &str = r#"
    {
        "type": "serviceaccount",
        "keyId": "181828078849229057",
        "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEpQIBAAKCAQEA9VIWALQqzx1ypi42t7MG4KSOMldD10brsEUjTcjqxhl6TJrP\nsjaNKWArnV/XH+6ZKRd55mUEFFx9VflqdwQtMVPjZKXpV4cFDiPwf1Z1h1DS6im4\nSo7eKR7OGb7TLBhwt7i2UPF4WnxBhTp/M6pG5kCJ1t8glIo5yRbrILXObRmvNWMz\nVIFAyw68NDZGYNhnR8AT43zjeJTFXG/suuEoXO/mMmMjsYY8kS0BbiQeq5t5hIrr\na/odswkDPn5Zd4P91iJHDnYlgfJuo3oRmgpOj/dDsl+vTol+vveeMO4TXPwZcl36\ngUNPok7nd6BA3gqmOS+fMImzmZB42trghARXXwIDAQABAoIBAQCbMOGQcml+ep+T\ntzqQPWYFaLQ37nKRVmE1Mpeh1o+G4Ik4utrXX6EvYpJUzVN29ObZUuufr5nEE7qK\nT+1k+zRntyzr9/VElLrC9kNnGtfg0WWMEvZt3DF4i+9P5CMNCy0LXIOhcxBzFZYR\nZS8hDQArGvrX/nFK5qKlrqTyHXFIHDFa6z59ErhXEnsTgRvx/Mo+6UkdBkHsKnlJ\nAbXqXFbfz6nDsF1DgRra5ODn1k8nZqnC/YcssE7/dlbuByz10ECkOSzqYcfufnsb\n9N1Ld4Xlj3yzsqPFzEJyHHm9eEHQXsPavaXiM64/+zpsksLscEIE/0KtIy5tngpZ\nSCqZAcj5AoGBAPb1bQFWUBmmUuSTtSymsxgXghJiJ3r+jJgdGbkv2IsRTs4En5Sz\n0SbPE1YWmMDDgTacJlB4/XiaojQ/j1EEY17inxYomE72UL6/ET7ycsEw3e9ALuD5\np0y2Sdzes2biH30bw5jD8kJ+hV18T745KtzrwSH4I0lAjnkmiH+0S67VAoGBAP5N\nTtAp/Qdxh9GjNSw1J7KRLtJrrr0pPrJ9av4GoFoWlz+Qw2X3dl8rjG3Bqz9LPV7A\ngiHMel8WTmdIM/S3F4Q3ufEfE+VzG+gncWd9SJfX5/LVhatPzTGLNsY7AYGEpSwT\n5/0anS1mHrLwsVcPrZnigekr5A5mfZl6nxtOnE9jAoGBALACqacbUkmFrmy1DZp+\nUQSptI3PoR3bEG9VxkCjZi1vr3/L8cS1CCslyT1BK6uva4d1cSVHpjfv1g1xA38V\nppE46XOMiUk16sSYPv1jJQCmCHd9givcIy3cefZOTwTTwueTAyv888wKipjfgaIs\n8my0JllEljmeJi0Ylo6V/J7lAoGBAIFqRlmZhLNtC3mcXUsKIhG14OYk9uA9RTMA\nsJpmNOSj6oTm3wndTdhRCT4x+TxUxf6aaZ9ZuEz7xRq6m/ZF1ynqUi5ramyyj9kt\neYD5OSBNODVUhJoSGpLEDjQDg1iucIBmAQHFsYeRGL5nz1hHGkneA87uDzlk3zZk\nOORktReRAoGAGUfU2UfaniAlqrZsSma3ZTlvJWs1x8cbVDyKTYMX5ShHhp+cA86H\nYjSSol6GI2wQPP+qIvZ1E8XyzD2miMJabl92/WY0tHejNNBEHwD8uBZKrtMoFWM7\nWJNl+Xneu/sT8s4pP2ng6QE7jpHXi2TUNmSlgQry9JN2AmA9TuSTW2Y=\n-----END RSA PRIVATE KEY-----\n",
        "userId": "181828061098934529"
    }"#;

    #[test]
    fn client_builder_without_auth_passes_requests() {
        let mut interceptor = ClientBuilder::new(ZITADEL_URL).get_chained_interceptor();
        let request = Request::new(());

        assert!(request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().is_empty());
    }

    #[test]
    fn client_builder_with_access_token_attaches_it() {
        let mut interceptor = ClientBuilder::new(ZITADEL_URL)
            .with_access_token("token")
            .get_chained_interceptor();
        let request = Request::new(());

        assert!(request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().contains_key("authorization"));
        assert_eq!(
            request.metadata().get("authorization").unwrap(),
            "Bearer token"
        );
    }

    #[test]
    fn client_builder_with_service_account_attaches_token() {
        let sa = ServiceAccount::load_from_json(SERVICE_ACCOUNT).unwrap();
        let mut interceptor = ClientBuilder::new(ZITADEL_URL)
            .with_service_account(&sa, None)
            .get_chained_interceptor();
        let request = Request::new(());

        assert!(request.metadata().is_empty());

        let request = interceptor.call(request).unwrap();

        assert!(request.metadata().contains_key("authorization"));
        assert!(!request
            .metadata()
            .get("authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .is_empty());
    }
}
