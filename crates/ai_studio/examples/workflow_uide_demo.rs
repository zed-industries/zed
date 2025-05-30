//! # AI Studio UIDE Integration Demo
//! 
//! This example demonstrates:
//! - Creating and saving workflows to UIDE
//! - Creating and storing AI configurations
//! - Searching and loading stored workflows and configurations
//! - Integration between workflow management and AI orchestration

use ai_studio::{
    ai_config::{AiConfig, AiConfigManager, AiRole, DecompositionStrategy},
    workflow::{
        persistence::{SerializableWorkflow, WorkflowManager, WorkflowMetadata},
        types::*,
        execution::WorkflowExecutor,
    },
};
use anyhow::Result;
use chrono::Utc;
use gpui::Point;
use std::collections::HashMap;
use tempfile::TempDir;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 AI Studio UIDE Integration Demo");
    println!("{}", "=".repeat(50));

    // Create temporary directory for the demo
    let temp_dir = TempDir::new()?;
    let uide_path = temp_dir.path().join("ai_studio_data");
    let uide_path_str = uide_path.to_string_lossy().to_string();

    println!("📁 Using UIDE database at: {}", uide_path_str);

    // Initialize managers
    let workflow_manager = WorkflowManager::new(&uide_path_str).await?;
    let ai_config_manager = AiConfigManager::new(&uide_path_str).await?;

    println!("\n🎯 Demo 1: Creating and Saving AI Configuration");
    println!("{}", "-".repeat(40));

    // Create and save AI configuration
    let mut ai_config = AiConfigManager::create_default_config();
    ai_config.name = "COMPASS Integration Config".to_string();
    ai_config.description = "Configuration for COMPASS-Zed speech integration".to_string();
    
    // Customize for our specific use case
    ai_config.orchestrator_config.decomposition_strategy = DecompositionStrategy::ComplexityAdaptive;
    ai_config.tags.push("compass".to_string());
    ai_config.tags.push("speech".to_string());

    let config_id = ai_config_manager.save_config(&ai_config).await?;
    println!("✅ Saved AI config with ID: {}", config_id);

    println!("\n🔧 Demo 2: Creating Complex Workflow");
    println!("{}", "-".repeat(40));

    // Create a sample workflow for COMPASS-speech integration
    let mut workflow_executor = WorkflowExecutor::new();
    
    // Add nodes for the workflow
    let input_node = workflow_executor.add_node(
        NodeType::Input,
        Point::new(100.0, 100.0),
    );
    
    let analysis_node = workflow_executor.add_node(
        NodeType::LLMPrompt,
        Point::new(300.0, 100.0),
    );
    
    let implementation_node = workflow_executor.add_node(
        NodeType::LLMPrompt,
        Point::new(500.0, 100.0),
    );
    
    let output_node = workflow_executor.add_node(
        NodeType::Output,
        Point::new(700.0, 100.0),
    );

    // Connect the nodes
    workflow_executor.connect_nodes(
        input_node,
        "output".to_string(),
        analysis_node,
        "input".to_string(),
    );
    
    workflow_executor.connect_nodes(
        analysis_node,
        "output".to_string(),
        implementation_node,
        "input".to_string(),
    );
    
    workflow_executor.connect_nodes(
        implementation_node,
        "output".to_string(),
        output_node,
        "input".to_string(),
    );

    // Customize the LLM prompt nodes
    if let Some(analysis_node_ref) = workflow_executor.get_node_mut(analysis_node) {
        analysis_node_ref.title = "Architecture Analysis".to_string();
        if let NodeConfig::LLMPrompt { prompt_template, .. } = &mut analysis_node_ref.config {
            *prompt_template = "Analyze the integration requirements between COMPASS and Zed speech system: {{input}}".to_string();
        }
    }

    if let Some(impl_node_ref) = workflow_executor.get_node_mut(implementation_node) {
        impl_node_ref.title = "Implementation Planning".to_string();
        if let NodeConfig::LLMPrompt { prompt_template, .. } = &mut impl_node_ref.config {
            *prompt_template = "Create implementation plan based on analysis: {{input}}".to_string();
        }
    }

    // Convert to serializable format
    let mut serializable_workflow = SerializableWorkflow::from(&workflow_executor);
    serializable_workflow.name = "COMPASS-Speech Integration Workflow".to_string();
    serializable_workflow.description = "Automated workflow for integrating COMPASS with Zed speech system".to_string();
    serializable_workflow.tags = vec![
        "ai_studio_workflow".to_string(),
        "compass".to_string(),
        "speech".to_string(),
        "integration".to_string(),
    ];
    
    // Add metadata linking to our AI config
    serializable_workflow.metadata = WorkflowMetadata {
        author: Some("AI Studio Demo".to_string()),
        category: "integration".to_string(),
        complexity: "complex".to_string(),
        estimated_runtime: Some(1800), // 30 minutes
        dependencies: vec!["compass_crate".to_string(), "speech_crate".to_string()],
        ai_config_id: Some(config_id.to_string()),
    };

    // Save the workflow
    let workflow_id = workflow_manager.save_workflow(&serializable_workflow).await?;
    println!("✅ Saved workflow with ID: {}", workflow_id);
    println!("   - Name: {}", serializable_workflow.name);
    println!("   - Nodes: {}", serializable_workflow.nodes.len());
    println!("   - Connections: {}", serializable_workflow.connections.len());

    println!("\n🔍 Demo 3: Searching and Loading");
    println!("{}", "-".repeat(40));

    // Search for workflows
    let workflow_results = workflow_manager.search_workflows("compass").await?;
    println!("📊 Found {} workflows matching 'compass':", workflow_results.len());
    for (id, workflow) in &workflow_results {
        println!("   - {}: {} ({})", id, workflow.name, workflow.metadata.complexity);
    }

    // Search for AI configurations
    let config_results = ai_config_manager.search_configs("compass").await?;
    println!("📊 Found {} AI configs matching 'compass':", config_results.len());
    for (id, config) in &config_results {
        println!("   - {}: {} (v{})", id, config.name, config.version);
    }

    // Load the workflow we just saved
    if let Some(loaded_workflow) = workflow_manager.load_workflow(workflow_id).await? {
        println!("✅ Successfully loaded workflow: {}", loaded_workflow.name);
        println!("   - Created: {}", loaded_workflow.created_at.format("%Y-%m-%d %H:%M"));
        println!("   - Category: {}", loaded_workflow.metadata.category);
    }

    // Load the AI config we just saved
    if let Some(loaded_config) = ai_config_manager.load_config(config_id).await? {
        println!("✅ Successfully loaded AI config: {}", loaded_config.name);
        println!("   - Strategy: {:?}", loaded_config.orchestrator_config.decomposition_strategy);
        println!("   - Roles configured: {}", loaded_config.role_configs.len());
    }

    println!("\n📋 Demo 4: Creating Orchestration Workflow");
    println!("{}", "-".repeat(40));

    // Create a more complex workflow that represents AI orchestration
    let orchestration_workflow = create_orchestration_workflow().await?;
    let orchestration_id = workflow_manager.save_workflow(&orchestration_workflow).await?;
    println!("✅ Saved AI orchestration workflow: {}", orchestration_id);
    println!("   - Phases: {}", orchestration_workflow.nodes.len());
    println!("   - Description: {}", orchestration_workflow.description);

    println!("\n📊 Demo 5: Listing All Stored Items");
    println!("{}", "-".repeat(40));

    // List all workflows
    let all_workflows = workflow_manager.list_workflows().await?;
    println!("📝 All stored workflows ({}):", all_workflows.len());
    for (id, workflow) in &all_workflows {
        println!("   - {}: {} [{}]", 
                 id.to_string().chars().take(8).collect::<String>(),
                 workflow.name,
                 workflow.metadata.category);
    }

    // List all AI configs
    let all_configs = ai_config_manager.list_configs().await?;
    println!("⚙️  All stored AI configs ({}):", all_configs.len());
    for (id, config) in &all_configs {
        println!("   - {}: {} [v{}]", 
                 id.to_string().chars().take(8).collect::<String>(),
                 config.name,
                 config.version);
    }

    println!("\n🎯 Demo 6: Workflow-Config Integration");
    println!("{}", "-".repeat(40));

    // Demonstrate how workflows and configs work together
    for (workflow_id, workflow) in &all_workflows {
        if let Some(ai_config_id_str) = &workflow.metadata.ai_config_id {
            println!("🔗 Workflow '{}' is linked to AI config: {}", 
                     workflow.name, 
                     ai_config_id_str.chars().take(8).collect::<String>());
            
            // In a real implementation, you would:
            // 1. Load the AI config
            // 2. Use it to configure the AI orchestration system
            // 3. Execute the workflow with the specified AI roles and prompts
            println!("   → This workflow would use the linked AI configuration for execution");
        }
    }

    println!("\n✨ Demo Complete!");
    println!("All workflows and AI configurations are now stored in UIDE and can be:");
    println!("  • Searched by text queries");
    println!("  • Loaded and executed");
    println!("  • Modified and updated");
    println!("  • Shared across sessions");
    println!("  • Used for learning and optimization");

    Ok(())
}

