use std::collections::BTreeMap;

use serde_json::value::Value as Json;

use crate::error::RenderError;
use crate::local_vars::LocalVars;

#[derive(Clone, Debug)]
pub enum BlockParamHolder {
    // a reference to certain context value
    Path(Vec<String>),
    // an actual value holder
    Value(Json),
}

impl BlockParamHolder {
    pub fn value(v: Json) -> BlockParamHolder {
        BlockParamHolder::Value(v)
    }

    pub fn path(r: Vec<String>) -> BlockParamHolder {
        BlockParamHolder::Path(r)
    }
}

/// A map holds block parameters. The parameter can be either a value or a reference
#[derive(Clone, Debug, Default)]
pub struct BlockParams<'reg> {
    data: BTreeMap<&'reg str, BlockParamHolder>,
}

impl<'reg> BlockParams<'reg> {
    /// Create a empty block parameter map.
    pub fn new() -> BlockParams<'reg> {
        BlockParams::default()
    }

    /// Add a path reference as the parameter. The `path` is a vector of path
    /// segments the relative to current block's base path.
    pub fn add_path(&mut self, k: &'reg str, path: Vec<String>) -> Result<(), RenderError> {
        self.data.insert(k, BlockParamHolder::path(path));
        Ok(())
    }

    /// Add a value as parameter.
    pub fn add_value(&mut self, k: &'reg str, v: Json) -> Result<(), RenderError> {
        self.data.insert(k, BlockParamHolder::value(v));
        Ok(())
    }

    /// Get a block parameter by its name.
    pub fn get(&self, k: &str) -> Option<&BlockParamHolder> {
        self.data.get(k)
    }
}

/// A data structure holds contextual data for current block scope.
#[derive(Debug, Clone, Default)]
pub struct BlockContext<'reg> {
    /// the base_path of current block scope
    base_path: Vec<String>,
    /// the base_value of current block scope, when the block is using a
    /// constant or derived value as block base
    base_value: Option<Json>,
    /// current block context variables
    block_params: BlockParams<'reg>,
    /// local variables in current context
    local_variables: LocalVars,
}

impl<'reg> BlockContext<'reg> {
    /// create a new `BlockContext` with default data
    pub fn new() -> BlockContext<'reg> {
        BlockContext::default()
    }

    /// set a local variable into current scope
    pub fn set_local_var(&mut self, name: &str, value: Json) {
        self.local_variables.put(name, value);
    }

    /// get a local variable from current scope
    pub fn get_local_var(&self, name: &str) -> Option<&Json> {
        self.local_variables.get(name)
    }

    /// borrow a reference to current scope's base path
    /// all paths inside this block will be relative to this path
    pub fn base_path(&self) -> &Vec<String> {
        &self.base_path
    }

    /// borrow a mutable reference to the base path
    pub fn base_path_mut(&mut self) -> &mut Vec<String> {
        &mut self.base_path
    }

    /// borrow the base value
    pub fn base_value(&self) -> Option<&Json> {
        self.base_value.as_ref()
    }

    /// set the base value
    pub fn set_base_value(&mut self, value: Json) {
        self.base_value = Some(value);
    }

    /// Get a block parameter from this block.
    /// Block parameters needed to be supported by the block helper.
    /// The typical syntax for block parameter is:
    ///
    /// ```skip
    /// {{#myblock param1 as |block_param1|}}
    ///    ...
    /// {{/myblock}}
    /// ```
    ///
    pub fn get_block_param(&self, block_param_name: &str) -> Option<&BlockParamHolder> {
        self.block_params.get(block_param_name)
    }

    /// Set a block parameter into this block.
    pub fn set_block_params(&mut self, block_params: BlockParams<'reg>) {
        self.block_params = block_params;
    }
}
