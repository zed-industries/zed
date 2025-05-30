use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uide::{
    universal::{
        DataType as UideDataType, RecordMetadata, StructuredBuilder, UniversalContent, UniversalRecord, Value,
    },
    RecordId, UnifiedDataEngine,
};
use uuid::Uuid;
use std::str::FromStr;
use gpui::Point;

use super::types::*;
use super::execution::WorkflowExecutor;

/// Serializable workflow representation for UIDE storage
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableNode {
    pub id: String, // UUID as string for serialization
    pub node_type: String,
    pub position: SerializablePoint,
    pub size: SerializableSize,
    pub title: String,
    pub inputs: Vec<SerializablePort>,
    pub outputs: Vec<SerializablePort>,
    pub config: SerializableNodeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePort {
    pub id: String,
    pub name: String,
    pub port_type: String,
    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableConnection {
    pub id: String,
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableNodeConfig {
    pub config_type: String,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    pub author: Option<String>,
    pub category: String,
    pub complexity: String, // "simple", "medium", "complex"
    pub estimated_runtime: Option<u32>, // in seconds
    pub dependencies: Vec<String>,
    pub ai_config_id: Option<String>,
}

/// Workflow manager with UIDE integration
pub struct WorkflowManager {
    uide_engine: UnifiedDataEngine,
}

impl WorkflowManager {
    pub async fn new(uide_path: impl Into<String>) -> Result<Self> {
        let uide_path = uide_path.into();
        let uide_engine = UnifiedDataEngine::new(uide_path).await
            .map_err(|e| anyhow::anyhow!("Failed to create UIDE engine: {}", e))?;
        Ok(Self { uide_engine })
    }

    pub async fn with_engine(uide_engine: UnifiedDataEngine) -> Self {
        Self { uide_engine }
    }

    /// Save a workflow to UIDE
    pub async fn save_workflow(
        &self,
        workflow: &SerializableWorkflow,
    ) -> Result<RecordId> {
        let content = StructuredBuilder::new()
            .text_field("name", &workflow.name)
            .text_field("description", &workflow.description)
            .text_field("version", &workflow.version)
            .text_field("category", &workflow.metadata.category)
            .text_field("complexity", &workflow.metadata.complexity)
            .field("created_at", Value::String(workflow.created_at.to_rfc3339()))
            .field("updated_at", Value::String(workflow.updated_at.to_rfc3339()))
            .field("nodes", self.nodes_to_value(&workflow.nodes)?)
            .field("connections", self.connections_to_value(&workflow.connections)?)
            .field("metadata", self.metadata_to_value(&workflow.metadata)?)
            .build();

        let metadata = RecordMetadata::new()
            .with_tags(workflow.tags.clone())
            .with_source("ai_studio_workflow".to_string())
            .with_confidence(1.0);

        let record = UniversalRecord::new(UideDataType::Structured, content)
            .with_metadata(metadata);

        self.uide_engine.store_record(record).await
            .map_err(|e| anyhow::anyhow!("Failed to store workflow: {}", e))
    }

    /// Load a workflow from UIDE
    pub async fn load_workflow(&self, record_id: RecordId) -> Result<Option<SerializableWorkflow>> {
        let record = self.uide_engine.get_record(record_id).await?;
        
        match record {
            Some(record) => {
                if let UniversalContent::Structured { fields, .. } = &record.content {
                    self.record_to_workflow(fields)
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Search workflows by name or description
    pub async fn search_workflows(&self, query: &str) -> Result<Vec<(RecordId, SerializableWorkflow)>> {
        let search_query = uide::query::UniversalQuery::text_search(query);
        let results = self.uide_engine.search(search_query).await?;

        let mut workflows = Vec::new();
        for result in results.results {
            if let UniversalContent::Structured { fields, .. } = &result.record.content {
                if let Some(workflow) = self.record_to_workflow(fields)? {
                    workflows.push((result.record.id, workflow));
                }
            }
        }

        Ok(workflows)
    }

    /// List all workflows
    pub async fn list_workflows(&self) -> Result<Vec<(RecordId, SerializableWorkflow)>> {
        let query = uide::query::UniversalQuery::by_type(UideDataType::Structured);
        let results = self.uide_engine.search(query).await?;

        let mut workflows = Vec::new();
        for result in results.results {
            // Check if this is a workflow record
            if result.record.metadata.tags.contains(&"ai_studio_workflow".to_string()) {
                if let UniversalContent::Structured { fields, .. } = &result.record.content {
                    if let Some(workflow) = self.record_to_workflow(fields)? {
                        workflows.push((result.record.id, workflow));
                    }
                }
            }
        }

        Ok(workflows)
    }

    /// Delete a workflow
    pub async fn delete_workflow(&self, record_id: RecordId) -> Result<bool> {
        self.uide_engine.delete(record_id).await
            .map_err(|e| anyhow::anyhow!("Failed to delete workflow: {}", e))
    }

    /// Update an existing workflow
    pub async fn update_workflow(
        &self,
        record_id: RecordId,
        workflow: &SerializableWorkflow,
    ) -> Result<()> {
        // Delete the old record and create a new one
        // In a more sophisticated implementation, you might want to update in place
        self.delete_workflow(record_id).await?;
        self.save_workflow(workflow).await?;
        Ok(())
    }

    // Helper methods for conversion
    fn nodes_to_value(&self, nodes: &[SerializableNode]) -> Result<Value> {
        let nodes_value = nodes.iter()
            .map(|node| {
                let node_obj = serde_json::to_value(node)?;
                Ok(self.json_value_to_uide_value(node_obj)?)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Value::Array(nodes_value))
    }

    fn connections_to_value(&self, connections: &[SerializableConnection]) -> Result<Value> {
        let connections_value = connections.iter()
            .map(|conn| {
                let conn_obj = serde_json::to_value(conn)?;
                Ok(self.json_value_to_uide_value(conn_obj)?)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Value::Array(connections_value))
    }

    fn metadata_to_value(&self, metadata: &WorkflowMetadata) -> Result<Value> {
        let metadata_obj = serde_json::to_value(metadata)?;
        self.json_value_to_uide_value(metadata_obj)
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

    fn record_to_workflow(&self, fields: &indexmap::IndexMap<String, Value>) -> Result<Option<SerializableWorkflow>> {
        // Extract basic fields
        let name = fields.get("name")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_default();

        let description = fields.get("description")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_default();

        let version = fields.get("version")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_else(|| "1.0.0".to_string());

        // Parse timestamps
        let created_at = fields.get("created_at")
            .and_then(|v| if let Value::String(s) = v { 
                DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&Utc))
            } else { None })
            .unwrap_or_else(Utc::now);

        let updated_at = fields.get("updated_at")
            .and_then(|v| if let Value::String(s) = v { 
                DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&Utc))
            } else { None })
            .unwrap_or_else(Utc::now);

        // Extract and deserialize nodes
        let nodes = if let Some(nodes_value) = fields.get("nodes") {
            self.deserialize_nodes(nodes_value)?
        } else {
            Vec::new()
        };

        // Extract and deserialize connections
        let connections = if let Some(connections_value) = fields.get("connections") {
            self.deserialize_connections(connections_value)?
        } else {
            Vec::new()
        };

        // Extract metadata
        let metadata = if let Some(metadata_value) = fields.get("metadata") {
            self.deserialize_metadata(metadata_value)?
        } else {
            WorkflowMetadata {
                author: None,
                category: "general".to_string(),
                complexity: "medium".to_string(),
                estimated_runtime: None,
                dependencies: Vec::new(),
                ai_config_id: None,
            }
        };

        let workflow = SerializableWorkflow {
            id: Uuid::new_v4(),
            name,
            description,
            version,
            created_at,
            updated_at,
            tags: Vec::new(), // TODO: Extract tags from metadata
            nodes,
            connections,
            metadata,
        };

        Ok(Some(workflow))
    }

    fn deserialize_nodes(&self, nodes_value: &Value) -> Result<Vec<SerializableNode>> {
        let mut nodes = Vec::new();
        
        if let Value::Array(nodes_array) = nodes_value {
            println!("🔍 Deserializing {} nodes from stored data", nodes_array.len());
            for node_value in nodes_array {
                if let Some(node) = self.deserialize_single_node(node_value)? {
                    println!("✅ Deserialized node: {} at ({:.0}, {:.0})", 
                        node.title, node.position.x, node.position.y);
                    nodes.push(node);
                }
            }
        } else {
            println!("⚠️  No nodes array found in stored data");
        }
        
        println!("📦 Total nodes deserialized: {}", nodes.len());
        Ok(nodes)
    }

    fn deserialize_single_node(&self, node_value: &Value) -> Result<Option<SerializableNode>> {
        if let Value::Object(node_obj) = node_value {
            let id = node_obj.get("id")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            let node_type = node_obj.get("node_type")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "Input".to_string());

            let title = node_obj.get("title")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "Node".to_string());

            // Extract position
            let position = if let Some(pos_value) = node_obj.get("position") {
                self.deserialize_position(pos_value)?
            } else {
                SerializablePoint { x: 0.0, y: 0.0 }
            };

            // Extract size
            let size = if let Some(size_value) = node_obj.get("size") {
                self.deserialize_size(size_value)?
            } else {
                SerializableSize { width: 200.0, height: 120.0 }
            };

            // Extract inputs and outputs (simplified for now)
            let inputs = if let Some(inputs_value) = node_obj.get("inputs") {
                self.deserialize_ports(inputs_value)?
            } else {
                Vec::new()
            };

            let outputs = if let Some(outputs_value) = node_obj.get("outputs") {
                self.deserialize_ports(outputs_value)?
            } else {
                Vec::new()
            };

            // Extract config (simplified for now)
            let config = if let Some(config_value) = node_obj.get("config") {
                self.deserialize_node_config(config_value)?
            } else {
                SerializableNodeConfig {
                    config_type: "Input".to_string(),
                    data: Value::Object(indexmap::IndexMap::new()),
                }
            };

            return Ok(Some(SerializableNode {
                id,
                node_type,
                position,
                size,
                title,
                inputs,
                outputs,
                config,
            }));
        }
        
        Ok(None)
    }

    fn deserialize_position(&self, pos_value: &Value) -> Result<SerializablePoint> {
        if let Value::Object(pos_obj) = pos_value {
            let x = pos_obj.get("x")
                .and_then(|v| if let Value::Number(n) = v { Some(*n as f32) } else { None })
                .unwrap_or(0.0);
            let y = pos_obj.get("y")
                .and_then(|v| if let Value::Number(n) = v { Some(*n as f32) } else { None })
                .unwrap_or(0.0);
            Ok(SerializablePoint { x, y })
        } else {
            Ok(SerializablePoint { x: 0.0, y: 0.0 })
        }
    }

    fn deserialize_size(&self, size_value: &Value) -> Result<SerializableSize> {
        if let Value::Object(size_obj) = size_value {
            let width = size_obj.get("width")
                .and_then(|v| if let Value::Number(n) = v { Some(*n as f32) } else { None })
                .unwrap_or(200.0);
            let height = size_obj.get("height")
                .and_then(|v| if let Value::Number(n) = v { Some(*n as f32) } else { None })
                .unwrap_or(120.0);
            Ok(SerializableSize { width, height })
        } else {
            Ok(SerializableSize { width: 200.0, height: 120.0 })
        }
    }

    fn deserialize_ports(&self, ports_value: &Value) -> Result<Vec<SerializablePort>> {
        let mut ports = Vec::new();
        
        if let Value::Array(ports_array) = ports_value {
            for port_value in ports_array {
                if let Value::Object(port_obj) = port_value {
                    let id = port_obj.get("id")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| Uuid::new_v4().to_string());
                    
                    let name = port_obj.get("name")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| "Port".to_string());
                    
                    let port_type = port_obj.get("port_type")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| "Input".to_string());
                    
                    let data_type = port_obj.get("data_type")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| "Text".to_string());
                    
                    ports.push(SerializablePort {
                        id,
                        name,
                        port_type,
                        data_type,
                    });
                }
            }
        }
        
        Ok(ports)
    }

    fn deserialize_node_config(&self, config_value: &Value) -> Result<SerializableNodeConfig> {
        if let Value::Object(config_obj) = config_value {
            let config_type = config_obj.get("config_type")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "Input".to_string());
            
            let data = config_obj.get("data")
                .cloned()
                .unwrap_or_else(|| Value::Object(indexmap::IndexMap::new()));
            
            Ok(SerializableNodeConfig {
                config_type,
                data,
            })
        } else {
            Ok(SerializableNodeConfig {
                config_type: "Input".to_string(),
                data: Value::Object(indexmap::IndexMap::new()),
            })
        }
    }

    fn deserialize_connections(&self, connections_value: &Value) -> Result<Vec<SerializableConnection>> {
        let mut connections = Vec::new();
        
        if let Value::Array(connections_array) = connections_value {
            for conn_value in connections_array {
                if let Value::Object(conn_obj) = conn_value {
                    let id = conn_obj.get("id")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| Uuid::new_v4().to_string());
                    
                    let from_node = conn_obj.get("from_node")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| Uuid::new_v4().to_string());
                    
                    let from_port = conn_obj.get("from_port")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| "output".to_string());
                    
                    let to_node = conn_obj.get("to_node")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| Uuid::new_v4().to_string());
                    
                    let to_port = conn_obj.get("to_port")
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_else(|| "input".to_string());
                    
                    connections.push(SerializableConnection {
                        id,
                        from_node,
                        from_port,
                        to_node,
                        to_port,
                    });
                }
            }
        }
        
        Ok(connections)
    }

    fn deserialize_metadata(&self, metadata_value: &Value) -> Result<WorkflowMetadata> {
        if let Value::Object(metadata_obj) = metadata_value {
            let author = metadata_obj.get("author")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None });
            
            let category = metadata_obj.get("category")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "general".to_string());
            
            let complexity = metadata_obj.get("complexity")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "medium".to_string());
            
            let estimated_runtime = metadata_obj.get("estimated_runtime")
                .and_then(|v| if let Value::Number(n) = v { Some(*n as u32) } else { None });
            
            Ok(WorkflowMetadata {
                author,
                category,
                complexity,
                estimated_runtime,
                dependencies: Vec::new(), // TODO: Deserialize dependencies
                ai_config_id: None, // TODO: Deserialize ai_config_id
            })
        } else {
            Ok(WorkflowMetadata {
                author: None,
                category: "general".to_string(),
                complexity: "medium".to_string(),
                estimated_runtime: None,
                dependencies: Vec::new(),
                ai_config_id: None,
            })
        }
    }
}

