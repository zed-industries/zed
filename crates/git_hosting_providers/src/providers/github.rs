use std::str::FromStr;
use std::sync::{Arc, LazyLock};

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::SharedString;
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request};
use regex::Regex;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use time::OffsetDateTime;
use url::Url;
use urlencoding::encode;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    PullRequest, PullRequestComment, RemoteUrl,
};

use crate::get_host_from_git_remote_url;

fn pull_request_number_regex() -> &'static Regex {
    static PULL_REQUEST_NUMBER_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\(#(\d+)\)$").unwrap());
    &PULL_REQUEST_NUMBER_REGEX
}

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
    email: String,
}

#[derive(Debug, Deserialize)]
struct User {
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub id: u64,
    pub avatar_url: String,
}

/// A pull request as returned by the GitHub REST API. Mapped into the
/// provider-agnostic [`PullRequest`] before leaving this module.
#[derive(Debug, Deserialize)]
struct GithubPullRequest {
    number: u32,
    /// The browser-facing URL (e.g. `https://github.com/owner/repo/pull/1`),
    /// as opposed to the API URL returned in the `url` field.
    html_url: Url,
}

/// A pull request review comment as returned by the GitHub REST API. Mapped
/// into the provider-agnostic [`PullRequestComment`] before leaving this module.
#[derive(Debug, Deserialize)]
struct GithubPullRequestComment {
    user: GithubCommentAuthor,
    body: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    path: String,
    #[serde(default)]
    line: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GithubCommentAuthor {
    login: String,
}

#[derive(Debug)]
pub struct Github {
    name: String,
    base_url: Url,
}

fn normalize_author_email(email: &str) -> &str {
    email.trim_start_matches('<').trim_end_matches('>')
}

fn build_cdn_avatar_url(email: &str) -> Result<Url> {
    let email = normalize_author_email(email);
    Url::parse(&format!(
        "https://avatars.githubusercontent.com/u/e?email={}&s=128",
        encode(email)
    ))
    .context("failed to construct avatar URL")
}

fn build_cdn_avatar_url_for_author_email(email: &str) -> Result<Option<Url>> {
    let email = normalize_author_email(email);
    if email.ends_with("[bot]@users.noreply.github.com") {
        return Ok(None);
    }

    build_cdn_avatar_url(email).map(Some)
}

impl Github {
    pub fn new(name: impl Into<String>, base_url: Url) -> Self {
        Self {
            name: name.into(),
            base_url,
        }
    }

    pub fn public_instance() -> Self {
        Self::new("GitHub", Url::parse("https://github.com").unwrap())
    }

    pub fn from_remote_url(remote_url: &str) -> Result<Self> {
        let host = get_host_from_git_remote_url(remote_url)?;
        if host == "github.com" {
            bail!("the GitHub instance is not self-hosted");
        }

        // TODO: detecting self hosted instances by checking whether "github" is in the url or not
        // is not very reliable. See https://github.com/zed-industries/zed/issues/26393 for more
        // information.
        if !host.contains("github") {
            bail!("not a GitHub URL");
        }

        Ok(Self::new(
            "GitHub Self-Hosted",
            Url::parse(&format!("https://{}", host))?,
        ))
    }

    /// Sends an authenticated GET request to the given GitHub API URL and
    /// deserializes the JSON response body into `T`.
    //
    // TODO(pr-comments): authentication is currently limited to the
    // `GITHUB_TOKEN` environment variable. Replace this with real auth (reuse
    // the user's git credentials or `gh auth token`) before shipping. Responses
    // are also not paginated yet, so only the first page of results is returned.
    async fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
        accept: Option<&str>,
        client: &Arc<dyn HttpClient>,
    ) -> Result<T> {
        let mut request =
            Request::get(url).follow_redirects(http_client::RedirectPolicy::FollowAll);
        if let Some(accept) = accept {
            request = request.header("Accept", accept);
        }
        if let Ok(github_token) = std::env::var("GITHUB_TOKEN") {
            request = request.header("Authorization", format!("Bearer {}", github_token));
        }

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching GitHub API at {url:?}"))?;

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

        serde_json::from_str::<T>(body_str)
            .with_context(|| format!("failed to deserialize GitHub API response from {url:?}"))
    }

