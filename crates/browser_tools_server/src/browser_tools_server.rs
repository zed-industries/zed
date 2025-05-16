mod api;
pub mod client;
mod errors;
mod models;

use std::sync::Arc;
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use client::BrowserToolsClient;
use log::{debug, error, info, warn};
use parking_lot::RwLock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio;

pub const DEFAULT_PORT: u16 = 4444; // WebDriver port
pub const DEFAULT_HOST: &str = "localhost";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserToolsSettings {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default)]
    pub browser_url: Option<String>,
}

impl Default for BrowserToolsSettings {
    fn default() -> Self {
        Self {
            port: DEFAULT_PORT,
            host: DEFAULT_HOST.to_string(),
            browser_url: None,
        }
    }
}

fn default_port() -> u16 {
    DEFAULT_PORT
}

fn default_host() -> String {
    DEFAULT_HOST.to_string()
}

/// The BrowserToolsServer provides WebDriver-based browser automation
pub struct BrowserToolsServer {
    client: RwLock<Option<BrowserToolsClient>>,
    settings: BrowserToolsSettings,
    api: RwLock<Option<api::BrowserToolsApi>>,
}

impl BrowserToolsServer {
    /// Create a new browser tools server with the given settings
    pub fn new(settings: BrowserToolsSettings) -> Self {
        Self {
            client: RwLock::new(None),
            settings,
            api: RwLock::new(None),
        }
    }

    /// Create a new browser tools server with default settings
    pub fn with_default_settings() -> Self {
        Self::new(BrowserToolsSettings::default())
    }

    /// Get the current client if available
    pub fn client(&self) -> Option<BrowserToolsClient> {
        self.client.read().clone()
    }
    
    /// Get the current settings
    pub fn settings(&self) -> BrowserToolsSettings {
        self.settings.clone()
    }
    
    /// Set the client explicitly
    pub fn set_client(&self, client: BrowserToolsClient) {
        *self.client.write() = Some(client);
    }

    /// Create a mock client for testing
    pub fn mock_client(&self) {
        // Create a mock client with mock_mode=true which prevents WebDriver initialization
        let mut client = self.client.write();
        *client = Some(BrowserToolsClient::mock(self.settings.host.clone(), self.settings.port));
        info!("Mock browser client initialized for testing");
    }

    /// Initialize the client
    pub fn initialize_client(&self) -> Result<()> {
        // Create the WebDriver client
        let client = BrowserToolsClient::new(
            self.settings.host.clone(),
            self.settings.port
        )?;
        
        // Create a new runtime for async operations
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| anyhow!("Failed to create Tokio runtime: {}", e))?;
        
