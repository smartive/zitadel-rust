//! Module with convenience functions to create clients for ZITADEL
//! API access. When the credentials feature is enabled, additional
//! modules provide access to functions that create clients with
//! specific interceptors for authentication.

use custom_error::custom_error;
use tonic::transport::{Channel, Endpoint};

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

async fn get_channel(api_endpoint: &str) -> Result<Channel, ClientError> {
    Endpoint::from_shared(api_endpoint.to_string())
        .map_err(|_| ClientError::InvalidUrl)?
        .connect()
        .await
        .map_err(|_| ClientError::ConnectionError)
}

/// Create a new [`AdminServiceClient`] to access the
/// [Admin API](https://docs.zitadel.com/docs/apis/proto/admin) of ZITADEL.
///
/// This client has no interceptors attached, therefore, authentication
/// must be handled for each call.
///
/// ### Errors
///
/// This function returns a [`ClientError`] if the provided API endpoint
/// cannot be parsed into a valid URL or if the connection to the endpoint
/// is not possible.
///
/// ### Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
/// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
/// let client = zitadel::api::clients::create_admin_client(ZITADEL_URL).await?;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "api")]
pub async fn create_admin_client(
    api_endpoint: &str,
) -> Result<AdminServiceClient<Channel>, ClientError> {
    let channel = get_channel(api_endpoint).await?;
    Ok(AdminServiceClient::new(channel))
}

/// Create a new [`AuthServiceClient`] to access the
/// [Auth API](https://docs.zitadel.com/docs/apis/proto/auth) of ZITADEL.
///
/// This client has no interceptors attached, therefore, authentication
/// must be handled for each call.
///
/// ### Errors
///
/// This function returns a [`ClientError`] if the provided API endpoint
/// cannot be parsed into a valid URL or if the connection to the endpoint
/// is not possible.
///
/// ### Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
/// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
/// let client = zitadel::api::clients::create_auth_client(ZITADEL_URL).await?;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "api")]
pub async fn create_auth_client(
    api_endpoint: &str,
) -> Result<AuthServiceClient<Channel>, ClientError> {
    let channel = get_channel(api_endpoint).await?;
    Ok(AuthServiceClient::new(channel))
}

/// Create a new [`ManagementServiceClient`] to access the
/// [Management API](https://docs.zitadel.com/docs/apis/proto/management) of ZITADEL.
///
/// This client has no interceptors attached, therefore, authentication
/// must be handled for each call.
///
/// ### Errors
///
/// This function returns a [`ClientError`] if the provided API endpoint
/// cannot be parsed into a valid URL or if the connection to the endpoint
/// is not possible.
///
/// ### Example
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
/// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
/// let client = zitadel::api::clients::create_management_client(ZITADEL_URL).await?;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "api")]
pub async fn create_management_client(
    api_endpoint: &str,
) -> Result<ManagementServiceClient<Channel>, ClientError> {
    let channel = get_channel(api_endpoint).await?;
    Ok(ManagementServiceClient::new(channel))
}

/// Module with convenience functions to create clients for ZITADEL
/// API access with an arbitrary access token. The access token
/// may be one of the following types:
/// - User provided access token that came from the users request
/// - Service account personal access key
/// - Some other means of fetching a valid access token
#[cfg(all(feature = "api", feature = "interceptors"))]
pub mod with_access_token {
    use tonic::codegen::InterceptedService;
    use tonic::transport::Channel;

    use super::{get_channel, ClientError};
    use crate::api::interceptors::AccessTokenInterceptor;
    use crate::api::zitadel::{
        admin::v1::admin_service_client::AdminServiceClient,
        auth::v1::auth_service_client::AuthServiceClient,
        management::v1::management_service_client::ManagementServiceClient,
    };

    /// Create a new [`AdminServiceClient`] to access the
    /// [Admin API](https://docs.zitadel.com/docs/apis/proto/admin) of ZITADEL.
    ///
    /// This client has the [`AccessTokenInterceptor`][crate::api::interceptors::AccessTokenInterceptor] attached that
    /// allows authentication against the ZITADEL API with some access token.
    /// This could be a personal access token of a service account.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    ///
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    /// let client = zitadel::api::clients::with_access_token::create_admin_client(ZITADEL_URL, "token").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "api")]
    pub async fn create_admin_client(
        api_endpoint: &str,
        access_token: &str,
    ) -> Result<AdminServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>, ClientError>
    {
        let channel = get_channel(api_endpoint).await?;
        Ok(AdminServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(access_token),
        ))
    }

    /// Create a new [`AuthServiceClient`] to access the
    /// [Auth API](https://docs.zitadel.com/docs/apis/proto/auth) of ZITADEL.
    ///
    /// This client has the [`AccessTokenInterceptor`][crate::api::interceptors::AccessTokenInterceptor] attached that
    /// allows authentication against the ZITADEL API with some access token.
    /// This could be a personal access token of a service account.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    ///
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    /// let client = zitadel::api::clients::with_access_token::create_auth_client(ZITADEL_URL, "token").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "api")]
    pub async fn create_auth_client(
        api_endpoint: &str,
        access_token: &str,
    ) -> Result<AuthServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>, ClientError>
    {
        let channel = get_channel(api_endpoint).await?;
        Ok(AuthServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(access_token),
        ))
    }

    /// Create a new [`ManagementServiceClient`] to access the
    /// [Management API](https://docs.zitadel.com/docs/apis/proto/management) of ZITADEL.
    ///
    /// This client has the [`AccessTokenInterceptor`][crate::api::interceptors::AccessTokenInterceptor] attached that
    /// allows authentication against the ZITADEL API with some access token.
    /// This could be a personal access token of a service account.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    ///
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    /// let client = zitadel::api::clients::with_access_token::create_management_client(ZITADEL_URL, "token").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "api")]
    pub async fn create_management_client(
        api_endpoint: &str,
        access_token: &str,
    ) -> Result<
        ManagementServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        ClientError,
    > {
        let channel = get_channel(api_endpoint).await?;
        Ok(ManagementServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(access_token),
        ))
    }
}

