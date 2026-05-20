use std::sync::Arc;

use anyhow::Result;
use client::{Client, UserStore, global_llm_token};
use cloud_api_client::LlmApiToken;
use cloud_api_types::OrganizationId;
use cloud_llm_client::{WebSearchBody, WebSearchResponse};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext, Context, Entity, Task};
use http_client::Method;
use web_search::{WebSearchProvider, WebSearchProviderId};

pub struct CloudWebSearchProvider {
    state: Entity<State>,
}

impl CloudWebSearchProvider {
    pub fn new(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State::new(client, user_store, cx));

        Self { state }
    }
}

pub struct State {
    client: Arc<Client>,
    user_store: Entity<UserStore>,
    llm_api_token: LlmApiToken,
}

impl State {
    pub fn new(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut Context<Self>) -> Self {
        let llm_api_token = global_llm_token(cx);

        Self {
            client,
            user_store,
            llm_api_token,
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
        let organization_id = state
            .user_store
            .read(cx)
            .current_organization()
            .map(|organization| organization.id.clone());
        let body = WebSearchBody { query };
        cx.background_spawn(async move {
            perform_web_search(client, llm_api_token, organization_id, body).await
        })
    }
}

async fn perform_web_search(
    client: Arc<Client>,
    llm_api_token: LlmApiToken,
    organization_id: Option<OrganizationId>,
    body: WebSearchBody,
) -> Result<WebSearchResponse> {
    let url = client.http_client().build_zed_llm_url("/web_search", &[])?;
    let body = serde_json::to_string(&body)?;
    let mut response = client
        .authenticated_llm_request(&llm_api_token, organization_id, |token| {
            Ok(http_client::Request::builder()
                .method(Method::POST)
                .uri(url.as_ref())
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .body(body.clone().into())?)
        })
        .await?;

    if response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        Ok(serde_json::from_str(&body)?)
    } else {
        let status = response.status();
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!("error performing web search.\nStatus: {status:?}\nBody: {body}");
    }
}
