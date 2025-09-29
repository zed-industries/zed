use crate::{Oid, status::StatusCode};
use anyhow::{Context as _, Result};
use collections::HashMap;
use std::path::Path;

pub async fn get_messages(working_directory: &Path, shas: &[Oid]) -> Result<HashMap<Oid, String>> {
    if shas.is_empty() {
        return Ok(HashMap::default());
    }

    const MARKER: &str = "<MARKER>";

    let output = util::command::new_smol_command("git")
        .current_dir(working_directory)
        .arg("show")
        .arg("-s")
        .arg(format!("--format=%B{}", MARKER))
        .args(shas.iter().map(ToString::to_string))
        .output()
        .await
        .context("starting git blame process")?;

    anyhow::ensure!(
        output.status.success(),
        "'git show' failed with error {:?}",
        output.status
    );

    Ok(shas
        .iter()
        .cloned()
        .zip(
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .split_terminator(MARKER)
                .map(|str| str.trim().replace("<", "&lt;").replace(">", "&gt;")),
        )
        .collect::<HashMap<Oid, String>>())
}

/// Parse the output of `git diff --name-status -z`
pub fn parse_git_diff_name_status(content: &str) -> impl Iterator<Item = (&str, StatusCode)> {
    let mut parts = content.split('\0');
    std::iter::from_fn(move || {
        loop {
            let status_str = parts.next()?;
            let path = parts.next()?;
            let status = match status_str {
                "M" => StatusCode::Modified,
                "A" => StatusCode::Added,
                "D" => StatusCode::Deleted,
                _ => continue,
            };
            return Some((path, status));
        }
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_git_diff_name_status() {
        let input = concat!(
            "M\x00Cargo.lock\x00",
            "M\x00crates/project/Cargo.toml\x00",
            "M\x00crates/project/src/buffer_store.rs\x00",
            "D\x00crates/project/src/git.rs\x00",
            "A\x00crates/project/src/git_store.rs\x00",
            "A\x00crates/project/src/git_store/git_traversal.rs\x00",
            "M\x00crates/project/src/project.rs\x00",
            "M\x00crates/project/src/worktree_store.rs\x00",
            "M\x00crates/project_panel/src/project_panel.rs\x00",
        );

        let output = parse_git_diff_name_status(input).collect::<Vec<_>>();
        assert_eq!(
            output,
            &[
                ("Cargo.lock", StatusCode::Modified),
                ("crates/project/Cargo.toml", StatusCode::Modified),
                ("crates/project/src/buffer_store.rs", StatusCode::Modified),
                ("crates/project/src/git.rs", StatusCode::Deleted),
                ("crates/project/src/git_store.rs", StatusCode::Added),
                (
                    "crates/project/src/git_store/git_traversal.rs",
                    StatusCode::Added,
                ),
                ("crates/project/src/project.rs", StatusCode::Modified),
                ("crates/project/src/worktree_store.rs", StatusCode::Modified),
                (
                    "crates/project_panel/src/project_panel.rs",
                    StatusCode::Modified
                ),
            ]
        );
    }
}
