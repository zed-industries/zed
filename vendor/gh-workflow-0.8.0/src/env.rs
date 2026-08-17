//!
//! Environment variable types and implementations for GitHub workflows.

use std::fmt::Display;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents environment variables in the workflow.
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Env(pub IndexMap<String, Value>);

impl From<IndexMap<String, Value>> for Env {
    /// Converts an `IndexMap` into an `Env`.
    fn from(value: IndexMap<String, Value>) -> Self {
        Env(value)
    }
}

impl Env {
    /// Sets the `GITHUB_TOKEN` environment variable.
    pub fn github() -> Self {
        let mut map = IndexMap::new();
        map.insert(
            "GITHUB_TOKEN".to_string(),
            Value::from("${{ secrets.GITHUB_TOKEN }}"),
        );
        Env(map)
    }

    /// Creates a new `Env` with a specified key-value pair.
    pub fn new<K: ToString, V: Into<Value>>(key: K, value: V) -> Self {
        Env::default().add(key, value)
    }

    /// Adds an environment variable to the `Env`.
    pub fn add<T1: ToString, T2: Into<Value>>(mut self, key: T1, value: T2) -> Self {
        self.0.insert(key.to_string(), value.into());
        self
    }
}

/// Represents environment variables as key-value pairs.
impl<S1: Display, S2: Display> From<(S1, S2)> for Env {
    /// Converts a tuple into an `Env`.
    fn from(value: (S1, S2)) -> Self {
        let mut index_map: IndexMap<String, Value> = IndexMap::new();
        index_map.insert(value.0.to_string(), Value::String(value.1.to_string()));
        Env(index_map)
    }
}
