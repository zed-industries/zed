use workflow_core::*;
use std::collections::HashMap;
use crate::{simple_factory, ModuleFactory};

/// Bitwise AND module
#[derive(Debug, Clone)]
pub struct AndModule;

impl AndModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("and".to_string(), "Bitwise AND".to_string(), "logical".to_string())
            .add_input(Port::new("input1".to_string(), "Input A".to_string(), BitType::Raw))
            .add_input(Port::new("input2".to_string(), "Input B".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "A AND B".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for AndModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let input1 = context.get_input(&"input1".to_string());
        let input2 = context.get_input(&"input2".to_string());

        match (input1, input2) {
            (Some(i1), Some(i2)) => {
                let min_len = i1.bits.len().min(i2.bits.len());
                let mut result_bits = Vec::new();

                for i in 0..min_len {
                    let bit1: bool = i1.bits[i].into();
                    let bit2: bool = i2.bits[i].into();
                    result_bits.push((bit1 && bit2).into());
                }

                let output = BitVector::new(result_bits, BitType::Raw);
                let mut outputs = HashMap::new();
                outputs.insert("output".to_string(), output);
                ExecutionResult::success(outputs)
            }
            _ => ExecutionResult::error("Both inputs are required".to_string()),
        }
    }

    fn description(&self) -> String {
        "Performs bitwise AND operation".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bitwise OR module
#[derive(Debug, Clone)]
pub struct OrModule;

impl OrModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("or".to_string(), "Bitwise OR".to_string(), "logical".to_string())
            .add_input(Port::new("input1".to_string(), "Input A".to_string(), BitType::Raw))
            .add_input(Port::new("input2".to_string(), "Input B".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "A OR B".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for OrModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let input1 = context.get_input(&"input1".to_string());
        let input2 = context.get_input(&"input2".to_string());

        match (input1, input2) {
            (Some(i1), Some(i2)) => {
                let min_len = i1.bits.len().min(i2.bits.len());
                let mut result_bits = Vec::new();

                for i in 0..min_len {
                    let bit1: bool = i1.bits[i].into();
                    let bit2: bool = i2.bits[i].into();
                    result_bits.push((bit1 || bit2).into());
                }

                let output = BitVector::new(result_bits, BitType::Raw);
                let mut outputs = HashMap::new();
                outputs.insert("output".to_string(), output);
                ExecutionResult::success(outputs)
            }
            _ => ExecutionResult::error("Both inputs are required".to_string()),
        }
    }

    fn description(&self) -> String {
        "Performs bitwise OR operation".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bitwise NOT module
#[derive(Debug, Clone)]
pub struct NotModule;

impl NotModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("not".to_string(), "Bitwise NOT".to_string(), "logical".to_string())
            .add_input(Port::new("input".to_string(), "Input".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "NOT Input".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for NotModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(input) = context.get_input(&"input".to_string()) {
            let result_bits: Vec<Bit> = input.bits.iter()
                .map(|&bit| {
                    let b: bool = bit.into();
                    (!b).into()
                })
                .collect();

            let output = BitVector::new(result_bits, input.bit_type.clone());
            let mut outputs = HashMap::new();
            outputs.insert("output".to_string(), output);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Input is required".to_string())
        }
    }

    fn description(&self) -> String {
        "Performs bitwise NOT operation (inversion)".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bitwise XOR module
#[derive(Debug, Clone)]
pub struct XorModule;

impl XorModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("xor".to_string(), "Bitwise XOR".to_string(), "logical".to_string())
            .add_input(Port::new("input1".to_string(), "Input A".to_string(), BitType::Raw))
            .add_input(Port::new("input2".to_string(), "Input B".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "A XOR B".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for XorModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let input1 = context.get_input(&"input1".to_string());
        let input2 = context.get_input(&"input2".to_string());

        match (input1, input2) {
            (Some(i1), Some(i2)) => {
                let min_len = i1.bits.len().min(i2.bits.len());
                let mut result_bits = Vec::new();

                for i in 0..min_len {
                    let bit1: bool = i1.bits[i].into();
                    let bit2: bool = i2.bits[i].into();
                    result_bits.push((bit1 ^ bit2).into());
                }

                let output = BitVector::new(result_bits, BitType::Raw);
                let mut outputs = HashMap::new();
                outputs.insert("output".to_string(), output);
                ExecutionResult::success(outputs)
            }
            _ => ExecutionResult::error("Both inputs are required".to_string()),
        }
    }

    fn description(&self) -> String {
        "Performs bitwise XOR operation".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bitwise NAND module
#[derive(Debug, Clone)]
pub struct NandModule;

impl NandModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("nand".to_string(), "Bitwise NAND".to_string(), "logical".to_string())
            .add_input(Port::new("input1".to_string(), "Input A".to_string(), BitType::Raw))
            .add_input(Port::new("input2".to_string(), "Input B".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "A NAND B".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for NandModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let input1 = context.get_input(&"input1".to_string());
        let input2 = context.get_input(&"input2".to_string());

        match (input1, input2) {
            (Some(i1), Some(i2)) => {
                let min_len = i1.bits.len().min(i2.bits.len());
                let mut result_bits = Vec::new();

                for i in 0..min_len {
                    let bit1: bool = i1.bits[i].into();
                    let bit2: bool = i2.bits[i].into();
                    result_bits.push((!(bit1 && bit2)).into());
                }

                let output = BitVector::new(result_bits, BitType::Raw);
                let mut outputs = HashMap::new();
                outputs.insert("output".to_string(), output);
                ExecutionResult::success(outputs)
            }
            _ => ExecutionResult::error("Both inputs are required".to_string()),
        }
    }

    fn description(&self) -> String {
        "Performs bitwise NAND operation".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bitwise NOR module
#[derive(Debug, Clone)]
pub struct NorModule;

impl NorModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("nor".to_string(), "Bitwise NOR".to_string(), "logical".to_string())
            .add_input(Port::new("input1".to_string(), "Input A".to_string(), BitType::Raw))
            .add_input(Port::new("input2".to_string(), "Input B".to_string(), BitType::Raw))
            .add_output(Port::new("output".to_string(), "A NOR B".to_string(), BitType::Raw))
    }
}

impl ModuleLogic for NorModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let input1 = context.get_input(&"input1".to_string());
        let input2 = context.get_input(&"input2".to_string());

        match (input1, input2) {
            (Some(i1), Some(i2)) => {
                let min_len = i1.bits.len().min(i2.bits.len());
                let mut result_bits = Vec::new();

                for i in 0..min_len {
                    let bit1: bool = i1.bits[i].into();
                    let bit2: bool = i2.bits[i].into();
                    result_bits.push((!(bit1 || bit2)).into());
                }

                let output = BitVector::new(result_bits, BitType::Raw);
                let mut outputs = HashMap::new();
                outputs.insert("output".to_string(), output);
                ExecutionResult::success(outputs)
            }
            _ => ExecutionResult::error("Both inputs are required".to_string()),
        }
    }

    fn description(&self) -> String {
        "Performs bitwise NOR operation".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bit comparison module - compares two bit vectors
#[derive(Debug, Clone)]
pub struct BitCompareModule;

impl BitCompareModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("bit_compare".to_string(), "Bit Compare".to_string(), "logical".to_string())
            .add_input(Port::new("input1".to_string(), "Input A".to_string(), BitType::Raw))
            .add_input(Port::new("input2".to_string(), "Input B".to_string(), BitType::Raw))
            .add_output(Port::new("equal".to_string(), "Equal".to_string(), BitType::Boolean))
            .add_output(Port::new("different".to_string(), "Different".to_string(), BitType::Boolean))
    }
}

impl ModuleLogic for BitCompareModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        let input1 = context.get_input(&"input1".to_string());
        let input2 = context.get_input(&"input2".to_string());

        match (input1, input2) {
            (Some(i1), Some(i2)) => {
                let are_equal = i1.bits == i2.bits;
                
                let equal_bit = BitVector::new(vec![are_equal.into()], BitType::Boolean);
                let different_bit = BitVector::new(vec![(!are_equal).into()], BitType::Boolean);

                let mut outputs = HashMap::new();
                outputs.insert("equal".to_string(), equal_bit);
                outputs.insert("different".to_string(), different_bit);
                ExecutionResult::success(outputs)
            }
            _ => ExecutionResult::error("Both inputs are required".to_string()),
        }
    }

    fn description(&self) -> String {
        "Compares two bit vectors for equality".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Bit counter module - counts the number of set bits (1s)
#[derive(Debug, Clone)]
pub struct BitCountModule;

impl BitCountModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("bit_count".to_string(), "Bit Count".to_string(), "logical".to_string())
            .add_input(Port::new("input".to_string(), "Input".to_string(), BitType::Raw))
            .add_output(Port::new("count".to_string(), "Count".to_string(), BitType::DWord))
    }
}

impl ModuleLogic for BitCountModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(input) = context.get_input(&"input".to_string()) {
            let count = input.bits.iter()
                .filter(|&&bit| matches!(bit, Bit::One))
                .count() as u32;

            let count_bytes = count.to_le_bytes();
            let count_bits = BitVector::from_bytes(&count_bytes, BitType::DWord);

            let mut outputs = HashMap::new();
            outputs.insert("count".to_string(), count_bits);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Input is required".to_string())
        }
    }

    fn description(&self) -> String {
        "Counts the number of set bits (1s)".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

/// Parity check module - calculates even/odd parity
#[derive(Debug, Clone)]
pub struct ParityModule;

impl ParityModule {
    pub fn new() -> Self {
        Self
    }

    pub fn template() -> ModuleTemplate {
        ModuleTemplate::new("parity".to_string(), "Parity Check".to_string(), "logical".to_string())
            .add_input(Port::new("input".to_string(), "Input".to_string(), BitType::Raw))
            .add_output(Port::new("even_parity".to_string(), "Even Parity".to_string(), BitType::Boolean))
            .add_output(Port::new("odd_parity".to_string(), "Odd Parity".to_string(), BitType::Boolean))
    }
}

impl ModuleLogic for ParityModule {
    fn execute(&self, context: &ExecutionContext) -> ExecutionResult {
        if let Some(input) = context.get_input(&"input".to_string()) {
            let ones_count = input.bits.iter()
                .filter(|&&bit| matches!(bit, Bit::One))
                .count();

            let even_parity = (ones_count % 2) == 0;
            let odd_parity = !even_parity;

            let even_bit = BitVector::new(vec![even_parity.into()], BitType::Boolean);
            let odd_bit = BitVector::new(vec![odd_parity.into()], BitType::Boolean);

            let mut outputs = HashMap::new();
            outputs.insert("even_parity".to_string(), even_bit);
            outputs.insert("odd_parity".to_string(), odd_bit);
            ExecutionResult::success(outputs)
        } else {
            ExecutionResult::error("Input is required".to_string())
        }
    }

    fn description(&self) -> String {
        "Calculates even/odd parity of input bits".to_string()
    }

    fn clone_box(&self) -> Box<dyn ModuleLogic> {
        Box::new(self.clone())
    }
}

// Factory implementations
simple_factory!(AndModuleFactory, AndModule);
simple_factory!(OrModuleFactory, OrModule);
simple_factory!(NotModuleFactory, NotModule);
simple_factory!(XorModuleFactory, XorModule);
simple_factory!(NandModuleFactory, NandModule);
simple_factory!(NorModuleFactory, NorModule);
simple_factory!(BitCompareModuleFactory, BitCompareModule);
simple_factory!(BitCountModuleFactory, BitCountModule);
simple_factory!(ParityModuleFactory, ParityModule); 