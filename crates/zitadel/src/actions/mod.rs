//! ZITADEL Actions v3 webhook support
//! 
//! This module provides types and traits for handling ZITADEL Actions v3 webhooks.
//! 
//! # Example
//! 
//! ```rust
//! use zitadel::actions::{ActionHandler, ActionRequest, ActionResponse};
//! 
//! struct MyHandler;
//! 
//! #[async_trait::async_trait]
//! impl ActionHandler for MyHandler {
//!     type Error = MyError;
//!     
//!     async fn complement_token(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
//!         Ok(ActionResponse::default()
//!             .add_claim("custom_claim", "value"))
//!     }
//! }
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod types;
mod verification;
mod helpers;

pub use types::*;
pub use verification::*;
pub use helpers::*;

/// Trait for handling ZITADEL Actions v3 webhooks
/// 
/// Implement this trait to define custom logic for different webhook events.
/// Each method corresponds to a specific ZITADEL action trigger.
#[async_trait]
pub trait ActionHandler: Send + Sync {
    /// Error type returned by handler methods
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Handle the complement_token action
    /// 
    /// Called when ZITADEL is creating or refreshing an access token.
    /// Use this to add custom claims to the token.
    async fn complement_token(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        Ok(ActionResponse::default())
    }
    
    /// Handle the pre_userinfo_creation action
    /// 
    /// Called before the userinfo endpoint returns data.
    /// Use this to modify userinfo claims.
    async fn pre_userinfo_creation(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        Ok(ActionResponse::default())
    }
    
    /// Handle the pre_access_token_creation action
    /// 
    /// Called before an access token is created.
    /// Use this to add custom claims or metadata.
    async fn pre_access_token_creation(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        Ok(ActionResponse::default())
    }
    
    /// Handle the post_authentication action
    /// 
    /// Called after successful authentication.
    /// Use this for logging or additional processing.
    async fn post_authentication(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        Ok(ActionResponse::default())
    }
    
    /// Handle the post_creation action
    /// 
    /// Called after a resource is created.
    /// Use this for post-processing or notifications.
    async fn post_creation(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        Ok(ActionResponse::default())
    }
    
    /// Handle the pre_creation action
    /// 
    /// Called before a resource is created.
    /// Use this for validation or preprocessing.
    async fn pre_creation(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        Ok(ActionResponse::default())
    }
}

/// Route an action request to the appropriate handler method
pub async fn route_action<H: ActionHandler>(
    handler: &H,
    request: &ActionRequest,
) -> Result<ActionResponse, H::Error> {
    match request.action.function.as_str() {
        "complement_token" => handler.complement_token(request).await,
        "pre_userinfo_creation" => handler.pre_userinfo_creation(request).await,
        "pre_access_token_creation" => handler.pre_access_token_creation(request).await,
        "post_authentication" => handler.post_authentication(request).await,
        "post_creation" => handler.post_creation(request).await,
        "pre_creation" => handler.pre_creation(request).await,
        _ => Ok(ActionResponse::default()),
    }
}