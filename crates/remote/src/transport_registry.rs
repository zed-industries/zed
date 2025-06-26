use anyhow::{Result, anyhow};
use collections::HashMap;
use gpui::{AsyncApp, Global};
use std::sync::Arc;

use crate::transport::{Transport, TransportConfig, TransportConnection, TransportDelegate};

/// Registry for managing available transports
pub struct TransportRegistry {
    transports: HashMap<&'static str, Arc<dyn Transport>>,
}

impl TransportRegistry {
    pub fn new() -> Self {
        Self {
            transports: HashMap::default(),
        }
    }

    /// Register a new transport
    pub fn register(&mut self, transport: Arc<dyn Transport>) {
        self.transports.insert(transport.name(), transport);
    }

    /// Create a connection using the appropriate transport
    pub async fn create_connection(
        &self,
        config: TransportConfig,
        delegate: Arc<dyn TransportDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Box<dyn TransportConnection>> {
        // Find the appropriate transport
        for transport in self.transports.values() {
            if transport.supports_config(&config) {
                return transport.connect(&config, delegate, cx).await;
            }
        }

        Err(anyhow!("No transport found for configuration"))
    }

    /// Get a transport by name
    pub fn get_transport(&self, name: &str) -> Option<Arc<dyn Transport>> {
        self.transports.get(name).cloned()
    }
}

impl Global for TransportRegistry {}

impl Default for TransportRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        // Register built-in transports
        use crate::ssh_transport::SshTransport;
        registry.register(Arc::new(SshTransport::new()));

        registry
    }
}
