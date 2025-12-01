//! Protocol-specific bindings for AsyncAPI
//!
//! This module provides type-safe bindings for different messaging protocols
//! including NATS, Kafka, and MQTT.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NATS channel binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsChannelBinding {
    #[serde(rename = "nats")]
    pub config: NatsChannelConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsChannelConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// NATS message binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsMessageBinding {
    #[serde(rename = "nats")]
    pub config: NatsMessageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsMessageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// Kafka channel binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaChannelBinding {
    #[serde(rename = "kafka")]
    pub config: KafkaChannelConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaChannelConfig {
    /// Kafka topic name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// Number of partitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<u32>,
    /// Replication factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<u32>,
    /// Topic configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_configuration: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// Kafka message binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaMessageBinding {
    #[serde(rename = "kafka")]
    pub config: KafkaMessageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaMessageConfig {
    /// Message key schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<serde_json::Value>,
    /// Schema ID location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id_location: Option<String>,
    /// Schema ID payload encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id_payload_encoding: Option<String>,
    /// Schema lookup strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_lookup_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// MQTT channel binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttChannelBinding {
    #[serde(rename = "mqtt")]
    pub config: MqttChannelConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttChannelConfig {
    /// QoS level (0, 1, or 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<u8>,
    /// Retain flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    /// Topic name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

/// MQTT message binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttMessageBinding {
    #[serde(rename = "mqtt")]
    pub config: MqttMessageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttMessageConfig {
    /// QoS level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<u8>,
    /// Retain flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}
