//! Webhook signature verification for ZITADEL Actions v3

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::actions::WebhookError;

type HmacSha256 = Hmac<Sha256>;

/// Webhook signature verifier
/// 
/// Verifies HMAC-SHA256 signatures for ZITADEL webhook requests
#[derive(Clone)]
pub struct WebhookVerifier {
    secret: String,
    max_timestamp_age: Option<u64>,
}

impl WebhookVerifier {
    /// Create a new webhook verifier with the given secret
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
            max_timestamp_age: Some(300), // 5 minutes default
        }
    }
    
    /// Set the maximum age for timestamp validation (in seconds)
    /// 
    /// Set to None to disable timestamp validation
    pub fn with_max_timestamp_age(mut self, seconds: Option<u64>) -> Self {
        self.max_timestamp_age = seconds;
        self
    }
    
    /// Verify a webhook signature
    /// 
    /// # Arguments
    /// 
    /// * `body` - The raw request body bytes
    /// * `signature` - The signature from the X-Zitadel-Signature header
    /// * `timestamp` - Optional timestamp from the X-Zitadel-Timestamp header
    pub fn verify(
        &self,
        body: &[u8],
        signature: &str,
        timestamp: Option<&str>,
    ) -> Result<(), WebhookError> {
        // Verify timestamp if provided
        if let Some(ts) = timestamp {
            self.verify_timestamp(ts)?;
        }
        
        // Create HMAC
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");
        
        // If timestamp is provided, include it in the signature
        if let Some(ts) = timestamp {
            mac.update(ts.as_bytes());
            mac.update(b".");
        }
        
        mac.update(body);
        
        // Verify signature
        let expected = mac.finalize();
        let expected_hex = hex::encode(expected.into_bytes());
        
        // Constant-time comparison
        if !constant_time_eq(&expected_hex, signature) {
            return Err(WebhookError::InvalidSignature);
        }
        
        Ok(())
    }
    
    /// Verify timestamp is within acceptable range
    fn verify_timestamp(&self, timestamp: &str) -> Result<(), WebhookError> {
        let Some(max_age) = self.max_timestamp_age else {
            return Ok(());
        };
        
        let ts = timestamp
            .parse::<u64>()
            .map_err(|_| WebhookError::InvalidTimestamp("Invalid timestamp format".to_string()))?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| WebhookError::InvalidTimestamp("System time error".to_string()))?
            .as_secs();
        
        let age = now.saturating_sub(ts);
        
        if age > max_age {
            return Err(WebhookError::InvalidTimestamp(
                format!("Timestamp too old: {} seconds", age)
            ));
        }
        
        // Also check for future timestamps (with 60 second tolerance)
        if ts > now + 60 {
            return Err(WebhookError::InvalidTimestamp(
                "Timestamp is in the future".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Constant-time string comparison
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    
    let mut result = 0u8;
    for i in 0..a.len() {
        result |= a_bytes[i] ^ b_bytes[i];
    }
    
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signature_verification() {
        let verifier = WebhookVerifier::new("test-secret");
        let body = b"test body";
        
        // Generate expected signature
        let mut mac = HmacSha256::new_from_slice(b"test-secret").unwrap();
        mac.update(body);
        let expected = hex::encode(mac.finalize().into_bytes());
        
        // Should verify correctly
        assert!(verifier.verify(body, &expected, None).is_ok());
        
        // Should fail with wrong signature
        assert!(verifier.verify(body, "wrong", None).is_err());
    }
    
    #[test]
    fn test_signature_with_timestamp() {
        let verifier = WebhookVerifier::new("test-secret");
        let body = b"test body";
        let timestamp = "1234567890";
        
        // Generate expected signature with timestamp
        let mut mac = HmacSha256::new_from_slice(b"test-secret").unwrap();
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(body);
        let expected = hex::encode(mac.finalize().into_bytes());
        
        // Should verify correctly
        assert!(verifier.verify(body, &expected, Some(timestamp)).is_ok());
    }
    
    #[test]
    fn test_timestamp_validation() {
        let verifier = WebhookVerifier::new("test-secret")
            .with_max_timestamp_age(Some(300)); // 5 minutes
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Current timestamp should be valid
        assert!(verifier.verify_timestamp(&now.to_string()).is_ok());
        
        // Old timestamp should fail
        let old = now - 400; // 400 seconds ago
        assert!(verifier.verify_timestamp(&old.to_string()).is_err());
        
        // Future timestamp should fail
        let future = now + 120; // 2 minutes in future
        assert!(verifier.verify_timestamp(&future.to_string()).is_err());
    }
}