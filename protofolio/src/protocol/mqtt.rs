//! MQTT protocol support

use super::bindings::{
    MqttChannelBinding, MqttChannelConfig, MqttMessageBinding, MqttMessageConfig,
};
use super::Protocol;

/// MQTT protocol identifier
pub const PROTOCOL: &str = "mqtt";

/// Default MQTT port
pub const DEFAULT_PORT: u16 = 1883;

/// Default MQTT secure port (TLS)
pub const DEFAULT_SECURE_PORT: u16 = 8883;

/// MQTT QoS levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MqttQos {
    /// At most once delivery (QoS 0)
    AtMostOnce = 0,
    /// At least once delivery (QoS 1)
    AtLeastOnce = 1,
    /// Exactly once delivery (QoS 2)
    ExactlyOnce = 2,
}

impl MqttQos {
    /// Get QoS as u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Create QoS from u8, returns None if invalid
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(MqttQos::AtMostOnce),
            1 => Some(MqttQos::AtLeastOnce),
            2 => Some(MqttQos::ExactlyOnce),
            _ => None,
        }
    }
}

/// MQTT protocol implementation
pub struct MqttProtocol;

impl Protocol for MqttProtocol {
    fn name() -> &'static str {
        "MQTT"
    }

    fn identifier() -> &'static str {
        PROTOCOL
    }
}

/// Helper functions for MQTT-specific configurations
impl MqttProtocol {
    /// Create an MQTT channel binding
    pub fn channel_binding(
        topic: Option<String>,
        qos: Option<MqttQos>,
        retain: Option<bool>,
    ) -> serde_json::Value {
        serde_json::to_value(MqttChannelBinding {
            config: MqttChannelConfig {
                qos: qos.map(|q| q.as_u8()),
                retain,
                topic,
                binding_version: Some("0.2.0".to_string()),
            },
        })
        .unwrap_or_else(|_| serde_json::json!({}))
    }

    /// Create an MQTT message binding
    pub fn message_binding(qos: Option<MqttQos>, retain: Option<bool>) -> serde_json::Value {
        serde_json::to_value(MqttMessageBinding {
            config: MqttMessageConfig {
                qos: qos.map(|q| q.as_u8()),
                retain,
                binding_version: Some("0.2.0".to_string()),
            },
        })
        .unwrap_or_else(|_| serde_json::json!({}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mqtt_protocol() {
        assert_eq!(MqttProtocol::identifier(), "mqtt");
        assert_eq!(MqttProtocol::name(), "MQTT");
    }

    #[test]
    fn test_mqtt_qos() {
        assert_eq!(MqttQos::AtMostOnce.as_u8(), 0);
        assert_eq!(MqttQos::AtLeastOnce.as_u8(), 1);
        assert_eq!(MqttQos::ExactlyOnce.as_u8(), 2);

        assert_eq!(MqttQos::from_u8(0), Some(MqttQos::AtMostOnce));
        assert_eq!(MqttQos::from_u8(1), Some(MqttQos::AtLeastOnce));
        assert_eq!(MqttQos::from_u8(2), Some(MqttQos::ExactlyOnce));
        assert_eq!(MqttQos::from_u8(3), None);
    }

    #[test]
    fn test_mqtt_channel_binding() {
        let binding = MqttProtocol::channel_binding(
            Some("test/topic".to_string()),
            Some(MqttQos::AtLeastOnce),
            Some(false),
        );

        assert_eq!(binding["mqtt"]["topic"], "test/topic");
        assert_eq!(binding["mqtt"]["qos"], 1);
        assert_eq!(binding["mqtt"]["retain"], false);
    }

    #[test]
    fn test_mqtt_message_binding() {
        let binding = MqttProtocol::message_binding(Some(MqttQos::ExactlyOnce), Some(true));

        assert_eq!(binding["mqtt"]["qos"], 2);
        assert_eq!(binding["mqtt"]["retain"], true);
    }
}
