use std::ops::Range;

use anyhow::{anyhow, Result};
use language::Point;
use url::Url;

enum GitHostingProvider {
    Github,
    Gitlab,
    Gitee,
}

impl GitHostingProvider {
    fn base_url(&self) -> Url {
        let base_url = match self {
            Self::Github => "https://github.com",
            Self::Gitlab => "https://gitlab.com",
            Self::Gitee => "https://gitee.com",
        };

        Url::parse(&base_url).unwrap()
    }

    /// Returns the fragment portion of the URL for the selected lines in
    /// the representation the [`GitHostingProvider`] expects.
    fn line_fragment(&self, selection: &Range<Point>) -> String {
        if selection.start.row == selection.end.row {
            let line = selection.start.row + 1;

            match self {
                Self::Github | Self::Gitlab | Self::Gitee => format!("L{}", line),
            }
        } else {
            let start_line = selection.start.row + 1;
            let end_line = selection.end.row + 1;

            match self {
                Self::Github => format!("L{}-L{}", start_line, end_line),
                Self::Gitlab => format!("L{}-{}", start_line, end_line),
                Self::Gitee => format!("L{}-{}", start_line, end_line),
            }
        }
    }
}

pub struct BuildPermalinkParams<'a> {
    pub remote_url: &'a str,
    pub sha: &'a str,
    pub path: &'a str,
    pub selection: Option<Range<Point>>,
}

pub fn build_permalink(params: BuildPermalinkParams) -> Result<Url> {
    let BuildPermalinkParams {
        remote_url,
        sha,
        path,
        selection,
    } = params;

    let ParsedGitRemote {
        provider,
        owner,
        repo,
    } = parse_git_remote_url(remote_url)
        .ok_or_else(|| anyhow!("failed to parse Git remote URL"))?;

    let path = match provider {
        GitHostingProvider::Github => format!("{owner}/{repo}/blob/{sha}/{path}"),
        GitHostingProvider::Gitlab => format!("{owner}/{repo}/-/blob/{sha}/{path}"),
        GitHostingProvider::Gitee => format!("{owner}/{repo}/blob/{sha}/{path}"),
    };
    let line_fragment = selection.map(|selection| provider.line_fragment(&selection));

    let mut permalink = provider.base_url().join(&path).unwrap();
    permalink.set_fragment(line_fragment.as_deref());

    Ok(permalink)
}

struct ParsedGitRemote<'a> {
    pub provider: GitHostingProvider,
    pub owner: &'a str,
    pub repo: &'a str,
}

fn parse_git_remote_url(url: &str) -> Option<ParsedGitRemote> {
    if url.starts_with("git@github.com:") || url.starts_with("https://github.com/") {
        let repo_with_owner = url
            .trim_start_matches("git@github.com:")
            .trim_start_matches("https://github.com/")
            .trim_end_matches(".git");

        let (owner, repo) = repo_with_owner.split_once("/")?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Github,
            owner,
            repo,
        });
    }

    if url.starts_with("git@gitlab.com:") || url.starts_with("https://gitlab.com/") {
        let repo_with_owner = url
            .trim_start_matches("git@gitlab.com:")
            .trim_start_matches("https://gitlab.com/")
            .trim_end_matches(".git");

        let (owner, repo) = repo_with_owner.split_once("/")?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Gitlab,
            owner,
            repo,
        });
    }

    if url.starts_with("git@gitee.com:") || url.starts_with("https://gitee.com/") {
        let repo_with_owner = url
            .trim_start_matches("git@gitee.com:")
            .trim_start_matches("https://gitee.com/")
            .trim_end_matches(".git");

        let (owner, repo) = repo_with_owner.split_once("/")?;

        return Some(ParsedGitRemote {
            provider: GitHostingProvider::Gitee,
            owner,
            repo,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_github_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@github.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@github.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(Point::new(6, 1)..Point::new(6, 10)),
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@github.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(Point::new(23, 1)..Point::new(47, 10)),
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_https_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://github.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_https_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://github.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: Some(Point::new(6, 1)..Point::new(6, 10)),
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_github_permalink_from_https_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://github.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: Some(Point::new(23, 1)..Point::new(47, 10)),
        })
        .unwrap();

        let expected_url = "https://github.com/zed-industries/zed/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L24-L48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitlab.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitlab.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(Point::new(6, 1)..Point::new(6, 10)),
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitlab.com:zed-industries/zed.git",
            sha: "e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(Point::new(23, 1)..Point::new(47, 10)),
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/e6ebe7974deb6bb6cc0e2595c8ec31f0c71084b7/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitlab.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitlab.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: Some(Point::new(6, 1)..Point::new(6, 10)),
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitlab_permalink_from_https_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitlab.com/zed-industries/zed.git",
            sha: "b2efec9824c45fcc90c9a7eb107a50d1772a60aa",
            path: "crates/zed/src/main.rs",
            selection: Some(Point::new(23, 1)..Point::new(47, 10)),
        })
        .unwrap();

        let expected_url = "https://gitlab.com/zed-industries/zed/-/blob/b2efec9824c45fcc90c9a7eb107a50d1772a60aa/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_ssh_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitee.com:libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/editor/src/git/permalink.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_ssh_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitee.com:libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(Point::new(6, 1)..Point::new(6, 10)),
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_ssh_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "git@gitee.com:libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/editor/src/git/permalink.rs",
            selection: Some(Point::new(23, 1)..Point::new(47, 10)),
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/editor/src/git/permalink.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitee.com/libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/zed/src/main.rs",
            selection: None,
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url_single_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitee.com/libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/zed/src/main.rs",
            selection: Some(Point::new(6, 1)..Point::new(6, 10)),
        })
        .unwrap();

        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs#L7";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }

    #[test]
    fn test_build_gitee_permalink_from_https_url_multi_line_selection() {
        let permalink = build_permalink(BuildPermalinkParams {
            remote_url: "https://gitee.com/libkitten/zed.git",
            sha: "e5fe811d7ad0fc26934edd76f891d20bdc3bb194",
            path: "crates/zed/src/main.rs",
            selection: Some(Point::new(23, 1)..Point::new(47, 10)),
        })
        .unwrap();
        let expected_url = "https://gitee.com/libkitten/zed/blob/e5fe811d7ad0fc26934edd76f891d20bdc3bb194/crates/zed/src/main.rs#L24-48";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
