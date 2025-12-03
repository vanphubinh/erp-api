use derive_more::{AsRef, Deref, Display};
use serde::{Deserialize, Serialize};
use shared::DomainError;
use utoipa::ToSchema;

/// Organization name with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref, ToSchema)]
#[schema(value_type = String, example = "Acme Corporation")]
pub struct OrganizationName(String);

impl OrganizationName {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into().trim().to_string();
        if name.is_empty() {
            return Err(DomainError::InvalidValue(
                "Organization name cannot be empty".to_string(),
            ));
        }
        if name.len() > 255 {
            return Err(DomainError::InvalidValue(
                "Organization name too long (max 255 chars)".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Email address with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref, ToSchema)]
#[schema(value_type = String, example = "contact@acme.com")]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, DomainError> {
        let email = email.into().trim().to_string();
        if !email.contains('@') || email.len() < 3 {
            return Err(DomainError::InvalidValue("Invalid email".to_string()));
        }
        Ok(Self(email))
    }
}

/// Phone number with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref, ToSchema)]
#[schema(value_type = String, example = "+1-555-0100")]
pub struct Phone(String);

impl Phone {
    pub fn new(phone: impl Into<String>) -> Result<Self, DomainError> {
        let phone = phone.into().trim().to_string();
        if phone.is_empty() {
            return Err(DomainError::InvalidValue(
                "Phone cannot be empty".to_string(),
            ));
        }
        Ok(Self(phone))
    }
}

/// URL with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref, ToSchema)]
#[schema(value_type = String, example = "https://acme.com")]
pub struct Url(String);

impl Url {
    pub fn new(url: impl Into<String>) -> Result<Self, DomainError> {
        let url = url.into().trim().to_string();
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(DomainError::InvalidValue("Invalid URL".to_string()));
        }
        Ok(Self(url))
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod organization_name {
        use super::*;

        #[test]
        fn accepts_valid_name() {
            let name = OrganizationName::new("Acme Corp").unwrap();
            assert_eq!(name.value(), "Acme Corp");
        }

        #[test]
        fn trims_whitespace() {
            let name = OrganizationName::new("  Acme Corp  ").unwrap();
            assert_eq!(name.value(), "Acme Corp");
        }

        #[test]
        fn rejects_empty_name() {
            assert!(OrganizationName::new("").is_err());
            assert!(OrganizationName::new("   ").is_err());
        }

        #[test]
        fn rejects_too_long_name() {
            let long_name = "a".repeat(256);
            assert!(OrganizationName::new(long_name).is_err());
        }

        #[test]
        fn accepts_max_length_name() {
            let max_name = "a".repeat(255);
            assert!(OrganizationName::new(max_name).is_ok());
        }
    }

    mod email {
        use super::*;

        #[test]
        fn accepts_valid_email() {
            assert!(Email::new("test@example.com").is_ok());
            assert!(Email::new("user@domain.org").is_ok());
        }

        #[test]
        fn rejects_email_without_at() {
            assert!(Email::new("invalid-email").is_err());
        }

        #[test]
        fn rejects_too_short_email() {
            assert!(Email::new("a@").is_err());
        }

        #[test]
        fn trims_whitespace() {
            let email = Email::new("  test@example.com  ").unwrap();
            assert_eq!(&*email, "test@example.com");
        }
    }

    mod phone {
        use super::*;

        #[test]
        fn accepts_valid_phone() {
            assert!(Phone::new("+1-555-0100").is_ok());
            assert!(Phone::new("555-0100").is_ok());
        }

        #[test]
        fn rejects_empty_phone() {
            assert!(Phone::new("").is_err());
            assert!(Phone::new("   ").is_err());
        }

        #[test]
        fn trims_whitespace() {
            let phone = Phone::new("  +1-555-0100  ").unwrap();
            assert_eq!(&*phone, "+1-555-0100");
        }
    }

    mod url {
        use super::*;

        #[test]
        fn accepts_https_url() {
            assert!(Url::new("https://example.com").is_ok());
        }

        #[test]
        fn accepts_http_url() {
            assert!(Url::new("http://example.com").is_ok());
        }

        #[test]
        fn rejects_invalid_url() {
            assert!(Url::new("example.com").is_err());
            assert!(Url::new("ftp://example.com").is_err());
        }

        #[test]
        fn trims_whitespace() {
            let url = Url::new("  https://example.com  ").unwrap();
            assert_eq!(&*url, "https://example.com");
        }
    }
}
