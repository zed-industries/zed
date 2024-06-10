use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context, Result};
use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandOutputSection};
use futures::AsyncReadExt;
use gpui::{AppContext, Task, WeakView};
use html_to_markdown::{convert_html_to_markdown, markdown, HandleTag};
use http::{AsyncBody, HttpClient, HttpClientWithUrl};
use language::LspAdapterDelegate;
use ui::{prelude::*, ButtonLike, ElevationIndex};
use workspace::Workspace;

pub(crate) struct FetchSlashCommand;

impl FetchSlashCommand {
    async fn build_message(http_client: Arc<HttpClientWithUrl>, url: &str) -> Result<String> {
        let mut url = url.to_owned();
        if !url.starts_with("https://") {
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

        let mut handlers: Vec<Box<dyn HandleTag>> = vec![
            Box::new(markdown::ParagraphHandler),
            Box::new(markdown::HeadingHandler),
            Box::new(markdown::ListHandler),
            Box::new(markdown::TableHandler::new()),
            Box::new(markdown::StyledTextHandler),
        ];
        if url.contains("wikipedia.org") {
            use html_to_markdown::structure::wikipedia;

            handlers.push(Box::new(wikipedia::WikipediaChromeRemover));
            handlers.push(Box::new(wikipedia::WikipediaInfoboxHandler));
            handlers.push(Box::new(wikipedia::WikipediaCodeHandler::new()));
        } else {
            handlers.push(Box::new(markdown::CodeHandler));
        }

        convert_html_to_markdown(&body[..], handlers)
    }
}

impl SlashCommand for FetchSlashCommand {
    fn name(&self) -> String {
        "fetch".into()
    }

    fn description(&self) -> String {
        "insert URL contents".into()
    }

    fn menu_text(&self) -> String {
        "Insert fetched URL contents".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(argument) = argument else {
            return Task::ready(Err(anyhow!("missing URL")));
        };
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let http_client = workspace.read(cx).client().http_client();
        let url = argument.to_string();

        let text = cx.background_executor().spawn({
            let url = url.clone();
            async move { Self::build_message(http_client, &url).await }
        });

        let url = SharedString::from(url);
        cx.foreground_executor().spawn(async move {
            let text = text.await?;
            let range = 0..text.len();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    render_placeholder: Arc::new(move |id, unfold, _cx| {
                        FetchPlaceholder {
                            id,
                            unfold,
                            url: url.clone(),
                        }
                        .into_any_element()
                    }),
                }],
                run_commands_in_text: false,
            })
        })
    }
}

#[derive(IntoElement)]
struct FetchPlaceholder {
    pub id: ElementId,
    pub unfold: Arc<dyn Fn(&mut WindowContext)>,
    pub url: SharedString,
}

impl RenderOnce for FetchPlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(IconName::AtSign))
            .child(Label::new(format!("fetch {url}", url = self.url)))
            .on_click(move |_, cx| unfold(cx))
    }
}
