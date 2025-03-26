use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context as _, Result};
use assistant_tool::{ActionLog, Tool};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Entity, Task};
use html_to_markdown::{convert_html_to_markdown, markdown, TagHandler};
use http_client::{AsyncBody, HttpClientWithUrl};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use util::markdown::MarkdownString;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ContentType {
    Html,
    Plaintext,
    Json,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FetchToolInput {
    /// The URL to fetch.
    url: String,
}

pub struct FetchTool {
    http_client: Arc<HttpClientWithUrl>,
}

impl FetchTool {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self { http_client }
    }

    async fn build_message(http_client: Arc<HttpClientWithUrl>, url: &str) -> Result<String> {
        let mut url = url.to_owned();
        if !url.starts_with("https://") && !url.starts_with("http://") {
            url = format!("https://{url}");
        }

        let mut response = http_client.get(&url, AsyncBody::default(), true).await?;

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading response body")?;

        if response.status().is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status().as_u16()
            );
        }

        let Some(content_type) = response.headers().get("content-type") else {
            bail!("missing Content-Type header");
        };
        let content_type = content_type
            .to_str()
            .context("invalid Content-Type header")?;
        let content_type = match content_type {
            "text/html" => ContentType::Html,
            "text/plain" => ContentType::Plaintext,
            "application/json" => ContentType::Json,
            _ => ContentType::Html,
        };

        match content_type {
            ContentType::Html => {
                let mut handlers: Vec<TagHandler> = vec![
                    Rc::new(RefCell::new(markdown::WebpageChromeRemover)),
                    Rc::new(RefCell::new(markdown::ParagraphHandler)),
                    Rc::new(RefCell::new(markdown::HeadingHandler)),
                    Rc::new(RefCell::new(markdown::ListHandler)),
                    Rc::new(RefCell::new(markdown::TableHandler::new())),
                    Rc::new(RefCell::new(markdown::StyledTextHandler)),
                ];
                if url.contains("wikipedia.org") {
                    use html_to_markdown::structure::wikipedia;

                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaChromeRemover)));
                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaInfoboxHandler)));
                    handlers.push(Rc::new(
                        RefCell::new(wikipedia::WikipediaCodeHandler::new()),
                    ));
                } else {
                    handlers.push(Rc::new(RefCell::new(markdown::CodeHandler)));
                }

                convert_html_to_markdown(&body[..], &mut handlers)
            }
            ContentType::Plaintext => Ok(std::str::from_utf8(&body)?.to_owned()),
            ContentType::Json => {
                let json: serde_json::Value = serde_json::from_slice(&body)?;

                Ok(format!(
                    "```json\n{}\n```",
                    serde_json::to_string_pretty(&json)?
                ))
            }
        }
    }
}

impl Tool for FetchTool {
    fn name(&self) -> String {
        "fetch".to_string()
    }

    fn needs_confirmation(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./fetch_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Globe
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(FetchToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<FetchToolInput>(input.clone()) {
            Ok(input) => format!("Fetch {}", MarkdownString::escape(&input.url)),
            Err(_) => "Fetch URL".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<FetchToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let text = cx.background_spawn({
            let http_client = self.http_client.clone();
            let url = input.url.clone();
            async move { Self::build_message(http_client, &url).await }
        });

        cx.foreground_executor().spawn(async move {
            let text = text.await?;
            if text.trim().is_empty() {
                bail!("no textual content found");
            }

            Ok(text)
        })
    }
}
