use std::str::FromStr;

use anyhow::{Result, bail};
use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    RemoteUrl,
};

use crate::get_host_from_git_remote_url;

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
        //
        // For now, we allow any self-hosted instance that isn't gitlab.com to be treated as GitLab.
        // This enables auto-detection for most GitLab instances, but users can still configure
        // specific instances in settings for better control and to avoid false positives.

        Ok(Self::new(
            "GitLab Self-Hosted",
            Url::parse(&format!("https://{}", host))?,
        ))
    }

    /// Creates a GitLab provider for a configured host.
    /// This bypasses the hostname heuristics and can be used for any host.
    pub fn from_configured_host(name: impl Into<String>, host: &str) -> Result<Self> {
        if host == "gitlab.com" {
            return Ok(Self::public_instance());
        }

        Ok(Self::new(
            name,
            Url::parse(&format!("https://{}", host))?,
        ))
    }
}

impl GitHostingProvider for Gitlab {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn base_url(&self) -> &Url {
        &self.base_url
    }

    fn parse_remote_url(&self, remote_url: &str) -> Option<ParsedGitRemote> {
        let remote_url = RemoteUrl::parse(remote_url)?;
        if remote_url.host() != self.base_url.host()? {
            return None;
        }

        let path = remote_url.path();
        let path = path.strip_prefix('/').unwrap_or(path);
        let path = path.strip_suffix(".git").unwrap_or(path);

        // GitLab supports subgroups.
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() < 2 {
            return None;
        }

        let repo = parts.last()?;
        let owner = parts[..parts.len() - 1].join("/");

        Some(ParsedGitRemote {
            owner: owner.into(),
            repo: (*repo).into(),
        })
    }

    fn build_permalink(
        &self,
        remote: ParsedGitRemote,
        params: BuildPermalinkParams,
    ) -> Url {
        let mut permalink = self.base_url.clone();
        permalink
            .path_segments_mut()
            .unwrap()
            .push(&remote.owner)
            .push(&remote.repo)
            .push("-")
            .push("blob")
            .push(params.sha)
            .push(params.path);

        if let Some(selection) = params.selection {
            if selection.start == selection.end {
                permalink
                    .set_fragment(Some(&format!("L{}", selection.start + 1)));
            } else {
                permalink.set_fragment(Some(&format!(
                    "L{}-{}",
                    selection.start + 1,
                    selection.end + 1
                )));
            }
        }

        permalink
    }

    fn build_commit_permalink(
        &self,
        remote: ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url {
        let mut permalink = self.base_url.clone();
        permalink
            .path_segments_mut()
            .unwrap()
            .push(&remote.owner)
            .push(&remote.repo)
            .push("-")
            .push("commit")
            .push(params.sha);

        permalink
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_invalid_self_hosted_remote_url() {
        let remote_url = "https://gitlab.com/zed-industries/zed.git";
        let github = Gitlab::from_remote_url(remote_url);
        assert!(github.is_err());
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
    fn test_build_gitlab_permalink_with_single_line_selection() {
        let permalink = Gitlab::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
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
    fn test_build_gitlab_permalink_with_multi_line_selection() {
        let permalink = Gitlab::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
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
    fn test_build_gitlab_self_hosted_permalink_from_ssh_url() {
        let gitlab =
            Gitlab::from_remote_url("git@gitlab.some-enterprise.com:zed-industries/zed.git")
                .unwrap();
        let permalink = gitlab.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitlab.some-enterprise.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
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
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitlab-instance.big-co.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_from_configured_host_custom_instance() {
        let gitlab = Gitlab::from_configured_host("Custom GitLab", "git.tdd.io").unwrap();
        assert_eq!(gitlab.name(), "Custom GitLab");
        assert_eq!(gitlab.base_url.as_str(), "https://git.tdd.io/");
    }

    #[test]
    fn test_from_configured_host_public_instance() {
        let gitlab = Gitlab::from_configured_host("GitLab", "gitlab.com").unwrap();
        assert_eq!(gitlab.name(), "GitLab");
        assert_eq!(gitlab.base_url.as_str(), "https://gitlab.com/");
    }

    #[test]
    fn test_custom_gitlab_instance_parse_remote() {
        let gitlab = Gitlab::from_configured_host("Custom GitLab", "git.tdd.io").unwrap();
        let parsed_remote = gitlab
            .parse_remote_url("git@git.tdd.io:engine/engine-api.git")
            .unwrap();
        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "engine".into(),
                repo: "engine-api".into(),
            }
        );
    }

    #[test]
    fn test_from_remote_url_auto_detection_works() {
        // Now that we removed the hardcoded hostname check, any non-gitlab.com host should work
        let result = Gitlab::from_remote_url("git@git.tdd.io:engine/engine-api.git");
        assert!(result.is_ok());
        let gitlab = result.unwrap();
        assert_eq!(gitlab.name(), "GitLab Self-Hosted");
        assert_eq!(gitlab.base_url.as_str(), "https://git.tdd.io/");
    }
}
