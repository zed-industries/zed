# Browser Tools Server

A WebDriver-based server for browser automation and control.

## Overview

The Browser Tools Server provides a high-level API for interacting with web browsers through the WebDriver protocol. It enables you to:

- Capture screenshots
- Monitor console logs
- Monitor network requests
- Run DOM operations
- Run accessibility and performance audits
- And more...

## Requirements

- WebDriver for your browser (e.g., ChromeDriver for Chrome)
- A compatible browser (e.g., Chrome)

## Usage

### Basic Usage

```rust
use browser_tools_server::{BrowserToolsServer, BrowserToolsSettings, DEFAULT_HOST, DEFAULT_PORT};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create server with default settings
    let settings = BrowserToolsSettings {
        host: DEFAULT_HOST.to_string(),
        port: DEFAULT_PORT,
        browser_url: Some("https://www.example.com".to_string()),
    };
    
    let server = BrowserToolsServer::new(settings);
    
    // Start the server
    server.start().await?;
    
    // Run a tool
    let result = server.run_tool("captureScreenshot".to_string(), None).await?;
    
    // Stop the server
    server.stop()?;
    
    Ok(())
}
```

### Available Tools

The server provides several tools for browser automation:

- `getConsoleLogs`: Get browser console logs
- `getConsoleErrors`: Get browser console errors
- `getNetworkLogs`: Get browser network logs
- `getNetworkErrors`: Get browser network errors
- `captureScreenshot`: Capture browser screenshot
- `getSelectedElement`: Get information about selected DOM element
- `clearLogs`: Clear browser logs
- `runAudit`: Run browser audit (accessibility, performance, SEO, best practices)

## Running ChromeDriver

Before using the Browser Tools Server, you need to have ChromeDriver running:

1. Download the appropriate version of ChromeDriver that matches your Chrome browser from [ChromeDriver Downloads](https://chromedriver.chromium.org/downloads)
2. Start ChromeDriver on the default port (4444):

```bash
./chromedriver --port=4444
```

## Error Handling

Common errors:

- **ChromeDriver version mismatch**: Make sure your ChromeDriver version matches your Chrome browser version
- **Connection errors**: Ensure ChromeDriver is running on the specified host and port
- **Browser not found**: Ensure the browser is installed and can be launched by ChromeDriver

## License

GPL-3.0-or-later 