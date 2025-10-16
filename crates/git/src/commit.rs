use crate::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, Oid, SHORT_SHA_LENGTH,
    parse_git_remote_url, status::StatusCode,
};
use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::SharedString;
use std::{hash::Hash, path::Path, sync::Arc};
use time::OffsetDateTime;

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

#[derive(Clone, Debug, Default)]
pub struct ParsedCommitMessage {
    pub message: SharedString,
    pub permalink: Option<url::Url>,
    pub pull_request: Option<crate::hosting_provider::PullRequest>,
    pub remote: Option<GitRemote>,
}

impl ParsedCommitMessage {
    pub fn new(
        sha: String,
        message: String,
        remote_url: Option<&str>,
        provider_registry: Option<Arc<GitHostingProviderRegistry>>,
    ) -> Self {
        if let Some((hosting_provider, remote)) = provider_registry
            .and_then(|reg| remote_url.and_then(|url| parse_git_remote_url(reg, url)))
        {
            let pull_request = hosting_provider.extract_pull_request(&remote, &message);
            Self {
                message: message.into(),
                permalink: Some(
                    hosting_provider
                        .build_commit_permalink(&remote, BuildCommitPermalinkParams { sha: &sha }),
                ),
                pull_request,
                remote: Some(GitRemote {
                    host: hosting_provider,
                    owner: remote.owner.into(),
                    repo: remote.repo.into(),
                }),
            }
        } else {
            Self::from_message(message)
        }
    }
    pub fn from_message(message: String) -> Self {
        Self {
            message: message.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommitSummary {
    pub sha: SharedString,
    pub subject: SharedString,
    /// This is a unix timestamp
    pub commit_timestamp: i64,
    pub author_name: SharedString,
    pub has_parent: bool,
}

impl PartialEq for CommitSummary {
    fn eq(&self, other: &Self) -> bool {
        self.sha == other.sha
    }
}
impl Eq for CommitSummary {}
impl Hash for CommitSummary {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sha.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CommitDetails {
    pub sha: SharedString,
    pub author_name: SharedString,
    pub author_email: SharedString,
    pub commit_time: OffsetDateTime,
    pub message: Option<ParsedCommitMessage>,
}

impl Default for CommitDetails {
    fn default() -> Self {
        Self {
            sha: Default::default(),
            author_name: Default::default(),
            author_email: Default::default(),
            commit_time: OffsetDateTime::now_utc(),
            message: Default::default(),
        }
    }
}

impl PartialEq for CommitDetails {
    fn eq(&self, other: &Self) -> bool {
        self.sha == other.sha
    }
}

impl Eq for CommitDetails {}

impl CommitDetails {
    pub fn short_sha(&self) -> SharedString {
        self.sha[..SHORT_SHA_LENGTH].to_string().into()
    }
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
