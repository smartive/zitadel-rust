//! Helper functions and constants for ZITADEL Actions v3

use serde::Serialize;
use serde_json::Value;

use crate::actions::{ActionResponse, Claim, Metadata};

/// Common ZITADEL claim keys
pub mod claims {
    /// Resource owner (organization) ID
    pub const RESOURCE_OWNER_ID: &str = "urn:zitadel:iam:user:resourceowner:id";
    
    /// Resource owner name
    pub const RESOURCE_OWNER_NAME: &str = "urn:zitadel:iam:user:resourceowner:name";
    
    /// Resource owner primary domain
    pub const RESOURCE_OWNER_PRIMARY_DOMAIN: &str = "urn:zitadel:iam:user:resourceowner:primary_domain";
    
    /// Project roles
    pub const PROJECT_ROLES: &str = "urn:zitadel:iam:org:project:roles";
    
    /// Project-specific roles format
    pub const PROJECT_ROLES_FORMAT: &str = "urn:zitadel:iam:org:project:%s:roles";
    
    /// User metadata
    pub const USER_METADATA: &str = "urn:zitadel:iam:user:metadata";
    
    /// Action log format
    pub const ACTION_LOG_FORMAT: &str = "urn:zitadel:iam:action:%s:log";
}

impl ActionResponse {
    /// Add a claim to the response
    /// 
    /// # Example
    /// 
    /// ```
    /// use zitadel::actions::ActionResponse;
    /// 
    /// let response = ActionResponse::default()
    ///     .add_claim("custom_claim", "value")
    ///     .add_claim("roles", vec!["admin", "user"]);
    /// ```
    pub fn add_claim<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        self.append_claims.push(Claim {
            key: key.into(),
            value: serde_json::to_value(value).unwrap_or(Value::Null),
        });
        self
    }
    
    /// Add user metadata
    /// 
    /// # Example
    /// 
    /// ```
    /// use zitadel::actions::ActionResponse;
    /// 
    /// let response = ActionResponse::default()
    ///     .add_metadata("preference", "dark_mode")
    ///     .add_metadata("last_login", "2024-01-01");
    /// ```
    pub fn add_metadata<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        self.set_user_metadata.push(Metadata {
            key: key.into(),
            value: serde_json::to_value(value).unwrap_or(Value::Null),
        });
        self
    }
    
    /// Add a log entry
    /// 
    /// Log entries are added to the token as an array under a special claim key.
    /// 
    /// # Example
    /// 
    /// ```
    /// use zitadel::actions::ActionResponse;
    /// 
    /// let response = ActionResponse::default()
    ///     .add_log("Custom claim added successfully")
    ///     .add_log("User verified");
    /// ```
    pub fn add_log<S>(mut self, message: S) -> Self
    where
        S: Into<String>,
    {
        self.append_log_claims.push(message.into());
        self
    }
    
    /// Check if the response is empty (no modifications)
    pub fn is_empty(&self) -> bool {
        self.append_claims.is_empty()
            && self.set_user_metadata.is_empty()
            && self.append_log_claims.is_empty()
    }
    
    /// Create a response with a single claim
    pub fn with_claim<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        Self::default().add_claim(key, value)
    }
    
    /// Create a response with a single metadata entry
    pub fn with_metadata<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        Self::default().add_metadata(key, value)
    }
}

/// Helper to create a claim
pub fn claim<K, V>(key: K, value: V) -> Claim
where
    K: Into<String>,
    V: Serialize,
{
    Claim {
        key: key.into(),
        value: serde_json::to_value(value).unwrap_or(Value::Null),
    }
}

/// Helper to create metadata
pub fn metadata<K, V>(key: K, value: V) -> Metadata
where
    K: Into<String>,
    V: Serialize,
{
    Metadata {
        key: key.into(),
        value: serde_json::to_value(value).unwrap_or(Value::Null),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_response_builder() {
        let response = ActionResponse::default()
            .add_claim("test", "value")
            .add_metadata("key", 123)
            .add_log("Test log");
        
        assert_eq!(response.append_claims.len(), 1);
        assert_eq!(response.set_user_metadata.len(), 1);
        assert_eq!(response.append_log_claims.len(), 1);
    }
    
    #[test]
    fn test_empty_response() {
        let response = ActionResponse::default();
        assert!(response.is_empty());
        
        let response = response.add_claim("test", "value");
        assert!(!response.is_empty());
    }
}