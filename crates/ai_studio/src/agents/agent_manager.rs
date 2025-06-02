use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use uide::{UnifiedDataEngine, RecordId, universal::{UniversalRecord, UniversalContent, Value, DataType, RecordMetadata, StructuredBuilder}};

use super::{AgentConfig, AgentType, AgentInstance, AgentStatus};

/// Manages agent configurations and instances with persistent storage via UIDE
pub struct AgentManager {
    agents: Arc<RwLock<HashMap<Uuid, AgentConfig>>>,
    active_instances: Arc<RwLock<HashMap<Uuid, AgentInstance>>>,
    uide_engine: Option<UnifiedDataEngine>,
}

impl AgentManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            active_instances: Arc::new(RwLock::new(HashMap::new())),
            uide_engine: None,
        })
    }

    pub async fn with_persistence(uide_path: impl Into<String>) -> Result<Self> {
        let uide_engine = UnifiedDataEngine::new(uide_path.into()).await
            .map_err(|e| anyhow::anyhow!("Failed to create UIDE engine: {}", e))?;
        
        let mut manager = Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            active_instances: Arc::new(RwLock::new(HashMap::new())),
            uide_engine: Some(uide_engine),
        };

        // Load existing agents from storage
        manager.load_agents_from_storage().await?;

        Ok(manager)
    }

    /// Add a new agent configuration
    pub async fn add_agent(&self, mut agent: AgentConfig) -> Result<Uuid> {
        agent.update_timestamp();
        let agent_id = agent.id;

        // Store in memory
        if let Ok(mut agents) = self.agents.write() {
            agents.insert(agent_id, agent.clone());
        }

        // Persist to storage if available
        if let Some(ref uide) = self.uide_engine {
            self.save_agent_to_storage(&agent, uide).await?;
        }

        println!("🤖 Added agent: {} ({})", agent.name, agent_id);
        Ok(agent_id)
    }

    /// Get an agent by ID
    pub fn get_agent(&self, id: &Uuid) -> Option<AgentConfig> {
        if let Ok(agents) = self.agents.read() {
            agents.get(id).cloned()
        } else {
            None
        }
    }

    /// Get all agent configurations
    pub fn get_all_agents(&self) -> Vec<AgentConfig> {
        if let Ok(agents) = self.agents.read() {
            agents.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get agents by type
    pub fn get_agents_by_type(&self, agent_type: &AgentType) -> Vec<AgentConfig> {
        if let Ok(agents) = self.agents.read() {
            agents.values()
                .filter(|agent| std::mem::discriminant(&agent.agent_type) == std::mem::discriminant(agent_type))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Update an agent configuration
    pub async fn update_agent(&self, mut agent: AgentConfig) -> Result<()> {
        agent.update_timestamp();
        let agent_id = agent.id;

        // Update in memory
        if let Ok(mut agents) = self.agents.write() {
            agents.insert(agent_id, agent.clone());
        }

        // Update in storage if available
        if let Some(ref uide) = self.uide_engine {
            self.save_agent_to_storage(&agent, uide).await?;
        }

        println!("🔄 Updated agent: {} ({})", agent.name, agent_id);
        Ok(())
    }

    /// Delete an agent
    pub async fn delete_agent(&self, id: &Uuid) -> Result<bool> {
        let agent_name = if let Ok(agents) = self.agents.read() {
            agents.get(id).map(|a| a.name.clone())
        } else {
            None
        };

        // Remove from memory
        let removed = if let Ok(mut agents) = self.agents.write() {
            agents.remove(id).is_some()
        } else {
            false
        };

        // Also remove any active instances
        if let Ok(mut instances) = self.active_instances.write() {
            instances.retain(|_, instance| instance.config.id != *id);
        }

        if removed {
            if let Some(name) = agent_name {
                println!("🗑️  Deleted agent: {} ({})", name, id);
            }
        }

        Ok(removed)
    }

    /// Spawn an agent instance from configuration
    pub async fn spawn_agent(&self, agent_id: &Uuid, parent_agent: Option<Uuid>) -> Result<Uuid> {
        if let Some(config) = self.get_agent(agent_id) {
            let instance = AgentInstance::new(config, parent_agent);
            let instance_id = instance.session_id;

            // Store active instance
            if let Ok(mut instances) = self.active_instances.write() {
                instances.insert(instance_id, instance);
            }

            println!("🚀 Spawned agent instance: {} ({})", instance_id, agent_id);
            Ok(instance_id)
        } else {
            Err(anyhow::anyhow!("Agent configuration not found: {}", agent_id))
        }
    }

    /// Get an active agent instance
    pub fn get_instance(&self, instance_id: &Uuid) -> Option<AgentInstance> {
        if let Ok(instances) = self.active_instances.read() {
            instances.get(instance_id).cloned()
        } else {
            None
        }
    }

    /// Get all active agent instances
    pub fn get_all_instances(&self) -> Vec<AgentInstance> {
        if let Ok(instances) = self.active_instances.read() {
            instances.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Shutdown an agent instance
    pub fn shutdown_instance(&self, instance_id: &Uuid) -> Result<()> {
        if let Ok(mut instances) = self.active_instances.write() {
            if let Some(mut instance) = instances.remove(instance_id) {
                instance.status = AgentStatus::Shutdown;
                println!("🛑 Shutdown agent instance: {}", instance_id);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Agent instance not found: {}", instance_id))
            }
        } else {
            Err(anyhow::anyhow!("Failed to access instances"))
        }
    }

    /// Get agents that can spawn other agents
    pub fn get_spawner_agents(&self) -> Vec<AgentConfig> {
        if let Ok(agents) = self.agents.read() {
            agents.values()
                .filter(|agent| agent.can_spawn_agents())
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Create some default agents for initial setup
    pub async fn create_default_agents(&self, model_id: Uuid) -> Result<()> {
        use super::{AgentType, AgentContext, AgentCapability, SpawningRules};

        let default_agents = vec![
            AgentConfig::new(
                "UI Designer".to_string(),
                AgentType::UIDesigner,
                model_id,
            )
            .with_description("Creates dynamic user interfaces using GPUI".to_string())
            .with_context(AgentContext {
                system_prompt: "You are a specialized UI/UX designer agent. You create beautiful, functional user interfaces using GPUI. You understand user needs and can generate appropriate UI components dynamically.".to_string(),
                ..Default::default()
            })
            .with_capabilities(vec![
                AgentCapability::UICreation,
                AgentCapability::CodeGeneration,
                AgentCapability::AgentSpawning,
            ])
            .with_tools(vec!["gpui_renderer".to_string()])
            .with_spawning_rules(SpawningRules {
                can_spawn_agents: true,
                max_child_agents: 2,
                allowed_agent_types: vec![AgentType::CodeSpecialist],
                ..Default::default()
            }),

            AgentConfig::new(
                "Code Specialist".to_string(),
                AgentType::CodeSpecialist,
                model_id,
            )
            .with_description("Generates and refactors code with expertise in Rust and GPUI".to_string())
            .with_context(AgentContext {
                system_prompt: "You are a code specialist with deep expertise in Rust and GPUI. You generate clean, efficient code and help with refactoring and optimization.".to_string(),
                ..Default::default()
            })
            .with_capabilities(vec![
                AgentCapability::CodeGeneration,
                AgentCapability::FileManagement,
                AgentCapability::ProjectAnalysis,
            ])
            .with_tools(vec!["file_manager".to_string(), "git_manager".to_string()]),

            AgentConfig::new(
                "Project Manager".to_string(),
                AgentType::ProjectManager,
                model_id,
            )
            .with_description("Orchestrates complex tasks and manages multiple agents".to_string())
            .with_context(AgentContext {
                system_prompt: "You are a project manager agent. You break down complex tasks, coordinate multiple agents, and ensure project goals are met efficiently.".to_string(),
                ..Default::default()
            })
            .with_capabilities(vec![
                AgentCapability::AgentSpawning,
                AgentCapability::ProjectAnalysis,
                AgentCapability::MemoryManagement,
                AgentCapability::ContextUnderstanding,
            ])
            .with_spawning_rules(SpawningRules {
                can_spawn_agents: true,
                max_child_agents: 5,
                allowed_agent_types: vec![
                    AgentType::UIDesigner,
                    AgentType::CodeSpecialist,
                    AgentType::Debugger,
                    AgentType::TestGenerator,
                ],
                ..Default::default()
            }),
        ];

        for agent in default_agents {
            self.add_agent(agent).await?;
        }

        println!("🎯 Created {} default agents", self.get_all_agents().len());
        Ok(())
    }

    async fn save_agent_to_storage(&self, agent: &AgentConfig, uide: &UnifiedDataEngine) -> Result<RecordId> {
        let content = StructuredBuilder::new()
            .text_field("name", &agent.name)
            .text_field("description", &agent.description)
            .field("agent_type", Value::String(format!("{:?}", agent.agent_type)))
            .field("model_id", Value::String(agent.model_id.to_string()))
            .field("context", serde_json::to_value(&agent.context).map(|v| self.json_to_uide_value(v))?)
            .field("capabilities", Value::Array(
                agent.capabilities.iter()
                    .map(|cap| Value::String(format!("{:?}", cap)))
                    .collect()
            ))
            .field("tools", Value::Array(
                agent.tools.iter()
                    .map(|tool| Value::String(tool.clone()))
                    .collect()
            ))
            .field("spawning_rules", serde_json::to_value(&agent.spawning_rules).map(|v| self.json_to_uide_value(v))?)
            .field("created_at", Value::String(agent.created_at.to_rfc3339()))
            .field("updated_at", Value::String(agent.updated_at.to_rfc3339()))
            .field("is_active", Value::Bool(agent.is_active))
            .field("agent_id", Value::String(agent.id.to_string()))
            .build();

        let metadata = RecordMetadata::new()
            .with_tags(vec!["ai_studio_agent".to_string(), format!("agent_type_{:?}", agent.agent_type)])
            .with_source("ai_studio_agents".to_string())
            .with_confidence(1.0);

        let record = UniversalRecord::new(DataType::Structured, content)
            .with_metadata(metadata);

        uide.store_record(record).await
            .map_err(|e| anyhow::anyhow!("Failed to store agent: {}", e))
    }

    async fn load_agents_from_storage(&mut self) -> Result<()> {
        if let Some(ref uide) = self.uide_engine {
            let query = uide::query::UniversalQuery::by_type(DataType::Structured);
            let results = uide.search(query).await?;

            let mut loaded_count = 0;
            for result in results.results {
                // Check if this is an agent record
                if result.record.metadata.tags.contains(&"ai_studio_agent".to_string()) {
                    if let UniversalContent::Structured { fields, .. } = &result.record.content {
                        if let Some(agent) = self.parse_agent_from_fields(fields)? {
                            if let Ok(mut agents) = self.agents.write() {
                                agents.insert(agent.id, agent);
                                loaded_count += 1;
                            }
                        }
                    }
                }
            }

            println!("📂 Loaded {} agents from storage", loaded_count);
        }

        Ok(())
    }

    fn parse_agent_from_fields(&self, _fields: &indexmap::IndexMap<String, Value>) -> Result<Option<AgentConfig>> {
        // This would parse the UIDE fields back into an AgentConfig
        // Implementation details would depend on the specific serialization format
        // For now, return None to avoid compilation errors
        Ok(None)
    }

    fn json_to_uide_value(&self, json_value: serde_json::Value) -> Value {
        match json_value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                Value::Array(arr.into_iter().map(|v| self.json_to_uide_value(v)).collect())
            }
            serde_json::Value::Object(obj) => {
                let mut uide_obj = indexmap::IndexMap::new();
                for (k, v) in obj {
                    uide_obj.insert(k, self.json_to_uide_value(v));
                }
                Value::Object(uide_obj)
            }
        }
    }
} 