use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uide::{
    universal::{
        DataType, RecordMetadata, StructuredBuilder, UniversalContent, UniversalRecord, Value,
    },
    RecordId, UnifiedDataEngine,
};
use uuid::Uuid;

/// AI Configuration for orchestration system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub orchestrator_config: OrchestratorConfig,
    pub role_configs: HashMap<String, RoleConfig>,
    pub global_settings: GlobalAiSettings,
    pub prompt_templates: HashMap<String, PromptTemplate>,
}

/// Configuration for the Master Orchestrator AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    pub model_provider: ModelProvider,
    pub decomposition_strategy: DecompositionStrategy,
    pub context_management: ContextManagementConfig,
    pub quality_checks: QualityCheckConfig,
    pub retry_policy: RetryPolicy,
}

/// Configuration for specialized AI roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConfig {
    pub role_type: AiRole,
    pub model_provider: ModelProvider,
    pub specialization_prompts: Vec<String>,
    pub capability_limits: CapabilityLimits,
    pub handoff_protocols: HandoffProtocols,
    pub quality_metrics: QualityMetrics,
}

/// AI role types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AiRole {
    Architect,
    Developer,
    Reviewer,
    Integrator,
    Tester,
    Documenter,
    ContextManager,
    PromptEngineer,
    QualityAssurance,
    Custom(String),
}

/// Model provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProvider {
    pub provider_type: String, // "openai", "claude", "local", etc.
    pub model_name: String,
    pub api_endpoint: Option<String>,
    pub api_key_ref: Option<String>, // Reference to secure storage
    pub temperature: f32,
    pub max_tokens: u32,
    pub timeout_seconds: u32,
    pub rate_limit: RateLimit,
    pub fallback_models: Vec<String>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub tokens_per_minute: u32,
    pub concurrent_requests: u32,
}

/// Task decomposition strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecompositionStrategy {
    Sequential,
    Parallel,
    Hybrid,
    DependencyBased,
    ComplexityAdaptive,
}

/// Context management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextManagementConfig {
    pub max_context_size: usize,
    pub compression_threshold: f32, // 0.0 to 1.0
    pub persistence_strategy: PersistenceStrategy,
    pub cleanup_policy: CleanupPolicy,
    pub forecasting_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistenceStrategy {
    InMemory,
    FileSystem,
    Database,
    Uide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupPolicy {
    pub max_sessions: u32,
    pub max_age_hours: u32,
    pub auto_compress_after_hours: u32,
}

/// Quality check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCheckConfig {
    pub enabled: bool,
    pub validation_rules: Vec<ValidationRule>,
    pub auto_fix_enabled: bool,
    pub manual_review_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: String,
    pub description: String,
    pub severity: Severity,
    pub auto_fixable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Retry policy for failed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_strategy: BackoffStrategy,
    pub retry_on_errors: Vec<String>,
    pub circuit_breaker_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Linear,
    Exponential,
    Fixed,
}

/// Capability limits for AI roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityLimits {
    pub max_execution_time_minutes: u32,
    pub max_output_size_kb: u32,
    pub max_iterations: u32,
    pub allowed_operations: Vec<String>,
    pub restricted_operations: Vec<String>,
}

/// Handoff protocols between AI roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffProtocols {
    pub required_outputs: Vec<String>,
    pub validation_checks: Vec<String>,
    pub context_transfer_format: String,
    pub notification_rules: Vec<NotificationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRule {
    pub condition: String,
    pub notification_type: String,
    pub recipients: Vec<String>,
}

/// Quality metrics for measuring AI performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub success_rate_threshold: f32,
    pub response_time_threshold_ms: u32,
    pub output_quality_metrics: Vec<String>,
    pub learning_rate: f32,
}

/// Global AI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAiSettings {
    pub logging_level: LoggingLevel,
    pub telemetry_enabled: bool,
    pub learning_enabled: bool,
    pub safety_checks: SafetyChecks,
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoggingLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyChecks {
    pub content_filtering: bool,
    pub output_validation: bool,
    pub harmful_content_detection: bool,
    pub privacy_protection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: u32,
    pub max_cpu_percent: u32,
    pub max_network_requests_per_minute: u32,
    pub max_file_operations_per_minute: u32,
}

