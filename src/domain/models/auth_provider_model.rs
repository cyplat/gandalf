/*
This module holds the model for supported auth providers
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    Local,
    Google,
    Microsoft,
    Apple,
    Facebook,
    Lti,
    Saml,
    Ldap,
    Custom,
}

impl Default for AuthProvider {
    fn default() -> Self {
        Self::Local
    }
}

impl std::str::FromStr for AuthProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(AuthProvider::Local),
            "google" => Ok(AuthProvider::Google),
            "microsoft" => Ok(AuthProvider::Microsoft),
            "apple" => Ok(AuthProvider::Apple),
            "facebook" => Ok(AuthProvider::Facebook),
            "lti" => Ok(AuthProvider::Lti),
            "saml" => Ok(AuthProvider::Saml),
            "ldap" => Ok(AuthProvider::Ldap),
            "custom" => Ok(AuthProvider::Custom),
            _ => Err(format!("Invalid auth provider: {}", s)),
        }
    }
}

impl ToString for AuthProvider {
    fn to_string(&self) -> String {
        match self {
            AuthProvider::Local => "local".to_string(),
            AuthProvider::Google => "google".to_string(),
            AuthProvider::Microsoft => "microsoft".to_string(),
            AuthProvider::Apple => "apple".to_string(),
            AuthProvider::Facebook => "facebook".to_string(),
            AuthProvider::Lti => "lti".to_string(),
            AuthProvider::Saml => "saml".to_string(),
            AuthProvider::Ldap => "ldap".to_string(),
            AuthProvider::Custom => "custom".to_string(),
        }
    }
}