async fn create_orchestration_workflow() -> Result<SerializableWorkflow> {
    // Create a workflow that represents the Pure AI Task Orchestration process
    let mut workflow_executor = WorkflowExecutor::new();

    // Master Orchestrator node
    let orchestrator_node = workflow_executor.add_node(
        NodeType::LLMPrompt,
        Point::new(100.0, 200.0),
    );

    // Architect AI node
    let architect_node = workflow_executor.add_node(
        NodeType::LLMPrompt,
        Point::new(300.0, 100.0),
    );

    // Developer AI node  
    let developer_node = workflow_executor.add_node(
        NodeType::LLMPrompt,
        Point::new(300.0, 200.0),
    );

    // Reviewer AI node
    let reviewer_node = workflow_executor.add_node(
        NodeType::LLMPrompt,
        Point::new(300.0, 300.0),
    );

    // Integration AI node
    let integrator_node = workflow_executor.add_node(
        NodeType::LLMPrompt,
        Point::new(500.0, 200.0),
    );

    // Final output node
    let output_node = workflow_executor.add_node(
        NodeType::Output,
        Point::new(700.0, 200.0),
    );

    // Connect orchestrator to specialized AIs
    workflow_executor.connect_nodes(
        orchestrator_node, "output".to_string(),
        architect_node, "input".to_string(),
    );
    workflow_executor.connect_nodes(
        orchestrator_node, "output".to_string(),
        developer_node, "input".to_string(),
    );
    workflow_executor.connect_nodes(
        orchestrator_node, "output".to_string(),
        reviewer_node, "input".to_string(),
    );

    // Connect specialized AIs to integrator
    workflow_executor.connect_nodes(
        architect_node, "output".to_string(),
        integrator_node, "input".to_string(),
    );
    workflow_executor.connect_nodes(
        developer_node, "output".to_string(),
        integrator_node, "input".to_string(),
    );
    workflow_executor.connect_nodes(
        reviewer_node, "output".to_string(),
        integrator_node, "input".to_string(),
    );

    // Connect integrator to output
    workflow_executor.connect_nodes(
        integrator_node, "output".to_string(),
        output_node, "input".to_string(),
    );

    // Customize node configurations
    if let Some(node) = workflow_executor.get_node_mut(orchestrator_node) {
        node.title = "Master Orchestrator AI".to_string();
        if let NodeConfig::LLMPrompt { prompt_template, .. } = &mut node.config {
            *prompt_template = "Decompose this complex task: {{input}}".to_string();
        }
    }

    if let Some(node) = workflow_executor.get_node_mut(architect_node) {
        node.title = "Architect AI".to_string();
        if let NodeConfig::LLMPrompt { prompt_template, .. } = &mut node.config {
            *prompt_template = "Design system architecture for: {{input}}".to_string();
        }
    }

    if let Some(node) = workflow_executor.get_node_mut(developer_node) {
        node.title = "Developer AI".to_string();
        if let NodeConfig::LLMPrompt { prompt_template, .. } = &mut node.config {
            *prompt_template = "Implement code solution for: {{input}}".to_string();
        }
    }

    if let Some(node) = workflow_executor.get_node_mut(reviewer_node) {
        node.title = "Reviewer AI".to_string();
        if let NodeConfig::LLMPrompt { prompt_template, .. } = &mut node.config {
            *prompt_template = "Review and validate: {{input}}".to_string();
        }
    }

    if let Some(node) = workflow_executor.get_node_mut(integrator_node) {
        node.title = "Integration AI".to_string();
        if let NodeConfig::LLMPrompt { prompt_template, .. } = &mut node.config {
            *prompt_template = "Integrate all components: {{input}}".to_string();
        }
    }

    // Convert to serializable format
    let mut serializable_workflow = SerializableWorkflow::from(&workflow_executor);
    serializable_workflow.name = "Pure AI Task Orchestration".to_string();
    serializable_workflow.description = "Multi-AI workflow for complex task execution with specialized roles".to_string();
    serializable_workflow.tags = vec![
        "ai_studio_workflow".to_string(),
        "orchestration".to_string(),
        "multi_ai".to_string(),
        "complex".to_string(),
    ];
    
    serializable_workflow.metadata = WorkflowMetadata {
        author: Some("AI Studio Orchestration System".to_string()),
        category: "orchestration".to_string(),
        complexity: "complex".to_string(),
        estimated_runtime: Some(3600), // 1 hour
        dependencies: vec!["ai_models".to_string(), "prompt_templates".to_string()],
        ai_config_id: None, // Would be linked to appropriate config
    };

    Ok(serializable_workflow)
} 