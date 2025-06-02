pub mod agent_config;
pub mod agent_manager;
pub mod agent_view;
pub mod context_types;
pub mod tool_registry;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

pub use agent_manager::AgentManager;

/// AI Agent configuration that uses models with specific contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub agent_type: AgentType,
    pub model_id: Uuid, // References a ModelConfig
    pub context: AgentContext,
    pub capabilities: Vec<AgentCapability>,
    pub tools: Vec<String>, // Tool IDs from ToolRegistry
    pub spawning_rules: SpawningRules,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    GeneralAssistant,
    CodeSpecialist,
    UIDesigner,
    ProjectManager,
    Debugger,
    DocumentationWriter,
    TestGenerator,
    RefactoringAgent,
    Custom { category: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub system_prompt: String,
    pub initial_instructions: Vec<String>,
    pub context_variables: HashMap<String, ContextValue>,
    pub memory_config: MemoryConfig,
    pub behavior_parameters: BehaviorParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ContextValue>),
    Object(HashMap<String, ContextValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub conversation_history_limit: usize,
    pub context_window_management: ContextWindowStrategy,
    pub persistent_memory: bool,
    pub memory_retrieval_strategy: MemoryRetrievalStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextWindowStrategy {
    Truncate,
    Summarize,
    RollingWindow,
    Hierarchical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryRetrievalStrategy {
    Recent,
    Semantic,
    Important,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorParameters {
    pub creativity_level: f32, // 0.0 to 1.0
    pub verbosity: VerbosityLevel,
    pub interaction_style: InteractionStyle,
    pub error_handling: ErrorHandlingStrategy,
    pub decision_making: DecisionMakingStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerbosityLevel {
    Minimal,
    Normal,
    Detailed,
    Verbose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionStyle {
    Direct,
    Conversational,
    Professional,
    Casual,
    Educational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorHandlingStrategy {
    AskForHelp,
    TryAlternatives,
    ProvideWorkarounds,
    EscalateToUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionMakingStyle {
    Conservative,
    Balanced,
    Aggressive,
    UserGuided,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentCapability {
    TextProcessing,
    CodeGeneration,
    UICreation,
    FileManagement,
    GitOperations,
    ProjectAnalysis,
    AgentSpawning,
    ToolUsage,
    MemoryManagement,
    ContextUnderstanding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawningRules {
    pub can_spawn_agents: bool,
    pub max_child_agents: usize,
    pub allowed_agent_types: Vec<AgentType>,
    pub spawn_triggers: Vec<SpawnTrigger>,
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnTrigger {
    TaskComplexity { threshold: f32 },
    SpecializedSkillNeeded { skill: String },
    WorkloadThreshold { max_concurrent_tasks: usize },
    UserRequest,
    ContextSwitch { domain: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: usize,
    pub max_cpu_percent: f32,
    pub max_tokens_per_hour: usize,
    pub max_tool_calls_per_minute: usize,
}

impl Default for AgentContext {
    fn default() -> Self {
        Self {
            system_prompt: "You are a helpful AI assistant.".to_string(),
            initial_instructions: Vec::new(),
            context_variables: HashMap::new(),
            memory_config: MemoryConfig::default(),
            behavior_parameters: BehaviorParameters::default(),
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            conversation_history_limit: 50,
            context_window_management: ContextWindowStrategy::RollingWindow,
            persistent_memory: true,
            memory_retrieval_strategy: MemoryRetrievalStrategy::Hybrid,
        }
    }
}

impl Default for BehaviorParameters {
    fn default() -> Self {
        Self {
            creativity_level: 0.7,
            verbosity: VerbosityLevel::Normal,
            interaction_style: InteractionStyle::Conversational,
            error_handling: ErrorHandlingStrategy::TryAlternatives,
            decision_making: DecisionMakingStyle::Balanced,
        }
    }
}

impl Default for SpawningRules {
    fn default() -> Self {
        Self {
            can_spawn_agents: false,
            max_child_agents: 3,
            allowed_agent_types: Vec::new(),
            spawn_triggers: Vec::new(),
            resource_limits: ResourceLimits::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_cpu_percent: 50.0,
            max_tokens_per_hour: 10000,
            max_tool_calls_per_minute: 60,
        }
    }
}

impl AgentConfig {
    pub fn new(name: String, agent_type: AgentType, model_id: Uuid) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            agent_type,
            model_id,
            context: AgentContext::default(),
            capabilities: Vec::new(),
            tools: Vec::new(),
            spawning_rules: SpawningRules::default(),
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_context(mut self, context: AgentContext) -> Self {
        self.context = context;
        self
    }

    pub fn with_capabilities(mut self, capabilities: Vec<AgentCapability>) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn with_tools(mut self, tools: Vec<String>) -> Self {
        self.tools = tools;
        self
    }

    pub fn with_spawning_rules(mut self, spawning_rules: SpawningRules) -> Self {
        self.spawning_rules = spawning_rules;
        self
    }

    pub fn can_spawn_agents(&self) -> bool {
        self.spawning_rules.can_spawn_agents && 
        self.capabilities.contains(&AgentCapability::AgentSpawning)
    }

    pub fn supports_capability(&self, capability: &AgentCapability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
}

/// Runtime instance of an agent
#[derive(Debug, Clone)]
pub struct AgentInstance {
    pub config: AgentConfig,
    pub session_id: Uuid,
    pub parent_agent: Option<Uuid>,
    pub child_agents: Vec<Uuid>,
    pub conversation_history: Vec<ConversationMessage>,
    pub current_task: Option<AgentTask>,
    pub status: AgentStatus,
    pub resource_usage: ResourceUsage,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct ConversationMessage {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, ContextValue>,
}

#[derive(Debug, Clone)]
pub enum MessageRole {
    User,
    Agent,
    System,
    Tool,
}

#[derive(Debug, Clone)]
pub struct AgentTask {
    pub id: Uuid,
    pub description: String,
    pub status: TaskStatus,
    pub progress: f32, // 0.0 to 1.0
    pub subtasks: Vec<AgentTask>,
    pub assigned_agents: Vec<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum AgentStatus {
    Idle,
    Processing,
    WaitingForInput,
    SpawningAgent,
    Error { message: String },
    Shutdown,
}

#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub memory_mb: usize,
    pub cpu_percent: f32,
    pub tokens_used: usize,
    pub tool_calls_made: usize,
    pub uptime_seconds: u64,
}

impl AgentInstance {
    pub fn new(config: AgentConfig, parent_agent: Option<Uuid>) -> Self {
        Self {
            config,
            session_id: Uuid::new_v4(),
            parent_agent,
            child_agents: Vec::new(),
            conversation_history: Vec::new(),
            current_task: None,
            status: AgentStatus::Idle,
            resource_usage: ResourceUsage::new(),
            created_at: chrono::Utc::now(),
        }
    }

    pub fn add_child_agent(&mut self, child_id: Uuid) {
        if self.child_agents.len() < self.config.spawning_rules.max_child_agents {
            self.child_agents.push(child_id);
        }
    }

    pub fn can_spawn_more_agents(&self) -> bool {
        self.config.can_spawn_agents() && 
        self.child_agents.len() < self.config.spawning_rules.max_child_agents
    }
}

impl ResourceUsage {
    pub fn new() -> Self {
        Self {
            memory_mb: 0,
            cpu_percent: 0.0,
            tokens_used: 0,
            tool_calls_made: 0,
            uptime_seconds: 0,
        }
    }
} 