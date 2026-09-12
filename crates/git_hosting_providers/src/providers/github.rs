use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::SharedString;
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request};
use regex::Regex;
use serde::Deserialize;
use url::Url;
use urlencoding::encode;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    PullRequest, RemoteUrl, RepositorySearchResult,
};

use crate::get_host_from_git_remote_url;

const REPOSITORY_SEARCH_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_REPOSITORY_SEARCH_RESULTS: usize = 8;

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

#[derive(Deserialize)]
struct RepositorySearchResponse {
    items: Vec<GithubRepository>,
}

#[derive(Deserialize)]
struct GithubRepository {
    full_name: String,
    clone_url: String,
    private: bool,
    description: Option<String>,
}

impl From<GithubRepository> for RepositorySearchResult {
    fn from(repository: GithubRepository) -> Self {
        let visibility = if repository.private {
            "private"
        } else {
            "public"
        };
        let detail = match repository.description {
            Some(description) => format!("{visibility} - {description}").into(),
            None => SharedString::new_static(visibility),
        };
        Self {
            name: repository.full_name.into(),
            detail,
            clone_url: repository.clone_url.into(),
        }
    }
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

        let mut request = Request::get(&url)
            .header("Content-Type", "application/json")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        if let Ok(github_token) = std::env::var("GITHUB_TOKEN") {
            request = request.header("Authorization", format!("Bearer {}", github_token));
        }

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching GitHub commit details at {:?}", url))?;

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
            .context("failed to deserialize GitHub commit details")
    }

    async fn search_public_repositories(
        &self,
        query: &str,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Vec<RepositorySearchResult>> {
        let mut url = Url::parse("https://api.github.com/search/repositories")?;
        url.query_pairs_mut()
            .append_pair("q", query)
            .append_pair("sort", "updated")
            .append_pair("per_page", &MAX_REPOSITORY_SEARCH_RESULTS.to_string());

        let mut request = Request::get(url.as_str())
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .follow_redirects(http_client::RedirectPolicy::FollowAll)
            .timeout(REPOSITORY_SEARCH_TIMEOUT);
        if let Some(token) = github_token() {
            request = request.header("Authorization", format!("Bearer {token}"));
        }

        let mut response = http_client
            .send(request.body(AsyncBody::default())?)
            .await
            .context("requesting GitHub repository suggestions")?;
        anyhow::ensure!(
            response.status().is_success(),
            "GitHub repository search returned HTTP {}",
            response.status(),
        );

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("reading GitHub repository suggestions")?;
        let response: RepositorySearchResponse =
            serde_json::from_slice(&body).context("parsing GitHub repository suggestions")?;

        Ok(response
            .items
            .into_iter()
            .take(MAX_REPOSITORY_SEARCH_RESULTS)
            .map(RepositorySearchResult::from)
            .collect())
    }
}

fn github_token() -> Option<String> {
    std::env::var("GITHUB_TOKEN")
        .ok()
        .filter(|token| !token.is_empty())
        .or_else(|| {
            std::env::var("GH_TOKEN")
                .ok()
                .filter(|token| !token.is_empty())
        })
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

    fn supports_repository_search(&self) -> bool {
        self.base_url.host_str() == Some("github.com")
    }

    async fn search_repositories(
        &self,
        query: &str,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Vec<RepositorySearchResult>> {
        if !self.supports_repository_search() {
            return Ok(Vec::new());
        }

        self.search_public_repositories(query, http_client).await
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
}

#[cfg(test)]
mod tests {
    use git::repository::repo_path;
    use http_client::{AsyncBody, FakeHttpClient, Response};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::sync::atomic::{AtomicBool, Ordering};

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
    fn test_repository_search_supports_only_public_github() {
        assert!(Github::public_instance().supports_repository_search());
        assert!(
            !Github::new(
                "GitHub Self-Hosted",
                Url::parse("https://github.example.com").expect("valid GitHub URL")
            )
            .supports_repository_search()
        );
    }

    #[test]
    fn test_search_repositories() {
        let http_client = FakeHttpClient::create(|request| async move {
            assert_eq!(
                request.uri().to_string(),
                "https://api.github.com/search/repositories?q=zed+editor&sort=updated&per_page=8"
            );
            assert_eq!(
                request
                    .headers()
                    .get("Accept")
                    .and_then(|value| value.to_str().ok()),
                Some("application/vnd.github+json")
            );
            Ok(Response::builder()
                .status(200)
                .body(AsyncBody::from(
                    r#"{"items":[{"full_name":"zed-industries/zed","clone_url":"https://github.com/zed-industries/zed.git","private":false,"description":"Code at the speed of thought"}]}"#,
                ))
                .expect("valid response"))
        });

        let results = futures::executor::block_on(
            Github::public_instance().search_repositories("zed editor", http_client),
        )
        .expect("repository search should succeed");

        assert_eq!(
            results,
            vec![RepositorySearchResult {
                name: "zed-industries/zed".into(),
                detail: "public - Code at the speed of thought".into(),
                clone_url: "https://github.com/zed-industries/zed.git".into(),
            }]
        );
    }

    #[test]
    fn test_search_repositories_reports_http_errors() {
        let http_client = FakeHttpClient::create(|_| async move {
            Ok(Response::builder()
                .status(403)
                .body(AsyncBody::default())
                .expect("valid response"))
        });

        let error = futures::executor::block_on(
            Github::public_instance().search_repositories("zed", http_client),
        )
        .expect_err("repository search should fail");

        assert_eq!(
            error.to_string(),
            "GitHub repository search returned HTTP 403 Forbidden"
        );
    }

    #[test]
    fn test_self_hosted_search_does_not_make_network_requests() {
        let request_sent = Arc::new(AtomicBool::new(false));
        let http_client = FakeHttpClient::create({
            let request_sent = request_sent.clone();
            move |_| {
                request_sent.store(true, Ordering::SeqCst);
                async move { anyhow::bail!("unexpected repository search request") }
            }
        });
        let github = Github::new(
            "GitHub Self-Hosted",
            Url::parse("https://github.example.com").expect("valid GitHub URL"),
        );

        let results = futures::executor::block_on(github.search_repositories("zed", http_client))
            .expect("unsupported repository search should be empty");

        assert!(results.is_empty());
        assert!(!request_sent.load(Ordering::SeqCst));
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
}
