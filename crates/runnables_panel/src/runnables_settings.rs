use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunnablesDockPosition {
    Left,
    #[default]
    Right,
}

#[derive(Serialize, Deserialize)]
pub struct RunnablesSettings {
    dock: RunnablesDockPosition,
    default_width: f32,
}
