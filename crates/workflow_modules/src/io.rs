// IO modules for external data input/output
// This module will be expanded later with file I/O, network I/O, etc. 

use workflow_core::*;
use crate::{simple_factory, ModuleFactory};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// File Read module - reads data from a file
#[derive(Debug, Clone)]
pub struct FileReadModule {
    pub file_path: String,
}

impl FileReadModule {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("file_read".to_string(), "File Read".to_string(), "I/O".to_string())
            .add_input(Port::new("path".to_string(), "File Path".to_string(), BitType::Text))
            .add_output(Port::new("data".to_string(), "File Data".to_string(), BitType::Raw))
            .add_output(Port::new("success".to_string(), "Success".to_string(), BitType::Boolean))
    }
}

impl ModuleLogic for FileReadModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let file_path = if let Some(path_input) = context.get_input(&"path".to_string()) {
            // Convert bit vector to string (simplified)
            String::from_utf8_lossy(&path_input.to_bytes()).to_string()
        } else {
            self.file_path.clone()
        };

        match std::fs::read(&file_path) {
            Ok(data) => {
                let data_bits = BitVector::from_bytes(&data, BitType::Raw);
                let success_bit = BitVector::new(vec![Bit::One], BitType::Boolean);
                
                let mut outputs = HashMap::new();
                outputs.insert("data".to_string(), data_bits);
                outputs.insert("success".to_string(), success_bit);
                ExecutionResult::success(outputs)
            }
            Err(_e) => {
                let empty_data = BitVector::new(Vec::new(), BitType::Raw);
                let failure_bit = BitVector::new(vec![Bit::Zero], BitType::Boolean);
                
                let mut outputs = HashMap::new();
                outputs.insert("data".to_string(), empty_data);
                outputs.insert("success".to_string(), failure_bit);
                ExecutionResult::success(outputs) // Don't fail the execution, just indicate failure
            }
        }
    }

    fn description(&self) -> String {
        format!("Reads data from file: {}", self.file_path)
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// File Write module - writes data to a file
#[derive(Debug, Clone)]
pub struct FileWriteModule {
    pub file_path: String,
}

impl FileWriteModule {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("file_write".to_string(), "File Write".to_string(), "I/O".to_string())
            .add_input(Port::new("path".to_string(), "File Path".to_string(), BitType::Text))
            .add_input(Port::new("data".to_string(), "Data".to_string(), BitType::Raw))
            .add_output(Port::new("success".to_string(), "Success".to_string(), BitType::Boolean))
    }
}

impl ModuleLogic for FileWriteModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let file_path = if let Some(path_input) = context.get_input(&"path".to_string()) {
            String::from_utf8_lossy(&path_input.to_bytes()).to_string()
        } else {
            self.file_path.clone()
        };

        if let Some(data) = context.get_input(&"data".to_string()) {
            let bytes = data.to_bytes();
            match std::fs::write(&file_path, &bytes) {
                Ok(_) => {
                    let success_bit = BitVector::new(vec![Bit::One], BitType::Boolean);
                    let mut outputs = HashMap::new();
                    outputs.insert("success".to_string(), success_bit);
                    ExecutionResult::success(outputs)
                }
                Err(_e) => {
                    let failure_bit = BitVector::new(vec![Bit::Zero], BitType::Boolean);
                    let mut outputs = HashMap::new();
                    outputs.insert("success".to_string(), failure_bit);
                    ExecutionResult::success(outputs)
                }
            }
        } else {
            ExecutionResult::error("Missing data input".to_string())
        }
    }

    fn description(&self) -> String {
        format!("Writes data to file: {}", self.file_path)
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Console Output module - prints data to console
#[derive(Debug, Clone)]
pub struct ConsoleOutputModule;

impl ConsoleOutputModule {
    pub fn new() -> Self { Self }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("console_output".to_string(), "Console Output".to_string(), "I/O".to_string())
            .add_input(Port::new("data".to_string(), "Data".to_string(), BitType::Raw))
            .add_input(Port::new("format".to_string(), "Format".to_string(), BitType::Text))
            .add_output(Port::new("success".to_string(), "Success".to_string(), BitType::Boolean))
    }
}

impl ModuleLogic for ConsoleOutputModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(data) = context.get_input(&"data".to_string()) {
            let format_type = if let Some(format_input) = context.get_input(&"format".to_string()) {
                String::from_utf8_lossy(&format_input.to_bytes()).to_string()
            } else {
                "hex".to_string()
            };

            let output = match format_type.as_str() {
                "text" => String::from_utf8_lossy(&data.to_bytes()).to_string(),
                "binary" => data.bits.iter().map(|b| if matches!(b, Bit::One) { '1' } else { '0' }).collect(),
                "decimal" => data.to_u64().unwrap_or(0).to_string(),
                _ => format!("{:02x?}", data.to_bytes()), // hex format
            };

            println!("Workflow Output: {}", output);

            let success_bit = BitVector::new(vec![Bit::One], BitType::Boolean);
            let mut outputs = HashMap::new();
            outputs.insert("success".to_string(), success_bit);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Missing data input".to_string())
        }
    }

    fn description(&self) -> String {
        "Prints data to console".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Network Send module - sends data over network (simplified)
#[derive(Debug, Clone)]
pub struct NetworkSendModule {
    pub endpoint: String,
}

impl NetworkSendModule {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("network_send".to_string(), "Network Send".to_string(), "I/O".to_string())
            .add_input(Port::new("endpoint".to_string(), "Endpoint".to_string(), BitType::Text))
            .add_input(Port::new("data".to_string(), "Data".to_string(), BitType::Raw))
            .add_output(Port::new("success".to_string(), "Success".to_string(), BitType::Boolean))
            .add_output(Port::new("response".to_string(), "Response".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for NetworkSendModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        // Simplified implementation - just simulate network operation
        if let Some(_data) = context.get_input(&"data".to_string()) {
            // In a real implementation, this would make an actual network request
            let success_bit = BitVector::new(vec![Bit::One], BitType::Boolean);
            let response_data = BitVector::from_bytes(b"OK", BitType::Raw);
            
            let mut outputs = HashMap::new();
            outputs.insert("success".to_string(), success_bit);
            outputs.insert("response".to_string(), response_data);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Missing data input".to_string())
        }
    }

    fn description(&self) -> String {
        format!("Sends data to: {}", self.endpoint)
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

// Factory implementations
simple_factory!(ConsoleOutputModuleFactory, ConsoleOutputModule);

#[derive(Serialize, Deserialize)]
pub struct FileReadConfig {
    pub file_path: String,
}

pub struct FileReadModuleFactory;

impl ModuleFactory for FileReadModuleFactory {
    fn create_logic(&self, parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
        let config: FileReadConfig = serde_json::from_value(
            parameters.get("config")?.clone()
        ).ok()?;
        Some(Box::new(FileReadModule::new(config.file_path)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileWriteConfig {
    pub file_path: String,
}

pub struct FileWriteModuleFactory;

impl ModuleFactory for FileWriteModuleFactory {
    fn create_logic(&self, parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
        let config: FileWriteConfig = serde_json::from_value(
            parameters.get("config")?.clone()
        ).ok()?;
        Some(Box::new(FileWriteModule::new(config.file_path)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct NetworkSendConfig {
    pub endpoint: String,
}

pub struct NetworkSendModuleFactory;

impl ModuleFactory for NetworkSendModuleFactory {
    fn create_logic(&self, parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
        let config: NetworkSendConfig = serde_json::from_value(
            parameters.get("config")?.clone()
        ).ok()?;
        Some(Box::new(NetworkSendModule::new(config.endpoint)))
    }
} 