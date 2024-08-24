//! Module with convenience functions to create clients for ZITADEL
//! API access. When the credentials feature is enabled, additional
//! modules provide access to functions that create clients with
//! specific interceptors for authentication.

use std::error::Error;

use custom_error::custom_error;
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};

#[cfg(feature = "interceptors")]
use crate::api::interceptors::{AccessTokenInterceptor, ServiceAccountInterceptor};

#[cfg(feature = "api-oidc-v2")]
use crate::api::zitadel::oidc::v2::oidc_service_client::OidcServiceClient;
#[cfg(feature = "api-org-v2")]
use crate::api::zitadel::org::v2::organization_service_client::OrganizationServiceClient;
#[cfg(feature = "api-session-v2")]
use crate::api::zitadel::session::v2::session_service_client::SessionServiceClient;
#[cfg(feature = "api-settings-v2")]
use crate::api::zitadel::settings::v2::settings_service_client::SettingsServiceClient;
#[cfg(feature = "api-user-v2")]
use crate::api::zitadel::user::v2::user_service_client::UserServiceClient;

#[cfg(feature = "api-admin-v1")]
use crate::api::zitadel::admin::v1::admin_service_client::AdminServiceClient;
#[cfg(feature = "api-auth-v1")]
use crate::api::zitadel::auth::v1::auth_service_client::AuthServiceClient;
#[cfg(feature = "api-management-v1")]
use crate::api::zitadel::management::v1::management_service_client::ManagementServiceClient;
#[cfg(feature = "api-system-v1")]
use crate::api::zitadel::system::v1::system_service_client::SystemServiceClient;

#[cfg(feature = "interceptors")]
use crate::credentials::{AuthenticationOptions, ServiceAccount};

custom_error! {
    /// Errors that may occur when creating a client.
    pub ClientError
        InvalidUrl = "the provided url is invalid",
        ConnectionError = "could not connect to provided endpoint",
        TlsInitializationError = "could not setup tls connection",
}

/// A builder to create configured gRPC clients for ZITADEL API access.
/// The builder accepts the api endpoint and (depending on activated features)
/// an authentication method.
pub struct ClientBuilder {
    api_endpoint: String,
}

#[cfg(feature = "interceptors")]
pub struct ClientBuilderWithInterceptor<T: Interceptor> {
    api_endpoint: String,
    interceptor: T,
}


impl ClientBuilder {
    /// Create a new client builder with the the provided endpoint.
    pub fn new(api_endpoint: &str) -> ClientBuilder {
        ClientBuilder {
            api_endpoint: api_endpoint.to_string(),
        }
    }
}

impl ClientBuilder {
    /// Configure the client builder to use a provided access token.
    /// This can be a pre-fetched token from ZITADEL or some other form
    /// of a valid access token like a personal access token (PAT).
    ///
    /// Clients with this authentication method will have the [`AccessTokenInterceptor`]
    /// attached.
    #[cfg(feature = "interceptors")]
    pub fn with_access_token(self, access_token: &str) -> ClientBuilderWithInterceptor<AccessTokenInterceptor> {
        ClientBuilderWithInterceptor {
            api_endpoint: self.api_endpoint,
            interceptor: AccessTokenInterceptor::new(access_token),
        }
    }

    /// Configure the client builder to use a [`ServiceAccount`][crate::credentials::ServiceAccount].
    /// The service account will be used to fetch a valid access token from ZITADEL.
    ///
    /// Clients with this authentication method will have the
    /// [`ServiceAccountInterceptor`][crate::api::interceptors::ServiceAccountInterceptor] attached
    /// that fetches an access token from ZITADEL and renewes it when it expires.
    #[cfg(feature = "interceptors")]
    pub fn with_service_account(
        self,
        service_account: &ServiceAccount,
        auth_options: Option<AuthenticationOptions>,
    ) -> ClientBuilderWithInterceptor<ServiceAccountInterceptor> {
        let interceptor = ServiceAccountInterceptor::new(
            &self.api_endpoint,
            service_account,
            auth_options.clone(),
        );
        ClientBuilderWithInterceptor {
            api_endpoint: self.api_endpoint,
            interceptor,
        }
    }
}