        // Initialize the client using our new init method
        match runtime.block_on(async {
            client.ping().await?;
            client.init().await
        }) {
            Ok(_) => {
                // Connection successful, store the client
                *self.client.write() = Some(client);
                info!("Successfully connected to ChromeDriver at {}:{}", self.settings.host, self.settings.port);
                Ok(())
            },
            Err(e) => {
                // Check for version mismatch errors
                let error_msg = e.to_string();
                if error_msg.contains("This version of ChromeDriver only supports Chrome version") && 
                   error_msg.contains("Current browser version is") {
                    error!("ChromeDriver version mismatch detected: {}", error_msg);
                    error!("You need to download the matching ChromeDriver version for your Chrome browser.");
                    error!("Visit https://chromedriver.chromium.org/downloads to get the matching version.");
                } else {
                    // Other connection errors
                    error!("Failed to connect to ChromeDriver: {}", e);
                }
                
                // Connection failed
                Err(anyhow!("Failed to connect to ChromeDriver: {}", e))
            }
        }
    }

    /// Start the browser tools server
    pub async fn start(&self) -> Result<()> {
        info!("Starting browser tools server");

        // Create browser tools client
        let client = match BrowserToolsClient::new(
            self.settings.host.clone(),
            self.settings.port
        ) {
            Ok(client) => client,
            Err(err) => {
                let error_msg = format!("Failed to create WebDriver client: {}. Check your connection settings.", err);
                error!("{}", error_msg);
                Self::log_error(&error_msg);
                return Err(anyhow!(error_msg));
            }
        };

        // Check if browser tools server is available and initialize the WebDriver
        if let Err(err) = client.ping().await {
            let error_msg = format!("Failed to connect to WebDriver: {}. Make sure a WebDriver instance is running at http://{}:{}",
                err, self.settings.host, self.settings.port);
            error!("{}", error_msg);
            Self::log_error(&error_msg);
            return Err(anyhow!(error_msg));
        }
        
        // Initialize the WebDriver
        if let Err(err) = client.init().await {
            let error_msg = if err.to_string().contains("This version of ChromeDriver only supports Chrome version") {
                format!("ChromeDriver version mismatch: {}. Visit https://chromedriver.chromium.org/downloads to get the matching version.", err)
            } else {
                format!("Failed to initialize WebDriver: {}. Make sure a compatible ChromeDriver is running at http://{}:{}",
                    err, self.settings.host, self.settings.port)
            };
            error!("{}", error_msg);
            Self::log_error(&error_msg);
            return Err(anyhow!(error_msg));
        }

        // Store the client
        *self.client.write() = Some(client.clone());
        
        // Initialize the API
        *self.api.write() = Some(api::BrowserToolsApi::new(client.clone()));

        // Inject console and network monitoring scripts
        if let Err(err) = self.inject_monitoring_scripts(&client).await {
            let error_msg = format!("Failed to inject monitoring scripts: {}", err);
            error!("{}", error_msg);
            Self::log_error(&error_msg);
            return Err(anyhow!(error_msg));
        }

        debug!("Browser tools server started successfully");
        Ok(())
    }

    /// Stop the browser tools server
    pub fn stop(&self) -> Result<()> {
        debug!("Stopping browser tools server");
        
        // Get client and close it
        if let Some(client) = self.client.read().clone() {
            futures::executor::block_on(client.close())?;
        }
        
        // Clear client and API
        *self.client.write() = None;
        *self.api.write() = None;
        
        debug!("Browser tools server stopped");
        Ok(())
    }
    
    /// Get the API if available
    pub fn api(&self) -> Option<api::BrowserToolsApi> {
        // Initialize API if needed
        if self.api.read().is_none() {
            if let Some(client) = self.client() {
                *self.api.write() = Some(api::BrowserToolsApi::new(client));
            }
        }
        
        self.api.read().clone()
    }
    
    /// Run a tool by name with the given arguments
    pub async fn run_tool(&self, tool_name: String, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<api::ToolResult> {
        if let Some(api) = self.api() {
            api.run_tool(tool_name, arguments).await
        } else {
            Err(anyhow!("API not initialized"))
        }
    }

    /// Get available tools
    pub fn available_tools(&self) -> Vec<api::ToolDescription> {
        if let Some(api) = self.api() {
            api.available_tools()
        } else {
            Vec::new()
        }
    }

    /// Inject monitoring scripts into the browser
    async fn inject_monitoring_scripts(&self, client: &BrowserToolsClient) -> Result<()> {
        // JavaScript to inject console logging
        let console_script = r#"
        // Initialize console logs array if it doesn't exist
        if (!window.console_logs) {
            window.console_logs = [];
            
            // Store original console methods
            const originalConsole = {
                log: console.log,
                info: console.info,
                warn: console.warn,
                error: console.error,
                debug: console.debug
            };
            
            // Override console methods
            for (const method in originalConsole) {
                console[method] = function() {
                    // Call original method
                    originalConsole[method].apply(console, arguments);
                    
                    // Convert arguments to a string
                    const message = Array.from(arguments).map(arg => {
                        if (typeof arg === 'object') {
                            try {
                                return JSON.stringify(arg);
                            } catch(e) {
                                return String(arg);
                            }
                        }
                        return String(arg);
                    }).join(' ');
                    
                    // Get stack trace
                    let stack;
                    try {
                        throw new Error();
                    } catch(e) {
                        stack = e.stack;
                    }
                    
                    // Add to logs
                    window.console_logs.push({
                        level: method,
                        message: message,
                        timestamp: new Date().toISOString(),
                        trace: stack
                    });
                };
            }
        }
        
        // Add selection tracking
        document.addEventListener('mouseup', function() {
            const selection = window.getSelection();
            if (selection && selection.rangeCount > 0) {
                const range = selection.getRangeAt(0);
                window.selectedElement = range.startContainer.nodeType === 3 ? 
                    range.startContainer.parentNode : 
                    range.startContainer;
            }
        });
        
        return true;
        "#;
        
        // JavaScript to inject network monitoring
        let network_script = r#"
        // Initialize network logs array if it doesn't exist
        if (!window.network_logs) {
            window.network_logs = [];
            
            // Intercept fetch requests
            const originalFetch = window.fetch;
            window.fetch = async function(input, init) {
                const startTime = Date.now();
                const url = typeof input === 'string' ? input : input.url;
                const method = init?.method || (typeof input === 'string' ? 'GET' : input.method);
                
                let requestHeaders = {};
                if (init?.headers) {
                    if (init.headers instanceof Headers) {
                        init.headers.forEach((value, key) => {
                            requestHeaders[key] = value;
                        });
                    } else {
                        requestHeaders = init.headers;
                    }
                }
                
                const requestBody = init?.body;
                
                let logEntry = {
                    url: url,
                    method: method || 'GET',
                    timestamp: new Date().toISOString(),
                    request_headers: requestHeaders,
                    request_body: typeof requestBody === 'string' ? requestBody : null
                };
                
                try {
                    const response = await originalFetch.apply(this, arguments);
                    
                    // Clone the response to read the body
                    const clonedResponse = response.clone();
                    
                    // Try to get response body
                    let responseBody = null;
                    try {
                        const contentType = clonedResponse.headers.get('content-type');
                        if (contentType && contentType.includes('application/json')) {
                            responseBody = await clonedResponse.text();
                        }
                    } catch (e) {
                        // Ignore body reading errors
                    }
                    
                    // Add response details
                    logEntry.status = response.status;
                    logEntry.status_text = response.statusText;
                    logEntry.type_ = 'fetch';
                    logEntry.duration = Date.now() - startTime;
                    
                    // Get response headers
                    let responseHeaders = {};
                    response.headers.forEach((value, key) => {
                        responseHeaders[key] = value;
                    });
                    logEntry.response_headers = responseHeaders;
                    logEntry.response_body = responseBody;
                    
                    // Add to logs
                    window.network_logs.push(logEntry);
                    
                    return response;
                } catch (e) {
                    // Error handling
                    logEntry.error = e.toString();
                    logEntry.duration = Date.now() - startTime;
                    
                    // Add to logs
                    window.network_logs.push(logEntry);
                    
                    throw e;
                }
            };
            
            // Intercept XMLHttpRequest
            const originalXHROpen = XMLHttpRequest.prototype.open;
            const originalXHRSend = XMLHttpRequest.prototype.send;
            
            XMLHttpRequest.prototype.open = function(method, url) {
                this._requestMethod = method;
                this._requestUrl = url;
                this._requestHeaders = {};
                this._startTime = Date.now();
                
                return originalXHROpen.apply(this, arguments);
            };
            
            XMLHttpRequest.prototype.setRequestHeader = function(name, value) {
                this._requestHeaders[name] = value;
                return XMLHttpRequest.prototype.setRequestHeader.apply(this, arguments);
            };
            
            XMLHttpRequest.prototype.send = function(body) {
                const xhr = this;
                const logEntry = {
                    url: xhr._requestUrl,
                    method: xhr._requestMethod,
                    timestamp: new Date().toISOString(),
                    request_headers: xhr._requestHeaders,
                    request_body: typeof body === 'string' ? body : null,
                    type_: 'xhr'
                };
                
                // Listen for load
                xhr.addEventListener('load', function() {
                    logEntry.status = xhr.status;
                    logEntry.status_text = xhr.statusText;
                    logEntry.duration = Date.now() - xhr._startTime;
                    
                    // Parse response headers
                    const responseHeaders = {};
                    const headerString = xhr.getAllResponseHeaders();
                    if (headerString) {
                        const headerLines = headerString.split('\r\n');
                        for (const line of headerLines) {
                            if (line) {
                                const parts = line.split(': ');
                                responseHeaders[parts[0]] = parts[1];
                            }
                        }
                    }
                    logEntry.response_headers = responseHeaders;
                    
                    // Try to get response text
                    if (xhr.responseType === '' || xhr.responseType === 'text') {
                        logEntry.response_body = xhr.responseText;
                    }
                    
                    // Add to logs
                    window.network_logs.push(logEntry);
                });
                
                // Listen for error
                xhr.addEventListener('error', function() {
                    logEntry.error = 'Network Error';
                    logEntry.duration = Date.now() - xhr._startTime;
                    
                    // Add to logs
                    window.network_logs.push(logEntry);
                });
                
                return originalXHRSend.apply(this, arguments);
            };
        }
        
        return true;
        "#;
        
        // Execute the scripts and check results
        let console_result = client.execute_js_script(console_script).await?;
        let network_result = client.execute_js_script(network_script).await?;
        
        // Check that the script results indicate success
        if console_result == "true" && network_result == "true" {
            debug!("Successfully injected monitoring scripts");
            Ok(())
        } else {
            Err(anyhow!("Failed to inject monitoring scripts"))
        }
    }

    // Helper method to log error messages
    fn log_error(error_msg: &str) {
        error!("Browser tools error: {}", error_msg);
        
        // Add extra logging for specific error types
        if error_msg.contains("ChromeDriver version mismatch") {
            warn!("This appears to be a ChromeDriver version mismatch. Visit https://chromedriver.chromium.org/downloads to get the matching version.");
        }
    }
} 