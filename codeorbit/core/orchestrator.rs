//! The orchestrator is the central component that manages agent communication and task coordination.

use crate::core::error::Result;
use crate::core::agent_registry::AgentRegistry;
use crate::core::context::Context;

/// The Orchestrator manages the lifecycle of agents and coordinates their activities.
pub struct Orchestrator {
    agent_registry: AgentRegistry,
    context: Context,
}

impl Orchestrator {
    /// Creates a new instance of the Orchestrator.
    pub fn new() -> Self {
        Self {
            agent_registry: AgentRegistry::new(),
            context: Context::new(),
        }
    }

    /// Initializes the orchestrator and all registered agents.
    pub async fn initialize(&mut self) -> Result<()> {
        log::info!("Initializing CodeOrbit Orchestrator");
        self.agent_registry.initialize().await?;
        Ok(())
    }

    /// Processes a user request by routing it to the appropriate agent(s).
    pub async fn process_request(&mut self, request: &str) -> Result<String> {
        log::debug!("Processing request: {}", request);
        
        // TODO: Implement request routing logic
        // For now, just return a placeholder response
        Ok("Request processed by CodeOrbit".to_string())
    }

    /// Shuts down the orchestrator and all agents.
    pub async fn shutdown(&mut self) -> Result<()> {
        log::info!("Shutting down CodeOrbit Orchestrator");
        self.agent_registry.shutdown().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator_initialization() {
        let mut orchestrator = Orchestrator::new();
        assert!(orchestrator.initialize().await.is_ok());
    }
}
