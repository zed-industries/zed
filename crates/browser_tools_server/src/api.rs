use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Utc;

use crate::client::BrowserToolsClient;
use crate::models::{
    AccessibilityAuditParams, AuditOutput, BestPracticesAuditParams, CaptureScreenshotInput,
    ConsoleLogsOutput, ElementInfoOutput, GetConsoleLogsInput, GetNetworkLogsInput, NetworkLog,
    NetworkLogsOutput, PerformanceAuditParams, RunAuditInput, ScreenshotOutput, SeoAuditParams,
};

/// Description of a tool available in the browser tools API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescription {
    /// Name of the tool
    pub name: String,
    /// Optional description of the tool
    pub description: Option<String>,
    /// Input schema for the tool
    pub input_schema: Value,
}

/// Result of running a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Content of the tool result
    pub content: Vec<ToolResultContent>,
    /// Whether the result is an error
    pub is_error: Option<bool>,
    /// Additional metadata
    pub meta: Option<HashMap<String, Value>>,
}

/// Content of a tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolResultContent {
    /// Text content
    #[serde(rename = "text")]
    Text { text: String },
    /// Image content
    #[serde(rename = "image")]
    Image { mime_type: String, data: String },
}

#[derive(Clone)]
pub struct BrowserToolsApi {
    client: BrowserToolsClient,
}

impl BrowserToolsApi {
    pub fn new(client: BrowserToolsClient) -> Self {
        Self { client }
    }

    pub fn available_tools(&self) -> Vec<ToolDescription> {
        vec![
            ToolDescription {
                name: "getConsoleLogs".to_string(),
                description: Some("Get browser console logs".to_string()),
                input_schema: serde_json::to_value(&GetConsoleLogsInput {
                    filter: None,
                    limit: None,
                }).unwrap_or_default(),
            },
            ToolDescription {
                name: "getConsoleErrors".to_string(),
                description: Some("Get browser console errors".to_string()),
                input_schema: serde_json::to_value(&GetConsoleLogsInput {
                    filter: None,
                    limit: None,
                }).unwrap_or_default(),
            },
            ToolDescription {
                name: "getNetworkLogs".to_string(),
                description: Some("Get browser network logs".to_string()),
                input_schema: serde_json::to_value(&GetNetworkLogsInput {
                    filter: None,
                    limit: None,
                    include_bodies: None,
                }).unwrap_or_default(),
            },
            ToolDescription {
                name: "getNetworkErrors".to_string(),
                description: Some("Get browser network errors".to_string()),
                input_schema: serde_json::to_value(&GetNetworkLogsInput {
                    filter: None,
                    limit: None,
                    include_bodies: None,
                }).unwrap_or_default(),
            },
            ToolDescription {
                name: "captureScreenshot".to_string(),
                description: Some("Capture browser screenshot".to_string()),
                input_schema: serde_json::to_value(&CaptureScreenshotInput {
                    full_page: None,
                    element_selector: None,
                }).unwrap_or_default(),
            },
            ToolDescription {
                name: "getSelectedElement".to_string(),
                description: Some("Get information about selected DOM element".to_string()),
                input_schema: Value::Null,
            },
            ToolDescription {
                name: "clearLogs".to_string(),
                description: Some("Clear browser logs".to_string()),
                input_schema: Value::Null,
            },
            ToolDescription {
                name: "runAudit".to_string(),
                description: Some("Run browser audit (accessibility, performance, SEO, best practices)".to_string()),
                input_schema: serde_json::to_value(&RunAuditInput {
                    audit_type: "accessibility".to_string(), // Default value
                    device: None,
                    throttling: None,
                }).unwrap_or_default(),
            },
        ]
    }

    pub async fn run_tool(
        &self,
        tool_name: String,
        arguments: Option<HashMap<String, Value>>,
    ) -> Result<ToolResult> {
        debug!("Running tool: {} with arguments: {:?}", tool_name, arguments);

        let arguments = arguments.unwrap_or_default();

        match tool_name.as_str() {
            "getConsoleLogs" => self.get_console_logs(arguments).await,
            "getConsoleErrors" => self.get_console_errors(arguments).await,
            "getNetworkLogs" => self.get_network_logs(arguments).await,
            "getNetworkErrors" => self.get_network_errors(arguments).await,
            "captureScreenshot" => self.capture_screenshot(arguments).await,
            "getSelectedElement" => self.get_selected_element().await,
            "clearLogs" => self.clear_logs().await,
            "runAudit" => self.run_audit(arguments).await,
            _ => Err(anyhow!("Unknown tool: {}", tool_name)),
        }
    }

