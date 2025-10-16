use std::str::FromStr;
use std::sync::Arc;

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::SharedString;
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request};
use serde::Deserialize;
use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    RemoteUrl,
};

#[derive(Debug, Deserialize)]
struct CommitDetails {
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    commit: Commit,
    author: Option<User>,
}

#[derive(Debug, Deserialize)]
struct Commit {
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    author: Author,
}

#[derive(Debug, Deserialize)]
struct Author {
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    name: String,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    email: String,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    date: String,
}

#[derive(Debug, Deserialize)]
struct User {
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub login: String,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
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
            .header("Content-Type", "application/json")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

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

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        let host = url.host_str()?;
        if host != "codeberg.org" {
            return None;
        }

        let mut path_segments = url.path_segments()?;
        let owner = path_segments.next()?;
        let repo = path_segments.next()?.trim_end_matches(".git");

        Some(ParsedGitRemote {
            owner: owner.into(),
            repo: repo.into(),
        })
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
        commit: SharedString,
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
    use git::repository::repo_path;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_remote_url_given_ssh_url() {
        let parsed_remote = Codeberg
            .parse_remote_url("git@codeberg.org:zed-industries/zed.git")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_https_url() {
        let parsed_remote = Codeberg
            .parse_remote_url("https://codeberg.org/zed-industries/zed.git")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            }
        );
    }

    #[test]
    fn test_build_codeberg_permalink() {
        let permalink = Codeberg.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://codeberg.org/zed-industries/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_with_single_line_selection() {
        let permalink = Codeberg.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(6..6),
            ),
        );

        let expected_url = "https://codeberg.org/zed-industries/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_codeberg_permalink_with_multi_line_selection() {
        let permalink = Codeberg.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(23..47),
            ),
        );

        let expected_url = "https://codeberg.org/zed-industries/zed/src/commit/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
