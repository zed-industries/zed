use editor::Editor;
use gpui::{
    DismissEvent, ElementId, Entity, EventEmitter, FocusHandle, Focusable, ListAlignment,
    ListSizingBehavior, ListState, Subscription, list,
};
use ui::{
    Color, GradientFade, HighlightedLabel, Icon, IconButton, IconName, IconSize, Label, LabelSize,
    PopoverMenu, Tooltip, WithScrollbar, prelude::*,
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

struct CsvFilterMenu {
    col: AnyColumn,
    view: Entity<CsvPreviewView>,
    editor: Entity<Editor>,
    /// Entry order frozen at open time: applied-first then sort_order.
    /// Kept stable so selections don't jump around while the menu is open.
    stable_order: Vec<Option<SharedString>>,
    /// Filtered entries for the current search query, recomputed each render.
    /// Stored on self so the list processor closure can index into it.
    entries: Vec<(FilterEntry, bool, Vec<usize>)>,
    list_state: ListState,
    /// Entry count as of the last `list_state` reset, used to detect when the
    /// search query has changed the filtered set and the list needs resetting.
    list_entry_count: usize,
    _editor_subscription: Subscription,
}

impl CsvFilterMenu {
    fn new(
        col: AnyColumn,
        view: Entity<CsvPreviewView>,
        sort_order: FilterSortOrder,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let stable_order = {
            let column_filters = view
                .read(cx)
                .engine
                .get_filters_for_column(col)
                .unwrap_or_default();

            let mut available: Vec<(&FilterEntry, bool)> = column_filters
                .iter()
                .filter_map(|(entry, state)| match state {
                    FilterEntryState::Available { is_applied } => Some((entry, *is_applied)),
                    _ => None,
                })
                .collect();

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

            available
                .into_iter()
                .map(|(e, _)| e.content.clone())
                .collect::<Vec<_>>()
        };

        let unique_count = stable_order.len();
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(
                &format!("Search {unique_count} unique values…"),
                window,
                cx,
            );
            editor
        });
        let subscription = cx.observe(&editor, |_, _, cx| cx.notify());

        Self {
            col,
            view,
            editor,
            stable_order,
            entries: Vec::new(),
            list_state: ListState::new(0, ListAlignment::Top, px(1000.)),
            list_entry_count: 0,
            _editor_subscription: subscription,
        }
    }
}

impl EventEmitter<DismissEvent> for CsvFilterMenu {}

impl Focusable for CsvFilterMenu {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for CsvFilterMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let col = self.col;
        let query = self.editor.read(cx).text(cx).to_lowercase();

        let column_filters = self
            .view
            .read(cx)
            .engine
            .get_filters_for_column(col)
            .unwrap_or_default();

        // Build a lookup: content key → (FilterEntry, is_applied).
        // Used to resolve live applied state while respecting stable_order.
        let lookup: Vec<(Option<SharedString>, FilterEntry, bool)> = column_filters
            .iter()
            .filter_map(|(entry, state)| match state {
                FilterEntryState::Available { is_applied } => {
                    Some((entry.content.clone(), entry.clone(), *is_applied))
                }
                _ => None,
            })
            .collect();

        let selected_rows: usize = lookup
            .iter()
            .filter(|(_, _, is_applied)| *is_applied)
            .map(|(_, e, _)| e.occurred_times())
            .sum();
        let total_rows: usize = lookup.iter().map(|(_, e, _)| e.occurred_times()).sum();
        let has_active_filters = selected_rows > 0;

        // Follow stable_order; look up live applied state from lookup.
        // Stored on self so the uniform_list processor closure can index into it.
        self.entries = self
            .stable_order
            .iter()
            .filter_map(|key| {
                lookup
                    .iter()
                    .find(|(k, _, _)| k == key)
                    .map(|(_, entry, is_applied)| (entry.clone(), *is_applied))
            })
            .filter_map(|(entry, is_applied)| {
                let display_text = match &entry.content {
                    Some(s) => s.as_ref().to_owned(),
                    None => "<null>".to_owned(),
                };
                if query.is_empty() {
                    return Some((entry, is_applied, vec![]));
                }
                let display_text_lower = display_text.to_lowercase();
                if let Some(byte_start) = display_text_lower.find(query.as_str()) {
                    let char_start = display_text_lower[..byte_start].chars().count();
                    let positions = (char_start..char_start + query.chars().count()).collect();
                    Some((entry, is_applied, positions))
                } else {
                    None
                }
            })
            .collect();