/// Convert runtime workflow to serializable format
impl From<&WorkflowExecutor> for SerializableWorkflow {
    fn from(executor: &WorkflowExecutor) -> Self {
        let nodes = executor.nodes.values()
            .map(|node| SerializableNode::from(node))
            .collect();

        let connections = executor.connections.iter()
            .map(|conn| SerializableConnection::from(conn))
            .collect();

        SerializableWorkflow {
            id: Uuid::new_v4(),
            name: "Untitled Workflow".to_string(),
            description: "".to_string(),
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: vec!["ai_studio_workflow".to_string()],
            nodes,
            connections,
            metadata: WorkflowMetadata {
                author: None,
                category: "general".to_string(),
                complexity: "medium".to_string(),
                estimated_runtime: None,
                dependencies: Vec::new(),
                ai_config_id: None,
            },
        }
    }
}

impl From<&WorkflowNode> for SerializableNode {
    fn from(node: &WorkflowNode) -> Self {
        Self {
            id: node.id.as_uuid().to_string(),
            node_type: format!("{:?}", node.node_type),
            position: SerializablePoint { x: node.position.x, y: node.position.y },
            size: SerializableSize { width: node.size.width, height: node.size.height },
            title: node.title.clone(),
            inputs: node.inputs.iter().map(SerializablePort::from).collect(),
            outputs: node.outputs.iter().map(SerializablePort::from).collect(),
            config: SerializableNodeConfig::from(&node.config),
        }
    }
}

