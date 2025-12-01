//! Information and server definitions
//!
//! This module contains types related to API information and server definitions.

use crate::spec::SecurityRequirement;
use serde::{Deserialize, Serialize};

/// External documentation reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    /// URL to the external documentation
    pub url: String,

    /// Description of the external documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Information about the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    /// Title of the API
    pub title: String,

    /// Version of the API
    pub version: String,

    /// Description of the API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// External documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

/// Server variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    /// Enum of possible values for this variable
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,

    /// Default value for this variable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// Description of this variable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Example value for this variable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
}

/// Server definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// Server URL (may contain variables like {host} or {port})
    pub url: String,

    /// Protocol used (e.g., "nats", "kafka", "mqtt")
    pub protocol: String,

    /// Server description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Security requirements for this server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,

    /// Server variables (for templated URLs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, ServerVariable>>,
}
