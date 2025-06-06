//! Agents module for the CodeOrbit extension.
//! 
//! This module contains all the agents that make up the CodeOrbit system.

pub mod frontend;
pub mod backend;
pub mod database;
pub mod devops;
pub mod docs;

use crate::core::Result;

/// Initializes all agents.
pub async fn initialize() -> Result<()> {
    // Initialize all agents here
    frontend::initialize().await?;
    backend::initialize().await?;
    database::initialize().await?;
    devops::initialize().await?;
    docs::initialize().await?;
    
    Ok(())
}