impl From<&NodePort> for SerializablePort {
    fn from(port: &NodePort) -> Self {
        Self {
            id: port.id.clone(),
            name: port.name.clone(),
            port_type: format!("{:?}", port.port_type),
            data_type: format!("{:?}", port.data_type),
        }
    }
}

impl From<&NodeConnection> for SerializableConnection {
    fn from(conn: &NodeConnection) -> Self {
        Self {
            id: conn.id.to_string(),
            from_node: conn.from_node.as_uuid().to_string(),
            from_port: conn.from_port.clone(),
            to_node: conn.to_node.as_uuid().to_string(),
            to_port: conn.to_port.clone(),
        }
    }
}

impl From<&NodeConfig> for SerializableNodeConfig {
    fn from(config: &NodeConfig) -> Self {
        match config {
            NodeConfig::Input { placeholder } => Self {
                config_type: "Input".to_string(),
                data: Value::Object({
                    let mut obj = indexmap::IndexMap::new();
                    obj.insert("placeholder".to_string(), Value::String(placeholder.clone()));
                    obj
                }),
            },
            NodeConfig::LLMPrompt { prompt_template, temperature, max_tokens, .. } => Self {
                config_type: "LLMPrompt".to_string(),
                data: Value::Object({
                    let mut obj = indexmap::IndexMap::new();
                    obj.insert("prompt_template".to_string(), Value::String(prompt_template.clone()));
                    obj.insert("temperature".to_string(), Value::Number(*temperature as f64));
                    obj.insert("max_tokens".to_string(), Value::Number(*max_tokens as f64));
                    obj
                }),
            },
            NodeConfig::TextProcessor { operation } => Self {
                config_type: "TextProcessor".to_string(),
                data: Value::Object({
                    let mut obj = indexmap::IndexMap::new();
                    obj.insert("operation".to_string(), Value::String(format!("{:?}", operation)));
                    obj
                }),
            },
            NodeConfig::Conditional { condition } => Self {
                config_type: "Conditional".to_string(),
                data: Value::Object({
                    let mut obj = indexmap::IndexMap::new();
                    obj.insert("condition".to_string(), Value::String(condition.clone()));
                    obj
                }),
            },
            NodeConfig::Output { format } => Self {
                config_type: "Output".to_string(),
                data: Value::Object({
                    let mut obj = indexmap::IndexMap::new();
                    obj.insert("format".to_string(), Value::String(format!("{:?}", format)));
                    obj
                }),
            },
            NodeConfig::DataSource { source_type } => Self {
                config_type: "DataSource".to_string(),
                data: Value::Object({
                    let mut obj = indexmap::IndexMap::new();
                    obj.insert("source_type".to_string(), Value::String(format!("{:?}", source_type)));
                    obj
                }),
            },
            NodeConfig::Transform { transformation } => Self {
                config_type: "Transform".to_string(),
                data: Value::Object({
                    let mut obj = indexmap::IndexMap::new();
                    obj.insert("transformation".to_string(), Value::String(transformation.clone()));
                    obj
                }),
            },
        }
    }
}

