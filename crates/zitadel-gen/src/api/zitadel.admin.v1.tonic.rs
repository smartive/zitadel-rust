// @generated
/// Generated client implementations.
pub mod admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AdminServiceClient<tonic::transport::Channel> {
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
    impl<T> AdminServiceClient<T>
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
        ) -> AdminServiceClient<InterceptedService<T, F>>
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
            AdminServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn healthz(
            &mut self,
            request: impl tonic::IntoRequest<super::HealthzRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HealthzResponse>,
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
                "/zitadel.admin.v1.AdminService/Healthz",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "Healthz"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_supported_languages(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSupportedLanguagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSupportedLanguagesResponse>,
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
                "/zitadel.admin.v1.AdminService/GetSupportedLanguages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetSupportedLanguages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_allowed_languages(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllowedLanguagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAllowedLanguagesResponse>,
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
                "/zitadel.admin.v1.AdminService/GetAllowedLanguages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetAllowedLanguages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_language(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDefaultLanguageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultLanguageResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultLanguage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultLanguage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_language(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultLanguageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultLanguageResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultLanguage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultLanguage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyInstanceResponse>,
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
                "/zitadel.admin.v1.AdminService/GetMyInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetMyInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_instance_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceDomainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstanceDomainsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListInstanceDomains",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListInstanceDomains",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_instance_trusted_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceTrustedDomainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstanceTrustedDomainsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListInstanceTrustedDomains",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListInstanceTrustedDomains",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_instance_trusted_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::AddInstanceTrustedDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddInstanceTrustedDomainResponse>,
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
                "/zitadel.admin.v1.AdminService/AddInstanceTrustedDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddInstanceTrustedDomain",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_instance_trusted_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveInstanceTrustedDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveInstanceTrustedDomainResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveInstanceTrustedDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveInstanceTrustedDomain",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_secret_generators(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSecretGeneratorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSecretGeneratorsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListSecretGenerators",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListSecretGenerators",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_secret_generator(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretGeneratorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSecretGeneratorResponse>,
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
                "/zitadel.admin.v1.AdminService/GetSecretGenerator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetSecretGenerator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_secret_generator(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecretGeneratorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSecretGeneratorResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateSecretGenerator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateSecretGenerator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_smtp_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSmtpConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSmtpConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/GetSMTPConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetSMTPConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_smtp_config_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSmtpConfigByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSmtpConfigByIdResponse>,
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
                "/zitadel.admin.v1.AdminService/GetSMTPConfigById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetSMTPConfigById"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_smtp_config(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSmtpConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddSmtpConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/AddSMTPConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddSMTPConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_smtp_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSmtpConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSmtpConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateSMTPConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "UpdateSMTPConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_smtp_config_password(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSmtpConfigPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSmtpConfigPasswordResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateSMTPConfigPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateSMTPConfigPassword",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate_smtp_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateSmtpConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateSmtpConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/ActivateSMTPConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ActivateSMTPConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_smtp_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateSmtpConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateSmtpConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/DeactivateSMTPConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "DeactivateSMTPConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_smtp_config(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveSmtpConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveSmtpConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveSMTPConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "RemoveSMTPConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn test_smtp_config_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::TestSmtpConfigByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestSmtpConfigByIdResponse>,
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
                "/zitadel.admin.v1.AdminService/TestSMTPConfigById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "TestSMTPConfigById",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn test_smtp_config(
            &mut self,
            request: impl tonic::IntoRequest<super::TestSmtpConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestSmtpConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/TestSMTPConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "TestSMTPConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_smtp_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSmtpConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSmtpConfigsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListSMTPConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "ListSMTPConfigs"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_email_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEmailProvidersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEmailProvidersResponse>,
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
                "/zitadel.admin.v1.AdminService/ListEmailProviders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListEmailProviders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_email_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEmailProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEmailProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/GetEmailProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetEmailProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_email_provider_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEmailProviderByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEmailProviderByIdResponse>,
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
                "/zitadel.admin.v1.AdminService/GetEmailProviderById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetEmailProviderById",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_email_provider_smtp(
            &mut self,
            request: impl tonic::IntoRequest<super::AddEmailProviderSmtpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddEmailProviderSmtpResponse>,
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
                "/zitadel.admin.v1.AdminService/AddEmailProviderSMTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddEmailProviderSMTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_email_provider_smtp(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEmailProviderSmtpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateEmailProviderSmtpResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateEmailProviderSMTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateEmailProviderSMTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_email_provider_http(
            &mut self,
            request: impl tonic::IntoRequest<super::AddEmailProviderHttpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddEmailProviderHttpResponse>,
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
                "/zitadel.admin.v1.AdminService/AddEmailProviderHTTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddEmailProviderHTTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_email_provider_http(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEmailProviderHttpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateEmailProviderHttpResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateEmailProviderHTTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateEmailProviderHTTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_email_provider_smtp_password(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateEmailProviderSmtpPasswordRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateEmailProviderSmtpPasswordResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateEmailProviderSMTPPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateEmailProviderSMTPPassword",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate_email_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateEmailProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateEmailProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/ActivateEmailProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ActivateEmailProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_email_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateEmailProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateEmailProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/DeactivateEmailProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "DeactivateEmailProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_email_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveEmailProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveEmailProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveEmailProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveEmailProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn test_email_provider_smtp_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::TestEmailProviderSmtpByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestEmailProviderSmtpByIdResponse>,
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
                "/zitadel.admin.v1.AdminService/TestEmailProviderSMTPById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "TestEmailProviderSMTPById",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn test_email_provider_smtp(
            &mut self,
            request: impl tonic::IntoRequest<super::TestEmailProviderSmtpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestEmailProviderSmtpResponse>,
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
                "/zitadel.admin.v1.AdminService/TestEmailProviderSMTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "TestEmailProviderSMTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_sms_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSmsProvidersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSmsProvidersResponse>,
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
                "/zitadel.admin.v1.AdminService/ListSMSProviders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "ListSMSProviders"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_sms_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSmsProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSmsProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/GetSMSProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetSMSProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_sms_provider_twilio(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSmsProviderTwilioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddSmsProviderTwilioResponse>,
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
                "/zitadel.admin.v1.AdminService/AddSMSProviderTwilio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddSMSProviderTwilio",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_sms_provider_twilio(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSmsProviderTwilioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSmsProviderTwilioResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateSMSProviderTwilio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateSMSProviderTwilio",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_sms_provider_twilio_token(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSmsProviderTwilioTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSmsProviderTwilioTokenResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateSMSProviderTwilioToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateSMSProviderTwilioToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_sms_provider_http(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSmsProviderHttpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddSmsProviderHttpResponse>,
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
                "/zitadel.admin.v1.AdminService/AddSMSProviderHTTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddSMSProviderHTTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_sms_provider_http(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSmsProviderHttpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSmsProviderHttpResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateSMSProviderHTTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateSMSProviderHTTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate_sms_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateSmsProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateSmsProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/ActivateSMSProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ActivateSMSProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_sms_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateSmsProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateSmsProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/DeactivateSMSProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "DeactivateSMSProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_sms_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveSmsProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveSmsProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveSMSProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "RemoveSMSProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_oidc_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOidcSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOidcSettingsResponse>,
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
                "/zitadel.admin.v1.AdminService/GetOIDCSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetOIDCSettings"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_oidc_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOidcSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOidcSettingsResponse>,
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
                "/zitadel.admin.v1.AdminService/AddOIDCSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddOIDCSettings"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_oidc_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOidcSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOidcSettingsResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateOIDCSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateOIDCSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_file_system_notification_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetFileSystemNotificationProviderRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetFileSystemNotificationProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/GetFileSystemNotificationProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetFileSystemNotificationProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_log_notification_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLogNotificationProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLogNotificationProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/GetLogNotificationProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetLogNotificationProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_security_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecurityPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSecurityPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetSecurityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetSecurityPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_security_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetSecurityPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetSecurityPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/SetSecurityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "SetSecurityPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_org_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrgByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrgByIdResponse>,
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
                "/zitadel.admin.v1.AdminService/GetOrgByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "GetOrgByID"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_org_unique(
            &mut self,
            request: impl tonic::IntoRequest<super::IsOrgUniqueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsOrgUniqueResponse>,
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
                "/zitadel.admin.v1.AdminService/IsOrgUnique",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "IsOrgUnique"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_org(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDefaultOrgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultOrgResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "SetDefaultOrg"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_org(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultOrgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultOrgResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetDefaultOrg"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_orgs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrgsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListOrgs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "ListOrgs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_up_org(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUpOrgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetUpOrgResponse>,
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
                "/zitadel.admin.v1.AdminService/SetUpOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "SetUpOrg"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_org(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOrgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOrgResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "RemoveOrg"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_idp_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIdpByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetIdpByIdResponse>,
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
                "/zitadel.admin.v1.AdminService/GetIDPByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "GetIDPByID"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_id_ps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIdPsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListIdPsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListIDPs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "ListIDPs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_oidcidp(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOidcidpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOidcidpResponse>,
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
                "/zitadel.admin.v1.AdminService/AddOIDCIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "AddOIDCIDP"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_jwtidp(
            &mut self,
            request: impl tonic::IntoRequest<super::AddJwtidpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddJwtidpResponse>,
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
                "/zitadel.admin.v1.AdminService/AddJWTIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "AddJWTIDP"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateIdpResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "UpdateIDP"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateIdpResponse>,
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
                "/zitadel.admin.v1.AdminService/DeactivateIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "DeactivateIDP"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateIdpResponse>,
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
                "/zitadel.admin.v1.AdminService/ReactivateIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "ReactivateIDP"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveIdpResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "RemoveIDP"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_idpoidc_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIdpoidcConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateIdpoidcConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateIDPOIDCConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateIDPOIDCConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_idpjwt_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIdpjwtConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateIdpjwtConfigResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateIDPJWTConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateIDPJWTConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProvidersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProvidersResponse>,
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
                "/zitadel.admin.v1.AdminService/ListProviders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "ListProviders"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_provider_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProviderByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProviderByIdResponse>,
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
                "/zitadel.admin.v1.AdminService/GetProviderByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetProviderByID"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_generic_o_auth_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGenericOAuthProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddGenericOAuthProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddGenericOAuthProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddGenericOAuthProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_generic_o_auth_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGenericOAuthProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGenericOAuthProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateGenericOAuthProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateGenericOAuthProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_generic_oidc_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGenericOidcProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddGenericOidcProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddGenericOIDCProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddGenericOIDCProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_generic_oidc_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGenericOidcProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGenericOidcProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateGenericOIDCProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateGenericOIDCProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn migrate_generic_oidc_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::MigrateGenericOidcProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MigrateGenericOidcProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/MigrateGenericOIDCProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "MigrateGenericOIDCProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_jwt_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddJwtProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddJwtProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddJWTProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddJWTProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_jwt_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJwtProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateJwtProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateJWTProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "UpdateJWTProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_azure_ad_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddAzureAdProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddAzureAdProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddAzureADProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddAzureADProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_azure_ad_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAzureAdProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAzureAdProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateAzureADProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateAzureADProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_git_hub_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGitHubProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddGitHubProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddGitHubProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddGitHubProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_git_hub_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGitHubProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGitHubProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateGitHubProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateGitHubProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_git_hub_enterprise_server_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::AddGitHubEnterpriseServerProviderRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::AddGitHubEnterpriseServerProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddGitHubEnterpriseServerProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddGitHubEnterpriseServerProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_git_hub_enterprise_server_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateGitHubEnterpriseServerProviderRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGitHubEnterpriseServerProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateGitHubEnterpriseServerProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateGitHubEnterpriseServerProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_git_lab_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGitLabProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddGitLabProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddGitLabProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddGitLabProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_git_lab_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGitLabProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGitLabProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateGitLabProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateGitLabProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_git_lab_self_hosted_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGitLabSelfHostedProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddGitLabSelfHostedProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddGitLabSelfHostedProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddGitLabSelfHostedProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_git_lab_self_hosted_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateGitLabSelfHostedProviderRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGitLabSelfHostedProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateGitLabSelfHostedProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateGitLabSelfHostedProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_google_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGoogleProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddGoogleProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddGoogleProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddGoogleProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_google_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoogleProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateGoogleProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateGoogleProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateGoogleProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_ldap_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddLdapProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddLdapProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddLDAPProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddLDAPProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ldap_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLdapProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateLdapProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateLDAPProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateLDAPProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_apple_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddAppleProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddAppleProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddAppleProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddAppleProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_apple_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAppleProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAppleProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateAppleProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateAppleProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_saml_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSamlProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddSamlProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/AddSAMLProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddSAMLProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_saml_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSamlProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSamlProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateSAMLProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateSAMLProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn regenerate_saml_provider_certificate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RegenerateSamlProviderCertificateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RegenerateSamlProviderCertificateResponse>,
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
                "/zitadel.admin.v1.AdminService/RegenerateSAMLProviderCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RegenerateSAMLProviderCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteProviderResponse>,
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
                "/zitadel.admin.v1.AdminService/DeleteProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "DeleteProvider"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_org_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrgIamPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrgIamPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetOrgIAMPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetOrgIAMPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_org_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrgIamPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrgIamPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateOrgIAMPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateOrgIAMPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_org_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomOrgIamPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomOrgIamPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomOrgIAMPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomOrgIAMPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_org_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomOrgIamPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomOrgIamPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/AddCustomOrgIAMPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddCustomOrgIAMPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_org_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomOrgIamPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomOrgIamPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateCustomOrgIAMPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateCustomOrgIAMPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_org_iam_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomOrgIamPolicyToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomOrgIamPolicyToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomOrgIAMPolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomOrgIAMPolicyToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_domain_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDomainPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDomainPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDomainPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetDomainPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_domain_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDomainPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDomainPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateDomainPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateDomainPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_domain_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomDomainPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomDomainPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomDomainPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomDomainPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_domain_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomDomainPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomDomainPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/AddCustomDomainPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddCustomDomainPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_domain_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomDomainPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomDomainPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateCustomDomainPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateCustomDomainPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_domain_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomDomainPolicyToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomDomainPolicyToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomDomainPolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomDomainPolicyToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLabelPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetLabelPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_preview_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPreviewLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPreviewLabelPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetPreviewLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetPreviewLabelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateLabelPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "UpdateLabelPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateLabelPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/ActivateLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ActivateLabelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_label_policy_logo(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveLabelPolicyLogoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveLabelPolicyLogoResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveLabelPolicyLogo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveLabelPolicyLogo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_label_policy_logo_dark(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveLabelPolicyLogoDarkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveLabelPolicyLogoDarkResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveLabelPolicyLogoDark",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveLabelPolicyLogoDark",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_label_policy_icon(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveLabelPolicyIconRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveLabelPolicyIconResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveLabelPolicyIcon",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveLabelPolicyIcon",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_label_policy_icon_dark(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveLabelPolicyIconDarkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveLabelPolicyIconDarkResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveLabelPolicyIconDark",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveLabelPolicyIconDark",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_label_policy_font(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveLabelPolicyFontRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveLabelPolicyFontResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveLabelPolicyFont",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveLabelPolicyFont",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLoginPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetLoginPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateLoginPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "UpdateLoginPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_login_policy_id_ps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLoginPolicyIdPsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLoginPolicyIdPsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListLoginPolicyIDPs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListLoginPolicyIDPs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_idp_to_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddIdpToLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddIdpToLoginPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/AddIDPToLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddIDPToLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_idp_from_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveIdpFromLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveIdpFromLoginPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveIDPFromLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveIDPFromLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_login_policy_second_factors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLoginPolicySecondFactorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLoginPolicySecondFactorsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListLoginPolicySecondFactors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListLoginPolicySecondFactors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_second_factor_to_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSecondFactorToLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddSecondFactorToLoginPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/AddSecondFactorToLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddSecondFactorToLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_second_factor_from_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveSecondFactorFromLoginPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RemoveSecondFactorFromLoginPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveSecondFactorFromLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveSecondFactorFromLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_login_policy_multi_factors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLoginPolicyMultiFactorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLoginPolicyMultiFactorsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListLoginPolicyMultiFactors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListLoginPolicyMultiFactors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_multi_factor_to_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMultiFactorToLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMultiFactorToLoginPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/AddMultiFactorToLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddMultiFactorToLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_multi_factor_from_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveMultiFactorFromLoginPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMultiFactorFromLoginPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveMultiFactorFromLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "RemoveMultiFactorFromLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_password_complexity_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPasswordComplexityPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPasswordComplexityPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetPasswordComplexityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetPasswordComplexityPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_password_complexity_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdatePasswordComplexityPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePasswordComplexityPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdatePasswordComplexityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdatePasswordComplexityPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_password_age_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPasswordAgePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPasswordAgePolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetPasswordAgePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetPasswordAgePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_password_age_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePasswordAgePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePasswordAgePolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdatePasswordAgePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdatePasswordAgePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_lockout_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLockoutPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLockoutPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetLockoutPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetLockoutPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_lockout_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLockoutPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateLockoutPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateLockoutPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateLockoutPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_privacy_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPrivacyPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPrivacyPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetPrivacyPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetPrivacyPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_privacy_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePrivacyPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePrivacyPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdatePrivacyPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdatePrivacyPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_notification_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddNotificationPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddNotificationPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/AddNotificationPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "AddNotificationPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_notification_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNotificationPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/GetNotificationPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetNotificationPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_notification_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNotificationPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateNotificationPolicyResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateNotificationPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "UpdateNotificationPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_init_message_text(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultInitMessageTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultInitMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultInitMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultInitMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_init_message_text(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomInitMessageTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomInitMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomInitMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomInitMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_init_message_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDefaultInitMessageTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultInitMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultInitMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultInitMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_init_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomInitMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomInitMessageTextToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomInitMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomInitMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_password_reset_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultPasswordResetMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultPasswordResetMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultPasswordResetMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultPasswordResetMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_password_reset_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomPasswordResetMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomPasswordResetMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomPasswordResetMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomPasswordResetMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_password_reset_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultPasswordResetMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultPasswordResetMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultPasswordResetMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultPasswordResetMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_password_reset_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomPasswordResetMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomPasswordResetMessageTextToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomPasswordResetMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomPasswordResetMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_verify_email_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultVerifyEmailMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultVerifyEmailMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultVerifyEmailMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultVerifyEmailMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_verify_email_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomVerifyEmailMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomVerifyEmailMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomVerifyEmailMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomVerifyEmailMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_verify_email_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultVerifyEmailMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultVerifyEmailMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultVerifyEmailMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultVerifyEmailMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_verify_email_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomVerifyEmailMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomVerifyEmailMessageTextToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomVerifyEmailMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomVerifyEmailMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_verify_phone_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultVerifyPhoneMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultVerifyPhoneMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultVerifyPhoneMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultVerifyPhoneMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_verify_phone_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomVerifyPhoneMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomVerifyPhoneMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomVerifyPhoneMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomVerifyPhoneMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_verify_phone_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultVerifyPhoneMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultVerifyPhoneMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultVerifyPhoneMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultVerifyPhoneMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_verify_phone_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomVerifyPhoneMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomVerifyPhoneMessageTextToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomVerifyPhoneMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomVerifyPhoneMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_verify_smsotp_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultVerifySmsotpMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultVerifySmsotpMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultVerifySMSOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultVerifySMSOTPMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_verify_smsotp_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomVerifySmsotpMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomVerifySmsotpMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomVerifySMSOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomVerifySMSOTPMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_verify_smsotp_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultVerifySmsotpMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultVerifySmsotpMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultVerifySMSOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultVerifySMSOTPMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_verify_smsotp_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomVerifySmsotpMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomVerifySmsotpMessageTextToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomVerifySMSOTPMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomVerifySMSOTPMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_verify_email_otp_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultVerifyEmailOtpMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultVerifyEmailOtpMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultVerifyEmailOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultVerifyEmailOTPMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_verify_email_otp_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomVerifyEmailOtpMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomVerifyEmailOtpMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomVerifyEmailOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomVerifyEmailOTPMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_verify_email_otp_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultVerifyEmailOtpMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultVerifyEmailOtpMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultVerifyEmailOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultVerifyEmailOTPMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_verify_email_otp_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomVerifyEmailOtpMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::ResetCustomVerifyEmailOtpMessageTextToDefaultResponse,
            >,
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
                "/zitadel.admin.v1.AdminService/ResetCustomVerifyEmailOTPMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomVerifyEmailOTPMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_domain_claimed_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultDomainClaimedMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultDomainClaimedMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultDomainClaimedMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultDomainClaimedMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_domain_claimed_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomDomainClaimedMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomDomainClaimedMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomDomainClaimedMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomDomainClaimedMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_domain_claimed_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultDomainClaimedMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultDomainClaimedMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultDomainClaimedMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultDomainClaimedMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_domain_claimed_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomDomainClaimedMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomDomainClaimedMessageTextToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomDomainClaimedMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomDomainClaimedMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_passwordless_registration_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultPasswordlessRegistrationMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::GetDefaultPasswordlessRegistrationMessageTextResponse,
            >,
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
                "/zitadel.admin.v1.AdminService/GetDefaultPasswordlessRegistrationMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultPasswordlessRegistrationMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_passwordless_registration_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomPasswordlessRegistrationMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomPasswordlessRegistrationMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomPasswordlessRegistrationMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomPasswordlessRegistrationMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_passwordless_registration_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultPasswordlessRegistrationMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::SetDefaultPasswordlessRegistrationMessageTextResponse,
            >,
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
                "/zitadel.admin.v1.AdminService/SetDefaultPasswordlessRegistrationMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultPasswordlessRegistrationMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_passwordless_registration_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomPasswordlessRegistrationMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::ResetCustomPasswordlessRegistrationMessageTextToDefaultResponse,
            >,
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
                "/zitadel.admin.v1.AdminService/ResetCustomPasswordlessRegistrationMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomPasswordlessRegistrationMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_password_change_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultPasswordChangeMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultPasswordChangeMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultPasswordChangeMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultPasswordChangeMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_password_change_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomPasswordChangeMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomPasswordChangeMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomPasswordChangeMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomPasswordChangeMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_password_change_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultPasswordChangeMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultPasswordChangeMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultPasswordChangeMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultPasswordChangeMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_password_change_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomPasswordChangeMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::ResetCustomPasswordChangeMessageTextToDefaultResponse,
            >,
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
                "/zitadel.admin.v1.AdminService/ResetCustomPasswordChangeMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomPasswordChangeMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_invite_user_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultInviteUserMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultInviteUserMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultInviteUserMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultInviteUserMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_invite_user_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetCustomInviteUserMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomInviteUserMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomInviteUserMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomInviteUserMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_default_invite_user_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetDefaultInviteUserMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetDefaultInviteUserMessageTextResponse>,
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
                "/zitadel.admin.v1.AdminService/SetDefaultInviteUserMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetDefaultInviteUserMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_invite_user_message_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomInviteUserMessageTextToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomInviteUserMessageTextToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomInviteUserMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomInviteUserMessageTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_login_texts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultLoginTextsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultLoginTextsResponse>,
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
                "/zitadel.admin.v1.AdminService/GetDefaultLoginTexts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetDefaultLoginTexts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_login_texts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomLoginTextsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomLoginTextsResponse>,
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
                "/zitadel.admin.v1.AdminService/GetCustomLoginTexts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "GetCustomLoginTexts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_login_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCustomLoginTextsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomLoginTextsResponse>,
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
                "/zitadel.admin.v1.AdminService/SetCustomLoginText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "SetCustomLoginText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_custom_login_text_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetCustomLoginTextsToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetCustomLoginTextsToDefaultResponse>,
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
                "/zitadel.admin.v1.AdminService/ResetCustomLoginTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ResetCustomLoginTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_iam_member_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIamMemberRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListIamMemberRolesResponse>,
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
                "/zitadel.admin.v1.AdminService/ListIAMMemberRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListIAMMemberRoles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_iam_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIamMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListIamMembersResponse>,
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
                "/zitadel.admin.v1.AdminService/ListIAMMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "ListIAMMembers"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_iam_member(
            &mut self,
            request: impl tonic::IntoRequest<super::AddIamMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddIamMemberResponse>,
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
                "/zitadel.admin.v1.AdminService/AddIAMMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "AddIAMMember"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_iam_member(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIamMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateIamMemberResponse>,
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
                "/zitadel.admin.v1.AdminService/UpdateIAMMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "UpdateIAMMember"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_iam_member(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveIamMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveIamMemberResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveIAMMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "RemoveIAMMember"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_views(
            &mut self,
            request: impl tonic::IntoRequest<super::ListViewsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListViewsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListViews",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "ListViews"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_failed_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFailedEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFailedEventsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListFailedEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "ListFailedEvents"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_failed_event(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFailedEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFailedEventResponse>,
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
                "/zitadel.admin.v1.AdminService/RemoveFailedEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "RemoveFailedEvent"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn import_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportDataResponse>,
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
                "/zitadel.admin.v1.AdminService/ImportData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "ImportData"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn export_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportDataResponse>,
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
                "/zitadel.admin.v1.AdminService/ExportData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "ExportData"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_event_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEventTypesResponse>,
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
                "/zitadel.admin.v1.AdminService/ListEventTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "ListEventTypes"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEventsResponse>,
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
                "/zitadel.admin.v1.AdminService/ListEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.admin.v1.AdminService", "ListEvents"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_aggregate_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAggregateTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAggregateTypesResponse>,
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
                "/zitadel.admin.v1.AdminService/ListAggregateTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ListAggregateTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate_feature_login_default_org(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ActivateFeatureLoginDefaultOrgRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ActivateFeatureLoginDefaultOrgResponse>,
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
                "/zitadel.admin.v1.AdminService/ActivateFeatureLoginDefaultOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.admin.v1.AdminService",
                        "ActivateFeatureLoginDefaultOrg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_milestones(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMilestonesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMilestonesResponse>,
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
                "/zitadel.admin.v1.AdminService/ListMilestones",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "ListMilestones"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_restrictions(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRestrictionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetRestrictionsResponse>,
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
                "/zitadel.admin.v1.AdminService/SetRestrictions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "SetRestrictions"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_restrictions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRestrictionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRestrictionsResponse>,
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
                "/zitadel.admin.v1.AdminService/GetRestrictions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.admin.v1.AdminService", "GetRestrictions"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
