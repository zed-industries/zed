use workflow_core::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{simple_factory, ModuleFactory};

/// Input module - provides external data input to the workflow
#[derive(Debug, Clone)]
pub struct InputModule {
    pub bit_type: BitType,
    pub default_value: Option<BitVector>,
}

impl InputModule {
    pub fn new(bit_type: BitType) -> Self {
        Self {
            bit_type,
            default_value: None,
        }
    }

    pub fn with_default(mut self, default: BitVector) -> Self {
        self.default_value = Some(default);
        self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("input".to_string(), "Input".to_string(), "io".to_string())
            .add_output(Port::new("data".to_string(), "Data".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for InputModule {
    fn execute(&self, _context: &ExecutionContext) -> ExecutionResult {
        // In a real implementation, this would get data from external source
        // For now, return the default value or empty data
        let output = self.default_value.clone().unwrap_or_else(|| {
            BitVector::new(Vec::new(), self.bit_type.clone())
        });
        
        let mut outputs = HashMap::new();
        outputs.insert("data".to_string(), output);
        ExecutionResult::success(outputs)
    }

    fn description(&self) -> String {
        format!("Input module for {} data", self.bit_type)
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Output module - consumes data and provides it as workflow output
#[derive(Debug, Clone)]
pub struct OutputModule {
    pub bit_type: BitType,
}

impl OutputModule {
    pub fn new(bit_type: BitType) -> Self {
        Self { bit_type }
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("output".to_string(), "Output".to_string(), "io".to_string())
            .add_input(Port::new("data".to_string(), "Data".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for OutputModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(_input) = context.get_input(&"data".to_string()) {
            // In a real implementation, this would send data to external destination
            ExecutionResult::success(HashMap::new())
        } else {
            ExecutionResult::error("No input data provided".to_string())
        }
    }

    fn description(&self) -> String {
        format!("Output module for {} data", self.bit_type)
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Constant module - provides a constant bit value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantModule {
    pub value: BitVector,
}

impl ConstantModule {
    pub fn new(value: BitVector) -> Self {
        Self { value }
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            value: BitVector::from_bytes(&[byte], BitType::Byte),
        }
    }

    pub fn from_bool(bit: bool) -> Self {
        Self {
            value: BitVector::new(vec![bit.into()], BitType::Boolean),
        }
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("constant".to_string(), "Constant".to_string(), "basic".to_string())
            .add_output(Port::new("value".to_string(), "Value".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for ConstantModule {
    fn execute(&self, _context: &ExecutionContext) -> ExecutionResult {
        let mut outputs = HashMap::new();
        outputs.insert("value".to_string(), self.value.clone());
        ExecutionResult::success(outputs)
    }

    fn description(&self) -> String {
        format!("Constant value: {} bits of type {}", self.value.len(), self.value.bit_type)
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bit selector module - selects specific bits from input
#[derive(Debug, Clone)]
pub struct BitSelectorModule {
    pub start_bit: usize,
    pub bit_count: usize,
}

impl BitSelectorModule {
    pub fn new(start_bit: usize, bit_count: usize) -> Self {
        Self { start_bit, bit_count }
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("bit_selector".to_string(), "Bit Selector".to_string(), "basic".to_string())
            .add_input(Port::new("input".to_string(), "Input".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "Selected Bits".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for BitSelectorModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(input) = context.get_input(&"input".to_string()) {
            if self.start_bit + self.bit_count > input.bits.len() {
                return ExecutionResult::error("Bit selection out of range".to_string());
            }

            let selected_bits = input.bits[self.start_bit..self.start_bit + self.bit_count].to_vec();
            let output = BitVector::new(selected_bits, BitType::Raw);

            let mut outputs = HashMap::new();
            outputs.insert("output".to_string(), output);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("No input provided".to_string())
        }
    }

    fn description(&self) -> String {
        format!("Selects {} bits starting from bit {}", self.bit_count, self.start_bit)
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bit concatenation module - combines multiple bit vectors
#[derive(Debug, Clone)]
pub struct BitConcatModule;

impl BitConcatModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("bit_concat".to_string(), "Bit Concatenation".to_string(), "basic".to_string())
            .add_input(Port::new("input1".to_string(), "Input 1".to_string(), BitType::Raw))
            .add_input(Port::new("input2".to_string(), "Input 2".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "Concatenated".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for BitConcatModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let input1 = context.get_input(&"input1".to_string());
        let input2 = context.get_input(&"input2".to_string());

        match (input1, input2) {
            (Some(i1), Some(i2)) => {
                let mut combined_bits = i1.bits.clone();
                combined_bits.extend(i2.bits.clone());
                
                let output = BitVector::new(combined_bits, BitType::Raw);
                let mut outputs = HashMap::new();
                outputs.insert("output".to_string(), output);
                ExecutionResult::success(outputs)
            }
            _ => ExecutionResult::error("Both inputs are required".to_string()),
        }
    }

    fn description(&self) -> String {
        "Concatenates two bit vectors".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bit shift module - shifts bits left or right
#[derive(Debug, Clone)]
pub struct BitShiftModule {
    pub direction: ShiftDirection,
    pub positions: usize,
    pub fill_bit: Bit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShiftDirection {
    Left,
    Right,
}

impl BitShiftModule {
    pub fn new(direction: ShiftDirection, positions: usize, fill_bit: Bit) -> Self {
        Self { direction, positions, fill_bit }
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("bit_shift".to_string(), "Bit Shift".to_string(), "basic".to_string())
            .add_input(Port::new("input".to_string(), "Input".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "Shifted".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for BitShiftModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(input) = context.get_input(&"input".to_string()) {
            let mut result_bits = input.bits.clone();
            
            match self.direction {
                ShiftDirection::Left => {
                    // Shift left: remove from front, add to back
                    for _ in 0..self.positions {
                        if !result_bits.is_empty() {
                            result_bits.remove(0);
                        }
                        result_bits.push(self.fill_bit);
                    }
                }
                ShiftDirection::Right => {
                    // Shift right: remove from back, add to front
                    for _ in 0..self.positions {
                        if !result_bits.is_empty() {
                            result_bits.pop();
                        }
                        result_bits.insert(0, self.fill_bit);
                    }
                }
            }

            let output = BitVector::new(result_bits, input.bit_type.clone());
            let mut outputs = HashMap::new();
            outputs.insert("output".to_string(), output);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("No input provided".to_string())
        }
    }

    fn description(&self) -> String {
        format!("Shifts bits {:?} by {} positions, filling with {:?}", 
            self.direction, self.positions, self.fill_bit)
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

// Factory implementations
pub struct InputModuleFactory;

impl crate::ModuleFactory for InputModuleFactory {
    fn create_logic(&self, _parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
        Some(Box::new(InputModule::new(BitType::Raw)))
    }
}

pub struct OutputModuleFactory;

impl crate::ModuleFactory for OutputModuleFactory {
    fn create_logic(&self, _parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
        Some(Box::new(OutputModule::new(BitType::Raw)))
    }
}

simple_factory!(BitConcatModuleFactory, BitConcatModule);

#[derive(Serialize, Deserialize)]
pub struct ConstantModuleConfig {
    pub value: Vec<u8>,
    pub bit_type: BitType,
}

pub struct ConstantModuleFactory;

impl crate::ModuleFactory for ConstantModuleFactory {
    fn create_logic(&self, parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
        let config: ConstantModuleConfig = serde_json::from_value(
            parameters.get("config")?.clone()
        ).ok()?;
        
        let bit_vector = BitVector::from_bytes(&config.value, config.bit_type);
        Some(Box::new(ConstantModule::new(bit_vector)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct BitSelectorConfig {
    pub start_bit: usize,
    pub bit_count: usize,
}

pub struct BitSelectorModuleFactory;

impl crate::ModuleFactory for BitSelectorModuleFactory {
    fn create_logic(&self, parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
        let config: BitSelectorConfig = serde_json::from_value(
            parameters.get("config")?.clone()
        ).ok()?;
        
        Some(Box::new(BitSelectorModule::new(config.start_bit, config.bit_count)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct BitShiftConfig {
    pub direction: ShiftDirection,
    pub positions: usize,
    pub fill_bit: bool,
}

pub struct BitShiftModuleFactory;

impl crate::ModuleFactory for BitShiftModuleFactory {
    fn create_logic(&self, parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
        let config: BitShiftConfig = serde_json::from_value(
            parameters.get("config")?.clone()
        ).ok()?;
        
        Some(Box::new(BitShiftModule::new(
            config.direction, 
            config.positions, 
            config.fill_bit.into()
        )))
    }
} 