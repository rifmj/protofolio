//! AsyncAPI specification validator implementation

use crate::error::ValidationError;
use crate::spec::*;
use crate::types::ASYNCAPI_VERSION;
use crate::protocol;

/// Validate an AsyncAPI specification
///
/// Returns `Ok(())` if the specification is valid, or an error describing
/// what validation failed.
///
/// # Example
///
/// ```rust,no_run
/// use protofolio::{AsyncApi, validate_spec, ValidationError};
/// # use protofolio_derive::AsyncApi;
/// #
/// # #[derive(AsyncApi)]
/// # #[asyncapi(info(title = "Test", version = "1.0.0"), channels("events"), messages())]
/// # struct MyApi;
///
/// let spec = MyApi::asyncapi();
/// match validate_spec(&spec) {
///     Ok(()) => println!("Spec is valid!"),
///     Err(ValidationError::InvalidChannelReference(channel)) => {
///         eprintln!("Invalid channel reference: {}", channel);
///     }
///     Err(ValidationError::DuplicateMessageId(id)) => {
///         eprintln!("Duplicate message ID: {}", id);
///     }
///     Err(e) => eprintln!("Validation error: {}", e),
/// }
/// ```
pub fn validate_spec(spec: &AsyncApiSpec) -> Result<(), ValidationError> {
    // Validate AsyncAPI version
    if spec.asyncapi != ASYNCAPI_VERSION {
        return Err(ValidationError::InvalidAsyncApiVersion(spec.asyncapi.clone()));
    }

    // Validate info section
    if spec.info.title.is_empty() {
        return Err(ValidationError::MissingRequiredField("info.title".to_string()));
    }
    if spec.info.version.is_empty() {
        return Err(ValidationError::MissingRequiredField("info.version".to_string()));
    }

    // Validate channels
    if spec.channels.is_empty() {
        return Err(ValidationError::EmptyChannels);
    }

    // Collect server names if servers are defined
    let server_names: std::collections::HashSet<String> = spec
        .servers
        .as_ref()
        .map(|servers| servers.keys().cloned().collect())
        .unwrap_or_default();

    // Track message IDs for duplicate detection
    let mut message_ids = std::collections::HashSet::new();

    // Validate each channel
    for (channel_name, channel) in &spec.channels {
        // Check if channel has messages
        if channel.messages.is_empty() {
            return Err(ValidationError::ChannelWithoutMessages(channel_name.clone()));
        }

        // Validate server references in channel
        if let Some(ref channel_servers) = channel.servers {
            for server_name in channel_servers {
                if !server_names.contains(server_name) {
                    let available: Vec<_> = server_names.iter().collect();
                    let suggestion = if available.is_empty() {
                        format!("No servers defined. Add servers(...) to your #[asyncapi] attribute. Referenced server: '{}'", server_name)
                    } else {
                        format!("Server '{}' not found. Available servers: {:?}. Update your channel's server reference or add the server in servers(...)", server_name, available)
                    };
                    return Err(ValidationError::InvalidServerReference(format!("{}: {}", server_name, suggestion)));
                }
            }
        }

        // Validate messages in channel
        for (message_name, message) in &channel.messages {
            // Basic message validation - ensure payload schema exists
            if message.payload.schema.is_null() {
                return Err(ValidationError::InvalidSchema(format!(
                    "Message '{}' in channel '{}' has null schema",
                    message_name, channel_name
                )));
            }

            // Check for duplicate message IDs
            if let Some(ref msg_id) = message.message_id {
                if !message_ids.insert(msg_id.clone()) {
                    return Err(ValidationError::DuplicateMessageId(format!(
                        "Message ID '{}' is used by multiple messages. Each message must have a unique messageId. Found in channel '{}', message '{}'",
                        msg_id, channel_name, message_name
                    )));
                }
            }
        }
    }

    // Validate operations if present
    if let Some(ref operations) = spec.operations {
        for (op_id, op) in operations {
            // Validate channel reference format
            if !op.channel.ref_path.starts_with("#/channels/") {
                return Err(ValidationError::InvalidChannelReference(
                    op.channel.ref_path.clone()
                ));
            }

            // Validate message references
            for msg_ref in &op.messages {
                if !msg_ref.ref_path.starts_with("#/channels/") {
                    return Err(ValidationError::InvalidSchema(format!(
                        "Invalid message reference format in operation '{}': {}",
                        op_id, msg_ref.ref_path
                    )));
                }
            }
        }
    }

    // Validate protocol identifiers
    if let Some(ref servers) = spec.servers {
        for (server_name, server) in servers {
            protocol::validate_protocol(&server.protocol)
                .map_err(|e| match e {
                    ValidationError::UnsupportedProtocol { protocol, supported } => {
                    ValidationError::InvalidProtocol(format!(
                        "Server '{}' uses unsupported protocol '{}'. Supported protocols: {:?}",
                            server_name, protocol, supported
                    ))
                    }
                    _ => e,
                })?;
        }
    }

    // Validate protocol-specific bindings
    for (channel_name, channel) in &spec.channels {
        if let Some(ref bindings) = channel.bindings {
            // Validate bindings structure matches protocol
            if let Some(protocol) = get_channel_protocol(channel, spec) {
                validate_channel_bindings(&protocol, bindings, channel_name)?;
            }
        }
    }

    Ok(())
}

