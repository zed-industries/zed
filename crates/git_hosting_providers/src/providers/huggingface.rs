use std::{str::FromStr, sync::Arc};

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use futures::AsyncReadExt;
use gpui::SharedString;
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request};
use serde::Deserialize;
use url::Url;

use git::{
    BuildCommitPermalinkParams, BuildPermalinkParams, GitHostingProvider, ParsedGitRemote,
    RemoteUrl,
};

#[derive(Debug, Deserialize)]
struct CommitEntry {
    authors: Vec<CommitAuthor>,
}

#[derive(Debug, Deserialize)]
struct CommitAuthor {
    avatar: Option<String>,
}

pub struct HuggingFace;

impl HuggingFace {
    fn api_url(&self, repo_owner: &str, repo: &str) -> String {
        // The owner may be prefixed with a repo type (e.g. "datasets/squad").
        // Map to the correct API path: /api/models/, /api/datasets/, or /api/spaces/.
        if let Some(owner) = repo_owner.strip_prefix("datasets/") {
            format!("https://huggingface.co/api/datasets/{owner}/{repo}")
        } else if let Some(owner) = repo_owner.strip_prefix("spaces/") {
            format!("https://huggingface.co/api/spaces/{owner}/{repo}")
        } else {
            format!("https://huggingface.co/api/models/{repo_owner}/{repo}")
        }
    }

    async fn fetch_commit_author_avatar(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: &str,
        client: &Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        let url = format!("{}/commits/{commit}?limit=1", self.api_url(repo_owner, repo));

        let request = Request::get(&url)
            .header("Content-Type", "application/json")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("error fetching Hugging Face commit details at {:?}", url))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        if response.status().is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status().as_u16()
            );
        }

        let body_str = std::str::from_utf8(&body)?;
        let commits: Vec<CommitEntry> = serde_json::from_str(body_str)
            .context("failed to deserialize Hugging Face commit details")?;

        let avatar = commits
            .first()
            .and_then(|entry| entry.authors.first())
            .and_then(|author| author.avatar.as_deref());

        let Some(avatar) = avatar else {
            return Ok(None);
        };

        if avatar.starts_with('/') {
            Ok(Some(Url::parse(&format!(
                "https://huggingface.co{avatar}"
            ))?))
        } else {
            Ok(Some(Url::parse(avatar)?))
        }
    }

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
        true
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

    async fn commit_author_avatar_url(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: SharedString,
        _author_email: Option<SharedString>,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        self.fetch_commit_author_avatar(repo_owner, repo, &commit, &http_client)
            .await
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
                "80f73ce5ee059377ce0662ec5c45b592c3025ae5",
                &repo_path("config.json"),
                None,
            ),
        );

        let expected_url = "https://huggingface.co/zed-industries/zeta/blob/80f73ce5ee059377ce0662ec5c45b592c3025ae5/config.json";
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
                "80f73ce5ee059377ce0662ec5c45b592c3025ae5",
                &repo_path("config.json"),
                Some(6..6),
            ),
        );

        let expected_url = "https://huggingface.co/zed-industries/zeta/blob/80f73ce5ee059377ce0662ec5c45b592c3025ae5/config.json#L7";
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
                "80f73ce5ee059377ce0662ec5c45b592c3025ae5",
                &repo_path("config.json"),
                Some(13..18),
            ),
        );

        let expected_url = "https://huggingface.co/zed-industries/zeta/blob/80f73ce5ee059377ce0662ec5c45b592c3025ae5/config.json#L14-L19";
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
                sha: "80f73ce5ee059377ce0662ec5c45b592c3025ae5",
            },
        );

        let expected_url = "https://huggingface.co/zed-industries/zeta/commit/80f73ce5ee059377ce0662ec5c45b592c3025ae5";
        assert_eq!(permalink.to_string(), expected_url.to_string())
    }
}
