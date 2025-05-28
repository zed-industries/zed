pub mod sequence_elements;
pub mod sequence_export;
pub mod sequence_import;
pub mod mermaid_export;

pub use sequence_elements::*;
pub use sequence_export::*;
pub use sequence_import::*;
pub use mermaid_export::*;

use workflow_core::*;
use workflow_schema::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sequence diagram representation of a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceDiagram {
    pub id: String,
    pub name: String,
    pub description: String,
    pub participants: Vec<Participant>,
    pub messages: Vec<Message>,
    pub activations: Vec<Activation>,
    pub notes: Vec<Note>,
    pub metadata: SequenceMetadata,
}

/// A participant in the sequence diagram (represents a module or external system)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub participant_type: ParticipantType,
    pub module_id: Option<ModuleId>,
    pub position: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantType {
    Module,
    ExternalSystem,
    User,
    Database,
    Service,
}

/// A message between participants (represents data flow)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub to: String,
    pub label: String,
    pub message_type: MessageType,
    pub data_type: Option<String>,
    pub sequence_number: usize,
    pub connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Synchronous,
    Asynchronous,
    Return,
    Create,
    Destroy,
    SelfMessage,
}

/// Activation box showing when a participant is active
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activation {
    pub id: String,
    pub participant: String,
    pub start_message: String,
    pub end_message: Option<String>,
}

