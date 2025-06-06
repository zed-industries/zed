//! UI Planner Agent for the CodeOrbit extension.
//! 
//! This agent is responsible for planning and coordinating UI-related tasks,
//! such as layout generation and component organization.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::core::agent_registry::Agent;
use crate::core::error::Result;

/// Represents a UI component in the application.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiComponent {
    /// The unique identifier for the component.
    pub id: String,
    /// The type of the component (e.g., "button", "form", "card").
    pub component_type: String,
    /// The properties of the component.
    pub props: serde_json::Value,
    /// The children components.
    pub children: Vec<UiComponent>,
}

/// The UI Planner Agent is responsible for planning and coordinating UI-related tasks.
pub struct UiPlannerAgent {
    /// The unique identifier for this agent.
    id: String,
}

impl UiPlannerAgent {
    /// Creates a new instance of the UI Planner Agent.
    pub fn new() -> Self {
        Self {
            id: "ui_planner_agent".to_string(),
        }
    }
    
    /// Generates a UI layout based on the given requirements.
    pub async fn generate_layout(&self, requirements: &str) -> Result<Vec<UiComponent>> {
        log::info!("Generating UI layout for requirements: {}", requirements);
        
        // TODO: Implement actual layout generation logic
        // This is a placeholder implementation
        let layout = vec![
            UiComponent {
                id: "header".to_string(),
                component_type: "header".to_string(),
                props: serde_json::json!({"title": "CodeOrbit"}),
                children: vec![],
            },
            UiComponent {
                id: "main-content".to_string(),
                component_type: "container".to_string(),
                props: serde_json::json!({}),
                children: vec![
                    UiComponent {
                        id: "sidebar".to_string(),
                        component_type: "sidebar".to_string(),
                        props: serde_json::json!({}),
                        children: vec![],
                    },
                    UiComponent {
                        id: "editor".to_string(),
                        component_type: "editor".to_string(),
                        props: serde_json::json!({}),
                        children: vec![],
                    },
                ],
            },
        ];
        
        Ok(layout)
    }
}

#[async_trait]
impl Agent for UiPlannerAgent {
    /// Returns the unique identifier for this agent.
    fn id(&self) -> &str {
        &self.id
    }
    
    /// Initializes the agent.
    async fn initialize(&mut self) -> Result<()> {
        log::info!("Initializing UI Planner Agent");
        Ok(())
    }
    
    /// Processes a request and returns a response.
    async fn process(&self, request: &str) -> Result<String> {
        log::debug!("UI Planner Agent processing request: {}", request);
        
        // Parse the request
        let request: serde_json::Value = serde_json::from_str(request)
            .map_err(|e| crate::core::error::Error::DeserializationError(e.to_string()))?;
        
        // Extract the action and parameters
        let action = request["action"].as_str()
            .ok_or_else(|| crate::core::error::Error::AgentError("Missing 'action' in request".to_string()))?;
        
        match action {
            "generate_layout" => {
                let requirements = request["requirements"].as_str()
                    .ok_or_else(|| crate::core::error::Error::AgentError("Missing 'requirements' in request".to_string()))?;
                
                let layout = self.generate_layout(requirements).await?;
                let response = serde_json::to_string(&layout)
                    .map_err(|e| crate::core::error::Error::SerializationError(e.to_string()))?;
                
                Ok(response)
            },
            _ => Err(crate::core::error::Error::AgentError(format!("Unknown action: {}", action))),
        }
    }
    
    /// Shuts down the agent and cleans up resources.
    async fn shutdown(&self) -> Result<()> {
        log::info!("Shutting down UI Planner Agent");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_generate_layout() {
        let agent = UiPlannerAgent::new();
        let requirements = "A simple editor with a sidebar and main content area";
        
        let layout = agent.generate_layout(requirements).await.unwrap();
        assert!(!layout.is_empty());
        
        // Check that we have at least a header and main content
        let has_header = layout.iter().any(|c| c.id == "header");
        let has_main_content = layout.iter().any(|c| c.id == "main-content");
        
        assert!(has_header);
        assert!(has_main_content);
    }
    
    #[tokio::test]
    async fn test_process_generate_layout() {
        let agent = UiPlannerAgent::new();
        let request = json!({
            "action": "generate_layout",
            "requirements": "A simple editor with a sidebar"
        });
        
        let response = agent.process(&request.to_string()).await.unwrap();
        let layout: Vec<UiComponent> = serde_json::from_str(&response).unwrap();
        
        assert!(!layout.is_empty());
    }
    
    #[tokio::test]
    async fn test_process_invalid_action() {
        let agent = UiPlannerAgent::new();
        let request = json!({
            "action": "invalid_action",
            "requirements": "Some requirements"
        });
        
        let result = agent.process(&request.to_string()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::core::error::Error::AgentError(_)));
    }
}
