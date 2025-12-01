//! Protocol-specific tests for Kafka and MQTT

use protofolio::{
    AsyncApiBuilder, Info, Server, Channel, Message, MessagePayload,
    Protocol, KafkaProtocol, KAFKA_PROTOCOL, KAFKA_DEFAULT_PORT,
    MqttProtocol, MQTT_PROTOCOL, MQTT_DEFAULT_PORT, MQTT_DEFAULT_SECURE_PORT, MqttQos,
    validate_spec,
};
use std::collections::HashMap;

#[test]
fn test_kafka_protocol_constants() {
    assert_eq!(KafkaProtocol::identifier(), "kafka");
    assert_eq!(KafkaProtocol::name(), "Apache Kafka");
    assert_eq!(KAFKA_PROTOCOL, "kafka");
    assert_eq!(KAFKA_DEFAULT_PORT, 9092);
}

#[test]
fn test_mqtt_protocol_constants() {
    assert_eq!(MqttProtocol::identifier(), "mqtt");
    assert_eq!(MqttProtocol::name(), "MQTT");
    assert_eq!(MQTT_PROTOCOL, "mqtt");
    assert_eq!(MQTT_DEFAULT_PORT, 1883);
    assert_eq!(MQTT_DEFAULT_SECURE_PORT, 8883);
}

#[test]
fn test_kafka_channel_binding() {
    let binding = KafkaProtocol::channel_binding(
        Some("user-events".to_string()),
        Some(3),
        Some(2),
    );
    
    assert!(binding["kafka"]["topic"].as_str().is_some());
    assert_eq!(binding["kafka"]["topic"], "user-events");
    assert_eq!(binding["kafka"]["partitions"], 3);
    assert_eq!(binding["kafka"]["replicas"], 2);
    assert_eq!(binding["kafka"]["binding_version"], "0.4.0");
}

#[test]
fn test_kafka_message_binding() {
    let key_schema = serde_json::json!({"type": "string"});
    let binding = KafkaProtocol::message_binding(Some(key_schema.clone()));
    
    assert!(!binding["kafka"]["key"].is_null());
    assert_eq!(binding["kafka"]["key"], key_schema);
    assert_eq!(binding["kafka"]["binding_version"], "0.4.0");
}

#[test]
fn test_mqtt_channel_binding() {
    let binding = MqttProtocol::channel_binding(
        Some("sensors/temperature".to_string()),
        Some(MqttQos::AtLeastOnce),
        Some(false),
    );
    
    assert_eq!(binding["mqtt"]["topic"], "sensors/temperature");
    assert_eq!(binding["mqtt"]["qos"], 1);
    assert_eq!(binding["mqtt"]["retain"], false);
    assert_eq!(binding["mqtt"]["binding_version"], "0.2.0");
}

#[test]
fn test_mqtt_message_binding() {
    let binding = MqttProtocol::message_binding(
        Some(MqttQos::ExactlyOnce),
        Some(true),
    );
    
    assert_eq!(binding["mqtt"]["qos"], 2);
    assert_eq!(binding["mqtt"]["retain"], true);
    assert_eq!(binding["mqtt"]["binding_version"], "0.2.0");
}

