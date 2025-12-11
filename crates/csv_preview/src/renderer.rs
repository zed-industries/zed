use gpui::{AnyElement, ElementId, Entity, MouseButton};
use std::ops::Range;
use ui::{
    Button, ButtonSize, ButtonStyle, ContextMenu, DefiniteLength, DropdownMenu, SharedString,
    Table, TableColumnWidths, TableResizeBehavior, Tooltip, div, h_flex, prelude::*,
};

use crate::{
    CsvPreviewView, Ordering, RowRenderMechanism,
    cell_selection::TableSelection,
    data_ordering::{OrderingDirection, generate_ordered_indices},
};

impl Render for CsvPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .w_full()
            .h_full()
            .p_4()
            .bg(theme.colors().editor_background)
            .child(self.render_settings_panel(window, cx))
            .child({
                if self.contents.headers.is_empty() {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_32()
                        .text_ui(cx)
                        .text_color(cx.theme().colors().text_muted)
                        .child("No CSV content to display")
                        .into_any_element()
                } else {
                    self.render_table_with_cols(cx)
                }
            })
            // Workaround to be able to `end_selection`, when cursor is not over selectable cell, but within the table
            .on_mouse_up(MouseButton::Left, {
                let view = cx.entity();
                move |_event, _window, cx| {
                    view.update(cx, |this, cx| {
                        this.selection.end_selection();
                        cx.notify();
                    });
                }
            })
    }
}

impl CsvPreviewView {
    /// Create header for data, which is orderable with text on the left and order button on the right
    fn create_header_element_for_orderables(
        &self,
        header_text: String,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: usize,
    ) -> AnyElement {
        // CSV data columns: text + sort button
        let sort_symbol = match self.ordering {
            Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                OrderingDirection::Asc => "↑",
                OrderingDirection::Desc => "↓",
            },
            _ => "↕", // Unsorted/available for sorting
        };

