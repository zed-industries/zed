use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use uide::{UnifiedDataEngine, RecordId, universal::{UniversalRecord, UniversalContent, Value, DataType, RecordMetadata, StructuredBuilder}};
use chrono::{DateTime, Utc};

use super::{ModelConfig, ModelType, ModelProvider, ModelCapability, ModelParameters};

/// Manages model configurations with persistent storage via UIDE
pub struct ModelManager {
    models: Arc<RwLock<HashMap<Uuid, ModelConfig>>>,
    uide_engine: Option<UnifiedDataEngine>,
    default_model_id: Arc<RwLock<Option<Uuid>>>,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            uide_engine: None,
            default_model_id: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn with_persistence(uide_path: impl Into<String>) -> Result<Self> {
        let uide_engine = UnifiedDataEngine::new(uide_path.into()).await
            .map_err(|e| anyhow::anyhow!("Failed to create UIDE engine: {}", e))?;
        
        let mut manager = Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            uide_engine: Some(uide_engine),
            default_model_id: Arc::new(RwLock::new(None)),
        };

        // Load existing models from storage
        manager.load_models_from_storage().await?;

        Ok(manager)
    }

    /// Add a new model configuration
    pub async fn add_model(&self, mut model: ModelConfig) -> Result<Uuid> {
        model.update_timestamp();
        let model_id = model.id;

        // Store in memory
        if let Ok(mut models) = self.models.write() {
            models.insert(model_id, model.clone());
        }

        // Persist to storage if available
        if let Some(ref uide) = self.uide_engine {
            self.save_model_to_storage(&model, uide).await?;
        }

        println!("📝 Added model: {} ({})", model.name, model_id);
        Ok(model_id)
    }

    /// Get a model by ID
    pub fn get_model(&self, id: &Uuid) -> Option<ModelConfig> {
        if let Ok(models) = self.models.read() {
            models.get(id).cloned()
        } else {
            None
        }
    }

    /// Get all models
    pub fn get_all_models(&self) -> Vec<ModelConfig> {
        if let Ok(models) = self.models.read() {
            models.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get models by type
    pub fn get_models_by_type(&self, model_type: &ModelType) -> Vec<ModelConfig> {
        if let Ok(models) = self.models.read() {
            models.values()
                .filter(|model| std::mem::discriminant(&model.model_type) == std::mem::discriminant(model_type))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get models by capability
    pub fn get_models_by_capability(&self, capability: &ModelCapability) -> Vec<ModelConfig> {
        if let Ok(models) = self.models.read() {
            models.values()
                .filter(|model| model.supports_capability(capability))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Update a model configuration
    pub async fn update_model(&self, mut model: ModelConfig) -> Result<()> {
        model.update_timestamp();
        let model_id = model.id;

        // Update in memory
        if let Ok(mut models) = self.models.write() {
            models.insert(model_id, model.clone());
        }

        // Update in storage if available
        if let Some(ref uide) = self.uide_engine {
            self.save_model_to_storage(&model, uide).await?;
        }

        println!("🔄 Updated model: {} ({})", model.name, model_id);
        Ok(())
    }

    /// Delete a model
    pub async fn delete_model(&self, id: &Uuid) -> Result<bool> {
        let model_name = if let Ok(models) = self.models.read() {
            models.get(id).map(|m| m.name.clone())
        } else {
            None
        };

        // Remove from memory
        let removed = if let Ok(mut models) = self.models.write() {
            models.remove(id).is_some()
        } else {
            false
        };

        // Remove from storage if available
        if let Some(ref uide) = self.uide_engine {
            // Search for the model record by ID and delete it
            match self.find_and_delete_model_record(id, uide).await {
                Ok(storage_deleted) => {
                    if !storage_deleted {
                        println!("⚠️  Model record not found in storage: {}", id);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to delete model from storage: {}", e);
                    // Continue anyway since we removed it from memory
                }
            }
        }

        if removed {
            if let Some(name) = model_name {
                println!("🗑️  Deleted model: {} ({})", name, id);
            }
        }

        Ok(removed)
    }

    async fn find_and_delete_model_record(&self, model_id: &Uuid, uide: &UnifiedDataEngine) -> Result<bool> {
        // Search for the model record by model_id field
        let query = uide::query::UniversalQuery::builder()
            .filter_tag("ai_studio_model".to_string())
            .build()?;

        let results = uide.search(query).await?;
        
        for result in results.results {
            if let UniversalContent::Structured { fields, .. } = &result.record.content {
                if let Some(Value::String(stored_id)) = fields.get("model_id") {
                    if let Ok(stored_uuid) = Uuid::parse_str(stored_id) {
                        if stored_uuid == *model_id {
                            // Found the record, delete it
                            match uide.delete(result.record.id).await {
                                Ok(deleted) => {
                                    if deleted {
                                        println!("🗑️  Deleted model record from storage: {}", result.record.id);
                                    }
                                    return Ok(deleted);
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to delete record {}: {}", result.record.id, e);
                                    return Err(e.into());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(false) // Not found
    }

    /// Set default model
    pub fn set_default_model(&self, id: Uuid) -> Result<()> {
        if let Ok(models) = self.models.read() {
            if models.contains_key(&id) {
                if let Ok(mut default_id) = self.default_model_id.write() {
                    *default_id = Some(id);
                    println!("✅ Set default model: {}", id);
                    return Ok(());
                }
            }
        }
        Err(anyhow::anyhow!("Model not found: {}", id))
    }

    /// Get default model
    pub fn get_default_model(&self) -> Option<ModelConfig> {
        if let Ok(default_id) = self.default_model_id.read() {
            if let Some(id) = *default_id {
                return self.get_model(&id);
            }
        }
        
        // Fallback to first active model
        if let Ok(models) = self.models.read() {
            models.values()
                .find(|model| model.is_active)
                .cloned()
        } else {
            None
        }
    }

    /// Create some default models for initial setup
    pub async fn create_default_models(&self) -> Result<()> {
        let default_models = vec![
            ModelConfig::new(
                "GPT-4".to_string(),
                ModelType::LanguageModel,
                ModelProvider::OpenAI {
                    api_key: "your-api-key".to_string(),
                    base_url: None,
                    organization: None,
                },
            )
            .with_description("OpenAI GPT-4 language model".to_string())
            .with_capabilities(vec![
                ModelCapability::TextGeneration,
                ModelCapability::CodeGeneration,
                ModelCapability::FunctionCalling,
                ModelCapability::StreamingResponse,
                ModelCapability::ConversationMemory,
            ]),

            ModelConfig::new(
                "Claude 3 Sonnet".to_string(),
                ModelType::LanguageModel,
                ModelProvider::Anthropic {
                    api_key: "your-api-key".to_string(),
                },
            )
            .with_description("Anthropic Claude 3 Sonnet model".to_string())
            .with_capabilities(vec![
                ModelCapability::TextGeneration,
                ModelCapability::CodeGeneration,
                ModelCapability::ToolUse,
                ModelCapability::StreamingResponse,
                ModelCapability::ConversationMemory,
            ]),

            ModelConfig::new(
                "Local Llama".to_string(),
                ModelType::LanguageModel,
                ModelProvider::Ollama {
                    base_url: "http://localhost:11434".to_string(),
                    model_name: "llama2".to_string(),
                },
            )
            .with_description("Local Llama model via Ollama".to_string())
            .with_capabilities(vec![
                ModelCapability::TextGeneration,
                ModelCapability::CodeGeneration,
                ModelCapability::StreamingResponse,
            ]),

            ModelConfig::new(
                "Code Specialist".to_string(),
                ModelType::CodeModel,
                ModelProvider::HuggingFace {
                    api_key: None,
                    model_id: "codellama/CodeLlama-7b-Instruct-hf".to_string(),
                },
            )
            .with_description("Specialized code generation model".to_string())
            .with_capabilities(vec![
                ModelCapability::CodeGeneration,
                ModelCapability::TextGeneration,
            ]),
        ];

        for model in default_models {
            self.add_model(model).await?;
        }

        // Set the first model as default
        if let Some(first_model) = self.get_all_models().first() {
            self.set_default_model(first_model.id)?;
        }

        println!("🎯 Created {} default models", self.get_all_models().len());
        Ok(())
    }

    async fn save_model_to_storage(&self, model: &ModelConfig, uide: &UnifiedDataEngine) -> Result<RecordId> {
        let content = StructuredBuilder::new()
            .text_field("name", &model.name)
            .text_field("description", &model.description)
            .field("model_type", Value::String(format!("{:?}", model.model_type)))
            .field("provider", serde_json::to_value(&model.provider).map(|v| self.json_to_uide_value(v))?)
            .field("parameters", serde_json::to_value(&model.parameters).map(|v| self.json_to_uide_value(v))?)
            .field("capabilities", Value::Array(
                model.capabilities.iter()
                    .map(|cap| Value::String(format!("{:?}", cap)))
                    .collect()
            ))
            .field("created_at", Value::String(model.created_at.to_rfc3339()))
            .field("updated_at", Value::String(model.updated_at.to_rfc3339()))
            .field("is_active", Value::Bool(model.is_active))
            .field("model_id", Value::String(model.id.to_string()))
            .build();

        let metadata = RecordMetadata::new()
            .with_tags(vec!["ai_studio_model".to_string(), format!("model_type_{:?}", model.model_type)])
            .with_source("ai_studio_models".to_string())
            .with_confidence(1.0);

        let record = UniversalRecord::new(DataType::Structured, content)
            .with_metadata(metadata);

        uide.store_record(record).await
            .map_err(|e| anyhow::anyhow!("Failed to store model: {}", e))
    }

    async fn load_models_from_storage(&mut self) -> Result<()> {
        if let Some(ref uide) = self.uide_engine {
            let query = uide::query::UniversalQuery::by_type(DataType::Structured);
            let results = uide.search(query).await?;

            let mut loaded_count = 0;
            for result in results.results {
                // Check if this is a model record
                if result.record.metadata.tags.contains(&"ai_studio_model".to_string()) {
                    if let UniversalContent::Structured { fields, .. } = &result.record.content {
                        if let Some(model) = self.parse_model_from_fields(fields)? {
                            if let Ok(mut models) = self.models.write() {
                                models.insert(model.id, model);
                                loaded_count += 1;
                            }
                        }
                    }
                }
            }

            println!("📂 Loaded {} models from storage", loaded_count);
        }

        Ok(())
    }

    fn parse_model_from_fields(&self, fields: &indexmap::IndexMap<String, Value>) -> Result<Option<ModelConfig>> {
        // Extract basic fields
        let name = match fields.get("name") {
            Some(Value::String(s)) => s.clone(),
            _ => return Ok(None),
        };

        let description = match fields.get("description") {
            Some(Value::String(s)) => s.clone(),
            _ => String::new(),
        };

        let model_id = match fields.get("model_id") {
            Some(Value::String(s)) => Uuid::parse_str(s)?,
            _ => return Ok(None),
        };

        // Parse model type
        let model_type = match fields.get("model_type") {
            Some(Value::String(s)) => match s.as_str() {
                "LanguageModel" => ModelType::LanguageModel,
                "CodeModel" => ModelType::CodeModel,
                "MultiModal" => ModelType::MultiModal,
                _ => return Ok(None),
            },
            _ => return Ok(None),
        };

        // Parse provider (this is complex due to enum variants)
        let provider = match fields.get("provider") {
            Some(provider_value) => self.parse_provider_from_value(provider_value)?,
            _ => return Ok(None),
        };

        // Parse parameters
        let parameters = match fields.get("parameters") {
            Some(params_value) => self.parse_parameters_from_value(params_value)?,
            _ => ModelParameters::default(),
        };

        // Parse capabilities
        let capabilities = match fields.get("capabilities") {
            Some(Value::Array(caps)) => {
                let mut capabilities = Vec::new();
                for cap in caps {
                    if let Value::String(cap_str) = cap {
                        match cap_str.as_str() {
                            "TextGeneration" => capabilities.push(ModelCapability::TextGeneration),
                            "CodeGeneration" => capabilities.push(ModelCapability::CodeGeneration),
                            "FunctionCalling" => capabilities.push(ModelCapability::FunctionCalling),
                            "ToolUse" => capabilities.push(ModelCapability::ToolUse),
                            "StreamingResponse" => capabilities.push(ModelCapability::StreamingResponse),
                            "ConversationMemory" => capabilities.push(ModelCapability::ConversationMemory),
                            "MultiModal" => capabilities.push(ModelCapability::MultiModal),
                            _ => {}
                        }
                    }
                }
                capabilities
            },
            _ => Vec::new(),
        };

        // Parse timestamps
        let created_at = match fields.get("created_at") {
            Some(Value::String(s)) => DateTime::parse_from_rfc3339(s)?.with_timezone(&Utc),
            _ => Utc::now(),
        };

        let updated_at = match fields.get("updated_at") {
            Some(Value::String(s)) => DateTime::parse_from_rfc3339(s)?.with_timezone(&Utc),
            _ => Utc::now(),
        };

        let is_active = match fields.get("is_active") {
            Some(Value::Bool(b)) => *b,
            _ => true,
        };

        // Create the model config
        let model = ModelConfig {
            id: model_id,
            name,
            description,
            model_type,
            provider,
            parameters,
            capabilities,
            created_at,
            updated_at,
            is_active,
        };

        Ok(Some(model))
    }

    fn parse_provider_from_value(&self, value: &Value) -> Result<ModelProvider> {
        match value {
            Value::Object(obj) => {
                // Handle serde enum serialization format
                if let Some(openai_value) = obj.get("OpenAI") {
                    if let Value::Object(openai_obj) = openai_value {
                        let api_key = openai_obj.get("api_key")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                            .ok_or_else(|| anyhow::anyhow!("Missing api_key for OpenAI provider"))?;
                        
                        let base_url = openai_obj.get("base_url")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None });
                        
                        let organization = openai_obj.get("organization")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None });
                        
                        return Ok(ModelProvider::OpenAI {
                            api_key,
                            base_url,
                            organization,
                        });
                    }
                } else if let Some(anthropic_value) = obj.get("Anthropic") {
                    if let Value::Object(anthropic_obj) = anthropic_value {
                        let api_key = anthropic_obj.get("api_key")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                            .ok_or_else(|| anyhow::anyhow!("Missing api_key for Anthropic provider"))?;
                        
                        return Ok(ModelProvider::Anthropic { api_key });
                    }
                } else if let Some(local_value) = obj.get("Local") {
                    if let Value::Object(local_obj) = local_value {
                        let model_path = local_obj.get("model_path")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                            .ok_or_else(|| anyhow::anyhow!("Missing model_path for Local provider"))?;
                        
                        let executable_path = local_obj.get("executable_path")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None });
                        
                        return Ok(ModelProvider::Local {
                            model_path,
                            executable_path,
                        });
                    }
                } else if let Some(ollama_value) = obj.get("Ollama") {
                    if let Value::Object(ollama_obj) = ollama_value {
                        let base_url = ollama_obj.get("base_url")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                            .ok_or_else(|| anyhow::anyhow!("Missing base_url for Ollama provider"))?;
                        
                        let model_name = ollama_obj.get("model_name")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                            .ok_or_else(|| anyhow::anyhow!("Missing model_name for Ollama provider"))?;
                        
                        return Ok(ModelProvider::Ollama {
                            base_url,
                            model_name,
                        });
                    }
                } else if let Some(hf_value) = obj.get("HuggingFace") {
                    if let Value::Object(hf_obj) = hf_value {
                        let model_id = hf_obj.get("model_id")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                            .ok_or_else(|| anyhow::anyhow!("Missing model_id for HuggingFace provider"))?;
                        
                        let api_key = hf_obj.get("api_key")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None });
                        
                        return Ok(ModelProvider::HuggingFace {
                            api_key,
                            model_id,
                        });
                    }
                } else if let Some(custom_value) = obj.get("Custom") {
                    if let Value::Object(custom_obj) = custom_value {
                        let name = custom_obj.get("name")
                            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                            .ok_or_else(|| anyhow::anyhow!("Missing name for Custom provider"))?;
                        
                        let config = custom_obj.get("config")
                            .and_then(|v| self.uide_value_to_json(v).ok())
                            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));
                        
                        return Ok(ModelProvider::Custom {
                            name,
                            config,
                        });
                    }
                }
            }
            _ => {}
        }
        
        Err(anyhow::anyhow!("Failed to parse provider from value"))
    }

    fn parse_parameters_from_value(&self, value: &Value) -> Result<ModelParameters> {
        let mut params = ModelParameters::default();
        
        if let Value::Object(obj) = value {
            if let Some(Value::Number(n)) = obj.get("max_tokens") {
                params.max_tokens = Some(*n as u32);
            }
            if let Some(Value::Number(n)) = obj.get("temperature") {
                params.temperature = Some(*n as f32);
            }
            if let Some(Value::Number(n)) = obj.get("top_p") {
                params.top_p = Some(*n as f32);
            }
            if let Some(Value::Number(n)) = obj.get("frequency_penalty") {
                params.frequency_penalty = Some(*n as f32);
            }
            if let Some(Value::Number(n)) = obj.get("presence_penalty") {
                params.presence_penalty = Some(*n as f32);
            }
            if let Some(Value::Array(stop_array)) = obj.get("stop_sequences") {
                let mut stop_sequences = Vec::new();
                for item in stop_array {
                    if let Value::String(s) = item {
                        stop_sequences.push(s.clone());
                    }
                }
                if !stop_sequences.is_empty() {
                    params.stop_sequences = Some(stop_sequences);
                }
            }
        }
        
        Ok(params)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn uide_value_to_json(&self, value: &Value) -> Result<serde_json::Value> {
        let json_value = match value {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(*b),
            Value::Number(n) => {
                if let Some(num) = serde_json::Number::from_f64(*n) {
                    serde_json::Value::Number(num)
                } else {
                    serde_json::Value::Null
                }
            },
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Array(arr) => {
                let mut json_arr = Vec::new();
                for item in arr {
                    json_arr.push(self.uide_value_to_json(item)?);
                }
                serde_json::Value::Array(json_arr)
            }
            Value::Object(obj) => {
                let mut json_obj = serde_json::Map::new();
                for (k, v) in obj {
                    json_obj.insert(k.clone(), self.uide_value_to_json(v)?);
                }
                serde_json::Value::Object(json_obj)
            }
            Value::Binary(_) => serde_json::Value::Null,
        };
        Ok(json_value)
    }

    #[allow(clippy::only_used_in_recursion)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_model_persistence() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_models.db");
        
        let model_id = {
            // Create a model manager with persistence
            let manager = ModelManager::with_persistence(db_path.to_string_lossy().to_string()).await.unwrap();
            
            // Create a test model
            let test_model = ModelConfig::new(
                "Test Model".to_string(),
                ModelType::LanguageModel,
                ModelProvider::OpenAI {
                    api_key: "test-key".to_string(),
                    base_url: None,
                    organization: None,
                },
            ).with_description("A test model for persistence".to_string());
            
            // Add the model
            let model_id = manager.add_model(test_model.clone()).await.unwrap();
            
            // Verify it was added
            let retrieved_model = manager.get_model(&model_id).unwrap();
            assert_eq!(retrieved_model.name, "Test Model");
            assert_eq!(retrieved_model.description, "A test model for persistence");
            
            model_id
        }; // Drop the first manager here
        
        // Create a new manager instance to test loading from storage
        let manager2 = ModelManager::with_persistence(db_path.to_string_lossy().to_string()).await.unwrap();
        
        // Verify the model was loaded from storage
        let loaded_model = manager2.get_model(&model_id).unwrap();
        assert_eq!(loaded_model.name, "Test Model");
        assert_eq!(loaded_model.description, "A test model for persistence");
        
        println!("✅ Model persistence test passed!");
    }

    #[tokio::test]
    async fn test_complete_model_workflow() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("workflow_test.db");
        
        // Test 1: Create manager and add multiple models
        let manager = ModelManager::with_persistence(db_path.to_string_lossy().to_string()).await.unwrap();
        
        // Add different types of models
        let openai_model = ModelConfig::new(
            "GPT-4".to_string(),
            ModelType::LanguageModel,
            ModelProvider::OpenAI {
                api_key: "sk-test".to_string(),
                base_url: Some("https://api.openai.com/v1".to_string()),
                organization: Some("org-test".to_string()),
            },
        ).with_description("OpenAI GPT-4 model".to_string());
        
        let anthropic_model = ModelConfig::new(
            "Claude-3".to_string(),
            ModelType::LanguageModel,
            ModelProvider::Anthropic {
                api_key: "sk-ant-test".to_string(),
            },
        ).with_description("Anthropic Claude-3 model".to_string());
        
        let local_model = ModelConfig::new(
            "Llama-2-7B".to_string(),
            ModelType::LanguageModel,
            ModelProvider::Local {
                model_path: "/path/to/llama-2-7b.gguf".to_string(),
                executable_path: Some("/usr/local/bin/llama-cpp".to_string()),
            },
        ).with_description("Local Llama-2 model".to_string());
        
        // Add all models
        let openai_id = manager.add_model(openai_model).await.unwrap();
        let anthropic_id = manager.add_model(anthropic_model).await.unwrap();
        let local_id = manager.add_model(local_model).await.unwrap();
        
        // Verify all models exist
        assert_eq!(manager.get_all_models().len(), 3);
        
        // Test 2: Set default model
        manager.set_default_model(openai_id).unwrap();
        let default_model = manager.get_default_model().unwrap();
        assert_eq!(default_model.name, "GPT-4");
        
        // Test 3: Update a model
        let mut updated_model = manager.get_model(&anthropic_id).unwrap();
        updated_model.description = "Updated Claude-3 model".to_string();
        manager.update_model(updated_model).await.unwrap();
        
        let retrieved_model = manager.get_model(&anthropic_id).unwrap();
        assert_eq!(retrieved_model.description, "Updated Claude-3 model");
        
        // Test 4: Filter by type
        let language_models = manager.get_models_by_type(&ModelType::LanguageModel);
        assert_eq!(language_models.len(), 3);
        
        // Test 5: Delete a model
        let deleted = manager.delete_model(&local_id).await.unwrap();
        assert!(deleted);
        assert_eq!(manager.get_all_models().len(), 2);
        
        println!("✅ Complete model workflow test passed!");
    }
} 