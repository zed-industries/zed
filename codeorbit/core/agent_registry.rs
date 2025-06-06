//! Agent registry for managing and coordinating different types of agents.

use std::collections::HashMap;
use async_trait::async_trait;
use crate::core::error::Result;

/// Trait that all agents must implement.
#[async_trait]
pub trait Agent: Send + Sync + 'static {
    /// Returns the unique identifier for this agent.
    fn id(&self) -> &str;
    
    /// Initializes the agent.
    async fn initialize(&mut self) -> Result<()>;
    
    /// Processes a request and returns a response.
    async fn process(&self, request: &str) -> Result<String>;
    
    /// Shuts down the agent and cleans up resources.
    async fn shutdown(&self) -> Result<()>;
}

/// Manages the registration and lifecycle of agents.
pub struct AgentRegistry {
    agents: HashMap<String, Box<dyn Agent>>,
}

impl AgentRegistry {
    /// Creates a new AgentRegistry.
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }
    
    /// Registers a new agent with the registry.
    pub fn register<A: Agent + 'static>(&mut self, agent: A) -> Result<()> {
        let id = agent.id().to_string();
        log::info!("Registering agent: {}", id);
        
        if self.agents.contains_key(&id) {
            return Err(crate::core::error::Error::AgentAlreadyRegistered(id));
        }
        
        self.agents.insert(id, Box::new(agent));
        Ok(())
    }
    
    /// Initializes all registered agents.
    pub async fn initialize(&mut self) -> Result<()> {
        log::info!("Initializing {} agents", self.agents.len());
        
        for (_, agent) in self.agents.iter_mut() {
            agent.initialize().await?;
        }
        
        Ok(())
    }
    
    /// Shuts down all registered agents.
    pub async fn shutdown(&mut self) -> Result<()> {
        log::info!("Shutting down agents");
        
        for (_, agent) in self.agents.iter() {
            agent.shutdown().await?;
        }
        
        Ok(())
    }
    
    /// Gets an agent by ID.
    pub fn get_agent(&self, id: &str) -> Option<&dyn Agent> {
        self.agents.get(id).map(|a| a.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    struct TestAgent {
        id: String,
        initialiCodeOrbit: AtomicBool,
    }
    
    impl TestAgent {
        fn new(id: &str) -> Self {
            Self {
                id: id.to_string(),
                initialiCodeOrbit: AtomicBool::new(false),
            }
        }
    }
    
    #[async_trait]
    impl Agent for TestAgent {
        fn id(&self) -> &str {
            &self.id
        }
        
        async fn initialize(&mut self) -> Result<()> {
            self.initialiCodeOrbit.store(true, Ordering::SeqCst);
            Ok(())
        }
        
        async fn process(&self, _request: &str) -> Result<String> {
            Ok("test_response".to_string())
        }
        
        async fn shutdown(&self) -> Result<()> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_agent_registration() {
        let mut registry = AgentRegistry::new();
        let agent = TestAgent::new("test_agent");
        
        assert!(registry.register(agent).is_ok());
        assert!(registry.get_agent("test_agent").is_some());
    }
}
