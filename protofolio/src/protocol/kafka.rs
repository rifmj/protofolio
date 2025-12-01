//! Kafka protocol support

use super::Protocol;
use super::bindings::{KafkaChannelBinding, KafkaChannelConfig, KafkaMessageBinding, KafkaMessageConfig};

/// Kafka protocol identifier
pub const PROTOCOL: &str = "kafka";

/// Default Kafka port
pub const DEFAULT_PORT: u16 = 9092;

/// Kafka protocol implementation
pub struct KafkaProtocol;

impl Protocol for KafkaProtocol {
    fn name() -> &'static str {
        "Apache Kafka"
    }
    
    fn identifier() -> &'static str {
        PROTOCOL
    }
}

/// Helper functions for Kafka-specific configurations
impl KafkaProtocol {
    /// Create a Kafka channel binding
    pub fn channel_binding(
        topic: Option<String>,
        partitions: Option<u32>,
        replicas: Option<u32>,
    ) -> serde_json::Value {
        serde_json::to_value(KafkaChannelBinding {
            config: KafkaChannelConfig {
                topic,
                partitions,
                replicas,
                topic_configuration: None,
                binding_version: Some("0.4.0".to_string()),
            },
        }).unwrap_or_else(|_| serde_json::json!({}))
    }
    
    /// Create a Kafka message binding with key
    pub fn message_binding(key_schema: Option<serde_json::Value>) -> serde_json::Value {
        serde_json::to_value(KafkaMessageBinding {
            config: KafkaMessageConfig {
                key: key_schema,
                schema_id_location: None,
                schema_id_payload_encoding: None,
                schema_lookup_strategy: None,
                binding_version: Some("0.4.0".to_string()),
            },
        }).unwrap_or_else(|_| serde_json::json!({}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_protocol() {
        assert_eq!(KafkaProtocol::identifier(), "kafka");
        assert_eq!(KafkaProtocol::name(), "Apache Kafka");
    }

    #[test]
    fn test_kafka_channel_binding() {
        let binding = KafkaProtocol::channel_binding(
            Some("test-topic".to_string()),
            Some(3),
            Some(2),
        );
        
        assert!(binding["kafka"]["topic"].as_str().is_some());
        assert_eq!(binding["kafka"]["topic"], "test-topic");
        assert_eq!(binding["kafka"]["partitions"], 3);
        assert_eq!(binding["kafka"]["replicas"], 2);
    }

    #[test]
    fn test_kafka_message_binding() {
        let key_schema = serde_json::json!({"type": "string"});
        let binding = KafkaProtocol::message_binding(Some(key_schema.clone()));
        
        assert!(!binding["kafka"]["key"].is_null());
        assert_eq!(binding["kafka"]["key"], key_schema);
    }
}

