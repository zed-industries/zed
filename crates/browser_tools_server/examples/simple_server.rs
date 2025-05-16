use browser_tools_server::{BrowserToolsServer, BrowserToolsSettings, DEFAULT_HOST, DEFAULT_PORT};
use anyhow::Result;
use env_logger::Env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    println!("Starting browser tools example...");
    
    // Create server with default settings
    let settings = BrowserToolsSettings {
        host: DEFAULT_HOST.to_string(),
        port: DEFAULT_PORT,
        browser_url: Some("https://www.example.com".to_string()),
    };
    
    let server = BrowserToolsServer::new(settings);
    
    // Start the server
    match server.start().await {
        Ok(_) => println!("Server started successfully!"),
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            println!("Trying mock mode...");
            server.mock_client();
        }
    }
    
    // Get available tools
    let tools = server.available_tools();
    println!("Available tools:");
    for tool in tools {
        println!("  - {}: {}", tool.name, tool.description.unwrap_or_default());
    }
    
    // If we have a real client, try to run a tool
    if let Some(client) = server.client() {
        if !client.is_mock() {
            println!("Running screenshot tool...");
            match server.run_tool("captureScreenshot".to_string(), None).await {
                Ok(result) => {
                    println!("Screenshot captured! Result has {} content items", result.content.len());
                    if let Some(meta) = result.meta {
                        println!("Screenshot metadata: {:?}", meta);
                    }
                },
                Err(e) => eprintln!("Failed to capture screenshot: {}", e),
            }
        } else {
            println!("Using mock client - tool operations will return mock data");
        }
    }
    
    // Wait a bit before stopping
    println!("Waiting 2 seconds before stopping server...");
    sleep(Duration::from_secs(2)).await;
    
    // Stop the server
    match server.stop() {
        Ok(_) => println!("Server stopped successfully!"),
        Err(e) => eprintln!("Failed to stop server: {}", e),
    }
    
    Ok(())
} 