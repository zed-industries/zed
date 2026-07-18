use std::str::FromStr;
use std::sync::LazyLock;

use async_trait::async_trait;
use regex::Regex;
use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    PullRequest, RemoteUrl,
};

fn pull_request_regex() -> &'static Regex {
    static PULL_REQUEST_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^Merged PR (\d+):").unwrap());
    &PULL_REQUEST_REGEX
}

#[derive(Debug)]
pub struct Azure;

impl Azure {
    fn parse_dev_azure_com_url(&self, url: &RemoteUrl) -> Option<ParsedGitRemote> {
        let host = url.host_str()?;

        if host == "ssh.dev.azure.com" {
            // SSH format: git@ssh.dev.azure.com:v3/{organization}/{project}/{repo}
            let mut path_segments = url.path_segments()?;
            let _v3 = path_segments.next()?;
            let organization = path_segments.next()?;
            let project = path_segments.next()?;
            let repo = path_segments.next()?.trim_end_matches(".git");

            return Some(ParsedGitRemote {
                owner: format!("{organization}/{project}").into(),
                repo: repo.into(),
            });
        }

        if host != "dev.azure.com" {
            return None;
        }

        // HTTPS format: https://dev.azure.com/{organization}/{project}/_git/{repo}
        // or: https://{organization}@dev.azure.com/{organization}/{project}/_git/{repo}
        let mut path_segments = url.path_segments()?;
        let organization = path_segments.next()?;
        let project = path_segments.next()?;
        let _git = path_segments.next()?;
        let repo = path_segments.next()?.trim_end_matches(".git");

        Some(ParsedGitRemote {
            owner: format!("{organization}/{project}").into(),
            repo: repo.into(),
        })
    }

    fn parse_visualstudio_com_url(&self, url: &RemoteUrl) -> Option<ParsedGitRemote> {
        let host = url.host_str()?;

        if !host.ends_with(".visualstudio.com") {
            return None;
        }

        let organization = host.strip_suffix(".visualstudio.com")?;

        // HTTPS format: https://{organization}.visualstudio.com/{project}/_git/{repo}
        // or with DefaultCollection: https://{organization}.visualstudio.com/DefaultCollection/{project}/_git/{repo}
        let mut path_segments = url.path_segments()?.peekable();

        let first_segment = path_segments.next()?;
        let project = if first_segment == "DefaultCollection" {
            path_segments.next()?
        } else {
            first_segment
        };

        let _git = path_segments.next()?;
        let repo = path_segments.next()?.trim_end_matches(".git");

        Some(ParsedGitRemote {
            owner: format!("{organization}/{project}").into(),
            repo: repo.into(),
        })
    }
}

#[async_trait]
impl GitHostingProvider for Azure {
    fn name(&self) -> String {
        "Azure DevOps".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://dev.azure.com").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        false
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("line={line}&lineEnd={line}&lineStartColumn=1&lineEndColumn=1")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("line={start_line}&lineEnd={end_line}&lineStartColumn=1&lineEndColumn=1")
    }

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        self.parse_dev_azure_com_url(&url)
            .or_else(|| self.parse_visualstudio_com_url(&url))
    }

    fn build_commit_permalink(
        &self,
        remote: &ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url {
        let BuildCommitPermalinkParams { sha } = params;
        let ParsedGitRemote { owner, repo } = remote;

        let mut url = self
            .base_url()
            .join(&format!("{owner}/_git/{repo}/commit/{sha}"))
            .expect("failed to build commit permalink");
        url.set_query(None);
        url
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
            .join(&format!("{owner}/_git/{repo}"))
            .expect("failed to build permalink");

        let mut query = format!("path=/{path}&version=GC{sha}");
        if let Some(selection) = selection {
            query.push('&');
            query.push_str(&self.line_fragment(&selection));
        }
        permalink.set_query(Some(&query));

        permalink
    }

    fn build_create_pull_request_url(
        &self,
        remote: &ParsedGitRemote,
        source_branch: &str,
    ) -> Option<Url> {
        let ParsedGitRemote { owner, repo } = remote;
        let encoded_source = urlencoding::encode(source_branch);

        let mut url = self
            .base_url()
            .join(&format!("{owner}/_git/{repo}/pullrequestcreate"))
            .ok()?;
        url.set_query(Some(&format!("sourceRef={encoded_source}")));
        Some(url)
    }

    fn extract_pull_request(&self, remote: &ParsedGitRemote, message: &str) -> Option<PullRequest> {
        let first_line = message.lines().next()?;
        let capture = pull_request_regex().captures(first_line)?;
        let number = capture.get(1)?.as_str().parse::<u32>().ok()?;

        let ParsedGitRemote { owner, repo } = remote;
        let url = self
            .base_url()
            .join(&format!("{owner}/_git/{repo}/pullrequest/{number}"))
            .ok()?;

        Some(PullRequest { number, url })
    }
}

