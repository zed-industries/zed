use crate::{SequenceDiagram, Participant, Message, MessageType, Note, NotePosition, SequenceError};

/// Export sequence diagram to Mermaid format
pub trait ToMermaid {
    fn to_mermaid(&self) -> Result<String, SequenceError>;
}

impl ToMermaid for SequenceDiagram {
    fn to_mermaid(&self) -> Result<String, SequenceError> {
        let mut mermaid = String::new();
        
        // Header
        mermaid.push_str("sequenceDiagram\n");
        mermaid.push_str(&format!("    title {}\n", self.name));
        mermaid.push('\n');

        // Participants
        for participant in &self.participants {
            let participant_type = match participant.participant_type {
                crate::ParticipantType::Module => "participant",
                crate::ParticipantType::ExternalSystem => "participant",
                crate::ParticipantType::User => "actor",
                crate::ParticipantType::Database => "participant",
                crate::ParticipantType::Service => "participant",
            };
            
            mermaid.push_str(&format!("    {} {} as {}\n", 
                participant_type, 
                sanitize_id(&participant.id), 
                participant.name));
        }
        mermaid.push('\n');

        // Messages sorted by sequence number
        let mut sorted_messages = self.messages.clone();
        sorted_messages.sort_by_key(|m| m.sequence_number);

        for message in &sorted_messages {
            let arrow = match message.message_type {
                MessageType::Synchronous => "->",
                MessageType::Asynchronous => "->>",
                MessageType::Return => "-->>",
                MessageType::Create => "->>+",
                MessageType::Destroy => "->>-",
                MessageType::SelfMessage => "->",
            };

            let from_id = sanitize_id(&message.from);
            let to_id = sanitize_id(&message.to);
            
            if message.message_type == MessageType::SelfMessage || from_id == to_id {
                mermaid.push_str(&format!("    {} {}+ {}\n", 
                    from_id, arrow, message.label));
            } else {
                mermaid.push_str(&format!("    {} {} {}: {}\n", 
                    from_id, arrow, to_id, message.label));
            }

            // Add data type as note if present
            if let Some(data_type) = &message.data_type {
                mermaid.push_str(&format!("    note right of {}: {}\n", 
                    to_id, data_type));
            }
        }

        // Notes
        for note in &self.notes {
            let position = match note.position {
                NotePosition::Left => "left of",
                NotePosition::Right => "right of", 
                NotePosition::Over => "over",
            };

            if let Some(attached_to) = &note.attached_to {
                // Try to find the participant this note is attached to
                if let Some(message) = self.messages.iter().find(|m| m.id == *attached_to) {
                    let participant_id = sanitize_id(&message.to);
                    mermaid.push_str(&format!("    note {} {}: {}\n", 
                        position, participant_id, note.text));
                }
            }
        }

        // Add activations as comments for reference
        if !self.activations.is_empty() {
            mermaid.push_str("\n    %% Activations:\n");
            for activation in &self.activations {
                mermaid.push_str(&format!("    %% {} active from {} to {}\n", 
                    activation.participant,
                    activation.start_message,
                    activation.end_message.as_deref().unwrap_or("end")));
            }
        }

        Ok(mermaid)
    }
}

/// Sanitize identifier for Mermaid compatibility
fn sanitize_id(id: &str) -> String {
    id.replace('-', "_")
      .replace(' ', "_")
      .chars()
      .filter(|c| c.is_alphanumeric() || *c == '_')
      .collect()
}

/// Export to PlantUML format as an alternative
pub trait ToPlantUml {
    fn to_plantuml(&self) -> Result<String, SequenceError>;
}

