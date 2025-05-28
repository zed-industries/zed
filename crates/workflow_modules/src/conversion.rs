use workflow_core::*;
use crate::{simple_factory, ModuleFactory};
use std::collections::HashMap;

/// Bit to Byte conversion module
#[derive(Debug, Clone)]
pub struct BitToByteModule;

impl BitToByteModule {
    pub fn new() -> Self { Self }
    
    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("bit_to_byte".to_string(), "Bit to Byte".to_string(), "Conversion".to_string())
            .add_input(Port::new("bits".to_string(), "Bits".to_string(), BitType::Raw))
            .add_output(Port::new("byte".to_string(), "Byte".to_string(), BitType::Byte))
    }
}

impl ModuleLogic for BitToByteModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(bits) = context.get_input(&"bits".to_string()) {
            if bits.bits.len() < 8 {
                return ExecutionResult::error("Need at least 8 bits for byte conversion".to_string());
            }
            
            // Take first 8 bits and convert to byte
            let byte_bits = bits.bits[0..8].to_vec();
            let byte_vector = BitVector::new(byte_bits, BitType::Byte);
            
            let mut outputs = HashMap::new();
            outputs.insert("byte".to_string(), byte_vector);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Missing input".to_string())
        }
    }
    
    fn description(&self) -> String { "Converts 8 bits to byte".to_string() }
    fn clone_box(&self) -> Box<dyn ModuleLogic> { Box::new(self.clone()) }
}

simple_factory!(BitToByteModuleFactory, BitToByteModule);

/// Byte to Word conversion module
#[derive(Debug, Clone)]
pub struct ByteToWordModule;

impl ByteToWordModule {
    pub fn new() -> Self { Self }
    
    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("byte_to_word".to_string(), "Byte to Word".to_string(), "Conversion".to_string())
            .add_input(Port::new("low_byte".to_string(), "Low Byte".to_string(), BitType::Byte))
            .add_input(Port::new("high_byte".to_string(), "High Byte".to_string(), BitType::Byte))
            .add_output(Port::new("word".to_string(), "Word".to_string(), BitType::Word))
    }
}

impl ModuleLogic for ByteToWordModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let (Some(low), Some(high)) = (context.get_input(&"low_byte".to_string()), context.get_input(&"high_byte".to_string())) {
            // Combine the two bytes into a word
            let mut word_bits = low.bits.clone();
            word_bits.extend(high.bits.clone());
            
            // Ensure we have exactly 16 bits
            word_bits.resize(16, Bit::Zero);
            
            let word_vector = BitVector::new(word_bits, BitType::Word);
            let mut outputs = HashMap::new();
            outputs.insert("word".to_string(), word_vector);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Missing inputs".to_string())
        }
    }
    
    fn description(&self) -> String { "Combines bytes to word".to_string() }
    fn clone_box(&self) -> Box<dyn ModuleLogic> { Box::new(self.clone()) }
}

simple_factory!(ByteToWordModuleFactory, ByteToWordModule);

/// Word to DWord conversion module
#[derive(Debug, Clone)]
pub struct WordToDWordModule;

impl WordToDWordModule {
    pub fn new() -> Self { Self }
    
    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("word_to_dword".to_string(), "Word to DWord".to_string(), "Conversion".to_string())
            .add_input(Port::new("low_word".to_string(), "Low Word".to_string(), BitType::Word))
            .add_input(Port::new("high_word".to_string(), "High Word".to_string(), BitType::Word))
            .add_output(Port::new("dword".to_string(), "DWord".to_string(), BitType::DWord))
    }
}

impl ModuleLogic for WordToDWordModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let (Some(low), Some(high)) = (context.get_input(&"low_word".to_string()), context.get_input(&"high_word".to_string())) {
            // Combine the two words into a dword
            let mut dword_bits = low.bits.clone();
            dword_bits.extend(high.bits.clone());
            
            // Ensure we have exactly 32 bits
            dword_bits.resize(32, Bit::Zero);
            
            let dword_vector = BitVector::new(dword_bits, BitType::DWord);
            let mut outputs = HashMap::new();
            outputs.insert("dword".to_string(), dword_vector);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Missing inputs".to_string())
        }
    }
    
    fn description(&self) -> String { "Combines words to dword".to_string() }
    fn clone_box(&self) -> Box<dyn ModuleLogic> { Box::new(self.clone()) }
}

simple_factory!(WordToDWordModuleFactory, WordToDWordModule);

/// Split Byte module - splits a byte into individual bits
#[derive(Debug, Clone)]
pub struct SplitByteModule;

impl SplitByteModule {
    pub fn new() -> Self { Self }
    
    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("split_byte".to_string(), "Split Byte".to_string(), "Conversion".to_string())
            .add_input(Port::new("byte".to_string(), "Byte".to_string(), BitType::Byte))
            .add_output(Port::new("bit0".to_string(), "Bit 0".to_string(), BitType::Boolean))
            .add_output(Port::new("bit1".to_string(), "Bit 1".to_string(), BitType::Boolean))
            .add_output(Port::new("bit2".to_string(), "Bit 2".to_string(), BitType::Boolean))
            .add_output(Port::new("bit3".to_string(), "Bit 3".to_string(), BitType::Boolean))
            .add_output(Port::new("bit4".to_string(), "Bit 4".to_string(), BitType::Boolean))
            .add_output(Port::new("bit5".to_string(), "Bit 5".to_string(), BitType::Boolean))
            .add_output(Port::new("bit6".to_string(), "Bit 6".to_string(), BitType::Boolean))
            .add_output(Port::new("bit7".to_string(), "Bit 7".to_string(), BitType::Boolean))
    }
}

impl ModuleLogic for SplitByteModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(byte_data) = context.get_input(&"byte".to_string()) {
            if byte_data.bits.len() < 8 {
                return ExecutionResult::error("Byte must have at least 8 bits".to_string());
            }
            
            let mut outputs = HashMap::new();
            for i in 0..8 {
                let bit = if i < byte_data.bits.len() {
                    byte_data.bits[i]
                } else {
                    Bit::Zero
                };
                let bit_vector = BitVector::new(vec![bit], BitType::Boolean);
                outputs.insert(format!("bit{}", i), bit_vector);
            }
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Missing input".to_string())
        }
    }
    
    fn description(&self) -> String { "Splits byte into individual bits".to_string() }
    fn clone_box(&self) -> Box<dyn ModuleLogic> { Box::new(self.clone()) }
}

simple_factory!(SplitByteModuleFactory, SplitByteModule); 