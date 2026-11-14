use std::str::FromStr;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    RemoteUrl,
};
use url::Url;

pub struct Tangled {
    name: String,
    base_url: Url,
}

impl Tangled {
    pub fn new(name: &str, base_url: Url) -> Self {
        Self {
            name: name.to_string(),
            base_url,
        }
    }

    pub fn public_instance() -> Self {
        Self::new("Tangled", Url::parse("https://tangled.org").unwrap())
    }

    /// Joins the given path to the base URL.
    ///
    /// The path is rooted with a leading `/`, as Tangled owners may be AT
    /// Protocol DIDs (e.g., `did:plc:abc123`). Passing a path starting with
    /// `did:` to `Url::join` would otherwise be interpreted as an absolute URL
    /// with a `did:` scheme.
    fn join_path(&self, path: &str) -> Url {
        self.base_url().join(&format!("/{path}")).unwrap()
    }
}

impl GitHostingProvider for Tangled {
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
        // The owner is either a handle (e.g., `user.tngl.sh`, sometimes
        // prefixed with `@` in web URLs) or an AT Protocol DID (e.g.,
        // `did:plc:abc123`).
        let owner = path_segments.next()?.trim_start_matches('@');
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

        self.join_path(&format!("{owner}/{repo}/commit/{sha}"))
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote { owner, repo } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
        } = params;

        let mut permalink = self.join_path(&format!("{owner}/{repo}/blob/{sha}/{path}"));
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
        let parsed_remote = Tangled::public_instance()
            .parse_remote_url("git@tangled.org:user.tngl.sh/zed")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "user.tngl.sh".into(),
                repo: "zed".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_ssh_url_with_did_owner() {
        let parsed_remote = Tangled::public_instance()
            .parse_remote_url("git@tangled.org:did:plc:j5hmlfdrwkvtxm7cjmu7j2is/core")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "did:plc:j5hmlfdrwkvtxm7cjmu7j2is".into(),
                repo: "core".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_https_url() {
        let parsed_remote = Tangled::public_instance()
            .parse_remote_url("https://tangled.org/user.tngl.sh/zed")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "user.tngl.sh".into(),
                repo: "zed".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_https_url_with_at_prefixed_owner() {
        let parsed_remote = Tangled::public_instance()
            .parse_remote_url("https://tangled.org/@user.tngl.sh/zed")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "user.tngl.sh".into(),
                repo: "zed".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_https_url_with_did_owner() {
        let parsed_remote = Tangled::public_instance()
            .parse_remote_url("https://tangled.org/did:plc:j5hmlfdrwkvtxm7cjmu7j2is/core")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "did:plc:j5hmlfdrwkvtxm7cjmu7j2is".into(),
                repo: "core".into(),
            }
        );
    }

    #[test]
    fn test_build_tangled_commit_permalink() {
        let permalink = Tangled::public_instance().build_commit_permalink(
            &ParsedGitRemote {
                owner: "user.tngl.sh".into(),
                repo: "zed".into(),
            },
            BuildCommitPermalinkParams {
                sha: "faa6f979be417239b2e070dbbf6392b909224e0b",
            },
        );

        let expected_url =
            "https://tangled.org/user.tngl.sh/zed/commit/faa6f979be417239b2e070dbbf6392b909224e0b";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_tangled_permalink() {
        let permalink = Tangled::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "user.tngl.sh".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://tangled.org/user.tngl.sh/zed/blob/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_tangled_permalink_with_did_owner() {
        let permalink = Tangled::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "did:plc:j5hmlfdrwkvtxm7cjmu7j2is".into(),
                repo: "core".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                None,
            ),
        );

        let expected_url = "https://tangled.org/did:plc:j5hmlfdrwkvtxm7cjmu7j2is/core/blob/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_tangled_permalink_with_single_line_selection() {
        let permalink = Tangled::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "user.tngl.sh".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(6..6),
            ),
        );

        let expected_url = "https://tangled.org/user.tngl.sh/zed/blob/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_tangled_permalink_with_multi_line_selection() {
        let permalink = Tangled::public_instance().build_permalink(
            ParsedGitRemote {
                owner: "user.tngl.sh".into(),
                repo: "zed".into(),
            },
            BuildPermalinkParams::new(
                "faa6f979be417239b2e070dbbf6392b909224e0b",
                &repo_path("crates/editor/src/git/permalink.rs"),
                Some(23..47),
            ),
        );

        let expected_url = "https://tangled.org/user.tngl.sh/zed/blob/faa6f979be417239b2e070dbbf6392b909224e0b/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
