//! Security scheme definitions
//!
//! This module contains types for AsyncAPI 3.0 security schemes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security scheme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SecurityScheme {
    /// User/password authentication
    #[serde(rename = "userPassword")]
    UserPassword {
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// API key authentication
    #[serde(rename = "apiKey")]
    ApiKey {
        /// Location of the API key (e.g., "user", "query", "header", "cookie")
        #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
        in_: Option<String>,
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// HTTP authentication
    #[serde(rename = "http")]
    Http {
        /// HTTP authentication scheme (e.g., "basic", "bearer", "digest")
        scheme: String,
        /// Bearer format (for bearer scheme)
        #[serde(skip_serializing_if = "Option::is_none")]
        bearer_format: Option<String>,
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// OAuth2 authentication
    #[serde(rename = "oauth2")]
    OAuth2 {
        /// OAuth2 flows
        flows: OAuth2Flows,
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// OpenID Connect authentication
    #[serde(rename = "openIdConnect")]
    OpenIdConnect {
        /// OpenID Connect URL
        open_id_connect_url: String,
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// X.509 certificate authentication
    #[serde(rename = "X509")]
    X509 {
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// Symmetric encryption
    #[serde(rename = "symmetricEncryption")]
    SymmetricEncryption {
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// Asymmetric encryption
    #[serde(rename = "asymmetricEncryption")]
    AsymmetricEncryption {
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// HTTP API key authentication
    #[serde(rename = "httpApiKey")]
    HttpApiKey {
        /// Name of the header, query or cookie parameter
        name: String,
        /// Location of the API key (e.g., "header", "query", "cookie")
        #[serde(rename = "in")]
        in_: String,
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    
    /// Mutual TLS authentication
    #[serde(rename = "mutualTLS")]
    MutualTls {
        /// Description of the security scheme
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

/// OAuth2 flows
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flows {
    /// Authorization code flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuth2Flow>,
    
    /// Client credentials flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuth2Flow>,
    
    /// Implicit flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuth2Flow>,
    
    /// Password flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuth2Flow>,
}

/// OAuth2 flow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flow {
    /// Authorization URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,
    
    /// Token URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
    
    /// Refresh URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    
    /// Scopes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<HashMap<String, String>>,
}

/// Security requirement (map of scheme name to scopes/requirements)
pub type SecurityRequirement = HashMap<String, Vec<String>>;

