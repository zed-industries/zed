use std::sync::LazyLock;
use std::{str::FromStr, sync::Arc};

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::SharedString;
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request};
use itertools::Itertools as _;
use regex::Regex;
use serde::Deserialize;
use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    PullRequest, RemoteUrl,
};

use crate::get_host_from_git_remote_url;

fn pull_request_regex() -> &'static Regex {
    static PULL_REQUEST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        // This matches Bitbucket PR reference pattern: (pull request #xxx)
        Regex::new(r"\(pull request #(\d+)\)").unwrap()
    });
    &PULL_REQUEST_REGEX
}

#[derive(Debug, Deserialize)]
struct CommitDetails {
    author: Author,
}

#[derive(Debug, Deserialize)]
struct Author {
    user: Account,
}

#[derive(Debug, Deserialize)]
struct Account {
    links: AccountLinks,
}

#[derive(Debug, Deserialize)]
struct AccountLinks {
    avatar: Option<Link>,
}

#[derive(Debug, Deserialize)]
struct Link {
    href: String,
}

#[derive(Debug, Deserialize)]
struct CommitDetailsSelfHosted {
    author: AuthorSelfHosted,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthorSelfHosted {
    avatar_url: Option<String>,
}

pub struct Bitbucket {
    name: String,
    base_url: Url,
}

impl Bitbucket {
    pub fn new(name: impl Into<String>, base_url: Url) -> Self {
        Self {
            name: name.into(),
            base_url,
        }
    }

    pub fn public_instance() -> Self {
        Self::new("Bitbucket", Url::parse("https://bitbucket.org").unwrap())
    }

    pub fn from_remote_url(remote_url: &str) -> Result<Self> {
        let host = get_host_from_git_remote_url(remote_url)?;
        if host == "bitbucket.org" {
            bail!("the BitBucket instance is not self-hosted");
        }

        // TODO: detecting self hosted instances by checking whether "bitbucket" is in the url or not
        // is not very reliable. See https://github.com/zed-industries/zed/issues/26393 for more
        // information.
        if !host.contains("bitbucket") {
            bail!("not a BitBucket URL");
        }

        Ok(Self::new(
            "BitBucket Self-Hosted",
            Url::parse(&format!("https://{}", host))?,
        ))
    }

    fn is_self_hosted(&self) -> bool {
        self.base_url
            .host_str()
            .is_some_and(|host| host != "bitbucket.org")
    }

    async fn fetch_bitbucket_commit_author(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Option<String>> {
        let Some(host) = self.base_url.host_str() else {
            bail!("failed to get host from bitbucket base url");
        };
        let is_self_hosted = self.is_self_hosted();
        let url = if is_self_hosted {
            format!(
                "https://{host}/rest/api/latest/projects/{repo_owner}/repos/{repo}/commits/{commit}?avatarSize=128"
            )
        } else {
            format!("https://api.{host}/2.0/repositories/{repo_owner}/{repo}/commit/{commit}")
        };

        let request = Request::get(&url)
            .header("Content-Type", "application/json")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching BitBucket commit details at {:?}", url))?;

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

        if is_self_hosted {
            serde_json::from_str::<CommitDetailsSelfHosted>(body_str)
                .map(|commit| commit.author.avatar_url)
        } else {
            serde_json::from_str::<CommitDetails>(body_str)
                .map(|commit| commit.author.user.links.avatar.map(|link| link.href))
        }
        .context("failed to deserialize BitBucket commit details")
    }
}

#[async_trait]
impl GitHostingProvider for Bitbucket {
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
        if self.is_self_hosted() {
            return format!("{line}");
        }
        format!("lines-{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        if self.is_self_hosted() {
            return format!("{start_line}-{end_line}");
        }
        format!("lines-{start_line}:{end_line}")
    }

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        let host = url.host_str()?;
        if host != self.base_url.host_str()? {
            return None;
        }

