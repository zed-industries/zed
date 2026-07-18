use std::{str::FromStr, sync::Arc};

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

pub struct Gitee;

#[derive(Debug, Deserialize)]
struct CommitDetails {
    author: Option<Author>,
}

#[derive(Debug, Deserialize)]
struct Author {
    avatar_url: String,
}

impl Gitee {
    async fn fetch_gitee_commit_author(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Option<Author>> {
        let url = format!("https://gitee.com/api/v5/repos/{repo_owner}/{repo}/commits/{commit}");

        let request = Request::get(&url)
            .header("Content-Type", "application/json")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching Gitee commit details at {:?}", url))?;

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
            .context("failed to deserialize Gitee commit details")
    }
}

#[async_trait]
impl GitHostingProvider for Gitee {
    fn name(&self) -> String {
        "Gitee".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://gitee.com").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        true
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-{end_line}")
    }

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        let host = url.host_str()?;
        if host != "gitee.com" {
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
            .join(&format!("{owner}/{repo}/blob/{sha}/{path}"))
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
        _author_email: Option<SharedString>,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        let commit = commit.to_string();
        let avatar_url = self
            .fetch_gitee_commit_author(repo_owner, repo, &commit, &http_client)
            .await?
            .map(|author| -> Result<Url, url::ParseError> {
                let mut url = Url::parse(&author.avatar_url)?;
                url.set_query(Some("width=128"));
                Ok(url)
            })
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
        let parsed_remote = Gitee
            .parse_remote_url("git@gitee.com:zed-industries/zed.git")
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
        let parsed_remote = Gitee
            .parse_remote_url("https://gitee.com/zed-industries/zed.git")
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
    fn test_build_gitee_permalink() {
        let permalink = Gitee.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://gitee.com/zed-industries/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_with_single_line_selection() {
        let permalink = Gitee.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(6..6),
            ),
        );

        let expected_url = "https://gitee.com/zed-industries/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_with_multi_line_selection() {
        let permalink = Gitee.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(23..47),
            ),
        );

        let expected_url = "https://gitee.com/zed-industries/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
