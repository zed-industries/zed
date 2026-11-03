use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context as _;
use async_trait::async_trait;
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClient, Method, Request, Response, http::header};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use url::Url;

use crate::*;

const BASE_URL: &str = "https://crates.io/api/v1";
// User agent and request rate requirements: https://crates.io/data-access
const USER_AGENT: &str = "Zed GPUI releases (https://github.com/zed-industries/zed)";
pub(crate) const MIN_REQUEST_INTERVAL: Duration = Duration::from_secs(1);
const MAX_PER_PAGE: usize = 100;
const GITHUB_CONFIGS_PATH: &str = "/trusted_publishing/github_configs";

/// Reserving the send time before waiting makes concurrent requests queue up
/// behind each other instead of all waking at the same instant.
pub(crate) struct RequestThrottle {
    next_request_at: Mutex<Option<Instant>>,
}

impl RequestThrottle {
    pub(crate) fn new() -> Self {
        Self {
            next_request_at: Mutex::new(None),
        }
    }

    pub(crate) fn reserve(&self, now: Instant) -> Instant {
        let mut next_request_at = self.next_request_at.lock();
        let send_at = next_request_at.map_or(now, |next_request_at| next_request_at.max(now));
        *next_request_at = Some(send_at + MIN_REQUEST_INTERVAL);
        send_at
    }
}

/// The request rate limit is enforced per client, so share a single client.
pub struct RealCratesIoClient {
    http_client: Arc<dyn HttpClient>,
    api_token: ApiToken,
    throttle: RequestThrottle,
}

impl RealCratesIoClient {
    pub fn new(http_client: Arc<dyn HttpClient>, api_token: ApiToken) -> Self {
        Self {
            http_client,
            api_token,
            throttle: RequestThrottle::new(),
        }
    }

    fn url(path: &str) -> Result<Url> {
        Ok(Url::parse(&format!("{BASE_URL}{path}")).context("Invalid crates.io URL")?)
    }

    fn crate_url(name: &CrateName) -> Result<Url> {
        let mut url = Self::url("/crates")?;
        url.path_segments_mut()
            .map_err(|()| anyhow::anyhow!("crates.io URL cannot have path segments"))?
            .push(name.as_str());
        Ok(url)
    }

    async fn request_response(
        &self,
        method: Method,
        url: Url,
        body: Option<Vec<u8>>,
    ) -> Result<Response<AsyncBody>> {
        let mut request = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, self.api_token.value());
        if body.is_some() {
            request = request.header(header::CONTENT_TYPE, "application/json");
        }
        let request = request
            .body(body.map_or_else(AsyncBody::empty, AsyncBody::from))
            .context("Failed to build crates.io request")?;

        let send_at = self.throttle.reserve(Instant::now());
        smol::Timer::at(send_at).await;

        let mut response = self
            .http_client
            .send(request)
            .await
            .with_context(|| format!("Failed to send crates.io request to {url}"))?;

        if !response.status().is_success() {
            let mut body = String::new();
            response
                .body_mut()
                .read_to_string(&mut body)
                .await
                .with_context(|| format!("Failed to read crates.io response from {url}"))?;
            return Err(CratesIoError::from_response(response.status(), &body));
        }

        Ok(response)
    }

    async fn request_json<T: DeserializeOwned>(
        &self,
        method: Method,
        url: Url,
        body: Option<Vec<u8>>,
    ) -> Result<T> {
        let mut response = self.request_response(method, url.clone(), body).await?;
        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .with_context(|| format!("Failed to read crates.io response from {url}"))?;
        Ok(serde_json::from_slice(&body)
            .with_context(|| format!("Failed to parse crates.io response from {url}"))?)
    }
}

#[async_trait]
impl CratesIoClient for RealCratesIoClient {
    async fn get_site_metadata(&self) -> Result<SiteMetadata> {
        self.request_json(Method::GET, Self::url("/site_metadata")?, None)
            .await
    }

