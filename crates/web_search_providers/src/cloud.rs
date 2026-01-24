use std::sync::Arc;

use anyhow::{Context as _, Result};
use client::Client;
use cloud_llm_client::{WebSearchBody, WebSearchResponse};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext, Context, Entity, Subscription, Task};
use http_client::{HttpClient, Method};
use language_model::{LlmApiToken, NeedsLlmTokenRefresh, RefreshLlmTokenListener};
use web_search::{WebSearchProvider, WebSearchProviderId};

pub struct CloudWebSearchProvider {
    state: Entity<State>,
}

impl CloudWebSearchProvider {
    pub fn new(client: Arc<Client>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State::new(client, cx));

        Self { state }
    }
}

pub struct State {
    client: Arc<Client>,
    llm_api_token: LlmApiToken,
    _llm_token_subscription: Subscription,
}

impl State {
    pub fn new(client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        let refresh_llm_token_listener = RefreshLlmTokenListener::global(cx);

        Self {
            client,
            llm_api_token: LlmApiToken::default(),
            _llm_token_subscription: cx.subscribe(
                &refresh_llm_token_listener,
                |this, _, _event, cx| {
                    let client = this.client.clone();
                    let llm_api_token = this.llm_api_token.clone();
                    cx.spawn(async move |_this, _cx| {
                        llm_api_token.refresh(&client).await?;
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                },
            ),
        }
    }
}

pub const ZED_WEB_SEARCH_PROVIDER_ID: &str = "zed.dev";

impl WebSearchProvider for CloudWebSearchProvider {
    fn id(&self) -> WebSearchProviderId {
        WebSearchProviderId(ZED_WEB_SEARCH_PROVIDER_ID.into())
    }

    fn search(&self, query: String, cx: &mut App) -> Task<Result<WebSearchResponse>> {
        let state = self.state.read(cx);
        let client = state.client.clone();
        let llm_api_token = state.llm_api_token.clone();
        let body = WebSearchBody { query };
        cx.background_spawn(async move { perform_web_search(client, llm_api_token, body).await })
    }
}

async fn perform_web_search(
    client: Arc<Client>,
    llm_api_token: LlmApiToken,
    body: WebSearchBody,
) -> Result<WebSearchResponse> {
    const MAX_RETRIES: usize = 3;

    let http_client = &client.http_client();
    let mut retries_remaining = MAX_RETRIES;
    let mut token = llm_api_token.acquire(&client).await?;

    loop {
        if retries_remaining == 0 {
            return Err(anyhow::anyhow!(
                "error performing web search, max retries exceeded"
            ));
        }

        let request = http_client::Request::builder()
            .method(Method::POST)
            .uri(http_client.build_zed_llm_url("/web_search", &[])?.as_ref())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {token}"))
            .body(serde_json::to_string(&body)?.into())?;
        let mut response = http_client
            .send(request)
            .await
            .context("failed to send web search request")?;

        if response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            return Ok(serde_json::from_str(&body)?);
        } else if response.needs_llm_token_refresh() {
            token = llm_api_token.refresh(&client).await?;
            retries_remaining -= 1;
        } else {
            // For now we will only retry if the LLM token is expired,
            // not if the request failed for any other reason.
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            anyhow::bail!(
                "error performing web search.\nStatus: {:?}\nBody: {body}",
                response.status(),
            );
        }
    }
}
