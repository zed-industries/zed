mod scripting_session;

pub use scripting_session::*;

use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScriptingToolInput {
    pub lua_script: String,
}

pub struct ScriptingTool;

impl ScriptingTool {
    pub const NAME: &str = "lua-interpreter";

    pub const DESCRIPTION: &str = include_str!("scripting_tool_description.md");

    pub fn input_schema() -> serde_json::Value {
        let schema = schemars::schema_for!(ScriptingToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    pub fn deserialize_input(
        input: serde_json::Value,
    ) -> Result<ScriptingToolInput, serde_json::Error> {
        serde_json::from_value(input)
    }
}
