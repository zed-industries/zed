use gpui::{Point, Size, Bounds, px, Pixels};
use language_model::LanguageModel;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CanvasElement {
    Node(NodeId),
    NodePort { node_id: NodeId, port_id: String },
    Connection(Uuid),
    Canvas,
}

#[derive(Clone)]
pub struct WorkflowNode {
    pub id: NodeId,
    pub node_type: NodeType,
    pub position: Point<f32>,
    pub size: Size<f32>,
    pub title: String,
    pub inputs: Vec<NodePort>,
    pub outputs: Vec<NodePort>,
    pub config: NodeConfig,
    pub state: NodeState,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Input,
    LLMPrompt,
    TextProcessor,
    Conditional,
    Output,
    DataSource,
    Transform,
}

#[derive(Clone)]
pub struct NodePort {
    pub id: String,
    pub name: String,
    pub port_type: PortType,
    pub data_type: DataType,
}

#[derive(Clone, Debug)]
pub enum PortType {
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub enum DataType {
    Text,
    Number,
    Boolean,
    Object,
    Array,
}

#[derive(Clone)]
pub struct NodeConnection {
    pub id: Uuid,
    pub from_node: NodeId,
    pub from_port: String,
    pub to_node: NodeId,
    pub to_port: String,
}

#[derive(Clone)]
pub enum NodeConfig {
    Input { placeholder: String },
    LLMPrompt { 
        model: Option<Arc<dyn LanguageModel>>,
        prompt_template: String,
        temperature: f32,
        max_tokens: u32,
    },
    TextProcessor { operation: TextOperation },
    Conditional { condition: String },
    Output { format: OutputFormat },
    DataSource { source_type: DataSourceType },
    Transform { transformation: String },
}

#[derive(Clone, Debug)]
pub enum TextOperation {
    Uppercase,
    Lowercase,
    Trim,
    Replace { from: String, to: String },
    Split { delimiter: String },
}

#[derive(Clone, Debug)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
}

#[derive(Clone, Debug)]
pub enum DataSourceType {
    File,
    Api,
    Database,
    UserInput,
}

#[derive(Clone, Debug)]
pub enum NodeState {
    Idle,
    Running,
    Completed,
    Error(String),
}

#[derive(Clone, Debug)]
pub enum ExecutionState {
    Stopped,
    Running,
    Paused,
    Completed,
    Error(String),
}

#[derive(Clone)]
pub struct CanvasViewport {
    pub offset: Point<f32>,
    pub scale: f32,
    pub bounds: Bounds<Pixels>,
}

impl Default for CanvasViewport {
    fn default() -> Self {
        Self {
            offset: Point::new(0.0, 0.0),
            scale: 1.0,
            bounds: Bounds::new(Point::new(px(0.0), px(0.0)), Size::new(px(800.0), px(600.0))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum InteractionState {
    None,
    NodeDrag { 
        node_id: NodeId, 
        drag_offset: Point<f32>,
    },
    CanvasPan { 
        start_screen_pos: Point<f32>, 
        start_offset: Point<f32> 
    },
}

impl Default for InteractionState {
    fn default() -> Self {
        Self::None
    }
}

impl WorkflowNode {
    pub fn create(id: NodeId, node_type: NodeType, position: Point<f32>) -> Self {
        let (title, inputs, outputs, config) = match &node_type {
            NodeType::Input => (
                "Input".to_string(),
                vec![],
                vec![NodePort {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    port_type: PortType::Output,
                    data_type: DataType::Text,
                }],
                NodeConfig::Input { placeholder: "Enter input...".to_string() },
            ),
            NodeType::LLMPrompt => (
                "LLM Prompt".to_string(),
                vec![NodePort {
                    id: "input".to_string(),
                    name: "Input".to_string(),
                    port_type: PortType::Input,
                    data_type: DataType::Text,
                }],
                vec![NodePort {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    port_type: PortType::Output,
                    data_type: DataType::Text,
                }],
                NodeConfig::LLMPrompt {
                    model: None,
                    prompt_template: "{{input}}".to_string(),
                    temperature: 0.7,
                    max_tokens: 1000,
                },
            ),
            NodeType::TextProcessor => (
                "Text Processor".to_string(),
                vec![NodePort {
                    id: "input".to_string(),
                    name: "Input".to_string(),
                    port_type: PortType::Input,
                    data_type: DataType::Text,
                }],
                vec![NodePort {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    port_type: PortType::Output,
                    data_type: DataType::Text,
                }],
                NodeConfig::TextProcessor { operation: TextOperation::Trim },
            ),
            NodeType::Conditional => (
                "Conditional".to_string(),
                vec![NodePort {
                    id: "input".to_string(),
                    name: "Input".to_string(),
                    port_type: PortType::Input,
                    data_type: DataType::Boolean,
                }],
                vec![
                    NodePort {
                        id: "true".to_string(),
                        name: "True".to_string(),
                        port_type: PortType::Output,
                        data_type: DataType::Text,
                    },
                    NodePort {
                        id: "false".to_string(),
                        name: "False".to_string(),
                        port_type: PortType::Output,
                        data_type: DataType::Text,
                    },
                ],
                NodeConfig::Conditional { condition: "input == true".to_string() },
            ),
            NodeType::Output => (
                "Output".to_string(),
                vec![NodePort {
                    id: "input".to_string(),
                    name: "Input".to_string(),
                    port_type: PortType::Input,
                    data_type: DataType::Text,
                }],
                vec![],
                NodeConfig::Output { format: OutputFormat::Text },
            ),
            NodeType::DataSource => (
                "Data Source".to_string(),
                vec![],
                vec![NodePort {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    port_type: PortType::Output,
                    data_type: DataType::Object,
                }],
                NodeConfig::DataSource { source_type: DataSourceType::File },
            ),
            NodeType::Transform => (
                "Transform".to_string(),
                vec![NodePort {
                    id: "input".to_string(),
                    name: "Input".to_string(),
                    port_type: PortType::Input,
                    data_type: DataType::Object,
                }],
                vec![NodePort {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    port_type: PortType::Output,
                    data_type: DataType::Object,
                }],
                NodeConfig::Transform { transformation: "identity".to_string() },
            ),
        };

        Self {
            id,
            node_type,
            position,
            size: Size::new(200.0, 120.0),
            title,
            inputs,
            outputs,
            config,
            state: NodeState::Idle,
        }
    }
} 