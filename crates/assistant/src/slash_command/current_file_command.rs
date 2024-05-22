use std::{borrow::Cow, cell::Cell, rc::Rc};

use anyhow::{anyhow, Result};
use collections::HashMap;
use editor::Editor;
use futures::channel::oneshot;
use gpui::{AppContext, Entity, Subscription, Task, WindowHandle};
use workspace::{Event as WorkspaceEvent, Workspace};

use super::{SlashCommand, SlashCommandCleanup, SlashCommandInvocation};

pub(crate) struct CurrentFileSlashCommand {
    workspace: WindowHandle<Workspace>,
}

impl CurrentFileSlashCommand {
    pub fn new(workspace: WindowHandle<Workspace>) -> Self {
        Self { workspace }
    }
}

impl SlashCommand for CurrentFileSlashCommand {
    fn name(&self) -> String {
        "current_file".into()
    }

    fn description(&self) -> String {
        "insert the current file".into()
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

    fn run(&self, _argument: Option<&str>, cx: &mut AppContext) -> SlashCommandInvocation {
        let (invalidate_tx, invalidate_rx) = oneshot::channel();
        let invalidate_tx = Rc::new(Cell::new(Some(invalidate_tx)));
        let mut subscriptions: Vec<Subscription> = Vec::new();
        let output = self.workspace.update(cx, |workspace, cx| {
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

            subscriptions.push({
                let workspace_view = cx.view().clone();
                let invalidate_tx = invalidate_tx.clone();
                cx.window_context()
                    .subscribe(&workspace_view, move |_workspace, event, _cx| match event {
                        WorkspaceEvent::ActiveItemChanged
                        | WorkspaceEvent::ItemAdded
                        | WorkspaceEvent::ItemRemoved
                        | WorkspaceEvent::PaneAdded(_)
                        | WorkspaceEvent::PaneRemoved => {
                            if let Some(invalidate_tx) = invalidate_tx.take() {
                                _ = invalidate_tx.send(());
                            }
                        }
                        _ => {}
                    })
            });

            if let Some((buffer, _)) = most_recent_buffer {
                subscriptions.push({
                    let invalidate_tx = invalidate_tx.clone();
                    cx.window_context().observe(&buffer, move |_buffer, _cx| {
                        if let Some(invalidate_tx) = invalidate_tx.take() {
                            _ = invalidate_tx.send(());
                        }
                    })
                });

                let snapshot = buffer.read(cx).snapshot();
                let path = snapshot.resolve_file_path(cx, true);
                cx.background_executor().spawn(async move {
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
                    Ok(output)
                })
            } else {
                Task::ready(Err(anyhow!("no recent buffer found")))
            }
        });

        SlashCommandInvocation {
            output: output.unwrap_or_else(|error| Task::ready(Err(error))),
            invalidated: invalidate_rx,
            cleanup: SlashCommandCleanup::new(move || drop(subscriptions)),
        }
    }
}
