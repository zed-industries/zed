use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::{Result, bail};
use regex::Regex;
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
}

impl GitHostingProvider for Bitbucket {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn base_url(&self) -> Url {
        self.base_url.clone()
    }

    fn supports_avatars(&self) -> bool {
        false
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
