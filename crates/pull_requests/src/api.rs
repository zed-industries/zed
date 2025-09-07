use anyhow::Result;
use async_trait::async_trait;
use futures::AsyncReadExt;
use git::ParsedGitRemote;
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::sync::Arc;
use url::Url;

use crate::models::{
    CreatePullRequest, CreateReview, PullRequest, PullRequestComment, PullRequestDiff,
    PullRequestReview, UpdatePullRequest,
};

#[async_trait]
pub trait PullRequestApi: Send + Sync {
    async fn list_pull_requests(
        &self,
        remote: &ParsedGitRemote,
        state: Option<&str>,
    ) -> Result<Vec<PullRequest>>;

    async fn get_pull_request(&self, remote: &ParsedGitRemote, number: u32) -> Result<PullRequest>;

    async fn create_pull_request(
        &self,
        remote: &ParsedGitRemote,
        pr: CreatePullRequest,
    ) -> Result<PullRequest>;

    async fn update_pull_request(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
        update: UpdatePullRequest,
    ) -> Result<PullRequest>;

    async fn get_pull_request_diff(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
    ) -> Result<PullRequestDiff>;

    async fn list_pull_request_comments(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
    ) -> Result<Vec<PullRequestComment>>;

    async fn create_review(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
        review: CreateReview,
    ) -> Result<PullRequestReview>;

    async fn merge_pull_request(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
        commit_title: Option<String>,
        commit_message: Option<String>,
        merge_method: Option<&str>,
    ) -> Result<()>;
}

pub struct GithubPrClient {
    http_client: Arc<dyn HttpClient>,
    token: Option<String>,
    base_api_url: Url,
}

impl GithubPrClient {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        let token = std::env::var("GITHUB_TOKEN").ok();
        let base_api_url = Url::parse("https://api.github.com").unwrap();

        Self {
            http_client,
            token,
            base_api_url,
        }
    }

    pub fn new_with_auth(http_client: Arc<dyn HttpClient>, token: Option<String>) -> Self {
        let base_api_url = Url::parse("https://api.github.com").unwrap();

        Self {
            http_client,
            token,
            base_api_url,
        }
    }

    pub fn with_base_url(mut self, base_url: Url) -> Self {
        let host = base_url.host_str().unwrap_or("github.com");
        if host == "github.com" {
            self.base_api_url = Url::parse("https://api.github.com").unwrap();
        } else {
            self.base_api_url = Url::parse(&format!("https://api.{}", host))
                .unwrap_or_else(|_| base_url.join("/api/v3/").unwrap());
        }
        self
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let url = self.base_api_url.join(path)?;

        let mut request = match method {
            "GET" => Request::get(url.as_str()),
            "POST" => Request::post(url.as_str()),
            "PUT" => Request::put(url.as_str()),
            "PATCH" => Request::patch(url.as_str()),
            "DELETE" => Request::delete(url.as_str()),
            _ => anyhow::bail!("Unsupported HTTP method: {}", method),
        };

        request = request
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "zed-pull-requests")
            .follow_redirects(http_client::RedirectPolicy::FollowAll);

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let body = if let Some(json_body) = body {
            AsyncBody::from(serde_json::to_vec(&json_body)?)
        } else {
            AsyncBody::default()
        };

        request = request.header("Content-Type", "application/json");

        let mut response = self
            .http_client
            .send(request.body(body)?)
            .await
            .map_err(|e| anyhow::anyhow!("HTTP request failed: {}", e))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        if !response.status().is_success() {
            let error_text = String::from_utf8_lossy(&body);
            anyhow::bail!("GitHub API error ({}): {}", response.status(), error_text);
        }

        serde_json::from_slice(&body)
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))
    }
}

#[async_trait]
impl PullRequestApi for GithubPrClient {
    async fn list_pull_requests(
        &self,
        remote: &ParsedGitRemote,
        state: Option<&str>,
    ) -> Result<Vec<PullRequest>> {
        let mut path = format!("repos/{}/{}/pulls", remote.owner, remote.repo);
        if let Some(state) = state {
            path.push_str(&format!("?state={}", state));
        }
        self.request("GET", &path, None).await
    }

    async fn get_pull_request(&self, remote: &ParsedGitRemote, number: u32) -> Result<PullRequest> {
        let path = format!("repos/{}/{}/pulls/{}", remote.owner, remote.repo, number);
        self.request("GET", &path, None).await
    }

    async fn create_pull_request(
        &self,
        remote: &ParsedGitRemote,
        pr: CreatePullRequest,
    ) -> Result<PullRequest> {
        let path = format!("repos/{}/{}/pulls", remote.owner, remote.repo);
        let body = serde_json::to_value(pr)?;
        self.request("POST", &path, Some(body)).await
    }

    async fn update_pull_request(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
        update: UpdatePullRequest,
    ) -> Result<PullRequest> {
        let path = format!("repos/{}/{}/pulls/{}", remote.owner, remote.repo, number);
        let body = serde_json::to_value(update)?;
        self.request("PATCH", &path, Some(body)).await
    }

    async fn get_pull_request_diff(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
    ) -> Result<PullRequestDiff> {
        let path = format!(
            "repos/{}/{}/pulls/{}/files",
            remote.owner, remote.repo, number
        );
        let files = self.request("GET", &path, None).await?;
        Ok(PullRequestDiff { files })
    }

    async fn list_pull_request_comments(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
    ) -> Result<Vec<PullRequestComment>> {
        // Try to get issue comments (general discussion)
        let path = format!(
            "repos/{}/{}/issues/{}/comments",
            remote.owner, remote.repo, number
        );
        
        // Use a custom request that handles both arrays and error objects
        match self.request::<Vec<PullRequestComment>>("GET", &path, None).await {
            Ok(comments) => Ok(comments),
            Err(e) => {
                // If it fails, log the error and return empty array
                log::warn!("Failed to fetch PR comments for #{}: {}", number, e);
                Ok(vec![])
            }
        }
    }

    async fn create_review(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
        review: CreateReview,
    ) -> Result<PullRequestReview> {
        let path = format!(
            "repos/{}/{}/pulls/{}/reviews",
            remote.owner, remote.repo, number
        );
        let body = serde_json::to_value(review)?;
        self.request("POST", &path, Some(body)).await
    }

    async fn merge_pull_request(
        &self,
        remote: &ParsedGitRemote,
        number: u32,
        commit_title: Option<String>,
        commit_message: Option<String>,
        merge_method: Option<&str>,
    ) -> Result<()> {
        let path = format!(
            "repos/{}/{}/pulls/{}/merge",
            remote.owner, remote.repo, number
        );

        let mut body = json!({});
        if let Some(title) = commit_title {
            body["commit_title"] = json!(title);
        }
        if let Some(message) = commit_message {
            body["commit_message"] = json!(message);
        }
        if let Some(method) = merge_method {
            body["merge_method"] = json!(method);
        }

        self.request::<serde_json::Value>("PUT", &path, Some(body))
            .await?;
        Ok(())
    }
}
