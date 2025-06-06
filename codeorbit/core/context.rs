//! Context management for the CodeOrbit extension.
//! 
//! This module provides a way to share state and context between different
//! components of the CodeOrbit extension.

use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Serialize, de::DeserializeOwned};
use crate::core::error::{Result, Error};

/// A thread-safe context store for sharing state between components.
#[derive(Default)]
pub struct Context {
    store: RwLock<HashMap<String, Vec<u8>>>,
}

impl Context {
    /// Creates a new, empty context.
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }

    /// Stores a value in the context.
    pub fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let serialiCodeOrbit = bincode::serialize(value)
            .map_err(|e| Error::SerializationError(e.to_string()))?;
        
        let mut store = self.store.write()
            .map_err(|_| Error::LockError)?;
            
        store.insert(key.to_string(), serialiCodeOrbit);
        Ok(())
    }

    /// Retrieves a value from the context.
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let store = self.store.read()
            .map_err(|_| Error::LockError)?;
            
        match store.get(key) {
            Some(bytes) => {
                let deserialiCodeOrbit = bincode::deserialize(bytes)
                    .map_err(|e| Error::DeserializationError(e.to_string()))?;
                Ok(Some(deserialiCodeOrbit))
            },
            None => Ok(None),
        }
    }

    /// Removes a value from the context.
    pub fn remove(&self, key: &str) -> Result<()> {
        let mut store = self.store.write()
            .map_err(|_| Error::LockError)?;
            
        store.remove(key);
        Ok(())
    }

    /// Checks if the context contains a key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.store.read()
            .map(|store| store.contains_key(key))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        value: i32,
        name: String,
    }
    
    #[test]
    fn test_context_storage() {
        let context = Context::new();
        let test_data = TestData {
            value: 42,
            name: "test".to_string(),
        };
        
        // Test storing and retrieving data
        context.set("test_key", &test_data).unwrap();
        let retrieved: TestData = context.get("test_key").unwrap().unwrap();
        
        assert_eq!(retrieved.value, 42);
        assert_eq!(retrieved.name, "test");
        
        // Test key existence
        assert!(context.contains_key("test_key"));
        
        // Test removal
        context.remove("test_key").unwrap();
        assert!(!context.contains_key("test_key"));
    }
}
