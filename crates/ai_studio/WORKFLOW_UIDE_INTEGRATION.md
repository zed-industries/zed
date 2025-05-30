# Workflow and AI Configuration UIDE Integration

This document describes the integration between AI Studio workflows, AI configurations, and the UIDE (Unified Intelligent Data Engine) for persistent storage and intelligent retrieval.

## 🎯 Overview

The AI Studio now supports:
- **Workflow Persistence**: Save and load complex AI workflows using UIDE
- **AI Configuration Management**: Store AI orchestration settings and prompt templates
- **Intelligent Search**: Find workflows and configurations using natural language queries
- **Cross-Session Continuity**: Maintain state and learning across sessions

## 📋 Components

### 1. Workflow Management (`workflow/persistence.rs`)

#### `WorkflowManager`
Manages workflow persistence using UIDE as the storage backend.

```rust
use ai_studio::workflow::{WorkflowManager, SerializableWorkflow};

// Initialize with UIDE
let manager = WorkflowManager::new("./ai_data").await?;

// Save a workflow
let workflow_id = manager.save_workflow(&workflow).await?;

// Search workflows
let results = manager.search_workflows("integration").await?;

// Load a specific workflow
let workflow = manager.load_workflow(workflow_id).await?;
```

#### `SerializableWorkflow`
UIDE-compatible representation of workflows with metadata:

```rust
pub struct SerializableWorkflow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub nodes: Vec<SerializableNode>,
    pub connections: Vec<SerializableConnection>,
    pub metadata: WorkflowMetadata,
}
```

### 2. AI Configuration Management (`ai_config.rs`)

#### `AiConfigManager`
Manages AI orchestration configurations including model settings, role definitions, and prompt templates.

```rust
use ai_studio::ai_config::{AiConfigManager, AiConfig};

// Initialize
let config_manager = AiConfigManager::new("./ai_data").await?;

// Create default configuration
let config = AiConfigManager::create_default_config();

// Save configuration
let config_id = config_manager.save_config(&config).await?;

// Search configurations
let configs = config_manager.search_configs("compass").await?;
```

#### `AiConfig`
Comprehensive AI orchestration configuration:

```rust
pub struct AiConfig {
    pub orchestrator_config: OrchestratorConfig,
    pub role_configs: HashMap<String, RoleConfig>,
    pub global_settings: GlobalAiSettings,
    pub prompt_templates: HashMap<String, PromptTemplate>,
}
```

## 🔧 Configuration Options

### Orchestrator Configuration
```rust
pub struct OrchestratorConfig {
    pub model_provider: ModelProvider,
    pub decomposition_strategy: DecompositionStrategy,
    pub context_management: ContextManagementConfig,
    pub quality_checks: QualityCheckConfig,
    pub retry_policy: RetryPolicy,
}
```

### Role Configuration
```rust
pub struct RoleConfig {
    pub role_type: AiRole, // Architect, Developer, Reviewer, etc.
    pub model_provider: ModelProvider,
    pub specialization_prompts: Vec<String>,
    pub capability_limits: CapabilityLimits,
    pub handoff_protocols: HandoffProtocols,
    pub quality_metrics: QualityMetrics,
}
```

### Supported AI Roles
- **Architect**: System design and technical planning
- **Developer**: Code implementation and testing  
- **Reviewer**: Code review and quality assurance
- **Integrator**: Component integration and system coherence
- **Tester**: Comprehensive testing strategies
- **Documenter**: Documentation and user guides
- **ContextManager**: Context optimization and compression
- **PromptEngineer**: Dynamic prompt generation and optimization

## 🚀 Usage Examples

### Creating and Saving Workflows

```rust
use ai_studio::workflow::{WorkflowManager, WorkflowExecutor, NodeType};
use gpui::Point;

// Create workflow
let mut executor = WorkflowExecutor::new();
let input_node = executor.add_node(NodeType::Input, Point::new(100.0, 100.0));
let llm_node = executor.add_node(NodeType::LLMPrompt, Point::new(300.0, 100.0));
let output_node = executor.add_node(NodeType::Output, Point::new(500.0, 100.0));

// Connect nodes
executor.connect_nodes(input_node, "output".to_string(), llm_node, "input".to_string());
executor.connect_nodes(llm_node, "output".to_string(), output_node, "input".to_string());

// Convert to serializable format
let mut workflow = SerializableWorkflow::from(&executor);
workflow.name = "My Workflow".to_string();
workflow.description = "Example workflow".to_string();
workflow.tags = vec!["example".to_string(), "demo".to_string()];

// Save to UIDE
let manager = WorkflowManager::new("./data").await?;
let workflow_id = manager.save_workflow(&workflow).await?;
```

### AI Configuration Setup

