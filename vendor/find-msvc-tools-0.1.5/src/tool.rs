use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

/// `Tool` found by `find-msvc-tools`
#[derive(Clone, Debug)]
pub struct Tool {
    pub(crate) tool: PathBuf,
    pub(crate) is_clang_cl: bool,
    pub(crate) env: Vec<(OsString, OsString)>,
}

impl Tool {
    /// Converts this compiler into a `Command` that's ready to be run.
    ///
    /// This is useful for when the compiler needs to be executed and the
    /// command returned will already have the initial arguments and environment
    /// variables configured.
    pub fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.tool);

        for (k, v) in self.env.iter() {
            cmd.env(k, v);
        }

        cmd
    }

    /// Check is the tool clang-cl related
    pub fn is_clang_cl(&self) -> bool {
        self.is_clang_cl
    }

    /// Get path to the tool
    pub fn path(&self) -> &Path {
        &self.tool
    }

    /// Get environment variables for the tools
    pub fn env(&self) -> impl IntoIterator<Item = &(OsString, OsString)> {
        &self.env
    }
}
