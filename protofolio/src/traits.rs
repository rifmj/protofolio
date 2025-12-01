//! Traits for AsyncAPI specification generation

use crate::spec::{AsyncApiSpec, Operation};
use crate::error::ValidationError;

/// Main trait for types that can generate an AsyncAPI specification
///
/// # Example
///
/// ```rust,no_run
/// use protofolio::AsyncApi;
/// use protofolio_derive::{AsyncApi, AsyncApiMessage};
/// use serde::{Deserialize, Serialize};
/// use schemars::JsonSchema;
///
/// #[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
/// #[asyncapi(channel = "events", messageId = "event-v1")]
/// pub struct Event {
///     pub id: String,
/// }
///
/// #[derive(AsyncApi)]
/// #[asyncapi(
///     info(title = "My API", version = "1.0.0"),
///     channels("events"),
///     messages(Event)
/// )]
/// pub struct MyApi;
///
/// // Generate spec (panics on error)
/// let spec = MyApi::asyncapi();
///
/// // Generate spec with error handling (recommended for production)
/// let spec = MyApi::try_asyncapi()?;
///
/// // Generate JSON or YAML
/// let json = MyApi::asyncapi_json()?;
/// let yaml = MyApi::asyncapi_yaml()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub trait AsyncApi {
    /// Generate the AsyncAPI specification
    ///
    /// This method panics if validation fails. For production code, use
    /// [`try_asyncapi()`](Self::try_asyncapi) instead.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - Schema generation fails for any message type
    /// - Channel references are invalid
    /// - Message references are invalid
    fn asyncapi() -> AsyncApiSpec;

    /// Generate the AsyncAPI specification, returning a Result
    ///
    /// This method provides error-aware generation, returning validation errors
    /// instead of panicking. Use this when you need to handle errors gracefully.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if:
    /// - Schema generation fails for any message type
    /// - Channel references are invalid
    /// - Message references are invalid
    /// - Any other validation error occurs
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use protofolio::{AsyncApi, ValidationError};
    /// # use protofolio_derive::AsyncApi;
    /// #
    /// # #[derive(AsyncApi)]
    /// # #[asyncapi(info(title = "Test", version = "1.0.0"), channels("events"), messages())]
    /// # struct MyApi;
    ///
    /// match MyApi::try_asyncapi() {
    ///     Ok(spec) => {
    ///         println!("Spec generated successfully");
    ///         // Use the spec
    ///     }
    ///     Err(ValidationError::InvalidChannelReference(channel)) => {
    ///         eprintln!("Invalid channel: {}", channel);
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Validation error: {}", e);
    ///     }
    /// }
    /// ```
    fn try_asyncapi() -> Result<AsyncApiSpec, ValidationError> {
        // Default implementation calls asyncapi() and validates
        // Macros will override this with proper error handling
        let spec = Self::asyncapi();
        crate::validation::validate_spec(&spec)?;
        Ok(spec)
    }

    /// Generate the AsyncAPI specification as YAML string
    ///
    /// Returns a YAML-formatted string representation of the specification.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use protofolio::AsyncApi;
    /// # use protofolio_derive::AsyncApi;
    /// #
    /// # #[derive(AsyncApi)]
    /// # #[asyncapi(info(title = "Test", version = "1.0.0"), channels("events"), messages())]
    /// # struct MyApi;
    ///
    /// let yaml = MyApi::asyncapi_yaml()?;
    /// println!("{}", yaml);
    /// # Ok::<(), serde_yaml_ng::Error>(())
    /// ```
    fn asyncapi_yaml() -> Result<String, serde_yaml_ng::Error> {
        serde_yaml_ng::to_string(&Self::asyncapi())
    }

    /// Generate the AsyncAPI specification as JSON string
    ///
    /// Returns a JSON-formatted string representation of the specification.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use protofolio::AsyncApi;
    /// # use protofolio_derive::AsyncApi;
    /// #
    /// # #[derive(AsyncApi)]
    /// # #[asyncapi(info(title = "Test", version = "1.0.0"), channels("events"), messages())]
    /// # struct MyApi;
    ///
    /// let json = MyApi::asyncapi_json()?;
    /// println!("{}", json);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    fn asyncapi_json() -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&Self::asyncapi())
    }
}

/// Trait for types that represent AsyncAPI operations
pub trait AsyncApiOperation {
    /// Get the operation ID
    fn operation_id() -> &'static str;
    
    /// Get the action (send or receive)
    fn action() -> &'static str;
    
    /// Get the channel name
    fn channel() -> &'static str;
    
    /// Get the message type names
    fn message_types() -> &'static [&'static str];
    
    /// Get the message names for use in AsyncAPI spec
    fn message_names() -> Vec<String>;
    
    /// Get the summary
    fn summary() -> Option<&'static str>;
    
    /// Get the description
    fn description() -> Option<&'static str>;
    
    /// Get the tags
    fn tags() -> Option<Vec<crate::spec::Tag>>;
    
    /// Get the external documentation
    fn external_docs() -> Option<crate::spec::ExternalDocumentation> {
        None
    }
    
    /// Convert this operation to an Operation struct
    fn to_operation() -> Operation {
        use crate::spec::{ChannelReference, MessageReference};
        
        let channel_ref = format!("#/channels/{}", Self::channel());
        let message_refs: Vec<MessageReference> = Self::message_names()
            .iter()
            .map(|msg_name| {
                let ref_path = format!("#/channels/{}/messages/{msg_name}", Self::channel());
                MessageReference { ref_path }
            })
            .collect();
        
        Operation {
            action: Self::action().to_string(),
            channel: ChannelReference { ref_path: channel_ref },
            messages: message_refs,
            summary: Self::summary().map(|s| s.to_string()),
            description: Self::description().map(|s| s.to_string()),
            tags: Self::tags(),
            external_docs: Self::external_docs(),
        }
    }
}
