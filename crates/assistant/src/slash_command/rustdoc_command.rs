use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;
use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandOutputSection};
use gpui::{AppContext, Task, WeakView};
use language::LspAdapterDelegate;
use rustdoc_to_markdown::convert_rustdoc_to_markdown;
use ui::{prelude::*, ButtonLike, ElevationIndex};
use workspace::Workspace;

pub(crate) struct RustdocSlashCommand;

impl RustdocSlashCommand {
    async fn build_message() -> Result<String> {
        let html = include_str!("/Users/maxdeviant/projects/zed/target/doc/gpui/index.html");

        convert_rustdoc_to_markdown(html)
    }
}

impl SlashCommand for RustdocSlashCommand {
    fn name(&self) -> String {
        "rustdoc".into()
    }

    fn description(&self) -> String {
        "insert the docs for a Rust crate".into()
    }

    fn tooltip_text(&self) -> String {
        "insert rustdoc".into()
    }

    fn requires_argument(&self) -> bool {
        false
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
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let text = cx
            .background_executor()
            .spawn(async move { Self::build_message().await });
        cx.foreground_executor().spawn(async move {
            let text = text.await?;
            let range = 0..text.len();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    render_placeholder: Arc::new(move |id, unfold, _cx| {
                        RustdocPlaceholder { id, unfold }.into_any_element()
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
}

impl RenderOnce for RustdocPlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(IconName::FileRust))
            .child(Label::new("rustdoc"))
            .on_click(move |_, cx| unfold(cx))
    }
}
