# Browser Tools Server

A standalone WebDriver-based server for browser automation and control. This package provides a server that can be used to control web browsers using the WebDriver protocol.

## Features

- Connect to Chrome, Firefox, and other browsers via WebDriver
- Navigate to URLs, interact with page elements
- Capture screenshots and page content
- Execute JavaScript in the browser context
- Handle browser events and notifications

## Installation

Add the package to your Cargo.toml:

```toml
[dependencies]
browser_tools_server = { path = "/path/to/browser_tools_server" }
```

Or use the published version (if available):

```toml
[dependencies]
browser_tools_server = "0.1.0"
```

## Usage

Basic usage example:

```rust
use browser_tools_server::{BrowserToolsServer, BrowserToolsSettings, DEFAULT_PORT, DEFAULT_HOST};
use anyhow::Result;
use context_server::ContextServerId;

async fn example() -> Result<()> {
    // Create settings
    let settings = BrowserToolsSettings {
        port: DEFAULT_PORT,
        host: DEFAULT_HOST.to_string(),
        browser_url: None,
    };
    
    // Create a context server ID
    let id = ContextServerId::new("my-browser-tools".to_string());
    
    // Initialize server
    let server = BrowserToolsServer::new(id, settings);
    
    // Get a client
    if let Some(client) = server.client() {
        // Navigate to a URL
        let title = client.navigate_to("https://example.com").await?;
        println!("Page title: {}", title);
        
        // Execute JavaScript
        let result = client.execute_js_script("return document.readyState").await?;
        println!("Page ready state: {}", result);
    }
    
    Ok(())
}
```

## Prerequisites

- ChromeDriver (or other WebDriver) installed and running
- Chrome browser (or compatible browser)

### Installing ChromeDriver

```bash
# MacOS
brew install chromedriver

# Ubuntu/Debian
apt-get install chromium-chromedriver

# Windows
choco install chromedriver
```

**Important:** Make sure ChromeDriver version matches your Chrome browser version.

## Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/browser_tools_server.git
cd browser_tools_server

# Build the project
cargo build

# Run tests
cargo test
```

## Development

This package was originally developed as part of the Zed editor ecosystem but has been separated to be used independently. It's designed to work with the browser-tools extension for Zed but can be used in any Rust project.

## License

GPL-3.0-or-later 