use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use futures::AsyncReadExt;
use gpui::{App, AppContext, Task};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::Deserialize;
use web_search::{
    WebSearchCitation, WebSearchProvider, WebSearchProviderId, WebSearchResponse, WebSearchResult,
};

const OPENAI_API_URL: &str = "https://api.openai.com/v1";

pub struct OpenAiWebSearchProvider {
    http_client: Arc<dyn HttpClient>,
}

impl OpenAiWebSearchProvider {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self { http_client }
    }
}

impl WebSearchProvider for OpenAiWebSearchProvider {
    fn id(&self) -> WebSearchProviderId {
        WebSearchProviderId("openai".into())
    }

    fn search(&self, query: String, cx: &mut App) -> Task<Result<WebSearchResponse>> {
        let client = self.http_client.clone();

        let input = serde_json::json!({
            "model": "gpt-4o",
            "input": query,
            "tools": [{"type": "web_search_preview"}],
            "tool_choice": {"type": "web_search_preview"}
        });

        let api_key_task = read_api_key(OPENAI_API_URL, cx);
        cx.background_spawn(async move {
            let api_key = api_key_task.await?;
            let response_json = perform_web_search(client, &input, &api_key).await?;
            let parsed_response: OpenAiWebSearchResponse =
                serde_json::from_str(&response_json).context("Failed to parse OpenAI response")?;
            Ok(parsed_response.into())
        })
    }
}

async fn perform_web_search(
    client: Arc<dyn HttpClient>,
    request: &serde_json::Value,
    api_key: &str,
) -> Result<String> {
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(format!("{OPENAI_API_URL}/responses"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        Ok(body)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        Err(anyhow!(
            "Failed to connect to OpenAI API: {} {}",
            response.status(),
            body,
        ))
    }
}

fn read_api_key(api_url: &str, cx: &mut App) -> Task<Result<String>> {
    const OPENAI_API_KEY_VAR: &str = "OPENAI_API_KEY";
    if let Ok(api_key) = std::env::var(OPENAI_API_KEY_VAR) {
        return Task::ready(Ok(api_key));
    };

    let task = cx.read_credentials(api_url);
    cx.background_spawn(async move {
        let (_, api_key) = task.await?.context("credentials not found")?;

        String::from_utf8(api_key).context("invalid API key")
    })
}

impl Into<WebSearchResponse> for OpenAiWebSearchResponse {
    fn into(self) -> WebSearchResponse {
        let mut results = Vec::new();
        for item in self.output {
            if let OutputItem::Message { content } = item {
                for content_item in content {
                    results.push(match content_item {
                        MessageContent::OutputText { text, annotations } => WebSearchResult {
                            summary: text,
                            citations: annotations
                                .into_iter()
                                .map(|annotation| match annotation {
                                    Annotation::UrlCitation {
                                        title,
                                        url,
                                        start_index,
                                        end_index,
                                    } => WebSearchCitation {
                                        title,
                                        url,
                                        range: Some(start_index..end_index),
                                    },
                                })
                                .collect(),
                        },
                    })
                }
            }
        }
        WebSearchResponse { results }
    }
}

#[derive(Debug, Deserialize)]
struct OpenAiWebSearchResponse {
    output: Vec<OutputItem>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum OutputItem {
    #[serde(rename = "web_search_call")]
    WebSearchCall,
    #[serde(rename = "message")]
    Message { content: Vec<MessageContent> },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum MessageContent {
    #[serde(rename = "output_text")]
    OutputText {
        text: String,
        annotations: Vec<Annotation>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Annotation {
    #[serde(rename = "url_citation")]
    UrlCitation {
        #[allow(dead_code)]
        start_index: usize,
        #[allow(dead_code)]
        end_index: usize,
        title: String,
        url: String,
    },
}
