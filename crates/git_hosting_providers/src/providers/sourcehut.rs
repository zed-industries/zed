use std::str::FromStr;

use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    RemoteUrl,
};

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

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        let host = url.host_str()?;
        if host != "git.sr.ht" {
            return None;
        }

        let mut path_segments = url.path_segments()?;
        let owner = path_segments.next()?.trim_start_matches('~');
        // We don't trim the `.git` suffix here like we do elsewhere, as
        // sourcehut treats a repo with `.git` suffix as a separate repo.
        //
        // For example, `git@git.sr.ht:~username/repo` and `git@git.sr.ht:~username/repo.git`
        // are two distinct repositories.
        let repo = path_segments.next()?;

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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_remote_url_given_ssh_url() {
        let parsed_remote = Sourcehut
            .parse_remote_url("git@git.sr.ht:~zed-industries/zed")
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
    fn test_parse_remote_url_given_ssh_url_with_git_suffix() {
        let parsed_remote = Sourcehut
            .parse_remote_url("git@git.sr.ht:~zed-industries/zed.git")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed.git".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_https_url() {
        let parsed_remote = Sourcehut
            .parse_remote_url("https://git.sr.ht/~zed-industries/zed")
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
    fn test_build_sourcehut_permalink() {
        let permalink = Sourcehut.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://git.sr.ht/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_with_git_suffix() {
        let permalink = Sourcehut.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed.git".into(),
            },
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://git.sr.ht/~zed-industries/zed.git/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_with_single_line_selection() {
        let permalink = Sourcehut.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://git.sr.ht/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_with_multi_line_selection() {
        let permalink = Sourcehut.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://git.sr.ht/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
