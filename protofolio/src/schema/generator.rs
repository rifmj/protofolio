//! JSON Schema generator implementation

use crate::error::SchemaError;
use schemars::JsonSchema;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

/// Global cache for generated schemas
/// Uses Arc to avoid cloning on cache hits, improving performance
/// Uses RwLock to allow multiple concurrent readers
static SCHEMA_CACHE: LazyLock<RwLock<HashMap<TypeId, Arc<serde_json::Value>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Generate JSON Schema for a type that implements JsonSchema
///
/// Returns an error if schema serialization fails.
/// Schemas are cached by TypeId to avoid regenerating the same schema multiple times.
/// Uses Arc internally to avoid cloning on cache hits.
///
/// # Example
///
/// ```rust,no_run
/// use protofolio::generate_schema;
/// use schemars::JsonSchema;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, JsonSchema)]
/// struct MyMessage {
///     id: String,
///     value: i32,
/// }
///
/// let schema = generate_schema::<MyMessage>()?;
/// println!("{}", serde_json::to_string_pretty(&schema)?);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # When to use
///
/// Typically, you don't need to call this directly - the `AsyncApi` derive macro
/// calls it automatically. Use this function if you need to generate schemas
/// programmatically or for testing.
pub fn generate_schema<T: JsonSchema + 'static>() -> Result<serde_json::Value, SchemaError> {
    let type_id = TypeId::of::<T>();
    
    // Check cache first (read lock for concurrent access)
    {
        let cache = SCHEMA_CACHE.read()
            .map_err(|e| SchemaError::Serialization(format!("Failed to acquire cache read lock: {}", e)))?;
        if let Some(cached) = cache.get(&type_id) {
            // Clone the Arc's inner value (cheap reference increment)
            return Ok((**cached).clone());
        }
    }
    
    // Generate schema if not in cache
    // In schemars 1.0+, use generate::SchemaGenerator instead of gen::SchemaGenerator
    use schemars::generate::SchemaGenerator;
    
    let mut gen = SchemaGenerator::default();
    let root_schema = T::json_schema(&mut gen);
    let value = serde_json::to_value(&root_schema)
        .map_err(|e| SchemaError::Serialization(e.to_string()))?;
    
    // Store in cache wrapped in Arc (write lock for exclusive access)
    {
        let value_arc = Arc::new(value.clone());
        let mut cache = SCHEMA_CACHE.write()
            .map_err(|e| SchemaError::Serialization(format!("Failed to acquire cache write lock: {}", e)))?;
        cache.insert(type_id, value_arc);
    }
    
    Ok(value)
}

/// Generate JSON Schema from a type name (for use in macros)
///
/// This is a helper that can be called from generated code.
/// Returns a Result to allow proper error handling.
/// Schemas are cached for performance.
///
/// # Example
///
/// ```rust,no_run
/// use protofolio::schema_for_type;
/// use schemars::JsonSchema;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, JsonSchema)]
/// struct MyMessage {
///     id: String,
/// }
///
/// let schema = schema_for_type::<MyMessage>()?;
/// # Ok::<(), protofolio::SchemaError>(())
/// ```
///
/// # Note
///
/// This is primarily used internally by the macro-generated code. For direct
/// schema generation, use [`generate_schema`] instead.
pub fn schema_for_type<T: JsonSchema + 'static>() -> Result<serde_json::Value, SchemaError> {
    generate_schema::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, JsonSchema)]
    struct TestStruct {
        name: String,
        age: u32,
    }

    #[derive(Serialize, Deserialize, JsonSchema)]
    struct SimpleStruct {
        value: String,
    }

    #[test]
    fn test_generate_schema() {
        let schema = generate_schema::<TestStruct>().unwrap();
        assert!(schema.is_object());
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"].is_object());
    }

    #[test]
    fn test_schema_for_type() {
        let schema = schema_for_type::<SimpleStruct>().unwrap();
        assert!(schema.is_object());
        assert_eq!(schema["type"], "object");
    }

    #[test]
    fn test_schema_contains_properties() {
        let schema = generate_schema::<TestStruct>().unwrap();
        let properties = schema["properties"].as_object().unwrap();
        assert!(properties.contains_key("name"));
        assert!(properties.contains_key("age"));
    }
}

