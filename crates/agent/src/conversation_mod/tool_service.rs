use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult, ToolWorkingSet};
use gpui::{AnyWindowHandle, App, AsyncApp, Context, Entity};
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestTool, 
    LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolUseId,
};

use crate::conversation_mod::conversation::MessageId;

/// Service responsible for handling tool execution
pub trait ToolService: Send + Sync {
    /// Get available tools for a model
    fn available_tools(
        &self, 
        model: Arc<dyn LanguageModel>,
        cx: &Context<impl ?Sized>,
    ) -> Vec<LanguageModelRequestTool>;
    
    /// Run a tool
    fn run_tool(
        &self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        input: serde_json::Value,
        message_id: MessageId,
        request: Arc<LanguageModelRequest>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<LanguageModelToolResult>> + Send>>;
    
    /// Handle a hallucinated tool (called when the model requests a tool that doesn't exist)
    fn handle_hallucinated_tool(
        &self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        error_message: Option<String>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<LanguageModelToolResult>> + Send>>;
}

/// Default implementation of ToolService
pub struct DefaultToolService {
    tools: Entity<ToolWorkingSet>,
}

impl DefaultToolService {
    pub fn new(tools: Entity<ToolWorkingSet>) -> Self {
        Self { tools }
    }
}

impl ToolService for DefaultToolService {
    fn available_tools(
        &self, 
        model: Arc<dyn LanguageModel>,
        cx: &Context<impl ?Sized>,
    ) -> Vec<LanguageModelRequestTool> {
        let mut available_tools = Vec::new();
        
        // Get enabled tools
        let enabled_tools = self.tools.read(cx).enabled_tools(cx);
        
        // Add each enabled tool
        for tool in enabled_tools {
            // Get tool schema in the correct format for the model
            let schema_format = model.tool_schema_format();
            if let Ok(schema) = tool.input_schema(schema_format) {
                available_tools.push(LanguageModelRequestTool {
                    name: tool.name().into(),
                    description: Some(tool.description().into()),
                    schema,
                });
            }
        }
        
        available_tools
    }
    
    fn run_tool(
        &self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        input: serde_json::Value,
        message_id: MessageId,
        request: Arc<LanguageModelRequest>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<LanguageModelToolResult>> + Send>> {
        let tools = self.tools.clone();
        
        Box::pin(async move {
            // Find the tool by name
            let tool = tools.read(cx).tool(&tool_name, cx)
                .ok_or_else(|| anyhow!("Tool '{}' does not exist", tool_name))?;
            
            // Get project and action log
            let project = request.project.clone();
            let action_log = request.action_log.clone();
            
            // Run the tool
            let tool_result = tool.run(
                input.clone(),
                request.clone(),
                project.clone(),
                action_log.clone(),
                model.clone(),
                window.clone(),
                cx,
            );
            
            // Extract and convert the result
            let result_output = tool_result.output.await?;
            
            // Create a tool result
            Ok(LanguageModelToolResult {
                tool_use_id: tool_use_id.clone(),
                tool_name: tool_name.clone(),
                content: match result_output.content {
                    crate::ToolResultContent::Text(text) => 
                        LanguageModelToolResultContent::Text(text.into()),
                    crate::ToolResultContent::Image(image) => 
                        LanguageModelToolResultContent::Image(image),
                },
                is_error: false,
                output: result_output.output,
            })
        })
    }
    
    fn handle_hallucinated_tool(
        &self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        error_message: Option<String>,
        cx: &mut AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<LanguageModelToolResult>> + Send>> {
        // Create an error message
        let error = error_message.unwrap_or_else(|| 
            format!("Tool '{}' does not exist", tool_name)
        );
        
        // Return error as a tool result
        Box::pin(async move {
            Ok(LanguageModelToolResult {
                tool_use_id,
                tool_name,
                content: LanguageModelToolResultContent::Text(error.into()),
                is_error: true,
                output: None,
            })
        })
    }
} 