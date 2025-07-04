// @generated
/// Generated client implementations.
pub mod zitadel_users_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ZitadelUsersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ZitadelUsersClient<tonic::transport::Channel> {
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
    impl<T> ZitadelUsersClient<T>
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
        ) -> ZitadelUsersClient<InterceptedService<T, F>>
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
            ZitadelUsersClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn search_users(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchUsersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/SearchUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "SearchUsers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/GetUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "GetUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_user(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/CreateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "CreateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn patch_user(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/PatchUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "PatchUser",
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
                "/zitadel.resources.user.v3alpha.ZITADELUsers/DeactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "DeactivateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateUserResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/ActivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "ActivateUser",
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
                "/zitadel.resources.user.v3alpha.ZITADELUsers/LockUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
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
                "/zitadel.resources.user.v3alpha.ZITADELUsers/UnlockUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "UnlockUser",
                    ),
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
                "/zitadel.resources.user.v3alpha.ZITADELUsers/DeleteUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "DeleteUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_contact_email(
            &mut self,
            request: impl tonic::IntoRequest<super::SetContactEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetContactEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/SetContactEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "SetContactEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_contact_email(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyContactEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyContactEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/VerifyContactEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "VerifyContactEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_contact_email_code(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendContactEmailCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendContactEmailCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/ResendContactEmailCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "ResendContactEmailCode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_contact_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::SetContactPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetContactPhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/SetContactPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "SetContactPhone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_contact_phone(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyContactPhoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyContactPhoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/VerifyContactPhone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "VerifyContactPhone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_contact_phone_code(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendContactPhoneCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendContactPhoneCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/ResendContactPhoneCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "ResendContactPhoneCode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_username(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUsernameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddUsernameResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/AddUsername",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "AddUsername",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_username(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveUsernameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveUsernameResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/RemoveUsername",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "RemoveUsername",
                    ),
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
                "/zitadel.resources.user.v3alpha.ZITADELUsers/SetPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "SetPassword",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_password_reset(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestPasswordResetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RequestPasswordResetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/RequestPasswordReset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "RequestPasswordReset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_web_auth_n_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::StartWebAuthNRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartWebAuthNRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/StartWebAuthNRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "StartWebAuthNRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_web_auth_n_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyWebAuthNRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyWebAuthNRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/VerifyWebAuthNRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "VerifyWebAuthNRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_web_auth_n_registration_link(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateWebAuthNRegistrationLinkRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CreateWebAuthNRegistrationLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/CreateWebAuthNRegistrationLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "CreateWebAuthNRegistrationLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_web_auth_n_authenticator(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveWebAuthNAuthenticatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveWebAuthNAuthenticatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/RemoveWebAuthNAuthenticator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "RemoveWebAuthNAuthenticator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_totp_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::StartTotpRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartTotpRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/StartTOTPRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "StartTOTPRegistration",
                    ),
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
                "/zitadel.resources.user.v3alpha.ZITADELUsers/VerifyTOTPRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "VerifyTOTPRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_totp_authenticator(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveTotpAuthenticatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveTotpAuthenticatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/RemoveTOTPAuthenticator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "RemoveTOTPAuthenticator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_otpsms_authenticator(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOtpsmsAuthenticatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOtpsmsAuthenticatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/AddOTPSMSAuthenticator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "AddOTPSMSAuthenticator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_otpsms_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyOtpsmsRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyOtpsmsRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/VerifyOTPSMSRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "VerifyOTPSMSRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_otpsms_authenticator(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOtpsmsAuthenticatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOtpsmsAuthenticatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/RemoveOTPSMSAuthenticator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "RemoveOTPSMSAuthenticator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_otp_email_authenticator(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOtpEmailAuthenticatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOtpEmailAuthenticatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/AddOTPEmailAuthenticator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "AddOTPEmailAuthenticator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_otp_email_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyOtpEmailRegistrationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyOtpEmailRegistrationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/VerifyOTPEmailRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "VerifyOTPEmailRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_otp_email_authenticator(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveOtpEmailAuthenticatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveOtpEmailAuthenticatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/RemoveOTPEmailAuthenticator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "RemoveOTPEmailAuthenticator",
                    ),
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
                "/zitadel.resources.user.v3alpha.ZITADELUsers/StartIdentityProviderIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "StartIdentityProviderIntent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_identity_provider_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIdentityProviderIntentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetIdentityProviderIntentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/GetIdentityProviderIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "GetIdentityProviderIntent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_idp_authenticator(
            &mut self,
            request: impl tonic::IntoRequest<super::AddIdpAuthenticatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddIdpAuthenticatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/AddIDPAuthenticator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "AddIDPAuthenticator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_idp_authenticator(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveIdpAuthenticatorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveIdpAuthenticatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.user.v3alpha.ZITADELUsers/RemoveIDPAuthenticator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.user.v3alpha.ZITADELUsers",
                        "RemoveIDPAuthenticator",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
