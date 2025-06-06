//! Component Generator Agent for the CodeOrbit extension.
//! 
//! This agent is responsible for generating UI components based on specifications.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::core::agent_registry::Agent;
use crate::core::error::Result;

/// Represents a generated UI component.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratedComponent {
    /// The name of the component.
    pub name: String,
    /// The source code of the component.
    pub code: String,
    /// The type of the component (e.g., "react", "vue", "svelte").
    pub component_type: String,
    /// Any additional metadata.
    pub metadata: serde_json::Value,
}

/// The Component Generator Agent is responsible for generating UI components.
pub struct ComponentGeneratorAgent {
    /// The unique identifier for this agent.
    id: String,
}

impl ComponentGeneratorAgent {
    /// Creates a new instance of the Component Generator Agent.
    pub fn new() -> Self {
        Self {
            id: "component_generator_agent".to_string(),
        }
    }
    
    /// Generates a UI component based on the given specification.
    pub async fn generate_component(
        &self,
        name: &str,
        spec: &serde_json::Value,
        component_type: &str,
    ) -> Result<GeneratedComponent> {
        log::info!(
            "Generating {} component '{}' with spec: {:?}",
            component_type,
            name,
            spec
        );
        
        // TODO: Implement actual component generation logic
        // This is a placeholder implementation
        let code = match component_type {
            "react" => self.generate_react_component(name, spec).await?,
            "vue" => self.generate_vue_component(name, spec).await?,
            "svelte" => self.generate_svelte_component(name, spec).await?,
            _ => return Err(crate::core::error::Error::AgentError(
                format!("Unsupported component type: {}", component_type)
            )),
        };
        
        Ok(GeneratedComponent {
            name: name.to_string(),
            code,
            component_type: component_type.to_string(),
            metadata: serde_json::json!({}),
        })
    }
    
    /// Generates a React component.
    async fn generate_react_component(&self, name: &str, _spec: &serde_json::Value) -> Result<String> {
        // This is a simplified example
        let code = format!(
            r#"import React from 'react';

interface {name}Props {{
  // Add your props here
}}

export const {name}: React.FC<{name}Props> = ({{ /* props */ }}) => {{
  return (
    <div className="{name}-container">
      <h2>{name} Component</h2>
      {/* Add your component JSX here */}
    </div>
  );
}};

export default {name};
"#
        );
        
        Ok(code)
    }
    
    /// Generates a Vue component.
    async fn generate_vue_component(&self, name: &str, _spec: &serde_json::Value) -> Result<String> {
        // This is a simplified example
        let code = format!(
            r#"<template>
  <div class="{name}-container">
    <h2>{{{{ name }}}}</h2>
    <!-- Add your component template here -->
  </div>
</template>

<script>
export default {{
  name: '{name}',
  // Add your component options here
  data() {{
    return {{
      // Add your reactive data here
    }};
  }},
  methods: {{
    // Add your methods here
  }}
}};
</script>

<style scoped>
.{name}-container {{
  /* Add your styles here */
}}
</style>
"#
        );
        
        Ok(code)
    }
    
    /// Generates a Svelte component.
    async fn generate_svelte_component(&self, name: &str, _spec: &serde_json::Value) -> Result<String> {
        // This is a simplified example
        let code = format!(
            r#"<script>
  // Add your component script here
  export let name = '{name}';
</script>

<div class="{name}-container">
  <h2>{name}</h2>
  <!-- Add your component markup here -->
</div>

<style>
  .{name}-container {{
    /* Add your styles here */
  }}
</style>
"#
        );
        
        Ok(code)
    }
}

#[async_trait]
impl Agent for ComponentGeneratorAgent {
    /// Returns the unique identifier for this agent.
    fn id(&self) -> &str {
        &self.id
    }
    
    /// Initializes the agent.
    async fn initialize(&mut self) -> Result<()> {
        log::info!("Initializing Component Generator Agent");
        Ok(())
    }
    
