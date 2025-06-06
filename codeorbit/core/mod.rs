//! Core module for CodeOrbit extension
//! 
//! This module contains the orchestrator and core functionality for the CodeOrbit extension.

pub mod orchestrator;
pub mod agent_registry;
pub mod context;
pub mod error;

/// Re-exports for commonly used types
pub use orchestrator::Orchestrator;
pub use agent_registry::AgentRegistry;
pub use context::Context;
pub use error::{Result, Error};

/// Initializes the CodeOrbit core components
pub fn initialize() -> Result<()> {
    // Initialize core components here
    Ok(())
}
