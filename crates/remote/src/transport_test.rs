#[cfg(test)]
mod tests {
    use crate::ssh_transport::SshTransport;
    use crate::transport::{Transport, TransportConfig};
    use crate::transport_registry::TransportRegistry;
    use crate::ssh_session::SshConnectionOptions;
    use std::sync::Arc;

    #[test]
    fn test_transport_registry_initialization() {
        let registry = TransportRegistry::default();
        
        // Should have SSH transport registered by default
        let ssh_transport = registry.get_transport("ssh");
        assert!(ssh_transport.is_some());
        assert_eq!(ssh_transport.unwrap().name(), "ssh");
    }

    #[test]
    fn test_ssh_transport_supports_config() {
        let transport = SshTransport::new();
        
        let ssh_config = TransportConfig::Ssh(SshConnectionOptions {
            host: "test.example.com".to_string(),
            username: Some("testuser".to_string()),
            port: Some(22),
            password: None,
            args: None,
            port_forwards: None,
            nickname: None,
            upload_binary_over_ssh: false,
        });
        
        assert!(transport.supports_config(&ssh_config));
    }

    #[test]
    fn test_transport_registry_register() {
        let mut registry = TransportRegistry::new();
        
        // Initially empty
        assert!(registry.get_transport("ssh").is_none());
        
        // Register SSH transport
        registry.register(Arc::new(SshTransport::new()));
        
        // Now should have SSH transport
        assert!(registry.get_transport("ssh").is_some());
    }
}