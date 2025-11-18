use std::str::FromStr;
use std::sync::{Arc, LazyLock};

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use futures::AsyncReadExt;
use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    PullRequest, RemoteUrl,
};
use gpui::SharedString;
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request};
use regex::Regex;
use serde::Deserialize;
use url::Url;

const CHROMIUM_REVIEW_URL: &str = "https://chromium-review.googlesource.com";

/// Parses Gerrit URLs like
/// https://chromium-review.googlesource.com/c/chromium/src/+/3310961.
fn pull_request_regex() -> &'static Regex {
    static PULL_REQUEST_NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(&format!(
            r#"Reviewed-on: ({CHROMIUM_REVIEW_URL}/c/(.*)/\+/(\d+))"#
        ))
        .unwrap()
    });
    &PULL_REQUEST_NUMBER_REGEX
}

/// https://gerrit-review.googlesource.com/Documentation/rest-api-changes.html
#[derive(Debug, Deserialize)]
struct ChangeInfo {
    owner: AccountInfo,
}

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "_account_id")]
    id: u64,
}

pub struct Chromium;

impl Chromium {
    async fn fetch_chromium_commit_author(
        &self,
        _repo: &str,
        commit: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Option<AccountInfo>> {
        let url = format!("{CHROMIUM_REVIEW_URL}/changes/{commit}");

        let request = Request::get(&url)
            .header("Content-Type", "application/json")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching Gerrit commit details at {:?}", url))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        if response.status().is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status().as_u16()
            );
        }

        // Remove XSSI protection prefix.
        let body_str = std::str::from_utf8(&body)?.trim_start_matches(")]}'");

        serde_json::from_str::<ChangeInfo>(body_str)
            .map(|change| Some(change.owner))
            .context("failed to deserialize Gerrit change info")
    }
}

#[async_trait]
impl GitHostingProvider for Chromium {
    fn name(&self) -> String {
        "Chromium".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://chromium.googlesource.com").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        true
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("{line}")
    }

    fn format_line_numbers(&self, start_line: u32, _end_line: u32) -> String {
        format!("{start_line}")
    }

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        let host = url.host_str()?;
        if host != self.base_url().host_str()? {
            return None;
        }

        let path_segments = url.path_segments()?.collect::<Vec<_>>();
        let joined_path = path_segments.join("/");
        let repo = joined_path.trim_end_matches(".git");

        Some(ParsedGitRemote {
            owner: Arc::from(""),
            repo: repo.into(),
        })
    }

    fn build_commit_permalink(
        &self,
        remote: &ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url {
        let BuildCommitPermalinkParams { sha } = params;
        let ParsedGitRemote { owner: _, repo } = remote;

        self.base_url().join(&format!("{repo}/+/{sha}")).unwrap()
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote { owner: _, repo } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
        } = params;

        let mut permalink = self
            .base_url()
            .join(&format!("{repo}/+/{sha}/{path}"))
            .unwrap();
        permalink.set_fragment(
            selection
                .map(|selection| self.line_fragment(&selection))
                .as_deref(),
        );
        permalink
    }

    fn extract_pull_request(&self, remote: &ParsedGitRemote, message: &str) -> Option<PullRequest> {
        let capture = pull_request_regex().captures(message)?;
        let url = Url::parse(capture.get(1)?.as_str()).unwrap();
        let repo = capture.get(2)?.as_str();
        if repo != remote.repo.as_ref() {
            return None;
        }

        let number = capture.get(3)?.as_str().parse::<u32>().ok()?;

        Some(PullRequest { number, url })
    }

    async fn commit_author_avatar_url(
        &self,
        _repo_owner: &str,
        repo: &str,
        commit: SharedString,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        let commit = commit.to_string();
        let Some(author) = self
            .fetch_chromium_commit_author(repo, &commit, &http_client)
            .await?
        else {
            return Ok(None);
        };

        let mut avatar_url = Url::parse(&format!(
            "{CHROMIUM_REVIEW_URL}/accounts/{}/avatar",
            &author.id
        ))?;
        avatar_url.set_query(Some("size=128"));

        Ok(Some(avatar_url))
    }
}

#[cfg(test)]
mod tests {
    use git::repository::repo_path;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_remote_url_given_https_url() {
        let parsed_remote = Chromium
            .parse_remote_url("https://chromium.googlesource.com/chromium/src")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: Arc::from(""),
                repo: "chromium/src".into(),
            }
        );
    }

    #[test]
    fn test_build_chromium_permalink() {
        let permalink = Chromium.build_permalink(
            ParsedGitRemote {
                owner: Arc::from(""),
                repo: "chromium/src".into(),
            },
            BuildPermalinkParams::new(
                "fea5080b182fc92e3be0c01c5dece602fe70b588",
                &repo_path("ui/base/cursor/cursor.h"),
                None,
            ),
        );

        let expected_url = "https://chromium.googlesource.com/chromium/src/+/fea5080b182fc92e3be0c01c5dece602fe70b588/ui/base/cursor/cursor.h";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_chromium_permalink_with_single_line_selection() {
        let permalink = Chromium.build_permalink(
            ParsedGitRemote {
                owner: Arc::from(""),
                repo: "chromium/src".into(),
            },
            BuildPermalinkParams::new(
                "fea5080b182fc92e3be0c01c5dece602fe70b588",
                &repo_path("ui/base/cursor/cursor.h"),
                Some(18..18),
            ),
        );

        let expected_url = "https://chromium.googlesource.com/chromium/src/+/fea5080b182fc92e3be0c01c5dece602fe70b588/ui/base/cursor/cursor.h#19";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_chromium_permalink_with_multi_line_selection() {
        let permalink = Chromium.build_permalink(
            ParsedGitRemote {
                owner: Arc::from(""),
                repo: "chromium/src".into(),
            },
            BuildPermalinkParams::new(
                "fea5080b182fc92e3be0c01c5dece602fe70b588",
                &repo_path("ui/base/cursor/cursor.h"),
                Some(18..30),
            ),
        );

        let expected_url = "https://chromium.googlesource.com/chromium/src/+/fea5080b182fc92e3be0c01c5dece602fe70b588/ui/base/cursor/cursor.h#19";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_chromium_pull_requests() {
        let remote = ParsedGitRemote {
            owner: Arc::from(""),
            repo: "chromium/src".into(),
        };

        let message = "This does not contain a pull request";
        assert!(Chromium.extract_pull_request(&remote, message).is_none());

        // Pull request number at end of "Reviewed-on:" line
        let message = indoc! {r#"
                Test commit header

                Test commit description with multiple
                lines.

                Bug: 1193775, 1270302
                Change-Id: Id15e9b4d75cce43ebd5fe34f0fb37d5e1e811b66
                Reviewed-on: https://chromium-review.googlesource.com/c/chromium/src/+/3310961
                Reviewed-by: Test reviewer <test@example.com>
                Cr-Commit-Position: refs/heads/main@{#1054973}
                "#
        };

        assert_eq!(
            Chromium
                .extract_pull_request(&remote, message)
                .unwrap()
                .url
                .as_str(),
            "https://chromium-review.googlesource.com/c/chromium/src/+/3310961"
        );
    }
}