/// Note attached to the diagram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub text: String,
    pub position: NotePosition,
    pub attached_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotePosition {
    Left,
    Right,
    Over,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceMetadata {
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub creator: String,
    pub version: String,
    pub workflow_id: WorkflowId,
}

/// Convert workflow to sequence diagram representation
pub trait ToSequence {
    fn to_sequence(&self) -> Result<SequenceDiagram, SequenceError>;
}

/// Convert sequence diagram back to workflow
pub trait FromSequence {
    fn from_sequence(sequence: &SequenceDiagram) -> Result<Self, SequenceError>
    where
        Self: Sized;
}

impl ToSequence for Workflow {
    fn to_sequence(&self) -> Result<SequenceDiagram, SequenceError> {
        let mut participants = Vec::new();
        let mut messages = Vec::new();
        let mut activations = Vec::new();
        let mut notes = Vec::new();

        // Create participants from modules
        for (i, (module_id, module)) in self.modules.iter().enumerate() {
            participants.push(Participant {
                id: format!("participant_{}", module_id),
                name: module.template.name.clone(),
                participant_type: determine_participant_type(&module.template),
                module_id: Some(*module_id),
                position: i,
            });
        }

        // Add external participants if needed
        if self.has_external_inputs() {
            participants.insert(0, Participant {
                id: "external_input".to_string(),
                name: "External Input".to_string(),
                participant_type: ParticipantType::ExternalSystem,
                module_id: None,
                position: 0,
            });
            
            // Adjust positions
            for participant in &mut participants[1..] {
                participant.position += 1;
            }
        }

        if self.has_external_outputs() {
            participants.push(Participant {
                id: "external_output".to_string(),
                name: "External Output".to_string(),
                participant_type: ParticipantType::ExternalSystem,
                module_id: None,
                position: participants.len(),
            });
        }

        // Create messages from connections
        let mut sequence_number = 1;
        for connection in &self.connections {
            let from_participant = format!("participant_{}", connection.from_module);
            let to_participant = format!("participant_{}", connection.to_module);
            
            let from_module = self.modules.get(&connection.from_module)
                .ok_or_else(|| SequenceError::InvalidConnection(format!("Module {} not found", connection.from_module)))?;
            let to_module = self.modules.get(&connection.to_module)
                .ok_or_else(|| SequenceError::InvalidConnection(format!("Module {} not found", connection.to_module)))?;

            let from_port = from_module.get_output_port(&connection.from_port)
                .ok_or_else(|| SequenceError::InvalidConnection(format!("Output port {} not found", connection.from_port)))?;
            let to_port = to_module.get_input_port(&connection.to_port)
                .ok_or_else(|| SequenceError::InvalidConnection(format!("Input port {} not found", connection.to_port)))?;

            messages.push(Message {
                id: format!("message_{}", sequence_number),
                from: from_participant,
                to: to_participant,
                label: format!("{}: {} → {}: {}", 
                    from_port.name, from_port.bit_type,
                    to_port.name, to_port.bit_type),
                message_type: determine_message_type(from_port, to_port),
                data_type: Some(from_port.bit_type.to_string()),
                sequence_number,
                connection_id: Some(format!("{}_{}_to_{}_{}", 
                    connection.from_module, connection.from_port,
                    connection.to_module, connection.to_port)),
            });

            sequence_number += 1;
        }

        // Create activations for each module
        for (module_id, _module) in &self.modules {
            let participant_id = format!("participant_{}", module_id);
            
            // Find first and last messages for this participant
            let first_message = messages.iter()
                .find(|m| m.to == participant_id)
                .map(|m| m.id.clone());
            
            let last_message = messages.iter()
                .filter(|m| m.from == participant_id)
                .last()
                .map(|m| m.id.clone());

            if let Some(start) = first_message {
                activations.push(Activation {
                    id: format!("activation_{}", module_id),
                    participant: participant_id,
                    start_message: start,
                    end_message: last_message,
                });
            }
        }

        // Add notes for complex bit transformations
        for (i, connection) in self.connections.iter().enumerate() {
            let from_module = &self.modules[&connection.from_module];
            let to_module = &self.modules[&connection.to_module];
            
            if let (Some(from_port), Some(to_port)) = (
                from_module.get_output_port(&connection.from_port),
                to_module.get_input_port(&connection.to_port)
            ) {
                if from_port.bit_type != to_port.bit_type {
                    notes.push(Note {
                        id: format!("note_{}", i),
                        text: format!("Bit transformation: {} → {}", 
                            from_port.bit_type, to_port.bit_type),
                        position: NotePosition::Right,
                        attached_to: Some(format!("message_{}", i + 1)),
                    });
                }
            }
        }

        Ok(SequenceDiagram {
            id: self.id.to_string(),
            name: self.name.clone(),
            description: self.description.clone(),
            participants,
            messages,
            activations,
            notes,
            metadata: SequenceMetadata {
                created: chrono::Utc::now(),
                modified: chrono::Utc::now(),
                creator: "Workflow AI".to_string(),
                version: "1.0".to_string(),
                workflow_id: self.id,
            },
        })
    }
}

fn determine_participant_type(template: &ModuleTemplate) -> ParticipantType {
    match template.category.as_str() {
        "input" | "output" => ParticipantType::ExternalSystem,
        "database" => ParticipantType::Database,
        "service" => ParticipantType::Service,
        "user" => ParticipantType::User,
        _ => ParticipantType::Module,
    }
}

fn determine_message_type(from_port: &Port, to_port: &Port) -> MessageType {
    // For now, all messages are synchronous
    // In the future, this could be determined by port metadata or module types
    MessageType::Synchronous
}

impl Workflow {
    fn has_external_inputs(&self) -> bool {
        // Check if any module has inputs that aren't connected
        for module in self.modules.values() {
            for input in &module.template.inputs {
                let is_connected = self.connections.iter()
                    .any(|c| c.to_module == module.id && c.to_port == input.id);
                if !is_connected {
                    return true;
                }
            }
        }
        false
    }

    fn has_external_outputs(&self) -> bool {
        // Check if any module has outputs that aren't connected
        for module in self.modules.values() {
            for output in &module.template.outputs {
                let is_connected = self.connections.iter()
                    .any(|c| c.from_module == module.id && c.from_port == output.id);
                if !is_connected {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SequenceError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Workflow error: {0}")]
    Workflow(#[from] WorkflowError),
    
    #[error("Invalid connection: {0}")]
    InvalidConnection(String),
    
    #[error("Invalid sequence structure: {0}")]
    InvalidStructure(String),
    
    #[error("Conversion error: {0}")]
    Conversion(String),
} 