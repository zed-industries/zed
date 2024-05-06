use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use url::Url;
use util::codeberg;
use util::http::HttpClient;

use crate::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, Oid, ParsedGitRemote,
};

pub struct Codeberg;

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
        let avatar_url =
            codeberg::fetch_codeberg_commit_author(repo_owner, repo, &commit, &http_client)
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
