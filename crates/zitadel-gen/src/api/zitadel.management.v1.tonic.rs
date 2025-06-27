// @generated
/// Generated client implementations.
pub mod management_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ManagementServiceClient<tonic::transport::Channel> {
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
    impl<T> ManagementServiceClient<T>
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
        ) -> ManagementServiceClient<InterceptedService<T, F>>
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
            ManagementServiceClient::new(InterceptedService::new(inner, interceptor))
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
                "/zitadel.management.v1.ManagementService/Healthz",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.management.v1.ManagementService", "Healthz"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_oidc_information(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOidcInformationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOidcInformationResponse>,
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
                "/zitadel.management.v1.ManagementService/GetOIDCInformation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetOIDCInformation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_iam(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIamRequest>,
        ) -> std::result::Result<tonic::Response<super::GetIamResponse>, tonic::Status> {
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
                "/zitadel.management.v1.ManagementService/GetIAM",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.management.v1.ManagementService", "GetIAM"),
                );
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
                "/zitadel.management.v1.ManagementService/GetSupportedLanguages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetSupportedLanguages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserByIdResponse>,
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
                "/zitadel.management.v1.ManagementService/GetUserByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetUserByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_by_login_name_global(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserByLoginNameGlobalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserByLoginNameGlobalResponse>,
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
                "/zitadel.management.v1.ManagementService/GetUserByLoginNameGlobal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetUserByLoginNameGlobal",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_users(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsersResponse>,
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
                "/zitadel.management.v1.ManagementService/ListUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListUsers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserChangesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListUserChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListUserChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_user_unique(
            &mut self,
            request: impl tonic::IntoRequest<super::IsUserUniqueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsUserUniqueResponse>,
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
                "/zitadel.management.v1.ManagementService/IsUserUnique",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "IsUserUnique",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_human_user(
            &mut self,
            request: impl tonic::IntoRequest<super::AddHumanUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddHumanUserResponse>,
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
                "/zitadel.management.v1.ManagementService/AddHumanUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddHumanUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn import_human_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportHumanUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportHumanUserResponse>,
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
                "/zitadel.management.v1.ManagementService/ImportHumanUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ImportHumanUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_machine_user(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMachineUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMachineUserResponse>,
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
                "/zitadel.management.v1.ManagementService/AddMachineUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddMachineUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateUserResponse>,
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
                "/zitadel.management.v1.ManagementService/DeactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeactivateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateUserResponse>,
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
                "/zitadel.management.v1.ManagementService/ReactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ReactivateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn lock_user(
            &mut self,
            request: impl tonic::IntoRequest<super::LockUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LockUserResponse>,
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
                "/zitadel.management.v1.ManagementService/LockUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "LockUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlockUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnlockUserResponse>,
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
                "/zitadel.management.v1.ManagementService/UnlockUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UnlockUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_user(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveUserResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_user_name(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserNameResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateUserName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateUserName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_user_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUserMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetUserMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/SetUserMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetUserMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn bulk_set_user_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkSetUserMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkSetUserMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/BulkSetUserMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "BulkSetUserMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/ListUserMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListUserMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/GetUserMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetUserMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_user_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveUserMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveUserMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveUserMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveUserMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn bulk_remove_user_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkRemoveUserMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkRemoveUserMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/BulkRemoveUserMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "BulkRemoveUserMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_human_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHumanProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHumanProfileResponse>,
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
                "/zitadel.management.v1.ManagementService/GetHumanProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetHumanProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_human_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHumanProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateHumanProfileResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateHumanProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateHumanProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_human_email(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHumanEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHumanEmailResponse>,
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
                "/zitadel.management.v1.ManagementService/GetHumanEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetHumanEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_human_email(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHumanEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateHumanEmailResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateHumanEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateHumanEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_human_initialization(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendHumanInitializationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendHumanInitializationResponse>,
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
                "/zitadel.management.v1.ManagementService/ResendHumanInitialization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResendHumanInitialization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_human_email_verification(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendHumanEmailVerificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendHumanEmailVerificationResponse>,
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
                "/zitadel.management.v1.ManagementService/ResendHumanEmailVerification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResendHumanEmailVerification",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_human_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHumanPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHumanPhoneResponse>,
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
                "/zitadel.management.v1.ManagementService/GetHumanPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetHumanPhone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_human_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHumanPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateHumanPhoneResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateHumanPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateHumanPhone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_human_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveHumanPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveHumanPhoneResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveHumanPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveHumanPhone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_human_phone_verification(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendHumanPhoneVerificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendHumanPhoneVerificationResponse>,
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
                "/zitadel.management.v1.ManagementService/ResendHumanPhoneVerification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResendHumanPhoneVerification",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_human_avatar(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveHumanAvatarRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveHumanAvatarResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveHumanAvatar",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveHumanAvatar",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_human_initial_password(
            &mut self,
            request: impl tonic::IntoRequest<super::SetHumanInitialPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetHumanInitialPasswordResponse>,
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
                "/zitadel.management.v1.ManagementService/SetHumanInitialPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetHumanInitialPassword",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_human_password(
            &mut self,
            request: impl tonic::IntoRequest<super::SetHumanPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetHumanPasswordResponse>,
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
                "/zitadel.management.v1.ManagementService/SetHumanPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetHumanPassword",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_human_reset_password_notification(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SendHumanResetPasswordNotificationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SendHumanResetPasswordNotificationResponse>,
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
                "/zitadel.management.v1.ManagementService/SendHumanResetPasswordNotification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SendHumanResetPasswordNotification",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_human_auth_factors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHumanAuthFactorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHumanAuthFactorsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListHumanAuthFactors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListHumanAuthFactors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_human_auth_factor_otp(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveHumanAuthFactorOtpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveHumanAuthFactorOtpResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveHumanAuthFactorOTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveHumanAuthFactorOTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_human_auth_factor_u2f(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveHumanAuthFactorU2fRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveHumanAuthFactorU2fResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveHumanAuthFactorU2F",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveHumanAuthFactorU2F",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_human_auth_factor_otpsms(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveHumanAuthFactorOtpsmsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveHumanAuthFactorOtpsmsResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveHumanAuthFactorOTPSMS",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveHumanAuthFactorOTPSMS",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_human_auth_factor_otp_email(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveHumanAuthFactorOtpEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveHumanAuthFactorOtpEmailResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveHumanAuthFactorOTPEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveHumanAuthFactorOTPEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_human_passwordless(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHumanPasswordlessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHumanPasswordlessResponse>,
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
                "/zitadel.management.v1.ManagementService/ListHumanPasswordless",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListHumanPasswordless",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_passwordless_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPasswordlessRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddPasswordlessRegistrationResponse>,
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
                "/zitadel.management.v1.ManagementService/AddPasswordlessRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddPasswordlessRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_passwordless_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::SendPasswordlessRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendPasswordlessRegistrationResponse>,
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
                "/zitadel.management.v1.ManagementService/SendPasswordlessRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SendPasswordlessRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_human_passwordless(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveHumanPasswordlessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveHumanPasswordlessResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveHumanPasswordless",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveHumanPasswordless",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_machine(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMachineRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMachineResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateMachine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateMachine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_machine_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateMachineSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateMachineSecretResponse>,
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
                "/zitadel.management.v1.ManagementService/GenerateMachineSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GenerateMachineSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_machine_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMachineSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMachineSecretResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveMachineSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveMachineSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_machine_key_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMachineKeyByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMachineKeyByIDsResponse>,
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
                "/zitadel.management.v1.ManagementService/GetMachineKeyByIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetMachineKeyByIDs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_machine_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMachineKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMachineKeysResponse>,
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
                "/zitadel.management.v1.ManagementService/ListMachineKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListMachineKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_machine_key(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMachineKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMachineKeyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddMachineKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddMachineKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_machine_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMachineKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMachineKeyResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveMachineKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveMachineKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_personal_access_token_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPersonalAccessTokenByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPersonalAccessTokenByIDsResponse>,
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
                "/zitadel.management.v1.ManagementService/GetPersonalAccessTokenByIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetPersonalAccessTokenByIDs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_personal_access_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPersonalAccessTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPersonalAccessTokensResponse>,
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
                "/zitadel.management.v1.ManagementService/ListPersonalAccessTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListPersonalAccessTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_personal_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPersonalAccessTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddPersonalAccessTokenResponse>,
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
                "/zitadel.management.v1.ManagementService/AddPersonalAccessToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddPersonalAccessToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_personal_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePersonalAccessTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePersonalAccessTokenResponse>,
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
                "/zitadel.management.v1.ManagementService/RemovePersonalAccessToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemovePersonalAccessToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_human_linked_id_ps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHumanLinkedIdPsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHumanLinkedIdPsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListHumanLinkedIDPs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListHumanLinkedIDPs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_human_linked_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveHumanLinkedIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveHumanLinkedIdpResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveHumanLinkedIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveHumanLinkedIDP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_memberships(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserMembershipsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserMembershipsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListUserMemberships",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListUserMemberships",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_org(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyOrgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyOrgResponse>,
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
                "/zitadel.management.v1.ManagementService/GetMyOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetMyOrg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_org_by_domain_global(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrgByDomainGlobalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrgByDomainGlobalResponse>,
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
                "/zitadel.management.v1.ManagementService/GetOrgByDomainGlobal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetOrgByDomainGlobal",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_org_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrgChangesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListOrgChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListOrgChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_org(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOrgRequest>,
        ) -> std::result::Result<tonic::Response<super::AddOrgResponse>, tonic::Status> {
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
                "/zitadel.management.v1.ManagementService/AddOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.management.v1.ManagementService", "AddOrg"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_org(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrgResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateOrg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_org(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateOrgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateOrgResponse>,
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
                "/zitadel.management.v1.ManagementService/DeactivateOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeactivateOrg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_org(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateOrgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateOrgResponse>,
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
                "/zitadel.management.v1.ManagementService/ReactivateOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ReactivateOrg",
                    ),
                );
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
                "/zitadel.management.v1.ManagementService/RemoveOrg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveOrg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_org_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::SetOrgMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetOrgMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/SetOrgMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetOrgMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn bulk_set_org_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkSetOrgMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkSetOrgMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/BulkSetOrgMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "BulkSetOrgMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_org_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrgMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/ListOrgMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListOrgMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_org_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrgMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrgMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/GetOrgMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetOrgMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_org_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOrgMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOrgMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveOrgMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveOrgMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn bulk_remove_org_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkRemoveOrgMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkRemoveOrgMetadataResponse>,
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
                "/zitadel.management.v1.ManagementService/BulkRemoveOrgMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "BulkRemoveOrgMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_org_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgDomainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrgDomainsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListOrgDomains",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListOrgDomains",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_org_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOrgDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOrgDomainResponse>,
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
                "/zitadel.management.v1.ManagementService/AddOrgDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddOrgDomain",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_org_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOrgDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOrgDomainResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveOrgDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveOrgDomain",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_org_domain_validation(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateOrgDomainValidationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateOrgDomainValidationResponse>,
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
                "/zitadel.management.v1.ManagementService/GenerateOrgDomainValidation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GenerateOrgDomainValidation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_org_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateOrgDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateOrgDomainResponse>,
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
                "/zitadel.management.v1.ManagementService/ValidateOrgDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ValidateOrgDomain",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_primary_org_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPrimaryOrgDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetPrimaryOrgDomainResponse>,
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
                "/zitadel.management.v1.ManagementService/SetPrimaryOrgDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetPrimaryOrgDomain",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_org_member_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgMemberRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrgMemberRolesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListOrgMemberRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListOrgMemberRoles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_org_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrgMembersResponse>,
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
                "/zitadel.management.v1.ManagementService/ListOrgMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListOrgMembers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_org_member(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOrgMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOrgMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/AddOrgMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddOrgMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_org_member(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrgMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrgMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateOrgMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateOrgMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_org_member(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOrgMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOrgMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveOrgMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveOrgMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_project_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProjectByIdResponse>,
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
                "/zitadel.management.v1.ManagementService/GetProjectByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetProjectByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_granted_project_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGrantedProjectByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGrantedProjectByIdResponse>,
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
                "/zitadel.management.v1.ManagementService/GetGrantedProjectByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetGrantedProjectByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjects",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_granted_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGrantedProjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGrantedProjectsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListGrantedProjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListGrantedProjects",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_granted_project_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGrantedProjectRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGrantedProjectRolesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListGrantedProjectRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListGrantedProjectRoles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectChangesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjectChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjectChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_project(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProjectResponse>,
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
                "/zitadel.management.v1.ManagementService/AddProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_project(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateProjectResponse>,
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
                "/zitadel.management.v1.ManagementService/DeactivateProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeactivateProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_project(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateProjectResponse>,
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
                "/zitadel.management.v1.ManagementService/ReactivateProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ReactivateProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_project(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveProjectResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectRolesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjectRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjectRoles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_project_role(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProjectRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProjectRoleResponse>,
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
                "/zitadel.management.v1.ManagementService/AddProjectRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddProjectRole",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn bulk_add_project_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkAddProjectRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkAddProjectRolesResponse>,
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
                "/zitadel.management.v1.ManagementService/BulkAddProjectRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "BulkAddProjectRoles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_project_role(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectRoleResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateProjectRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateProjectRole",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_project_role(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveProjectRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveProjectRoleResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveProjectRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveProjectRole",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_member_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectMemberRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectMemberRolesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjectMemberRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjectMemberRoles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectMembersResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjectMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjectMembers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_project_member(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProjectMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProjectMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/AddProjectMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddProjectMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_project_member(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateProjectMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateProjectMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_project_member(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveProjectMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveProjectMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveProjectMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveProjectMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_app_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAppByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAppByIdResponse>,
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
                "/zitadel.management.v1.ManagementService/GetAppByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetAppByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_apps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAppsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListApps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListApps",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_app_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAppChangesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListAppChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListAppChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_oidc_app(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOidcAppRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOidcAppResponse>,
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
                "/zitadel.management.v1.ManagementService/AddOIDCApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddOIDCApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_saml_app(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSamlAppRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddSamlAppResponse>,
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
                "/zitadel.management.v1.ManagementService/AddSAMLApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddSAMLApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_api_app(
            &mut self,
            request: impl tonic::IntoRequest<super::AddApiAppRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddApiAppResponse>,
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
                "/zitadel.management.v1.ManagementService/AddAPIApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddAPIApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_app(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAppRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAppResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_oidc_app_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOidcAppConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOidcAppConfigResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateOIDCAppConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateOIDCAppConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_saml_app_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSamlAppConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSamlAppConfigResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateSAMLAppConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateSAMLAppConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_api_app_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApiAppConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateApiAppConfigResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateAPIAppConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateAPIAppConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_app(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateAppRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateAppResponse>,
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
                "/zitadel.management.v1.ManagementService/DeactivateApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeactivateApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_app(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateAppRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateAppResponse>,
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
                "/zitadel.management.v1.ManagementService/ReactivateApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ReactivateApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_app(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveAppRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveAppResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn regenerate_oidc_client_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::RegenerateOidcClientSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegenerateOidcClientSecretResponse>,
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
                "/zitadel.management.v1.ManagementService/RegenerateOIDCClientSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RegenerateOIDCClientSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn regenerate_api_client_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::RegenerateApiClientSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegenerateApiClientSecretResponse>,
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
                "/zitadel.management.v1.ManagementService/RegenerateAPIClientSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RegenerateAPIClientSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_app_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAppKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAppKeyResponse>,
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
                "/zitadel.management.v1.ManagementService/GetAppKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetAppKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_app_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAppKeysResponse>,
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
                "/zitadel.management.v1.ManagementService/ListAppKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListAppKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_app_key(
            &mut self,
            request: impl tonic::IntoRequest<super::AddAppKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddAppKeyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddAppKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddAppKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_app_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveAppKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveAppKeyResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveAppKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveAppKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_grant_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectGrantChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectGrantChangesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjectGrantChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjectGrantChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_project_grant_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectGrantByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProjectGrantByIdResponse>,
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
                "/zitadel.management.v1.ManagementService/GetProjectGrantByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetProjectGrantByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_grants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectGrantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectGrantsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjectGrants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjectGrants",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_all_project_grants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAllProjectGrantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAllProjectGrantsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListAllProjectGrants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListAllProjectGrants",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_project_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProjectGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProjectGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/AddProjectGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddProjectGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_project_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateProjectGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateProjectGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_project_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateProjectGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateProjectGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/DeactivateProjectGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeactivateProjectGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_project_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateProjectGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateProjectGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/ReactivateProjectGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ReactivateProjectGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_project_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveProjectGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveProjectGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveProjectGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveProjectGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_grant_member_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectGrantMemberRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectGrantMemberRolesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjectGrantMemberRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjectGrantMemberRoles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_grant_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectGrantMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectGrantMembersResponse>,
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
                "/zitadel.management.v1.ManagementService/ListProjectGrantMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProjectGrantMembers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_project_grant_member(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProjectGrantMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProjectGrantMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/AddProjectGrantMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddProjectGrantMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_project_grant_member(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectGrantMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectGrantMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateProjectGrantMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateProjectGrantMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_project_grant_member(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveProjectGrantMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveProjectGrantMemberResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveProjectGrantMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveProjectGrantMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_grant_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserGrantByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserGrantByIdResponse>,
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
                "/zitadel.management.v1.ManagementService/GetUserGrantByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetUserGrantByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_grants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/ListUserGrants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListUserGrants",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_user_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUserGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddUserGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/AddUserGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddUserGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_user_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateUserGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateUserGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_user_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateUserGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateUserGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/DeactivateUserGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeactivateUserGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_user_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateUserGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateUserGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/ReactivateUserGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ReactivateUserGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_user_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveUserGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveUserGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveUserGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveUserGrant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn bulk_remove_user_grant(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkRemoveUserGrantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkRemoveUserGrantResponse>,
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
                "/zitadel.management.v1.ManagementService/BulkRemoveUserGrant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "BulkRemoveUserGrant",
                    ),
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
                "/zitadel.management.v1.ManagementService/GetOrgIAMPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetOrgIAMPolicy",
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
                "/zitadel.management.v1.ManagementService/GetDomainPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDomainPolicy",
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
                "/zitadel.management.v1.ManagementService/GetLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultLoginPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/GetDefaultLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomLoginPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddCustomLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddCustomLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomLoginPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateCustomLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateCustomLoginPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_login_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetLoginPolicyToDefaultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResetLoginPolicyToDefaultResponse>,
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
                "/zitadel.management.v1.ManagementService/ResetLoginPolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetLoginPolicyToDefault",
                    ),
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
                "/zitadel.management.v1.ManagementService/ListLoginPolicyIDPs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddIDPToLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/RemoveIDPFromLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/ListLoginPolicySecondFactors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddSecondFactorToLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/RemoveSecondFactorFromLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/ListLoginPolicyMultiFactors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddMultiFactorToLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/RemoveMultiFactorFromLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/GetPasswordComplexityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetPasswordComplexityPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_password_complexity_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDefaultPasswordComplexityPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultPasswordComplexityPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/GetDefaultPasswordComplexityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultPasswordComplexityPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_password_complexity_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::AddCustomPasswordComplexityPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomPasswordComplexityPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddCustomPasswordComplexityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddCustomPasswordComplexityPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_password_complexity_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateCustomPasswordComplexityPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomPasswordComplexityPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateCustomPasswordComplexityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateCustomPasswordComplexityPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_password_complexity_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetPasswordComplexityPolicyToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetPasswordComplexityPolicyToDefaultResponse>,
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
                "/zitadel.management.v1.ManagementService/ResetPasswordComplexityPolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetPasswordComplexityPolicyToDefault",
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
                "/zitadel.management.v1.ManagementService/GetPasswordAgePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetPasswordAgePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_password_age_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultPasswordAgePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultPasswordAgePolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/GetDefaultPasswordAgePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultPasswordAgePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_password_age_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomPasswordAgePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomPasswordAgePolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddCustomPasswordAgePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddCustomPasswordAgePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_password_age_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomPasswordAgePolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomPasswordAgePolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateCustomPasswordAgePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateCustomPasswordAgePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_password_age_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetPasswordAgePolicyToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetPasswordAgePolicyToDefaultResponse>,
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
                "/zitadel.management.v1.ManagementService/ResetPasswordAgePolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetPasswordAgePolicyToDefault",
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
                "/zitadel.management.v1.ManagementService/GetLockoutPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetLockoutPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_lockout_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultLockoutPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultLockoutPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/GetDefaultLockoutPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultLockoutPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_lockout_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomLockoutPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomLockoutPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddCustomLockoutPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddCustomLockoutPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_lockout_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomLockoutPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomLockoutPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateCustomLockoutPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateCustomLockoutPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_lockout_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetLockoutPolicyToDefaultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResetLockoutPolicyToDefaultResponse>,
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
                "/zitadel.management.v1.ManagementService/ResetLockoutPolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetLockoutPolicyToDefault",
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
                "/zitadel.management.v1.ManagementService/GetPrivacyPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetPrivacyPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_privacy_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultPrivacyPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultPrivacyPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/GetDefaultPrivacyPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultPrivacyPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_privacy_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomPrivacyPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomPrivacyPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddCustomPrivacyPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddCustomPrivacyPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_privacy_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomPrivacyPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomPrivacyPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateCustomPrivacyPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateCustomPrivacyPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_privacy_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetPrivacyPolicyToDefaultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResetPrivacyPolicyToDefaultResponse>,
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
                "/zitadel.management.v1.ManagementService/ResetPrivacyPolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetPrivacyPolicyToDefault",
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
                "/zitadel.management.v1.ManagementService/GetNotificationPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetNotificationPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_notification_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultNotificationPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultNotificationPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/GetDefaultNotificationPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultNotificationPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_notification_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomNotificationPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomNotificationPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddCustomNotificationPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddCustomNotificationPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_notification_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateCustomNotificationPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomNotificationPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateCustomNotificationPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateCustomNotificationPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_notification_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResetNotificationPolicyToDefaultRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ResetNotificationPolicyToDefaultResponse>,
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
                "/zitadel.management.v1.ManagementService/ResetNotificationPolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetNotificationPolicyToDefault",
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
                "/zitadel.management.v1.ManagementService/GetLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetLabelPolicy",
                    ),
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
                "/zitadel.management.v1.ManagementService/GetPreviewLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetPreviewLabelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_default_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultLabelPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/GetDefaultLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultLabelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_custom_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCustomLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCustomLabelPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/AddCustomLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddCustomLabelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_custom_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomLabelPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateCustomLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateCustomLabelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate_custom_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateCustomLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateCustomLabelPolicyResponse>,
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
                "/zitadel.management.v1.ManagementService/ActivateCustomLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ActivateCustomLabelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_custom_label_policy_logo(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveCustomLabelPolicyLogoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveCustomLabelPolicyLogoResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveCustomLabelPolicyLogo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveCustomLabelPolicyLogo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_custom_label_policy_logo_dark(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveCustomLabelPolicyLogoDarkRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RemoveCustomLabelPolicyLogoDarkResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveCustomLabelPolicyLogoDark",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveCustomLabelPolicyLogoDark",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_custom_label_policy_icon(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveCustomLabelPolicyIconRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveCustomLabelPolicyIconResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveCustomLabelPolicyIcon",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveCustomLabelPolicyIcon",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_custom_label_policy_icon_dark(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveCustomLabelPolicyIconDarkRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RemoveCustomLabelPolicyIconDarkResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveCustomLabelPolicyIconDark",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveCustomLabelPolicyIconDark",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_custom_label_policy_font(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveCustomLabelPolicyFontRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveCustomLabelPolicyFontResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveCustomLabelPolicyFont",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveCustomLabelPolicyFont",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_label_policy_to_default(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetLabelPolicyToDefaultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResetLabelPolicyToDefaultResponse>,
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
                "/zitadel.management.v1.ManagementService/ResetLabelPolicyToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetLabelPolicyToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomInitMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomInitMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultInitMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultInitMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_init_message_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCustomInitMessageTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomInitMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomInitMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomInitMessageText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomInitMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomInitMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomPasswordResetMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomPasswordResetMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultPasswordResetMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultPasswordResetMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_password_reset_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomPasswordResetMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomPasswordResetMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomPasswordResetMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomPasswordResetMessageText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomPasswordResetMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomPasswordResetMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomVerifyEmailMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomVerifyEmailMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultVerifyEmailMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultVerifyEmailMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_verify_email_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomVerifyEmailMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomVerifyEmailMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomVerifyEmailMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomVerifyEmailMessageText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomVerifyEmailMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomVerifyEmailMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomVerifyPhoneMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomVerifyPhoneMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultVerifyPhoneMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultVerifyPhoneMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_verify_phone_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomVerifyPhoneMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomVerifyPhoneMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomVerifyPhoneMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomVerifyPhoneMessageText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomVerifyPhoneMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomVerifyPhoneMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomVerifySMSOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomVerifySMSOTPMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultVerifySMSOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultVerifySMSOTPMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_verify_smsotp_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomVerifySmsotpMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomVerifySmsotpMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomVerifySMSOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomVerifySMSOTPMessageText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomVerifySMSOTPMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomVerifySMSOTPMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomVerifyEmailOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomVerifyEmailOTPMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultVerifyEmailOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultVerifyEmailOTPMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_verify_email_otp_message_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomVerifyEmailOtpMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomVerifyEmailOtpMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomVerifyEmailOTPMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomVerifyEmailOTPMessageText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomVerifyEmailOTPMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomVerifyEmailOTPMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomDomainClaimedMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomDomainClaimedMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultDomainClaimedMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultDomainClaimedMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_domain_claimed_message_custom_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomDomainClaimedMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomDomainClaimedMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomDomainClaimedMessageCustomText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomDomainClaimedMessageCustomText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomDomainClaimedMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomDomainClaimedMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomPasswordlessRegistrationMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomPasswordlessRegistrationMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultPasswordlessRegistrationMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultPasswordlessRegistrationMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_passwordless_registration_message_custom_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomPasswordlessRegistrationMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomPasswordlessRegistrationMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomPasswordlessRegistrationMessageCustomText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomPasswordlessRegistrationMessageCustomText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomPasswordlessRegistrationMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomPasswordlessRegistrationMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomPasswordChangeMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomPasswordChangeMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultPasswordChangeMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultPasswordChangeMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_password_change_message_custom_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomPasswordChangeMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomPasswordChangeMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomPasswordChangeMessageCustomText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomPasswordChangeMessageCustomText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomPasswordChangeMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomPasswordChangeMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomInviteUserMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomInviteUserMessageText",
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
                "/zitadel.management.v1.ManagementService/GetDefaultInviteUserMessageText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultInviteUserMessageText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_custom_invite_user_message_custom_text(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SetCustomInviteUserMessageTextRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SetCustomInviteUserMessageTextResponse>,
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
                "/zitadel.management.v1.ManagementService/SetCustomInviteUserMessageCustomText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetCustomInviteUserMessageCustomText",
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
                "/zitadel.management.v1.ManagementService/ResetCustomInviteUserMessageTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomInviteUserMessageTextToDefault",
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
                "/zitadel.management.v1.ManagementService/GetCustomLoginTexts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetCustomLoginTexts",
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
                "/zitadel.management.v1.ManagementService/GetDefaultLoginTexts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetDefaultLoginTexts",
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
                "/zitadel.management.v1.ManagementService/SetCustomLoginText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/ResetCustomLoginTextToDefault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ResetCustomLoginTextToDefault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_org_idp_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrgIdpByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrgIdpByIdResponse>,
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
                "/zitadel.management.v1.ManagementService/GetOrgIDPByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetOrgIDPByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_org_id_ps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgIdPsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrgIdPsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListOrgIDPs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListOrgIDPs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_org_oidcidp(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOrgOidcidpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOrgOidcidpResponse>,
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
                "/zitadel.management.v1.ManagementService/AddOrgOIDCIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddOrgOIDCIDP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_org_jwtidp(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOrgJwtidpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOrgJwtidpResponse>,
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
                "/zitadel.management.v1.ManagementService/AddOrgJWTIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddOrgJWTIDP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_org_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateOrgIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateOrgIdpResponse>,
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
                "/zitadel.management.v1.ManagementService/DeactivateOrgIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeactivateOrgIDP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_org_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateOrgIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateOrgIdpResponse>,
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
                "/zitadel.management.v1.ManagementService/ReactivateOrgIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ReactivateOrgIDP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_org_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOrgIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOrgIdpResponse>,
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
                "/zitadel.management.v1.ManagementService/RemoveOrgIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "RemoveOrgIDP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_org_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrgIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrgIdpResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateOrgIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateOrgIDP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_org_idpoidc_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrgIdpoidcConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrgIdpoidcConfigResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateOrgIDPOIDCConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateOrgIDPOIDCConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_org_idpjwt_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrgIdpjwtConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrgIdpjwtConfigResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateOrgIDPJWTConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateOrgIDPJWTConfig",
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
                "/zitadel.management.v1.ManagementService/ListProviders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListProviders",
                    ),
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
                "/zitadel.management.v1.ManagementService/GetProviderByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetProviderByID",
                    ),
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
                "/zitadel.management.v1.ManagementService/AddGenericOAuthProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/UpdateGenericOAuthProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddGenericOIDCProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/UpdateGenericOIDCProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/MigrateGenericOIDCProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddJWTProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddJWTProvider",
                    ),
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
                "/zitadel.management.v1.ManagementService/UpdateJWTProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateJWTProvider",
                    ),
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
                "/zitadel.management.v1.ManagementService/AddAzureADProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/UpdateAzureADProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddGitHubProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddGitHubProvider",
                    ),
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
                "/zitadel.management.v1.ManagementService/UpdateGitHubProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddGitHubEnterpriseServerProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/UpdateGitHubEnterpriseServerProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddGitLabProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddGitLabProvider",
                    ),
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
                "/zitadel.management.v1.ManagementService/UpdateGitLabProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddGitLabSelfHostedProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/UpdateGitLabSelfHostedProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddGoogleProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddGoogleProvider",
                    ),
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
                "/zitadel.management.v1.ManagementService/UpdateGoogleProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddLDAPProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddLDAPProvider",
                    ),
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
                "/zitadel.management.v1.ManagementService/UpdateLDAPProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddAppleProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddAppleProvider",
                    ),
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
                "/zitadel.management.v1.ManagementService/UpdateAppleProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/AddSAMLProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "AddSAMLProvider",
                    ),
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
                "/zitadel.management.v1.ManagementService/UpdateSAMLProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/RegenerateSAMLProviderCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
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
                "/zitadel.management.v1.ManagementService/DeleteProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeleteProvider",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListActionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListActionsResponse>,
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
                "/zitadel.management.v1.ManagementService/ListActions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListActions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_action(
            &mut self,
            request: impl tonic::IntoRequest<super::GetActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetActionResponse>,
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
                "/zitadel.management.v1.ManagementService/GetAction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "GetAction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_action(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateActionResponse>,
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
                "/zitadel.management.v1.ManagementService/CreateAction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "CreateAction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_action(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateActionResponse>,
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
                "/zitadel.management.v1.ManagementService/UpdateAction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "UpdateAction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_action(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateActionResponse>,
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
                "/zitadel.management.v1.ManagementService/DeactivateAction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeactivateAction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_action(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateActionResponse>,
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
                "/zitadel.management.v1.ManagementService/ReactivateAction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ReactivateAction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_action(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteActionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteActionResponse>,
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
                "/zitadel.management.v1.ManagementService/DeleteAction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "DeleteAction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_flow_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFlowTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFlowTypesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListFlowTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListFlowTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_flow_trigger_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFlowTriggerTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFlowTriggerTypesResponse>,
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
                "/zitadel.management.v1.ManagementService/ListFlowTriggerTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ListFlowTriggerTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFlowResponse>,
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
                "/zitadel.management.v1.ManagementService/GetFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.management.v1.ManagementService", "GetFlow"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn clear_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClearFlowResponse>,
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
                "/zitadel.management.v1.ManagementService/ClearFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "ClearFlow",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_trigger_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTriggerActionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetTriggerActionsResponse>,
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
                "/zitadel.management.v1.ManagementService/SetTriggerActions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.management.v1.ManagementService",
                        "SetTriggerActions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
