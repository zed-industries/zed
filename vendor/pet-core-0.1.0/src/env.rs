// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::path::PathBuf;

use pet_fs::path::norm_case;

use crate::pyvenv_cfg::PyVenvCfg;

#[derive(Debug)]
pub struct PythonEnv {
    /// Executable of the Python environment.
    ///
    /// Can be `/usr/bin/python` or `/opt/homebrew/bin/python3.12`.
    /// Or even the fully (resolved &) qualified path to the python executable such as `/opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12`.
    ///
    /// Note: This can be a symlink as well.
    pub executable: PathBuf,
    /// Environment prefix
    pub prefix: Option<PathBuf>,
    /// Version of the Python environment.
    pub version: Option<String>,
    /// Possible symlink (or known alternative link).
    /// For instance:
    ///
    /// If `executable` is `/opt/homebrew/Cellar/python@3.12/3.12.3/Frameworks/Python.framework/Versions/3.12/bin/python3.12`,
    /// then `symlink`` can be `/opt/homebrew/bin/python3.12` (or vice versa).
    pub symlinks: Option<Vec<PathBuf>>,
}

impl PythonEnv {
    pub fn new(executable: PathBuf, prefix: Option<PathBuf>, version: Option<String>) -> Self {
        let mut prefix = prefix.clone();
        if let Some(value) = prefix {
            prefix = norm_case(value).into();
        }
        // if the prefix is not defined, try to get this.
        // For instance, if the file is bin/python or Scripts/python
        // And we have a pyvenv.cfg file in the parent directory, then we can get the prefix.
        if prefix.is_none() {
            let mut exe = executable.clone();
            exe.pop();
            if exe.ends_with("Scripts") || exe.ends_with("bin") {
                exe.pop();
                if PyVenvCfg::find(&exe).is_some() {
                    prefix = Some(exe);
                }
            }
        }
        Self {
            executable: norm_case(executable),
            prefix,
            version,
            symlinks: None,
        }
    }
}
