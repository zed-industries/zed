use gpui::ElementId;
use std::collections::HashMap;
use ui::{ContextMenu, PopoverMenu, Tooltip, prelude::*};

use crate::{
    CsvPreviewView,
    settings::FontType,
    table_data_engine::{
        filtering_by_column::FilterEntry,
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
            this.re_sort_indices();
            cx.notify();
        }));
        sort_btn
    }

    fn create_filter_button(
        &self,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: AnyColumn,
    ) -> PopoverMenu<ContextMenu> {
        let has_filters = self
            .engine
            .applied_filtering
            .get_column_filters(col_idx)
            .map_or(false, |filters| !filters.is_empty());

        let id = ElementId::NamedInteger("filter-menu".into(), col_idx.get() as u64);
        PopoverMenu::new(id)
            .trigger_with_tooltip(
                Button::new(
                    ElementId::NamedInteger("filter-button".into(), col_idx.get() as u64),
                    "⚏",
                )
                .size(ButtonSize::Compact)
                .style(if has_filters {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                }),
                Tooltip::text(if has_filters {
                    "Column has active filters. Click to manage filters"
                } else {
                    "No filters applied. Click to add filters"
                }),
            )
            .menu({
                let view_entity = cx.entity();
                move |window, cx| {
                    let view = view_entity.read(cx);
                    if let Some(available_filters) =
                        view.engine.get_available_filters_for_column(col_idx)
                    {
                        let available_filters = available_filters.clone();
                        let applied_filters = view
                            .engine
                            .applied_filtering
                            .get_column_filters(col_idx)
                            .cloned();

                        let filter_menu = Self::create_filter_menu(
                            window,
                            cx,
                            view_entity.clone(),
                            col_idx,
                            &available_filters,
                            &applied_filters,
                        );
                        Some(filter_menu)
                    } else {
                        None
                    }
                }
            })
    }

    fn create_filter_menu(
        window: &mut ui::Window,
        cx: &mut ui::App,
        view_entity: gpui::Entity<CsvPreviewView>,
        col_idx: AnyColumn,
        available_filters: &Vec<FilterEntry>,
        applied_filters: &Option<HashMap<u64, FilterEntry>>,
    ) -> gpui::Entity<ContextMenu> {
        // Sort filters by occurrence count (descending), then by content
        let mut sorted_filters = available_filters.clone();
        sorted_filters.sort_by(|a, b| {
            b.occured_times
                .cmp(&a.occured_times)
                .then_with(|| a.content.cmp(&b.content))
        });

        ContextMenu::build(window, cx, move |menu, _, _| {
            let mut menu = menu;

            for filter in sorted_filters.iter() {
                let is_applied = applied_filters
                    .as_ref()
                    .map_or(false, |filters| filters.contains_key(&filter.hash));

                menu = menu.toggleable_entry(
                    &format!("{} ({})", filter.content, filter.occured_times),
                    is_applied,
                    ui::IconPosition::Start,
                    None,
                    {
                        let view_entity = view_entity.clone();
                        let content = filter.content.clone();
                        move |_window, cx| {
                            view_entity.update(cx, |view, cx| {
                                view.engine.toggle_filter(col_idx, content.clone());
                                view.engine.calculate_d2d_mapping();
                                let filtered_row_count =
                                    view.engine.get_d2d_mapping().filtered_row_count();
                                view.list_state = gpui::ListState::new(
                                    filtered_row_count,
                                    gpui::ListAlignment::Top,
                                    ui::px(1.),
                                );
                                cx.notify();
                            });
                        }
                    },
                );
            }

            menu
        })
    }
}
