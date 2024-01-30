use schemars::JsonSchema;
use serde_derive::{Serialize,Deserialize};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct ErlangSettings;
