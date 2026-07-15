use gpui::ElementId;
use ui::{ContextMenu, PopoverMenu, Tooltip, prelude::*};

use crate::{
    CsvPreviewView,
    settings::FilterSortOrder,
    table_data_engine::{
        filtering_by_column::{FilterEntry, FilterEntryState},
        sorting_by_column::{AppliedSorting, SortDirection},
    },
    types::AnyColumn,
};

impl CsvPreviewView {
    /// Create header for data, which is orderable with text on the left and sort button on the right
    pub(crate) fn create_header_element_with_sort_button(
        &self,
        header_text: SharedString,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: AnyColumn,
    ) -> AnyElement {
        // CSV data columns: text + filter/sort buttons
        h_flex()
            .justify_between()
            .items_center()
            .w_full()
            .font_buffer(cx)
            .child(div().child(header_text))
            .child(
                h_flex()
                    .gap_1()
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
        let sort_btn = Button::new(
            ElementId::NamedInteger("sort-button".into(), col_idx.get() as u64),
            match self.engine.applied_sorting {
                Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                    SortDirection::Asc => "↓",
                    SortDirection::Desc => "↑",
                },
                _ => "↕", // Unsorted/available for sorting
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
                Some(ordering) if ordering.col_idx == col_idx => {
                    // Same column clicked - cycle through states
                    match ordering.direction {
                        SortDirection::Asc => Some(AppliedSorting {
                            col_idx,
                            direction: SortDirection::Desc,
                        }),
                        SortDirection::Desc => None, // Clear sorting
                    }
                }
                _ => {
                    // Different column or no sorting - start with ascending
                    Some(AppliedSorting {
                        col_idx,
                        direction: SortDirection::Asc,
                    })
                }
            };

            this.engine.applied_sorting = new_sorting;
            this.apply_sort(cx);
            cx.notify();
        }));
        sort_btn
    }

    fn create_filter_button(
        &self,
        cx: &mut Context<'_, CsvPreviewView>,
        col: AnyColumn,
    ) -> PopoverMenu<ContextMenu> {
        let has_active_filters = self.engine.has_active_filters(col);

        PopoverMenu::new(ElementId::NamedInteger(
            "filter-menu".into(),
            col.get() as u64,
        ))
        .trigger_with_tooltip(
            Button::new(
                ElementId::NamedInteger("filter-button".into(), col.get() as u64),
                if has_active_filters { "⛊" } else { "⛉" },
            )
            .size(ButtonSize::Compact)
            .style(if has_active_filters {
                ButtonStyle::Filled
            } else {
                ButtonStyle::Subtle
            }),
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
                let filter_sort_order = view.settings.filter_sort_order;
                let filter_menu = Self::create_filter_menu(
                    window,
                    cx,
                    view_entity.clone(),
                    col,
                    &column_filters,
                    has_active_filters,
                    filter_sort_order,
                );
                Some(filter_menu)
            }
        })
    }

    fn create_filter_menu(
        window: &mut ui::Window,
        cx: &mut ui::App,
        view_entity: gpui::Entity<CsvPreviewView>,
        col: AnyColumn,
        column_filters: &[(FilterEntry, FilterEntryState)],
        has_active_filters: bool,
        sort_order: FilterSortOrder,
    ) -> gpui::Entity<ContextMenu> {
        let mut available: Vec<&FilterEntry> = column_filters
            .iter()
            .filter_map(|(entry, state)| {
                matches!(state, FilterEntryState::Available { .. }).then_some(entry)
            })
            .collect();

        match sort_order {
            FilterSortOrder::AlphaThenCount => available.sort_by(|a, b| {
                a.content
                    .cmp(&b.content)
                    .then_with(|| b.occurred_times().cmp(&a.occurred_times()))
            }),
            FilterSortOrder::CountThenAlpha => available.sort_by(|a, b| {
                b.occurred_times()
                    .cmp(&a.occurred_times())
                    .then_with(|| a.content.cmp(&b.content))
            }),
        }

        let unavailable: Vec<(&FilterEntry, AnyColumn)> = column_filters
            .iter()
            .filter_map(|(entry, state)| {
                if let FilterEntryState::Unavailable { blocked_by } = state {
                    Some((entry, *blocked_by))
                } else {
                    None
                }
            })
            .collect();

        // Pre-build applied-state lookup before moving into the closure
        let applied_states: Vec<(FilterEntry, bool)> = column_filters
            .iter()
            .filter_map(|(entry, state)| {
                if let FilterEntryState::Available { is_applied } = state {
                    Some((entry.clone(), *is_applied))
                } else {
                    None
                }
            })
            .collect();

        let available_cloned: Vec<FilterEntry> = available.iter().map(|e| (*e).clone()).collect();
        let unavailable_cloned: Vec<(FilterEntry, AnyColumn)> = unavailable
            .into_iter()
            .map(|(e, col)| (e.clone(), col))
            .collect();

        ContextMenu::build(window, cx, move |menu, _, _| {
            let mut menu = menu;

            if has_active_filters {
                menu = menu
                    .toggleable_entry("Clear all", false, ui::IconPosition::Start, None, {
                        let view_entity = view_entity.clone();
                        move |_window, cx| {
                            view_entity.update(cx, |view, cx| {
                                view.clear_filters(col, cx);
                                cx.notify();
                            });
                        }
                    })
                    .separator();
            }

            for entry in &available_cloned {
                let is_applied = applied_states
                    .iter()
                    .find(|(e, _)| e.content == entry.content)
                    .map_or(false, |(_, applied)| *applied);

                let label: SharedString =
                    format_filter_label(entry.content.as_ref(), entry.occurred_times()).into();
                let entry_value = entry.content.clone();

                menu = menu.toggleable_entry(&label, is_applied, ui::IconPosition::Start, None, {
                    let view_entity = view_entity.clone();
                    move |_window, cx| {
                        view_entity.update(cx, |view, cx| {
                            view.toggle_filter(col, entry_value.clone(), cx);
                            cx.notify();
                        });
                    }
                });
            }

            if !unavailable_cloned.is_empty() {
                menu = menu.separator().header("Hidden by other filters");
                for (entry, _blocked_by) in &unavailable_cloned {
                    let label: SharedString =
                        format_filter_label(entry.content.as_ref(), entry.occurred_times()).into();
                    menu = menu.custom_entry(
                        {
                            let label = label.clone();
                            move |_window, cx| {
                                div()
                                    .px_2()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child(label.clone())
                                    .into_any_element()
                            }
                        },
                        |_, _| {},
                    );
                }
            }

            menu
        })
    }
}

fn format_filter_label(content: Option<&SharedString>, count: usize) -> String {
    match content {
        Some(s) => format!("{s} ({count})"),
        None => format!("<null> ({count})"),
    }
}
