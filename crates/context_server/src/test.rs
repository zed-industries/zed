//! Test utilities for context servers using RMCP
//!
//! Note: The old fake transport system has been removed in favor of RMCP.
//! Test setups now need to use RMCP's built-in testing utilities.

use std::sync::Arc;

/// Mock context server configuration for testing
#[derive(Debug, Clone)]
pub struct MockContextServerConfig {
    pub name: String,
    pub tools: Vec<rmcp::model::Tool>,
    pub prompts: Vec<rmcp::model::Prompt>,
    pub resources: Vec<rmcp::model::Resource>,
}

impl MockContextServerConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tools: Vec::new(),
            prompts: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn with_tool(mut self, tool: rmcp::model::Tool) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn with_prompt(mut self, prompt: rmcp::model::Prompt) -> Self {
        self.prompts.push(prompt);
        self
    }

    pub fn with_resource(mut self, resource: rmcp::model::Resource) -> Self {
        self.resources.push(resource);
        self
    }
}

/// Creates a simple test tool for testing purposes
pub fn create_test_tool(name: impl Into<String>) -> rmcp::model::Tool {
    let schema_map = serde_json::json!({
        "type": "object",
        "properties": {
            "message": {
                "type": "string",
                "description": "Message to echo"
            }
        },
        "required": ["message"]
    })
    .as_object()
    .unwrap()
    .clone();

    rmcp::model::Tool {
        name: name.into().into(),
        description: Some("Test tool".into()),
        input_schema: Arc::new(schema_map),
        output_schema: None,
        annotations: None,
    }
}

/// Creates a simple test prompt for testing purposes
pub fn create_test_prompt(name: impl Into<String>) -> rmcp::model::Prompt {
    rmcp::model::Prompt {
        name: name.into().into(),
        description: Some("Test prompt".into()),
        arguments: Some(vec![rmcp::model::PromptArgument {
            name: "input".into(),
            description: Some("Input parameter".into()),
            required: Some(true),
        }]),
    }
}

/// Creates a simple test resource for testing purposes
pub fn create_test_resource(
    name: impl Into<String>,
    uri: impl Into<String>,
) -> rmcp::model::Resource {
    rmcp::model::Resource {
        raw: rmcp::model::RawResource {
            uri: uri.into().into(),
            name: name.into().into(),
            description: Some("Test resource".into()),
            mime_type: Some("text/plain".into()),
        },
        annotations: None,
    }
}

/// Test utilities for RMCP-based context servers
pub struct TestContextServer {
    config: MockContextServerConfig,
}

impl TestContextServer {
    pub fn new(config: MockContextServerConfig) -> Self {
        Self { config }
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn tools(&self) -> &[rmcp::model::Tool] {
        &self.config.tools
    }

    pub fn prompts(&self) -> &[rmcp::model::Prompt] {
        &self.config.prompts
    }

    pub fn resources(&self) -> &[rmcp::model::Resource] {
        &self.config.resources
    }
}

// TODO: Implement actual RMCP-based test server when needed
// This would involve creating a mock RMCP server that can be used in tests
// For now, tests that require context server functionality will need to be
// updated or disabled until proper RMCP test infrastructure is in place.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_config_creation() {
        let config = MockContextServerConfig::new("test_server")
            .with_tool(create_test_tool("echo"))
            .with_prompt(create_test_prompt("test_prompt"))
            .with_resource(create_test_resource("test_resource", "test://resource/1"));

        assert_eq!(config.name, "test_server");
        assert_eq!(config.tools.len(), 1);
        assert_eq!(config.prompts.len(), 1);
        assert_eq!(config.resources.len(), 1);
    }

    #[test]
    fn test_create_test_tool() {
        let tool = create_test_tool("test_tool");
        assert_eq!(tool.name, "test_tool");
        assert!(tool.description.is_some());
        assert!(!tool.input_schema.is_empty());
    }

    #[test]
    fn test_create_test_prompt() {
        let prompt = create_test_prompt("test_prompt");
        assert_eq!(prompt.name, "test_prompt");
        assert!(prompt.description.is_some());
        assert!(prompt.arguments.is_some());
    }

    #[test]
    fn test_create_test_resource() {
        let resource = create_test_resource("test_resource", "test://uri");
        assert_eq!(resource.raw.name, "test_resource");
        assert_eq!(resource.raw.uri, "test://uri");
        assert!(resource.raw.description.is_some());
        assert!(resource.raw.mime_type.is_some());
    }
}
