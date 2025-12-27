use gpui::ElementId;
use ui::{ContextMenu, PopoverMenu, Tooltip, prelude::*};

use crate::{
    CsvPreviewView,
    settings::FontType,
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
            .map(|div| match self.settings.font_type {
                FontType::Ui => div.font_ui(cx),
                FontType::Monospace => div.font_buffer(cx),
            })
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
            this.apply_sort();
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

        let id = ElementId::NamedInteger("filter-menu".into(), col.get() as u64);
        PopoverMenu::new(id)
            .trigger_with_tooltip(
                Button::new(
                    ElementId::NamedInteger("filter-button".into(), col.get() as u64),
                    "⚏",
                )
                .size(ButtonSize::Compact)
                .style(if has_active_filters {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                }),
                Tooltip::text(if has_active_filters {
                    "Column has active filters. Click to manage filters"
                } else {
                    "No filters applied. Click to add filters"
                }),
            )
            .menu({
                let view_entity = cx.entity();
                move |window, cx| {
                    let view = view_entity.read(cx);
                    let column_filters = view.engine.get_filters_for_column(col);
                    let filter_menu = Self::create_filter_menu(
                        window,
                        cx,
                        view_entity.clone(),
                        col,
                        &column_filters,
                        has_active_filters,
                    );
                    Some(filter_menu)
                }
            })
    }

    fn create_filter_menu(
        window: &mut ui::Window,
        cx: &mut ui::App,
        view_entity: gpui::Entity<CsvPreviewView>,
        col_idx: AnyColumn,
        column_filters: &[(FilterEntry, FilterEntryState)],
        has_active_filters: bool,
    ) -> gpui::Entity<ContextMenu> {
        ContextMenu::build(window, cx, move |menu, _, _| {
            let mut menu = menu;
            if has_active_filters {
                menu = menu
                    .toggleable_entry("Clear all", false, ui::IconPosition::Start, None, {
                        let view_entity = view_entity.clone();
                        move |_window, cx| {
                            view_entity.update(cx, |view, cx| {
                                view.clear_filters(col_idx);
                                cx.notify();
                            });
                        }
                    })
                    .separator();
            }

            for (filter, state) in column_filters.iter() {
                let state = *state;
                let is_applied = match state {
                    FilterEntryState::Available { is_applied } => is_applied,
                    FilterEntryState::Unavailable { .. } => false, // TODO: Instead of false, make the toggleable_entry non-interactive
                };
                let content = filter
                    .content
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("<null>");
                let text = match state {
                    FilterEntryState::Available { .. } => {
                        format!("{} ({})", content, filter.occured_times())
                    }
                    FilterEntryState::Unavailable { blocked_by } => format!(
                        "X({}) {} ({})",
                        *blocked_by,
                        content,
                        filter.occured_times(),
                    ),
                };
                // TODO: Use more customizeable entries
                menu = menu.toggleable_entry(&text, is_applied, ui::IconPosition::Start, None, {
                    let view_entity = view_entity.clone();
                    let content_hash = filter.hash;
                    move |_window, cx| {
                        view_entity.update(cx, |view, cx| {
                            if matches!(state, FilterEntryState::Available { .. }) {
                                view.toggle_filter(col_idx, content_hash);
                                cx.notify();
                            }
                        });
                    }
                });
            }

            menu
        })
    }
}
