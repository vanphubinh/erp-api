use derive_more::{AsRef, Deref, Display};
use serde::{Deserialize, Serialize};
use shared::DomainError;
use utoipa::ToSchema;

/// Party type enum - company or person
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PartyType {
    Company,
    Person,
}

impl PartyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PartyType::Company => "company",
            PartyType::Person => "person",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "company" => Ok(PartyType::Company),
            "person" => Ok(PartyType::Person),
            _ => Err(DomainError::InvalidValue(format!(
                "Invalid party type: {}. Must be 'company' or 'person'",
                s
            ))),
        }
    }
}

impl std::fmt::Display for PartyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Display name with validation (required field)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref, ToSchema)]
#[schema(value_type = String, example = "Acme Corporation")]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into().trim().to_string();
        if name.is_empty() {
            return Err(DomainError::InvalidValue(
                "Display name cannot be empty".to_string(),
            ));
        }
        if name.len() > 255 {
            return Err(DomainError::InvalidValue(
                "Display name too long (max 255 chars)".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Legal name with validation (optional field)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref, ToSchema)]
#[schema(value_type = String, example = "Acme Corporation Ltd.")]
pub struct LegalName(String);

impl LegalName {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into().trim().to_string();
        if name.is_empty() {
            return Err(DomainError::InvalidValue(
                "Legal name cannot be empty".to_string(),
            ));
        }
        if name.len() > 255 {
            return Err(DomainError::InvalidValue(
                "Legal name too long (max 255 chars)".to_string(),
            ));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Tax Identification Number (TIN)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref, ToSchema)]
#[schema(value_type = String, example = "0123456789")]
pub struct Tin(String);

impl Tin {
    pub fn new(tin: impl Into<String>) -> Result<Self, DomainError> {
        let tin = tin.into().trim().to_string();
        if tin.is_empty() {
            return Err(DomainError::InvalidValue(
                "TIN cannot be empty".to_string(),
            ));
        }
        if tin.len() > 50 {
            return Err(DomainError::InvalidValue(
                "TIN too long (max 50 chars)".to_string(),
            ));
        }
        Ok(Self(tin))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Business Registration Number
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, Deref, ToSchema)]
#[schema(value_type = String, example = "BRN-12345")]
pub struct RegistrationNumber(String);

impl RegistrationNumber {
    pub fn new(number: impl Into<String>) -> Result<Self, DomainError> {
        let number = number.into().trim().to_string();
        if number.is_empty() {
            return Err(DomainError::InvalidValue(
                "Registration number cannot be empty".to_string(),
            ));
        }
        if number.len() > 100 {
            return Err(DomainError::InvalidValue(
                "Registration number too long (max 100 chars)".to_string(),
            ));
        }
        Ok(Self(number))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod party_type {
        use super::*;

        #[test]
        fn from_str_accepts_company() {
            assert_eq!(PartyType::from_str("company").unwrap(), PartyType::Company);
            assert_eq!(PartyType::from_str("COMPANY").unwrap(), PartyType::Company);
            assert_eq!(PartyType::from_str("Company").unwrap(), PartyType::Company);
        }

        #[test]
        fn from_str_accepts_person() {
            assert_eq!(PartyType::from_str("person").unwrap(), PartyType::Person);
            assert_eq!(PartyType::from_str("PERSON").unwrap(), PartyType::Person);
            assert_eq!(PartyType::from_str("Person").unwrap(), PartyType::Person);
        }

        #[test]
        fn from_str_rejects_invalid() {
            assert!(PartyType::from_str("invalid").is_err());
            assert!(PartyType::from_str("").is_err());
        }

        #[test]
        fn as_str_returns_lowercase() {
            assert_eq!(PartyType::Company.as_str(), "company");
            assert_eq!(PartyType::Person.as_str(), "person");
        }
    }

    mod display_name {
        use super::*;

        #[test]
        fn accepts_valid_name() {
            let name = DisplayName::new("Acme Corp").unwrap();
            assert_eq!(name.value(), "Acme Corp");
        }

        #[test]
        fn trims_whitespace() {
            let name = DisplayName::new("  Acme Corp  ").unwrap();
            assert_eq!(name.value(), "Acme Corp");
        }

        #[test]
        fn rejects_empty_name() {
            assert!(DisplayName::new("").is_err());
            assert!(DisplayName::new("   ").is_err());
        }

        #[test]
        fn rejects_too_long_name() {
            let long_name = "a".repeat(256);
            assert!(DisplayName::new(long_name).is_err());
        }

        #[test]
        fn accepts_max_length_name() {
            let max_name = "a".repeat(255);
            assert!(DisplayName::new(max_name).is_ok());
        }
    }

    mod legal_name {
        use super::*;

        #[test]
        fn accepts_valid_name() {
            let name = LegalName::new("Acme Corporation Ltd.").unwrap();
            assert_eq!(name.value(), "Acme Corporation Ltd.");
        }

        #[test]
        fn trims_whitespace() {
            let name = LegalName::new("  Acme Corp  ").unwrap();
            assert_eq!(name.value(), "Acme Corp");
        }

        #[test]
        fn rejects_empty_name() {
            assert!(LegalName::new("").is_err());
            assert!(LegalName::new("   ").is_err());
        }
    }

    mod tin {
        use super::*;

        #[test]
        fn accepts_valid_tin() {
            let tin = Tin::new("0123456789").unwrap();
            assert_eq!(tin.value(), "0123456789");
        }

        #[test]
        fn trims_whitespace() {
            let tin = Tin::new("  0123456789  ").unwrap();
            assert_eq!(tin.value(), "0123456789");
        }

        #[test]
        fn rejects_empty_tin() {
            assert!(Tin::new("").is_err());
            assert!(Tin::new("   ").is_err());
        }

        #[test]
        fn rejects_too_long_tin() {
            let long_tin = "a".repeat(51);
            assert!(Tin::new(long_tin).is_err());
        }
    }

    mod registration_number {
        use super::*;

        #[test]
        fn accepts_valid_number() {
            let num = RegistrationNumber::new("BRN-12345").unwrap();
            assert_eq!(num.value(), "BRN-12345");
        }

        #[test]
        fn trims_whitespace() {
            let num = RegistrationNumber::new("  BRN-12345  ").unwrap();
            assert_eq!(num.value(), "BRN-12345");
        }

        #[test]
        fn rejects_empty_number() {
            assert!(RegistrationNumber::new("").is_err());
            assert!(RegistrationNumber::new("   ").is_err());
        }

        #[test]
        fn rejects_too_long_number() {
            let long_num = "a".repeat(101);
            assert!(RegistrationNumber::new(long_num).is_err());
        }
    }
}
