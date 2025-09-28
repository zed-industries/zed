use editor::{Editor, EditorEvent, InlayId, display_map::Inlay};
use gpui::{Context, Entity, Render, Subscription, Task, Window};
use language::language_settings::all_language_settings;
use project::Project;
use std::time::Duration;
use ui::prelude::*;

use language::{ToOffset, ToPoint};

use workspace::{ItemHandle, StatusItemView, Workspace};

/// Adds inline reference-count hints next to symbols in the active editor and logs counts.
pub struct SymbolRefHints {
    pub enabled: bool,
    project: Entity<Project>,
    _observe_active_editor: Option<Subscription>,
    _observe_settings: Option<Subscription>,
    ongoing_task: Task<()>,
    refresh_rev: u64,
}

const HINT_BASE_ID: usize = 900_000_000; // avoid collisions with other inlays
const MAX_REMOVE: usize = 1024; // remove up to this many old hints each refresh

impl SymbolRefHints {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            enabled: true,
            project: workspace.project().clone(),
            _observe_active_editor: None,
            _observe_settings: None,
            ongoing_task: Task::ready(()),
            refresh_rev: 0,
        }
    }


        fn cancel_task(&mut self) {
            // Replace any ongoing task with a completed one, dropping captured handles.
            self.ongoing_task = Task::ready(());
        }

    // --- Helpers to reduce duplication while preserving behavior ---
    fn removal_ids() -> Vec<InlayId> {
        (0..MAX_REMOVE)
            .map(|i| InlayId::DebuggerValue(HINT_BASE_ID + i))
            .collect()
    }

    fn bump_and_clear(&mut self, editor: &Entity<Editor>, cx: &mut Context<Self>) {
        self.refresh_rev = self.refresh_rev.wrapping_add(1);
        let _ = editor.update(cx, |ed, cx| {
            ed.splice_inlays(&Self::removal_ids(), Vec::new(), cx)
        });
    }

    fn is_singleton(editor: &Entity<Editor>, cx: &mut Context<Self>) -> bool {
        editor.read_with(cx, |ed, app| ed.buffer().read(app).as_singleton().is_some())
    }

    fn inlays_enabled(&self, editor: &Entity<Editor>, cx: &mut Context<Self>) -> bool {
        self.enabled && editor.read(cx).inlay_hints_enabled()
    }

    fn debounce_for_event(
        &self,
        editor: &Entity<Editor>,
        event: &EditorEvent,
        cx: &mut Context<Self>,
    ) -> Duration {
        let (edit_ms, scroll_ms) = editor.read_with(cx, |_ed, app| {
            let all_settings = all_language_settings(None, app);
            let s = &all_settings.defaults.inlay_hints;
            (s.edit_debounce_ms, s.scroll_debounce_ms)
        });
        match event {
            EditorEvent::ScrollPositionChanged { .. } => Duration::from_millis(scroll_ms),
            _ => Duration::from_millis(edit_ms),
        }
    }

    fn edit_debounce(&self, editor: &Entity<Editor>, cx: &mut Context<Self>) -> Duration {
        let (edit_ms, _) = editor.read_with(cx, |_ed, app| {
            let all_settings = all_language_settings(None, app);
            let s = &all_settings.defaults.inlay_hints;
            (s.edit_debounce_ms, s.scroll_debounce_ms)
        });
        Duration::from_millis(edit_ms)
    }

    fn flatten_document_symbols(
        mut doc_symbols: Vec<project::DocumentSymbol>,
    ) -> Vec<project::DocumentSymbol> {
        let mut flat_syms: Vec<project::DocumentSymbol> = Vec::new();
        let mut stack: Vec<project::DocumentSymbol> = Vec::new();
        stack.append(&mut doc_symbols);
        while let Some(mut sym) = stack.pop() {
            if !sym.children.is_empty() {
                for child in sym.children.iter().cloned() {
                    stack.push(child);
                }
            }
            sym.children.clear();
            flat_syms.push(sym);
        }
        flat_syms
    }

    fn on_symbols_changed(
        &mut self,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
        event: &EditorEvent,
    ) {
        // Respect our toggle and core inlay hints
        // If core inlay hints were just disabled, clear immediately.
        if let EditorEvent::InlayHintsToggled { enabled } = event {
            if !enabled {
                self.bump_and_clear(editor, cx);
                return;
            }
        }

        if !self.inlays_enabled(editor, cx) {
            self.bump_and_clear(editor, cx);
            return;
        }

        // Skip and clear when MultiBuffer contains more than one excerpt (multi-buffer sources)
        if !Self::is_singleton(editor, cx) {
            self.bump_and_clear(editor, cx);
            return;
        }

        // Use inlay-hint-like debounce: scroll vs edit
        let debounce = self.debounce_for_event(editor, event, cx);
        self.refresh_symbol_ref_hints(editor, window, cx, debounce);
    }

    fn refresh_symbol_ref_hints(
        &mut self,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
        debounce: Duration,
    ) {
        // If not a singleton multibuffer, clear and bail.
        if !Self::is_singleton(editor, cx) {
            self.bump_and_clear(editor, cx);
            self.cancel_task();
            return;
        }

        // Capture the active excerpt, buffer and its outline items synchronously.
        let maybe_data = editor
            .read(cx)
            .active_excerpt(cx)
            .map(|(excerpt_id, buffer, _)| {
                let items = buffer.read(cx).snapshot().outline(None).items;
                (excerpt_id, buffer, items)
            });
        let Some((excerpt_id, buffer, items)) = maybe_data else {
            return;
        };
        let project = self.project.clone();
        let editor_handle = editor.clone();

        // Debounce to align with inlay-hints cadence
        let rev = self.refresh_rev;
        self.ongoing_task = cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(debounce).await;

            // If disabled or invalidated since we started, do nothing.
            let inlay_enabled = editor_handle
                .read_with(cx, |ed, _| ed.inlay_hints_enabled())
                .unwrap_or(false);
            let our_enabled = this.update(cx, |this, _| this.enabled).unwrap_or(true);
            if !(our_enabled && inlay_enabled) {
                return;
            }
            let invalidated = this
                .update(cx, |this, _| this.refresh_rev != rev)
                .unwrap_or(true);
            if invalidated {
                return;
            }

            // Prefer querying references at the symbol's identifier using LSP document symbols,
            // falling back to the outline item's start if we can't find a matching symbol.
            let doc_symbols = if let Some(task) = project
                .update(cx, |p, cx| p.document_symbols(&buffer, cx))
                .ok()
            {
                (task.await).unwrap_or_default()
            } else {
                Vec::new()
            };

            // Flatten nested document symbols for easier matching.
            let flat_syms = Self::flatten_document_symbols(doc_symbols);

            // Compute, for each outline item, the position at which to ask for references.
            // We do this inside a read_with closure to access the App and buffer snapshot.
            let positions = editor_handle
                .read_with(cx, |_, app| {
                    let snapshot = buffer.read(app).snapshot();
                    items
                        .iter()
                        .map(|item| {
                            let item_off = item.range.start.to_offset(&snapshot);
                            // Find the smallest containing document symbol (closest match)
                            let mut best_sym: Option<&project::DocumentSymbol> = None;
                            for s in &flat_syms {
                                let rs = s.range.start.to_offset(&snapshot);
                                let re = s.range.end.to_offset(&snapshot);
                                if rs <= item_off && item_off < re {
                                    match &best_sym {
                                        None => best_sym = Some(s),
                                        Some(prev) => {
                                            let prev_span = prev.range.end.to_offset(&snapshot)
                                                - prev.range.start.to_offset(&snapshot);
                                            let this_span = re - rs;
                                            if this_span <= prev_span {
                                                best_sym = Some(s);
                                            }
                                        }
                                    }
                                }
                            }
                            // Return a Point for the symbol selection if found; otherwise, the outline start.
                            match best_sym {
                                Some(sym) => sym.selection_range.start.to_point(&snapshot),
                                None => item.range.start.to_point(&snapshot),
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            // Query references for each position and count the results.
            let mut counts: Vec<usize> = Vec::with_capacity(items.len());
            for pos in &positions {
                let n = if let Some(task) = project
                    .update(cx, |p, cx| p.references(&buffer, *pos, cx))
                    .ok()
                {
                    match task.await {
                        Ok(Some(locations)) => locations.len(),
                        Ok(None) => 0,
                        Err(_) => 0,
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
                            let pos =
                                mb_snapshot.anchor_in_excerpt(excerpt_id, item.range.start)?;
                            let text = format!("{} ", counts[i]);
                            Some(Inlay::debugger(HINT_BASE_ID + i, pos, text))
                        })
                        .collect::<Vec<Inlay>>()
                })
                .unwrap_or_default();

            // If disabled or invalidated since we computed, skip applying.
            let inlay_enabled = editor_handle
                .read_with(cx, |ed, _| ed.inlay_hints_enabled())
                .unwrap_or(false);
            let our_enabled = this.update(cx, |this, _| this.enabled).unwrap_or(true);
            if inlays.is_empty() || !(our_enabled && inlay_enabled) {
                return;
            }
            let invalidated = this
                .update(cx, |this, _| this.refresh_rev != rev)
                .unwrap_or(true);
            if invalidated {
                return;
            }

            let _ = editor_handle.update(cx, |ed, cx| {
                ed.splice_inlays(&Self::removal_ids(), inlays, cx)
            });
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
        // Cancel any previous pending task tied to a different editor.
        self.cancel_task();
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            // Observe editor events related to syntax/outline updates.
            self._observe_active_editor = Some(cx.subscribe_in(
                &editor,
                window,
                |this, editor, event: &EditorEvent, window, cx| match event {
                    EditorEvent::Reparsed(_)
                    | EditorEvent::ExcerptsEdited { .. }
                    | EditorEvent::Edited { .. }
                    | EditorEvent::BufferEdited
                    | EditorEvent::Saved
                    | EditorEvent::ScrollPositionChanged { .. }
                    | EditorEvent::InlayHintsToggled { .. } => {
                        this.on_symbols_changed(&editor, window, cx, event);
                    }
                    _ => {}
                },
            ));

            // Observe settings changes to apply/remove hints immediately.
            let editor_for_settings = editor.clone();
            self._observe_settings = Some(cx.observe_global_in::<settings::SettingsStore>(
                window,
                move |this, window, cx| {
                    let our_enabled = this.enabled;
                    let inlay_enabled = editor_for_settings.read(cx).inlay_hints_enabled();
                    let is_singleton = editor_for_settings
                        .read_with(cx, |ed, app| ed.buffer().read(app).as_singleton().is_some());
                    if !(our_enabled && inlay_enabled) || !is_singleton {
                        this.bump_and_clear(&editor_for_settings, cx);
                        this.cancel_task();
                    } else {
                        // Request immediate refresh when enabling
                        let debounce = this.edit_debounce(&editor_for_settings, cx);
                        this.refresh_symbol_ref_hints(&editor_for_settings, window, cx, debounce);
                    }
                },
            ));

            // Prime once on activation.
            let debounce = self.edit_debounce(&editor, cx);
            self.refresh_symbol_ref_hints(&editor, window, cx, debounce);

            // Ensure a follow-up refresh after initial parse by triggering a reparse now.
            if self.enabled && editor.read(cx).inlay_hints_enabled() {
                let _ = editor.update(cx, |ed, cx| {
                    ed.buffer().update(cx, |mb, cx| {
                        if let Some(buffer) = mb.as_singleton() {
                            buffer.update(cx, |b, cx| b.reparse(cx));
                        }
                    });
                });
            }
        } else {
            // Clear subscription when no active editor and cancel any pending task
            self._observe_active_editor = None;
            self._observe_settings = None;
            self.cancel_task();
        }
        cx.notify();
    }
}
