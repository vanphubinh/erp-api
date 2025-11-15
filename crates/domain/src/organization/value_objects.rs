use derive_more::{AsRef, Deref, Display};
use serde::{Deserialize, Serialize};
use shared::DomainError;

/// Organization name with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref)]
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

// Add more value objects with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref)]
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
