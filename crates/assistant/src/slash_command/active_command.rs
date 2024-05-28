use super::{file_command::FilePlaceholder, SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use collections::HashMap;
use editor::Editor;
use gpui::{AppContext, Entity, Task, WeakView};
use language::LspAdapterDelegate;
use std::{borrow::Cow, sync::Arc};
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

    fn tooltip_text(&self) -> String {
        "insert active tab".into()
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancel: std::sync::Arc<std::sync::atomic::AtomicBool>,
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
            let mut timestamps_by_entity_id = HashMap::default();
            for pane in workspace.panes() {
                let pane = pane.read(cx);
                for entry in pane.activation_history() {
                    timestamps_by_entity_id.insert(entry.entity_id, entry.timestamp);
                }
            }

            let mut most_recent_buffer = None;
            for editor in workspace.items_of_type::<Editor>(cx) {
                let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() else {
                    continue;
                };

                let timestamp = timestamps_by_entity_id
                    .get(&editor.entity_id())
                    .copied()
                    .unwrap_or_default();
                if most_recent_buffer
                    .as_ref()
                    .map_or(true, |(_, prev_timestamp)| timestamp > *prev_timestamp)
                {
                    most_recent_buffer = Some((buffer, timestamp));
                }
            }

            if let Some((buffer, _)) = most_recent_buffer {
                let snapshot = buffer.read(cx).snapshot();
                let path = snapshot.resolve_file_path(cx, true);
                let text = cx.background_executor().spawn({
                    let path = path.clone();
                    async move {
                        let path = path
                            .as_ref()
                            .map(|path| path.to_string_lossy())
                            .unwrap_or_else(|| Cow::Borrowed("untitled"));

                        let mut output = String::with_capacity(path.len() + snapshot.len() + 9);
                        output.push_str("```");
                        output.push_str(&path);
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
                    Ok(SlashCommandOutput {
                        text: text.await,
                        render_placeholder: Arc::new(move |id, unfold, _| {
                            FilePlaceholder {
                                id,
                                path: path.clone(),
                                unfold,
                            }
                            .into_any_element()
                        }),
                    })
                })
            } else {
                Task::ready(Err(anyhow!("no recent buffer found")))
            }
        });
        output.unwrap_or_else(|error| Task::ready(Err(error)))
    }
}