    async fn list_crates(&self, query: &CrateListQuery) -> Result<Vec<Crate>> {
        #[derive(Deserialize)]
        struct CrateListResponse {
            crates: Vec<Crate>,
            meta: CrateListMeta,
        }

        #[derive(Deserialize)]
        struct CrateListMeta {
            /// A query string for the next page, which preserves the filters.
            next_page: Option<String>,
        }

        let mut url = Self::url("/crates")?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("per_page", &MAX_PER_PAGE.to_string());
            if let Some(search) = &query.search {
                pairs.append_pair("q", search);
            }
            if let Some(category) = &query.category {
                pairs.append_pair("category", category);
            }
            if let Some(keyword) = &query.keyword {
                pairs.append_pair("keyword", keyword);
            }
            if let Some(user_id) = query.user_id {
                pairs.append_pair("user_id", &user_id.0.to_string());
            }
            if query.exclude_yanked {
                pairs.append_pair("include_yanked", "no");
            }
        }

        let mut crates = Vec::new();
        loop {
            let mut page: CrateListResponse = self.request_json(Method::GET, url, None).await?;
            let is_full_page = page.crates.len() == MAX_PER_PAGE;
            crates.append(&mut page.crates);

            match page.meta.next_page {
                Some(next_page) if is_full_page => {
                    url = Self::url(&format!("/crates{next_page}"))?;
                }
                _ => return Ok(crates),
            }
        }
    }

    async fn get_crate(&self, name: &CrateName) -> Result<CrateDetails> {
        #[derive(Deserialize)]
        struct CrateResponse {
            #[serde(rename = "crate")]
            krate: Crate,
            versions: Option<Vec<Version>>,
        }

        let mut url = Self::crate_url(name)?;
        url.query_pairs_mut()
            .append_pair("include", "versions,keywords,categories");
        let response: CrateResponse = self.request_json(Method::GET, url, None).await?;
        Ok(CrateDetails {
            krate: response.krate,
            versions: response.versions.unwrap_or_default(),
        })
    }

    async fn update_crate(&self, name: &CrateName, update: &CrateUpdate) -> Result<Crate> {
        #[derive(Serialize)]
        struct UpdateRequest<'a> {
            #[serde(rename = "crate")]
            krate: &'a CrateUpdate,
        }

        #[derive(Deserialize)]
        struct UpdateResponse {
            #[serde(rename = "crate")]
            krate: Crate,
        }

        let body = serde_json::to_vec(&UpdateRequest { krate: update })
            .context("Failed to serialize crate update")?;
        let response: UpdateResponse = self
            .request_json(Method::PATCH, Self::crate_url(name)?, Some(body))
            .await?;
        Ok(response.krate)
    }

    async fn list_github_configs(&self, crate_name: &CrateName) -> Result<Vec<GitHubConfig>> {
        #[derive(Deserialize)]
        struct GitHubConfigListResponse {
            github_configs: Vec<GitHubConfig>,
        }

        let mut url = Self::url(GITHUB_CONFIGS_PATH)?;
        url.query_pairs_mut()
            .append_pair("per_page", &MAX_PER_PAGE.to_string())
            .append_pair("crate", crate_name.as_str());
        let response: GitHubConfigListResponse = self.request_json(Method::GET, url, None).await?;
        if response.github_configs.len() >= MAX_PER_PAGE {
            return Err(anyhow::anyhow!(
                "crates.io returned more GitHub Trusted Publishing configurations for `{crate_name}` than expected"
            )
            .into());
        }
        Ok(response.github_configs)
    }

    async fn create_github_config(&self, config: &NewGitHubConfig) -> Result<GitHubConfig> {
        #[derive(Serialize)]
        struct CreateRequest<'a> {
            github_config: &'a NewGitHubConfig,
        }

        #[derive(Deserialize)]
        struct CreateResponse {
            github_config: GitHubConfig,
        }

        let body = serde_json::to_vec(&CreateRequest {
            github_config: config,
        })
        .context("Failed to serialize GitHub Trusted Publishing configuration")?;
        let response: CreateResponse = self
            .request_json(Method::POST, Self::url(GITHUB_CONFIGS_PATH)?, Some(body))
            .await?;
        Ok(response.github_config)
    }

    async fn delete_github_config(&self, id: GitHubConfigId) -> Result<()> {
        let url = Self::url(&format!("{GITHUB_CONFIGS_PATH}/{id}"))?;
        self.request_response(Method::DELETE, url, None).await?;
        Ok(())
    }
}
