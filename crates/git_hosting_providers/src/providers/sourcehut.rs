use url::Url;

use git::{BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote};

pub struct Sourcehut;

impl GitHostingProvider for Sourcehut {
    fn name(&self) -> String {
        "Gitee".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://git.sr.ht").unwrap()
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
        if url.starts_with("git@git.sr.ht:") || url.starts_with("https://git.sr.ht/") {
            // sourcehut indicates a repo with '.git' suffix as a separate repo.
            // For example, "git@git.sr.ht:~username/repo" and "git@git.sr.ht:~username/repo.git"
            // are two distinct repositories.
            let repo_with_owner = url
                .trim_start_matches("git@git.sr.ht:~")
                .trim_start_matches("https://git.sr.ht/~");

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
            .join(&format!("~{owner}/{repo}/commit/{sha}"))
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
            .join(&format!("~{owner}/{repo}/tree/{sha}/item/{path}"))
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
    fn test_build_sourcehut_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Sourcehut.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_ssh_url_with_git_prefix() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed.git",
        };
        let permalink = Sourcehut.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed.git/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_ssh_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Sourcehut.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_ssh_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Sourcehut.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_https_url() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Sourcehut.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/zed/src/main.rs",
                selection: None,
            },
        );

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_https_url_single_line_selection() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Sourcehut.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/zed/src/main.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_from_https_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            owner: "rajveermalviya",
            repo: "zed",
        };
        let permalink = Sourcehut.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/zed/src/main.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://git.sr.ht/~rajveermalviya/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
