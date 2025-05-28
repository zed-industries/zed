use workflow_core::*;
use crate::{simple_factory, ModuleFactory};
use std::collections::HashMap;

/// Addition module - adds two bit values
#[derive(Debug, Clone)]
pub struct AddModule;

impl AddModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new(
            "add".to_string(),
            "Add".to_string(),
            "Arithmetic".to_string(),
        )
        .add_input(Port::new("a".to_string(), "A".to_string(), BitType::Byte))
        .add_input(Port::new("b".to_string(), "B".to_string(), BitType::Byte))
        .add_output(Port::new("result".to_string(), "Result".to_string(), BitType::Byte))
    }
}

impl ModuleLogic for AddModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let a = context.get_input(&"a".to_string());
        let b = context.get_input(&"b".to_string());
        
        if let (Some(a), Some(b)) = (a, b) {
            match a.add(b) {
                Ok(result) => {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), result);
                    ExecutionResult::success(outputs)
                }
                Err(e) => ExecutionResult::error(format!("Addition failed: {}", e))
            }
        } else {
            ExecutionResult::error("Missing inputs".to_string())
        }
    }

    fn description(&self) -> String {
        "Adds two values".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

simple_factory!(AddModuleFactory, AddModule);

/// Subtraction module - subtracts one bit value from another
#[derive(Debug, Clone)]
pub struct SubtractModule;

impl SubtractModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new(
            "subtract".to_string(),
            "Subtract".to_string(),
            "Arithmetic".to_string(),
        )
        .add_input(Port::new("a".to_string(), "A".to_string(), BitType::Byte))
        .add_input(Port::new("b".to_string(), "B".to_string(), BitType::Byte))
        .add_output(Port::new("result".to_string(), "Result".to_string(), BitType::Byte))
    }
}

impl ModuleLogic for SubtractModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let a = context.get_input(&"a".to_string());
        let b = context.get_input(&"b".to_string());
        
        if let (Some(a), Some(b)) = (a, b) {
            match a.subtract(b) {
                Ok(result) => {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), result);
                    ExecutionResult::success(outputs)
                }
                Err(e) => ExecutionResult::error(format!("Subtraction failed: {}", e))
            }
        } else {
            ExecutionResult::error("Missing inputs".to_string())
        }
    }

    fn description(&self) -> String {
        "Subtracts B from A".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

simple_factory!(SubtractModuleFactory, SubtractModule);

/// Multiplication module - multiplies two bit values
#[derive(Debug, Clone)]
pub struct MultiplyModule;

impl MultiplyModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new(
            "multiply".to_string(),
            "Multiply".to_string(),
            "Arithmetic".to_string(),
        )
        .add_input(Port::new("a".to_string(), "A".to_string(), BitType::Byte))
        .add_input(Port::new("b".to_string(), "B".to_string(), BitType::Byte))
        .add_output(Port::new("result".to_string(), "Result".to_string(), BitType::Word))
    }
}

impl ModuleLogic for MultiplyModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let a = context.get_input(&"a".to_string());
        let b = context.get_input(&"b".to_string());
        
        if let (Some(a), Some(b)) = (a, b) {
            match a.multiply(b) {
                Ok(result) => {
                    let mut outputs = HashMap::new();
                    outputs.insert("result".to_string(), result);
                    ExecutionResult::success(outputs)
                }
                Err(e) => ExecutionResult::error(format!("Multiplication failed: {}", e))
            }
        } else {
            ExecutionResult::error("Missing inputs".to_string())
        }
    }

    fn description(&self) -> String {
        "Multiplies two values".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

simple_factory!(MultiplyModuleFactory, MultiplyModule);

/// Division module - divides one bit value by another
#[derive(Debug, Clone)]
pub struct DivideModule;

impl DivideModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new(
            "divide".to_string(),
            "Divide".to_string(),
            "Arithmetic".to_string(),
        )
        .add_input(Port::new("a".to_string(), "A".to_string(), BitType::Byte))
        .add_input(Port::new("b".to_string(), "B".to_string(), BitType::Byte))
        .add_output(Port::new("quotient".to_string(), "Quotient".to_string(), BitType::Byte))
        .add_output(Port::new("remainder".to_string(), "Remainder".to_string(), BitType::Byte))
    }
}

impl ModuleLogic for DivideModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let a = context.get_input(&"a".to_string());
        let b = context.get_input(&"b".to_string());
        
        if let (Some(a), Some(b)) = (a, b) {
            match a.divide(b) {
                Ok((quotient, remainder)) => {
                    let mut outputs = HashMap::new();
                    outputs.insert("quotient".to_string(), quotient);
                    outputs.insert("remainder".to_string(), remainder);
                    ExecutionResult::success(outputs)
                }
                Err(e) => ExecutionResult::error(format!("Division failed: {}", e))
            }
        } else {
            ExecutionResult::error("Missing inputs".to_string())
        }
    }

    fn description(&self) -> String {
        "Divides A by B".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

simple_factory!(DivideModuleFactory, DivideModule); 