use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    BackgroundExecutor, DismissEvent, ElementId, Entity, Focusable, ForegroundExecutor, Task,
};
use picker::{Picker, PickerDelegate};
use ui::{
    Color, GradientFade, HighlightedLabel, Icon, IconButton, IconName, IconSize, Label,
    LabelSize, ListItem, ListItemSpacing, PopoverMenu, Tooltip, prelude::*,
};

use crate::{
    CsvPreviewView,
    settings::FilterSortOrder,
    table_data_engine::{
        filtering_by_column::{FilterEntry, FilterEntryState},
        sorting_by_column::{AppliedSorting, SortDirection},
    },
    types::AnyColumn,
};

struct ColumnFilterRow {
    entry: FilterEntry,
    is_applied: bool,
    /// Set when this value is hidden because another column's active filter
    /// excludes every row containing it.
    hidden_by: Option<AnyColumn>,
}

enum ColumnFilterListEntry {
    Header(SharedString),
    Row {
        row_index: usize,
        positions: Vec<usize>,
    },
}

struct ColumnFilterDelegate {
    col: AnyColumn,
    view: Entity<CsvPreviewView>,
    /// Row order frozen at open time (available entries sorted per the
    /// column's `FilterSortOrder`, then entries hidden by other columns'
    /// filters). Kept stable so toggling a value doesn't reshuffle the list
    /// under the user's cursor; only `is_applied`/`hidden_by`/counts are
    /// refreshed in place after a toggle.
    rows: Vec<ColumnFilterRow>,
    /// Number of available (non-hidden) rows, shown in the search placeholder.
    available_count: usize,
    string_candidates: Arc<Vec<StringMatchCandidate>>,
    filtered: Vec<ColumnFilterListEntry>,
    selected_index: usize,
    query: String,
    foreground: ForegroundExecutor,
    background: BackgroundExecutor,
    cancel: Option<Arc<AtomicBool>>,
}

impl ColumnFilterDelegate {
    fn new(
        col: AnyColumn,
        view: Entity<CsvPreviewView>,
        sort_order: FilterSortOrder,
        column_filters: Arc<Vec<(FilterEntry, FilterEntryState)>>,
        cx: &mut Context<Picker<Self>>,
    ) -> Self {
        let mut available: Vec<(FilterEntry, bool)> = Vec::new();
        let mut hidden: Vec<(FilterEntry, AnyColumn)> = Vec::new();
        for (entry, state) in column_filters.iter() {
            match state {
                FilterEntryState::Available { is_applied } => {
                    available.push((entry.clone(), *is_applied))
                }
                FilterEntryState::Unavailable { blocked_by } => {
                    hidden.push((entry.clone(), *blocked_by))
                }
            }
        }

        match sort_order {
            FilterSortOrder::AlphaThenCount => available.sort_by(|(a, a_app), (b, b_app)| {
                b_app
                    .cmp(a_app)
                    .then_with(|| a.content.cmp(&b.content))
                    .then_with(|| b.occurred_times().cmp(&a.occurred_times()))
            }),
            FilterSortOrder::CountThenAlpha => available.sort_by(|(a, a_app), (b, b_app)| {
                b_app
                    .cmp(a_app)
                    .then_with(|| b.occurred_times().cmp(&a.occurred_times()))
                    .then_with(|| a.content.cmp(&b.content))
            }),
        }

        let available_count = available.len();

        let rows: Vec<ColumnFilterRow> = available
            .into_iter()
            .map(|(entry, is_applied)| ColumnFilterRow {
                entry,
                is_applied,
                hidden_by: None,
            })
            .chain(hidden.into_iter().map(|(entry, blocked_by)| ColumnFilterRow {
                entry,
                is_applied: false,
                hidden_by: Some(blocked_by),
            }))
            .collect();

        let string_candidates = Arc::new(
            rows.iter()
                .enumerate()
                .map(|(index, row)| {
                    StringMatchCandidate::new(index, &Self::display_text(&row.entry))
                })
                .collect::<Vec<_>>(),
        );

        let filtered = Self::build_entries(
            &rows,
            rows.iter()
                .enumerate()
                .map(|(index, _)| (index, Vec::new()))
                .collect(),
        );

        Self {
            col,
            view,
            rows,
            available_count,
            string_candidates,
            filtered,
            selected_index: 0,
            query: String::new(),
            foreground: cx.foreground_executor().clone(),
            background: cx.background_executor().clone(),
            cancel: None,
        }
    }

