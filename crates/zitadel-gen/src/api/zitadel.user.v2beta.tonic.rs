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
                "/zitadel.user.v2beta.UserService/AddHumanUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "AddHumanUser"),
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
                "/zitadel.user.v2beta.UserService/GetUserByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "GetUserByID"),
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
                "/zitadel.user.v2beta.UserService/ListUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2beta.UserService", "ListUsers"));
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
                "/zitadel.user.v2beta.UserService/SetEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2beta.UserService", "SetEmail"));
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
                "/zitadel.user.v2beta.UserService/ResendEmailCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "ResendEmailCode"),
                );
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
                "/zitadel.user.v2beta.UserService/VerifyEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "VerifyEmail"),
                );
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
                "/zitadel.user.v2beta.UserService/SetPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2beta.UserService", "SetPhone"));
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
                "/zitadel.user.v2beta.UserService/RemovePhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "RemovePhone"),
                );
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
                "/zitadel.user.v2beta.UserService/ResendPhoneCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "ResendPhoneCode"),
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
                "/zitadel.user.v2beta.UserService/VerifyPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "VerifyPhone"),
                );
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
                "/zitadel.user.v2beta.UserService/UpdateHumanUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "UpdateHumanUser"),
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
                "/zitadel.user.v2beta.UserService/DeactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "DeactivateUser"),
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
                "/zitadel.user.v2beta.UserService/ReactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "ReactivateUser"),
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
                "/zitadel.user.v2beta.UserService/LockUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2beta.UserService", "LockUser"));
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
                "/zitadel.user.v2beta.UserService/UnlockUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "UnlockUser"),
                );
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
                "/zitadel.user.v2beta.UserService/DeleteUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "DeleteUser"),
                );
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
                "/zitadel.user.v2beta.UserService/RegisterPasskey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "RegisterPasskey"),
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
                "/zitadel.user.v2beta.UserService/VerifyPasskeyRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2beta.UserService",
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
                "/zitadel.user.v2beta.UserService/CreatePasskeyRegistrationLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2beta.UserService",
                        "CreatePasskeyRegistrationLink",
                    ),
                );
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
                "/zitadel.user.v2beta.UserService/RegisterU2F",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "RegisterU2F"),
                );
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
                "/zitadel.user.v2beta.UserService/VerifyU2FRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2beta.UserService",
                        "VerifyU2FRegistration",
                    ),
                );
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
                "/zitadel.user.v2beta.UserService/RegisterTOTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "RegisterTOTP"),
                );
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
                "/zitadel.user.v2beta.UserService/VerifyTOTPRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2beta.UserService",
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
                "/zitadel.user.v2beta.UserService/RemoveTOTP",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "RemoveTOTP"),
                );
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
                "/zitadel.user.v2beta.UserService/AddOTPSMS",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.user.v2beta.UserService", "AddOTPSMS"));
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
                "/zitadel.user.v2beta.UserService/RemoveOTPSMS",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "RemoveOTPSMS"),
                );
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
                "/zitadel.user.v2beta.UserService/AddOTPEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "AddOTPEmail"),
                );
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
                "/zitadel.user.v2beta.UserService/RemoveOTPEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "RemoveOTPEmail"),
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
                "/zitadel.user.v2beta.UserService/StartIdentityProviderIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2beta.UserService",
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
                "/zitadel.user.v2beta.UserService/RetrieveIdentityProviderIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2beta.UserService",
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
                "/zitadel.user.v2beta.UserService/AddIDPLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "AddIDPLink"),
                );
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
                "/zitadel.user.v2beta.UserService/PasswordReset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "PasswordReset"),
                );
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
                "/zitadel.user.v2beta.UserService/SetPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.user.v2beta.UserService", "SetPassword"),
                );
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
                "/zitadel.user.v2beta.UserService/ListAuthenticationMethodTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.v2beta.UserService",
                        "ListAuthenticationMethodTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
