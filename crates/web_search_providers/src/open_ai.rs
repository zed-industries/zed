use std::sync::Arc;

use anyhow::Result;
use client::Client;
use gpui::{App, Task};
use web_search::{WebSearchProvider, WebSearchProviderId, WebSearchResponse};

pub struct OpenAiWebSearchProvider {
    client: Arc<Client>,
}

impl OpenAiWebSearchProvider {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl WebSearchProvider for OpenAiWebSearchProvider {
    fn id(&self) -> WebSearchProviderId {
        WebSearchProviderId("openai".into())
    }

    fn search(&self, query: String, cx: &mut App) -> Task<Result<WebSearchResponse>> {
        todo!()
    }
}