/// Convert from serializable types back to runtime types
impl SerializableWorkflow {
    /// Convert to runtime workflow data
    pub fn to_runtime_data(&self) -> Result<(Vec<WorkflowNode>, Vec<NodeConnection>)> {
        let mut nodes = Vec::new();
        let mut connections = Vec::new();
        
        // Convert nodes
        for serializable_node in &self.nodes {
            let node = serializable_node.to_runtime_node()?;
            nodes.push(node);
        }
        
        // Convert connections
        for serializable_conn in &self.connections {
            let connection = serializable_conn.to_runtime_connection()?;
            connections.push(connection);
        }
        
        Ok((nodes, connections))
    }
}

impl SerializableNode {
    fn to_runtime_node(&self) -> Result<WorkflowNode> {
        let node_id = NodeId::from_uuid(Uuid::from_str(&self.id)?);
        let node_type = self.parse_node_type()?;
        let position = Point::new(self.position.x, self.position.y);
        let size = gpui::Size::new(self.size.width, self.size.height);
        
        let inputs: Vec<NodePort> = self.inputs.iter()
            .map(|p| p.to_runtime_port())
            .collect::<Result<Vec<_>>>()?;
            
        let outputs: Vec<NodePort> = self.outputs.iter()
            .map(|p| p.to_runtime_port())
            .collect::<Result<Vec<_>>>()?;
        
        let config = self.config.to_runtime_config()?;
        
        Ok(WorkflowNode {
            id: node_id,
            node_type,
            position,
            size,
            title: self.title.clone(),
            inputs,
            outputs,
            config,
            state: NodeState::Idle,
        })
    }
    
