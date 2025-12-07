use gpui::{AnyElement, ElementId, Entity, MouseButton};
use ui::{
    Button, ButtonSize, ButtonStyle, DefiniteLength, SharedString, Table, TableColumnWidths,
    TableResizeBehavior, div, h_flex, prelude::*,
};

use crate::{
    CsvPreviewView, Ordering,
    cell_selection::TableSelection,
    data_ordering::{OrderingDirection, generate_ordered_indecies},
};

impl Render for CsvPreviewView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .w_full()
            .h_full()
            .p_4()
            .bg(theme.colors().editor_background)
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
    }
}

impl CsvPreviewView {
    /// Create a header element with text on the left and clickable sort button on the right
    fn create_header_element(
        &self,
        col_idx: Option<usize>, // None for line number column
        header_text: String,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if let Some(col_idx) = col_idx {
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
        } else {
            // Line number column: just text, no sort button
            div().child(header_text).into_any_element()
        }
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

    /// Calculate the optimal width for the line number column based on the total number of rows.
    ///
    /// This ensures the column is wide enough to display the largest line number comfortably,
    /// but not wastefully wide for small files.
    fn calculate_line_number_column_width(&self) -> f32 {
        let max_line_number = self.contents.rows.len() + 1;

        // Count digits in the maximum line number
        let digit_count = if max_line_number == 0 {
            1
        } else {
            (max_line_number as f32).log10().floor() as usize + 1
        };

        let char_width_px = 9.0; // TODO: get real width of the characters

        let base_width = (digit_count as f32) * char_width_px;
        let padding = 20.0;
        let min_width = 50.;
        (base_width + padding).max(min_width)
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
        headers.push(self.create_header_element(None, "Line #".to_string(), cx));

        // Add the actual CSV headers with ordering buttons
        for i in 0..(COLS - 1) {
            let header_text = self
                .contents
                .headers
                .get(i)
                .map(|h| h.as_ref().to_string())
                .unwrap_or_else(|| format!("Col {}", i + 1));

            headers.push(self.create_header_element(Some(i), header_text, cx));
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
            .header(headers_array)
            .uniform_list("csv-table", row_count, {
                let line_num_text_color = cx.theme().colors().editor_line_number;
                let selected_bg = cx.theme().colors().element_selected;
                cx.processor(move |this, range: std::ops::Range<usize>, _window, cx| {
                    let ordered_indices = generate_ordered_indecies(this.ordering, &this.contents);

                    range
                        .filter_map(|display_index| {
                            // Get the actual row index from our ordered indices
                            let row_index = *ordered_indices.get(display_index)?;
                            let row = this.contents.rows.get(row_index)?;

                            let mut elements = Vec::with_capacity(COLS);

                            // First column: original line number (row_index + 2 because of 0-based indexing + header row)
                            let line_number: SharedString = (row_index + 2).to_string().into();
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
                                let ordered_indices =
                                    generate_ordered_indecies(this.ordering, &this.contents);
                                let display_to_data_converter =
                                    |dr: usize| ordered_indices.get(dr).copied();
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

                            let elements_array: [gpui::AnyElement; COLS] =
                                elements.try_into().ok()?;
                            Some(elements_array)
                        })
                        .collect()
                })
            });

        div()
            .w_full()
            .h_full()
            .child(table)
            // Workaround for selection to end_selection, when cursor is not over selectable cell
            .on_mouse_up(MouseButton::Left, {
                let view = cx.entity();
                move |_event, _window, cx| {
                    view.update(cx, |this, cx| {
                        this.selection.end_selection();
                        cx.notify();
                    });
                }
            })
            .into_any_element()
    }
}
