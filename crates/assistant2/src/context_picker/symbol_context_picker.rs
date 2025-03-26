use std::cmp::Reverse;
use std::ops::Range;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use editor::{Anchor, AnchorRangeExt, Editor, MultiBufferSnapshot};
use file_icons::FileIcons;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, AppContext, DismissEvent, Entity, FocusHandle, Focusable, Stateful, Task, WeakEntity,
};
use language::OutlineItem;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use project::{DocumentSymbol, Symbol};
use ui::{prelude::*, ListItem, Tooltip};
use util::ResultExt as _;
use workspace::{notifications::NotifyResultExt, Workspace};

use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::{ContextStore, FileInclusion};

pub struct SymbolContextPicker {
    picker: Entity<Picker<SymbolContextPickerDelegate>>,
}

impl SymbolContextPicker {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
        confirm_behavior: ConfirmBehavior,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = SymbolContextPickerDelegate::new(
            context_picker,
            workspace,
            context_store,
            confirm_behavior,
        );
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
    confirm_behavior: ConfirmBehavior,
    matches: Vec<SymbolMatch>,
    selected_index: usize,
}

impl SymbolContextPickerDelegate {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
        confirm_behavior: ConfirmBehavior,
    ) -> Self {
        Self {
            context_picker,
            workspace,
            context_store,
            confirm_behavior,
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
        cx.spawn_in(window, async move |this, cx| {
            let symbols = search_task
                .await
                .context("Failed to load symbols")
                .log_err();

            this.update(cx, |this, _cx| {
                this.delegate.matches = symbols.unwrap_or_default();
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let project = workspace.read(cx).project().clone();
        let path = mat.symbol.path.clone();
        let symbol = mat.symbol.clone();
        let context_store = self.context_store.clone();
        let confirm_behavior = self.confirm_behavior;

        let open_buffer_task = project.update(cx, |project, cx| project.open_buffer(path, cx));
        cx.spawn_in(window, async move |this, cx| {
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
                find_matching_symbol(&symbol, &document_symbols)
            {
                (name, selection_range, range)
            } else {
                // If we do not find a matching document symbol, fall back to
                // just the symbol itself
                (symbol.name, symbol.range.clone(), symbol.range)
            };

            let (range, enclosing_range) = buffer.read_with(cx, |buffer, cx| {
                (
                    buffer.anchor_after(range.start)..buffer.anchor_before(range.end),
                    buffer.anchor_after(enclosing_range.start)
                        ..buffer.anchor_before(enclosing_range.end),
                )
            })?;

            context_store
                .update(cx, move |context_store, cx| {
                    context_store.add_symbol(buffer, name.into(), range, enclosing_range, cx)
                })?
                .await?;

            this.update_in(cx, |this, window, cx| match confirm_behavior {
                ConfirmBehavior::KeepOpen => {}
                ConfirmBehavior::Close => this.delegate.dismissed(window, cx),
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
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];

        Some(ListItem::new(ix).inset(true).toggle_state(selected).child(
            render_symbol_context_entry(
                ElementId::NamedInteger("symbol-ctx-picker".into(), ix),
                mat,
                self.context_store.clone(),
                cx,
            ),
        ))
    }
}

pub(crate) struct SymbolMatch {
    pub mat: StringMatch,
    pub symbol: Symbol,
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

pub(crate) fn search_symbols(
    query: String,
    cancellation_flag: Arc<AtomicBool>,
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Task<Result<Vec<SymbolMatch>>> {
    let symbols_task = workspace.update(cx, |workspace, cx| {
        workspace
            .project()
            .update(cx, |project, cx| project.symbols(&query, cx))
    });
    let project = workspace.read(cx).project().clone();
    cx.spawn(async move |cx| {
        let symbols = symbols_task.await?;
        let (visible_match_candidates, external_match_candidates): (Vec<_>, Vec<_>) = project
            .update(cx, |project, cx| {
                symbols
                    .iter()
                    .enumerate()
                    .map(|(id, symbol)| StringMatchCandidate::new(id, &symbol.label.filter_text()))
                    .partition(|candidate| {
                        project
                            .entry_for_path(&symbols[candidate.id].path, cx)
                            .map_or(false, |e| !e.is_ignored)
                    })
            })?;

        const MAX_MATCHES: usize = 100;
        let mut visible_matches = cx.background_executor().block(fuzzy::match_strings(
            &visible_match_candidates,
            &query,
            false,
            MAX_MATCHES,
            &cancellation_flag,
            cx.background_executor().clone(),
        ));
        let mut external_matches = cx.background_executor().block(fuzzy::match_strings(
            &external_match_candidates,
            &query,
            false,
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

        Ok(matches
            .into_iter()
            .map(|mut mat| {
                let symbol = symbols[mat.candidate_id].clone();
                let filter_start = symbol.label.filter_range.start;
                for position in &mut mat.positions {
                    *position += filter_start;
                }
                SymbolMatch { mat, symbol }
            })
            .collect())
    })
}

pub fn render_symbol_context_entry(
    id: ElementId,
    mat: &SymbolMatch,
    context_store: WeakEntity<ContextStore>,
    cx: &App,
) -> Stateful<Div> {
    //TODO: Check if symbol is added
    let path = mat
        .symbol
        .path
        .path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_default();
    let symbol_location = format!("{} L{}", path, mat.symbol.range.start.0.row);

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
            h_flex().gap_1().child(Label::new(&mat.symbol.name)).child(
                Label::new(symbol_location)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            ),
        )
}
