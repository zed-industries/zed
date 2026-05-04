use std::str::FromStr;

use anyhow::{Result, bail};
use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    RemoteUrl,
};

use crate::get_host_from_git_remote_url;

pub struct SourceHut {
    name: String,
    base_url: Url,
}

impl SourceHut {
    pub fn new(name: &str, base_url: Url) -> Self {
        Self {
            name: name.to_string(),
            base_url,
        }
    }

    pub fn public_instance() -> Self {
        Self::new("SourceHut", Url::parse("https://git.sr.ht").unwrap())
    }

    pub fn from_remote_url(remote_url: &str) -> Result<Self> {
        let host = get_host_from_git_remote_url(remote_url)?;
        if host == "git.sr.ht" {
            bail!("the SourceHut instance is not self-hosted");
        }

        // TODO: detecting self hosted instances by checking whether "sourcehut" is in the url or not
        // is not very reliable. See https://github.com/zed-industries/zed/issues/26393 for more
        // information.
        if !host.contains("sourcehut") {
            bail!("not a SourceHut URL");
        }

        Ok(Self::new(
            "SourceHut Self-Hosted",
            Url::parse(&format!("https://{}", host))?,
        ))
    }
}

impl GitHostingProvider for SourceHut {
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

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        let host = url.host_str()?;
        if host != self.base_url.host_str()? {
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
    use git::repository::repo_path;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_remote_url_given_ssh_url() {
        let parsed_remote = SourceHut::public_instance()
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
        let parsed_remote = SourceHut::public_instance()
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
        let parsed_remote = SourceHut::public_instance()
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
    fn test_parse_remote_url_given_self_hosted_ssh_url() {
        let remote_url = "git@sourcehut.org:~zed-industries/zed";

        let parsed_remote = SourceHut::from_remote_url(remote_url)
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
    fn test_parse_remote_url_given_self_hosted_ssh_url_with_git_suffix() {
        let remote_url = "git@sourcehut.org:~zed-industries/zed.git";

        let parsed_remote = SourceHut::from_remote_url(remote_url)
            .unwrap()
            .parse_remote_url(remote_url)
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
    fn test_parse_remote_url_given_self_hosted_https_url() {
        let remote_url = "https://sourcehut.org/~zed-industries/zed";

        let parsed_remote = SourceHut::from_remote_url(remote_url)
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
    fn test_build_sourcehut_permalink() {
        let permalink = SourceHut::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://git.sr.ht/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_with_git_suffix() {
        let permalink = SourceHut::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed.git".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://git.sr.ht/~zed-industries/zed.git/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_self_hosted_permalink() {
        let permalink = SourceHut::from_remote_url("https://sourcehut.org/~zed-industries/zed")
            .unwrap()
            .build_permalink(
                ParsedGitRemote {
                    owner: "zed-industries".into(),
                    repo: "zed".into(),
                },
                BuildPermalinkParams::new(
                    "faa6f979be417239b2e070dbbf6392b909224e0b",
                    &repo_path("crates/editor/src/git/permalink.rs"),
                    None,
                ),
            );

        let expected_url = "https://sourcehut.org/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_self_hosted_permalink_with_git_suffix() {
        let permalink = SourceHut::from_remote_url("https://sourcehut.org/~zed-industries/zed.git")
            .unwrap()
            .build_permalink(
                ParsedGitRemote {
                    owner: "zed-industries".into(),
                    repo: "zed.git".into(),
                },
                BuildPermalinkParams::new(
                    "faa6f979be417239b2e070dbbf6392b909224e0b",
                    &repo_path("crates/editor/src/git/permalink.rs"),
                    None,
                ),
            );

        let expected_url = "https://sourcehut.org/~zed-industries/zed.git/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_with_single_line_selection() {
        let permalink = SourceHut::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(6..6),
            ),
        );

        let expected_url = "https://git.sr.ht/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_permalink_with_multi_line_selection() {
        let permalink = SourceHut::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(23..47),
            ),
        );

        let expected_url = "https://git.sr.ht/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_self_hosted_permalink_with_single_line_selection() {
        let permalink = SourceHut::from_remote_url("https://sourcehut.org/~zed-industries/zed")
            .unwrap()
            .build_permalink(
                ParsedGitRemote {
                    owner: "zed-industries".into(),
                    repo: "zed".into(),
                },
                BuildPermalinkParams::new(
                    "faa6f979be417239b2e070dbbf6392b909224e0b",
                    &repo_path("crates/editor/src/git/permalink.rs"),
                    Some(6..6),
                ),
            );

        let expected_url = "https://sourcehut.org/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_sourcehut_self_hosted_permalink_with_multi_line_selection() {
        let permalink = SourceHut::from_remote_url("https://sourcehut.org/~zed-industries/zed")
            .unwrap()
            .build_permalink(
                ParsedGitRemote {
                    owner: "zed-industries".into(),
                    repo: "zed".into(),
                },
                BuildPermalinkParams::new(
                    "faa6f979be417239b2e070dbbf6392b909224e0b",
                    &repo_path("crates/editor/src/git/permalink.rs"),
                    Some(23..47),
                ),
            );

        let expected_url = "https://sourcehut.org/~zed-industries/zed/tree/faa6f979be417239b2e070dbbf6392b909224e0b/item/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