    /// Processes a request and returns a response.
    async fn process(&self, request: &str) -> Result<String> {
        log::debug!("Component Generator Agent processing request: {}", request);
        
        // Parse the request
        let request: serde_json::Value = serde_json::from_str(request)
            .map_err(|e| crate::core::error::Error::DeserializationError(e.to_string()))?;
        
        // Extract the action and parameters
        let action = request["action"].as_str()
            .ok_or_else(|| crate::core::error::Error::AgentError("Missing 'action' in request".to_string()))?;
        
        match action {
            "generate_component" => {
                let name = request["name"].as_str()
                    .ok_or_else(|| crate::core::error::Error::AgentError("Missing 'name' in request".to_string()))?;
                
                let spec = request["spec"].clone();
                
                let component_type = request["component_type"].as_str()
                    .unwrap_or("react");
                
                let component = self.generate_component(name, &spec, component_type).await?;
                
                let response = serde_json::to_string(&component)
                    .map_err(|e| crate::core::error::Error::SerializationError(e.to_string()))?;
                
                Ok(response)
            },
            _ => Err(crate::core::error::Error::AgentError(format!("Unknown action: {}", action))),
        }
    }
    
    /// Shuts down the agent and cleans up resources.
    async fn shutdown(&self) -> Result<()> {
        log::info!("Shutting down Component Generator Agent");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_generate_react_component() {
        let agent = ComponentGeneratorAgent::new();
        let spec = json!({});
        
        let component = agent.generate_component("TestComponent", &spec, "react").await.unwrap();
        
        assert_eq!(component.name, "TestComponent");
        assert_eq!(component.component_type, "react");
        assert!(component.code.contains("TestComponent"));
        assert!(component.code.contains("React.FC"));
    }
    
    #[tokio::test]
    async fn test_generate_vue_component() {
        let agent = ComponentGeneratorAgent::new();
        let spec = json!({});
        
        let component = agent.generate_component("TestComponent", &spec, "vue").await.unwrap();
        
        assert_eq!(component.name, "TestComponent");
        assert_eq!(component.component_type, "vue");
        assert!(component.code.contains("<template>"));
        assert!(component.code.contains("<script>"));
        assert!(component.code.contains("<style"));
    }
    
    #[tokio::test]
    async fn test_generate_svelte_component() {
        let agent = ComponentGeneratorAgent::new();
        let spec = json!({});
        
        let component = agent.generate_component("TestComponent", &spec, "svelte").await.unwrap();
        
        assert_eq!(component.name, "TestComponent");
        assert_eq!(component.component_type, "svelte");
        assert!(component.code.contains("<script>"));
        assert!(component.code.contains("<style>"));
    }
    
    #[tokio::test]
    async fn test_process_generate_component() {
        let agent = ComponentGeneratorAgent::new();
        let request = json!({
            "action": "generate_component",
            "name": "TestButton",
            "spec": {{
                "type": "button",
                "props": ["label", "onClick"],
                "styles": {}
            }},
            "component_type": "react"
        }});
        
        let response = agent.process(&request.to_string()).await.unwrap();
        let component: GeneratedComponent = serde_json::from_str(&response).unwrap();
        
        assert_eq!(component.name, "TestButton");
        assert_eq!(component.component_type, "react");
        assert!(component.code.contains("TestButton"));
    }
    
    #[tokio::test]
    async fn test_process_invalid_action() {
        let agent = ComponentGeneratorAgent::new();
        let request = json!({
            "action": "invalid_action",
            "name": "TestComponent"
        });
        
        let result = agent.process(&request.to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::core::error::Error::AgentError(_)));
    }
    
    #[tokio::test]
    async fn test_process_unsupported_component_type() {
        let agent = ComponentGeneratorAgent::new();
        let request = json!({
            "action": "generate_component",
            "name": "TestComponent",
            "spec": {{}},
            "component_type": "invalid_type"
        });
        
        let result = agent.process(&request.to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::core::error::Error::AgentError(_)));
    }
}