impl ClientBuilder {
    /// Create a new [`AdminServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-admin-v1")]
    pub async fn build_admin_client(self) -> Result<AdminServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(AdminServiceClient::new(channel))
    }

    /// Create a new [`AuthServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-auth-v1")]
    pub async fn build_auth_client(self) -> Result<AuthServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(AuthServiceClient::new(channel))
    }

    /// Create a new [`ManagementServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-management-v1")]
    pub async fn build_management_client(
        self,
    ) -> Result<ManagementServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(ManagementServiceClient::new(channel))
    }

    /// Create a new [`OidcServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-oidc-v2")]
    pub async fn build_oidc_client(self) -> Result<OidcServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(OidcServiceClient::new(channel))
    }

    /// Create a new [`OrganizationServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-org-v2")]
    pub async fn build_organization_client(
        self,
    ) -> Result<OrganizationServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(OrganizationServiceClient::new(channel))
    }

    /// Create a new [`SessionServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-session-v2")]
    pub async fn build_session_client(
        self,
    ) -> Result<SessionServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(SessionServiceClient::new(channel))
    }

    /// Create a new [`SettingsServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-settings-v2")]
    pub async fn build_settings_client(
        self,
    ) -> Result<SettingsServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(SettingsServiceClient::new(channel))
    }

    /// Create a new [`SystemServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-system-v1")]
    pub async fn build_system_client(self) -> Result<SystemServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(SystemServiceClient::new(channel))
    }

    /// Create a new [`UserServiceClient`].
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    #[cfg(feature = "api-user-v2")]
    pub async fn build_user_client(self) -> Result<UserServiceClient<Channel>, Box<dyn Error>> {
        let channel = crate::api::clients::get_channel(&self.api_endpoint).await?;
        Ok(UserServiceClient::new(channel))
    }
}

