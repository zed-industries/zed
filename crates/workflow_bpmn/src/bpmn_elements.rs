use serde::{Deserialize, Serialize};

/// BPMN Element types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BpmnElement {
    StartEvent(BpmnStartEvent),
    EndEvent(BpmnEndEvent),
    IntermediateEvent(BpmnIntermediateEvent),
    Task(BpmnTask),
    ServiceTask(BpmnServiceTask),
    UserTask(BpmnUserTask),
    ScriptTask(BpmnScriptTask),
    Gateway(BpmnGateway),
    SubProcess(BpmnSubProcess),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnStartEvent {
    pub id: String,
    pub name: String,
    pub outgoing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnEndEvent {
    pub id: String,
    pub name: String,
    pub incoming: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnIntermediateEvent {
    pub id: String,
    pub name: String,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub event_definition: Option<BpmnEventDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnTask {
    pub id: String,
    pub name: String,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub io_specification: Option<BpmnIoSpecification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnServiceTask {
    pub id: String,
    pub name: String,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub implementation: String,
    pub operation_ref: Option<String>,
    pub io_specification: Option<BpmnIoSpecification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnUserTask {
    pub id: String,
    pub name: String,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub implementation: String,
    pub io_specification: Option<BpmnIoSpecification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnScriptTask {
    pub id: String,
    pub name: String,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub script_format: String,
    pub script: String,
    pub io_specification: Option<BpmnIoSpecification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnGateway {
    pub id: String,
    pub name: String,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub gateway_type: BpmnGatewayType,
    pub gateway_direction: BpmnGatewayDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BpmnGatewayType {
    Exclusive,
    Inclusive,
    Parallel,
    Complex,
    EventBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BpmnGatewayDirection {
    Unspecified,
    Converging,
    Diverging,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnSubProcess {
    pub id: String,
    pub name: String,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,
    pub triggered_by_event: bool,
    pub elements: Vec<BpmnElement>,
    pub sequence_flows: Vec<BpmnSequenceFlow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnSequenceFlow {
    pub id: String,
    pub name: String,
    pub source_ref: String,
    pub target_ref: String,
    pub condition_expression: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnDataObject {
    pub id: String,
    pub name: String,
    pub item_subject_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnIoSpecification {
    pub data_inputs: Vec<BpmnDataInput>,
    pub data_outputs: Vec<BpmnDataOutput>,
    pub input_sets: Vec<BpmnInputSet>,
    pub output_sets: Vec<BpmnOutputSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnDataInput {
    pub id: String,
    pub name: String,
    pub item_subject_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnDataOutput {
    pub id: String,
    pub name: String,
    pub item_subject_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnInputSet {
    pub id: String,
    pub data_input_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnOutputSet {
    pub id: String,
    pub data_output_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BpmnEventDefinition {
    Timer(BpmnTimerEventDefinition),
    Message(BpmnMessageEventDefinition),
    Signal(BpmnSignalEventDefinition),
    Error(BpmnErrorEventDefinition),
    Escalation(BpmnEscalationEventDefinition),
    Compensation(BpmnCompensationEventDefinition),
    Conditional(BpmnConditionalEventDefinition),
    Link(BpmnLinkEventDefinition),
    Terminate(BpmnTerminateEventDefinition),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnTimerEventDefinition {
    pub id: String,
    pub time_date: Option<String>,
    pub time_duration: Option<String>,
    pub time_cycle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnMessageEventDefinition {
    pub id: String,
    pub message_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnSignalEventDefinition {
    pub id: String,
    pub signal_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnErrorEventDefinition {
    pub id: String,
    pub error_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnEscalationEventDefinition {
    pub id: String,
    pub escalation_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnCompensationEventDefinition {
    pub id: String,
    pub activity_ref: Option<String>,
    pub wait_for_completion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnConditionalEventDefinition {
    pub id: String,
    pub condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnLinkEventDefinition {
    pub id: String,
    pub name: String,
    pub source: Vec<String>,
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmnTerminateEventDefinition {
    pub id: String,
} 