use anyhow::{anyhow, bail, Result};
use url::Url;
use util::maybe;

use git::{BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote};

#[derive(Debug)]
pub struct Gitlab {
    name: String,
    base_url: Url,
}

impl Gitlab {
    pub fn new() -> Self {
        Self {
            name: "GitLab".to_string(),
            base_url: Url::parse("https://gitlab.com").unwrap(),
        }
    }

    pub fn from_remote_url(remote_url: &str) -> Result<Self> {
        let host = maybe!({
            if let Some(remote_url) = remote_url.strip_prefix("git@") {
                if let Some((host, _)) = remote_url.trim_start_matches("git@").split_once(':') {
                    return Some(host.to_string());
                }
            }

            Url::parse(&remote_url)
                .ok()
                .and_then(|remote_url| remote_url.host_str().map(|host| host.to_string()))
        })
        .ok_or_else(|| anyhow!("URL has no host"))?;

        if !host.contains("gitlab") {
            bail!("not a GitLab URL");
        }

        Ok(Self {
            name: "GitLab Self-Hosted".to_string(),
            base_url: Url::parse(&format!("https://{}", host))?,
        })
    }
}

impl GitHostingProvider for Gitlab {
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
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-{end_line}")
    }

    fn parse_remote_url<'a>(&self, url: &'a str) -> Option<ParsedGitRemote<'a>> {
        let host = self.base_url.host_str()?;

        if url.starts_with(&format!("git@{host}")) || url.starts_with(&format!("https://{host}/")) {
            let repo_with_owner = url
                .trim_start_matches(&format!("git@{host}:"))
                .trim_start_matches(&format!("https://{host}/"))
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
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = Gitlab::new().build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = Gitlab::new().build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = Gitlab::new().build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = Gitlab::new().build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = Gitlab::new().build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = Gitlab::new().build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_self_hosted_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
        let gitlab =
            Gitlab::from_remote_url("git@gitlab.some-enterprise.com:zed-industries/zed.git")
                .unwrap();
        let permalink = gitlab.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitlab.some-enterprise.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_self_hosted_permalink_from_https_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
        let gitlab =
            Gitlab::from_remote_url("https://gitlab-instance.big-co.com/zed-industries/zed.git")
                .unwrap();
        let permalink = gitlab.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitlab-instance.big-co.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
