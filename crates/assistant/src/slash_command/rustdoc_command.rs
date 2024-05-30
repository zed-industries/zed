use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context, Result};
use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandOutputSection};
use futures::AsyncReadExt;
use gpui::{AppContext, Task, WeakView};
use http::{AsyncBody, HttpClient, HttpClientWithUrl};
use language::LspAdapterDelegate;
use rustdoc_to_markdown::convert_rustdoc_to_markdown;
use ui::{prelude::*, ButtonLike, ElevationIndex};
use workspace::Workspace;

pub(crate) struct RustdocSlashCommand;

impl RustdocSlashCommand {
    async fn build_message(
        http_client: Arc<HttpClientWithUrl>,
        crate_name: String,
    ) -> Result<String> {
        let mut response = http_client
            .get(
                &format!("https://docs.rs/{crate_name}"),
                AsyncBody::default(),
                true,
            )
            .await?;

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading docs.rs response body")?;

        if response.status().is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status().as_u16()
            );
        }

        convert_rustdoc_to_markdown(&body[..])
    }
}

impl SlashCommand for RustdocSlashCommand {
    fn name(&self) -> String {
        "rustdoc".into()
    }

    fn description(&self) -> String {
        "insert Rust docs".into()
    }

    fn menu_text(&self) -> String {
        "Insert Rust Documentation".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancel: Arc<AtomicBool>,
        _workspace: WeakView<Workspace>,
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
            return Task::ready(Err(anyhow!("missing crate name")));
        };
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let http_client = workspace.read(cx).client().http_client();
        let crate_name = argument.to_string();

        let text = cx.background_executor().spawn({
            let crate_name = crate_name.clone();
            async move { Self::build_message(http_client, crate_name).await }
        });

        let crate_name = SharedString::from(crate_name);
        cx.foreground_executor().spawn(async move {
            let text = text.await?;
            let range = 0..text.len();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    render_placeholder: Arc::new(move |id, unfold, _cx| {
                        RustdocPlaceholder {
                            id,
                            unfold,
                            crate_name: crate_name.clone(),
                        }
                        .into_any_element()
                    }),
                }],
            })
        })
    }
}

#[derive(IntoElement)]
struct RustdocPlaceholder {
    pub id: ElementId,
    pub unfold: Arc<dyn Fn(&mut WindowContext)>,
    pub crate_name: SharedString,
}

impl RenderOnce for RustdocPlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(IconName::FileRust))
            .child(Label::new(format!("rustdoc: {}", self.crate_name)))
            .on_click(move |_, cx| unfold(cx))
    }
}
