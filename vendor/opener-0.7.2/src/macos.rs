use crate::OpenError;
use std::ffi::OsStr;
use std::process::{Command, Stdio};

pub(crate) fn open(path: &OsStr) -> Result<(), OpenError> {
    let mut open = Command::new("open")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(OpenError::Io)?;

    crate::wait_child(&mut open, "open")
}

#[cfg(feature = "reveal")]
pub(crate) fn reveal(path: &std::path::Path) -> Result<(), OpenError> {
    let mut open = Command::new("open")
        .arg("-R")
        .arg("--")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(OpenError::Io)?;

    crate::wait_child(&mut open, "open")
}