        let entry_count = self.entries.len();
        // Entries can wrap to multiple lines, so `list`/`ListState` (variable item
        // height) is used instead of `uniform_list`. Reset only when the filtered
        // set actually changes size, so typing doesn't drop scroll position/cache
        // on every render.
        if entry_count != self.list_entry_count {
            self.list_state.reset(entry_count);
            self.list_entry_count = entry_count;
        }
        // Cap like the command palette picker: grow to fit content, up to a
        // fraction of the window, then scroll. Avoids hand-computing row height.
        let max_list_height = window.viewport_size().height * 0.6;
        let border_color = cx.theme().colors().border_variant;

        v_flex()
            .key_context("CsvFilterMenu")
            .bg(cx.theme().colors().elevated_surface_background)
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .overflow_hidden()
            .w(px(300.))
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| {
                cx.emit(DismissEvent);
            }))
            // Search editor
            .child(
                h_flex()
                    .h_8()
                    .px_2()
                    .gap_1()
                    .border_b_1()
                    .border_color(border_color)
                    .items_center()
                    .child(
                        Icon::new(IconName::MagnifyingGlass)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.editor.clone()),
            )
            // Entry list
            .child(
                v_flex()
                    .relative()
                    .w_full()
                    .flex_grow_1()
                    .min_h_0()
                    .max_h(max_list_height)
                    .overflow_hidden()
                    .child(
                        list(
                            self.list_state.clone(),
                            cx.processor(move |this, ix: usize, _window, cx| {
                                let hover_bg = cx.theme().colors().element_hover;
                                let (entry, is_applied, positions) = &this.entries[ix];
                                let entry_value = entry.content.clone();
                                let value_text: SharedString = match &entry.content {
                                    Some(s) => s.clone(),
                                    None => "<null>".into(),
                                };
                                let count_text =
                                    SharedString::from(entry.occurred_times().to_string());
                                let is_applied = *is_applied;
                                let positions = positions.clone();
                                let view = this.view.clone();

                                h_flex()
                                    .id(ElementId::NamedInteger(
                                        format!("csv-filter-entry-{}", col.get()).into(),
                                        ix as u64,
                                    ))
                                    .w_full()
                                    .px_2()
                                    .py_1()
                                    .gap_2()
                                    .items_center()
                                    .justify_between()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(hover_bg))
                                    // Left: check icon + value text
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .min_w_0()
                                            .flex_1()
                                            .child(
                                                h_flex()
                                                    .flex_none()
                                                    .when(!is_applied, |el| el.invisible())
                                                    .child(
                                                        Icon::new(IconName::Check)
                                                            .size(IconSize::Small)
                                                            .color(Color::Accent),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .id(ElementId::NamedInteger(
                                                        format!("csv-filter-value-{}", col.get())
                                                            .into(),
                                                        ix as u64,
                                                    ))
                                                    .min_w_0()
                                                    .overflow_hidden()
                                                    .tooltip(Tooltip::element({
                                                        let value_text = value_text.clone();
                                                        move |_window, cx| {
                                                            div()
                                                                .font_buffer(cx)
                                                                .child(value_text.clone())
                                                                .into_any_element()
                                                        }
                                                    }))
                                                    .child(
                                                        HighlightedLabel::new(
                                                            value_text, positions,
                                                        )
                                                        .size(LabelSize::Small)
                                                        .single_line()
                                                        .truncate(),
                                                    ),
                                            ),
                                    )
                                    // Right: occurrence count
                                    .child(
                                        Label::new(count_text)
                                            .size(LabelSize::Small)
                                            .mr_2()
                                            .color(Color::Muted),
                                    )
                                    .on_click(move |_, _window, cx| {
                                        view.update(cx, |this: &mut CsvPreviewView, cx| {
                                            this.toggle_filter(col, entry_value.clone(), cx);
                                        });
                                    })
                                    .into_any_element()
                            }),
                        )
                        .w_full()
                        .flex_grow_1()
                        .with_sizing_behavior(ListSizingBehavior::Infer),
                    )
                    .vertical_scrollbar_for(&self.list_state, window, cx),
            )
            // Footer: row count + clear all — shown as soon as any filter is selected
            .when(has_active_filters, |this| {
                this.child(
                    h_flex()
                        .px_2()
                        .py_1()
                        .border_t_1()
                        .border_color(border_color)
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
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.view.update(cx, |view: &mut CsvPreviewView, cx| {
                                        view.clear_filters(col, cx);
                                    });
                                    cx.emit(DismissEvent);
                                })),
                        ),
                )
            })
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
    ) -> PopoverMenu<CsvFilterMenu> {
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
                let menu = cx
                    .new(|cx| CsvFilterMenu::new(col, view_entity.clone(), sort_order, window, cx));
                menu.focus_handle(cx).focus(window, cx);
                Some(menu)
            }
        })
    }
}
