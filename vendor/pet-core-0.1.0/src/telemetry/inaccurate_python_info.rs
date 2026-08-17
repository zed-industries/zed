// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};

use crate::python_environment::PythonEnvironmentKind;

/// Information about an environment that was discovered to be inaccurate.
/// If the discovered information is None, then it means that the information was not found.
/// And we will not report that as an inaccuracy.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Copy)]
pub struct InaccuratePythonEnvironmentInfo {
    /// Python Env kind
    pub kind: Option<PythonEnvironmentKind>,
    /// Whether the actual exe is not what we expected.
    pub invalid_executable: Option<bool>,
    /// Whether the actual exe was not even in the list of symlinks that we expected.
    pub executable_not_in_symlinks: Option<bool>,
    /// Whether the prefix is not what we expected.
    pub invalid_prefix: Option<bool>,
    /// Whether the version is not what we expected.
    pub invalid_version: Option<bool>,
    /// Whether the architecture is not what we expected.
    pub invalid_arch: Option<bool>,
}

impl std::fmt::Display for InaccuratePythonEnvironmentInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Environment {:?} incorrectly identified", self.kind).unwrap_or_default();
        if self.invalid_executable.unwrap_or_default() {
            writeln!(f, "   Executable is incorrect").unwrap_or_default();
        }
        if self.executable_not_in_symlinks.unwrap_or_default() {
            writeln!(f, "   Executable is not in the list of symlinks").unwrap_or_default();
        }
        if self.invalid_prefix.unwrap_or_default() {
            writeln!(f, "   Prefix is incorrect").unwrap_or_default();
        }
        if self.invalid_version.unwrap_or_default() {
            writeln!(f, "   Version is incorrect").unwrap_or_default();
        }
        if self.invalid_arch.unwrap_or_default() {
            writeln!(f, "   Architecture is incorrect").unwrap_or_default();
        }
        Ok(())
    }
}
