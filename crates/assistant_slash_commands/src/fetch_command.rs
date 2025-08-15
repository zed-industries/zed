use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::{Context as _, Result, anyhow, bail};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use futures::AsyncReadExt;
use gpui::{Task, WeakEntity};
use html_to_markdown::{TagHandler, convert_html_to_markdown, markdown};
use http_client::{AsyncBody, HttpClient, HttpClientWithUrl};
use language::{BufferSnapshot, LspAdapterDelegate};
use ui::prelude::*;
use workspace::Workspace;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ContentType {
    Html,
    Plaintext,
    Json,
}

pub struct FetchSlashCommand;

impl FetchSlashCommand {
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
        let content_type = if content_type.starts_with("text/html") {
            ContentType::Html
        } else if content_type.starts_with("text/plain") {
            ContentType::Plaintext
        } else if content_type.starts_with("application/json") {
            ContentType::Json
        } else {
            ContentType::Html
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

impl SlashCommand for FetchSlashCommand {
    fn name(&self) -> String {
        "fetch".into()
    }

    fn description(&self) -> String {
        "Insert fetched URL contents".into()
    }

    fn icon(&self) -> IconName {
        IconName::ToolWeb
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let Some(argument) = arguments.first() else {
            return Task::ready(Err(anyhow!("missing URL")));
        };
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let http_client = workspace.read(cx).client().http_client();
        let url = argument.to_string();

        let text = cx.background_spawn({
            let url = url.clone();
            async move { Self::build_message(http_client, &url).await }
        });

        let url = SharedString::from(url);
        cx.foreground_executor().spawn(async move {
            let text = text.await?;
            if text.trim().is_empty() {
                bail!("no textual content found");
            }

            let range = 0..text.len();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::ToolWeb,
                    label: format!("fetch {}", url).into(),
                    metadata: None,
                }],
                run_commands_in_text: false,
            }
            .to_event_stream())
        })
    }
}
