# Browser Tools Extension for Zed

A standalone Zed extension providing browser automation and control functionality. This extension uses the `browser_tools_server` package to interact with web browsers via WebDriver.

## Features

- Interact with web browsers directly from Zed
- Navigate to URLs and execute JavaScript
- Capture screenshots and inspect page elements
- Debug web applications without leaving your editor

## Project Structure

This extension consists of two separate packages:

1. **browser_tools_server** - A WebDriver-based server for browser automation
2. **browser-tools-extension** - The Zed extension that uses the server

Both packages can be compiled and used independently.

## Installation

### Development Installation

Use the provided installation script:

```bash
./install-dev.sh
```

This will:
1. Build both the server and extension
2. Install it as a development extension in Zed
3. Configure all necessary dependencies

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

## Configuration

In Zed settings:

```json
{
  "extensions": {
    "browser-tools": {
      "enabled": true,
      "port": 4444,
      "host": "localhost",
      "lazy_connect": true
    }
  }
}
```

## Dependencies

This extension depends on:
- `browser_tools_server` - The server component that communicates with WebDriver
- `zed_extension_api` - Zed's extension API
- `context_server` - For API communication between the extension and server

## Development

The extension is built as a WebAssembly module for Zed. It handles communication between Zed and the `browser_tools_server` package.

### Building

```bash
# Full build and installation
./build.sh

# Just build the WASM module
cargo build --target wasm32-wasip1
```

## Debugging

The extension includes scripts for debugging:

- `debug.sh` - Run with detailed logging
- `logs.sh` - View extension logs
- `clean.sh` - Clean up build artifacts

## License

GPL-3.0-or-later 