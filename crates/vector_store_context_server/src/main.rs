mod server;

use anyhow::Result;
use context_server::{ContextServer, ContextServerId};
use context_server::transport::Transport;
use futures::{Stream, stream::empty};
use std::{path::PathBuf, pin::Pin, sync::Arc};
use async_trait::async_trait;
use server::VectorStoreServer;

// Create a minimal transport that doesn't do much
struct DummyTransport {
    _server: Arc<VectorStoreServer>,
}

#[async_trait]
impl Transport for DummyTransport {
    async fn send(&self, _message: String) -> Result<()> {
        // This is just a stub implementation
        Ok(())
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(empty())
    }
    
    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(empty())
    }
}

fn main() -> Result<()> {
    // Default path for database
    let db_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/zed/vector_stores");
    
    // Create the vector store server
    let vector_store_server = Arc::new(VectorStoreServer::new(db_path));
    
    // Create a transport
    let transport = Arc::new(DummyTransport {
        _server: vector_store_server.clone(),
    });
    
    // Create the context server with our transport
    let server_id = ContextServerId(Arc::from("vector_store"));
    let _context_server = Arc::new(ContextServer::new(server_id, transport));
    
    println!("Vector store context server initialized");
    
    // Since this is a CLI tool, just wait forever
    std::thread::park();
    
    Ok(())
} 