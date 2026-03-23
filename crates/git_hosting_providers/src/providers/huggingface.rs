use std::str::FromStr;

use async_trait::async_trait;
use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    RemoteUrl,
};

pub struct HuggingFace;

impl HuggingFace {
    fn parse_remote_url_inner(&self, url: &str) -> Option<ParsedGitRemote> {
        let url = RemoteUrl::from_str(url).ok()?;

        let host = url.host_str()?;
        if host != "huggingface.co" && host != "hf.co" {
            return None;
        }

        let mut path_segments = url.path_segments()?;
        let first = path_segments.next()?;

        // Repos can be prefixed with a type: datasets/ or spaces/
        let (owner, repo) = if first == "datasets" || first == "spaces" {
            let owner = path_segments.next()?;
            let repo = path_segments.next()?.trim_end_matches(".git");
            let prefixed_owner = format!("{first}/{owner}");
            (prefixed_owner, repo.to_string())
        } else {
            let repo = path_segments.next()?.trim_end_matches(".git");
            (first.to_string(), repo.to_string())
        };

        if owner.is_empty() || repo.is_empty() {
            return None;
        }

        Some(ParsedGitRemote {
            owner: owner.into(),
            repo: repo.into(),
        })
    }
}

#[async_trait]
impl GitHostingProvider for HuggingFace {
    fn name(&self) -> String {
        "Hugging Face".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://huggingface.co").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        false
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-L{end_line}")
    }

    fn parse_remote_url(&self, url: &str) -> Option<ParsedGitRemote> {
        self.parse_remote_url_inner(url)
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
    use git::repository::repo_path;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_remote_url_given_https_url() {
        let parsed_remote = HuggingFace
            .parse_remote_url("https://huggingface.co/zed-industries/zeta")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zeta".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_https_url_with_dotgit() {
        let parsed_remote = HuggingFace
            .parse_remote_url("https://huggingface.co/zed-industries/zeta.git")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zeta".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_ssh_url() {
        let parsed_remote = HuggingFace
            .parse_remote_url("git@hf.co:zed-industries/zeta.git")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zeta".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_ssh_url_huggingface_host() {
        let parsed_remote = HuggingFace
            .parse_remote_url("git@huggingface.co:zed-industries/zeta.git")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zeta".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_dataset_https_url() {
        let parsed_remote = HuggingFace
            .parse_remote_url("https://huggingface.co/datasets/squad/squad")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "datasets/squad".into(),
                repo: "squad".into(),
            }
        );
    }

    #[test]
    fn test_parse_remote_url_given_space_ssh_url() {
        let parsed_remote = HuggingFace
            .parse_remote_url("git@hf.co:spaces/user/my-app.git")
            .unwrap();

        assert_eq!(
            parsed_remote,
            ParsedGitRemote {
                owner: "spaces/user".into(),
                repo: "my-app".into(),
            }
        );
    }

    #[test]
    fn test_build_huggingface_permalink() {
        let permalink = HuggingFace.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zeta".into(),
            },
            BuildPermalinkParams::new(
                "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                &repo_path("src/main.py"),
                None,
            ),
        );

        let expected_url = "https://huggingface.co/zed-industries/zeta/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/src/main.py";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_huggingface_permalink_with_single_line_selection() {
        let permalink = HuggingFace.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zeta".into(),
            },
            BuildPermalinkParams::new(
                "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                &repo_path("src/main.py"),
                Some(6..6),
            ),
        );

        let expected_url = "https://huggingface.co/zed-industries/zeta/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/src/main.py#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_huggingface_permalink_with_multi_line_selection() {
        let permalink = HuggingFace.build_permalink(
            ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zeta".into(),
            },
            BuildPermalinkParams::new(
                "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
                &repo_path("src/main.py"),
                Some(23..47),
            ),
        );

        let expected_url = "https://huggingface.co/zed-industries/zeta/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/src/main.py#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_huggingface_permalink_dataset() {
        let permalink = HuggingFace.build_permalink(
            ParsedGitRemote {
                owner: "datasets/squad".into(),
                repo: "squad".into(),
            },
            BuildPermalinkParams::new(
                "abc123",
                &repo_path("README.md"),
                None,
            ),
        );

        let expected_url =
            "https://huggingface.co/datasets/squad/squad/blob/abc123/README.md";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_huggingface_commit_permalink() {
        let permalink = HuggingFace.build_commit_permalink(
            &ParsedGitRemote {
                owner: "zed-industries".into(),
                repo: "zeta".into(),
            },
            BuildCommitPermalinkParams {
                sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            },
        );

        let expected_url = "https://huggingface.co/zed-industries/zeta/commit/e5fe811d7ad0fc26934edd76f891d20bdc3bb194";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