    async fn get_console_logs(&self, arguments: HashMap<String, Value>) -> Result<ToolResult> {
        let limit = arguments
            .get("limit")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let filter = arguments
            .get("filter")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let logs = self.client.get_console_logs().await?;

        let filtered_logs = if let Some(filter_text) = filter {
            logs.into_iter()
                .filter(|log| log.message.contains(&filter_text))
                .collect()
        } else {
            logs
        };

        let filtered_logs = if let Some(limit_count) = limit {
            filtered_logs
                .into_iter()
                .take(limit_count as usize)
                .collect()
        } else {
            filtered_logs
        };

        let output = ConsoleLogsOutput {
            logs: filtered_logs.clone(),
            count: filtered_logs.len() as i32,
        };

        let content = format!("Retrieved {} console logs", filtered_logs.len());
        
        Ok(ToolResult {
            content: vec![ToolResultContent::Text { text: content }],
            is_error: None,
            meta: Some(HashMap::from([
                ("logs".to_string(), serde_json::to_value(&output)?)
            ])),
        })
    }

    async fn get_console_errors(&self, arguments: HashMap<String, Value>) -> Result<ToolResult> {
        let limit = arguments
            .get("limit")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let filter = arguments
            .get("filter")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let logs = self.client.get_console_errors().await?;

        let filtered_logs = if let Some(filter_text) = filter {
            logs.into_iter()
                .filter(|log| log.message.contains(&filter_text))
                .collect()
        } else {
            logs
        };

        let filtered_logs = if let Some(limit_count) = limit {
            filtered_logs
                .into_iter()
                .take(limit_count as usize)
                .collect()
        } else {
            filtered_logs
        };

        let output = ConsoleLogsOutput {
            logs: filtered_logs.clone(),
            count: filtered_logs.len() as i32,
        };

        let content = format!("Retrieved {} console errors", filtered_logs.len());
        
        Ok(ToolResult {
            content: vec![ToolResultContent::Text { text: content }],
            is_error: None,
            meta: Some(HashMap::from([
                ("logs".to_string(), serde_json::to_value(&output)?)
            ])),
        })
    }

    async fn get_network_logs(&self, arguments: HashMap<String, Value>) -> Result<ToolResult> {
        let limit = arguments
            .get("limit")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let filter = arguments
            .get("filter")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let include_bodies = arguments
            .get("include_bodies")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let logs = self.client.get_all_xhr().await?;

        let filtered_logs = if let Some(filter_text) = filter {
            logs.into_iter()
                .filter(|log| log.url.contains(&filter_text))
                .collect()
        } else {
            logs
        };

        let filtered_logs = if let Some(limit_count) = limit {
            filtered_logs
                .into_iter()
                .take(limit_count as usize)
                .collect()
        } else {
            filtered_logs
        };

        // Remove request and response bodies if not requested
        let logs_without_bodies: Vec<NetworkLog> = if !include_bodies {
            filtered_logs
                .into_iter()
                .map(|mut log| {
                    log.request_body = None;
                    log.response_body = None;
                    log
                })
                .collect()
        } else {
            filtered_logs
        };

        let output = NetworkLogsOutput {
            logs: logs_without_bodies.clone(),
            count: logs_without_bodies.len() as i32,
        };

        let content = format!("Retrieved {} network logs", logs_without_bodies.len());
        
        Ok(ToolResult {
            content: vec![ToolResultContent::Text { text: content }],
            is_error: None,
            meta: Some(HashMap::from([
                ("logs".to_string(), serde_json::to_value(&output)?)
            ])),
        })
    }

    async fn get_network_errors(&self, arguments: HashMap<String, Value>) -> Result<ToolResult> {
        let limit = arguments
            .get("limit")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let filter = arguments
            .get("filter")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let include_bodies = arguments
            .get("include_bodies")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let logs = self.client.get_network_errors().await?;

        let filtered_logs = if let Some(filter_text) = filter {
            logs.into_iter()
                .filter(|log| log.url.contains(&filter_text))
                .collect()
        } else {
            logs
        };

        let filtered_logs = if let Some(limit_count) = limit {
            filtered_logs
                .into_iter()
                .take(limit_count as usize)
                .collect()
        } else {
            filtered_logs
        };

        // Remove request and response bodies if not requested
        let logs_without_bodies: Vec<NetworkLog> = if !include_bodies {
            filtered_logs
                .into_iter()
                .map(|mut log| {
                    log.request_body = None;
                    log.response_body = None;
                    log
                })
                .collect()
        } else {
            filtered_logs
        };

        let output = NetworkLogsOutput {
            logs: logs_without_bodies.clone(),
            count: logs_without_bodies.len() as i32,
        };

        let content = format!("Retrieved {} network errors", logs_without_bodies.len());
        
        Ok(ToolResult {
            content: vec![ToolResultContent::Text { text: content }],
            is_error: None,
            meta: Some(HashMap::from([
                ("logs".to_string(), serde_json::to_value(&output)?)
            ])),
        })
    }

