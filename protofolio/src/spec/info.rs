//! Information and server definitions
//!
//! This module contains types related to API information and server definitions.

use crate::spec::SecurityRequirement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

/// Server definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// Server URL
    pub url: String,
    
    /// Protocol used (e.g., "nats", "kafka", "mqtt")
    pub protocol: String,
    
    /// Server description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Security requirements for this server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
}

