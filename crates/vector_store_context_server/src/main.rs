mod server;

use anyhow::Result;
use std::{path::PathBuf, sync::Arc};
use server::VectorStoreServer;

fn main() -> Result<()> {
    // Setup logging
    env_logger::try_init().ok();
    
    // Default path for database
    let db_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/zed/vector_stores");
    
    // Create the vector store server
    let vector_store_server = Arc::new(VectorStoreServer::new(db_path));
    
    // Print server information
    log::info!("Vector store context server initialized");
    log::info!("Server capabilities: {:?}", vector_store_server.capabilities());
    
    // In a real implementation, this would handle requests from the context server
    // For now, just demonstrate the functionality is available
    log::info!("Server ready to handle requests");
    
    // Wait for user input to exit
    println!("Press Enter to exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    log::info!("Shutting down");
    
    Ok(())
} 