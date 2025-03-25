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
use project::{ProjectPath, Symbol, WorktreeId};
use rope::Point;
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

        let search_task = search_symbols(query, &workspace, cx);

        cx.spawn_in(window, async move |this, cx| {
            // TODO: This should be probably be run in the background.
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

        let Some(task) = self
            .context_store
            .update(cx, |context_store, cx| {
                context_store.add_symbol(
                    mat.path.clone(),
                    mat.symbol.range.start.text_anchor..mat.symbol.range.end.text_anchor,
                    cx,
                )
            })
            .ok()
        else {
            return;
        };

        let confirm_behavior = self.confirm_behavior;
        cx.spawn_in(window, async move |this, cx| {
            match task.await.notify_async_err(cx) {
                None => anyhow::Ok(()),
                Some(()) => this.update_in(cx, |this, window, cx| match confirm_behavior {
                    ConfirmBehavior::KeepOpen => {}
                    ConfirmBehavior::Close => this.delegate.dismissed(window, cx),
                }),
            }
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
    pub symbol: OutlineItem<Anchor>,
    pub path: ProjectPath,
    pub range: Range<Point>,
}

pub(crate) fn search_symbols(
    query: String,
    workspace: &Entity<Workspace>,
    cx: &mut App,
) -> Task<Result<Vec<SymbolMatch>>> {
    let Some(editor) = workspace.read(cx).active_item_as::<Editor>(cx) else {
        return Task::ready(Ok(Vec::new()));
    };

    let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);

    let Some(outline) = snapshot.outline(None) else {
        return Task::ready(Ok(Vec::new()));
    };

    let Some(path) = editor.read(cx).project_path(cx) else {
        return Task::ready(Ok(Vec::new()));
    };

    cx.spawn(async move |cx| {
        let task = outline.search(&query, cx.background_executor().clone());
        let matches = task.await;
        Ok(matches
            .into_iter()
            .map(|mat| {
                let symbol = outline.items[mat.candidate_id].clone();

                SymbolMatch {
                    mat,
                    range: symbol.range.to_point(&snapshot),
                    path: path.clone(),
                    symbol,
                }
            })
            .collect::<Vec<_>>())
    })

    // let symbols_task = project.update(cx, |project, cx| project.symbols(&query, cx));
    // cx.spawn(async move |cx| {
    //     let symbols = symbols_task.await?;
    //     let (visible_match_candidates, external_match_candidates): (Vec<_>, Vec<_>) = project
    //         .update(cx, |project, cx| {
    //             symbols
    //                 .iter()
    //                 .enumerate()
    //                 .map(|(id, symbol)| StringMatchCandidate::new(id, &symbol.label.filter_text()))
    //                 .partition(|candidate| {
    //                     project
    //                         .entry_for_path(&symbols[candidate.id].path, cx)
    //                         .map_or(false, |e| !e.is_ignored)
    //                 })
    //         })?;

    //     const MAX_MATCHES: usize = 100;
    //     let mut visible_matches = cx.background_executor().block(fuzzy::match_strings(
    //         &visible_match_candidates,
    //         &query,
    //         false,
    //         MAX_MATCHES,
    //         &cancellation_flag,
    //         cx.background_executor().clone(),
    //     ));
    //     let mut external_matches = cx.background_executor().block(fuzzy::match_strings(
    //         &external_match_candidates,
    //         &query,
    //         false,
    //         MAX_MATCHES - visible_matches.len().min(MAX_MATCHES),
    //         &cancellation_flag,
    //         cx.background_executor().clone(),
    //     ));
    //     let sort_key_for_match = |mat: &StringMatch| {
    //         let symbol = &symbols[mat.candidate_id];
    //         (Reverse(OrderedFloat(mat.score)), symbol.label.filter_text())
    //     };

    //     visible_matches.sort_unstable_by_key(sort_key_for_match);
    //     external_matches.sort_unstable_by_key(sort_key_for_match);
    //     let mut matches = visible_matches;
    //     matches.append(&mut external_matches);

    //     Ok(matches
    //         .into_iter()
    //         .map(|mut mat| {
    //             let symbol = symbols[mat.candidate_id].clone();
    //             let filter_start = symbol.label.filter_range.start;
    //             for position in &mut mat.positions {
    //                 *position += filter_start;
    //             }
    //             (mat, symbol)
    //         })
    //         .collect())
    // })
}

pub fn render_symbol_context_entry(
    id: ElementId,
    mat: &SymbolMatch,
    context_store: WeakEntity<ContextStore>,
    cx: &App,
) -> Stateful<Div> {
    // let added = context_store.upgrade().and_then(|context_store| {
    //     context_store
    //         .read(cx)
    //         .included_th(path)
    //         .map(FileInclusion::Direct)
    // });
    let path = mat
        .path
        .path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_default();
    let symbol_location = if mat.range.start.row == mat.range.end.row {
        format!("{} L{}", path, mat.range.start.row)
    } else {
        format!("{} L{}-{}", path, mat.range.start.row, mat.range.end.row)
    };

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
            h_flex().gap_1().child(Label::new(&mat.symbol.text)).child(
                Label::new(symbol_location)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            ),
        )
    // .when_some(added, |el, added| match added {
    //     FileInclusion::Direct(_) => el.child(
    //         h_flex()
    //             .w_full()
    //             .justify_end()
    //             .gap_0p5()
    //             .child(
    //                 Icon::new(IconName::Check)
    //                     .size(IconSize::Small)
    //                     .color(Color::Success),
    //             )
    //             .child(Label::new("Added").size(LabelSize::Small)),
    //     ),
    //     FileInclusion::InDirectory(dir_name) => {
    //         let dir_name = dir_name.to_string_lossy().into_owned();

    //         el.child(
    //             h_flex()
    //                 .w_full()
    //                 .justify_end()
    //                 .gap_0p5()
    //                 .child(
    //                     Icon::new(IconName::Check)
    //                         .size(IconSize::Small)
    //                         .color(Color::Success),
    //                 )
    //                 .child(Label::new("Included").size(LabelSize::Small)),
    //         )
    //         .tooltip(Tooltip::text(format!("in {dir_name}")))
    //     }
    // })
}
