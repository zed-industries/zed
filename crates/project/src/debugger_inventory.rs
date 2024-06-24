use std::collections::VecDeque;

use dap::configh_templates::DebuggerConfigTemplate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use task::static_source::TrackedFile;

pub struct DebuggerInventory {
    sources: Vec<SourceInInventory>,
}

struct SourceInInventory {
    source: StaticSource,
    // kind: TaskSourceKind, TODO: Change this and impl Debugger Source Kind (Might not be needed)
}

pub struct StaticSource {
    tasks: TrackedFile<DebuggerConfigTemplates>,
}

/// A group of Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DebuggerConfigTemplates(pub Vec<DebuggerConfigTemplate>);
