//! Contains convenience functions to create gRPC clients for the
//! ZITADEL API with authentication interceptors directly attached to
//! the clients.

/// Module with convenience functions that create gRPC
/// clients for the ZITADEL API that authenticate calls
/// with an access token. The used [interceptor][crate::grpc::interceptors::AccessTokenInterceptor]
/// does not refresh nor update the given access token.
///
/// These clients may be used for short living client examples
/// where a series of calls will be executed and the client is
/// destroyed afterwards.
pub mod with_access_token {
    use std::str::FromStr;

    use tonic::codegen::InterceptedService;
    use tonic::transport::{Channel, Endpoint};

    use crate::credentials::ServiceAccount;
    use crate::grpc::interceptors::AccessTokenInterceptor;
    use crate::grpc::zitadel::admin::v1::admin_service_client::AdminServiceClient;
    use crate::grpc::zitadel::auth::v1::auth_service_client::AuthServiceClient;
    use crate::grpc::zitadel::management::v1::management_service_client::ManagementServiceClient;

    /// Create a new [`ManagementServiceClient`][ManagementServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a [`ServiceAccount`][ServiceAccount] and fetches an
    /// access token from the service account. Further, the default [`API Endpoint`][crate::API_ENDPOINT]
    /// is used to connect the client.
    ///
    /// Returns a fully configured [`ManagementServiceClient`][ManagementServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::management_client_with_account;
    /// use zitadel::grpc::zitadel::management::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let mut client = management_client_with_account(&sa).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn management_client_with_account(
        account: &ServiceAccount,
    ) -> Result<
        ManagementServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_static(crate::API_ENDPOINT).connect().await?;
        let token = account.authenticate().await?;
        Ok(ManagementServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`ManagementServiceClient`][ManagementServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a pre-fetched access token. Further, the default [`API Endpoint`][crate::API_ENDPOINT]
    /// is used to connect the client.
    ///
    /// Returns a fully configured [`ManagementServiceClient`][ManagementServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::management_client_with_token;
    /// use zitadel::grpc::zitadel::management::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let token = sa.authenticate().await?;
    /// let mut client = management_client_with_token(token).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn management_client_with_token(
        token: String,
    ) -> Result<
        ManagementServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_static(crate::API_ENDPOINT).connect().await?;
        Ok(ManagementServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`ManagementServiceClient`][ManagementServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a [`ServiceAccount`][ServiceAccount] and fetches an
    /// access token from the service account. Further, one needs to specify the API
    /// endpoint that the client shall connect to (endpoint). This can be used
    /// to connect the grpc client to a custom self hosted ZITADEL API.
    ///
    /// Returns a fully configured [`ManagementServiceClient`][ManagementServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::custom_management_client_with_account;
    /// use zitadel::grpc::zitadel::management::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let mut client = custom_management_client_with_account("https://custom.api.com", &sa).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn custom_management_client_with_account(
        endpoint: &str,
        account: &ServiceAccount,
    ) -> Result<
        ManagementServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_str(endpoint)?.connect().await?;
        let token = account.authenticate().await?;
        Ok(ManagementServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`ManagementServiceClient`][ManagementServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a pre-fetched access token. Further, one needs to specify the API
    /// endpoint that the client shall connect to (endpoint). This can be used
    /// to connect the grpc client to a custom self hosted ZITADEL API.
    ///
    /// Returns a fully configured [`ManagementServiceClient`][ManagementServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::custom_management_client_with_token;
    /// use zitadel::grpc::zitadel::management::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let token = sa.authenticate().await?;
    /// let mut client = custom_management_client_with_token("https://custom.api.com", token).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn custom_management_client_with_token(
        endpoint: &str,
        token: String,
    ) -> Result<
        ManagementServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_str(endpoint)?.connect().await?;
        Ok(ManagementServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`AuthServiceClient`][AuthServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a [`ServiceAccount`][ServiceAccount] and fetches an
    /// access token from the service account. Further, the default [`API Endpoint`][crate::API_ENDPOINT]
    /// is used to connect the client.
    ///
    /// Returns a fully configured [`AuthServiceClient`][AuthServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::auth_client_with_account;
    /// use zitadel::grpc::zitadel::auth::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let mut client = auth_client_with_account(&sa).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn auth_client_with_account(
        account: &ServiceAccount,
    ) -> Result<
        AuthServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_static(crate::API_ENDPOINT).connect().await?;
        let token = account.authenticate().await?;
        Ok(AuthServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`AuthServiceClient`][AuthServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a pre-fetched access token. Further, the default [`API Endpoint`][crate::API_ENDPOINT]
    /// is used to connect the client.
    ///
    /// Returns a fully configured [`AuthServiceClient`][AuthServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::auth_client_with_token;
    /// use zitadel::grpc::zitadel::auth::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let token = sa.authenticate().await?;
    /// let mut client = auth_client_with_token(token).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn auth_client_with_token(
        token: String,
    ) -> Result<
        AuthServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_static(crate::API_ENDPOINT).connect().await?;
        Ok(AuthServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`AuthServiceClient`][AuthServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a [`ServiceAccount`][ServiceAccount] and fetches an
    /// access token from the service account. Further, one needs to specify the API
    /// endpoint that the client shall connect to (endpoint). This can be used
    /// to connect the grpc client to a custom self hosted ZITADEL API.
    ///
    /// Returns a fully configured [`AuthServiceClient`][AuthServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::custom_auth_client_with_account;
    /// use zitadel::grpc::zitadel::auth::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let mut client = custom_auth_client_with_account("https://custom.api.com", &sa).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn custom_auth_client_with_account(
        endpoint: &str,
        account: &ServiceAccount,
    ) -> Result<
        AuthServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_str(endpoint)?.connect().await?;
        let token = account.authenticate().await?;
        Ok(AuthServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`AuthServiceClient`][AuthServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a pre-fetched access token. Further, one needs to specify the API
    /// endpoint that the client shall connect to (endpoint). This can be used
    /// to connect the grpc client to a custom self hosted ZITADEL API.
    ///
    /// Returns a fully configured [`AuthServiceClient`][AuthServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::custom_auth_client_with_token;
    /// use zitadel::grpc::zitadel::auth::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let token = sa.authenticate().await?;
    /// let mut client = custom_auth_client_with_token("https://custom.api.com", token).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn custom_auth_client_with_token(
        endpoint: &str,
        token: String,
    ) -> Result<
        AuthServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_str(endpoint)?.connect().await?;
        Ok(AuthServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`AdminServiceClient`][AdminServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a [`ServiceAccount`][ServiceAccount] and fetches an
    /// access token from the service account. Further, the default [`API Endpoint`][crate::API_ENDPOINT]
    /// is used to connect the client.
    ///
    /// Returns a fully configured [`AdminServiceClient`][AdminServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::admin_client_with_account;
    /// use zitadel::grpc::zitadel::admin::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let mut client = admin_client_with_account(&sa).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn admin_client_with_account(
        account: &ServiceAccount,
    ) -> Result<
        AdminServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_static(crate::API_ENDPOINT).connect().await?;
        let token = account.authenticate().await?;
        Ok(AdminServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`AdminServiceClient`][AdminServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a pre-fetched access token. Further, the default [`API Endpoint`][crate::API_ENDPOINT]
    /// is used to connect the client.
    ///
    /// Returns a fully configured [`AdminServiceClient`][AdminServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::admin_client_with_token;
    /// use zitadel::grpc::zitadel::admin::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let token = sa.authenticate().await?;
    /// let mut client = admin_client_with_token(token).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn admin_client_with_token(
        token: String,
    ) -> Result<
        AdminServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_static(crate::API_ENDPOINT).connect().await?;
        Ok(AdminServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`AdminServiceClient`][AdminServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a [`ServiceAccount`][ServiceAccount] and fetches an
    /// access token from the service account. Further, one needs to specify the API
    /// endpoint that the client shall connect to (endpoint). This can be used
    /// to connect the grpc client to a custom self hosted ZITADEL API.
    ///
    /// Returns a fully configured [`AdminServiceClient`][AdminServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::custom_admin_client_with_account;
    /// use zitadel::grpc::zitadel::admin::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let mut client = custom_admin_client_with_account("https://custom.api.com", &sa).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn custom_admin_client_with_account(
        endpoint: &str,
        account: &ServiceAccount,
    ) -> Result<
        AdminServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_str(endpoint)?.connect().await?;
        let token = account.authenticate().await?;
        Ok(AdminServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }

    /// Create a new [`AdminServiceClient`][AdminServiceClient] with an
    /// [`AccessTokenInterceptor`][AccessTokenInterceptor] attached.
    ///
    /// This function takes a pre-fetched access token. Further, one needs to specify the API
    /// endpoint that the client shall connect to (endpoint). This can be used
    /// to connect the grpc client to a custom self hosted ZITADEL API.
    ///
    /// Returns a fully configured [`AdminServiceClient`][AdminServiceClient]
    /// with the attached interceptor and an access token. The interceptor does not
    /// check if the access token is valid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// use zitadel::credentials::ServiceAccount;
    /// use zitadel::grpc::clients::with_access_token::custom_admin_client_with_token;
    /// use zitadel::grpc::zitadel::admin::v1::HealthzRequest;
    ///
    /// let sa = ServiceAccount::load_from_json("./serviceaccount.json")?;
    /// let token = sa.authenticate().await?;
    /// let mut client = custom_admin_client_with_token("https://custom.api.com", token).await?;
    /// let request = tonic::Request::new(HealthzRequest{});
    /// let result = client.healthz(request).await?;
    /// println!("{:#?}", result.into_inner());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn custom_admin_client_with_token(
        endpoint: &str,
        token: String,
    ) -> Result<
        AdminServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
        Box<dyn std::error::Error>,
    > {
        let channel = Endpoint::from_str(endpoint)?.connect().await?;
        Ok(AdminServiceClient::with_interceptor(
            channel,
            AccessTokenInterceptor::new(token),
        ))
    }
}
