use crate::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, Oid, parse_git_remote_url,
    repository::GitBinary, status::StatusCode,
};
use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use gpui::SharedString;
use std::{str::FromStr, sync::Arc};

#[derive(Clone, Debug, Default)]
pub struct ParsedCommitMessage {
    pub message: SharedString,
    pub permalink: Option<url::Url>,
    pub pull_request: Option<crate::hosting_provider::PullRequest>,
    pub remote: Option<GitRemote>,
}

impl ParsedCommitMessage {
    pub fn parse(
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
            Self {
                message: message.into(),
                ..Default::default()
            }
        }
    }
}

pub(crate) async fn get_messages(git: &GitBinary, shas: &[Oid]) -> Result<HashMap<Oid, String>> {
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
            let partial = get_messages_impl(git, shas).await?;
            result.extend(partial);
        }
        result
    } else {
        get_messages_impl(git, shas).await?
    };

    Ok(shas
        .iter()
        .cloned()
        .zip(output)
        .collect::<HashMap<Oid, String>>())
}

pub(crate) async fn get_tag_names(
    git: &GitBinary,
    shas: &[Oid],
) -> Result<HashMap<Oid, Vec<String>>> {
    if shas.is_empty() {
        return Ok(HashMap::default());
    }

    let output = git
        .build_command(&[
            "for-each-ref",
            "refs/tags",
            "--sort=-creatordate",
            "--format=%(objectname)%00%(*objectname)%00%(refname:short)",
        ])
        .output()
        .await
        .context("starting git for-each-ref process")?;
    anyhow::ensure!(
        output.status.success(),
        "'git for-each-ref' failed with error {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(parse_tag_names(
        &String::from_utf8_lossy(&output.stdout),
        shas,
    ))
}

fn parse_tag_names(output: &str, shas: &[Oid]) -> HashMap<Oid, Vec<String>> {
    let shas = shas.iter().copied().collect::<HashSet<_>>();
    let mut result = HashMap::<Oid, Vec<String>>::default();

    for line in output.lines() {
        let mut fields = line.split('\0');
        let object_sha = fields.next();
        let peeled_sha = fields.next().filter(|sha| !sha.is_empty());
        let Some(sha) = peeled_sha
            .or(object_sha)
            .and_then(|sha| Oid::from_str(sha).ok())
        else {
            continue;
        };
        let Some(tag_name) = fields.next().filter(|tag_name| !tag_name.is_empty()) else {
            continue;
        };
        result.entry(sha).or_default().push(tag_name.to_string());
    }

    result.retain(|sha, _| shas.contains(sha));
    result
}

async fn get_messages_impl(git: &GitBinary, shas: &[Oid]) -> Result<Vec<String>> {
    const MARKER: &str = "<MARKER>";
    let output = git
        .build_command(&["show"])
        .arg("-s")
        .arg(format!("--format=%B{}", MARKER))
        .args(shas.iter().map(ToString::to_string))
        .output()
        .await
        .context("starting git show process")?;
    anyhow::ensure!(
        output.status.success(),
        "'git show' failed with error {:?}",
        String::from_utf8_lossy(&output.stderr)
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
                "T" => StatusCode::TypeChanged,
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
            "T\x00LICENSE-GPL\x00",
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
                ("LICENSE-GPL", StatusCode::TypeChanged),
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

    #[test]
    fn test_parse_tag_names_for_lightweight_and_annotated_tags() -> Result<()> {
        let tagged_commit = Oid::from_str("1111111111111111111111111111111111111111")?;
        let tag_object = Oid::from_str("2222222222222222222222222222222222222222")?;
        let other_commit = Oid::from_str("3333333333333333333333333333333333333333")?;
        let output = format!(
            "{tagged_commit}\0\0v1.0.0\n\
             {tag_object}\0{tagged_commit}\0v1.1.0\n\
             {other_commit}\0\0ignored\n"
        );

        let parsed = parse_tag_names(&output, &[tagged_commit]);

        assert_eq!(
            parsed,
            HashMap::from_iter([(
                tagged_commit,
                vec![String::from("v1.0.0"), String::from("v1.1.0")]
            )])
        );
        Ok(())
    }
}