    async fn capture_screenshot(&self, arguments: HashMap<String, Value>) -> Result<ToolResult> {
        let full_page = arguments
            .get("full_page")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let element_selector = arguments
            .get("element_selector")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // TODO: Implement element_selector and full_page logic
        let _ = (full_page, element_selector);

        // Capture screenshot
        let screenshot_data = self.client.capture_screenshot().await?;
        
        let now = Utc::now().timestamp_millis().to_string();
        
        // Define the output
        let _output = ScreenshotOutput {
            url: screenshot_data.clone(),
            timestamp: now,
        };

        let content = "Screenshot captured successfully".to_string();
        
        Ok(ToolResult {
            content: vec![
                ToolResultContent::Text { text: content },
                ToolResultContent::Image { data: screenshot_data, mime_type: "image/png".to_string() },
            ],
            is_error: None,
            meta: None,
        })
    }

    async fn get_selected_element(&self) -> Result<ToolResult> {
        let element_info = self.client.get_selected_element().await?;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let output = ElementInfoOutput {
            element: element_info.clone(),
            timestamp,
        };

        let content = format!("Selected element: {}", element_info.selector);
        
        Ok(ToolResult {
            content: vec![ToolResultContent::Text { text: content }],
            is_error: None,
            meta: Some(HashMap::from([
                ("element".to_string(), serde_json::to_value(&output)?)
            ])),
        })
    }

    async fn clear_logs(&self) -> Result<ToolResult> {
        self.client.clear_logs().await?;
        
        let content = "Browser logs cleared successfully".to_string();
        
        Ok(ToolResult {
            content: vec![ToolResultContent::Text { text: content }],
            is_error: None,
            meta: None,
        })
    }

    async fn run_audit(&self, arguments: HashMap<String, Value>) -> Result<ToolResult> {
        let audit_type = arguments
            .get("audit_type")
            .and_then(|v| v.as_str())
            .unwrap_or("accessibility")
            .to_string();

        let device = arguments
            .get("device")
            .and_then(|v| v.as_str())
            .unwrap_or("desktop")
            .to_string();

        let _throttling = arguments
            .get("throttling")
            .and_then(|v| v.as_object())
            .map(|obj| obj.clone())
            .unwrap_or_else(serde_json::Map::new);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let result = match audit_type.as_str() {
            "accessibility" => {
                let params = AccessibilityAuditParams {
                    category: "accessibility".to_string(),
                    source: device,
                    timestamp: timestamp as i64,
                };
                self.client.run_accessibility_audit(params).await?
            }
            "performance" => {
                let params = PerformanceAuditParams {
                    category: "performance".to_string(),
                    source: device,
                    timestamp: timestamp as i64,
                };
                self.client.run_performance_audit(params).await?
            }
            "seo" => {
                let params = SeoAuditParams {
                    category: "seo".to_string(),
                    source: device,
                    timestamp: timestamp as i64,
                };
                self.client.run_seo_audit(params).await?
            }
            "best-practices" => {
                let params = BestPracticesAuditParams {
                    category: "best-practices".to_string(),
                    source: device,
                    timestamp: timestamp as i64,
                };
                self.client.run_best_practices_audit(params).await?
            }
            _ => return Err(anyhow!("Unknown audit type: {}", audit_type)),
        };

        let output = AuditOutput {
            report: result.clone(),
            score: 0, // We would calculate a score in a real implementation
            timestamp: timestamp.to_string(),
        };

        let content = format!("Completed {} audit", audit_type);
        
        Ok(ToolResult {
            content: vec![ToolResultContent::Text { text: content }],
            is_error: None,
            meta: Some(HashMap::from([
                ("audit".to_string(), serde_json::to_value(&output)?)
            ])),
        })
    }
} 