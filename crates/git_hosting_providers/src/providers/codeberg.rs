use std::sync::Arc;

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use futures::AsyncReadExt;
use http_client::HttpClient;
use isahc::config::Configurable;
use isahc::{AsyncBody, Request};
use serde::Deserialize;
use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, Oid, ParsedGitRemote,
};

#[derive(Debug, Deserialize)]
struct CommitDetails {
    commit: Commit,
    author: Option<User>,
}

#[derive(Debug, Deserialize)]
struct Commit {
    author: Author,
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    email: String,
    date: String,
}

#[derive(Debug, Deserialize)]
struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
}

pub struct Codeberg;

impl Codeberg {
    async fn fetch_codeberg_commit_author(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Option<User>> {
        let url =
            format!("https://codeberg.org/api/v1/repos/{repo_owner}/{repo}/git/commits/{commit}");

        let mut request = Request::get(&url)
            .redirect_policy(isahc::config::RedirectPolicy::Follow)
            .header("Content-Type", "application/json");

        if let Ok(codeberg_token) = std::env::var("CODEBERG_TOKEN") {
            request = request.header("Authorization", format!("Bearer {}", codeberg_token));
        }

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching Codeberg commit details at {:?}", url))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        if response.status().is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status().as_u16()
            );
        }

        let body_str = std::str::from_utf8(&body)?;

        serde_json::from_str::<CommitDetails>(body_str)
            .map(|commit| commit.author)
            .context("failed to deserialize Codeberg commit details")
    }
}

#[async_trait]
impl GitHostingProvider for Codeberg {
    fn name(&self) -> String {
        "Codeberg".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://codeberg.org").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        true
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-L{end_line}")
    }

    fn parse_remote_url<'a>(&self, url: &'a str) -> Option<ParsedGitRemote<'a>> {
        if url.starts_with("git@codeberg.org:") || url.starts_with("https://codeberg.org/") {
            let repo_with_owner = url
                .trim_start_matches("git@codeberg.org:")
                .trim_start_matches("https://codeberg.org/")
                .trim_end_matches(".git");

            let (owner, repo) = repo_with_owner.split_once('/')?;

            return Some(ParsedGitRemote { owner, repo });
        }

        None
    }

    fn build_commit_permalink(
        &self,
        remote: &ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url {
        let BuildCommitPermalinkParams { sha } = params;
        let ParsedGitRemote { owner, repo } = remote;

        self.base_url()
            .join(&format!("{owner}/{repo}/commit/{sha}"))
            .unwrap()
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote { owner, repo } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
        } = params;

        let mut permalink = self
            .base_url()
            .join(&format!("{owner}/{repo}/src/commit/{sha}/{path}"))
            .unwrap();
        permalink.set_fragment(
            selection
                .map(|selection| self.line_fragment(&selection))
                .as_deref(),
        );
        permalink
    }

    async fn commit_author_avatar_url(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: Oid,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        let commit = commit.to_string();
        let avatar_url = self
            .fetch_codeberg_commit_author(repo_owner, repo, &commit, &http_client)
            .await?
            .map(|author| Url::parse(&author.avatar_url))
            .transpose()?;
        Ok(avatar_url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_codeberg_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Codeberg.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_ssh_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Codeberg.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_ssh_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Codeberg.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_https_url() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Codeberg.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/zed/src/main.rs",
                selection: None,
            },
        );

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_https_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Codeberg.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/zed/src/main.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_from_https_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Codeberg.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/zed/src/main.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://codeberg.org/rajveermalviya/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/zed/src/main.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
