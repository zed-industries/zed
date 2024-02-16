//! Contains definitions for runnables that can be deserialized from e.g. JSON.
use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunnableProvider {
    pub version: String,
    pub runnables: Vec<Definition>,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reveal {
    #[default]
    Always,
    Never,
    Silent,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Presentation {
    pub reveal: Reveal,
    pub focus: bool,
    pub clear: bool,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Definition {
    pub label: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub presentation: Presentation,
    #[serde(default)]
    pub options: Options,
    #[serde(default)]
    pub spawn_in_new_terminal: bool,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Options {
    pub env: BTreeMap<String, String>,
    pub cwd: Option<PathBuf>,
}