    fn parse_node_type(&self) -> Result<NodeType> {
        match self.node_type.as_str() {
            "Input" => Ok(NodeType::Input),
            "LLMPrompt" => Ok(NodeType::LLMPrompt),
            "TextProcessor" => Ok(NodeType::TextProcessor),
            "Conditional" => Ok(NodeType::Conditional),
            "Output" => Ok(NodeType::Output),
            "DataSource" => Ok(NodeType::DataSource),
            "Transform" => Ok(NodeType::Transform),
            _ => Err(anyhow::anyhow!("Unknown node type: {}", self.node_type)),
        }
    }
}

impl SerializablePort {
    fn to_runtime_port(&self) -> Result<NodePort> {
        let port_type = match self.port_type.as_str() {
            "Input" => PortType::Input,
            "Output" => PortType::Output,
            _ => return Err(anyhow::anyhow!("Unknown port type: {}", self.port_type)),
        };
        
        let data_type = match self.data_type.as_str() {
            "Text" => super::types::DataType::Text,
            "Number" => super::types::DataType::Number,
            "Boolean" => super::types::DataType::Boolean,
            "Object" => super::types::DataType::Object,
            "Array" => super::types::DataType::Array,
            _ => return Err(anyhow::anyhow!("Unknown data type: {}", self.data_type)),
        };
        
        Ok(NodePort {
            id: self.id.clone(),
            name: self.name.clone(),
            port_type,
            data_type,
        })
    }
}

