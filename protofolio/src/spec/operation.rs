//! Operation definitions
//!
//! This module contains types related to operations (send/receive actions).

use crate::spec::{ExternalDocumentation, Tag};
use serde::{Deserialize, Serialize};

/// Operation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// Operation action (send, receive)
    pub action: String,
    
    /// Channel reference
    pub channel: ChannelReference,
    
    /// Message references
    pub messages: Vec<MessageReference>,
    
    /// Operation summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    
    /// Operation description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Operation tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    
    /// External documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

/// Channel reference in operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelReference {
    /// Channel reference path
    #[serde(rename = "$ref")]
    pub ref_path: String,
}

/// Message reference in operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReference {
    /// Message reference path
    #[serde(rename = "$ref")]
    pub ref_path: String,
}