        let mut path_segments = url.path_segments()?.collect::<Vec<_>>();
        let repo = path_segments.pop()?.trim_end_matches(".git");
        let owner = if path_segments.get(0).is_some_and(|v| *v == "scm") && path_segments.len() > 1
        {
            // Skip the "scm" segment if it's not the only segment
            // https://github.com/gitkraken/vscode-gitlens/blob/a6e3c6fbb255116507eaabaa9940c192ed7bb0e1/src/git/remotes/bitbucket-server.ts#L72-L74
            path_segments.into_iter().skip(1).join("/")
        } else {
            path_segments.into_iter().join("/")
        };

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
        if self.is_self_hosted() {
            return self
                .base_url()
                .join(&format!("projects/{owner}/repos/{repo}/commits/{sha}"))
                .unwrap();
        }
        self.base_url()
            .join(&format!("{owner}/{repo}/commits/{sha}"))
            .unwrap()
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote { owner, repo } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
        } = params;

        let mut permalink = if self.is_self_hosted() {
            self.base_url()
                .join(&format!(
                    "projects/{owner}/repos/{repo}/browse/{path}?at={sha}"
                ))
                .unwrap()
        } else {
            self.base_url()
                .join(&format!("{owner}/{repo}/src/{sha}/{path}"))
                .unwrap()
        };