impl SerializableConnection {
    fn to_runtime_connection(&self) -> Result<NodeConnection> {
        Ok(NodeConnection {
            id: Uuid::from_str(&self.id)?,
            from_node: NodeId::from_uuid(Uuid::from_str(&self.from_node)?),
            from_port: self.from_port.clone(),
            to_node: NodeId::from_uuid(Uuid::from_str(&self.to_node)?),
            to_port: self.to_port.clone(),
        })
    }
}

impl SerializableNodeConfig {
    fn to_runtime_config(&self) -> Result<NodeConfig> {
        match self.config_type.as_str() {
            "Input" => {
                let placeholder = self.extract_string_field("placeholder")?
                    .unwrap_or_else(|| "Enter input...".to_string());
                Ok(NodeConfig::Input { placeholder })
            },
            "LLMPrompt" => {
                let prompt_template = self.extract_string_field("prompt_template")?
                    .unwrap_or_else(|| "{{input}}".to_string());
                let temperature = self.extract_number_field("temperature")?
                    .unwrap_or(0.7) as f32;
                let max_tokens = self.extract_number_field("max_tokens")?
                    .unwrap_or(1000.0) as u32;
                Ok(NodeConfig::LLMPrompt {
                    model: None,
                    prompt_template,
                    temperature,
                    max_tokens,
                })
            },
            "TextProcessor" => {
                let operation = TextOperation::Trim; // Default, could parse from data
                Ok(NodeConfig::TextProcessor { operation })
            },
            "Conditional" => {
                let condition = self.extract_string_field("condition")?
                    .unwrap_or_else(|| "input == true".to_string());
                Ok(NodeConfig::Conditional { condition })
            },
            "Output" => {
                Ok(NodeConfig::Output { format: OutputFormat::Text })
            },
            "DataSource" => {
                Ok(NodeConfig::DataSource { source_type: DataSourceType::File })
            },
            "Transform" => {
                let transformation = self.extract_string_field("transformation")?
                    .unwrap_or_else(|| "identity".to_string());
                Ok(NodeConfig::Transform { transformation })
            },
            _ => Err(anyhow::anyhow!("Unknown config type: {}", self.config_type)),
        }
    }
    
    fn extract_string_field(&self, field_name: &str) -> Result<Option<String>> {
        if let Value::Object(obj) = &self.data {
            if let Some(Value::String(value)) = obj.get(field_name) {
                return Ok(Some(value.clone()));
            }
        }
        Ok(None)
    }
    
    fn extract_number_field(&self, field_name: &str) -> Result<Option<f64>> {
        if let Value::Object(obj) = &self.data {
            if let Some(Value::Number(value)) = obj.get(field_name) {
                return Ok(Some(*value));
            }
        }
        Ok(None)
    }
} 