    fn display_text(entry: &FilterEntry) -> String {
        match &entry.content {
            Some(s) => s.as_ref().to_owned(),
            None => "<null>".to_owned(),
        }
    }

    /// Assembles the flat list shown to the user from matched
    /// `(row_index, positions)` pairs (which must be sorted ascending by
    /// `row_index`, matching `rows`' available-then-hidden order), inserting a
    /// "Hidden by other filters" header before the first hidden row.
    fn build_entries(
        rows: &[ColumnFilterRow],
        matches: Vec<(usize, Vec<usize>)>,
    ) -> Vec<ColumnFilterListEntry> {
        let mut entries = Vec::with_capacity(matches.len() + 1);
        let mut header_inserted = false;
        for (row_index, positions) in matches {
            if rows[row_index].hidden_by.is_some() && !header_inserted {
                entries.push(ColumnFilterListEntry::Header(
                    "Hidden by other filters".into(),
                ));
                header_inserted = true;
            }
            entries.push(ColumnFilterListEntry::Row {
                row_index,
                positions,
            });
        }
        entries
    }

    fn first_selectable_index(&self) -> usize {
        self.filtered
            .iter()
            .position(|entry| {
                matches!(entry, ColumnFilterListEntry::Row { row_index, .. }
                    if self.rows[*row_index].hidden_by.is_none())
            })
            .unwrap_or(0)
    }

    fn matches_for_query(&self, query: &str) -> Vec<(usize, Vec<usize>)> {
        if query.is_empty() {
            return self
                .rows
                .iter()
                .enumerate()
                .map(|(index, _)| (index, Vec::new()))
                .collect();
        }

        let cancel_flag = AtomicBool::new(false);
        let mut matches: Vec<StringMatch> = self.foreground.block_on(match_strings(
            self.string_candidates.as_ref(),
            query,
            false,
            true,
            usize::MAX,
            &cancel_flag,
            self.background.clone(),
        ));
        matches.sort_by_key(|m| m.candidate_id);
        matches
            .into_iter()
            .map(|m| (m.candidate_id, m.positions))
            .collect()
    }

    /// Re-fetches this column's filter entries from the engine (e.g. after a
    /// toggle changes counts/availability across the cascade) while keeping
    /// `rows`' frozen order, then re-applies the current search query.
    fn refresh_rows(&mut self, cx: &mut Context<Picker<Self>>) {
        let column_filters = match self.view.read(cx).engine.get_filters_for_column(self.col) {
            Ok(filters) => filters,
            Err(err) => {
                log::error!("Failed to get filters for column: {err}");
                return;
            }
        };

        let mut lookup: HashMap<Option<SharedString>, (FilterEntry, FilterEntryState)> =
            column_filters
                .iter()
                .map(|(entry, state)| (entry.content.clone(), (entry.clone(), *state)))
                .collect();

        for row in &mut self.rows {
            let Some((entry, state)) = lookup.remove(&row.entry.content) else {
                continue;
            };
            row.entry = entry;
            match state {
                FilterEntryState::Available { is_applied } => {
                    row.is_applied = is_applied;
                    row.hidden_by = None;
                }
                FilterEntryState::Unavailable { blocked_by } => {
                    row.is_applied = false;
                    row.hidden_by = Some(blocked_by);
                }
            }
        }

        self.filtered = Self::build_entries(&self.rows, self.matches_for_query(&self.query));
        self.selected_index = self
            .selected_index
            .min(self.filtered.len().saturating_sub(1));
    }
}

