use url::Url;

use git::{BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote};

pub struct GitlabSelfHosted;

impl GitHostingProvider for GitlabSelfHosted {
    fn name(&self) -> String {
        "GitLab self hosted".to_string()
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
        if !url.starts_with("git@gitlab-self-hosted.com:")
            && !url.starts_with("https://gitlab-self-hosted.com/")
            && url.contains("gitlab")
        {
            if url.starts_with("https://") {
                let raw_url = url.trim_start_matches("https://").trim_end_matches(".git");
                let raw_url_cleaned = match raw_url.contains('@') {
                    true => raw_url.split_once('@')?.1,
                    false => raw_url,
                };
                let (base_url_raw, rest) = raw_url_cleaned.split_once('/')?;
                let (owner, repo) = rest.split_once('/')?;
                let base_url = Url::parse(format!("https://{base_url_raw}").as_str()).ok()?;

                return Some(ParsedGitRemote {
                    base_url,
                    owner,
                    repo,
                });
            } else if url.starts_with("git@") {
                let raw_url = url.trim_start_matches("git@").trim_end_matches(".git");
                let (base_url_raw, rest) = raw_url.split_once(':')?;
                let (owner, repo) = rest.split_once('/')?;
                let base_url = Url::parse(format!("https://{base_url_raw}").as_str()).ok()?;

                return Some(ParsedGitRemote {
                    base_url,
                    owner,
                    repo,
                });
            }
        }

        None
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
    static BASE_URL: std::sync::LazyLock<Url> =
        std::sync::LazyLock::new(|| Url::parse("https://gitlab-self-hosted.com").unwrap());
    use super::*;

    #[test]
    fn test_parse_remote_url_none() {
        assert_eq!(GitlabSelfHosted.parse_remote_url("").is_none(), true);
    }

    #[test]
    fn test_parse_remote_url_simple() {
        let actual = GitlabSelfHosted
            .parse_remote_url(
                "https://gitlab-forge.din.developpement-durable.gouv.fr/pub/pnm-public/camino.git",
            )
            .unwrap();
        assert_eq!(
            actual.base_url.as_str(),
            "https://gitlab-forge.din.developpement-durable.gouv.fr/"
        );
        assert_eq!(actual.owner, "pub");
        assert_eq!(actual.repo, "pnm-public/camino");
    }

    #[test]
    fn test_parse_remote_url_with_credentials() {
        let actual = GitlabSelfHosted.parse_remote_url("https://my_user:my_password@gitlab-forge.din.developpement-durable.gouv.fr/pub/pnm-public/camino.git").unwrap();
        assert_eq!(
            actual.base_url.as_str(),
            "https://gitlab-forge.din.developpement-durable.gouv.fr/"
        );
        assert_eq!(actual.owner, "pub");
        assert_eq!(actual.repo, "pnm-public/camino");
    }

    #[test]
    fn test_parse_remote_url_git() {
        let actual = GitlabSelfHosted
            .parse_remote_url("git@gitlab.something.fr:internal/part-of-repo/repo.git")
            .unwrap();
        assert_eq!(actual.base_url.as_str(), "https://gitlab.something.fr/");
        assert_eq!(actual.owner, "internal");
        assert_eq!(actual.repo, "part-of-repo/repo");
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url() {
        let remote = ParsedGitRemote {
            base_url: BASE_URL.clone(),
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = GitlabSelfHosted.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                path: "crates/editor/src/git/permalink.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitlab-self-hosted.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url_single_line_selection() {
        let remote = ParsedGitRemote {
            base_url: BASE_URL.clone(),
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = GitlabSelfHosted.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://gitlab-self-hosted.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            base_url: BASE_URL.clone(),
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = GitlabSelfHosted.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
                path: "crates/editor/src/git/permalink.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://gitlab-self-hosted.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url() {
        let remote = ParsedGitRemote {
            base_url: BASE_URL.clone(),
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = GitlabSelfHosted.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: None,
            },
        );

        let expected_url = "https://gitlab-self-hosted.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url_single_line_selection() {
        let remote = ParsedGitRemote {
            base_url: BASE_URL.clone(),
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = GitlabSelfHosted.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: Some(6..6),
            },
        );

        let expected_url = "https://gitlab-self-hosted.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url_multi_line_selection() {
        let remote = ParsedGitRemote {
            base_url: BASE_URL.clone(),
            owner: "zed-industries",
            repo: "zed",
        };
        let permalink = GitlabSelfHosted.build_permalink(
            remote,
            BuildPermalinkParams {
                sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
                path: "crates/zed/src/main.rs",
                selection: Some(23..47),
            },
        );

        let expected_url = "https://gitlab-self-hosted.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