    async fn fetch_github_commit_author(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Option<User>> {
        let Some(host) = self.base_url.host_str() else {
            bail!("failed to get host from github base url");
        };
        let url = format!("https://api.{host}/repos/{repo_owner}/{repo}/commits/{commit}");

        let details = self.get_json::<CommitDetails>(&url, None, client).await?;
        Ok(details.author)
    }

    async fn fetch_github_pull_request_for_branch(
        &self,
        repo_owner: &str,
        repo: &str,
        branch: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Option<PullRequest>> {
        let Some(host) = self.base_url.host_str() else {
            bail!("failed to get host from github base url");
        };
        let url = format!(
            "https://api.{host}/repos/{repo_owner}/{repo}/pulls?head={repo_owner}:{branch}"
        );

        // The list-pull-requests endpoint always returns a JSON array, even when
        // no pull request matches the branch. Take the first match, if any.
        let pull_requests = self
            .get_json::<Vec<GithubPullRequest>>(&url, None, client)
            .await?;
        Ok(pull_requests
            .into_iter()
            .next()
            .map(|pull_request| PullRequest {
                number: pull_request.number,
                url: pull_request.html_url,
            }))
    }

    async fn fetch_github_pull_request_comments(
        &self,
        repo_owner: &str,
        repo: &str,
        pull_request_id: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Vec<PullRequestComment>> {
        let Some(host) = self.base_url.host_str() else {
            bail!("failed to get host from github base url");
        };
        let url = format!(
            "https://api.{host}/repos/{repo_owner}/{repo}/pulls/{pull_request_id}/comments"
        );

        let comments = self
            .get_json::<Vec<GithubPullRequestComment>>(
                &url,
                Some("application/vnd.github+json"),
                client,
            )
            .await?;
        Ok(comments
            .into_iter()
            .map(|comment| PullRequestComment {
                author_name: comment.user.login,
                body: comment.body,
                created_at: comment.created_at,
                file_path: comment.path,
                line: comment.line,
            })
            .collect())
    }
}

#[async_trait]
impl GitHostingProvider for Github {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn base_url(&self) -> Url {
        self.base_url.clone()
    }

    fn supports_avatars(&self) -> bool {
        // Avatars are not supported for self-hosted GitHub instances
        // See tracking issue: https://github.com/zed-industries/zed/issues/11043
        &self.name == "GitHub"
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
        if host != self.base_url.host_str()? {
            return None;
        }

        let mut path_segments = url.path_segments()?;
        let mut owner = path_segments.next()?;
        if owner.is_empty() {
            owner = path_segments.next()?;
        }

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
        if path.ends_with(".md") {
            permalink.set_query(Some("plain=1"));
        }
        permalink.set_fragment(
            selection
                .map(|selection| self.line_fragment(&selection))
                .as_deref(),
        );
        permalink
    }

    fn build_create_pull_request_url(
        &self,
        remote: &ParsedGitRemote,
        source_branch: &str,
    ) -> Option<Url> {
        let ParsedGitRemote { owner, repo } = remote;
        let encoded_source = encode(source_branch);

        self.base_url()
            .join(&format!("{owner}/{repo}/pull/new/{encoded_source}"))
            .ok()
    }

    fn extract_pull_request(&self, remote: &ParsedGitRemote, message: &str) -> Option<PullRequest> {
        let line = message.lines().next()?;
        let capture = pull_request_number_regex().captures(line)?;
        let number = capture.get(1)?.as_str().parse::<u32>().ok()?;

        let mut url = self.base_url();
        let path = format!("/{}/{}/pull/{}", remote.owner, remote.repo, number);
        url.set_path(&path);

        Some(PullRequest { number, url })
    }

    async fn commit_author_avatar_url(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: SharedString,
        author_email: Option<SharedString>,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        if let Some(email) = author_email
            && let Some(avatar_url) = build_cdn_avatar_url_for_author_email(&email)?
        {
            return Ok(Some(avatar_url));
        }

        let commit = commit.to_string();
        let avatar_url = self
            .fetch_github_commit_author(repo_owner, repo, &commit, &http_client)
            .await?
            .map(|author| -> Result<Url, url::ParseError> {
                let mut url = Url::parse(&author.avatar_url)?;
                url.set_query(Some("size=128"));
                Ok(url)
            })
            .transpose()?;
        Ok(avatar_url)
    }

    async fn pull_request_for_branch(
        &self,
        repo_owner: &str,
        repo: &str,
        branch: &str,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<PullRequest>> {
        self.fetch_github_pull_request_for_branch(repo_owner, repo, branch, &http_client)
            .await
    }

    async fn pull_request_comments(
        &self,
        repo_owner: &str,
        repo: &str,
        pull_request_id: &str,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Vec<PullRequestComment>> {
        self.fetch_github_pull_request_comments(repo_owner, repo, pull_request_id, &http_client)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use git::repository::repo_path;
    use http_client::{FakeHttpClient, Response};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use time::macros::datetime;

    use super::*;

    #[test]
    fn test_remote_url_with_root_slash() {
        let remote_url = "git@github.com:/zed-industries/zed";
        let parsed_remote = Github::public_instance()
            .parse_remote_url(remote_url)
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
    fn test_invalid_self_hosted_remote_url() {
        let remote_url = "git@github.com:zed-industries/zed.git";
        let github = Github::from_remote_url(remote_url);
        assert!(github.is_err());
    }

    #[test]
    fn test_from_remote_url_ssh() {
        let remote_url = "git@github.my-enterprise.com:zed-industries/zed.git";
        let github = Github::from_remote_url(remote_url).unwrap();

        assert!(!github.supports_avatars());
        assert_eq!(github.name, "GitHub Self-Hosted".to_string());
        assert_eq!(
            github.base_url,
            Url::parse("https://github.my-enterprise.com").unwrap()
        );
    }

    #[test]
    fn test_from_remote_url_https() {
        let remote_url = "https://github.my-enterprise.com/zed-industries/zed.git";
        let github = Github::from_remote_url(remote_url).unwrap();

        assert!(!github.supports_avatars());
        assert_eq!(github.name, "GitHub Self-Hosted".to_string());
        assert_eq!(
            github.base_url,
            Url::parse("https://github.my-enterprise.com").unwrap()
        );
    }

    #[test]
    fn test_parse_remote_url_given_self_hosted_ssh_url() {
        let remote_url = "git@github.my-enterprise.com:zed-industries/zed.git";
        let parsed_remote = Github::from_remote_url(remote_url)
            .unwrap()
            .parse_remote_url(remote_url)
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
    fn test_parse_remote_url_given_self_hosted_https_url_with_subgroup() {
        let remote_url = "https://github.my-enterprise.com/zed-industries/zed.git";
        let parsed_remote = Github::from_remote_url(remote_url)
            .unwrap()
            .parse_remote_url(remote_url)
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
    fn test_parse_remote_url_given_ssh_url() {
        let parsed_remote = Github::public_instance()
            .parse_remote_url("git@github.com:zed-industries/zed.git")
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
        let parsed_remote = Github::public_instance()
            .parse_remote_url("https://github.com/zed-industries/zed.git")
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
    fn test_parse_remote_url_given_https_url_with_username() {
        let parsed_remote = Github::public_instance()
            .parse_remote_url("https://jlannister@github.com/some-org/some-repo.git")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "some-org".into(),
                repo: "some-repo".into(),
            }
        );
    }

    #[test]
    fn test_build_github_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };
        let permalink = Github::public_instance().build_permalink(
            remote,
            BuildPermalinkParams::new(
                "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink() {
        let permalink = Github::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                &repo_path("crates/zed/src/main.rs"),
                None,
            ),
        );

        let expected_url = "https://github.com/zed-industries/zed/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_with_single_line_selection() {
        let permalink = Github::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(6..6),
            ),
        );

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_with_multi_line_selection() {
        let permalink = Github::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(23..47),
            ),
        );

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_create_pr_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let provider = Github::public_instance();

        let url = provider
            .build_create_pull_request_url(&remote, "feature/something cool")
            .expect("url should be constructed");

        assert_eq!(
            url.as_str(),
            "https://github.com/zed-industries/zed/pull/new/feature%2Fsomething%20cool"
        );
    }

    #[test]
    fn test_github_pull_requests() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let github = Github::public_instance();
        let message = "This does not contain a pull request";
        assert!(github.extract_pull_request(&remote, message).is_none());

        // Pull request number at end of first line
        let message = indoc! {r#"
            project panel: do not expand collapsed worktrees on "collapse all entries" (#10687)

            Fixes #10597

            Release Notes:

            - Fixed "project panel: collapse all entries" expanding collapsed worktrees.
            "#
        };

        assert_eq!(
            github
                .extract_pull_request(&remote, message)
                .unwrap()
                .url
                .as_str(),
            "https://github.com/zed-industries/zed/pull/10687"
        );

        // Pull request number in middle of line, which we want to ignore
        let message = indoc! {r#"
            Follow-up to #10687 to fix problems

            See the original PR, this is a fix.
            "#
        };
        assert_eq!(github.extract_pull_request(&remote, message), None);
    }

    /// Regression test for issue #39875
    #[test]
    fn test_git_permalink_url_escaping() {
        let permalink = Github::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "nonexistent".into(),
            },
            BuildPermalinkParams::new(
                "3ef1539900037dd3601be7149b2b39ed6d0ce3db",
                &repo_path("app/blog/[slug]/page.tsx"),
                Some(7..7),
            ),
        );

        let expected_url = "https://github.com/zed-industries/nonexistent/blob/3ef1539900037dd3601be7149b2b39ed6d0ce3db/app/blog/%5Bslug%5D/page.tsx#L8";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_create_pull_request_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let github = Github::public_instance();
        let url = github
            .build_create_pull_request_url(&remote, "feature/new-feature")
            .unwrap();

        assert_eq!(
            url.as_str(),
            "https://github.com/zed-industries/zed/pull/new/feature%2Fnew-feature"
        );

        let base_url = Url::parse("https://github.zed.com").unwrap();
        let github = Github::new("GitHub Self-Hosted", base_url);
        let url = github
            .build_create_pull_request_url(&remote, "feature/new-feature")
            .expect("should be able to build pull request url");

        assert_eq!(
            url.as_str(),
            "https://github.zed.com/zed-industries/zed/pull/new/feature%2Fnew-feature"
        );
    }

    #[test]
    fn test_build_cdn_avatar_url_simple_email() {
        let url = build_cdn_avatar_url("user@example.com").unwrap();
        assert_eq!(
            url.as_str(),
            "https://avatars.githubusercontent.com/u/e?email=user%40example.com&s=128"
        );
    }

    #[test]
    fn test_build_cdn_avatar_url_with_angle_brackets() {
        let url = build_cdn_avatar_url("<user@example.com>").unwrap();
        assert_eq!(
            url.as_str(),
            "https://avatars.githubusercontent.com/u/e?email=user%40example.com&s=128"
        );
    }

    #[test]
    fn test_build_cdn_avatar_url_with_special_chars() {
        let url = build_cdn_avatar_url("user+tag@example.com").unwrap();
        assert_eq!(
            url.as_str(),
            "https://avatars.githubusercontent.com/u/e?email=user%2Btag%40example.com&s=128"
        );
    }

    #[test]
    fn test_build_cdn_avatar_url_for_author_email_skips_bot_noreply_emails() {
        for email in [
            "41898282+github-actions[bot]@users.noreply.github.com",
            "<41898282+github-actions[bot]@users.noreply.github.com>",
        ] {
            assert_eq!(build_cdn_avatar_url_for_author_email(email).unwrap(), None);
        }
    }

    #[test]
    fn test_build_cdn_avatar_url_for_author_email_uses_user_noreply_emails() {
        let url = build_cdn_avatar_url_for_author_email("12345+octocat@users.noreply.github.com")
            .unwrap()
            .unwrap();

        assert_eq!(
            url.as_str(),
            "https://avatars.githubusercontent.com/u/e?email=12345%2Boctocat%40users.noreply.github.com&s=128"
        );
    }

    /// A [`FakeHttpClient`] that records the URL and headers of the last request
    /// it received, then replies with the given status and body.
    fn recording_client(
        status: u16,
        body: &'static str,
    ) -> (Arc<dyn HttpClient>, Arc<Mutex<Option<RecordedRequest>>>) {
        let recorded = Arc::new(Mutex::new(None));
        let client: Arc<dyn HttpClient> = FakeHttpClient::create({
            let recorded = recorded.clone();
            move |request| {
                let recorded = recorded.clone();
                async move {
                    *recorded.lock().unwrap() = Some(RecordedRequest {
                        url: request.uri().to_string(),
                        accept: request
                            .headers()
                            .get("Accept")
                            .and_then(|value| value.to_str().ok())
                            .map(str::to_owned),
                    });
                    Ok(Response::builder()
                        .status(status)
                        .body(body.into())
                        .unwrap())
                }
            }
        });
        (client, recorded)
    }

    struct RecordedRequest {
        url: String,
        accept: Option<String>,
    }

    #[gpui::test]
    async fn test_pull_request_for_branch() {
        let (client, recorded) = recording_client(
            200,
            r#"[
                {
                    "number": 1234,
                    "url": "https://api.github.com/repos/zed-industries/zed/pulls/1234",
                    "html_url": "https://github.com/zed-industries/zed/pull/1234"
                }
            ]"#,
        );

        let pull_request = Github::public_instance()
            .pull_request_for_branch("zed-industries", "zed", "some-branch", client)
            .await
            .unwrap();

        assert_eq!(
            recorded.lock().unwrap().as_ref().unwrap().url,
            "https://api.github.com/repos/zed-industries/zed/pulls?head=zed-industries:some-branch"
        );
        assert_eq!(
            pull_request,
            Some(PullRequest {
                number: 1234,
                url: Url::parse("https://github.com/zed-industries/zed/pull/1234").unwrap(),
            })
        );
    }

