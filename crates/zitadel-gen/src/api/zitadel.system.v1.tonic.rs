// @generated
/// Generated client implementations.
pub mod system_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SystemServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SystemServiceClient<tonic::transport::Channel> {
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
    impl<T> SystemServiceClient<T>
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
        ) -> SystemServiceClient<InterceptedService<T, F>>
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
            SystemServiceClient::new(InterceptedService::new(inner, interceptor))
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
                "/zitadel.system.v1.SystemService/Healthz",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.system.v1.SystemService", "Healthz"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstancesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "ListInstances"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInstanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "GetInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::AddInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddInstanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/AddInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "AddInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateInstanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/UpdateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "UpdateInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInstanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/CreateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "CreateInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveInstanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/RemoveInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "RemoveInstance"),
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
                "/zitadel.system.v1.SystemService/ListIAMMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "ListIAMMembers"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn exists_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::ExistsDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExistsDomainResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/ExistsDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "ExistsDomain"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDomainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDomainsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/ListDomains",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "ListDomains"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::AddDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddDomainResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/AddDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.system.v1.SystemService", "AddDomain"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveDomainResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/RemoveDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "RemoveDomain"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_primary_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPrimaryDomainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetPrimaryDomainResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/SetPrimaryDomain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.system.v1.SystemService",
                        "SetPrimaryDomain",
                    ),
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
                "/zitadel.system.v1.SystemService/ListViews",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.system.v1.SystemService", "ListViews"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn clear_view(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClearViewResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/ClearView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.system.v1.SystemService", "ClearView"));
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
                "/zitadel.system.v1.SystemService/ListFailedEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.system.v1.SystemService",
                        "ListFailedEvents",
                    ),
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
                "/zitadel.system.v1.SystemService/RemoveFailedEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.system.v1.SystemService",
                        "RemoveFailedEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_quota(
            &mut self,
            request: impl tonic::IntoRequest<super::AddQuotaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddQuotaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/AddQuota",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.system.v1.SystemService", "AddQuota"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_quota(
            &mut self,
            request: impl tonic::IntoRequest<super::SetQuotaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetQuotaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/SetQuota",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.system.v1.SystemService", "SetQuota"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_quota(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveQuotaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveQuotaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/RemoveQuota",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "RemoveQuota"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_instance_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInstanceFeatureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetInstanceFeatureResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/SetInstanceFeature",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.system.v1.SystemService",
                        "SetInstanceFeature",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLimitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetLimitsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/SetLimits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.system.v1.SystemService", "SetLimits"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn bulk_set_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkSetLimitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BulkSetLimitsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/BulkSetLimits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "BulkSetLimits"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetLimitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResetLimitsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.system.v1.SystemService/ResetLimits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.system.v1.SystemService", "ResetLimits"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
