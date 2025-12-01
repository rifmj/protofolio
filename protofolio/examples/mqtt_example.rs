//! MQTT-specific example
//!
//! This example demonstrates MQTT-specific features:
//! - MQTT server configuration (including MQTTS)
//! - Topic-based channel naming with hierarchical structure
//! - IoT device messaging patterns

use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define message types following MQTT topic naming conventions
// MQTT uses hierarchical topic structures with forward slashes

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "iot/sensors/temperature",
    messageId = "temperature-reading-v1",
    name = "TemperatureReading",
    title = "Temperature Sensor Reading",
    summary = "Published by temperature sensors",
    description = "Temperature readings from IoT sensors",
    tags = ["iot", "sensors", "temperature"]
)]
pub struct TemperatureReading {
    pub device_id: String,
    pub temperature: f64,
    pub unit: String,
    pub timestamp: i64,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "iot/sensors/humidity",
    messageId = "humidity-reading-v1",
    name = "HumidityReading",
    title = "Humidity Sensor Reading",
    summary = "Published by humidity sensors",
    tags = ["iot", "sensors", "humidity"]
)]
pub struct HumidityReading {
    pub device_id: String,
    pub humidity: f64,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "iot/devices/status",
    messageId = "device-status-v1",
    name = "DeviceStatus",
    title = "Device Status Update",
    summary = "Published when device status changes",
    description = "Device online/offline status and health information",
    tags = ["iot", "devices", "status"]
)]
pub struct DeviceStatus {
    pub device_id: String,
    pub status: String, // "online", "offline", "error"
    pub battery_level: Option<u8>,
    pub signal_strength: Option<i32>,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(
    channel = "iot/devices/commands",
    messageId = "device-command-v1",
    name = "DeviceCommand",
    title = "Device Command",
    summary = "Commands sent to IoT devices",
    description = "Control commands sent from cloud to devices",
    tags = ["iot", "devices", "commands"]
)]
pub struct DeviceCommand {
    pub device_id: String,
    pub command: String,
    pub parameters: Option<serde_json::Value>,
    pub timestamp: i64,
}

// Define the MQTT AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    info(
        title = "MQTT IoT Service API",
        version = "1.0.0",
        description = "IoT device messaging over MQTT protocol"
    ),
    servers(
        (name = "mqtt", url = "mqtt://localhost:1883", protocol = "mqtt"),
        (name = "mqtts", url = "mqtts://localhost:8883", protocol = "mqtt")
    ),
    channels(
        "iot/sensors/temperature",
        "iot/sensors/humidity",
        "iot/devices/status",
        "iot/devices/commands"
    ),
    messages(TemperatureReading, HumidityReading, DeviceStatus, DeviceCommand)
)]
pub struct MqttIoTApi;

fn main() {
    println!("=== MQTT Example ===\n");
    println!("This example demonstrates MQTT-specific configuration.\n");
    println!("MQTT uses hierarchical topic structures with forward slashes:");
    println!("  - iot/sensors/temperature");
    println!("  - iot/sensors/humidity");
    println!("  - iot/devices/status");
    println!("  - iot/devices/commands\n");

    // Display JSON output
    let json = MqttIoTApi::asyncapi_json().expect("Failed to generate JSON");
    println!("Generated AsyncAPI specification (JSON):");
    println!("{}", json);

    // Display YAML output
    let yaml = MqttIoTApi::asyncapi_yaml().expect("Failed to generate YAML");
    println!("\nGenerated AsyncAPI specification (YAML):");
    println!("{}", yaml);

    println!("\n=== MQTT Best Practices ===");
    println!("✓ Use hierarchical topic structures: service/device/event");
    println!("✓ Choose appropriate QoS levels based on message importance:");
    println!("  - QoS 0 (AtMostOnce): Fire and forget");
    println!("  - QoS 1 (AtLeastOnce): At least once delivery");
    println!("  - QoS 2 (ExactlyOnce): Exactly once delivery");
    println!("✓ Use wildcards (+, #) for topic subscriptions");
    println!("✓ Consider retain flags for last-known-good values");
    println!("✓ Use secure connections (MQTTS) in production");
    println!("✓ Design topics for scalability and filtering");
}
