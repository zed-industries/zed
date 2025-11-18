use crate::{Oid, status::StatusCode};
use anyhow::{Context as _, Result};
use collections::HashMap;
use std::path::Path;

pub async fn get_messages(working_directory: &Path, shas: &[Oid]) -> Result<HashMap<Oid, String>> {
    if shas.is_empty() {
        return Ok(HashMap::default());
    }

    let output = if cfg!(windows) {
        // Windows has a maximum invocable command length, so we chunk the input.
        // Actual max is 32767, but we leave some room for the rest of the command as we aren't in precise control of what std might do here
        const MAX_CMD_LENGTH: usize = 30000;
        // 40 bytes of hash, 2 quotes and a separating space
        const SHA_LENGTH: usize = 40 + 2 + 1;
        const MAX_ENTRIES_PER_INVOCATION: usize = MAX_CMD_LENGTH / SHA_LENGTH;

        let mut result = vec![];
        for shas in shas.chunks(MAX_ENTRIES_PER_INVOCATION) {
            let partial = get_messages_impl(working_directory, shas).await?;
            result.extend(partial);
        }
        result
    } else {
        get_messages_impl(working_directory, shas).await?
    };

    Ok(shas
        .iter()
        .cloned()
        .zip(output)
        .collect::<HashMap<Oid, String>>())
}

async fn get_messages_impl(working_directory: &Path, shas: &[Oid]) -> Result<Vec<String>> {
    const MARKER: &str = "<MARKER>";
    let mut cmd = util::command::new_smol_command("git");
    cmd.current_dir(working_directory)
        .arg("show")
        .arg("-s")
        .arg(format!("--format=%B{}", MARKER))
        .args(shas.iter().map(ToString::to_string));
    let output = cmd
        .output()
        .await
        .with_context(|| format!("starting git blame process: {:?}", cmd))?;
    anyhow::ensure!(
        output.status.success(),
        "'git show' failed with error {:?}",
        output.status
    );
    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .split_terminator(MARKER)
        .map(|str| str.trim().replace("<", "&lt;").replace(">", "&gt;"))
        .collect::<Vec<_>>())
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
