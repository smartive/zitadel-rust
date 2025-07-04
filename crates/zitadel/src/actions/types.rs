//! Types for ZITADEL Actions v3 webhooks

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// ZITADEL Actions v3 webhook request
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionRequest {
    /// Action context information
    pub action: ActionInfo,
    
    /// User information if this is a user token
    pub user: Option<User>,
    
    /// Service account information if this is a service account token
    pub service_account: Option<ServiceAccount>,
    
    /// Current token information
    pub token: Option<TokenInfo>,
    
    /// HTTP request context
    pub request: Option<RequestContext>,
    
    /// Organization context
    pub org: Option<Organization>,
    
    /// Additional context data
    #[serde(flatten)]
    pub additional: HashMap<String, Value>,
}

/// Action context information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionInfo {
    /// The function being called (e.g., "complement_token")
    pub function: String,
    
    /// Action ID
    pub id: String,
    
    /// Action name
    pub name: String,
}

/// User information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User ID
    pub id: String,
    
    /// Username
    pub username: String,
    
    /// Preferred username
    pub preferred_username: Option<String>,
    
    /// Human-readable name
    pub human: Option<HumanUser>,
    
    /// Machine user information
    pub machine: Option<MachineUser>,
    
    /// Resource owner (organization) ID
    pub resource_owner: String,
}

/// Human user details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanUser {
    /// First name
    pub first_name: String,
    
    /// Last name
    pub last_name: String,
    
    /// Display name
    pub display_name: Option<String>,
    
    /// Email address
    pub email: Option<String>,
    
    /// Email verified status
    pub email_verified: Option<bool>,
    
    /// Phone number
    pub phone: Option<String>,
    
    /// Phone verified status
    pub phone_verified: Option<bool>,
}

/// Machine user details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineUser {
    /// Machine name
    pub name: String,
    
    /// Machine description
    pub description: Option<String>,
}

/// Service account information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    /// User ID of the service account
    pub user_id: String,
    
    /// Client ID
    pub client_id: String,
    
    /// Resource owner (organization) ID
    pub resource_owner: String,
}

/// Token information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    /// Current token claims
    #[serde(default)]
    pub claims: HashMap<String, Value>,
    
    /// Token scopes
    #[serde(default)]
    pub scopes: Vec<String>,
    
    /// Token audience
    #[serde(default)]
    pub audience: Vec<String>,
}

/// HTTP request context
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    /// Client ID making the request
    pub client_id: String,
    
    /// Client name
    pub client_name: Option<String>,
    
    /// Grant type used
    pub grant_type: String,
    
    /// Authentication time
    pub auth_time: Option<String>,
    
    /// IP address
    pub ip_address: Option<String>,
    
    /// User agent
    pub user_agent: Option<String>,
}

/// Organization information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// Organization ID
    pub id: String,
    
    /// Organization name
    pub name: String,
    
    /// Primary domain
    pub primary_domain: String,
}

/// ZITADEL Actions v3 webhook response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionResponse {
    /// Claims to append to the token
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub append_claims: Vec<Claim>,
    
    /// User metadata to set
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub set_user_metadata: Vec<Metadata>,
    
    /// Log entries to append to claims
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub append_log_claims: Vec<String>,
}

/// Claim to append to token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    /// Claim key
    pub key: String,
    
    /// Claim value
    pub value: Value,
}

/// Metadata to set on user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Metadata key
    pub key: String,
    
    /// Metadata value
    pub value: Value,
}

/// Error type for webhook handling
#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    /// Invalid signature
    #[error("Invalid webhook signature")]
    InvalidSignature,
    
    /// Missing signature header
    #[error("Missing signature header")]
    MissingSignature,
    
    /// Invalid request body
    #[error("Invalid request body: {0}")]
    InvalidBody(String),
    
    /// Handler error
    #[error("Handler error: {0}")]
    HandlerError(Box<dyn std::error::Error + Send + Sync>),
    
    /// Timestamp validation failed
    #[error("Timestamp validation failed: {0}")]
    InvalidTimestamp(String),
}