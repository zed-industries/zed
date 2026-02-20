use std::str::FromStr;
use std::sync::{Arc, LazyLock};

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
    PullRequest, RemoteUrl,
};

fn merge_request_number_regex() -> &'static Regex {
    static MERGE_REQUEST_NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        // Matches GitLab MR references:
        // - "(!123)" at the end of line (squash merge pattern)
        // - "See merge request group/project!123" (standard merge commit)
        Regex::new(r"(?:\(!(\d+)\)$|See merge request [^\s]+!(\d+))").unwrap()
    });
    &MERGE_REQUEST_NUMBER_REGEX
}

use crate::get_host_from_git_remote_url;

#[derive(Debug, Deserialize)]
struct CommitDetails {
    author_email: String,
}

#[derive(Debug, Deserialize)]
struct AvatarInfo {
    avatar_url: String,
}

#[derive(Debug)]
pub struct Gitlab {
    name: String,
    base_url: Url,
}

impl Gitlab {
    pub fn new(name: impl Into<String>, base_url: Url) -> Self {
        Self {
            name: name.into(),
            base_url,
        }
    }

    pub fn public_instance() -> Self {
        Self::new("GitLab", Url::parse("https://gitlab.com").unwrap())
    }

    pub fn from_remote_url(remote_url: &str) -> Result<Self> {
        let host = get_host_from_git_remote_url(remote_url)?;
        if host == "gitlab.com" {
            bail!("the GitLab instance is not self-hosted");
        }

        // TODO: detecting self hosted instances by checking whether "gitlab" is in the url or not
        // is not very reliable. See https://github.com/zed-industries/zed/issues/26393 for more
        // information.
        if !host.contains("gitlab") {
            bail!("not a GitLab URL");
        }

        Ok(Self::new(
            "GitLab Self-Hosted",
            Url::parse(&format!("https://{}", host))?,
        ))
    }

    async fn fetch_gitlab_commit_author(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Option<AvatarInfo>> {
        let Some(host) = self.base_url.host_str() else {
            bail!("failed to get host from gitlab base url");
        };
        let project_path = format!("{}/{}", repo_owner, repo);
        let project_path_encoded = urlencoding::encode(&project_path);
        let url = format!(
            "https://{host}/api/v4/projects/{project_path_encoded}/repository/commits/{commit}"
        );

        let request = Request::get(&url)
            .header("Content-Type", "application/json")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching GitLab commit details at {:?}", url))?;

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

        let author_email = serde_json::from_str::<CommitDetails>(body_str)
            .map(|commit| commit.author_email)
            .context("failed to deserialize GitLab commit details")?;

        let avatar_info_url = format!("https://{host}/api/v4/avatar?email={author_email}");

        let request = Request::get(&avatar_info_url)
            .header("Content-Type", "application/json")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching GitLab avatar info at {:?}", url))?;

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

        serde_json::from_str::<Option<AvatarInfo>>(body_str)
            .context("failed to deserialize GitLab avatar info")
    }
}

#[async_trait]
impl GitHostingProvider for Gitlab {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn base_url(&self) -> Url {
        self.base_url.clone()
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
        if host != self.base_url.host_str()? {
            return None;
        }

        let mut path_segments = url.path_segments()?.collect::<Vec<_>>();
        let repo = path_segments.pop()?.trim_end_matches(".git");
        let owner = path_segments.join("/");

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
            .join(&format!("{owner}/{repo}/-/commit/{sha}"))
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
            .join(&format!("{owner}/{repo}/-/blob/{sha}/{path}"))
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
        let mut url = self
            .base_url()
            .join(&format!(
                "{}/{}/-/merge_requests/new",
                remote.owner, remote.repo
            ))
            .ok()?;

        let query = format!("merge_request%5Bsource_branch%5D={}", encode(source_branch));

        url.set_query(Some(&query));
        Some(url)
    }

