use std::cmp::Reverse;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::Result;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, AppContext, DismissEvent, Entity, FocusHandle, Focusable, Stateful, Task, WeakEntity,
};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use project::{DocumentSymbol, Symbol};
use ui::{ListItem, prelude::*};
use util::ResultExt as _;
use workspace::Workspace;

use crate::context_picker::ContextPicker;
use agent::context::AgentContextHandle;
use agent::context_store::ContextStore;

pub struct SymbolContextPicker {
    picker: Entity<Picker<SymbolContextPickerDelegate>>,
}

impl SymbolContextPicker {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = SymbolContextPickerDelegate::new(context_picker, workspace, context_store);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        Self { picker }
    }
}

impl Focusable for SymbolContextPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for SymbolContextPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

pub struct SymbolContextPickerDelegate {
    context_picker: WeakEntity<ContextPicker>,
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    matches: Vec<SymbolEntry>,
    selected_index: usize,
}

impl SymbolContextPickerDelegate {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
    ) -> Self {
        Self {
            context_picker,
            workspace,
            context_store,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for SymbolContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search symbols…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(());
        };

        let search_task = search_symbols(query, Arc::<AtomicBool>::default(), &workspace, cx);
        let context_store = self.context_store.clone();
        cx.spawn_in(window, async move |this, cx| {
            let symbols = search_task.await;

            let symbol_entries = context_store
                .read_with(cx, |context_store, cx| {
                    compute_symbol_entries(symbols, context_store, cx)
                })
                .log_err()
                .unwrap_or_default();

            this.update(cx, |this, _cx| {
                this.delegate.matches = symbol_entries;
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let add_symbol_task = add_symbol(
            mat.symbol.clone(),
            true,
            workspace,
            self.context_store.clone(),
            cx,
        );

        let selected_index = self.selected_index;
        cx.spawn(async move |this, cx| {
            let (_, included) = add_symbol_task.await?;
            this.update(cx, |this, _| {
                if let Some(mat) = this.delegate.matches.get_mut(selected_index) {
                    mat.is_included = included;
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.context_picker
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];

        Some(ListItem::new(ix).inset(true).toggle_state(selected).child(
            render_symbol_context_entry(ElementId::named_usize("symbol-ctx-picker", ix), mat),
        ))
    }
}

pub(crate) struct SymbolEntry {
    pub symbol: Symbol,
    pub is_included: bool,
}

pub(crate) fn add_symbol(
    symbol: Symbol,
    remove_if_exists: bool,
    workspace: Entity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    cx: &mut App,
) -> Task<Result<(Option<AgentContextHandle>, bool)>> {
    let project = workspace.read(cx).project().clone();
    let open_buffer_task = project.update(cx, |project, cx| {
        project.open_buffer(symbol.path.clone(), cx)
    });
    cx.spawn(async move |cx| {
        let buffer = open_buffer_task.await?;
        let document_symbols = project
            .update(cx, |project, cx| project.document_symbols(&buffer, cx))?
            .await?;

        // Try to find a matching document symbol. Document symbols include
        // not only the symbol itself (e.g. function name), but they also
        // include the context that they contain (e.g. function body).
        let (name, range, enclosing_range) = if let Some(DocumentSymbol {
            name,
            range,
            selection_range,
            ..
        }) =
            find_matching_symbol(&symbol, document_symbols.as_slice())
        {
            (name, selection_range, range)
        } else {
            // If we do not find a matching document symbol, fall back to
            // just the symbol itself
            (symbol.name, symbol.range.clone(), symbol.range)
        };

        let (range, enclosing_range) = buffer.read_with(cx, |buffer, _| {
            (
                buffer.anchor_after(range.start)..buffer.anchor_before(range.end),
                buffer.anchor_after(enclosing_range.start)
                    ..buffer.anchor_before(enclosing_range.end),
            )
        })?;

        context_store.update(cx, move |context_store, cx| {
            context_store.add_symbol(
                buffer,
                name.into(),
                range,
                enclosing_range,
                remove_if_exists,
                cx,
            )
        })
    })
}

fn find_matching_symbol(symbol: &Symbol, candidates: &[DocumentSymbol]) -> Option<DocumentSymbol> {
    let mut candidates = candidates.iter();
    let mut candidate = candidates.next()?;

    loop {
        if candidate.range.start > symbol.range.end {
            return None;
        }
        if candidate.range.end < symbol.range.start {
            candidate = candidates.next()?;
            continue;
        }
        if candidate.selection_range == symbol.range {
            return Some(candidate.clone());
        }
        if candidate.range.start <= symbol.range.start && symbol.range.end <= candidate.range.end {
            candidates = candidate.children.iter();
            candidate = candidates.next()?;
            continue;
        }
        return None;
    }
}

pub struct SymbolMatch {
    pub symbol: Symbol,
}

pub(crate) fn search_symbols(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Task<Vec<SymbolMatch>> {
    let symbols_task = workspace.update(cx, |workspace, cx| {
        workspace
            .project()
            .update(cx, |project, cx| project.symbols(&query, cx))
    });
    let project = workspace.read(cx).project().clone();
    cx.spawn(async move |cx| {
        let Some(symbols) = symbols_task.await.log_err() else {
            return Vec::new();
        };
        let Some((visible_match_candidates, external_match_candidates)): Option<(Vec<_>, Vec<_>)> =
            project
                .update(cx, |project, cx| {
                    symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| {
                            StringMatchCandidate::new(id, symbol.label.filter_text())
                        })
                        .partition(|candidate| {
                            project
                                .entry_for_path(&symbols[candidate.id].path, cx)
                                .is_some_and(|e| !e.is_ignored)
                        })
                })
                .log_err()
        else {
            return Vec::new();
        };

        const MAX_MATCHES: usize = 100;
        let mut visible_matches = cx.background_executor().block(fuzzy::match_strings(
            &visible_match_candidates,
            &query,
            false,
            true,
            MAX_MATCHES,
            &cancellation_flag,
            cx.background_executor().clone(),
        ));
        let mut external_matches = cx.background_executor().block(fuzzy::match_strings(
            &external_match_candidates,
            &query,
            false,
            true,
            MAX_MATCHES - visible_matches.len().min(MAX_MATCHES),
            &cancellation_flag,
            cx.background_executor().clone(),
        ));
        let sort_key_for_match = |mat: &StringMatch| {
            let symbol = &symbols[mat.candidate_id];
            (Reverse(OrderedFloat(mat.score)), symbol.label.filter_text())
        };

        visible_matches.sort_unstable_by_key(sort_key_for_match);
        external_matches.sort_unstable_by_key(sort_key_for_match);
        let mut matches = visible_matches;
        matches.append(&mut external_matches);

        matches
            .into_iter()
            .map(|mut mat| {
                let symbol = symbols[mat.candidate_id].clone();
                let filter_start = symbol.label.filter_range.start;
                for position in &mut mat.positions {
                    *position += filter_start;
                }
                SymbolMatch { symbol }
            })
            .collect()
    })
}

fn compute_symbol_entries(
    symbols: Vec<SymbolMatch>,
    context_store: &ContextStore,
    cx: &App,
) -> Vec<SymbolEntry> {
    symbols
        .into_iter()
        .map(|SymbolMatch { symbol, .. }| SymbolEntry {
            is_included: context_store.includes_symbol(&symbol, cx),
            symbol,
        })
        .collect::<Vec<_>>()
}

pub fn render_symbol_context_entry(id: ElementId, entry: &SymbolEntry) -> Stateful<Div> {
    let path = entry
        .symbol
        .path
        .path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_default();
    let symbol_location = format!("{} L{}", path, entry.symbol.range.start.0.row + 1);

    h_flex()
        .id(id)
        .gap_1p5()
        .w_full()
        .child(
            Icon::new(IconName::Code)
                .size(IconSize::Small)
                .color(Color::Muted),
        )
        .child(
            h_flex()
                .gap_1()
                .child(Label::new(&entry.symbol.name))
                .child(
                    Label::new(symbol_location)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
        .when(entry.is_included, |el| {
            el.child(
                h_flex()
                    .w_full()
                    .justify_end()
                    .gap_0p5()
                    .child(
                        Icon::new(IconName::Check)
                            .size(IconSize::Small)
                            .color(Color::Success),
                    )
                    .child(Label::new("Added").size(LabelSize::Small)),
            )
        })
}
