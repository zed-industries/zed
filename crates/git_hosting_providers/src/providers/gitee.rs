use url::Url;

use git::{BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote};

pub struct Gitee;

impl GitHostingProvider for Gitee {
    fn name(&self) -> String {
        "Gitee".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://gitee.com").unwrap()
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
        if url.starts_with("git@gitee.com:") || url.starts_with("https://gitee.com/") {
            let repo_with_owner = url
                .trim_start_matches("git@gitee.com:")
                .trim_start_matches("https://gitee.com/")
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
    fn test_build_gitee_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            owner: "libkitten",
            repo: "zed",
        };
        let permalink = Gitee.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_ssh_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "libkitten",
            repo: "zed",
        };
        let permalink = Gitee.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_ssh_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "libkitten",
            repo: "zed",
        };
        let permalink = Gitee.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url() {
        let remote = ParsedGitRemote {
            owner: "libkitten",
            repo: "zed",
        };
        let permalink = Gitee.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                path: "crates/zed/src/main.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "libkitten",
            repo: "zed",
        };
        let permalink = Gitee.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                path: "crates/zed/src/main.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "libkitten",
            repo: "zed",
        };
        let permalink = Gitee.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                path: "crates/zed/src/main.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
