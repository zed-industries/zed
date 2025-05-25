use anyhow::{anyhow, Result};
use assistant_tool::{Tool, ToolResult, ToolResultOutput, ToolSource};
use chrono;
use gpui::{AnyWindowHandle, App, Entity};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ui::IconName;

use crate::schema;

/// Tool for creating and managing model context protocols
pub struct ContextProtocolTool {
    /// Storage for context protocols
    protocols: Arc<Mutex<HashMap<String, ModelContextProtocol>>>,
}

/// Model context protocol structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelContextProtocol {
    /// Protocol name
    pub name: String,
    /// Protocol version
    pub version: String,
    /// Protocol description
    pub description: String,
    /// Interaction patterns
    pub patterns: Vec<InteractionPattern>,
    /// Context management rules
    pub context_rules: ContextRules,
    /// Thinking strategies
    pub thinking_strategies: Vec<ThinkingStrategy>,
    /// Created timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// Interaction pattern for model context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractionPattern {
    /// Pattern name
    pub name: String,
    /// Pattern trigger conditions
    pub trigger: String,
    /// Pattern template
    pub template: String,
    /// Expected outputs
    pub outputs: Vec<String>,
    /// Pattern priority
    pub priority: u32,
}

/// Rules for managing context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextRules {
    /// Maximum context windows
    pub max_windows: usize,
    /// Context prioritization strategy
    pub prioritization: String,
    /// Memory management strategy
    pub memory_strategy: String,
    /// Context compression enabled
    pub compression_enabled: bool,
    /// Context retention policy
    pub retention_policy: String,
}

/// Thinking strategy for human-like reasoning
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThinkingStrategy {
    /// Strategy name
    pub name: String,
    /// Strategy type
    pub strategy_type: String,
    /// Strategy parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Strategy conditions
    pub conditions: Vec<String>,
}

/// Input for the context protocol tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContextProtocolInput {
    /// Action to perform
    pub action: ContextProtocolAction,
    /// Protocol name (for actions that require it)
    pub protocol_name: Option<String>,
    /// Protocol data (for create/update actions)
    pub protocol_data: Option<ProtocolData>,
    /// Pattern data (for adding patterns)
    pub pattern_data: Option<PatternData>,
    /// Strategy data (for adding strategies)
    pub strategy_data: Option<StrategyData>,
}

/// Actions that can be performed with context protocols
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContextProtocolAction {
    /// Create a new protocol
    Create,
    /// Update an existing protocol
    Update,
    /// Delete a protocol
    Delete,
    /// List all protocols
    List,
    /// Get details of a specific protocol
    Get,
    /// Add a pattern to a protocol
    AddPattern,
    /// Add a strategy to a protocol
    AddStrategy,
    /// Analyze and suggest improvements
    Analyze,
    /// Export protocol to file
    Export,
    /// Import protocol from file
    Import,
}

/// Data for creating/updating a protocol
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProtocolData {
    pub name: String,
    pub description: String,
    pub max_windows: Option<usize>,
    pub prioritization: Option<String>,
    pub memory_strategy: Option<String>,
    pub compression_enabled: Option<bool>,
    pub retention_policy: Option<String>,
}

/// Data for adding a pattern
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PatternData {
    pub name: String,
    pub trigger: String,
    pub template: String,
    pub outputs: Vec<String>,
    pub priority: Option<u32>,
}

/// Data for adding a strategy
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StrategyData {
    pub name: String,
    pub strategy_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub conditions: Vec<String>,
}

