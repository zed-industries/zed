// Tool registry for agent tools
// This will contain GPUI renderer and other tools for agents

use anyhow::Result;
use std::collections::HashMap;

/// Registry for tools that agents can use
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn AgentTool>>,
}

pub trait AgentTool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value>;
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register_tool(&mut self, tool: Box<dyn AgentTool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn AgentTool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    pub fn list_tools(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }
} 