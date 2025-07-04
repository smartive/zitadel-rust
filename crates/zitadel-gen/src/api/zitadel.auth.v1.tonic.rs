// @generated
/// Generated client implementations.
pub mod auth_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthServiceClient<tonic::transport::Channel> {
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
    impl<T> AuthServiceClient<T>
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
        ) -> AuthServiceClient<InterceptedService<T, F>>
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
            AuthServiceClient::new(InterceptedService::new(inner, interceptor))
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
                "/zitadel.auth.v1.AuthService/Healthz",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "Healthz"));
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
                "/zitadel.auth.v1.AuthService/GetSupportedLanguages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "GetSupportedLanguages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "GetMyUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_user(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "RemoveMyUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_user_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyUserChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyUserChangesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyUserChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyUserChanges"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_user_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyUserSessionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyUserSessionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyUserSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyUserSessions"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyMetadataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyMetadata"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyMetadataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "GetMyMetadata"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_refresh_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyRefreshTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyRefreshTokensResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyRefreshTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyRefreshTokens"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn revoke_my_refresh_token(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeMyRefreshTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeMyRefreshTokenResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RevokeMyRefreshToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "RevokeMyRefreshToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn revoke_all_my_refresh_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeAllMyRefreshTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeAllMyRefreshTokensResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RevokeAllMyRefreshTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "RevokeAllMyRefreshTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_my_user_name(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMyUserNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMyUserNameResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/UpdateMyUserName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "UpdateMyUserName"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_password_complexity_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyPasswordComplexityPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyPasswordComplexityPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyPasswordComplexityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "GetMyPasswordComplexityPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_my_password(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMyPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMyPasswordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/UpdateMyPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "UpdateMyPassword"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyProfileResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "GetMyProfile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_my_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMyProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMyProfileResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/UpdateMyProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "UpdateMyProfile"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_email(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "GetMyEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_my_email(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMyEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetMyEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/SetMyEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "SetMyEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_my_email(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMyEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyMyEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/VerifyMyEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "VerifyMyEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_my_email_verification(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendMyEmailVerificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendMyEmailVerificationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ResendMyEmailVerification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "ResendMyEmailVerification",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyPhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "GetMyPhone"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_my_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMyPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetMyPhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/SetMyPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "SetMyPhone"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_my_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMyPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyMyPhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/VerifyMyPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "VerifyMyPhone"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_my_phone_verification(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendMyPhoneVerificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendMyPhoneVerificationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ResendMyPhoneVerification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "ResendMyPhoneVerification",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyPhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.auth.v1.AuthService", "RemoveMyPhone"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_avatar(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyAvatarRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyAvatarResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyAvatar",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "RemoveMyAvatar"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_linked_id_ps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyLinkedIdPsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyLinkedIdPsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyLinkedIDPs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyLinkedIDPs"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_linked_idp(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyLinkedIdpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyLinkedIdpResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyLinkedIDP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "RemoveMyLinkedIDP"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_auth_factors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyAuthFactorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyAuthFactorsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyAuthFactors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyAuthFactors"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_my_auth_factor_otp(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMyAuthFactorOtpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMyAuthFactorOtpResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/AddMyAuthFactorOTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "AddMyAuthFactorOTP"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_my_auth_factor_otp(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMyAuthFactorOtpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyMyAuthFactorOtpResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/VerifyMyAuthFactorOTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "VerifyMyAuthFactorOTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_auth_factor_otp(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyAuthFactorOtpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyAuthFactorOtpResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyAuthFactorOTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "RemoveMyAuthFactorOTP",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_my_auth_factor_otpsms(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMyAuthFactorOtpsmsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMyAuthFactorOtpsmsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/AddMyAuthFactorOTPSMS",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "AddMyAuthFactorOTPSMS",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_auth_factor_otpsms(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyAuthFactorOtpsmsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyAuthFactorOtpsmsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyAuthFactorOTPSMS",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "RemoveMyAuthFactorOTPSMS",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_my_auth_factor_otp_email(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMyAuthFactorOtpEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMyAuthFactorOtpEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/AddMyAuthFactorOTPEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "AddMyAuthFactorOTPEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_auth_factor_otp_email(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyAuthFactorOtpEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyAuthFactorOtpEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyAuthFactorOTPEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "RemoveMyAuthFactorOTPEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_my_auth_factor_u2f(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMyAuthFactorU2fRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMyAuthFactorU2fResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/AddMyAuthFactorU2F",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "AddMyAuthFactorU2F"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_my_auth_factor_u2f(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMyAuthFactorU2fRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyMyAuthFactorU2fResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/VerifyMyAuthFactorU2F",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "VerifyMyAuthFactorU2F",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_auth_factor_u2f(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyAuthFactorU2fRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyAuthFactorU2fResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyAuthFactorU2F",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "RemoveMyAuthFactorU2F",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_passwordless(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyPasswordlessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyPasswordlessResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyPasswordless",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyPasswordless"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_my_passwordless(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMyPasswordlessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMyPasswordlessResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/AddMyPasswordless",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "AddMyPasswordless"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_my_passwordless_link(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMyPasswordlessLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMyPasswordlessLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/AddMyPasswordlessLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "AddMyPasswordlessLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_my_passwordless_link(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMyPasswordlessLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendMyPasswordlessLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/SendMyPasswordlessLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "SendMyPasswordlessLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_my_passwordless(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMyPasswordlessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyMyPasswordlessResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/VerifyMyPasswordless",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "VerifyMyPasswordless",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_my_passwordless(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMyPasswordlessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMyPasswordlessResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/RemoveMyPasswordless",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "RemoveMyPasswordless",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_user_grants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyUserGrantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyUserGrantsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyUserGrants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyUserGrants"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_project_orgs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyProjectOrgsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyProjectOrgsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyProjectOrgs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyProjectOrgs"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_zitadel_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyZitadelPermissionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyZitadelPermissionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyZitadelPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "ListMyZitadelPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_project_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyProjectPermissionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyProjectPermissionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyProjectPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.auth.v1.AuthService",
                        "ListMyProjectPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_my_memberships(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMyMembershipsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMyMembershipsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/ListMyMemberships",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "ListMyMemberships"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_label_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyLabelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyLabelPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyLabelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "GetMyLabelPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_privacy_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyPrivacyPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyPrivacyPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyPrivacyPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "GetMyPrivacyPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_my_login_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMyLoginPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMyLoginPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.auth.v1.AuthService/GetMyLoginPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.auth.v1.AuthService", "GetMyLoginPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
