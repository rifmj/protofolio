//! Reusable components
//!
//! This module contains types for reusable AsyncAPI components.

use crate::spec::{Message, MessageTrait, OperationTrait, Parameter, SecurityScheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reusable components
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Components {
    /// Message components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<HashMap<String, Message>>,

    /// Schema components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<HashMap<String, serde_json::Value>>,

    /// Security scheme components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<HashMap<String, SecurityScheme>>,

    /// Parameter components (reusable parameter definitions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Parameter>>,

    /// Channel binding components (reusable channel bindings)
    #[serde(rename = "channelBindings", skip_serializing_if = "Option::is_none")]
    pub channel_bindings: Option<HashMap<String, serde_json::Value>>,

    /// Message binding components (reusable message bindings)
    #[serde(rename = "messageBindings", skip_serializing_if = "Option::is_none")]
    pub message_bindings: Option<HashMap<String, serde_json::Value>>,

    /// Server binding components (reusable server bindings)
    #[serde(rename = "serverBindings", skip_serializing_if = "Option::is_none")]
    pub server_bindings: Option<HashMap<String, serde_json::Value>>,

    /// Operation trait components (reusable operation traits)
    #[serde(rename = "operationTraits", skip_serializing_if = "Option::is_none")]
    pub operation_traits: Option<HashMap<String, OperationTrait>>,

    /// Message trait components (reusable message traits)
    #[serde(rename = "messageTraits", skip_serializing_if = "Option::is_none")]
    pub message_traits: Option<HashMap<String, MessageTrait>>,
}
