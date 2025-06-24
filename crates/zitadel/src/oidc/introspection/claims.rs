//! Simplified token claims structures for ZITADEL.
//!
//! This module provides user-friendly claim structures that flatten the complex
//! nested responses from ZITADEL into easy-to-use types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use openidconnect::TokenIntrospectionResponse;

use crate::oidc::introspection::ZitadelIntrospectionResponse;

/// Simplified ZITADEL token claims structure.
///
/// This structure provides a flattened view of ZITADEL token claims, making it
/// easier to work with than the raw introspection response. It includes standard
/// OIDC claims as well as ZITADEL-specific extensions.
///
/// # Example
///
/// ```no_run
/// use zitadel::oidc::introspection::claims::ZitadelClaims;
///
/// # fn example(claims: ZitadelClaims) {
/// // Access standard claims
/// println!("User ID: {}", claims.sub);
/// println!("Email: {:?}", claims.email);
///
/// // Check roles using built-in methods
/// if claims.has_role("admin") {
///     println!("User is an admin");
/// }
///
/// // Check project-specific roles
/// if claims.has_role_in_project("project123", "editor") {
///     println!("User can edit project123");
/// }
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZitadelClaims {
    // Standard OIDC claims
    /// Subject (user ID)
    pub sub: String,
    /// Issuer
    pub iss: String,
    /// Audiences
    pub aud: Vec<String>,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at time (Unix timestamp)
    pub iat: i64,
    /// Not before time (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    /// JWT ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    
    // User information
    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Whether email is verified
    #[serde(default)]
    pub email_verified: bool,
    /// Full name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Given name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// Family name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// Preferred username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    /// Locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    
    // Organization/resource owner info
    /// Organization ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    /// Organization name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    /// Primary organization domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_domain: Option<String>,
    /// Resource owner ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    /// Resource owner name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_name: Option<String>,
    
    // Roles and permissions
    /// All roles (flattened list)
    #[serde(default)]
    pub roles: Vec<String>,
    /// Project-specific roles (Project ID -> roles)
    #[serde(default)]
    pub project_roles: HashMap<String, Vec<String>>,
    /// Organization-specific roles (Org ID -> roles)
    #[serde(default)]
    pub org_roles: HashMap<String, Vec<String>>,
    
    // Token metadata
    /// OAuth scopes
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Client ID that requested the token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Whether the token is active
    #[serde(default = "default_true")]
    pub active: bool,
    
    // User metadata
    /// User metadata key-value pairs
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    
    // Additional custom claims
    /// Any additional claims not captured above
    #[serde(flatten)]
    pub custom_claims: HashMap<String, serde_json::Value>,
}

fn default_true() -> bool {
    true
}

impl ZitadelClaims {
    

    /// Checks if the token has a specific audience.
    pub fn has_audience(&self, audience: &str) -> bool {
        self.aud.iter().any(|a| a == audience)
    }

    /// Checks if the token has a specific scope.
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }


    /// Checks if the token has expired.
    ///
    /// Takes an optional leeway in seconds for clock skew.
    pub fn is_expired(&self, leeway: Option<u64>) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let leeway = leeway.unwrap_or(0) as i64;
        self.exp < now - leeway
    }

    /// Checks if the token is valid for use now.
    ///
    /// This checks both expiration and not-before times with optional leeway.
    pub fn is_valid_now(&self, leeway: Option<u64>) -> bool {
        if self.is_expired(leeway) {
            return false;
        }

        if let Some(nbf) = self.nbf {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            
            let leeway = leeway.unwrap_or(0) as i64;
            if nbf > now + leeway {
                return false;
            }
        }

        true
    }

    // RBAC methods

    /// Checks if the user has a specific role (in any context).
    pub fn has_role(&self, role: &str) -> bool {
        // Check global roles
        if self.roles.iter().any(|r| r == role) {
            return true;
        }

        // Check project roles
        for roles in self.project_roles.values() {
            if roles.iter().any(|r| r == role) {
                return true;
            }
        }

        // Check org roles
        for roles in self.org_roles.values() {
            if roles.iter().any(|r| r == role) {
                return true;
            }
        }

        false
    }


    /// Checks if the user has a specific role in a project.
    pub fn has_role_in_project(&self, project_id: &str, role: &str) -> bool {
        self.project_roles
            .get(project_id)
            .map(|roles| roles.iter().any(|r| r == role))
            .unwrap_or(false)
    }


    /// Checks if the user has a specific role in an organization.
    pub fn has_role_in_org(&self, org_id: &str, role: &str) -> bool {
        self.org_roles
            .get(org_id)
            .map(|roles| roles.iter().any(|r| r == role))
            .unwrap_or(false)
    }




}

