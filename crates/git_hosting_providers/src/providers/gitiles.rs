use std::str::FromStr;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    RemoteUrl,
};
use url::Url;

pub struct Gitiles {
    name: String,
    base_url: Url,
}

impl Gitiles {
    pub fn new(name: &str, base_url: Url) -> Self {
        Self {
            name: name.to_string(),
            base_url,
        }
    }

    /// Joins `path` onto the base URL, preserving any existing path prefix.
    ///
    /// `Url::join` cannot be used here because it drops the base path prefix
    /// (e.g. `/plugins/gitiles`) when resolving a relative reference.
    fn build_url(&self, path: &str) -> Url {
        let mut url = self.base_url();
        let base_path = url.path().trim_end_matches('/').to_string();
        url.set_path(&format!("{base_path}/{path}"));
        url
    }
}

impl GitHostingProvider for Gitiles {
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
        format!("{line}")
    }

    fn format_line_numbers(&self, start_line: u32, _end_line: u32) -> String {
        format!("{start_line}")
    }

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        let host = url.host_str()?;
        if host != self.base_url.host_str()? {
            return None;
        }

        let path_segments = url.path_segments()?.collect::<Vec<_>>();
        let joined_path = path_segments.join("/");
        let repo = joined_path.trim_end_matches(".git");

        Some(ParsedGitRemote {
            owner: "".into(),
            repo: repo.into(),
        })
    }

    fn build_commit_permalink(
        &self,
        remote: &ParsedGitRemote,
        params: BuildCommitPermalinkParams,
    ) -> Url {
        let BuildCommitPermalinkParams { sha } = params;
        let ParsedGitRemote { owner: _, repo } = remote;

        self.build_url(&format!("{repo}/+/{sha}"))
    }

    fn build_permalink(&self, remote: ParsedGitRemote, params: BuildPermalinkParams) -> Url {
        let ParsedGitRemote { owner: _, repo } = remote;
        let BuildPermalinkParams {
            sha,
            path,
            selection,
        } = params;

        let mut permalink = self.build_url(&format!("{repo}/+/{sha}/{path}"));
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

    fn android() -> Gitiles {
        Gitiles::new(
            "Android",
            Url::parse("https://android.googlesource.com").unwrap(),
        )
    }

    fn gerrit() -> Gitiles {
        Gitiles::new(
            "Gerrit",
            Url::parse("https://gerrit.googlesource.com").unwrap(),
        )
    }

    fn with_path_prefix() -> Gitiles {
        Gitiles::new(
            "Gitiles",
            Url::parse("https://git.example.com/plugins/gitiles").unwrap(),
        )
    }

    #[test]
    fn test_parse_remote_url_given_https_url() {
        let parsed = android()
            .parse_remote_url("https://android.googlesource.com/platform/build/soong")
            .unwrap();

        assert_eq!(
            parsed,
            ParsedGitRemote {
                owner: "".into(),
                repo: "platform/build/soong".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_ssh_url_with_port() {
        let parsed = gerrit()
            .parse_remote_url("ssh://user@gerrit.googlesource.com:29418/gitiles")
            .unwrap();

        assert_eq!(
            parsed,
            ParsedGitRemote {
                owner: "".into(),
                repo: "gitiles".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_ssh_url_without_port() {
        let parsed = android()
            .parse_remote_url("ssh://user@android.googlesource.com/platform/build/soong")
            .unwrap();

        assert_eq!(
            parsed,
            ParsedGitRemote {
                owner: "".into(),
                repo: "platform/build/soong".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_scp_style_url() {
        let parsed = gerrit()
            .parse_remote_url("git@gerrit.googlesource.com:gitiles.git")
            .unwrap();

        assert_eq!(
            parsed,
            ParsedGitRemote {
                owner: "".into(),
                repo: "gitiles".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_trims_dot_git_suffix() {
        let parsed = android()
            .parse_remote_url("https://android.googlesource.com/platform/build/soong.git")
            .unwrap();

        assert_eq!(
            parsed,
            ParsedGitRemote {
                owner: "".into(),
                repo: "platform/build/soong".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_with_deeply_nested_project() {
        let parsed = android()
            .parse_remote_url("https://android.googlesource.com/platform/frameworks/opt/telephony")
            .unwrap();

        assert_eq!(
            parsed,
            ParsedGitRemote {
                owner: "".into(),
                repo: "platform/frameworks/opt/telephony".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_rejects_other_host() {
        assert!(
            android()
                .parse_remote_url("https://gerrit.googlesource.com/gitiles")
                .is_none()
        );
    }

    #[test]
    fn test_build_gitiles_permalink() {
        let permalink = gerrit().build_permalink(
            ParsedGitRemote {
                owner: "".into(),
                repo: "gitiles".into(),
            },
            BuildPermalinkParams::new(
                "a9409d9529e6f2da746e126dba7a8f4126f01d3e",
                &repo_path("java/com/google/gitiles/CommitData.java"),
                None,
            ),
        );

        let expected_url = "https://gerrit.googlesource.com/gitiles/+/a9409d9529e6f2da746e126dba7a8f4126f01d3e/java/com/google/gitiles/CommitData.java";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitiles_permalink_with_single_line_selection() {
        let permalink = gerrit().build_permalink(
            ParsedGitRemote {
                owner: "".into(),
                repo: "gitiles".into(),
            },
            BuildPermalinkParams::new(
                "a9409d9529e6f2da746e126dba7a8f4126f01d3e",
                &repo_path("java/com/google/gitiles/CommitData.java"),
                Some(16..16),
            ),
        );

        let expected_url = "https://gerrit.googlesource.com/gitiles/+/a9409d9529e6f2da746e126dba7a8f4126f01d3e/java/com/google/gitiles/CommitData.java#17";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitiles_permalink_with_multi_line_selection() {
        let permalink = gerrit().build_permalink(
            ParsedGitRemote {
                owner: "".into(),
                repo: "gitiles".into(),
            },
            BuildPermalinkParams::new(
                "a9409d9529e6f2da746e126dba7a8f4126f01d3e",
                &repo_path("java/com/google/gitiles/GitilesUrls.java"),
                Some(14..30),
            ),
        );

        let expected_url = "https://gerrit.googlesource.com/gitiles/+/a9409d9529e6f2da746e126dba7a8f4126f01d3e/java/com/google/gitiles/GitilesUrls.java#15";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitiles_permalink_with_path_prefix() {
        let permalink = with_path_prefix().build_permalink(
            ParsedGitRemote {
                owner: "".into(),
                repo: "project/my-repo".into(),
            },
            BuildPermalinkParams::new(
                "a9409d9529e6f2da746e126dba7a8f4126f01d3e",
                &repo_path("src/main.rs"),
                Some(9..9),
            ),
        );

        let expected_url = "https://git.example.com/plugins/gitiles/project/my-repo/+/a9409d9529e6f2da746e126dba7a8f4126f01d3e/src/main.rs#10";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitiles_commit_permalink() {
        let permalink = gerrit().build_commit_permalink(
            &ParsedGitRemote {
                owner: "".into(),
                repo: "gitiles".into(),
            },
            BuildCommitPermalinkParams {
                sha: "a9409d9529e6f2da746e126dba7a8f4126f01d3e",
            },
        );

        let expected_url =
            "https://gerrit.googlesource.com/gitiles/+/a9409d9529e6f2da746e126dba7a8f4126f01d3e";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
