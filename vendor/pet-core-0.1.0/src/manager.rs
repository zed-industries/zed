// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum EnvManagerType {
    Conda,
    Mamba,
    Pipenv,
    Poetry,
    Pyenv,
}

impl Ord for EnvManagerType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        format!("{self:?}").cmp(&format!("{other:?}"))
    }
}
impl PartialOrd for EnvManagerType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct EnvManager {
    pub executable: PathBuf,
    pub version: Option<String>,
    pub tool: EnvManagerType,
}
impl Ord for EnvManager {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.executable.cmp(&other.executable) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.version.cmp(&other.version) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.tool.cmp(&other.tool)
    }
}

impl PartialOrd for EnvManager {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl EnvManager {
    pub fn new(executable_path: PathBuf, tool: EnvManagerType, version: Option<String>) -> Self {
        Self {
            executable: executable_path,
            version,
            tool,
        }
    }
}

impl std::fmt::Display for EnvManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Manager ({:?})", self.tool).unwrap_or_default();
        writeln!(
            f,
            "   Executable  : {}",
            self.executable.to_str().unwrap_or_default()
        )
        .unwrap_or_default();
        if let Some(version) = &self.version {
            writeln!(f, "   Version     : {version}").unwrap_or_default();
        }
        Ok(())
    }
}
