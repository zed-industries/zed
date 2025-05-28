pub mod basic;
pub mod arithmetic;
pub mod logical;
pub mod conversion;
pub mod io;
pub mod control;

pub use basic::*;
pub use arithmetic::*;
pub use logical::*;
pub use conversion::*;
pub use io::*;
pub use control::*;

use workflow_core::*;
use std::collections::HashMap;

/// Registry for all built-in modules
#[derive(Default)]
pub struct ModuleRegistry {
    templates: HashMap<String, ModuleTemplate>,
    factories: HashMap<String, Box<dyn ModuleFactory>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();
        registry.register_built_in_modules();
        registry
    }

    fn register_built_in_modules(&mut self) {
        // Basic modules
        self.register_module("input".to_string(), InputModule::template(), Box::new(InputModuleFactory));
        self.register_module("output".to_string(), OutputModule::template(), Box::new(OutputModuleFactory));
        self.register_module("constant".to_string(), ConstantModule::template(), Box::new(ConstantModuleFactory));
        self.register_module("bit_selector".to_string(), BitSelectorModule::template(), Box::new(BitSelectorModuleFactory));
        self.register_module("bit_concat".to_string(), BitConcatModule::template(), Box::new(BitConcatModuleFactory));
        self.register_module("bit_shift".to_string(), BitShiftModule::template(), Box::new(BitShiftModuleFactory));
        
        // Arithmetic modules
        self.register_module("add".to_string(), AddModule::template(), Box::new(AddModuleFactory));
        self.register_module("subtract".to_string(), SubtractModule::template(), Box::new(SubtractModuleFactory));
        self.register_module("multiply".to_string(), MultiplyModule::template(), Box::new(MultiplyModuleFactory));
        self.register_module("divide".to_string(), DivideModule::template(), Box::new(DivideModuleFactory));
        
        // Logical modules
        self.register_module("and".to_string(), AndModule::template(), Box::new(AndModuleFactory));
        self.register_module("or".to_string(), OrModule::template(), Box::new(OrModuleFactory));
        self.register_module("not".to_string(), NotModule::template(), Box::new(NotModuleFactory));
        self.register_module("xor".to_string(), XorModule::template(), Box::new(XorModuleFactory));
        self.register_module("nand".to_string(), NandModule::template(), Box::new(NandModuleFactory));
        self.register_module("nor".to_string(), NorModule::template(), Box::new(NorModuleFactory));
        self.register_module("bit_compare".to_string(), BitCompareModule::template(), Box::new(BitCompareModuleFactory));
        self.register_module("bit_count".to_string(), BitCountModule::template(), Box::new(BitCountModuleFactory));
        self.register_module("parity".to_string(), ParityModule::template(), Box::new(ParityModuleFactory));
        
        // Conversion modules
        self.register_module("bit_to_byte".to_string(), BitToByteModule::template(), Box::new(BitToByteModuleFactory));
        self.register_module("byte_to_word".to_string(), ByteToWordModule::template(), Box::new(ByteToWordModuleFactory));
        self.register_module("word_to_dword".to_string(), WordToDWordModule::template(), Box::new(WordToDWordModuleFactory));
        self.register_module("split_byte".to_string(), SplitByteModule::template(), Box::new(SplitByteModuleFactory));
        
        // I/O modules
        self.register_module("console_output".to_string(), ConsoleOutputModule::template(), Box::new(ConsoleOutputModuleFactory));
        self.register_module("file_read".to_string(), FileReadModule::template(), Box::new(FileReadModuleFactory));
        self.register_module("file_write".to_string(), FileWriteModule::template(), Box::new(FileWriteModuleFactory));
        self.register_module("network_send".to_string(), NetworkSendModule::template(), Box::new(NetworkSendModuleFactory));
        
        // Control modules
        self.register_module("if".to_string(), IfModule::template(), Box::new(IfModuleFactory));
        self.register_module("switch".to_string(), SwitchModule::template(), Box::new(SwitchModuleFactory));
        self.register_module("loop".to_string(), LoopModule::template(), Box::new(LoopModuleFactory));
    }

    pub fn register_module(&mut self, id: String, template: ModuleTemplate, factory: Box<dyn ModuleFactory>) {
        self.templates.insert(id.clone(), template);
        self.factories.insert(id, factory);
    }

    pub fn get_template(&self, id: &str) -> Option<&ModuleTemplate> {
        self.templates.get(id)
    }

    pub fn create_module(&self, id: &str, parameters: &HashMap<String, serde_json::Value>) -> Option<ModuleInstance> {
        let template = self.templates.get(id)?.clone();
        let logic = self.factories.get(id)?.create_logic(parameters)?;
        Some(template.instantiate(logic))
    }

    pub fn list_modules(&self) -> Vec<&ModuleTemplate> {
        self.templates.values().collect()
    }

    pub fn list_modules_by_category(&self, category: &str) -> Vec<&ModuleTemplate> {
        self.templates.values()
            .filter(|template| template.category.as_str() == category)
            .collect()
    }

    pub fn search_modules(&self, query: &str) -> Vec<&ModuleTemplate> {
        let query = query.to_lowercase();
        self.templates.values()
            .filter(|template| {
                template.name.to_lowercase().contains(&query) ||
                template.description.to_lowercase().contains(&query) ||
                template.id.to_lowercase().contains(&query)
            })
            .collect()
    }
}

/// Factory trait for creating module logic instances
pub trait ModuleFactory: Send + Sync {
    fn create_logic(&self, parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>>;
}

/// Helper macro for creating simple module factories
#[macro_export]
macro_rules! simple_factory {
    ($factory_name:ident, $module_type:ty) => {
        pub struct $factory_name;
        
        impl ModuleFactory for $factory_name {
            fn create_logic(&self, _parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
                Some(Box::new(<$module_type>::new()))
            }
        }
    };
}

/// Helper macro for creating parameterized module factories
#[macro_export]
macro_rules! parameterized_factory {
    ($factory_name:ident, $module_type:ty, $param_type:ty) => {
        pub struct $factory_name;
        
        impl ModuleFactory for $factory_name {
            fn create_logic(&self, parameters: &HashMap<String, serde_json::Value>) -> Option<Box<dyn ModuleLogic>> {
                let params: $param_type = serde_json::from_value(
                    parameters.get("config")?.clone()
                ).ok()?;
                Some(Box::new(<$module_type>::new(params)))
            }
        }
    };
} 