        permalink.set_fragment(
            selection
                .map(|selection| self.line_fragment(&selection))
                .as_deref(),
        );
        permalink
    }

    fn extract_pull_request(&self, remote: &ParsedGitRemote, message: &str) -> Option<PullRequest> {
        // Check first line of commit message for PR references
        let first_line = message.lines().next()?;

        // Try to match against our PR patterns
        let capture = pull_request_regex().captures(first_line)?;
        let number = capture.get(1)?.as_str().parse::<u32>().ok()?;

        // Construct the PR URL in Bitbucket format
        let mut url = self.base_url();
        let path = if self.is_self_hosted() {
            format!(
                "/projects/{}/repos/{}/pull-requests/{}",
                remote.owner, remote.repo, number
            )
        } else {
            format!("/{}/{}/pull-requests/{}", remote.owner, remote.repo, number)
        };
        url.set_path(&path);

        Some(PullRequest { number, url })
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
            .fetch_bitbucket_commit_author(repo_owner, repo, &commit, &http_client)
            .await?
            .map(|avatar_url| Url::parse(&avatar_url))
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
        let parsed_remote = Bitbucket::public_instance()
            .parse_remote_url("git@bitbucket.org:zed-industries/zed.git")
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
        let parsed_remote = Bitbucket::public_instance()
            .parse_remote_url("https://bitbucket.org/zed-industries/zed.git")
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
        let parsed_remote = Bitbucket::public_instance()
            .parse_remote_url("https://thorstenballzed@bitbucket.org/zed-industries/zed.git")
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
        let remote_url = "git@bitbucket.company.com:zed-industries/zed.git";

        let parsed_remote = Bitbucket::from_remote_url(remote_url)
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
    fn test_parse_remote_url_given_self_hosted_https_url() {
        let remote_url = "https://bitbucket.company.com/zed-industries/zed.git";

        let parsed_remote = Bitbucket::from_remote_url(remote_url)
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

        // Test with "scm" in the path
        let remote_url = "https://bitbucket.company.com/scm/zed-industries/zed.git";

        let parsed_remote = Bitbucket::from_remote_url(remote_url)
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

        // Test with only "scm" as owner
        let remote_url = "https://bitbucket.company.com/scm/zed.git";

        let parsed_remote = Bitbucket::from_remote_url(remote_url)
            .unwrap()
            .parse_remote_url(remote_url)
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "scm".into(),
                repo: "zed".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_self_hosted_https_url_with_username() {
        let remote_url = "https://thorstenballzed@bitbucket.company.com/zed-industries/zed.git";

        let parsed_remote = Bitbucket::from_remote_url(remote_url)
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
    fn test_build_bitbucket_permalink() {
        let permalink = Bitbucket::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new("f00b4r", &repo_path("main.rs"), None),
        );

        let expected_url = "https://bitbucket.org/zed-industries/zed/src/f00b4r/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_self_hosted_permalink() {
        let permalink =
            Bitbucket::from_remote_url("git@bitbucket.company.com:zed-industries/zed.git")
                .unwrap()
                .build_permalink(
                    ParsedGitRemote {
                        owner: "zed-industries".into(),
                        repo: "zed".into(),
                    },
                    BuildPermalinkParams::new("f00b4r", &repo_path("main.rs"), None),
                );

        let expected_url = "https://bitbucket.company.com/projects/zed-industries/repos/zed/browse/main.rs?at=f00b4r";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_permalink_with_single_line_selection() {
        let permalink = Bitbucket::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new("f00b4r", &repo_path("main.rs"), Some(6..6)),
        );

        let expected_url = "https://bitbucket.org/zed-industries/zed/src/f00b4r/main.rs#lines-7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_self_hosted_permalink_with_single_line_selection() {
        let permalink =
            Bitbucket::from_remote_url("https://bitbucket.company.com/zed-industries/zed.git")
                .unwrap()
                .build_permalink(
                    ParsedGitRemote {
                        owner: "zed-industries".into(),
                        repo: "zed".into(),
                    },
                    BuildPermalinkParams::new("f00b4r", &repo_path("main.rs"), Some(6..6)),
                );

        let expected_url = "https://bitbucket.company.com/projects/zed-industries/repos/zed/browse/main.rs?at=f00b4r#7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_permalink_with_multi_line_selection() {
        let permalink = Bitbucket::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new("f00b4r", &repo_path("main.rs"), Some(23..47)),
        );

        let expected_url =
            "https://bitbucket.org/zed-industries/zed/src/f00b4r/main.rs#lines-24:48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_bitbucket_self_hosted_permalink_with_multi_line_selection() {
        let permalink =
            Bitbucket::from_remote_url("git@bitbucket.company.com:zed-industries/zed.git")
                .unwrap()
                .build_permalink(
                    ParsedGitRemote {
                        owner: "zed-industries".into(),
                        repo: "zed".into(),
                    },
                    BuildPermalinkParams::new("f00b4r", &repo_path("main.rs"), Some(23..47)),
                );

        let expected_url = "https://bitbucket.company.com/projects/zed-industries/repos/zed/browse/main.rs?at=f00b4r#24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_bitbucket_pull_requests() {
        use indoc::indoc;

        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let bitbucket = Bitbucket::public_instance();

        // Test message without PR reference
        let message = "This does not contain a pull request";
        assert!(bitbucket.extract_pull_request(&remote, message).is_none());

        // Pull request number at end of first line
        let message = indoc! {r#"
            Merged in feature-branch (pull request #123)

            Some detailed description of the changes.
        "#};

        let pr = bitbucket.extract_pull_request(&remote, message).unwrap();
        assert_eq!(pr.number, 123);
        assert_eq!(
            pr.url.as_str(),
            "https://bitbucket.org/zed-industries/zed/pull-requests/123"
        );
    }

    #[test]
    fn test_bitbucket_self_hosted_pull_requests() {
        use indoc::indoc;

        let remote = ParsedGitRemote {
            owner: "zed-industries".into(),
            repo: "zed".into(),
        };

        let bitbucket =
            Bitbucket::from_remote_url("https://bitbucket.company.com/zed-industries/zed.git")
                .unwrap();

        // Test message without PR reference
        let message = "This does not contain a pull request";
        assert!(bitbucket.extract_pull_request(&remote, message).is_none());

        // Pull request number at end of first line
        let message = indoc! {r#"
            Merged in feature-branch (pull request #123)

            Some detailed description of the changes.
        "#};

        let pr = bitbucket.extract_pull_request(&remote, message).unwrap();
        assert_eq!(pr.number, 123);
        assert_eq!(
            pr.url.as_str(),
            "https://bitbucket.company.com/projects/zed-industries/repos/zed/pull-requests/123"
        );
    }
}
