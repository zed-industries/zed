//! Capability management for the `_meta.symposium` object in ACP messages.
//!
//! This module provides traits and types for working with capabilities stored in
//! the `_meta.symposium` field of `InitializeRequest` and `InitializeResponse`.
//!
//! # Example
//!
//! ```rust,no_run
//! use agent_client_protocol::{MetaCapability, MetaCapabilityExt};
//! # use agent_client_protocol::schema::v1::InitializeResponse;
//! # let init_response: InitializeResponse = unimplemented!();
//!
//! struct Proxy;
//! impl MetaCapability for Proxy {
//!     fn key(&self) -> &'static str { "proxy" }
//! }
//!
//! let response = init_response.add_meta_capability(Proxy);
//! if response.has_meta_capability(Proxy) {
//!     // Agent has the proxy capability
//! }
//! ```

use crate::schema::v1::{InitializeRequest, InitializeResponse};
use serde_json::{Value, json};

/// Trait for capabilities stored in the `_meta.symposium` object.
///
/// Capabilities are key-value pairs that signal features or context to components
/// in the proxy chain. Implement this trait to define new capabilities.
pub trait MetaCapability {
    /// The key name in the `_meta.symposium` object (e.g., "proxy", "mcp_acp_transport")
    fn key(&self) -> &'static str;

    /// The value to set when adding this capability (defaults to `true`)
    fn value(&self) -> serde_json::Value {
        serde_json::Value::Bool(true)
    }
}

/// Extension trait for checking and modifying capabilities in `InitializeRequest`.
pub trait MetaCapabilityExt {
    /// Check if a capability is present in `_meta.symposium`
    fn has_meta_capability(&self, capability: impl MetaCapability) -> bool;

    /// Add a capability to `_meta.symposium`, creating the structure if needed
    #[must_use]
    fn add_meta_capability(self, capability: impl MetaCapability) -> Self;

    /// Remove a capability from `_meta.symposium` if present
    #[must_use]
    fn remove_meta_capability(self, capability: impl MetaCapability) -> Self;
}

impl MetaCapabilityExt for InitializeRequest {
    fn has_meta_capability(&self, capability: impl MetaCapability) -> bool {
        self.client_capabilities
            .meta
            .as_ref()
            .and_then(|meta| meta.get("symposium"))
            .and_then(|symposium| symposium.get(capability.key()))
            .is_some_and(|v| !matches!(v, Value::Bool(false) | Value::Null))
    }

    fn add_meta_capability(mut self, capability: impl MetaCapability) -> Self {
        let meta = self
            .client_capabilities
            .meta
            .get_or_insert_with(Default::default);

        let symposium = meta.entry("symposium").or_insert_with(|| json!({}));

        if let Some(symposium_obj) = symposium.as_object_mut() {
            symposium_obj.insert("version".to_string(), json!("1.0"));
            symposium_obj.insert(capability.key().to_string(), capability.value());
        }

        self
    }

    fn remove_meta_capability(mut self, capability: impl MetaCapability) -> Self {
        if let Some(ref mut meta) = self.client_capabilities.meta
            && let Some(symposium) = meta.get_mut("symposium")
            && let Some(symposium_obj) = symposium.as_object_mut()
        {
            symposium_obj.remove(capability.key());
        }
        self
    }
}

impl MetaCapabilityExt for InitializeResponse {
    fn has_meta_capability(&self, capability: impl MetaCapability) -> bool {
        self.agent_capabilities
            .meta
            .as_ref()
            .and_then(|meta| meta.get("symposium"))
            .and_then(|symposium| symposium.get(capability.key()))
            .is_some_and(|v| !matches!(v, Value::Bool(false) | Value::Null))
    }

    fn add_meta_capability(mut self, capability: impl MetaCapability) -> Self {
        let meta = self
            .agent_capabilities
            .meta
            .get_or_insert_with(Default::default);

        let symposium = meta.entry("symposium").or_insert_with(|| json!({}));

        if let Some(symposium_obj) = symposium.as_object_mut() {
            symposium_obj.insert("version".to_string(), json!("1.0"));
            symposium_obj.insert(capability.key().to_string(), capability.value());
        }

        self
    }

    fn remove_meta_capability(mut self, capability: impl MetaCapability) -> Self {
        if let Some(ref mut meta) = self.agent_capabilities.meta
            && let Some(symposium) = meta.get_mut("symposium")
            && let Some(symposium_obj) = symposium.as_object_mut()
        {
            symposium_obj.remove(capability.key());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::ProtocolVersion;
    use crate::schema::v1::ClientCapabilities;
    use serde_json::json;

    struct TestCapability;
    impl MetaCapability for TestCapability {
        fn key(&self) -> &'static str {
            "test_cap"
        }
    }

    #[test]
    fn test_add_capability_to_request() {
        let request = InitializeRequest::new(ProtocolVersion::V1);

        let request = request.add_meta_capability(TestCapability);

        assert!(request.has_meta_capability(TestCapability));
        assert_eq!(
            request.client_capabilities.meta.as_ref().unwrap()["symposium"]["test_cap"],
            json!(true)
        );
    }

    #[test]
    fn test_remove_capability_from_request() {
        let mut meta = serde_json::Map::new();
        meta.insert(
            "symposium".to_string(),
            json!({
                "version": "1.0",
                "test_cap": true
            }),
        );
        let client_capabilities = ClientCapabilities::new().meta(meta);

        let request =
            InitializeRequest::new(ProtocolVersion::V1).client_capabilities(client_capabilities);

        let request = request.remove_meta_capability(TestCapability);

        assert!(!request.has_meta_capability(TestCapability));
    }

    #[test]
    fn test_add_capability_to_response() {
        let response = InitializeResponse::new(ProtocolVersion::V1);

        let response = response.add_meta_capability(TestCapability);

        assert!(response.has_meta_capability(TestCapability));
        assert_eq!(
            response.agent_capabilities.meta.as_ref().unwrap()["symposium"]["test_cap"],
            json!(true)
        );
    }

    #[test]
    fn test_has_meta_capability_false_value() {
        let mut meta = serde_json::Map::new();
        meta.insert(
            "symposium".to_string(),
            json!({
                "version": "1.0",
                "test_cap": false
            }),
        );
        let client_capabilities = ClientCapabilities::new().meta(meta);

        let request =
            InitializeRequest::new(ProtocolVersion::V1).client_capabilities(client_capabilities);

        assert!(!request.has_meta_capability(TestCapability));
    }

    #[test]
    fn test_has_meta_capability_null_value() {
        let mut meta = serde_json::Map::new();
        meta.insert(
            "symposium".to_string(),
            json!({
                "version": "1.0",
                "test_cap": null
            }),
        );
        let client_capabilities = ClientCapabilities::new().meta(meta);

        let request =
            InitializeRequest::new(ProtocolVersion::V1).client_capabilities(client_capabilities);

        assert!(!request.has_meta_capability(TestCapability));
    }
}
