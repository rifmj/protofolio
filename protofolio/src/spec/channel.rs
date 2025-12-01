//! Channel and message definitions
//!
//! This module contains types related to channels, messages, and their metadata.

use crate::spec::operation::MessageReference;
use crate::spec::ExternalDocumentation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Message or reference to a component message
///
/// In AsyncAPI 3.0, messages in channels can be either:
/// - Inline message definitions
/// - References to reusable components using `$ref`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageOrRef {
    /// Inline message definition
    Message(Message),
    /// Reference to a component message
    Ref(MessageReference),
}

impl MessageOrRef {
    /// Create a MessageOrRef from an inline Message
    pub fn message(message: Message) -> Self {
        Self::Message(message)
    }

    /// Create a MessageOrRef from a component reference
    pub fn component_ref(component_name: &str) -> Self {
        Self::Ref(MessageReference {
            ref_path: format!("#/components/messages/{}", component_name),
        })
    }

    /// Create a MessageOrRef from a channel message reference
    pub fn channel_ref(channel_name: &str, message_name: &str) -> Self {
        Self::Ref(MessageReference {
            ref_path: format!("#/channels/{}/messages/{}", channel_name, message_name),
        })
    }

    /// Check if this is a reference to a component
    pub fn is_component_ref(&self) -> bool {
        matches!(self, Self::Ref(ref_msg) if ref_msg.ref_path.starts_with("#/components/messages/"))
    }

    /// Get the component name if this is a component reference
    pub fn component_name(&self) -> Option<&str> {
        match self {
            Self::Ref(ref_msg) if ref_msg.ref_path.starts_with("#/components/messages/") => {
                ref_msg.ref_path.strip_prefix("#/components/messages/")
            }
            _ => None,
        }
    }
}

/// Channel definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Channel address (required in AsyncAPI 3.0)
    pub address: String,

    /// Channel description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Messages available on this channel
    /// Can contain both inline messages and references to component messages
    pub messages: HashMap<String, MessageOrRef>,

    /// Servers this channel is available on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<String>>,

    /// Channel parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Parameter>>,

    /// Protocol-specific bindings (inline or reference to component)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<ChannelBindingsOrRef>,
}

/// Correlation ID definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationId {
    /// Runtime expression specifying where the correlation ID is located within the message
    pub location: String,

    /// Description of the correlation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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

    /// External documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// Message examples
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<serde_json::Value>>,

    /// Message headers schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<MessagePayload>,

    /// Correlation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<CorrelationId>,

    /// Message traits (reusable message properties)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<MessageTraitOrRef>>,

    /// Protocol-specific message bindings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<MessageBindingsOrRef>,
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

/// Channel bindings or reference to component bindings
///
/// In AsyncAPI 3.0, channel bindings can be either:
/// - Inline bindings (JSON object)
/// - References to reusable component bindings using `$ref`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelBindingsOrRef {
    /// Inline bindings (JSON object)
    Bindings(serde_json::Value),
    /// Reference to component bindings
    Ref(crate::spec::operation::MessageReference),
}

impl ChannelBindingsOrRef {
    /// Create ChannelBindingsOrRef from inline bindings
    pub fn bindings(bindings: serde_json::Value) -> Self {
        Self::Bindings(bindings)
    }

    /// Create ChannelBindingsOrRef from a component reference
    pub fn component_ref(component_name: &str) -> Self {
        Self::Ref(crate::spec::operation::MessageReference {
            ref_path: format!("#/components/channelBindings/{}", component_name),
        })
    }
}

/// Message trait or reference to a component message trait
///
/// In AsyncAPI 3.0, message traits can be either:
/// - Inline trait definitions
/// - References to reusable components using `$ref`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageTraitOrRef {
    /// Inline message trait definition
    Trait(crate::spec::MessageTrait),
    /// Reference to a component message trait
    Ref(crate::spec::operation::MessageReference),
}

impl MessageTraitOrRef {
    /// Create a MessageTraitOrRef from an inline MessageTrait
    pub fn trait_(trait_: crate::spec::MessageTrait) -> Self {
        Self::Trait(trait_)
    }

    /// Create a MessageTraitOrRef from a component reference
    pub fn component_ref(component_name: &str) -> Self {
        Self::Ref(crate::spec::operation::MessageReference {
            ref_path: format!("#/components/messageTraits/{}", component_name),
        })
    }
}

/// Message bindings or reference to component bindings
///
/// In AsyncAPI 3.0, message bindings can be either:
/// - Inline bindings (JSON object)
/// - References to reusable component bindings using `$ref`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageBindingsOrRef {
    /// Inline bindings (JSON object)
    Bindings(serde_json::Value),
    /// Reference to component bindings
    Ref(crate::spec::operation::MessageReference),
}

impl MessageBindingsOrRef {
    /// Create MessageBindingsOrRef from inline bindings
    pub fn bindings(bindings: serde_json::Value) -> Self {
        Self::Bindings(bindings)
    }

    /// Create MessageBindingsOrRef from a component reference
    pub fn component_ref(component_name: &str) -> Self {
        Self::Ref(crate::spec::operation::MessageReference {
            ref_path: format!("#/components/messageBindings/{}", component_name),
        })
    }
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
