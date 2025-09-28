use editor::{display_map::Inlay, Editor, EditorEvent, InlayId};
use gpui::{Context, Entity, Render, Subscription, Task, Window};
use log::info;
use project::Project;
use std::time::Duration;
use ui::prelude::*;
use workspace::{ItemHandle, StatusItemView, Workspace};

/// Adds inline reference-count hints next to symbols in the active editor and logs counts.
pub struct SymbolRefHints {
    project: Entity<Project>,
    _observe_active_editor: Option<Subscription>,
    ongoing_task: Task<()>,
}

const HINT_BASE_ID: usize = 900_000_000; // avoid collisions with other inlays
const MAX_REMOVE: usize = 1024; // remove up to this many old hints each refresh

impl SymbolRefHints {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            project: workspace.project().clone(),
            _observe_active_editor: None,
            ongoing_task: Task::ready(()),
        }
    }

    fn on_symbols_changed(
        &mut self,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
        _event: &EditorEvent,
    ) {
        self.refresh_symbol_ref_hints(editor, window, cx);
    }

    fn refresh_symbol_ref_hints(
        &mut self,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Capture the active excerpt, buffer and its outline items synchronously.
        let maybe_data = editor.read(cx).active_excerpt(cx).and_then(|(excerpt_id, buffer, _)| {
            let items = buffer.read(cx).snapshot().outline(None).items;
            Some((excerpt_id, buffer, items))
        });
        let Some((excerpt_id, buffer, items)) = maybe_data else { return; };
        let project = self.project.clone();
        let editor_handle = editor.clone();

        // Debounce a bit to avoid excessive LSP traffic while typing rapidly.
        let debounce = Duration::from_millis(150);
        self.ongoing_task = cx.spawn_in(window, async move |_this, cx| {
            cx.background_executor().timer(debounce).await;

            // For each outline item, request references at the start anchor of the item.
            let mut counts: Vec<usize> = Vec::with_capacity(items.len());
            for item in &items {
                let symbol_label = item.text.clone();
                let position_anchor = item.range.start;

                let n = if let Some(task) = project
                    .update(cx, |p, cx| p.references(&buffer, position_anchor, cx))
                    .ok()
                {
                    match task.await {
                        Ok(Some(locations)) => {
                            let n = locations.len();
                            info!("symbol_refs: '{}' -> {} refs", symbol_label, n);
                            n
                        }
                        Ok(None) => {
                            info!("symbol_refs: '{}' -> (references not supported)", symbol_label);
                            0
                        }
                        Err(err) => {
                            info!("symbol_refs: '{}' -> error: {}", symbol_label, err);
                            0
                        }
                    }
                } else {
                    0
                };
                counts.push(n);
            }

            // Build inline hints, converting text anchors to editor anchors.
            let inlays = editor_handle
                .read_with(cx, |ed, app| {
                    let mb_snapshot = ed.buffer().read(app).snapshot(app);
                    items
                        .into_iter()
                        .enumerate()
                        .filter_map(|(i, item)| {
                            let pos = mb_snapshot.anchor_in_excerpt(excerpt_id, item.range.start)?;
                            let text = format!("{} ", counts[i]);
                            Some(Inlay::debugger(HINT_BASE_ID + i, pos, text))
                        })
                        .collect::<Vec<Inlay>>()
                })
                .unwrap_or_default();

            if inlays.is_empty() { return; }

            let to_remove: Vec<InlayId> = (0..MAX_REMOVE)
                .map(|i| InlayId::DebuggerValue(HINT_BASE_ID + i))
                .collect();

            let _ = editor_handle.update(cx, |ed, cx| ed.splice_inlays(&to_remove, inlays, cx));
        });
    }
}

impl Render for SymbolRefHints {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Invisible status item.
        div().w_0().invisible()
    }
}

impl StatusItemView for SymbolRefHints {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            // Observe editor events related to syntax/outline updates.
            self._observe_active_editor = Some(cx.subscribe_in(&editor, window, |this, editor, event: &EditorEvent, window, cx| {
                match event {
                    EditorEvent::Reparsed(_)
                    | EditorEvent::ExcerptsEdited { .. }
                    | EditorEvent::Edited { .. }
                    | EditorEvent::BufferEdited
                    | EditorEvent::Saved => {
                        this.on_symbols_changed(&editor, window, cx, event);
                    }
                    _ => {}
                }
            }));
            // Prime once on activation.
            self.refresh_symbol_ref_hints(&editor, window, cx);
        } else {
            // Clear subscription when no active editor.
            self._observe_active_editor = None;
        }
        cx.notify();
    }
}

