//! Protocol binding validation helpers

use crate::error::ValidationError;
use crate::spec::{AsyncApiSpec, Channel};

/// Get the protocol for a channel based on its server references
pub(crate) fn get_channel_protocol(channel: &Channel, spec: &AsyncApiSpec) -> Option<String> {
    // Determine protocol from channel's server references
    if let Some(ref servers) = channel.servers {
        if let Some(ref spec_servers) = spec.servers {
            for server_name in servers {
                if let Some(server) = spec_servers.get(server_name) {
                    return Some(server.protocol.clone());
                }
            }
        }
    }
    None
}

/// Validate channel bindings match the protocol
pub(crate) fn validate_channel_bindings(
    protocol: &str,
    bindings: &serde_json::Value,
    channel_name: &str,
) -> Result<(), ValidationError> {
    match protocol {
        "kafka" => {
            // Validate Kafka bindings structure
            if !bindings.as_object().and_then(|o| o.get("kafka")).is_some() {
                return Err(ValidationError::InvalidSchema(format!(
                    "Channel '{}': Kafka channel bindings must have 'kafka' key",
                    channel_name
                )));
            }
        }
        "mqtt" => {
            // Validate MQTT bindings structure
            if !bindings.as_object().and_then(|o| o.get("mqtt")).is_some() {
                return Err(ValidationError::InvalidSchema(format!(
                    "Channel '{}': MQTT channel bindings must have 'mqtt' key",
                    channel_name
                )));
            }
        }
        "nats" => {
            // Validate NATS bindings structure
            if !bindings.as_object().and_then(|o| o.get("nats")).is_some() {
                return Err(ValidationError::InvalidSchema(format!(
                    "Channel '{}': NATS channel bindings must have 'nats' key",
                    channel_name
                )));
            }
        }
        _ => {
            // Unknown protocol, but we already validated protocol identifier
            // so we can skip binding validation
        }
    }
    Ok(())
}