#[cfg(test)]
mod tests {
    use git::repository::repo_path;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_remote_url_https_dev_azure() {
        let parsed_remote = Azure
            .parse_remote_url("https://dev.azure.com/myorg/myproject/_git/myrepo")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_https_dev_azure_with_username() {
        let parsed_remote = Azure
            .parse_remote_url("https://myorg@dev.azure.com/myorg/myproject/_git/myrepo")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_ssh_dev_azure() {
        let parsed_remote = Azure
            .parse_remote_url("git@ssh.dev.azure.com:v3/myorg/myproject/myrepo")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_visualstudio_com() {
        let parsed_remote = Azure
            .parse_remote_url("https://myorg.visualstudio.com/myproject/_git/myrepo")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_visualstudio_com_with_default_collection() {
        let parsed_remote = Azure
            .parse_remote_url(
                "https://myorg.visualstudio.com/DefaultCollection/myproject/_git/myrepo",
            )
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_returns_none_for_github() {
        let result = Azure.parse_remote_url("https://github.com/owner/repo.git");
        assert!(result.is_none());
    }

    #[test]
    fn test_build_azure_permalink() {
        let permalink = Azure.build_permalink(
            ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            },
            BuildPermalinkParams::new("abc123def456", &repo_path("src/main.rs"), None),
        );

        let expected_url = "https://dev.azure.com/myorg/myproject/_git/myrepo?path=/src/main.rs&version=GCabc123def456";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_azure_permalink_with_single_line_selection() {
        let permalink = Azure.build_permalink(
            ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            },
            BuildPermalinkParams::new("abc123def456", &repo_path("src/main.rs"), Some(6..6)),
        );

        let expected_url = "https://dev.azure.com/myorg/myproject/_git/myrepo?path=/src/main.rs&version=GCabc123def456&line=7&lineEnd=7&lineStartColumn=1&lineEndColumn=1";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_azure_permalink_with_multi_line_selection() {
        let permalink = Azure.build_permalink(
            ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            },
            BuildPermalinkParams::new("abc123def456", &repo_path("src/main.rs"), Some(23..47)),
        );

        let expected_url = "https://dev.azure.com/myorg/myproject/_git/myrepo?path=/src/main.rs&version=GCabc123def456&line=24&lineEnd=48&lineStartColumn=1&lineEndColumn=1";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_azure_commit_permalink() {
        let permalink = Azure.build_commit_permalink(
            &ParsedGitRemote {
                owner: "myorg/myproject".into(),
                repo: "myrepo".into(),
            },
            BuildCommitPermalinkParams {
                sha: "abc123def456",
            },
        );

        let expected_url = "https://dev.azure.com/myorg/myproject/_git/myrepo/commit/abc123def456";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_azure_create_pr_url() {
        let remote = ParsedGitRemote {
            owner: "myorg/myproject".into(),
            repo: "myrepo".into(),
        };

        let url = Azure
            .build_create_pull_request_url(&remote, "feature/my-branch")
            .expect("url should be constructed");

        assert_eq!(
            url.as_str(),
            "https://dev.azure.com/myorg/myproject/_git/myrepo/pullrequestcreate?sourceRef=feature%2Fmy-branch"
        );
    }

    #[test]
    fn test_azure_extract_pull_request() {
        use indoc::indoc;

        let remote = ParsedGitRemote {
            owner: "myorg/myproject".into(),
            repo: "myrepo".into(),
        };

        let message = "This does not contain a pull request";
        assert!(Azure.extract_pull_request(&remote, message).is_none());

        let message = indoc! {r#"
            Merged PR 123: Add new feature

            This PR adds a new feature to the application.
        "#};

        let pull_request = Azure.extract_pull_request(&remote, message).unwrap();
        assert_eq!(pull_request.number, 123);
        assert_eq!(
            pull_request.url.as_str(),
            "https://dev.azure.com/myorg/myproject/_git/myrepo/pullrequest/123"
        );

        let message = "Merged PR 456: Fix bug in authentication";
        let pull_request = Azure.extract_pull_request(&remote, message).unwrap();
        assert_eq!(pull_request.number, 456);
        assert_eq!(
            pull_request.url.as_str(),
            "https://dev.azure.com/myorg/myproject/_git/myrepo/pullrequest/456"
        );

        let message = "This mentions PR 789 but not at the start";
        assert!(Azure.extract_pull_request(&remote, message).is_none());
    }
}
