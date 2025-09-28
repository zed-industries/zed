use editor::{display_map::Inlay, Editor, EditorEvent, InlayId};
use gpui::{Context, Entity, Render, Subscription, Task, Window};
use project::Project;
use std::time::Duration;
use ui::prelude::*;
use language::language_settings::{all_language_settings};

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
                self.refresh_rev = self.refresh_rev.wrapping_add(1);
                let _ = editor.update(cx, |ed, cx| {
                    let to_remove: Vec<InlayId> = (0..MAX_REMOVE)
                        .map(|i| InlayId::DebuggerValue(HINT_BASE_ID + i))
                        .collect();
                    ed.splice_inlays(&to_remove, Vec::new(), cx);
                });
                return;
            }
        }

        let inlay_enabled = editor.read(cx).inlay_hints_enabled();
        if !(self.enabled && inlay_enabled) {
            self.refresh_rev = self.refresh_rev.wrapping_add(1);
            let _ = editor.update(cx, |ed, cx| {
                let to_remove: Vec<InlayId> = (0..MAX_REMOVE)
                    .map(|i| InlayId::DebuggerValue(HINT_BASE_ID + i))
                    .collect();
                ed.splice_inlays(&to_remove, Vec::new(), cx);
            });
            return;
        }
        // Skip and clear when MultiBuffer contains more than one excerpt (multi-buffer sources)
        let is_singleton = editor
            .read_with(cx, |ed, app| ed.buffer().read(app).as_singleton().is_some());
        if !is_singleton {
            self.refresh_rev = self.refresh_rev.wrapping_add(1);
            let _ = editor.update(cx, |ed, cx| {
                let to_remove: Vec<InlayId> = (0..MAX_REMOVE)
                    .map(|i| InlayId::DebuggerValue(HINT_BASE_ID + i))
                    .collect();
                ed.splice_inlays(&to_remove, Vec::new(), cx);
            });
            return;
        }

        // Use inlay-hint-like debounce: scroll vs edit
        let (edit_ms, scroll_ms) = editor.read_with(cx, |_ed, app| {
            let als = all_language_settings(None, app);
            let s = &als.defaults.inlay_hints;
            // Don't block if the editor itself disables debounce (0)
            let edit_ms = if s.edit_debounce_ms == 0 { 0 } else { s.edit_debounce_ms };
            let scroll_ms = if s.scroll_debounce_ms == 0 { 0 } else { s.scroll_debounce_ms };
            (edit_ms, scroll_ms)
        });
        let debounce = match event {
            EditorEvent::ScrollPositionChanged { .. } => Duration::from_millis(scroll_ms),
            _ => Duration::from_millis(edit_ms),
        };
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
        let is_singleton = editor
            .read_with(cx, |ed, app| ed.buffer().read(app).as_singleton().is_some());
        if !is_singleton {
            self.refresh_rev = self.refresh_rev.wrapping_add(1);
            let _ = editor.update(cx, |ed, cx| {
                let to_remove: Vec<InlayId> = (0..MAX_REMOVE)
                    .map(|i| InlayId::DebuggerValue(HINT_BASE_ID + i))
                    .collect();
                ed.splice_inlays(&to_remove, Vec::new(), cx);
            });
            return;
        }

        // Capture the active excerpt, buffer and its outline items synchronously.
        let maybe_data = editor.read(cx).active_excerpt(cx).and_then(|(excerpt_id, buffer, _)| {
            let items = buffer.read(cx).snapshot().outline(None).items;
            Some((excerpt_id, buffer, items))
        });
        let Some((excerpt_id, buffer, items)) = maybe_data else { return; };
        let project = self.project.clone();
        let editor_handle = editor.clone();

        // Debounce to align with inlay-hints cadence
        let rev = self.refresh_rev;
        self.ongoing_task = cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(debounce).await;

            // If disabled or invalidated since we started, do nothing.
            let inlay_enabled = editor_handle.read_with(cx, |ed, _| ed.inlay_hints_enabled()).unwrap_or(false);
            let our_enabled = this.update(cx, |this, _| this.enabled).unwrap_or(true);
            if !(our_enabled && inlay_enabled) { return; }
            let invalidated = this.update(cx, |this, _| this.refresh_rev != rev).unwrap_or(true);
            if invalidated { return; }

            // Prefer querying references at the symbol's identifier using LSP document symbols,
            // falling back to the outline item's start if we can't find a matching symbol.
            let doc_symbols = if let Some(task) = project
                .update(cx, |p, cx| p.document_symbols(&buffer, cx))
                .ok()
            {
                match task.await {
                    Ok(symbols) => symbols,
                    Err(_) => Vec::new(),
                }
            } else {
                Vec::new()
            };

            // Flatten nested document symbols for easier matching.
            let mut flat_syms: Vec<project::DocumentSymbol> = Vec::new();
            let mut stack: Vec<project::DocumentSymbol> = doc_symbols;
            while let Some(sym) = stack.pop() {
                if !sym.children.is_empty() {
                    for child in sym.children.iter().cloned() {
                        stack.push(child);
                    }
                }
                let mut sym_no_children = sym.clone();
                sym_no_children.children.clear();
                flat_syms.push(sym_no_children);
            }

            // Compute, for each outline item, the position at which to ask for references.
            // We do this inside a read_with closure to access the App and buffer snapshot.
            let positions = editor_handle
                .read_with(cx, |ed, app| {
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
                            let pos = mb_snapshot.anchor_in_excerpt(excerpt_id, item.range.start)?;
                            let text = format!("{} ", counts[i]);
                            Some(Inlay::debugger(HINT_BASE_ID + i, pos, text))
                        })
                        .collect::<Vec<Inlay>>()
                })
                .unwrap_or_default();

            // If disabled or invalidated since we computed, skip applying.
            let inlay_enabled = editor_handle.read_with(cx, |ed, _| ed.inlay_hints_enabled()).unwrap_or(false);
            let our_enabled = this.update(cx, |this, _| this.enabled).unwrap_or(true);
            if inlays.is_empty() || !(our_enabled && inlay_enabled) { return; }
            let invalidated = this.update(cx, |this, _| this.refresh_rev != rev).unwrap_or(true);
            if invalidated { return; }

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
                    | EditorEvent::Saved
                    | EditorEvent::ScrollPositionChanged { .. }
                    | EditorEvent::InlayHintsToggled { .. } => {
                        this.on_symbols_changed(&editor, window, cx, event);
                    }
                    _ => {}
                }
            }));

            // Observe settings changes to apply/remove hints immediately.
            let editor_for_settings = editor.clone();
            self._observe_settings = Some(cx.observe_global_in::<settings::SettingsStore>(window, move |this, window, cx| {
                let our_enabled = this.enabled;
                let inlay_enabled = editor_for_settings.read(cx).inlay_hints_enabled();
                let is_singleton = editor_for_settings
                    .read_with(cx, |ed, app| ed.buffer().read(app).as_singleton().is_some());
                if !(our_enabled && inlay_enabled) || !is_singleton {
                    this.refresh_rev = this.refresh_rev.wrapping_add(1);
                    let _ = editor_for_settings.update(cx, |ed, cx| {
                        let to_remove: Vec<InlayId> = (0..MAX_REMOVE)
                            .map(|i| InlayId::DebuggerValue(HINT_BASE_ID + i))
                            .collect();
                        ed.splice_inlays(&to_remove, Vec::new(), cx);
                    });
                } else {
                    // Request immediate refresh when enabling
                    let (edit_ms, _) = editor_for_settings.read_with(cx, |_ed, app| {
                        let als = all_language_settings(None, app);
                        let s = &als.defaults.inlay_hints;
                        (s.edit_debounce_ms, s.scroll_debounce_ms)
                    });
                    this.refresh_symbol_ref_hints(&editor_for_settings, window, cx, Duration::from_millis(edit_ms));
                }
            }));

            // Prime once on activation.
            let (edit_ms, _) = editor.read_with(cx, |_ed, app| {
                let als = all_language_settings(None, app);
                let s = &als.defaults.inlay_hints;
                (s.edit_debounce_ms, s.scroll_debounce_ms)
            });
            self.refresh_symbol_ref_hints(&editor, window, cx, Duration::from_millis(edit_ms));

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
            // Clear subscription when no active editor.
            self._observe_active_editor = None;
            self._observe_settings = None;
        }
        cx.notify();
    }
}

