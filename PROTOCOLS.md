# Protocol Support Guide

This document provides detailed information about protocol support in `protofolio`, including configuration examples and best practices for each supported protocol.

## Supported Protocols

- **NATS**: Lightweight, high-performance messaging system
- **Kafka**: Distributed event streaming platform  
- **MQTT**: IoT messaging protocol

## NATS

### Overview

NATS is a lightweight, high-performance messaging system designed for cloud-native applications.

### Server Configuration

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats")
    ),
    // ...
)]
struct MyApi;
```

### Channel Bindings

NATS channel bindings allow you to configure queue groups and subject patterns:

```rust
use protofolio::{AsyncApiBuilder, Channel, NatsChannelBinding, NatsChannelConfig};
use std::collections::HashMap;

let binding = NatsChannelBinding {
    queue: Some("my-queue-group".to_string()),
    subject: Some("events.>".to_string()),
    config: Some(NatsChannelConfig {
        durable: Some(true),
        max_deliver: Some(10),
    }),
};

// Use with AsyncApiBuilder
let spec = AsyncApiBuilder::new()
    .channel_with_bindings(
        "events".to_string(),
        Channel { /* ... */ },
        serde_json::to_value(binding).unwrap(),
    )
    .build();
```

### Message Bindings

NATS message bindings configure message headers and other message-specific settings:

```rust
use protofolio::NatsMessageBinding;

let binding = NatsMessageBinding {
    headers: Some(serde_json::json!({
        "X-Custom-Header": "value"
    })),
    config: Some(NatsMessageConfig {
        // NATS-specific message configuration
    }),
};
```

### Best Practices

- Use subject hierarchies: `service.action.entity` (e.g., `trip.created`, `trip.status.changed`)
- Configure queue groups for load balancing across subscribers
- Use wildcards (`*`, `>`) for subject patterns when appropriate

## Kafka

### Overview

Kafka is a distributed event streaming platform designed for high-throughput, fault-tolerant event processing.

### Server Configuration

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    servers(
        (name = "kafka", url = "kafka://localhost:9092", protocol = "kafka")
    ),
    // ...
)]
struct MyApi;
```

### Channel Bindings

Kafka channel bindings configure topics, partitions, and replication:

```rust
use protofolio::{KafkaChannelBinding, KafkaChannelConfig};

let binding = KafkaChannelBinding {
    topic: Some("my-topic".to_string()),
    partitions: Some(3),
    replicas: Some(2),
    config: Some(KafkaChannelConfig {
        cleanup_policy: Some(vec!["delete".to_string()]),
        retention_ms: Some(604800000), // 7 days
    }),
};
```

### Message Bindings

Kafka message bindings configure keys, schemas, and headers:

```rust
use protofolio::KafkaMessageBinding;

let binding = KafkaMessageBinding {
    key: Some(serde_json::json!({
        "type": "string",
        "description": "Message key for partitioning"
    })),
    schema_id_location: Some("$message.payload#/schemaId".to_string()),
    config: Some(KafkaMessageConfig {
        // Kafka-specific message configuration
    }),
};
```

### Best Practices

- Use meaningful topic names: `service.entity.action` (e.g., `trip-service.trips.created`)
- Configure appropriate partition counts for parallelism
- Use message keys for partitioning related messages together
- Set appropriate retention policies based on your use case

## MQTT

### Overview

MQTT is a lightweight messaging protocol designed for IoT and low-bandwidth, high-latency networks.

### Server Configuration

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    servers(
        (name = "mqtt", url = "mqtt://localhost:1883", protocol = "mqtt"),
        (name = "mqtts", url = "mqtts://localhost:8883", protocol = "mqtt")
    ),
    // ...
)]
struct MyApi;
```

### Channel Bindings

MQTT channel bindings configure topics and QoS levels:

```rust
use protofolio::{MqttChannelBinding, MqttChannelConfig, MqttQos};

let binding = MqttChannelBinding {
    topic: Some("devices/+/events".to_string()),
    qos: Some(MqttQos::AtLeastOnce),
    config: Some(MqttChannelConfig {
        retain: Some(false),
        will_message: None,
    }),
};
```

### Message Bindings

MQTT message bindings configure message-specific QoS and retain flags:

```rust
use protofolio::{MqttMessageBinding, MqttQos};

let binding = MqttMessageBinding {
    payload_format_indicator: Some(1), // UTF-8 encoded
    content_type: Some("application/json".to_string()),
    config: Some(MqttMessageConfig {
        qos: Some(MqttQos::ExactlyOnce),
        retain: Some(false),
    }),
};
```

### QoS Levels

MQTT supports three QoS levels:

- **`AtMostOnce` (0)**: Fire and forget - message may be lost
- **`AtLeastOnce` (1)**: Message delivered at least once (may be duplicated)
- **`ExactlyOnce` (2)**: Message delivered exactly once (most reliable, highest overhead)

### Best Practices

- Use topic hierarchies: `service/device/event` (e.g., `iot/sensors/temperature`)
- Choose appropriate QoS levels based on message importance
- Use wildcards (`+`, `#`) for topic subscriptions
- Consider retain flags for last-known-good values
- Use secure connections (MQTTS) in production

## Protocol Selection Guide

### When to Use NATS

- Microservices communication
- High-performance, low-latency requirements
- Simple pub/sub patterns
- Cloud-native applications

### When to Use Kafka

- Event streaming and event sourcing
- High-throughput requirements
- Need for message replay
- Complex event processing pipelines
- Multiple consumers with different processing speeds

### When to Use MQTT

- IoT devices and sensors
- Low-bandwidth networks
- Intermittent connectivity
- Mobile applications
- Simple device-to-cloud communication

## Common Patterns

### Multi-Protocol Support

You can define multiple servers with different protocols:

```rust
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "Multi-Protocol API", version = "1.0.0"),
    servers(
        (name = "nats", url = "nats://localhost:4222", protocol = "nats"),
        (name = "kafka", url = "kafka://localhost:9092", protocol = "kafka")
    ),
    // ...
)]
struct MultiProtocolApi;
```

### Protocol-Specific Channel Configuration

Different channels can use different protocols by referencing different servers:

```rust
// Channel 1 uses NATS
// Channel 2 uses Kafka
// Configure via server references in channel bindings
```

## Troubleshooting

### Protocol Validation Errors

If you see `InvalidProtocol` errors:

1. Check that the protocol string matches exactly: `"nats"`, `"kafka"`, or `"mqtt"` (case-sensitive)
2. Verify the protocol is supported (check `protocol/mod.rs`)
3. Ensure server URL scheme matches protocol (e.g., `nats://` for NATS)

### Binding Configuration Issues

- Ensure binding JSON structure matches the protocol's binding specification
- Check that required fields are present
- Validate JSON schema if using programmatic binding creation

## Further Reading

- [AsyncAPI Specification](https://www.asyncapi.com/docs/specifications/v3.0.0)
- [NATS Documentation](https://docs.nats.io/)
- [Kafka Documentation](https://kafka.apache.org/documentation/)
- [MQTT Specification](https://mqtt.org/mqtt-specification/)