impl ContextProtocolTool {
    /// Create a new context protocol tool
    pub fn new() -> Self {
        let mut protocols = HashMap::new();
        
        // Add default protocols
        protocols.insert(
            "chain_of_thought".to_string(),
            ModelContextProtocol {
                name: "Chain of Thought".to_string(),
                version: "1.0".to_string(),
                description: "Sequential reasoning with step-by-step problem decomposition".to_string(),
                patterns: vec![
                    InteractionPattern {
                        name: "problem_decomposition".to_string(),
                        trigger: "complex_problem".to_string(),
                        template: "Let me break this down:\n1. {step1}\n2. {step2}\n3. {step3}".to_string(),
                        outputs: vec!["steps".to_string(), "conclusion".to_string()],
                        priority: 1,
                    },
                ],
                context_rules: ContextRules {
                    max_windows: 5,
                    prioritization: "recency_weighted".to_string(),
                    memory_strategy: "hierarchical".to_string(),
                    compression_enabled: true,
                    retention_policy: "adaptive".to_string(),
                },
                thinking_strategies: vec![
                    ThinkingStrategy {
                        name: "sequential_analysis".to_string(),
                        strategy_type: "chain_of_thought".to_string(),
                        parameters: HashMap::new(),
                        conditions: vec!["complexity > 0.7".to_string()],
                    },
                ],
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
        );

        Self {
            protocols: Arc::new(Mutex::new(protocols)),
        }
    }

    fn create_protocol(&self, data: ProtocolData) -> Result<ModelContextProtocol> {
        let mut protocols = self.protocols.lock().unwrap();
        
        if protocols.contains_key(&data.name) {
            return Err(anyhow!("Protocol '{}' already exists", data.name));
        }

        let protocol = ModelContextProtocol {
            name: data.name.clone(),
            version: "1.0".to_string(),
            description: data.description,
            patterns: Vec::new(),
            context_rules: ContextRules {
                max_windows: data.max_windows.unwrap_or(5),
                prioritization: data.prioritization.unwrap_or_else(|| "adaptive".to_string()),
                memory_strategy: data.memory_strategy.unwrap_or_else(|| "dynamic".to_string()),
                compression_enabled: data.compression_enabled.unwrap_or(true),
                retention_policy: data.retention_policy.unwrap_or_else(|| "smart".to_string()),
            },
            thinking_strategies: Vec::new(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        };

        protocols.insert(data.name, protocol.clone());
        Ok(protocol)
    }

    fn add_pattern(&self, protocol_name: &str, pattern: PatternData) -> Result<()> {
        let mut protocols = self.protocols.lock().unwrap();
        
        let protocol = protocols.get_mut(protocol_name)
            .ok_or_else(|| anyhow!("Protocol '{}' not found", protocol_name))?;

        protocol.patterns.push(InteractionPattern {
            name: pattern.name,
            trigger: pattern.trigger,
            template: pattern.template,
            outputs: pattern.outputs,
            priority: pattern.priority.unwrap_or(1),
        });

        protocol.modified_at = chrono::Utc::now();
        Ok(())
    }

    fn add_strategy(&self, protocol_name: &str, strategy: StrategyData) -> Result<()> {
        let mut protocols = self.protocols.lock().unwrap();
        
        let protocol = protocols.get_mut(protocol_name)
            .ok_or_else(|| anyhow!("Protocol '{}' not found", protocol_name))?;

        protocol.thinking_strategies.push(ThinkingStrategy {
            name: strategy.name,
            strategy_type: strategy.strategy_type,
            parameters: strategy.parameters,
            conditions: strategy.conditions,
        });

        protocol.modified_at = chrono::Utc::now();
        Ok(())
    }

    fn analyze_protocol(&self, protocol_name: &str) -> Result<String> {
        let protocols = self.protocols.lock().unwrap();
        let protocol = protocols.get(protocol_name)
            .ok_or_else(|| anyhow!("Protocol '{}' not found", protocol_name))?;

        let mut analysis = format!("Analysis of protocol '{}':\n\n", protocol_name);
        
        // Analyze patterns
        analysis.push_str(&format!("Patterns: {} defined\n", protocol.patterns.len()));
        if protocol.patterns.is_empty() {
            analysis.push_str("  - Warning: No interaction patterns defined\n");
        }
        
        // Analyze strategies
        analysis.push_str(&format!("Strategies: {} defined\n", protocol.thinking_strategies.len()));
        if protocol.thinking_strategies.is_empty() {
            analysis.push_str("  - Warning: No thinking strategies defined\n");
        }
        
        // Analyze context rules
        analysis.push_str(&format!("\nContext Rules:\n"));
        analysis.push_str(&format!("  - Max Windows: {}\n", protocol.context_rules.max_windows));
        analysis.push_str(&format!("  - Prioritization: {}\n", protocol.context_rules.prioritization));
        analysis.push_str(&format!("  - Memory Strategy: {}\n", protocol.context_rules.memory_strategy));
        
        // Suggestions
        analysis.push_str("\nSuggestions:\n");
        if protocol.patterns.len() < 3 {
            analysis.push_str("  - Consider adding more interaction patterns for better coverage\n");
        }
        if protocol.thinking_strategies.is_empty() {
            analysis.push_str("  - Add thinking strategies to enable advanced reasoning\n");
        }
        
        Ok(analysis)
    }
}

impl Tool for ContextProtocolTool {
    fn name(&self) -> String {
        "create_context_protocol".to_string()
    }

    fn description(&self) -> String {
        "Create and manage model context protocols for advanced AI reasoning and interaction patterns".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Brain
    }

    fn source(&self) -> ToolSource {
        ToolSource::Native
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
        false
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        schema::json_schema_for::<ContextProtocolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        let action = input
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let protocol_name = input
            .get("protocol_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unnamed");

        match action {
            "create" => format!("Creating new context protocol: {}", 
                input.get("protocol_data")
                    .and_then(|d| d.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("unnamed")),
            "list" => "Listing all context protocols".to_string(),
            "get" => format!("Getting context protocol: {}", protocol_name),
            "add_pattern" => format!("Adding pattern to protocol: {}", protocol_name),
            "add_strategy" => format!("Adding strategy to protocol: {}", protocol_name),
            "analyze" => format!("Analyzing protocol: {}", protocol_name),
            _ => format!("Performing {} on context protocols", action),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        _project: Entity<Project>,
        _action_log: Entity<assistant_tool::ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input: ContextProtocolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(e) => {
                return ToolResult::from(cx.background_executor().spawn(async move {
                    Ok(ToolResultOutput::from(format!("Invalid input: {}", e)))
                }));
            }
        };

        let result = match input.action {
            ContextProtocolAction::Create => {
                match input.protocol_data {
                    Some(data) => match self.create_protocol(data) {
                        Ok(protocol) => serde_json::to_string_pretty(&protocol)
                            .unwrap_or_else(|e| format!("Failed to serialize protocol: {}", e)),
                        Err(e) => format!("Failed to create protocol: {}", e),
                    },
                    None => "Protocol data required for create action".to_string(),
                }
            }
            ContextProtocolAction::List => {
                let protocols = self.protocols.lock().unwrap();
                let names: Vec<String> = protocols.keys().cloned().collect();
                serde_json::to_string_pretty(&names)
                    .unwrap_or_else(|e| format!("Failed to serialize list: {}", e))
            }
            ContextProtocolAction::Get => {
                match input.protocol_name {
                    Some(name) => {
                        let protocols = self.protocols.lock().unwrap();
                        match protocols.get(&name) {
                            Some(protocol) => serde_json::to_string_pretty(&protocol)
                                .unwrap_or_else(|e| format!("Failed to serialize protocol: {}", e)),
                            None => format!("Protocol '{}' not found", name),
                        }
                    }
                    None => "Protocol name required for get action".to_string(),
                }
            }
            ContextProtocolAction::AddPattern => {
                match (input.protocol_name, input.pattern_data) {
                    (Some(name), Some(pattern)) => {
                        match self.add_pattern(&name, pattern) {
                            Ok(_) => "Pattern added successfully".to_string(),
                            Err(e) => format!("Failed to add pattern: {}", e),
                        }
                    }
                    _ => "Protocol name and pattern data required for add_pattern action".to_string(),
                }
            }
            ContextProtocolAction::AddStrategy => {
                match (input.protocol_name, input.strategy_data) {
                    (Some(name), Some(strategy)) => {
                        match self.add_strategy(&name, strategy) {
                            Ok(_) => "Strategy added successfully".to_string(),
                            Err(e) => format!("Failed to add strategy: {}", e),
                        }
                    }
                    _ => "Protocol name and strategy data required for add_strategy action".to_string(),
                }
            }
            ContextProtocolAction::Analyze => {
                match input.protocol_name {
                    Some(name) => {
                        match self.analyze_protocol(&name) {
                            Ok(analysis) => analysis,
                            Err(e) => format!("Failed to analyze protocol: {}", e),
                        }
                    }
                    None => "Protocol name required for analyze action".to_string(),
                }
            }
            _ => format!("Action {:?} not implemented yet", input.action),
        };

        ToolResult::from(cx.background_executor().spawn(async move {
            Ok(ToolResultOutput::from(result))
        }))
    }
} 