impl ToPlantUml for SequenceDiagram {
    fn to_plantuml(&self) -> Result<String, SequenceError> {
        let mut plantuml = String::new();
        
        // Header
        plantuml.push_str("@startuml\n");
        plantuml.push_str(&format!("title {}\n", self.name));
        plantuml.push('\n');

        // Participants
        for participant in &self.participants {
            let participant_type = match participant.participant_type {
                crate::ParticipantType::Module => "participant",
                crate::ParticipantType::ExternalSystem => "participant",
                crate::ParticipantType::User => "actor",
                crate::ParticipantType::Database => "database",
                crate::ParticipantType::Service => "control",
            };
            
            plantuml.push_str(&format!("{} \"{}\" as {}\n", 
                participant_type, 
                participant.name,
                sanitize_id(&participant.id)));
        }
        plantuml.push('\n');

        // Messages sorted by sequence number
        let mut sorted_messages = self.messages.clone();
        sorted_messages.sort_by_key(|m| m.sequence_number);

        for message in &sorted_messages {
            let arrow = match message.message_type {
                MessageType::Synchronous => "->",
                MessageType::Asynchronous => "->>",
                MessageType::Return => "-->>",
                MessageType::Create => "->",
                MessageType::Destroy => "->",
                MessageType::SelfMessage => "->",
            };

            let from_id = sanitize_id(&message.from);
            let to_id = sanitize_id(&message.to);
            
            plantuml.push_str(&format!("{} {} {} : {}\n", 
                from_id, arrow, to_id, message.label));

            // Add activation
            if message.message_type == MessageType::Create {
                plantuml.push_str(&format!("activate {}\n", to_id));
            } else if message.message_type == MessageType::Destroy {
                plantuml.push_str(&format!("deactivate {}\n", to_id));
            }
        }

        // Notes
        for note in &self.notes {
            let position = match note.position {
                NotePosition::Left => "left",
                NotePosition::Right => "right", 
                NotePosition::Over => "over",
            };

            if let Some(attached_to) = &note.attached_to {
                if let Some(message) = self.messages.iter().find(|m| m.id == *attached_to) {
                    let participant_id = sanitize_id(&message.to);
                    plantuml.push_str(&format!("note {} of {} : {}\n", 
                        position, participant_id, note.text));
                }
            }
        }

        plantuml.push_str("\n@enduml\n");

        Ok(plantuml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use workflow_core::*;

    #[test]
    fn test_mermaid_export() {
        let mut workflow = Workflow::new("Test Workflow".to_string());
        
        // Create a simple workflow for testing
        let template1 = ModuleTemplate::new("input".to_string(), "Input".to_string(), "input".to_string())
            .add_output(Port::new("out".to_string(), "Output".to_string(), BitType::Byte));
        
        let template2 = ModuleTemplate::new("process".to_string(), "Process".to_string(), "processing".to_string())
            .add_input(Port::new("in".to_string(), "Input".to_string(), BitType::Byte))
            .add_output(Port::new("out".to_string(), "Output".to_string(), BitType::Word));

        let module1 = template1.instantiate(Box::new(TestModule));
        let module2 = template2.instantiate(Box::new(TestModule));
        
        let module1_id = module1.id;
        let module2_id = module2.id;
        
        workflow.add_module(module1);
        workflow.add_module(module2);
        
        workflow.add_connection(Connection {
            from_module: module1_id,
            from_port: "out".to_string(),
            to_module: module2_id,
            to_port: "in".to_string(),
        }).unwrap();

        let sequence = workflow.to_sequence().unwrap();
        let mermaid = sequence.to_mermaid().unwrap();
        
        assert!(mermaid.contains("sequenceDiagram"));
        assert!(mermaid.contains("Input"));
        assert!(mermaid.contains("Process"));
        assert!(mermaid.contains("->"));
    }

    #[derive(Debug, Clone)]
    struct TestModule;

    impl ModuleLogic for TestModule {
        fn execute(&self, _context: &ExecutionContext) -> ExecutionResult {
            ExecutionResult::success(std::collections::HashMap::new())
        }

        fn description(&self) -> String {
            "Test module".to_string()
        }

        fn clone_box(&self) -> Box<dyn ModuleLogic> {
            Box::new(self.clone())
        }
    }
} 