        h_flex()
            .justify_between()
            .items_center()
            .w_full()
            .child(
                // Header text on the left
                div().child(header_text),
            )
            .child(
                // Clickable sort button on the right
                Button::new(
                    ElementId::NamedInteger("sort-button".into(), col_idx as u64),
                    sort_symbol,
                )
                .size(ButtonSize::Compact)
                .style(if self.ordering.is_some_and(|o| o.col_idx == col_idx) {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    let new_ordering = match this.ordering {
                        Some(ordering) if ordering.col_idx == col_idx => {
                            // Same column clicked - cycle through states
                            match ordering.direction {
                                OrderingDirection::Asc => Some(Ordering {
                                    col_idx,
                                    direction: OrderingDirection::Desc,
                                }),
                                OrderingDirection::Desc => None, // Clear sorting
                            }
                        }
                        _ => {
                            // Different column or no sorting - start with ascending
                            Some(Ordering {
                                col_idx,
                                direction: OrderingDirection::Asc,
                            })
                        }
                    };

                    this.ordering = new_ordering;
                    cx.notify();
                })),
            )
            .into_any_element()
    }

    pub(crate) fn create_table<const COLS: usize>(
        &self,
        current_widths: &Entity<TableColumnWidths<COLS>>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        assert!(COLS > 0, "Expected to have at least 1 column");

        let remaining_col_number = COLS - 1;
        let fraction = if remaining_col_number > 0 {
            1. / remaining_col_number as f32
        } else {
            1. // only columns with line numbers is present. Put 100%, but it will be overwritten anyways :D
        };
        let mut widths = [DefiniteLength::Fraction(fraction); COLS];
        let line_number_width = self.calculate_line_number_column_width();
        widths[0] = DefiniteLength::Absolute(AbsoluteLength::Pixels(line_number_width.into()));

        let mut resize_behaviors = [TableResizeBehavior::Resizable; COLS];
        resize_behaviors[0] = TableResizeBehavior::None;

        self.create_table_inner(
            self.contents.rows.len(),
            widths,
            resize_behaviors,
            current_widths,
            cx,
        )
    }

    fn create_table_inner<const COLS: usize>(
        &self,
        row_count: usize,
        widths: [DefiniteLength; COLS],
        resize_behaviors: [TableResizeBehavior; COLS],
        current_widths: &Entity<TableColumnWidths<COLS>>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // Create headers array with interactive elements
        let mut headers = Vec::with_capacity(COLS);

        // First column: line numbers (not sortable)
        headers.push(div().child("Line #".to_string()).into_any_element());

        // Add the actual CSV headers with ordering buttons
        for i in 0..(COLS - 1) {
            let header_text = self
                .contents
                .headers
                .get(i)
                .map(|h| h.as_ref().to_string())
                .unwrap_or_else(|| format!("Col {}", i + 1));

            headers.push(self.create_header_element_for_orderables(header_text, cx, i));
        }

        // Manually construct array to avoid Debug trait requirement
        let headers_array: [AnyElement; COLS] = {
            assert_eq!(headers.len(), COLS, "Headers vector has wrong length");
            let mut iter = headers.into_iter();
            std::array::from_fn(|_| iter.next().unwrap())
        };

        let table = Table::new()
            .interactable(&self.table_interaction_state)
            .striped()
            .column_widths(widths)
            .resizable_columns(resize_behaviors, current_widths, cx)
            .header(headers_array);

        // Choose rendering method based on settings
        match self.settings.rendering_with {
            RowRenderMechanism::VariableList => table
                .variable_list(row_count, {
                    let line_num_text_color = cx.theme().colors().editor_line_number;
                    let selected_bg = cx.theme().colors().element_selected;
                    cx.processor(move |this, display_index: usize, _window, cx| {
                        Self::render_table_row_for_variable_list::<COLS>(
                            this,
                            display_index,
                            line_num_text_color,
                            selected_bg,
                            cx,
                        )
                    })
                })
                .into_any_element(),
            RowRenderMechanism::UniformList => table
                .uniform_list("csv-table", row_count, {
                    let line_num_text_color = cx.theme().colors().editor_line_number;
                    let selected_bg = cx.theme().colors().element_selected;
                    cx.processor(move |this, range: Range<usize>, _window, cx| {
                        Self::render_table_rows_for_uniform_list::<COLS>(
                            this,
                            range,
                            line_num_text_color,
                            selected_bg,
                            cx,
                        )
                    })
                })
                .into_any_element(),
        }
    }

    /// Render settings panel above the table
    fn render_settings_panel(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let current_mode_text = match self.settings.rendering_with {
            RowRenderMechanism::VariableList => "Variable Height",
            RowRenderMechanism::UniformList => "Uniform Height",
        };

        let view = cx.entity();
        let dropdown_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
            menu.entry("Variable Height", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.rendering_with = RowRenderMechanism::VariableList;
                        cx.notify();
                    });
                }
            })
            .entry("Uniform Height", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.rendering_with = RowRenderMechanism::UniformList;
                        cx.notify();
                    });
                }
            })
        });

        h_flex()
            .gap_2()
            .p_2()
            .bg(cx.theme().colors().surface_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().colors().text_muted)
                    .child("Rendering Mode:"),
            )
            .child(
                DropdownMenu::new(
                    ElementId::Name("rendering-mode-dropdown".into()),
                    current_mode_text,
                    dropdown_menu,
                )
                .trigger_size(ButtonSize::Compact)
                .trigger_tooltip(Tooltip::text("Choose between variable height (multiline support) or uniform height (better performance)"))
            )
            .into_any_element()
    }

    /// Render a single row for variable_list (supports variable heights)
    fn render_table_row_for_variable_list<const COLS: usize>(
        this: &CsvPreviewView,
        display_index: usize,
        line_num_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &mut Context<CsvPreviewView>,
    ) -> [AnyElement; COLS] {
        let ordered_indices = generate_ordered_indices(this.ordering, &this.contents);

        // Get the actual row index from our ordered indices
        let row_index = match ordered_indices.get(display_index) {
            Some(&idx) => idx,
            None => {
                // Return empty row array if index not found
                return std::array::from_fn(|_| div().into_any_element());
            }
        };

        let row = match this.contents.rows.get(row_index) {
            Some(r) => r,
            None => {
                // Return empty row array if row not found
                return std::array::from_fn(|_| div().into_any_element());
            }
        };

        let mut elements = Vec::with_capacity(COLS);

        // First column: original line number from parsed data
        let line_number: SharedString = this
            .contents
            .line_numbers
            .get(row_index)
            .map(|ln| ln.display_string().into())
            .unwrap_or_else(|| "".into());
        elements.push(
            div()
                .child(line_number)
                .text_color(line_num_text_color)
                .into_any_element(),
        );

        // Remaining columns: actual CSV data
        for col in 0..(COLS - 1) {
            let cell_content: SharedString = row.get(col).cloned().unwrap_or_else(|| "".into());

            // Check if this cell is selected using display coordinates
            let ordered_indices = generate_ordered_indices(this.ordering, &this.contents);
            let display_to_data_converter = |dr: usize| ordered_indices.get(dr).copied();
            let is_selected =
                this.selection
                    .is_cell_selected(display_index, col, display_to_data_converter);

            elements.push(TableSelection::create_selectable_cell(
                display_index,
                col,
                cell_content,
                cx.entity(),
                selected_bg,
                is_selected,
            ));
        }

        // Convert to fixed-size array, padding with empty divs if needed
        let mut elements_iter = elements.into_iter();
        std::array::from_fn(|_| {
            elements_iter
                .next()
                .unwrap_or_else(|| div().into_any_element())
        })
    }

    /// Render multiple rows for uniform_list (uniform heights only)
    fn render_table_rows_for_uniform_list<const COLS: usize>(
        this: &CsvPreviewView,
        range: Range<usize>,
        line_num_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &mut Context<CsvPreviewView>,
    ) -> Vec<[AnyElement; COLS]> {
        let ordered_indices = generate_ordered_indices(this.ordering, &this.contents);

        range
            .filter_map(|display_index| {
                // Get the actual row index from our ordered indices
                let row_index = *ordered_indices.get(display_index)?;
                let row = this.contents.rows.get(row_index)?;

                let mut elements = Vec::with_capacity(COLS);

                // First column: original line number from parsed data
                let line_number: SharedString = this
                    .contents
                    .line_numbers
                    .get(row_index)?
                    .display_string()
                    .into();
                elements.push(
                    div()
                        .child(line_number)
                        .text_color(line_num_text_color)
                        .into_any_element(),
                );

                // Remaining columns: actual CSV data
                for col in 0..(COLS - 1) {
                    let cell_content: SharedString =
                        row.get(col).cloned().unwrap_or_else(|| "".into());

                    // Check if this cell is selected using display coordinates
                    let ordered_indices = generate_ordered_indices(this.ordering, &this.contents);
                    let display_to_data_converter = |dr: usize| ordered_indices.get(dr).copied();
                    let is_selected = this.selection.is_cell_selected(
                        display_index,
                        col,
                        display_to_data_converter,
                    );

                    elements.push(TableSelection::create_selectable_cell(
                        display_index,
                        col,
                        cell_content,
                        cx.entity(),
                        selected_bg,
                        is_selected,
                    ));
                }

                let elements_array: [gpui::AnyElement; COLS] = elements.try_into().ok()?;
                Some(elements_array)
            })
            .collect()
    }
}
