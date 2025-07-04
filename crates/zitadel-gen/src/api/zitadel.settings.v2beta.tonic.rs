// @generated
/// Generated client implementations.
pub mod settings_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SettingsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SettingsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SettingsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SettingsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SettingsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_general_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGeneralSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGeneralSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetGeneralSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetGeneralSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_login_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLoginSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLoginSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetLoginSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetLoginSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get the current active identity providers
*/
        pub async fn get_active_identity_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetActiveIdentityProvidersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetActiveIdentityProvidersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetActiveIdentityProviders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetActiveIdentityProviders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get the password complexity settings
*/
        pub async fn get_password_complexity_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPasswordComplexitySettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPasswordComplexitySettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetPasswordComplexitySettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetPasswordComplexitySettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get the password expiry settings
*/
        pub async fn get_password_expiry_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPasswordExpirySettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPasswordExpirySettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetPasswordExpirySettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetPasswordExpirySettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get the current active branding settings
*/
        pub async fn get_branding_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandingSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBrandingSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetBrandingSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetBrandingSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get the domain settings
*/
        pub async fn get_domain_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDomainSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDomainSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetDomainSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetDomainSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get the legal and support settings
*/
        pub async fn get_legal_and_support_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLegalAndSupportSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLegalAndSupportSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetLegalAndSupportSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetLegalAndSupportSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get the lockout settings
*/
        pub async fn get_lockout_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLockoutSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLockoutSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetLockoutSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetLockoutSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_security_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecuritySettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSecuritySettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/GetSecuritySettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "GetSecuritySettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_security_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::SetSecuritySettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetSecuritySettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.settings.v2beta.SettingsService/SetSecuritySettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.settings.v2beta.SettingsService",
                        "SetSecuritySettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
