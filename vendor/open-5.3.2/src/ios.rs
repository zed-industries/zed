use std::{ffi::OsStr, process::Command};

pub fn commands<T: AsRef<OsStr>>(path: T) -> Vec<Command> {
    let mut cmd = Command::new("uiopen");
    cmd.arg("--url").arg(path.as_ref());
    vec![cmd]
}

pub fn with_command<T: AsRef<OsStr>>(path: T, app: impl Into<String>) -> Command {
    let mut cmd = Command::new("uiopen");
    cmd.arg("--url")
        .arg(path.as_ref())
        .arg("--bundleid")
        .arg(app.into());
    cmd
}
