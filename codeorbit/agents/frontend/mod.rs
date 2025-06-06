//! Frontend agents for the CodeOrbit extension.
//! 
//! This module contains agents that handle frontend-related tasks such as
//! UI generation, component creation, and frontend optimization.

use crate::core::Result;
use crate::core::agent_registry::AgentRegistry;
use crate::core::orchestrator::Orchestrator;

mod ui_planner_agent;
mod component_generator_agent;

/// Initializes all frontend agents.
pub async fn initialize() -> Result<()> {
    // Register frontend agents here
    let mut registry = AgentRegistry::new();
    
    // Register UI Planner Agent
    registry.register(ui_planner_agent::UiPlannerAgent::new())?;
    
    // Register Component Generator Agent
    registry.register(component_generator_agent::ComponentGeneratorAgent::new())?;
    
    // Initialize all registered agents
    registry.initialize().await?;
    
    log::info!("Frontend agents initialized");
    Ok(())
}

/// Shuts down all frontend agents.
pub async fn shutdown() -> Result<()> {
    // Shutdown logic for frontend agents
    Ok(())
}
