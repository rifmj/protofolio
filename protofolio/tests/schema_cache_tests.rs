//! Tests for schema generation caching

use protofolio::{generate_schema, schema_for_type};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Barrier};
use std::thread;

#[derive(Serialize, Deserialize, JsonSchema)]
struct TestStruct1 {
    name: String,
    age: u32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct TestStruct2 {
    value: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct TestStruct3 {
    data: String,
}

#[test]
fn test_schema_cache_hit() {
    // First call should generate and cache
    let schema1 = generate_schema::<TestStruct1>().unwrap();
    
    // Second call should hit cache
    let schema2 = generate_schema::<TestStruct1>().unwrap();
    
    // Schemas should be identical
    assert_eq!(schema1, schema2);
}

#[test]
fn test_schema_cache_different_types() {
    // Generate schemas for different types
    let schema1 = generate_schema::<TestStruct1>().unwrap();
    let schema2 = generate_schema::<TestStruct2>().unwrap();
    let schema3 = generate_schema::<TestStruct3>().unwrap();
    
    // All should be different
    assert_ne!(schema1, schema2);
    assert_ne!(schema1, schema3);
    assert_ne!(schema2, schema3);
    
    // Verify each type has correct structure
    assert_eq!(schema1["type"], "object");
    assert_eq!(schema2["type"], "object");
    assert_eq!(schema3["type"], "object");
    
    // Verify properties are different
    assert!(schema1["properties"].as_object().unwrap().contains_key("name"));
    assert!(schema2["properties"].as_object().unwrap().contains_key("value"));
    assert!(schema3["properties"].as_object().unwrap().contains_key("data"));
}

#[test]
fn test_schema_cache_thread_safety() {
    const NUM_THREADS: usize = 10;
    const ITERATIONS: usize = 100;
    
    let barrier = Arc::new(Barrier::new(NUM_THREADS));
    let mut handles = vec![];
    
    for _ in 0..NUM_THREADS {
        let barrier = barrier.clone();
        let handle = thread::spawn(move || {
            // Wait for all threads to start
            barrier.wait();
            
            // Generate schemas concurrently
            for _ in 0..ITERATIONS {
                let schema1 = generate_schema::<TestStruct1>().unwrap();
                let schema2 = generate_schema::<TestStruct2>().unwrap();
                let schema3 = generate_schema::<TestStruct3>().unwrap();
                
                // Verify schemas are valid
                assert_eq!(schema1["type"], "object");
                assert_eq!(schema2["type"], "object");
                assert_eq!(schema3["type"], "object");
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_schema_for_type_uses_cache() {
    // Test that schema_for_type also uses the cache
    let schema1 = schema_for_type::<TestStruct1>().unwrap();
    let schema2 = schema_for_type::<TestStruct1>().unwrap();
    
    assert_eq!(schema1, schema2);
    assert_eq!(schema1["type"], "object");
}

#[test]
fn test_cache_persistence_across_calls() {
    // Generate schema
    let schema1 = generate_schema::<TestStruct1>().unwrap();
    
    // Generate other schemas
    let _ = generate_schema::<TestStruct2>().unwrap();
    let _ = generate_schema::<TestStruct3>().unwrap();
    
    // Original schema should still be cached
    let schema2 = generate_schema::<TestStruct1>().unwrap();
    assert_eq!(schema1, schema2);
}