impl PickerDelegate for ColumnFilterDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "csv column filter"
    }

    fn match_count(&self) -> usize {
        self.filtered.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered.len().saturating_sub(1));
        cx.notify();
    }

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        match self.filtered.get(ix) {
            Some(ColumnFilterListEntry::Row { row_index, .. }) => {
                self.rows[*row_index].hidden_by.is_none()
            }
            _ => false,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        format!("Search {} unique values…", self.available_count).into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.query = query.clone();

        if query.is_empty() {
            self.filtered = Self::build_entries(&self.rows, self.matches_for_query(&query));
            self.selected_index = self.first_selectable_index();
            cx.notify();
            return Task::ready(());
        }

        if let Some(prev) = &self.cancel {
            prev.store(true, Ordering::Relaxed);
        }
        let cancel = Arc::new(AtomicBool::new(false));
        self.cancel = Some(cancel.clone());

        let string_candidates = self.string_candidates.clone();
        let background = self.background.clone();

        cx.spawn_in(window, async move |this, cx| {
            let mut matches = match_strings(
                string_candidates.as_ref(),
                &query,
                false,
                true,
                usize::MAX,
                cancel.as_ref(),
                background,
            )
            .await;
            matches.sort_by_key(|m| m.candidate_id);

            this.update_in(cx, |this, _, cx| {
                if this.delegate.query != query {
                    return;
                }
                let rows = &this.delegate.rows;
                let matches = matches.into_iter().map(|m| (m.candidate_id, m.positions)).collect();
                this.delegate.filtered = ColumnFilterDelegate::build_entries(rows, matches);
                this.delegate.selected_index = this.delegate.first_selectable_index();
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(ColumnFilterListEntry::Row { row_index, .. }) =
            self.filtered.get(self.selected_index)
        else {
            return;
        };
        let row_index = *row_index;
        if self.rows[row_index].hidden_by.is_some() {
            return;
        }
        let col = self.col;
        let value = self.rows[row_index].entry.content.clone();
        self.view
            .update(cx, |view, cx| view.toggle_filter(col, value, cx));
        self.refresh_rows(cx);
        cx.notify();
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.filtered.get(ix)? {
            ColumnFilterListEntry::Header(label) => Some(
                div()
                    .px_2()
                    .pt_2()
                    .pb_1()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Label::new(label.clone())
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            ),
            ColumnFilterListEntry::Row {
                row_index,
                positions,
            } => {
                let row = &self.rows[*row_index];
                let value_text: SharedString = match &row.entry.content {
                    Some(s) => s.clone(),
                    None => "<null>".into(),
                };
                let count_text = SharedString::from(row.entry.occurred_times().to_string());
                let label = HighlightedLabel::new(value_text.clone(), positions.clone())
                    .size(LabelSize::Small)
                    .single_line()
                    .truncate();

                if row.hidden_by.is_some() {
                    return Some(
                        ListItem::new(("csv-filter-hidden", ix))
                            .disabled(true)
                            .inset(true)
                            .spacing(ListItemSpacing::Sparse)
                            .child(label.color(Color::Disabled))
                            .end_slot(
                                Label::new(count_text)
                                    .size(LabelSize::Small)
                                    .color(Color::Disabled),
                            )
                            .into_any_element(),
                    );
                }

                Some(
                    ListItem::new(("csv-filter-value", ix))
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .start_slot(
                            h_flex()
                                .flex_none()
                                .when(!row.is_applied, |el| el.invisible())
                                .child(
                                    Icon::new(IconName::Check)
                                        .size(IconSize::Small)
                                        .color(Color::Accent),
                                ),
                        )
                        .child(label)
                        .end_slot(
                            Label::new(count_text)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .tooltip(Tooltip::text(value_text))
                        .into_any_element(),
                )
            }
        }
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let selected_rows: usize = self
            .rows
            .iter()
            .filter(|row| row.hidden_by.is_none() && row.is_applied)
            .map(|row| row.entry.occurred_times())
            .sum();
        let total_rows: usize = self
            .rows
            .iter()
            .filter(|row| row.hidden_by.is_none())
            .map(|row| row.entry.occurred_times())
            .sum();
        if selected_rows == 0 {
            return None;
        }

        let col = self.col;
        Some(
            h_flex()
                .w_full()
                .px_2()
                .py_1()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .justify_between()
                .items_center()
                .child(
                    Label::new(format!("{selected_rows} / {total_rows} rows selected"))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    div()
                        .id("csv-filter-clear-all")
                        .cursor_pointer()
                        .child(
                            Label::new("Clear all")
                                .size(LabelSize::Small)
                                .color(Color::Accent),
                        )
                        .on_click(cx.listener(move |picker, _, _, cx| {
                            picker
                                .delegate
                                .view
                                .update(cx, |view, cx| view.clear_filters(col, cx));
                            picker.delegate.refresh_rows(cx);
                            cx.notify();
                            cx.emit(DismissEvent);
                        })),
                )
                .into_any(),
        )
    }
}

impl CsvPreviewView {
    /// Create header for data, which is orderable with text on the left and sort button on the right
    pub(crate) fn create_header_element_with_sort_button(
        &self,
        header_text: SharedString,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: AnyColumn,
    ) -> AnyElement {
        let has_active_filter = self.engine.has_active_filters(col_idx);
        let has_active_sort = self
            .engine
            .applied_sorting
            .is_some_and(|o| o.col_idx == col_idx);
        let always_show_buttons = has_active_filter || has_active_sort;
        let group_name = SharedString::from(format!("csv-col-header-{}", col_idx.get()));

        let colors = cx.theme().colors();
        let base_bg = colors.editor_background;
        let grad_width_hovered = px(100.);
        let grad_width = if always_show_buttons {
            grad_width_hovered
        } else {
            px(20.)
        };
        h_flex()
            .group(group_name.clone())
            .relative()
            .overflow_hidden()
            .w_full()
            .items_center()
            .font_buffer(cx)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(header_text),
            )
            .child(
                GradientFade::new(base_bg, base_bg, base_bg)
                    .width(grad_width)
                    .width_hovered(grad_width_hovered)
                    .right(px(0.))
                    .gradient_stop(0.8)
                    .group_name(group_name.clone()),
            )
            .child(
                h_flex()
                    .absolute()
                    .right_0()
                    .top_0()
                    .h_full()
                    .items_center()
                    .gap_1()
                    .when(!always_show_buttons, |this| {
                        this.visible_on_hover(group_name)
                    })
                    .child(self.create_filter_button(cx, col_idx))
                    .child(self.create_sort_button(cx, col_idx)),
            )
            .into_any_element()
    }

    fn create_sort_button(
        &self,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: AnyColumn,
    ) -> Button {
        Button::new(
            ElementId::NamedInteger("sort-button".into(), col_idx.get() as u64),
            match self.engine.applied_sorting {
                Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                    SortDirection::Asc => "↓",
                    SortDirection::Desc => "↑",
                },
                _ => "↕",
            },
        )
        .size(ButtonSize::Compact)
        .style(
            if self
                .engine
                .applied_sorting
                .is_some_and(|o| o.col_idx == col_idx)
            {
                ButtonStyle::Filled
            } else {
                ButtonStyle::Subtle
            },
        )
        .tooltip(Tooltip::text(match self.engine.applied_sorting {
            Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                SortDirection::Asc => "Sorted A-Z. Click to sort Z-A",
                SortDirection::Desc => "Sorted Z-A. Click to disable sorting",
            },
            _ => "Not sorted. Click to sort A-Z",
        }))
        .on_click(cx.listener(move |this, _event, _window, cx| {
            let new_sorting = match this.engine.applied_sorting {
                Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                    SortDirection::Asc => Some(AppliedSorting {
                        col_idx,
                        direction: SortDirection::Desc,
                    }),
                    SortDirection::Desc => None,
                },
                _ => Some(AppliedSorting {
                    col_idx,
                    direction: SortDirection::Asc,
                }),
            };
            this.engine.applied_sorting = new_sorting;
            this.apply_sort(cx);
            cx.notify();
        }))
    }

    fn create_filter_button(
        &self,
        cx: &mut Context<'_, CsvPreviewView>,
        col: AnyColumn,
    ) -> PopoverMenu<Picker<ColumnFilterDelegate>> {
        let has_active_filters = self.engine.has_active_filters(col);
        let sort_order = self.settings.filter_sort_order;

        PopoverMenu::new(ElementId::NamedInteger(
            "filter-menu".into(),
            col.get() as u64,
        ))
        .trigger_with_tooltip(
            IconButton::new(
                ElementId::NamedInteger("filter-button".into(), col.get() as u64),
                IconName::Filter,
            )
            .icon_size(IconSize::Small)
            .style(if has_active_filters {
                ButtonStyle::Filled
            } else {
                ButtonStyle::Subtle
            })
            .toggle_state(has_active_filters),
            Tooltip::text(if has_active_filters {
                "Column has active filters. Click to manage"
            } else {
                "No filters applied. Click to add filters"
            }),
        )
        .menu({
            let view_entity = cx.entity();
            move |window, cx| {
                let view = view_entity.read(cx);
                let column_filters = match view.engine.get_filters_for_column(col) {
                    Ok(filters) => filters,
                    Err(err) => {
                        log::error!("Failed to get filters for column: {err}");
                        return None;
                    }
                };
                let view_entity = view_entity.clone();

                let picker = cx.new(|cx| {
                    let delegate =
                        ColumnFilterDelegate::new(col, view_entity, sort_order, column_filters, cx);
                    Picker::list(delegate, window, cx)
                        .popover()
                        .initial_width(rems(18.75))
                        .show_scrollbar(true)
                });
                picker.focus_handle(cx).focus(window, cx);
                Some(picker)
            }
        })
    }
}
