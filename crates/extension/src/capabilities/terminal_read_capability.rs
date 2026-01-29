use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminalReadCapability;

impl TerminalReadCapability {
    pub fn allows(&self) -> bool {
        true
    }
}
