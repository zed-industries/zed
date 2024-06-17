use super::{
    file_command::{codeblock_fence_for_path, EntryPlaceholder},
    SlashCommand, SlashCommandOutput,
};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use editor::Editor;
use gpui::{AppContext, Task, WeakView};
use language::LspAdapterDelegate;
use std::sync::Arc;
use ui::{IntoElement, WindowContext};
use workspace::Workspace;

pub(crate) struct ActiveSlashCommand;

impl SlashCommand for ActiveSlashCommand {
    fn name(&self) -> String {
        "active".into()
    }

    fn description(&self) -> String {
        "insert active tab".into()
    }

    fn menu_text(&self) -> String {
        "Insert Active Tab".into()
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancel: std::sync::Arc<std::sync::atomic::AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn run(
        self: Arc<Self>,
        _argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let output = workspace.update(cx, |workspace, cx| {
            let Some(active_item) = workspace.active_item(cx) else {
                return Task::ready(Err(anyhow!("no active tab")));
            };
            let Some(buffer) = active_item
                .downcast::<Editor>()
                .and_then(|editor| editor.read(cx).buffer().read(cx).as_singleton())
            else {
                return Task::ready(Err(anyhow!("active tab is not an editor")));
            };

            let snapshot = buffer.read(cx).snapshot();
            let path = snapshot.resolve_file_path(cx, true);
            let text = cx.background_executor().spawn({
                let path = path.clone();
                async move {
                    let mut output = String::new();
                    output.push_str(&codeblock_fence_for_path(path.as_deref(), None));
                    output.push('\n');
                    for chunk in snapshot.as_rope().chunks() {
                        output.push_str(chunk);
                    }
                    if !output.ends_with('\n') {
                        output.push('\n');
                    }
                    output.push_str("```");
                    output
                }
            });
            cx.foreground_executor().spawn(async move {
                let text = text.await;
                let range = 0..text.len();
                Ok(SlashCommandOutput {
                    text,
                    sections: vec![SlashCommandOutputSection {
                        range,
                        render_placeholder: Arc::new(move |id, unfold, _| {
                            EntryPlaceholder {
                                id,
                                path: path.clone(),
                                is_directory: false,
                                line_range: None,
                                unfold,
                            }
                            .into_any_element()
                        }),
                    }],
                    run_commands_in_text: false,
                })
            })
        });
        output.unwrap_or_else(|error| Task::ready(Err(error)))
    }
}
