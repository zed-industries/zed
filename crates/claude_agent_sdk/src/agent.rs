//! Agent module - Core agentic loop for Claude
//!
//! This module implements the agent loop that:
//! 1. Sends requests to Claude
//! 2. Receives tool calls
//! 3. Executes tools
//! 4. Returns results to Claude
//! 5. Loops until completion

use anyhow::Result;
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::tools::{Tool, ToolCall, ToolResult};

/// Maximum number of agent loop iterations to prevent infinite loops
pub const MAX_AGENT_ITERATIONS: usize = 100;

/// Configuration for the agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Maximum iterations before stopping
    pub max_iterations: usize,
    /// Whether to auto-execute tools or ask for confirmation
    pub auto_execute_tools: bool,
    /// Tool execution timeout in milliseconds
    pub tool_timeout_ms: u64,
    /// Whether to stream thinking tokens
    pub stream_thinking: bool,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_iterations: MAX_AGENT_ITERATIONS,
            auto_execute_tools: true,
            tool_timeout_ms: 30000,
            stream_thinking: true,
        }
    }
}

/// Agent state during execution
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentState {
    /// Agent is idle, waiting for input
    Idle,
    /// Agent is processing a request
    Processing,
    /// Agent is waiting for tool execution
    WaitingForTool,
    /// Agent is executing a tool
    ExecutingTool(String),
    /// Agent has completed
    Completed,
    /// Agent encountered an error
    Error(String),
}

/// An event emitted by the agent during execution
#[derive(Debug, Clone)]
pub enum AgentEvent {
    /// State changed
    StateChanged(AgentState),
    /// Text output from Claude
    Text(String),
    /// Thinking output from Claude
    Thinking(String),
    /// Tool call requested
    ToolCallRequested(ToolCall),
    /// Tool execution started
    ToolExecutionStarted(String),
    /// Tool execution completed
    ToolExecutionCompleted(ToolResult),
    /// Token usage update
    TokenUsage {
        input_tokens: u64,
        output_tokens: u64,
    },
    /// Agent completed
    Done,
    /// Error occurred
    Error(String),
}

/// The Claude Agent - orchestrates the agentic loop
pub struct Agent {
    /// Configuration
    pub config: AgentConfig,
    /// Available tools
    tools: Vec<Arc<dyn Tool>>,
    /// Current state
    state: AgentState,
    /// Iteration count
    iteration: usize,
}

impl Agent {
    /// Create a new agent with default configuration
    pub fn new() -> Self {
        Self {
            config: AgentConfig::default(),
            tools: Vec::new(),
            state: AgentState::Idle,
            iteration: 0,
        }
    }

    /// Create a new agent with custom configuration
    pub fn with_config(config: AgentConfig) -> Self {
        Self {
            config,
            tools: Vec::new(),
            state: AgentState::Idle,
            iteration: 0,
        }
    }

    /// Register a tool with the agent
    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.push(tool);
    }

    /// Get all registered tools
    pub fn tools(&self) -> &[Arc<dyn Tool>] {
        &self.tools
    }

    /// Get current state
    pub fn state(&self) -> &AgentState {
        &self.state
    }

    /// Get tool by name
    pub fn get_tool(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.iter().find(|t| t.name() == name)
    }

    /// Execute a tool call
    pub async fn execute_tool(&mut self, tool_call: &ToolCall) -> Result<ToolResult> {
        self.state = AgentState::ExecutingTool(tool_call.name.clone());

        let tool = self
            .get_tool(&tool_call.name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", tool_call.name))?
            .clone();

        let result = tool.execute(&tool_call.input).await?;

        self.state = AgentState::Processing;

        Ok(ToolResult {
            tool_use_id: tool_call.id.clone(),
            content: result,
            is_error: false,
        })
    }

    /// Reset the agent state
    pub fn reset(&mut self) {
        self.state = AgentState::Idle;
        self.iteration = 0;
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}
