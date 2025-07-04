//! Axum integration for ZITADEL Actions v3 webhooks

use axum::{
    body::Bytes,
    extract::{FromRequest, FromRequestParts},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::actions::{
    ActionHandler, ActionRequest, ActionResponse, WebhookError, WebhookVerifier, route_action,
};

/// Axum extractor for verified webhook payloads
pub struct VerifiedWebhook<T> {
    pub payload: T,
}

/// Configuration for webhook handler
pub struct WebhookConfig {
    pub verifier: Option<WebhookVerifier>,
}

impl WebhookConfig {
    /// Create a new webhook configuration
    pub fn new() -> Self {
        Self { verifier: None }
    }
    
    /// Add signature verification with the given secret
    pub fn with_verification(mut self, secret: impl Into<String>) -> Self {
        self.verifier = Some(WebhookVerifier::new(secret));
        self
    }
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Create an Axum handler for ZITADEL Actions v3 webhooks
/// 
/// # Example
/// 
/// ```rust
/// use axum::{Router, routing::post};
/// use zitadel::axum::actions::webhook_handler;
/// use zitadel::actions::{ActionHandler, ActionRequest, ActionResponse};
/// 
/// struct MyHandler;
/// 
/// #[async_trait::async_trait]
/// impl ActionHandler for MyHandler {
///     type Error = MyError;
///     
///     async fn complement_token(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
///         Ok(ActionResponse::default().add_claim("custom", "value"))
///     }
/// }
/// 
/// let app = Router::new()
///     .route("/webhook", post(webhook_handler(MyHandler, "webhook-secret")));
/// ```
pub fn webhook_handler<H>(
    handler: H,
    secret: impl Into<String>,
) -> impl Fn(HeaderMap, Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Json<ActionResponse>, WebhookError>> + Send>>
where
    H: ActionHandler + 'static,
{
    let handler = Arc::new(handler);
    let config = WebhookConfig::new().with_verification(secret);
    let verifier = config.verifier;
    
    move |headers: HeaderMap, body: Bytes| {
        let handler = handler.clone();
        let verifier = verifier.clone();
        
        Box::pin(async move {
            // Verify signature if configured
            if let Some(verifier) = &verifier {
                let signature = headers
                    .get("x-zitadel-signature")
                    .ok_or(WebhookError::MissingSignature)?
                    .to_str()
                    .map_err(|_| WebhookError::InvalidSignature)?;
                
                let timestamp = headers
                    .get("x-zitadel-timestamp")
                    .and_then(|h| h.to_str().ok());
                
                verifier.verify(&body, signature, timestamp)?;
            }
            
            // Parse request
            let request: ActionRequest = serde_json::from_slice(&body)
                .map_err(|e| WebhookError::InvalidBody(e.to_string()))?;
            
            // Route to appropriate handler
            let response = route_action(&*handler, &request)
                .await
                .map_err(|e| WebhookError::HandlerError(Box::new(e)))?;
            
            Ok(Json(response))
        })
    }
}

/// Create an Axum handler without signature verification
/// 
/// **Warning**: Only use this in development or when ZITADEL webhooks
/// are already behind authentication.
pub fn webhook_handler_unsafe<H>(
    handler: H,
) -> impl Fn(Json<ActionRequest>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Json<ActionResponse>, WebhookError>> + Send>>
where
    H: ActionHandler + 'static,
{
    let handler = Arc::new(handler);
    
    move |Json(request): Json<ActionRequest>| {
        let handler = handler.clone();
        
        Box::pin(async move {
            let response = route_action(&*handler, &request)
                .await
                .map_err(|e| WebhookError::HandlerError(Box::new(e)))?;
            
            Ok(Json(response))
        })
    }
}

impl IntoResponse for WebhookError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            WebhookError::InvalidSignature | WebhookError::MissingSignature => {
                (StatusCode::UNAUTHORIZED, "Unauthorized")
            }
            WebhookError::InvalidBody(_) => {
                (StatusCode::BAD_REQUEST, "Invalid request body")
            }
            WebhookError::InvalidTimestamp(_) => {
                (StatusCode::BAD_REQUEST, "Invalid timestamp")
            }
            WebhookError::HandlerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        
        // In production, don't expose detailed error messages
        let body = Json({
            let mut obj = json!({
                "error": message,
            });
            #[cfg(debug_assertions)]
            {
                obj["details"] = serde_json::Value::String(self.to_string());
            }
            obj
        });
        
        (status, body).into_response()
    }
}

/// State extractor for webhook handlers that need access to app state
/// 
/// # Example
/// 
/// ```rust
/// use axum::extract::State;
/// use zitadel::actions::{ActionHandler, ActionRequest, ActionResponse};
/// 
/// struct MyHandler {
///     db: DatabasePool,
/// }
/// 
/// #[async_trait::async_trait]
/// impl ActionHandler for MyHandler {
///     type Error = MyError;
///     
///     async fn complement_token(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
///         // Access self.db here
///         Ok(ActionResponse::default())
///     }
/// }
/// 
/// // In your router setup:
/// let handler = MyHandler { db: pool };
/// app.route("/webhook", post(webhook_handler(handler, "secret")));
/// ```
pub struct WebhookState<S> {
    pub state: S,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::{ActionInfo, ActionRequest, ActionResponse};
    use async_trait::async_trait;
    use std::collections::HashMap;
    
    struct TestHandler;
    
    #[derive(Debug, thiserror::Error)]
    #[error("Test error")]
    struct TestError;
    
    #[async_trait]
    impl ActionHandler for TestHandler {
        type Error = TestError;
        
        async fn complement_token(&self, _req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
            Ok(ActionResponse::default().add_claim("test", "value"))
        }
    }
    
    #[tokio::test]
    async fn test_webhook_handler_unsafe() {
        let handler = webhook_handler_unsafe(TestHandler);
        
        let request = ActionRequest {
            action: ActionInfo {
                function: "complement_token".to_string(),
                id: "test".to_string(),
                name: "test".to_string(),
            },
            user: None,
            service_account: None,
            token: None,
            request: None,
            org: None,
            additional: HashMap::new(),
        };
        
        let result = handler(Json(request)).await;
        assert!(result.is_ok());
        
        let Json(response) = result.unwrap();
        assert_eq!(response.append_claims.len(), 1);
        assert_eq!(response.append_claims[0].key, "test");
    }
}