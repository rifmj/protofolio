//! Reusable components
//!
//! This module contains types for reusable AsyncAPI components.

use crate::spec::{Message, SecurityScheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reusable components (for future expansion)
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
}