    fn extract_pull_request(&self, remote: &ParsedGitRemote, message: &str) -> Option<PullRequest> {
        // Check commit message for GitLab MR references
        let capture = merge_request_number_regex().captures(message)?;
        // The regex has two capture groups - one for "(!123)" pattern, one for "See merge request" pattern
        let number = capture
            .get(1)
            .or_else(|| capture.get(2))?
            .as_str()
            .parse::<u32>()
            .ok()?;

        let mut url = self.base_url();
        let path = format!(
            "{}/{}/-/merge_requests/{}",
            remote.owner, remote.repo, number
        );
        url.set_path(&path);

        Some(PullRequest { number, url })
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
            .fetch_gitlab_commit_author(repo_owner, repo, &commit, &http_client)
            .await?
            .map(|author| -> Result<Url, url::ParseError> {
                let mut url = Url::parse(&author.avatar_url)?;
                if let Some(host) = url.host_str() {
                    let size_query = if host.contains("gravatar") || host.contains("libravatar") {
                        Some("s=128")
                    } else if self
                        .base_url
                        .host_str()
                        .is_some_and(|base_host| host.contains(base_host))
                    {
                        Some("width=128")
                    } else {
                        None
                    };
                    url.set_query(size_query);
                }
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
    fn test_invalid_self_hosted_remote_url() {
        let remote_url = "https://gitlab.com/zed-industries/zed.git";
        let gitlab = Gitlab::from_remote_url(remote_url);
        assert!(gitlab.is_err());
    }

    #[test]
    fn test_parse_remote_url_given_ssh_url() {
        let parsed_remote = Gitlab::public_instance()
            .parse_remote_url("git@gitlab.com:zed-industries/zed.git")
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
        let parsed_remote = Gitlab::public_instance()
            .parse_remote_url("https://gitlab.com/zed-industries/zed.git")
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
    fn test_parse_remote_url_given_self_hosted_ssh_url() {
        let remote_url = "git@gitlab.my-enterprise.com:zed-industries/zed.git";

        let parsed_remote = Gitlab::from_remote_url(remote_url)
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
        let remote_url = "https://gitlab.my-enterprise.com/group/subgroup/zed.git";
        let parsed_remote = Gitlab::from_remote_url(remote_url)
            .unwrap()
            .parse_remote_url(remote_url)
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "group/subgroup".into(),
                repo: "zed".into(),
            }
        );
    }

    #[test]
    fn test_build_gitlab_permalink() {
        let permalink = Gitlab::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_with_single_line_selection() {
        let permalink = Gitlab::public_instance().build_permalink(
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

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_with_multi_line_selection() {
        let permalink = Gitlab::public_instance().build_permalink(
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

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_create_pr_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let provider = Gitlab::public_instance();

        let url = provider
            .build_create_pull_request_url(&remote, "feature/cool stuff")
            .expect("create PR url should be constructed");

        assert_eq!(
            url.as_str(),
            "https://gitlab.com/zed-industries/zed/-/merge_requests/new?merge_request%5Bsource_branch%5D=feature%2Fcool%20stuff"
        );
    }

    #[test]
    fn test_build_gitlab_self_hosted_permalink_from_ssh_url() {
        let gitlab =
            Gitlab::from_remote_url("git@gitlab.some-enterprise.com:zed-industries/zed.git")
                .unwrap();
        let permalink = gitlab.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://gitlab.some-enterprise.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_self_hosted_permalink_from_https_url() {
        let gitlab =
            Gitlab::from_remote_url("https://gitlab-instance.big-co.com/zed-industries/zed.git")
                .unwrap();
        let permalink = gitlab.build_permalink(
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

        let expected_url = "https://gitlab-instance.big-co.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_create_pull_request_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let github = Gitlab::public_instance();
        let url = github
            .build_create_pull_request_url(&remote, "feature/new-feature")
            .unwrap();

        assert_eq!(
            url.as_str(),
            "https://gitlab.com/zed-industries/zed/-/merge_requests/new?merge_request%5Bsource_branch%5D=feature%2Fnew-feature"
        );

        let base_url = Url::parse("https://gitlab.zed.com").unwrap();
        let github = Gitlab::new("GitLab Self-Hosted", base_url);
        let url = github
            .build_create_pull_request_url(&remote, "feature/new-feature")
            .expect("should be able to build pull request url");

        assert_eq!(
            url.as_str(),
            "https://gitlab.zed.com/zed-industries/zed/-/merge_requests/new?merge_request%5Bsource_branch%5D=feature%2Fnew-feature"
        );
    }

    #[test]
    fn test_extract_merge_request_from_squash_commit() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let provider = Gitlab::public_instance();

        // Test squash merge pattern: "commit message (!123)"
        let message = "Add new feature (!456)";
        let pull_request = provider.extract_pull_request(&remote, message).unwrap();

        assert_eq!(pull_request.number, 456);
        assert_eq!(
            pull_request.url.as_str(),
            "https://gitlab.com/zed-industries/zed/-/merge_requests/456"
        );
    }

    #[test]
    fn test_extract_merge_request_from_merge_commit() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let provider = Gitlab::public_instance();

        // Test standard merge commit pattern: "See merge request group/project!123"
        let message =
            "Merge branch 'feature' into 'main'\n\nSee merge request zed-industries/zed!789";
        let pull_request = provider.extract_pull_request(&remote, message).unwrap();

        assert_eq!(pull_request.number, 789);
        assert_eq!(
            pull_request.url.as_str(),
            "https://gitlab.com/zed-industries/zed/-/merge_requests/789"
        );
    }

    #[test]
    fn test_extract_merge_request_self_hosted() {
        let base_url = Url::parse("https://gitlab.my-company.com").unwrap();
        let provider = Gitlab::new("GitLab Self-Hosted", base_url);

        let remote = ParsedGitRemote {
            owner: "team".into(),
            repo: "project".into(),
        };

        let message = "Fix bug (!42)";
        let pull_request = provider.extract_pull_request(&remote, message).unwrap();

        assert_eq!(pull_request.number, 42);
        assert_eq!(
            pull_request.url.as_str(),
            "https://gitlab.my-company.com/team/project/-/merge_requests/42"
        );
    }

    #[test]
    fn test_extract_merge_request_no_match() {
        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let provider = Gitlab::public_instance();

        // No MR reference in message
        let message = "Just a regular commit message";
        let pull_request = provider.extract_pull_request(&remote, message);

        assert!(pull_request.is_none());
    }
}
