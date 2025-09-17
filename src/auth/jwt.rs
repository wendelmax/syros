use crate::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
    pub iss: String,  // Issuer
    pub aud: String,  // Audience
    pub role: String, // User role
}

impl Claims {
    pub fn new(user_id: String, role: String, expiration_hours: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Self {
            sub: user_id,
            exp: now + (expiration_hours * 3600) as usize,
            iat: now,
            iss: "syros-platform".to_string(),
            aud: "syros-api".to_string(),
            role,
        }
    }
}

#[derive(Clone)]
pub struct JwtAuth {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtAuth {
    pub fn new(secret: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&["syros-platform"]);
        validation.set_audience(&["syros-api"]);

        Self {
            encoding_key,
            decoding_key,
            validation,
        }
    }

    pub fn generate_token(
        &self,
        user_id: String,
        role: String,
        expiration_hours: u64,
    ) -> Result<String> {
        let claims = Claims::new(user_id, role, expiration_hours);
        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| crate::SyrosError::ConfigError(format!("JWT encoding error: {}", e)))?;
        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|e| crate::SyrosError::ConfigError(format!("JWT validation error: {}", e)))?;
        Ok(token_data.claims)
    }

    pub fn extract_token_from_header(auth_header: &str) -> Option<String> {
        if auth_header.starts_with("Bearer ") {
            Some(auth_header[7..].to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_validation() {
        let jwt_auth = JwtAuth::new("test-secret");
        let user_id = "test-user-123".to_string();
        let role = "admin".to_string();

        // Generate token
        let token = jwt_auth
            .generate_token(user_id.clone(), role.clone(), 1)
            .unwrap();
        assert!(!token.is_empty());

        // Validate token
        let claims = jwt_auth.validate_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, role);
        assert_eq!(claims.iss, "syros-platform");
        assert_eq!(claims.aud, "syros-api");
    }

    #[test]
    fn test_token_extraction() {
        let header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
        let token = JwtAuth::extract_token_from_header(header);
        assert_eq!(
            token,
            Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string())
        );

        let invalid_header = "Invalid header";
        let token = JwtAuth::extract_token_from_header(invalid_header);
        assert_eq!(token, None);
    }
}
