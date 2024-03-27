use crate::Oid;
use anyhow::{anyhow, Result};
use collections::HashMap;
use std::path::Path;
use std::process::Command;

pub fn get_messages(working_directory: &Path, shas: &[Oid]) -> Result<HashMap<Oid, String>> {
    const MARKER: &'static str = "<MARKER>";

    let output = Command::new("git")
        .current_dir(working_directory)
        .arg("show")
        .arg("-s")
        .arg(format!("--format=%B{}", MARKER))
        .args(shas.iter().map(ToString::to_string))
        .output()
        .map_err(|e| anyhow!("Failed to start git blame process: {}", e))?;

    anyhow::ensure!(
        output.status.success(),
        "'git show' failed with error {:?}",
        output.status
    );

    let messages = String::from_utf8_lossy(&output.stdout)
        .trim()
        .split_terminator(MARKER)
        .map(|str| String::from(str.trim()))
        .collect::<Vec<_>>();

    Ok(shas
        .iter()
        .cloned()
        .zip(messages.into_iter())
        .collect::<HashMap<Oid, String>>())
}
