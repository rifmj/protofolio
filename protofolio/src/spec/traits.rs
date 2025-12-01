//! Operation and message trait definitions
//!
//! Traits are reusable sets of properties that can be applied to operations or messages.
//! They allow you to define common patterns once and reference them multiple times.

use crate::spec::{CorrelationId, ExternalDocumentation, MessagePayload, Tag};
use serde::{Deserialize, Serialize};

/// Operation trait definition
///
/// Operation traits define reusable properties that can be applied to operations.
/// When an operation references a trait, the trait's properties are merged with
/// the operation's own properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationTrait {
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

    /// Protocol-specific operation bindings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<serde_json::Value>,
}

/// Message trait definition
///
/// Message traits define reusable properties that can be applied to messages.
/// When a message references a trait, the trait's properties are merged with
/// the message's own properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageTrait {
    /// Message headers schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessagePayload>,

    /// Correlation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<CorrelationId>,

    /// Content type (default: "application/json")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

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

    /// Message tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// External documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// Message examples
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<serde_json::Value>>,

    /// Protocol-specific message bindings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<serde_json::Value>,
}