    #[gpui::test]
    async fn test_pull_request_for_branch_when_none_exists() {
        let (client, _recorded) = recording_client(200, "[]");

        let pull_request = Github::public_instance()
            .pull_request_for_branch("zed-industries", "zed", "some-branch", client)
            .await
            .unwrap();

        assert_eq!(pull_request, None);
    }

    #[gpui::test]
    async fn test_pull_request_for_branch_self_hosted_url() {
        let (client, recorded) = recording_client(200, "[]");

        let github = Github::new(
            "GitHub Self-Hosted",
            Url::parse("https://github.my-enterprise.com").unwrap(),
        );
        github
            .pull_request_for_branch("zed-industries", "zed", "some-branch", client)
            .await
            .unwrap();

        assert_eq!(
            recorded.lock().unwrap().as_ref().unwrap().url,
            "https://api.github.my-enterprise.com/repos/zed-industries/zed/pulls?head=zed-industries:some-branch"
        );
    }

    #[gpui::test]
    async fn test_pull_request_for_branch_returns_error_on_client_error() {
        let (client, _recorded) = recording_client(404, "{ \"message\": \"Not Found\" }");

        let result = Github::public_instance()
            .pull_request_for_branch("zed-industries", "zed", "some-branch", client)
            .await;

        assert!(result.is_err());
    }

