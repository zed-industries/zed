use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Runnables {
    pub version: String,
    pub tasks: Vec<Definition>,
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
    pub options: Options,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Options {
    pub env: BTreeMap<String, String>,
    pub cwd: Option<PathBuf>,
}