impl From<ZitadelIntrospectionResponse> for ZitadelClaims {
    fn from(response: ZitadelIntrospectionResponse) -> Self {
        let extra = response.extra_fields();
        
        // Extract audiences
        let aud = response
            .aud()
            .map(|a| a.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        
        // Extract user info
        let user_id = response.sub().unwrap_or_default().to_string();
        let username = response.username().map(|s| s.to_string());
        let email = extra.email.clone();
        let email_verified = extra.email_verified.unwrap_or(false);
        let name = extra.name.clone();
        let given_name = extra.given_name.clone();
        let family_name = extra.family_name.clone();
        let preferred_username = extra.preferred_username.clone()
            .or_else(|| username.clone());
        let locale = extra.locale.clone();
        
        // Extract organization info from custom claims if available
        let org_id = extra.custom_claims.as_ref()
            .and_then(|claims| claims.get("org_id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let org_domain = extra.custom_claims.as_ref()
            .and_then(|claims| claims.get("org_domain"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let resource_owner = extra.resource_owner_id.clone();
        let resource_owner_name = extra.resource_owner_name.clone();
        
        // Extract and flatten roles
        let mut all_roles = Vec::new();
        let mut project_roles = HashMap::new();
        let mut org_roles = HashMap::new();
        
        // Process project roles
        if let Some(roles_claim) = &extra.project_roles {
            for (context, context_roles) in roles_claim {
                let roles_list: Vec<String> = context_roles
                    .iter()
                    .map(|(_, role_value)| role_value.clone())
                    .collect();
                
                all_roles.extend(roles_list.clone());
                
                // Try to determine if this is a project or org
                if context.contains("project") || context.chars().all(|c| c.is_numeric()) {
                    project_roles.insert(context.clone(), roles_list);
                } else {
                    org_roles.insert(context.clone(), roles_list);
                }
            }
        }
        
        // Remove duplicates from all_roles
        all_roles.sort();
        all_roles.dedup();
        
        // Extract scopes from custom claims if available
        let scopes = extra.custom_claims.as_ref()
            .and_then(|claims| claims.get("scope"))
            .and_then(|v| v.as_str())
            .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        
        // Extract metadata
        let metadata = extra.metadata.clone().unwrap_or_default();
        
        // Build custom claims from remaining fields
        let mut custom_claims = HashMap::new();
        if let Ok(value) = serde_json::to_value(&response) {
            if let Some(obj) = value.as_object() {
                // Add any fields we haven't explicitly handled
                for (key, val) in obj {
                    match key.as_str() {
                        // Skip fields we've already processed
                        "sub" | "iss" | "aud" | "exp" | "iat" | "nbf" | "jti" |
                        "active" | "scope" | "client_id" | "username" |
                        "urn:zitadel:iam:user:metadata" |
                        "urn:zitadel:iam:org:project:roles" |
                        "urn:zitadel:iam:org:domain:primary" |
                        "urn:zitadel:iam:org:id" |
                        "urn:zitadel:iam:user:resourceowner:id" |
                        "urn:zitadel:iam:user:resourceowner:name" => continue,
                        _ => {
                            custom_claims.insert(key.clone(), val.clone());
                        }
                    }
                }
            }
        }
        
        Self {
            sub: user_id,
            iss: response.iss().unwrap_or_default().to_string(),
            aud,
            exp: response.exp().map(|dt| dt.timestamp()).unwrap_or(0),
            iat: response.iat().map(|dt| dt.timestamp()).unwrap_or(0),
            nbf: response.nbf().map(|dt| dt.timestamp()),
            jti: response.jti().map(|s| s.to_string()),
            username,
            email,
            email_verified,
            name,
            given_name,
            family_name,
            preferred_username,
            locale,
            org_id,
            org_name: None, // Not available in introspection response
            org_domain,
            resource_owner,
            resource_owner_name,
            roles: all_roles,
            project_roles,
            org_roles,
            scopes,
            client_id: response.client_id().map(|s| s.to_string()),
            active: response.active(),
            metadata,
            custom_claims,
        }
    }
}

impl TryFrom<serde_json::Value> for ZitadelClaims {
    type Error = serde_json::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        // First try to deserialize as ZitadelIntrospectionResponse
        if let Ok(introspection) = serde_json::from_value::<ZitadelIntrospectionResponse>(value.clone()) {
            return Ok(introspection.into());
        }
        
        // Otherwise try direct deserialization
        serde_json::from_value(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audience_and_scope_helpers() {
        let mut claims = create_test_claims();
        claims.aud = vec!["aud1".to_string(), "aud2".to_string()];
        claims.scopes = vec!["read".to_string(), "write".to_string()];

        assert!(claims.has_audience("aud1"));
        assert!(claims.has_audience("aud2"));
        assert!(!claims.has_audience("aud3"));
        assert!(claims.has_scope("read"));
        assert!(!claims.has_scope("admin"));
    }

    #[test]
    fn test_role_checks() {
        let mut claims = create_test_claims();
        claims.roles = vec!["global_role".to_string()];
        claims.project_roles.insert("project1".to_string(), 
            vec!["editor".to_string(), "viewer".to_string()]);
        claims.org_roles.insert("org1".to_string(), 
            vec!["admin".to_string()]);

        // Test has_role
        assert!(claims.has_role("global_role"));
        assert!(claims.has_role("editor"));
        assert!(claims.has_role("admin"));
        assert!(!claims.has_role("nonexistent"));

        // Test project-specific roles
        assert!(claims.has_role_in_project("project1", "editor"));
        assert!(!claims.has_role_in_project("project1", "admin"));
        
        // Test org-specific roles
        assert!(claims.has_role_in_org("org1", "admin"));
        assert!(!claims.has_role_in_org("org1", "editor"));
    }

    #[test]
    fn test_expiration_check() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Expired token
        let expired_claims = ZitadelClaims {
            exp: now - 100,
            ..create_test_claims()
        };
        assert!(expired_claims.is_expired(None));
        assert!(expired_claims.is_expired(Some(50)));
        assert!(!expired_claims.is_expired(Some(200))); // With large leeway

        // Valid token
        let valid_claims = ZitadelClaims {
            exp: now + 100,
            ..create_test_claims()
        };
        assert!(!valid_claims.is_expired(None));
        assert!(!valid_claims.is_expired(Some(50)));
    }

    #[test]
    fn test_validity_check() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Valid token
        let valid_claims = ZitadelClaims {
            exp: now + 100,
            nbf: Some(now - 100),
            ..create_test_claims()
        };
        assert!(valid_claims.is_valid_now(None));

        // Not yet valid
        let not_yet_valid = ZitadelClaims {
            exp: now + 100,
            nbf: Some(now + 100),
            ..create_test_claims()
        };
        assert!(!not_yet_valid.is_valid_now(None));
        assert!(not_yet_valid.is_valid_now(Some(200))); // With leeway

        // Expired
        let expired = ZitadelClaims {
            exp: now - 100,
            nbf: Some(now - 200),
            ..create_test_claims()
        };
        assert!(!expired.is_valid_now(None));
    }

    fn create_test_claims() -> ZitadelClaims {
        ZitadelClaims {
            sub: "user123".to_string(),
            iss: "https://example.zitadel.cloud".to_string(),
            aud: vec!["test".to_string()],
            exp: 0,
            iat: 0,
            active: true,
            ..Default::default()
        }
    }
}