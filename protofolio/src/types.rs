//! Core types for protofolio
//!
//! This module provides core type definitions including enums and type aliases
//! that improve type safety throughout the crate.

/// Operation action type
///
/// Represents whether an operation sends or receives messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationAction {
    /// Send/publish action
    Send,
    /// Receive/subscribe action
    Receive,
}

impl OperationAction {
    /// Get the string representation of the action
    pub const fn as_str(&self) -> &'static str {
        match self {
            OperationAction::Send => "send",
            OperationAction::Receive => "receive",
        }
    }
}

impl TryFrom<&str> for OperationAction {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "send" => Ok(OperationAction::Send),
            "receive" => Ok(OperationAction::Receive),
            _ => Err(format!(
                "Invalid operation action: '{}'. Expected 'send' or 'receive'",
                s
            )),
        }
    }
}

// Note: From<&str> is intentionally not implemented to encourage use of TryFrom
// for better error handling. Use OperationAction::try_from(s) instead.

impl From<OperationAction> for String {
    fn from(action: OperationAction) -> Self {
        action.as_str().to_string()
    }
}

/// AsyncAPI specification version
pub const ASYNCAPI_VERSION: &str = "3.0.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_action_as_str() {
        assert_eq!(OperationAction::Send.as_str(), "send");
        assert_eq!(OperationAction::Receive.as_str(), "receive");
    }

    #[test]
    fn test_operation_action_try_from_valid() {
        assert_eq!(
            OperationAction::try_from("send").unwrap(),
            OperationAction::Send
        );
        assert_eq!(
            OperationAction::try_from("receive").unwrap(),
            OperationAction::Receive
        );
    }

    #[test]
    fn test_operation_action_try_from_invalid() {
        assert!(OperationAction::try_from("invalid").is_err());
        assert!(OperationAction::try_from("").is_err());
        assert!(OperationAction::try_from("SEND").is_err()); // case sensitive
    }

    #[test]
    fn test_operation_action_to_string() {
        assert_eq!(String::from(OperationAction::Send), "send");
        assert_eq!(String::from(OperationAction::Receive), "receive");
    }

    #[test]
    fn test_operation_action_const_fn() {
        // Verify const fn works at compile time
        const SEND_STR: &str = OperationAction::Send.as_str();
        const RECEIVE_STR: &str = OperationAction::Receive.as_str();
        assert_eq!(SEND_STR, "send");
        assert_eq!(RECEIVE_STR, "receive");
    }
}
