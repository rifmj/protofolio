//! NATS protocol support

use super::Protocol;

/// NATS protocol identifier
pub const PROTOCOL: &str = "nats";

/// Default NATS port
pub const DEFAULT_PORT: u16 = 4222;

/// NATS protocol implementation
pub struct NatsProtocol;

impl Protocol for NatsProtocol {
    fn name() -> &'static str {
        "NATS"
    }
    
    fn identifier() -> &'static str {
        PROTOCOL
    }
}

