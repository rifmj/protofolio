//! AsyncAPI 3.0 specification data structures
//!
//! This module contains the core data structures for AsyncAPI 3.0.
//! Currently implements MVP subset, structured for full AsyncAPI 3.0 support.

mod info;
mod channel;
mod operation;
mod components;
mod security;

pub use info::*;
pub use channel::*;
pub use operation::*;
pub use components::*;
pub use security::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root AsyncAPI specification document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncApiSpec {
    /// AsyncAPI specification version (e.g., "3.0.0")
    pub asyncapi: String,
    
    /// Information about the API
    pub info: Info,
    
    /// Server definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Servers>,
    
    /// Channel definitions
    pub channels: Channels,
    
    /// Operation definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Operations>,
    
    /// Reusable components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
}

/// Channel definitions (map of channel name to Channel)
pub type Channels = HashMap<String, Channel>;

/// Message definitions (map of message name to Message)
pub type Messages = HashMap<String, Message>;

/// Server definitions (map of server name to Server)
pub type Servers = HashMap<String, Server>;

/// Operation definitions (map of operation ID to Operation)
pub type Operations = HashMap<String, Operation>;

