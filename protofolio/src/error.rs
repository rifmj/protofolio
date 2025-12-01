//! Error types for protofolio
//!
//! This module provides centralized error types used throughout the crate.

use thiserror::Error;

/// Error type for schema generation
#[derive(Debug, Error, Clone, PartialEq)]
pub enum SchemaError {
    #[error("Failed to serialize schema to JSON: {0}\n\nHint: This is typically an internal error. Ensure your types are properly serializable")]
    Serialization(String),
}

impl From<serde_json::Error> for SchemaError {
    fn from(err: serde_json::Error) -> Self {
        SchemaError::Serialization(err.to_string())
    }
}

/// Error type for AsyncAPI specification validation
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ValidationError {
    #[error("Missing required field: {0}\n\nHint: Add the missing field to your #[asyncapi] attribute. For info fields, use info(title = \"...\", version = \"...\")")]
    MissingRequiredField(String),

    #[error("Invalid AsyncAPI version: {0}. Expected 3.0.0\n\nHint: This library only supports AsyncAPI 3.0.0. The version field is automatically set and should not be modified")]
    InvalidAsyncApiVersion(String),

    #[error("Channel '{0}' referenced in message but not defined in channels\n\nHint: Add '{0}' to the channels(...) list in your #[asyncapi] attribute")]
    InvalidChannelReference(String),

    #[error("Server '{0}' referenced in channel but not defined in servers\n\nHint: Add the server to servers(...) in your #[asyncapi] attribute, or remove the server reference from the channel")]
    InvalidServerReference(String),

    #[error("Invalid schema: {0}\n\nHint: Ensure all message types implement JsonSchema and have valid schemas")]
    InvalidSchema(String),

    #[error("Empty channels: specification must have at least one channel\n\nHint: Add at least one channel to channels(...) in your #[asyncapi] attribute")]
    EmptyChannels,

    #[error("Channel '{0}' has no messages\n\nHint: Add at least one message type to messages(...) in your #[asyncapi] attribute that uses this channel")]
    ChannelWithoutMessages(String),

    #[error("Duplicate message ID: {0}\n\nHint: Each message must have a unique messageId. Update one of the conflicting messages to use a different messageId")]
    DuplicateMessageId(String),

    #[error("Unsupported protocol: {protocol}\n\nHint: Supported protocols: {supported:?}. Enable the corresponding feature flag (e.g., `features = [\"nats\"]`) in your Cargo.toml")]
    UnsupportedProtocol {
        protocol: String,
        supported: Vec<String>,
    },

    #[error("Invalid protocol: {0}\n\nHint: Check that the protocol name matches exactly (case-sensitive) and the corresponding feature flag is enabled")]
    InvalidProtocol(String),

    #[error("Schema generation failed for type '{0}': {1}\n\nHint: Ensure the type implements JsonSchema (usually via #[derive(JsonSchema)]) and all nested types also implement JsonSchema")]
    SchemaGenerationFailed(String, String),

    #[error("Message '{message}' not found in channel '{channel}'\n\nHint: Ensure the message type is included in messages(...) in your #[asyncapi] attribute and uses the correct channel")]
    MessageNotFound {
        channel: String,
        message: String,
    },
}

