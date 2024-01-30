use anyhow::{anyhow, Result};
use language::Point;
use text::Selection;
use url::Url;

enum GitHostingProvider {
    Github,
}

impl GitHostingProvider {
    fn base_url(&self) -> Url {
        let base_url = match self {
            GitHostingProvider::Github => "https://github.com",
        };

        Url::parse(&base_url).unwrap()
    }
}

pub struct BuildPermalinkParams<'a> {
    pub remote_url: &'a str,
    pub sha: &'a str,
    pub path: &'a str,
    pub selection: Option<&'a Selection<Point>>,
}

pub fn build_permalink(params: BuildPermalinkParams) -> Result<Url> {
    let BuildPermalinkParams {
        remote_url,
        sha,
        path,
        selection,
    } = params;

    let ParsedGitRemote {
        provider,
        owner,
        repo,
    } = parse_git_remote_url(remote_url)
        .ok_or_else(|| anyhow!("failed to parse Git remote URL"))?;

    let line_selector = {
        selection.map(|selection| {
            if selection.start.row == selection.end.row {
                return format!("L{}", selection.start.row + 1);
            } else {
                format!("L{}-L{}", selection.start.row + 1, selection.end.row + 1)
            }
        })
    };

    let mut permalink = provider
        .base_url()
        .join(&format!("{owner}/{repo}/blob/{sha}/{path}"))
        .unwrap();
    permalink.set_fragment(line_selector.as_deref());

    Ok(permalink)
}

struct ParsedGitRemote<'a> {
    pub provider: GitHostingProvider,
    pub owner: &'a str,
    pub repo: &'a str,
}

fn parse_git_remote_url(url: &str) -> Option<ParsedGitRemote> {
    if url.starts_with("git@github.com:") {
        let repo_with_owner = url
            .trim_start_matches("git@github.com:")
            .trim_end_matches(".git");

        let (owner, repo) = repo_with_owner.split_once("/")?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Github,
            owner,
            repo,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_github_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@github.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