    #[gpui::test]
    async fn test_pull_request_comments() {
        let (client, recorded) = recording_client(
            200,
            r#"[
                {
                    "user": { "login": "octocat" },
                    "body": "Looks good to me!",
                    "created_at": "2024-01-01T00:00:00Z",
                    "path": "src/main.rs",
                    "line": 42
                },
                {
                    "user": { "login": "nanobot" },
                    "body": "One nit below.",
                    "created_at": "2024-01-02T00:00:00Z",
                    "path": "src/lib.rs",
                    "line": null
                }
            ]"#,
        );

        let comments = Github::public_instance()
            .pull_request_comments("zed-industries", "zed", "1234", client)
            .await
            .unwrap();

        let recorded = recorded.lock().unwrap();
        let recorded = recorded.as_ref().unwrap();
        assert_eq!(
            recorded.url,
            "https://api.github.com/repos/zed-industries/zed/pulls/1234/comments"
        );
        assert_eq!(
            recorded.accept.as_deref(),
            Some("application/vnd.github+json")
        );

        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].author_name, "octocat");
        assert_eq!(comments[0].body, "Looks good to me!");
        assert_eq!(comments[0].created_at, datetime!(2024-01-01 0:00 UTC));
        assert_eq!(comments[0].file_path, "src/main.rs");
        assert_eq!(comments[0].line, Some(42));
        assert_eq!(comments[1].author_name, "nanobot");
        assert_eq!(comments[1].body, "One nit below.");
        assert_eq!(comments[1].created_at, datetime!(2024-01-02 0:00 UTC));
        assert_eq!(comments[1].file_path, "src/lib.rs");
        assert_eq!(comments[1].line, None);
    }

    #[gpui::test]
    async fn test_pull_request_comments_when_empty() {
        let (client, _recorded) = recording_client(200, "[]");

        let comments = Github::public_instance()
            .pull_request_comments("zed-industries", "zed", "1234", client)
            .await
            .unwrap();

        assert!(comments.is_empty());
    }

    #[gpui::test]
    async fn test_pull_request_comments_returns_error_on_client_error() {
        let (client, _recorded) = recording_client(403, "{ \"message\": \"Forbidden\" }");

        let result = Github::public_instance()
            .pull_request_comments("zed-industries", "zed", "1234", client)
            .await;

        assert!(result.is_err());
    }
}