```rust
use ai_studio::ai_config::{AiConfigManager, DecompositionStrategy};

// Create custom configuration
let mut config = AiConfigManager::create_default_config();
config.name = "Custom AI Setup".to_string();
config.orchestrator_config.decomposition_strategy = DecompositionStrategy::ComplexityAdaptive;

// Add custom role configuration
let custom_role = RoleConfig {
    role_type: AiRole::Custom("SpecializedAnalyst".to_string()),
    model_provider: ModelProvider {
        provider_type: "openai".to_string(),
        model_name: "gpt-4".to_string(),
        temperature: 0.1,
        max_tokens: 2000,
        // ... other settings
    },
    // ... other role settings
};

config.role_configs.insert("analyst".to_string(), custom_role);

// Save configuration
let config_manager = AiConfigManager::new("./data").await?;
let config_id = config_manager.save_config(&config).await?;
```

### Searching and Loading

```rust
// Search workflows by content
let workflow_results = manager.search_workflows("integration COMPASS").await?;
for (id, workflow) in workflow_results {
    println!("Found: {} - {}", workflow.name, workflow.description);
}

// Search AI configurations
let config_results = config_manager.search_configs("default").await?;
for (id, config) in config_results {
    println!("Config: {} (v{})", config.name, config.version);
}

// Load specific items
let workflow = manager.load_workflow(workflow_id).await?;
let config = config_manager.load_config(config_id).await?;
```

### Linking Workflows and Configurations

```rust
// Link workflow to AI configuration
workflow.metadata.ai_config_id = Some(config_id.to_string());
manager.update_workflow(workflow_id, &workflow).await?;

// In execution, load linked config
if let Some(config_id_str) = &workflow.metadata.ai_config_id {
    let config = config_manager.load_config(config_id_str.parse()?).await?;
    // Use config to set up AI orchestration
}
```

## 📊 UIDE Integration Details

### Data Storage Structure

Workflows and AI configurations are stored in UIDE as structured records:

- **Type**: `DataType::Structured`
- **Tags**: Used for categorization and filtering
- **Metadata**: Source tracking and confidence scoring
- **Content**: Structured fields with nested data

### Search Capabilities

UIDE enables intelligent search across:
- Workflow names and descriptions
- Node configurations and prompt templates
- AI configuration settings and role definitions
- Tags and metadata fields

### Performance Optimization

- **Lazy Loading**: Only load full data when needed
- **Caching**: Frequently accessed workflows/configs stay in memory
- **Compression**: Large prompt templates are compressed automatically
- **Indexing**: UIDE automatically indexes searchable content

## 🔄 Workflow Execution Integration

### Phase 1: Load Configuration
```rust
let config = config_manager.load_config(config_id).await?;
let orchestrator_settings = &config.orchestrator_config;
```

### Phase 2: Initialize AI Roles
```rust
for (role_name, role_config) in &config.role_configs {
    let ai_worker = create_ai_worker(role_config);
    orchestrator.register_role(role_name, ai_worker);
}
```

### Phase 3: Execute Workflow
```rust
let workflow = manager.load_workflow(workflow_id).await?;
let execution_result = orchestrator.execute_workflow(workflow, config).await?;
```

### Phase 4: Learning and Optimization
```rust
// Update configuration based on execution results
if config.global_settings.learning_enabled {
    config.update_from_execution(&execution_result);
    config_manager.save_config(&config).await?;
}
```

## 🧪 Running the Demo

```bash
# Run the comprehensive demo
cargo run --example workflow_uide_demo

# Example output:
# 🚀 AI Studio UIDE Integration Demo
# ✅ Saved AI config with ID: abc123...
# ✅ Saved workflow with ID: def456...
# 📊 Found 2 workflows matching 'compass'
# 🔗 Workflow 'COMPASS-Speech Integration' is linked to AI config: abc123...
```

## 🔮 Future Enhancements

### Planned Features
- **Version Control**: Track workflow and configuration changes
- **Collaboration**: Share workflows across team members
- **Templates**: Pre-built workflow templates for common tasks
- **Analytics**: Execution metrics and performance tracking
- **Export/Import**: Backup and migrate workflows/configurations

### Integration Opportunities
- **COMPASS Bridge**: Direct integration with COMPASS agent system
- **Speech Commands**: Voice-activated workflow creation and execution
- **Visual Editor**: Drag-and-drop workflow designer with UIDE persistence
- **Real-time Collaboration**: Multi-user workflow editing

## 📚 API Reference

### WorkflowManager Methods
- `new(uide_path)` - Initialize with UIDE database
- `save_workflow(&workflow)` - Save workflow to UIDE
- `load_workflow(id)` - Load workflow by ID
- `search_workflows(query)` - Search workflows by text
- `list_workflows()` - List all workflows
- `delete_workflow(id)` - Delete workflow
- `update_workflow(id, &workflow)` - Update existing workflow

### AiConfigManager Methods
- `new(uide_path)` - Initialize with UIDE database
- `save_config(&config)` - Save AI configuration
- `load_config(id)` - Load configuration by ID
- `search_configs(query)` - Search configurations by text
- `list_configs()` - List all configurations
- `delete_config(id)` - Delete configuration
- `create_default_config()` - Create default configuration

---

**This integration brings the power of UIDE's intelligent data management to AI Studio's workflow and orchestration capabilities, enabling persistent, searchable, and optimizable AI workflows.** 