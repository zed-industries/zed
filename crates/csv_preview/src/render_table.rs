use gpui::{AnyElement, ElementId, Entity};
use std::ops::Range;
use ui::{
    Button, ButtonSize, ButtonStyle, DefiniteLength, SharedString, Table, TableColumnWidths,
    TableResizeBehavior, div, h_flex, prelude::*,
};

use crate::{
    CsvPreviewView, Ordering,
    data_ordering::OrderingDirection,
    settings::FontType,
    settings::RowRenderMechanism,
    types::{DisplayCellId, DisplayRow},
};

impl CsvPreviewView {
    /// Create header for data, which is orderable with text on the left and order button on the right
    fn create_header_element_for_orderables(
        &self,
        header_text: String,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: usize,
    ) -> AnyElement {
        // CSV data columns: text + sort button
        h_flex()
            .justify_between()
            .items_center()
            .w_full()
            .map(|div| match self.settings.font_type {
                FontType::Ui => div.font_ui(cx),
                FontType::Monospace => div.font_buffer(cx),
            })
            .child(div().child(header_text))
            .child(self.create_sort_button(cx, col_idx))
            .into_any_element()
    }

    fn create_sort_button(&self, cx: &mut Context<'_, CsvPreviewView>, col_idx: usize) -> Button {
        let sort_btn = Button::new(
            ElementId::NamedInteger("sort-button".into(), col_idx as u64),
            match self.ordering {
                Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                    OrderingDirection::Asc => "↑",
                    OrderingDirection::Desc => "↓",
                },
                _ => "↕", // Unsorted/available for sorting
            },
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
            this.update_ordered_indices();
            cx.notify();
        }));
        sort_btn
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
        let line_number_width = self.calculate_row_identifier_column_width();
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

        headers.push(self.create_row_identitifier_header(cx));

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

        Table::new()
            .interactable(&self.table_interaction_state)
            .striped()
            .column_widths(widths)
            .resizable_columns(resize_behaviors, current_widths, cx)
            .header(headers_array)
            .map(|table| {
                let row_identifier_text_color = cx.theme().colors().editor_line_number;
                let selected_bg = cx.theme().colors().element_selected;
                match self.settings.rendering_with {
                    RowRenderMechanism::VariableList => table.variable_list(row_count, {
                        cx.processor(move |this, display_index: usize, _window, cx| {
                            Self::render_table_row_for_variable_list::<COLS>(
                                this,
                                display_index,
                                row_identifier_text_color,
                                selected_bg,
                                cx,
                            )
                        })
                    }),
                    RowRenderMechanism::UniformList => {
                        table.uniform_list("csv-table", row_count, {
                            cx.processor(move |this, range: Range<usize>, _window, cx| {
                                Self::render_table_rows_for_uniform_list::<COLS>(
                                    this,
                                    range,
                                    row_identifier_text_color,
                                    selected_bg,
                                    cx,
                                )
                            })
                        })
                    }
                }
            })
            .into_any_element()
    }

    /// Render a single row for variable_list (supports variable heights)
    fn render_single_table_row<const COLS: usize>(
        this: &CsvPreviewView,
        display_index: usize,
        row_identifier_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &Context<CsvPreviewView>,
    ) -> Option<[AnyElement; COLS]> {
        let ordered_indices = this.get_ordered_indices();

        // Get the actual row index from our ordered indices
        let data_row = ordered_indices.get_data_row(DisplayRow::new(display_index))?;
        let row_index = data_row.get();
        let row = this.contents.rows.get(row_index)?;

        let mut elements = Vec::with_capacity(COLS);

        elements.push(this.create_row_identifier_cell(
            display_index,
            row_identifier_text_color,
            cx,
            row_index,
        )?);

        // Remaining columns: actual CSV data
        for col in 0..(COLS - 1) {
            let cell_content: SharedString = row.get(col).cloned().unwrap_or_else(|| "".into());

            // Check if this cell is selected using display coordinates
            let is_selected =
                this.selection
                    .is_cell_selected(display_index.into(), col, &ordered_indices);

            // Check if this cell is focused using display coordinates
            let is_focused =
                this.selection
                    .is_cell_focused(display_index.into(), col, &ordered_indices);

            // Check if this cell is the selection anchor using display coordinates
            let is_anchor =
                this.selection
                    .is_cell_anchor(display_index.into(), col, &ordered_indices);

            elements.push(CsvPreviewView::create_selectable_cell(
                DisplayCellId::new(display_index.into(), col),
                cell_content,
                cx.entity(),
                selected_bg,
                is_selected,
                is_focused,
                is_anchor,
                this.settings.vertical_alignment,
                this.settings.font_type,
                cx,
            ));
        }

        let elements_array: [AnyElement; COLS] = elements.try_into().ok()?;
        Some(elements_array)
    }

    fn render_table_row_for_variable_list<const COLS: usize>(
        this: &CsvPreviewView,
        display_index: usize,
        row_identifier_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &Context<CsvPreviewView>,
    ) -> [AnyElement; COLS] {
        Self::render_single_table_row(
            this,
            display_index,
            row_identifier_text_color,
            selected_bg,
            cx,
        )
        .unwrap_or_else(|| std::array::from_fn(|_| div().into_any_element()))
    }

    /// Render multiple rows for uniform_list (uniform heights only)
    fn render_table_rows_for_uniform_list<const COLS: usize>(
        this: &CsvPreviewView,
        display_indices: Range<usize>,
        row_identifier_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &Context<CsvPreviewView>,
    ) -> Vec<[AnyElement; COLS]> {
        display_indices
            .filter_map(|display_index| {
                Self::render_single_table_row(
                    this,
                    display_index,
                    row_identifier_text_color,
                    selected_bg,
                    cx,
                )
            })
            .collect()
    }
}