#[cfg(feature = "interceptors")]
impl<T: Interceptor> ClientBuilderWithInterceptor<T> {
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
    #[cfg(feature = "api-admin-v1")]
    pub async fn build_admin_client(
        self,
    ) -> Result<AdminServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(AdminServiceClient::with_interceptor(
            channel,
            self.interceptor,
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
    #[cfg(feature = "api-auth-v1")]
    pub async fn build_auth_client(
        self,
    ) -> Result<AuthServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(AuthServiceClient::with_interceptor(
            channel,
            self.interceptor,
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
    #[cfg(feature = "api-management-v1")]
    pub async fn build_management_client(
        self,
    ) -> Result<ManagementServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(ManagementServiceClient::with_interceptor(
            channel,
            self.interceptor,
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
    #[cfg(feature = "api-oidc-v2")]
    pub async fn build_oidc_client(
        self,
    ) -> Result<OidcServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(OidcServiceClient::with_interceptor(
            channel,
            self.interceptor,
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
    #[cfg(feature = "api-org-v2")]
    pub async fn build_organization_client(
        self,
    ) -> Result<OrganizationServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(OrganizationServiceClient::with_interceptor(
            channel,
            self.interceptor,
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
    #[cfg(feature = "api-session-v2")]
    pub async fn build_session_client(
        self,
    ) -> Result<SessionServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(SessionServiceClient::with_interceptor(
            channel,
            self.interceptor,
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
    #[cfg(feature = "api-settings-v2")]
    pub async fn build_settings_client(
        self,
    ) -> Result<SettingsServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(SettingsServiceClient::with_interceptor(
            channel,
            self.interceptor,
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
    #[cfg(feature = "api-system-v1")]
    pub async fn build_system_client(
        self,
    ) -> Result<SystemServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(SystemServiceClient::with_interceptor(
            channel,
            self.interceptor,
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
    #[cfg(feature = "api-user-v2")]
    pub async fn build_user_client(
        self,
    ) -> Result<UserServiceClient<InterceptedService<Channel, T>>, Box<dyn Error>> {
        let channel = get_channel(&self.api_endpoint).await?;
        Ok(UserServiceClient::with_interceptor(
            channel,
            self.interceptor,
        ))
    }
}

async fn get_channel(api_endpoint: &str) -> Result<Channel, ClientError> {
    Endpoint::from_shared(api_endpoint.to_string())
        .map_err(|_| ClientError::InvalidUrl)?
        .tls_config(
            ClientTlsConfig::default()
                .assume_http2(true)
                .with_native_roots(),
        )
        .map_err(|_| ClientError::TlsInitializationError)?
        .connect()
        .await
        .map_err(|_| ClientError::ConnectionError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    const SERVICE_ACCOUNT: &str = r#"
    {
        "type": "serviceaccount",
        "keyId": "181828078849229057",
        "key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEpQIBAAKCAQEA9VIWALQqzx1ypi42t7MG4KSOMldD10brsEUjTcjqxhl6TJrP\nsjaNKWArnV/XH+6ZKRd55mUEFFx9VflqdwQtMVPjZKXpV4cFDiPwf1Z1h1DS6im4\nSo7eKR7OGb7TLBhwt7i2UPF4WnxBhTp/M6pG5kCJ1t8glIo5yRbrILXObRmvNWMz\nVIFAyw68NDZGYNhnR8AT43zjeJTFXG/suuEoXO/mMmMjsYY8kS0BbiQeq5t5hIrr\na/odswkDPn5Zd4P91iJHDnYlgfJuo3oRmgpOj/dDsl+vTol+vveeMO4TXPwZcl36\ngUNPok7nd6BA3gqmOS+fMImzmZB42trghARXXwIDAQABAoIBAQCbMOGQcml+ep+T\ntzqQPWYFaLQ37nKRVmE1Mpeh1o+G4Ik4utrXX6EvYpJUzVN29ObZUuufr5nEE7qK\nT+1k+zRntyzr9/VElLrC9kNnGtfg0WWMEvZt3DF4i+9P5CMNCy0LXIOhcxBzFZYR\nZS8hDQArGvrX/nFK5qKlrqTyHXFIHDFa6z59ErhXEnsTgRvx/Mo+6UkdBkHsKnlJ\nAbXqXFbfz6nDsF1DgRra5ODn1k8nZqnC/YcssE7/dlbuByz10ECkOSzqYcfufnsb\n9N1Ld4Xlj3yzsqPFzEJyHHm9eEHQXsPavaXiM64/+zpsksLscEIE/0KtIy5tngpZ\nSCqZAcj5AoGBAPb1bQFWUBmmUuSTtSymsxgXghJiJ3r+jJgdGbkv2IsRTs4En5Sz\n0SbPE1YWmMDDgTacJlB4/XiaojQ/j1EEY17inxYomE72UL6/ET7ycsEw3e9ALuD5\np0y2Sdzes2biH30bw5jD8kJ+hV18T745KtzrwSH4I0lAjnkmiH+0S67VAoGBAP5N\nTtAp/Qdxh9GjNSw1J7KRLtJrrr0pPrJ9av4GoFoWlz+Qw2X3dl8rjG3Bqz9LPV7A\ngiHMel8WTmdIM/S3F4Q3ufEfE+VzG+gncWd9SJfX5/LVhatPzTGLNsY7AYGEpSwT\n5/0anS1mHrLwsVcPrZnigekr5A5mfZl6nxtOnE9jAoGBALACqacbUkmFrmy1DZp+\nUQSptI3PoR3bEG9VxkCjZi1vr3/L8cS1CCslyT1BK6uva4d1cSVHpjfv1g1xA38V\nppE46XOMiUk16sSYPv1jJQCmCHd9givcIy3cefZOTwTTwueTAyv888wKipjfgaIs\n8my0JllEljmeJi0Ylo6V/J7lAoGBAIFqRlmZhLNtC3mcXUsKIhG14OYk9uA9RTMA\nsJpmNOSj6oTm3wndTdhRCT4x+TxUxf6aaZ9ZuEz7xRq6m/ZF1ynqUi5ramyyj9kt\neYD5OSBNODVUhJoSGpLEDjQDg1iucIBmAQHFsYeRGL5nz1hHGkneA87uDzlk3zZk\nOORktReRAoGAGUfU2UfaniAlqrZsSma3ZTlvJWs1x8cbVDyKTYMX5ShHhp+cA86H\nYjSSol6GI2wQPP+qIvZ1E8XyzD2miMJabl92/WY0tHejNNBEHwD8uBZKrtMoFWM7\nWJNl+Xneu/sT8s4pP2ng6QE7jpHXi2TUNmSlgQry9JN2AmA9TuSTW2Y=\n-----END RSA PRIVATE KEY-----\n",
        "userId": "181828061098934529"
    }"#;

    #[test]
    fn client_builder_with_access_token_attaches_it() {
        let mut interceptor = ClientBuilder::new(ZITADEL_URL)
            .with_access_token("token")
            .interceptor;
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
            .interceptor;
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
