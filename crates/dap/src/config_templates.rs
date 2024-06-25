use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{Deserialize, Serialize};

/// A template definition of a Zed task to run.
/// May use the [`VariableName`] to get the corresponding substitutions into its fields.
///
/// Template itself is not ready to spawn a task, it needs to be resolved with a [`TaskContext`] first, that
/// contains all relevant Zed state in task variables.
/// A single template may produce different tasks (or none) for different contexts.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DebuggerConfigTemplate {
    pub _type: String,
    pub request: String,
    #[serde(default)]
    pub args: Vec<String>,
}

impl DebuggerConfigTemplate {
    /// Generates JSON schema of Tasks JSON template format.
    pub fn generate_json_schema() -> serde_json_lenient::Value {
        let schema = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator()
            .into_root_schema_for::<Self>();

        serde_json_lenient::to_value(schema).unwrap()
    }
}

/// [`VsCodeTaskFile`] is a superset of Code's task definition format.
#[derive(Debug, Deserialize, PartialEq)]
pub struct ZedDebugConfigFile {
    debugger_configs: Vec<DebuggerConfigTemplate>,
}

// impl TryFrom<ZedDebugConfigFile> for DebuggerConfigTemplate {
//     type Error = anyhow::Error;

//     fn try_from(value: ZedDebugConfigFile) -> Result<Self, Self::Error> {

//         let templates = value
//             .
//         Ok(Self(templates))
//     }
// }
