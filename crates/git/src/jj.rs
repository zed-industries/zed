use anyhow::{Context as _, Result};

use std::path::Path;
use util::command::new_command;

pub(crate) async fn visible_heads(work_directory: &Path) -> Result<Option<Vec<String>>> {
    if !work_directory.join(".jj").is_dir() || !jj_binary_is_available().await {
        return Ok(None);
    }

    let mut command = new_command("jj");
    command
        .arg("--repository")
        .arg(work_directory)
        .arg("--ignore-working-copy")
        .args(["--color", "never"])
        .arg("--quiet")
        .arg("--no-pager")
        .arg("log")
        .args(["-r", "visible_heads()"])
        .arg("--no-graph")
        .args(["-T", r#"commit_id ++ "\n""#]);

    let output = command.output().await?;
    anyhow::ensure!(
        output.status.success(),
        "jj command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let head_revisions = String::from_utf8(output.stdout)?
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    Ok(Some(head_revisions))
}

async fn jj_binary_is_available() -> bool {
    new_command("jj")
        .arg("--version")
        .output()
        .await
        .is_ok_and(|output| output.status.success())
}
