//! contains interceptors

use tonic::{Request, Status};
use tonic::service::Interceptor;

/// Simple gRPC `Interceptor` that attaches a given access token to any request
/// a client sends. The token is attached with the `Bearer` auth-scheme.
/// *Note*: This interceptor does not check if the token is valid.
pub struct AccessTokenInterceptor {
    access_token: String,
}

impl AccessTokenInterceptor {
    /// Create a new [`AccessTokenInterceptor`][AccessTokenInterceptor].
    pub fn new(token: String) -> Self {
        AccessTokenInterceptor {
            access_token: token,
        }
    }
}

impl Interceptor for AccessTokenInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", self.access_token).parse().unwrap(),
        );
        Ok(request)
    }
}