#[test]
fn test_kafka_spec_with_builder() {
    let spec = AsyncApiBuilder::new()
        .info(Info {
            title: "Kafka Events API".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Kafka-based event streaming".to_string()),
        })
        .server(
            "kafka-broker".to_string(),
            Server {
                url: format!("kafka://localhost:{}", KAFKA_DEFAULT_PORT),
                protocol: KAFKA_PROTOCOL.to_string(),
                description: Some("Main Kafka broker".to_string()),
            },
        )
        .kafka_channel(
            "user.events".to_string(),
            Channel {
                description: Some("User events channel".to_string()),
                messages: {
                    let mut m = HashMap::new();
                    m.insert(
                        "UserEvent".to_string(),
                        Message {
                            message_id: Some("user-event-v1".to_string()),
                            name: Some("UserEvent".to_string()),
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
                servers: Some(vec!["kafka-broker".to_string()]),
                parameters: None,
                bindings: None,
            },
            Some("user-events".to_string()),
            Some(3),
            Some(2),
        )
        .build();

    assert_eq!(spec.info.title, "Kafka Events API");
    assert!(spec.servers.is_some());
    let servers = spec.servers.as_ref().unwrap();
    assert_eq!(servers["kafka-broker"].protocol, "kafka");
    
    assert!(spec.channels.contains_key("user.events"));
    let channel = &spec.channels["user.events"];
    assert!(channel.bindings.is_some());
    let bindings = channel.bindings.as_ref().unwrap();
    assert_eq!(bindings["kafka"]["topic"], "user-events");
    assert_eq!(bindings["kafka"]["partitions"], 3);
    
    assert!(validate_spec(&spec).is_ok());
}

#[test]
fn test_mqtt_spec_with_builder() {
    let spec = AsyncApiBuilder::new()
        .info(Info {
            title: "MQTT IoT API".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        })
        .server(
            "mqtt-broker".to_string(),
            Server {
                url: format!("mqtt://mqtt.example.com:{}", MQTT_DEFAULT_PORT),
                protocol: MQTT_PROTOCOL.to_string(),
                description: None,
            },
        )
        .mqtt_channel(
            "sensors/temperature".to_string(),
            Channel {
                description: Some("Temperature sensor data".to_string()),
                messages: {
                    let mut m = HashMap::new();
                    m.insert(
                        "TemperatureReading".to_string(),
                        Message {
                            message_id: Some("temp-reading-v1".to_string()),
                            name: Some("TemperatureReading".to_string()),
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
                servers: Some(vec!["mqtt-broker".to_string()]),
                parameters: None,
                bindings: None,
            },
            Some("sensors/temperature".to_string()),
            Some(MqttQos::AtLeastOnce),
            Some(false),
        )
        .build();

    assert_eq!(spec.info.title, "MQTT IoT API");
    assert!(spec.servers.is_some());
    let servers = spec.servers.as_ref().unwrap();
    assert_eq!(servers["mqtt-broker"].protocol, "mqtt");
    
    assert!(spec.channels.contains_key("sensors/temperature"));
    let channel = &spec.channels["sensors/temperature"];
    assert!(channel.bindings.is_some());
    let bindings = channel.bindings.as_ref().unwrap();
    assert_eq!(bindings["mqtt"]["topic"], "sensors/temperature");
    assert_eq!(bindings["mqtt"]["qos"], 1);
    assert_eq!(bindings["mqtt"]["retain"], false);
    
    assert!(validate_spec(&spec).is_ok());
}

#[test]
fn test_protocol_validation() {
    // Test protocol validation through server creation
    let spec = AsyncApiBuilder::new()
        .info(Info {
            title: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        })
        .server(
            "kafka-server".to_string(),
            Server {
                url: "kafka://localhost:9092".to_string(),
                protocol: "kafka".to_string(),
                description: None,
            },
        )
        .server(
            "mqtt-server".to_string(),
            Server {
                url: "mqtt://localhost:1883".to_string(),
                protocol: "mqtt".to_string(),
                description: None,
            },
        )
        .server(
            "nats-server".to_string(),
            Server {
                url: "nats://localhost:4222".to_string(),
                protocol: "nats".to_string(),
                description: None,
            },
        )
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

    // All valid protocols should pass validation
    assert!(validate_spec(&spec).is_ok());
}

#[test]
fn test_mqtt_qos_enum() {
    assert_eq!(MqttQos::AtMostOnce.as_u8(), 0);
    assert_eq!(MqttQos::AtLeastOnce.as_u8(), 1);
    assert_eq!(MqttQos::ExactlyOnce.as_u8(), 2);

    assert_eq!(MqttQos::from_u8(0), Some(MqttQos::AtMostOnce));
    assert_eq!(MqttQos::from_u8(1), Some(MqttQos::AtLeastOnce));
    assert_eq!(MqttQos::from_u8(2), Some(MqttQos::ExactlyOnce));
    assert_eq!(MqttQos::from_u8(3), None);
}

