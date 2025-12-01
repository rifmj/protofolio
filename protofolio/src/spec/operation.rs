//! Operation definitions
//!
//! This module contains types related to operations (send/receive actions).

use crate::spec::{ExternalDocumentation, Tag};
use serde::{Deserialize, Serialize};

/// Operation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// Unique operation identifier
    pub operation_id: String,

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

    /// Operation traits (reusable operation properties)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<OperationTraitOrRef>>,

    /// Protocol-specific operation bindings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<OperationBindingsOrRef>,
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

/// Operation trait or reference to a component operation trait
///
/// In AsyncAPI 3.0, operation traits can be either:
/// - Inline trait definitions
/// - References to reusable components using `$ref`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperationTraitOrRef {
    /// Inline operation trait definition
    Trait(crate::spec::OperationTrait),
    /// Reference to a component operation trait
    Ref(MessageReference),
}

impl OperationTraitOrRef {
    /// Create an OperationTraitOrRef from an inline OperationTrait
    pub fn trait_(trait_: crate::spec::OperationTrait) -> Self {
        Self::Trait(trait_)
    }

    /// Create an OperationTraitOrRef from a component reference
    pub fn component_ref(component_name: &str) -> Self {
        Self::Ref(MessageReference {
            ref_path: format!("#/components/operationTraits/{}", component_name),
        })
    }
}

/// Operation bindings or reference to component bindings
///
/// In AsyncAPI 3.0, operation bindings can be either:
/// - Inline bindings (JSON object)
/// - References to reusable component bindings using `$ref`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperationBindingsOrRef {
    /// Inline bindings (JSON object)
    Bindings(serde_json::Value),
    /// Reference to component bindings
    Ref(MessageReference),
}

impl OperationBindingsOrRef {
    /// Create OperationBindingsOrRef from inline bindings
    pub fn bindings(bindings: serde_json::Value) -> Self {
        Self::Bindings(bindings)
    }

    /// Create OperationBindingsOrRef from a component reference
    pub fn component_ref(component_name: &str) -> Self {
        Self::Ref(MessageReference {
            ref_path: format!("#/components/operationBindings/{}", component_name),
        })
    }
}
