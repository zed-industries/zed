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
        module_path: Vec<String>,
    ) -> Result<String> {
        let version = "latest";
        let path = format!(
            "{crate_name}/{version}/{crate_name}/{module_path}",
            module_path = module_path.join("/")
        );

        let mut response = http_client
            .get(
                &format!("https://docs.rs/{path}"),
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
        let mut path_components = argument.split("::");
        let crate_name = match path_components
            .next()
            .ok_or_else(|| anyhow!("missing crate name"))
        {
            Ok(crate_name) => crate_name.to_string(),
            Err(err) => return Task::ready(Err(err)),
        };
        let module_path = path_components.map(ToString::to_string).collect::<Vec<_>>();

        let text = cx.background_executor().spawn({
            let crate_name = crate_name.clone();
            let module_path = module_path.clone();
            async move { Self::build_message(http_client, crate_name, module_path).await }
        });

        let crate_name = SharedString::from(crate_name);
        let module_path = if module_path.is_empty() {
            None
        } else {
            Some(SharedString::from(module_path.join("::")))
        };
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
                            module_path: module_path.clone(),
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
    pub module_path: Option<SharedString>,
}

impl RenderOnce for RustdocPlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;

        let crate_path = self
            .module_path
            .map(|module_path| format!("{crate_name}::{module_path}", crate_name = self.crate_name))
            .unwrap_or(self.crate_name.to_string());

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(IconName::FileRust))
            .child(Label::new(format!("rustdoc: {crate_path}")))
            .on_click(move |_, cx| unfold(cx))
    }
}
