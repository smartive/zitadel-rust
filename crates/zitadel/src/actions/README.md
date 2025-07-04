# ZITADEL Actions v3 Webhook Support

This module provides comprehensive support for handling ZITADEL Actions v3 webhooks in Rust applications.

## Features

- **Type-safe webhook handling**: Strongly typed request and response structures
- **Signature verification**: HMAC-SHA256 signature verification with timestamp validation
- **Framework integration**: Built-in support for Axum (with Actix and Rocket coming soon)
- **Builder pattern**: Fluent API for constructing responses
- **Async trait**: Define your webhook logic with async/await support

## Usage

### 1. Enable the feature

Add the `actions-v3` feature to your `Cargo.toml`:

```toml
[dependencies]
zitadel = { version = "0.1", features = ["actions-v3", "axum"] }
```

### 2. Implement the ActionHandler trait

```rust
use async_trait::async_trait;
use zitadel::actions::{ActionHandler, ActionRequest, ActionResponse};

struct MyHandler;

#[async_trait]
impl ActionHandler for MyHandler {
    type Error = MyError;
    
    async fn complement_token(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
        // Add custom claims based on your business logic
        Ok(ActionResponse::default()
            .add_claim("custom_claim", "value")
            .add_metadata("key", "value"))
    }
}
```

### 3. Set up the webhook endpoint

```rust
use axum::{routing::post, Router};
use zitadel::axum::actions::webhook_handler;

let app = Router::new()
    .route("/webhook", post(webhook_handler(MyHandler, "your-webhook-secret")));
```

### 4. Configure ZITADEL Action

In ZITADEL, create an Actions v3 with:
- Trigger: `complement_token` (or other supported triggers)
- Type: Webhook
- URL: Your webhook endpoint
- Secret: The same secret used in your handler

## Supported Triggers

- `complement_token`: Modify access token claims
- `pre_userinfo_creation`: Modify userinfo endpoint response
- `pre_access_token_creation`: Modify token before creation
- `post_authentication`: React to successful authentication
- `post_creation`: React to resource creation
- `pre_creation`: Validate before resource creation

## Security

- Always use HTTPS in production
- Keep your webhook secret secure
- Implement proper error handling
- Validate all inputs from the webhook payload
- Use the built-in signature verification

## Example: Job-Specific Claims

```rust
async fn complement_token(&self, req: &ActionRequest) -> Result<ActionResponse, Self::Error> {
    if let Some(service_account) = &req.service_account {
        // Extract job ID from audience
        if let Some(job_id) = extract_job_id(&req) {
            // Verify access in your database
            if self.verify_access(&service_account.client_id, &job_id).await? {
                return Ok(ActionResponse::default()
                    .add_claim("job_id", job_id)
                    .add_claim("permissions", vec!["read", "write"]));
            }
        }
    }
    Ok(ActionResponse::default())
}
```