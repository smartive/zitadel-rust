// @generated
/// Generated client implementations.
pub mod zitadel_user_schemas_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ZitadelUserSchemasClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ZitadelUserSchemasClient<tonic::transport::Channel> {
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
    impl<T> ZitadelUserSchemasClient<T>
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
        ) -> ZitadelUserSchemasClient<InterceptedService<T, F>>
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
            ZitadelUserSchemasClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn search_user_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchUserSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchUserSchemasResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.userschema.v3alpha.ZITADELUserSchemas/SearchUserSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.userschema.v3alpha.ZITADELUserSchemas",
                        "SearchUserSchemas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.userschema.v3alpha.ZITADELUserSchemas/GetUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.userschema.v3alpha.ZITADELUserSchemas",
                        "GetUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Create a user schema

 Create the first revision of a new user schema. The schema can then be used on users to store and validate their data.
*/
        pub async fn create_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.userschema.v3alpha.ZITADELUserSchemas/CreateUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.userschema.v3alpha.ZITADELUserSchemas",
                        "CreateUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Patch a user schema

 Patch an existing user schema to a new revision. Users based on the current revision will not be affected until they are updated.
*/
        pub async fn patch_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.userschema.v3alpha.ZITADELUserSchemas/PatchUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.userschema.v3alpha.ZITADELUserSchemas",
                        "PatchUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Deactivate a user schema

 Deactivate an existing user schema and change it into a read-only state. Users based on this schema cannot be updated anymore, but are still able to authenticate.
*/
        pub async fn deactivate_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.userschema.v3alpha.ZITADELUserSchemas/DeactivateUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.userschema.v3alpha.ZITADELUserSchemas",
                        "DeactivateUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Reactivate a user schema

 Reactivate an previously deactivated user schema and change it into an active state again.
*/
        pub async fn reactivate_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.userschema.v3alpha.ZITADELUserSchemas/ReactivateUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.userschema.v3alpha.ZITADELUserSchemas",
                        "ReactivateUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Delete a user schema

 Delete an existing user schema. This operation is only allowed if there are no associated users to it.
*/
        pub async fn delete_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.resources.userschema.v3alpha.ZITADELUserSchemas/DeleteUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.resources.userschema.v3alpha.ZITADELUserSchemas",
                        "DeleteUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
