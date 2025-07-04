// @generated
/// Generated client implementations.
pub mod user_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserServiceClient<tonic::transport::Channel> {
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
    impl<T> UserServiceClient<T>
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
        ) -> UserServiceClient<InterceptedService<T, F>>
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
            UserServiceClient::new(InterceptedService::new(inner, interceptor))
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
                "/zitadel.user.v2.UserService/AddHumanUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "AddHumanUser"));
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
                "/zitadel.user.v2.UserService/GetUserByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "GetUserByID"));
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
                "/zitadel.user.v2.UserService/ListUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "ListUsers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_email(
            &mut self,
            request: impl tonic::IntoRequest<super::SetEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/SetEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "SetEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_email_code(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendEmailCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendEmailCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/ResendEmailCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "ResendEmailCode"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_email_code(
            &mut self,
            request: impl tonic::IntoRequest<super::SendEmailCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendEmailCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/SendEmailCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "SendEmailCode"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_email(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/VerifyEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "VerifyEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetPhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/SetPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "SetPhone"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RemovePhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "RemovePhone"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_phone_code(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendPhoneCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendPhoneCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/ResendPhoneCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "ResendPhoneCode"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyPhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/VerifyPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "VerifyPhone"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_human_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHumanUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateHumanUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/UpdateHumanUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "UpdateHumanUser"),
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
                "/zitadel.user.v2.UserService/DeactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "DeactivateUser"),
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
                "/zitadel.user.v2.UserService/ReactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "ReactivateUser"),
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
                "/zitadel.user.v2.UserService/LockUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "LockUser"));
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
                "/zitadel.user.v2.UserService/UnlockUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "UnlockUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/DeleteUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "DeleteUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_passkey(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterPasskeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterPasskeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RegisterPasskey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "RegisterPasskey"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_passkey_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyPasskeyRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyPasskeyRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/VerifyPasskeyRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2.UserService",
                        "VerifyPasskeyRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_passkey_registration_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePasskeyRegistrationLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePasskeyRegistrationLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/CreatePasskeyRegistrationLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2.UserService",
                        "CreatePasskeyRegistrationLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_passkeys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPasskeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPasskeysResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/ListPasskeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "ListPasskeys"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_passkey(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePasskeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePasskeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RemovePasskey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "RemovePasskey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_u2f(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterU2fRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterU2fResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RegisterU2F",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "RegisterU2F"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_u2f_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyU2fRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyU2fRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/VerifyU2FRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2.UserService",
                        "VerifyU2FRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_u2f(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveU2fRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveU2fResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RemoveU2F",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "RemoveU2F"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_totp(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterTotpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterTotpResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RegisterTOTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "RegisterTOTP"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_totp_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyTotpRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyTotpRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/VerifyTOTPRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2.UserService",
                        "VerifyTOTPRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_totp(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveTotpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveTotpResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RemoveTOTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "RemoveTOTP"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_otpsms(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOtpsmsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOtpsmsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/AddOTPSMS",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "AddOTPSMS"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_otpsms(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOtpsmsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOtpsmsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RemoveOTPSMS",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "RemoveOTPSMS"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_otp_email(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOtpEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOtpEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/AddOTPEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "AddOTPEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_otp_email(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOtpEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOtpEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RemoveOTPEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "RemoveOTPEmail"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_identity_provider_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::StartIdentityProviderIntentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartIdentityProviderIntentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/StartIdentityProviderIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2.UserService",
                        "StartIdentityProviderIntent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_identity_provider_intent(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RetrieveIdentityProviderIntentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveIdentityProviderIntentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RetrieveIdentityProviderIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2.UserService",
                        "RetrieveIdentityProviderIntent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_idp_link(
            &mut self,
            request: impl tonic::IntoRequest<super::AddIdpLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddIdpLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/AddIDPLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "AddIDPLink"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_idp_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIdpLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListIdpLinksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/ListIDPLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "ListIDPLinks"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_idp_link(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveIdpLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveIdpLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/RemoveIDPLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "RemoveIDPLink"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn password_reset(
            &mut self,
            request: impl tonic::IntoRequest<super::PasswordResetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PasswordResetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/PasswordReset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "PasswordReset"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_password(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetPasswordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/SetPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2.UserService", "SetPassword"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_authentication_method_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthenticationMethodTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuthenticationMethodTypesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/ListAuthenticationMethodTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2.UserService",
                        "ListAuthenticationMethodTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_authentication_factors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthenticationFactorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuthenticationFactorsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/ListAuthenticationFactors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2.UserService",
                        "ListAuthenticationFactors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_invite_code(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInviteCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInviteCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/CreateInviteCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "CreateInviteCode"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_invite_code(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendInviteCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendInviteCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/ResendInviteCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "ResendInviteCode"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_invite_code(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyInviteCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyInviteCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/VerifyInviteCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "VerifyInviteCode"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn human_mfa_init_skipped(
            &mut self,
            request: impl tonic::IntoRequest<super::HumanMfaInitSkippedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HumanMfaInitSkippedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.v2.UserService/HumanMFAInitSkipped",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2.UserService", "HumanMFAInitSkipped"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
