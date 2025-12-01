//! Channel and message definitions
//!
//! This module contains types related to channels, messages, and their metadata.

use crate::spec::Messages;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Channel definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Channel description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Messages available on this channel
    pub messages: Messages,
    
    /// Servers this channel is available on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<String>>,
    
    /// Channel parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Parameter>>,
    
    /// Protocol-specific bindings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<serde_json::Value>,
}

/// Message definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    
    /// Message name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Message title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    
    /// Message summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    
    /// Message description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Content type (default: "application/json")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    
    /// Message tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    
    /// Message payload schema
    pub payload: MessagePayload,
}

/// Message payload schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    /// JSON Schema for the payload
    #[serde(flatten)]
    pub schema: serde_json::Value,
}

/// Tag definition for messages and operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Tag {
    /// Tag name
    pub name: String,
    
    /// Tag description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Parameter definition for channels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Parameter {
    /// Parameter description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Parameter schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<serde_json::Value>,
    
    /// Parameter location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

