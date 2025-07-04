//! Example of handling ZITADEL Actions v3 webhooks
//! 
//! This example shows how to create a webhook handler for ZITADEL Actions v3
//! that adds custom claims to tokens.

use async_trait::async_trait;
use axum::{routing::post, Router};
use std::net::SocketAddr;
use zitadel::actions::{ActionHandler, ActionRequest, ActionResponse};

/// Custom webhook handler that adds job-specific claims
struct JobLoggerHandler;

#[derive(Debug, thiserror::Error)]
#[error("Handler error")]
struct HandlerError;

#[async_trait]
impl ActionHandler for JobLoggerHandler {
    type Error = HandlerError;
    
    async fn complement_token(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        // Check if this is a service account requesting a token
        if let Some(service_account) = &req.service_account {
            println!("Service account {} is requesting a token", service_account.client_id);
            
            // Look for job ID in the audience
            if let Some(token) = &req.token {
                for audience in &token.audience {
                    if let Some(job_id) = audience.strip_prefix("logger:job:") {
                        println!("Found job ID in audience: {}", job_id);
                        
                        // In a real application, you would verify the service account
                        // has access to this job by checking your database
                        
                        return Ok(ActionResponse::default()
                            .add_claim("urn:zitadel:iam:job:id", job_id)
                            .add_claim("urn:zitadel:iam:job:permissions", vec!["log.read", "log.write"])
                            .add_log(format!("Added job claims for job {}", job_id)));
                    }
                }
            }
        }
        
        // Return empty response if no custom claims needed
        Ok(ActionResponse::default())
    }
    
    async fn pre_userinfo_creation(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        // You can also modify userinfo responses
        if let Some(user) = &req.user {
            println!("Modifying userinfo for user {}", user.id);
            
            return Ok(ActionResponse::default()
                .add_claim("custom_userinfo", "value")
                .add_metadata("last_webhook_call", chrono::Utc::now().to_rfc3339()));
        }
        
        Ok(ActionResponse::default())
    }
}

#[tokio::main]
async fn main() {
    // Create the webhook handler
    let handler = JobLoggerHandler;
    
    // Get webhook secret from environment
    let webhook_secret = std::env::var("ZITADEL_WEBHOOK_SECRET")
        .unwrap_or_else(|_| "test-secret".to_string());
    
    // Create the router with webhook endpoint
    let app = Router::new()
        .route(
            "/webhook",
            post(zitadel::axum::actions::webhook_handler(handler, webhook_secret))
        );
    
    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Webhook server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}