/// Module with convenience functions to create clients for ZITADEL
/// API access with [`ServiceAccount`][crate::credentials::ServiceAccount]
/// authentication. Provide a service account JWT profile to create
/// a gRPC service client that fetches a valid access token from ZITADEL.
#[cfg(all(feature = "api", feature = "interceptors", feature = "credentials"))]
pub mod with_service_account {
    use tonic::codegen::InterceptedService;
    use tonic::transport::Channel;

    use super::{get_channel, ClientError};
    use crate::api::interceptors::ServiceAccountInterceptor;
    use crate::api::zitadel::{
        admin::v1::admin_service_client::AdminServiceClient,
        auth::v1::auth_service_client::AuthServiceClient,
        management::v1::management_service_client::ManagementServiceClient,
    };
    use crate::credentials::{AuthenticationOptions, ServiceAccount};

    /// Create a new [`AdminServiceClient`] to access the
    /// [Admin API](https://docs.zitadel.com/docs/apis/proto/admin) of ZITADEL.
    ///
    /// This client has the [`ServiceAccountInterceptor`][crate::api::interceptors::ServiceAccountInterceptor]
    /// attached that fetches a valid access token with a service account JWT profile from ZITADEL.
    /// The provided `api_endpoint` is used as audience for the service account token.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    ///
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    /// # let service_account = zitadel::credentials::ServiceAccount::load_from_json(r#"{"keyId": "1337", "userId": "42", "key": "foobar"}"#)?;
    /// let client = zitadel::api::clients::with_service_account::create_admin_client(ZITADEL_URL, &service_account, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "api")]
    pub async fn create_admin_client(
        api_endpoint: &str,
        service_account: &ServiceAccount,
        auth_options: Option<AuthenticationOptions>,
    ) -> Result<
        AdminServiceClient<InterceptedService<Channel, ServiceAccountInterceptor>>,
        ClientError,
    > {
        let channel = get_channel(api_endpoint).await?;
        Ok(AdminServiceClient::with_interceptor(
            channel,
            ServiceAccountInterceptor::new(api_endpoint, service_account, auth_options),
        ))
    }

    /// Create a new [`AuthServiceClient`] to access the
    /// [Auth API](https://docs.zitadel.com/docs/apis/proto/auth) of ZITADEL.
    ///
    /// This client has the [`ServiceAccountInterceptor`][crate::api::interceptors::ServiceAccountInterceptor]
    /// attached that fetches a valid access token with a service account JWT profile from ZITADEL.
    /// The provided `api_endpoint` is used as audience for the service account token.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    ///
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    /// # let service_account = zitadel::credentials::ServiceAccount::load_from_json(r#"{"keyId": "1337", "userId": "42", "key": "foobar"}"#)?;
    /// let client = zitadel::api::clients::with_service_account::create_auth_client(ZITADEL_URL, &service_account, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "api")]
    pub async fn create_auth_client(
        api_endpoint: &str,
        service_account: &ServiceAccount,
        auth_options: Option<AuthenticationOptions>,
    ) -> Result<
        AuthServiceClient<InterceptedService<Channel, ServiceAccountInterceptor>>,
        ClientError,
    > {
        let channel = get_channel(api_endpoint).await?;
        Ok(AuthServiceClient::with_interceptor(
            channel,
            ServiceAccountInterceptor::new(api_endpoint, service_account, auth_options),
        ))
    }

    /// Create a new [`ManagementServiceClient`] to access the
    /// [Management API](https://docs.zitadel.com/docs/apis/proto/management) of ZITADEL.
    ///
    /// This client has the [`ServiceAccountInterceptor`][crate::api::interceptors::ServiceAccountInterceptor]
    /// attached that fetches a valid access token with a service account JWT profile from ZITADEL.
    /// The provided `api_endpoint` is used as audience for the service account token.
    ///
    /// ### Errors
    ///
    /// This function returns a [`ClientError`] if the provided API endpoint
    /// cannot be parsed into a valid URL or if the connection to the endpoint
    /// is not possible.
    ///
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// # const ZITADEL_URL: &str = "https://zitadel-libraries-l8boqa.zitadel.cloud";
    /// # let service_account = zitadel::credentials::ServiceAccount::load_from_json(r#"{"keyId": "1337", "userId": "42", "key": "foobar"}"#)?;
    /// let client = zitadel::api::clients::with_service_account::create_management_client(ZITADEL_URL, &service_account, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "api")]
    pub async fn create_management_client(
        api_endpoint: &str,
        service_account: &ServiceAccount,
        auth_options: Option<AuthenticationOptions>,
    ) -> Result<
        ManagementServiceClient<InterceptedService<Channel, ServiceAccountInterceptor>>,
        ClientError,
    > {
        let channel = get_channel(api_endpoint).await?;
        Ok(ManagementServiceClient::with_interceptor(
            channel,
            ServiceAccountInterceptor::new(api_endpoint, service_account, auth_options),
        ))
    }
}