/// Get the protocol for a channel based on its server references
fn get_channel_protocol(
    channel: &Channel,
    spec: &AsyncApiSpec,
) -> Option<String> {
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
fn validate_channel_bindings(
    protocol: &str,
    bindings: &serde_json::Value,
    channel_name: &str,
) -> Result<(), ValidationError> {
    match protocol {
        "kafka" => {
            // Validate Kafka bindings structure
            if !bindings.as_object()
                .and_then(|o| o.get("kafka"))
                .is_some() {
                return Err(ValidationError::InvalidSchema(format!(
                    "Channel '{}': Kafka channel bindings must have 'kafka' key",
                    channel_name
                )));
            }
        }
        "mqtt" => {
            // Validate MQTT bindings structure
            if !bindings.as_object()
                .and_then(|o| o.get("mqtt"))
                .is_some() {
                return Err(ValidationError::InvalidSchema(format!(
                    "Channel '{}': MQTT channel bindings must have 'mqtt' key",
                    channel_name
                )));
            }
        }
        "nats" => {
            // Validate NATS bindings structure
            if !bindings.as_object()
                .and_then(|o| o.get("nats"))
                .is_some() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::AsyncApiBuilder;
    use std::collections::HashMap;

    #[test]
    fn test_validate_valid_spec() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test API".to_string(),
                version: "1.0.0".to_string(),
                description: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    description: None,
                    messages: {
                        let mut m = HashMap::new();
                        m.insert(
                            "TestMessage".to_string(),
                            Message {
                                message_id: None,
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                content_type: None,
                                tags: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                            },
                        );
                        m
                    },
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(validate_spec(&spec).is_ok());
    }

    #[test]
    fn test_validate_missing_title() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: String::new(),
                version: "1.0.0".to_string(),
                description: None,
            })
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::MissingRequiredField(_))
        ));
    }

    #[test]
    fn test_validate_empty_channels() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
            })
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::EmptyChannels)
        ));
    }

    #[test]
    fn test_validate_invalid_server_reference() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    description: None,
                    messages: {
                        let mut m = HashMap::new();
                        m.insert(
                            "TestMessage".to_string(),
                            Message {
                                message_id: None,
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                content_type: None,
                                tags: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                            },
                        );
                        m
                    },
                    servers: Some(vec!["nonexistent".to_string()]),
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::InvalidServerReference(_))
        ));
    }

    #[test]
    fn test_validate_channel_without_messages() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    description: None,
                    messages: HashMap::new(),
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::ChannelWithoutMessages(_))
        ));
    }

    #[test]
    fn test_validate_duplicate_message_ids() {
        let spec = AsyncApiBuilder::new()
            .info(Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
            })
            .channel(
                "test.channel".to_string(),
                Channel {
                    description: None,
                    messages: {
                        let mut m = HashMap::new();
                        m.insert(
                            "Message1".to_string(),
                            Message {
                                message_id: Some("duplicate-id".to_string()),
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                content_type: None,
                                tags: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                            },
                        );
                        m.insert(
                            "Message2".to_string(),
                            Message {
                                message_id: Some("duplicate-id".to_string()),
                                name: None,
                                title: None,
                                summary: None,
                                description: None,
                                content_type: None,
                                tags: None,
                                payload: MessagePayload {
                                    schema: serde_json::json!({"type": "object"}),
                                },
                            },
                        );
                        m
                    },
                    servers: None,
                    parameters: None,
                    bindings: None,
                },
            )
            .build();

        assert!(matches!(
            validate_spec(&spec),
            Err(ValidationError::DuplicateMessageId(_))
        ));
    }
}

