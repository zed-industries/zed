use url::Url;

use git::{BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote};

pub struct Gitlab;

impl GitHostingProvider for Gitlab {
    fn name(&self) -> String {
        "GitLab".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://gitlab.com").unwrap()
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
        if url.starts_with("git@gitlab.com:") || url.starts_with("https://gitlab.com/") {
            let repo_with_owner = url
                .trim_start_matches("git@gitlab.com:")
                .trim_start_matches("https://gitlab.com/")
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
    use super::*;

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
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
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
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
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
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
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
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
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
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
        let remote = ParsedGitRemote {
            owner: "zed-industries",
            repo: "zed",
        };
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
}
