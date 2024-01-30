use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct ErlangSettings;
