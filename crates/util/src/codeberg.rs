use crate::{git_author::GitAuthor, http::HttpClient};
use anyhow::{bail, Context, Result};
use futures::AsyncReadExt;
use isahc::{config::Configurable, AsyncBody, Request};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct CommitDetails {
    commit: Commit,
    author: Option<User>,
}

#[derive(Debug, Deserialize)]
struct Commit {
    author: Author,
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    email: String,
    date: String,
}

#[derive(Debug, Deserialize)]
struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
}

pub async fn fetch_codeberg_commit_author(
    repo_owner: &str,
    repo: &str,
    commit: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<Option<GitAuthor>> {
    let url = format!("https://codeberg.org/api/v1/repos/{repo_owner}/{repo}/git/commits/{commit}");

    let mut request = Request::get(&url)
        .redirect_policy(isahc::config::RedirectPolicy::Follow)
        .header("Content-Type", "application/json");

    if let Ok(codeberg_token) = std::env::var("CODEBERG_TOKEN") {
        request = request.header("Authorization", format!("Bearer {}", codeberg_token));
    }

    let mut response = client
        .send(request.body(AsyncBody::default())?)
        .await
        .with_context(|| format!("error fetching Codeberg commit details at {:?}", url))?;

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

    serde_json::from_str::<CommitDetails>(body_str)
        .map(|codeberg_commit| {
            if let Some(author) = codeberg_commit.author {
                Some(GitAuthor {
                    avatar_url: author.avatar_url,
                })
            } else {
                None
            }
        })
        .context("deserializing Codeberg commit details failed")
}
