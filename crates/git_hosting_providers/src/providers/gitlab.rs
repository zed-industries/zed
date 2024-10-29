use url::Url;

use git::{BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote};

pub struct Gitlab;

impl Gitlab {
    fn is_gitlab_instance(&self, host: &str) -> bool {
        host == "gitlab.com" || host.contains("gitlab")
    }

    fn extract_base_url(&self, url: &str) -> Option<Url> {
        if url.starts_with("git@") {
            let ssh_url = url.strip_prefix("git@")?;
            let (host, _) = ssh_url.split_once(':')?;

            if !self.is_gitlab_instance(host) {
                return None;
            }

            return Url::parse(&format!("https://{}", host)).ok();
        }

        if !url.starts_with("https://") {
            return None;
        }

        let url_obj = Url::parse(url).ok()?;
        let host = url_obj.host_str()?;

        if !self.is_gitlab_instance(host) {
            return None;
        }

        Some(url_obj.join("/").ok()?)
    }
}

impl GitHostingProvider for Gitlab {
    fn name(&self) -> String {
        "GitLab".to_string()
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
        let base_url = self.extract_base_url(url)?;
        let host = base_url.host_str()?;

        let repo_with_owner = if url.starts_with("git@") {
            let prefix = format!("git@{}:", host);
            let without_prefix = url.strip_prefix(&prefix)?;
            without_prefix
                .strip_suffix(".git")
                .unwrap_or(without_prefix)
        } else {
            let without_prefix = url.strip_prefix(base_url.as_str())?;
            let without_leading_slash = without_prefix.trim_start_matches('/');
            without_leading_slash
                .strip_suffix(".git")
                .unwrap_or(without_leading_slash)
        };

        let (owner, repo) = repo_with_owner.split_once('/')?;
        if owner.is_empty() || repo.is_empty() {
            return None;
        }

        Some(ParsedGitRemote {
            base_url,
            owner,
            repo,
        })
    }

    fn build_commit_permalink(
        &self,
        remote: &ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url {
        let BuildCommitPermalinkParams { sha } = params;
        let ParsedGitRemote {
            owner,
            repo,
            base_url,
        } = remote;

        base_url
            .join(&format!("{owner}/{repo}/-/commit/{sha}"))
            .unwrap()
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote {
            owner,
            repo,
            base_url,
        } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
        } = params;

        let mut permalink = base_url
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
    use super::*;

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url() {
        let remote = Gitlab
            .parse_remote_url("git@gitlab.com:zed-industries/zed.git")
            .unwrap();
        let permalink = Gitlab.build_permalink(
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
        let remote = Gitlab
            .parse_remote_url("git@gitlab.com:zed-industries/zed.git")
            .unwrap();
        let permalink = Gitlab.build_permalink(
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
        let remote = Gitlab
            .parse_remote_url("git@gitlab.com:zed-industries/zed.git")
            .unwrap();
        let permalink = Gitlab.build_permalink(
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
        let remote = Gitlab
            .parse_remote_url("https://gitlab.com/zed-industries/zed")
            .unwrap();
        let permalink = Gitlab.build_permalink(
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
        let remote = Gitlab
            .parse_remote_url("https://gitlab.com/zed-industries/zed")
            .unwrap();
        let permalink = Gitlab.build_permalink(
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
        let remote = Gitlab
            .parse_remote_url("https://gitlab.com/zed-industries/zed")
            .unwrap();
        let permalink = Gitlab.build_permalink(
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
    fn test_self_hosted_gitlab_ssh_url() {
        let parsed = Gitlab
            .parse_remote_url("git@gitlab.zed.dev:zed-industries/zed.git")
            .unwrap();
        assert_eq!(parsed.owner, "zed-industries");
        assert_eq!(parsed.repo, "zed");

        let permalink = Gitlab.build_permalink(
            parsed,
            BuildPermalinkParams {
                sha: "123456",
                path: "src/main.rs",
                selection: None,
            },
        );
        assert_eq!(
            permalink.to_string(),
            "https://gitlab.zed.dev/zed-industries/zed/-/blob/123456/src/main.rs"
        );
    }

    #[test]
    fn test_self_hosted_gitlab_https_url() {
        let parsed = Gitlab
            .parse_remote_url("https://gitlab.zed.dev/zed-industries/zed")
            .unwrap();
        assert_eq!(parsed.owner, "zed-industries");
        assert_eq!(parsed.repo, "zed");

        let permalink = Gitlab.build_permalink(
            parsed,
            BuildPermalinkParams {
                sha: "123456",
                path: "src/main.rs",
                selection: None,
            },
        );
        assert_eq!(
            permalink.to_string(),
            "https://gitlab.zed.dev/zed-industries/zed/-/blob/123456/src/main.rs"
        );
    }
}
