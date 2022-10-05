// use tonic::{
//     codegen::InterceptedService,
//     transport::{Channel, Endpoint},
// };

// use super::{
//     interceptors::AccessTokenInterceptor,
//     zitadel::management::v1::management_service_client::ManagementServiceClient,
// };

// pub async fn create_management_client(
//     api_endpoint: &str,
// ) -> Result<
//     ManagementServiceClient<InterceptedService<Channel, AccessTokenInterceptor>>,
//     Box<dyn std::error::Error>,
// > {
//     let channel = Endpoint::from_static("api_endpoint").connect().await?;

//     Ok(ManagementServiceClient::with_interceptor(
//         channel,
//         AccessTokenInterceptor::new("token"),
//     ))
// }