/// Prompt template for AI roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub role: AiRole,
    pub template: String,
    pub variables: Vec<PromptVariable>,
    pub examples: Vec<PromptExample>,
    pub validation_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptVariable {
    pub name: String,
    pub description: String,
    pub variable_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptExample {
    pub description: String,
    pub input_variables: HashMap<String, String>,
    pub expected_output_pattern: String,
}

/// AI configuration manager with UIDE integration
pub struct AiConfigManager {
    uide_engine: UnifiedDataEngine,
}

impl AiConfigManager {
    pub async fn new(uide_path: impl Into<String>) -> Result<Self> {
        let uide_path = uide_path.into();
        let uide_engine = UnifiedDataEngine::new(uide_path).await
            .map_err(|e| anyhow::anyhow!("Failed to create UIDE engine: {}", e))?;
        Ok(Self { uide_engine })
    }

    pub async fn with_engine(uide_engine: UnifiedDataEngine) -> Self {
        Self { uide_engine }
    }

    /// Save AI configuration to UIDE
    pub async fn save_config(&self, config: &AiConfig) -> Result<RecordId> {
        let content = StructuredBuilder::new()
            .text_field("name", &config.name)
            .text_field("description", &config.description)
            .text_field("version", &config.version)
            .field("created_at", Value::String(config.created_at.to_rfc3339()))
            .field("updated_at", Value::String(config.updated_at.to_rfc3339()))
            .field("orchestrator_config", self.orchestrator_config_to_value(&config.orchestrator_config)?)
            .field("role_configs", self.role_configs_to_value(&config.role_configs)?)
            .field("global_settings", self.global_settings_to_value(&config.global_settings)?)
            .field("prompt_templates", self.prompt_templates_to_value(&config.prompt_templates)?)
            .build();

        let metadata = RecordMetadata::new()
            .with_tags(config.tags.clone())
            .with_source("ai_studio_config".to_string())
            .with_confidence(1.0);

        let record = UniversalRecord::new(DataType::Structured, content)
            .with_metadata(metadata);

        self.uide_engine.store_record(record).await
            .map_err(|e| anyhow::anyhow!("Failed to store AI config: {}", e))
    }

    /// Load AI configuration from UIDE
    pub async fn load_config(&self, record_id: RecordId) -> Result<Option<AiConfig>> {
        let record = self.uide_engine.get_record(record_id).await?;
        
        match record {
            Some(record) => {
                if let UniversalContent::Structured { fields, .. } = &record.content {
                    self.record_to_config(fields)
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Search AI configurations
    pub async fn search_configs(&self, query: &str) -> Result<Vec<(RecordId, AiConfig)>> {
        let search_query = uide::query::UniversalQuery::text_search(query);
        let results = self.uide_engine.search(search_query).await?;

        let mut configs = Vec::new();
        for result in results.results {
            if result.record.metadata.tags.contains(&"ai_studio_config".to_string()) {
                if let UniversalContent::Structured { fields, .. } = &result.record.content {
                    if let Some(config) = self.record_to_config(fields)? {
                        configs.push((result.record.id, config));
                    }
                }
            }
        }

        Ok(configs)
    }

    /// List all AI configurations
    pub async fn list_configs(&self) -> Result<Vec<(RecordId, AiConfig)>> {
        let query = uide::query::UniversalQuery::by_type(DataType::Structured);
        let results = self.uide_engine.search(query).await?;

        let mut configs = Vec::new();
        for result in results.results {
            if result.record.metadata.tags.contains(&"ai_studio_config".to_string()) {
                if let UniversalContent::Structured { fields, .. } = &result.record.content {
                    if let Some(config) = self.record_to_config(fields)? {
                        configs.push((result.record.id, config));
                    }
                }
            }
        }

        Ok(configs)
    }

    /// Delete AI configuration
    pub async fn delete_config(&self, record_id: RecordId) -> Result<bool> {
        self.uide_engine.delete(record_id).await
            .map_err(|e| anyhow::anyhow!("Failed to delete AI config: {}", e))
    }

    /// Create default AI configuration
    pub fn create_default_config() -> AiConfig {
        AiConfig {
            id: Uuid::new_v4(),
            name: "Default AI Orchestration Config".to_string(),
            description: "Default configuration for Pure AI Task Orchestration System".to_string(),
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: vec!["ai_studio_config".to_string(), "default".to_string()],
            orchestrator_config: OrchestratorConfig {
                model_provider: ModelProvider {
                    provider_type: "openai".to_string(),
                    model_name: "gpt-4".to_string(),
                    api_endpoint: None,
                    api_key_ref: Some("OPENAI_API_KEY".to_string()),
                    temperature: 0.7,
                    max_tokens: 4000,
                    timeout_seconds: 120,
                    rate_limit: RateLimit {
                        requests_per_minute: 60,
                        tokens_per_minute: 100000,
                        concurrent_requests: 5,
                    },
                    fallback_models: vec!["gpt-3.5-turbo".to_string()],
                },
                decomposition_strategy: DecompositionStrategy::ComplexityAdaptive,
                context_management: ContextManagementConfig {
                    max_context_size: 80000, // 80KB
                    compression_threshold: 0.8,
                    persistence_strategy: PersistenceStrategy::Uide,
                    cleanup_policy: CleanupPolicy {
                        max_sessions: 100,
                        max_age_hours: 168, // 1 week
                        auto_compress_after_hours: 24,
                    },
                    forecasting_enabled: true,
                },
                quality_checks: QualityCheckConfig {
                    enabled: true,
                    validation_rules: vec![
                        ValidationRule {
                            rule_type: "syntax_check".to_string(),
                            description: "Validate code syntax".to_string(),
                            severity: Severity::Error,
                            auto_fixable: true,
                        },
                        ValidationRule {
                            rule_type: "completeness_check".to_string(),
                            description: "Check task completeness".to_string(),
                            severity: Severity::Warning,
                            auto_fixable: false,
                        },
                    ],
                    auto_fix_enabled: true,
                    manual_review_threshold: 0.8,
                },
                retry_policy: RetryPolicy {
                    max_retries: 3,
                    backoff_strategy: BackoffStrategy::Exponential,
                    retry_on_errors: vec!["timeout".to_string(), "rate_limit".to_string()],
                    circuit_breaker_threshold: 5,
                },
            },
            role_configs: Self::create_default_role_configs(),
            global_settings: GlobalAiSettings {
                logging_level: LoggingLevel::Info,
                telemetry_enabled: true,
                learning_enabled: true,
                safety_checks: SafetyChecks {
                    content_filtering: true,
                    output_validation: true,
                    harmful_content_detection: true,
                    privacy_protection: true,
                },
                resource_limits: ResourceLimits {
                    max_memory_mb: 2048,
                    max_cpu_percent: 80,
                    max_network_requests_per_minute: 1000,
                    max_file_operations_per_minute: 100,
                },
            },
            prompt_templates: Self::create_default_prompt_templates(),
        }
    }

    fn create_default_role_configs() -> HashMap<String, RoleConfig> {
        let mut configs = HashMap::new();
        
        // Architect AI role
        configs.insert("architect".to_string(), RoleConfig {
            role_type: AiRole::Architect,
            model_provider: ModelProvider {
                provider_type: "openai".to_string(),
                model_name: "gpt-4".to_string(),
                api_endpoint: None,
                api_key_ref: Some("OPENAI_API_KEY".to_string()),
                temperature: 0.3, // Lower temperature for more consistent architectural decisions
                max_tokens: 4000,
                timeout_seconds: 180,
                rate_limit: RateLimit {
                    requests_per_minute: 30,
                    tokens_per_minute: 50000,
                    concurrent_requests: 2,
                },
                fallback_models: vec!["gpt-3.5-turbo".to_string()],
            },
            specialization_prompts: vec![
                "You are a Senior Software Architect specializing in system design.".to_string(),
                "Focus on scalability, maintainability, and performance.".to_string(),
            ],
            capability_limits: CapabilityLimits {
                max_execution_time_minutes: 30,
                max_output_size_kb: 50,
                max_iterations: 5,
                allowed_operations: vec!["design".to_string(), "analyze".to_string(), "plan".to_string()],
                restricted_operations: vec!["execute_code".to_string()],
            },
            handoff_protocols: HandoffProtocols {
                required_outputs: vec!["architecture_diagram".to_string(), "component_specs".to_string()],
                validation_checks: vec!["completeness".to_string(), "feasibility".to_string()],
                context_transfer_format: "structured_json".to_string(),
                notification_rules: vec![],
            },
            quality_metrics: QualityMetrics {
                success_rate_threshold: 0.9,
                response_time_threshold_ms: 30000,
                output_quality_metrics: vec!["clarity".to_string(), "completeness".to_string()],
                learning_rate: 0.1,
            },
        });

        // Developer AI role
        configs.insert("developer".to_string(), RoleConfig {
            role_type: AiRole::Developer,
            model_provider: ModelProvider {
                provider_type: "openai".to_string(),
                model_name: "gpt-4".to_string(),
                api_endpoint: None,
                api_key_ref: Some("OPENAI_API_KEY".to_string()),
                temperature: 0.2, // Very low temperature for consistent code generation
                max_tokens: 6000,
                timeout_seconds: 240,
                rate_limit: RateLimit {
                    requests_per_minute: 20,
                    tokens_per_minute: 80000,
                    concurrent_requests: 3,
                },
                fallback_models: vec!["gpt-3.5-turbo".to_string()],
            },
            specialization_prompts: vec![
                "You are a Senior Rust Developer with expertise in systems programming.".to_string(),
                "Write clean, efficient, and well-tested code.".to_string(),
            ],
            capability_limits: CapabilityLimits {
                max_execution_time_minutes: 45,
                max_output_size_kb: 100,
                max_iterations: 10,
                allowed_operations: vec!["code".to_string(), "test".to_string(), "debug".to_string()],
                restricted_operations: vec!["delete_files".to_string()],
            },
            handoff_protocols: HandoffProtocols {
                required_outputs: vec!["source_code".to_string(), "tests".to_string(), "documentation".to_string()],
                validation_checks: vec!["syntax".to_string(), "compilation".to_string(), "test_coverage".to_string()],
                context_transfer_format: "code_with_comments".to_string(),
                notification_rules: vec![],
            },
            quality_metrics: QualityMetrics {
                success_rate_threshold: 0.95,
                response_time_threshold_ms: 60000,
                output_quality_metrics: vec!["correctness".to_string(), "efficiency".to_string(), "maintainability".to_string()],
                learning_rate: 0.05,
            },
        });

        configs
    }

    fn create_default_prompt_templates() -> HashMap<String, PromptTemplate> {
        let mut templates = HashMap::new();

        templates.insert("master_orchestrator".to_string(), PromptTemplate {
            id: "master_orchestrator".to_string(),
            name: "Master Task Orchestrator".to_string(),
            description: "Main prompt for task decomposition and orchestration".to_string(),
            role: AiRole::Custom("orchestrator".to_string()),
            template: r#"You are the Master Task Orchestrator AI. Your role is to:

1. ANALYZE incoming complex tasks
2. DECOMPOSE them into manageable subtasks
3. DETERMINE optimal execution sequence
4. CREATE specialized prompts for worker AIs
5. COORDINATE execution and integration
6. MAINTAIN context across sessions

TASK DECOMPOSITION FRAMEWORK:
- Identify task complexity and scope
- Break into logical phases with clear boundaries
- Determine required expertise for each phase
- Estimate context requirements
- Create dependency mapping
- Generate handoff protocols

CURRENT TASK: {task_description}

DECOMPOSITION OUTPUT FORMAT:
```json
{
  "task_id": "unique_identifier",
  "phases": [
    {
      "phase_id": "phase_1",
      "name": "descriptive_name",
      "role": "specialized_role",
      "prompt_template": "detailed_prompt_for_worker_ai",
      "context_requirements": ["file1", "file2"],
      "dependencies": ["previous_phase_id"],
      "expected_output": "description_of_deliverables"
    }
  ],
  "integration_strategy": "how_to_combine_results",
  "success_criteria": ["criterion_1", "criterion_2"]
}
```"#.to_string(),
            variables: vec![
                PromptVariable {
                    name: "task_description".to_string(),
                    description: "The complex task to be decomposed".to_string(),
                    variable_type: "string".to_string(),
                    required: true,
                    default_value: None,
                    validation_pattern: Some(r".{10,}".to_string()), // At least 10 characters
                }
            ],
            examples: vec![],
            validation_rules: vec!["output_must_be_valid_json".to_string()],
        });

        templates
    }

    // Helper methods for UIDE conversion (simplified for brevity)
    fn orchestrator_config_to_value(&self, config: &OrchestratorConfig) -> Result<Value> {
        let json_value = serde_json::to_value(config)?;
        self.json_value_to_uide_value(json_value)
    }

    fn role_configs_to_value(&self, configs: &HashMap<String, RoleConfig>) -> Result<Value> {
        let json_value = serde_json::to_value(configs)?;
        self.json_value_to_uide_value(json_value)
    }

    fn global_settings_to_value(&self, settings: &GlobalAiSettings) -> Result<Value> {
        let json_value = serde_json::to_value(settings)?;
        self.json_value_to_uide_value(json_value)
    }

    fn prompt_templates_to_value(&self, templates: &HashMap<String, PromptTemplate>) -> Result<Value> {
        let json_value = serde_json::to_value(templates)?;
        self.json_value_to_uide_value(json_value)
    }

    fn json_value_to_uide_value(&self, json_value: serde_json::Value) -> Result<Value> {
        let value = match json_value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Value::Number(f)
                } else {
                    Value::Number(0.0)
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                let uide_arr = arr.into_iter()
                    .map(|v| self.json_value_to_uide_value(v))
                    .collect::<Result<Vec<_>>>()?;
                Value::Array(uide_arr)
            }
            serde_json::Value::Object(obj) => {
                let mut uide_obj = indexmap::IndexMap::new();
                for (k, v) in obj {
                    uide_obj.insert(k, self.json_value_to_uide_value(v)?);
                }
                Value::Object(uide_obj)
            }
        };
        Ok(value)
    }

    fn record_to_config(&self, fields: &indexmap::IndexMap<String, Value>) -> Result<Option<AiConfig>> {
        // For simplicity, return a default config
        // In a real implementation, you'd deserialize all fields properly
        let name = fields.get("name")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_else(|| "Loaded Config".to_string());

        let description = fields.get("description")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_default();

        let mut config = Self::create_default_config();
        config.name = name;
        config.description = description;
        
        Ok(Some(config))
    }
} 