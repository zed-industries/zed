use crate::data_table::Table;
use crate::data_table::TableColumnWidths;
use crate::data_table::TableResizeBehavior;
use crate::data_table::UncheckedTableRow;
use crate::types::TableCell;
use gpui::{AnyElement, Entity};
use std::ops::Range;
use ui::{DefiniteLength, div, prelude::*};

use crate::{
    CsvPreviewView,
    settings::RowRenderMechanism,
    types::{AnyColumn, DisplayCellId, DisplayRow},
};

impl CsvPreviewView {
    /// Creates a new table.
    /// Column number is derived from the `TableColumnWidths` entity.
    pub(crate) fn create_table(
        &self,
        current_widths: &Entity<TableColumnWidths>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let cols = current_widths.read(cx).cols();
        let remaining_col_number = cols - 1;
        let fraction = if remaining_col_number > 0 {
            1. / remaining_col_number as f32
        } else {
            1. // only column with line numbers is present. Put 100%, but it will be overwritten anyways :D
        };
        let mut widths = vec![DefiniteLength::Fraction(fraction); cols];
        let line_number_width = self.calculate_row_identifier_column_width();
        widths[0] = DefiniteLength::Absolute(AbsoluteLength::Pixels(line_number_width.into()));

        let mut resize_behaviors = vec![TableResizeBehavior::Resizable; cols];
        resize_behaviors[0] = TableResizeBehavior::None;

        self.create_table_inner(
            self.engine.contents.rows.len(),
            widths,
            resize_behaviors,
            current_widths,
            cx,
        )
    }

    fn create_table_inner(
        &self,
        row_count: usize,
        widths: UncheckedTableRow<DefiniteLength>,
        resize_behaviors: UncheckedTableRow<TableResizeBehavior>,
        current_widths: &Entity<TableColumnWidths>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let cols = widths.len();
        // Create headers array with interactive elements
        let mut headers = Vec::with_capacity(cols);

        headers.push(self.create_row_identitifier_header(cx));

        // Add the actual CSV headers with sort buttons
        for i in 0..(cols - 1) {
            let header_text = self
                .engine
                .contents
                .headers
                .get(AnyColumn(i))
                .and_then(|h| h.display_value().cloned())
                .unwrap_or_else(|| format!("Col {}", i + 1).into());

            headers.push(self.create_header_element_with_sort_button(
                header_text,
                cx,
                AnyColumn::from(i),
            ));
        }

        Table::new(cols)
            .interactable(&self.table_interaction_state)
            // .width(Length::Definite(DefiniteLength::Absolute( // Uncomment to apply width bigger than the parent
            //     AbsoluteLength::Pixels(3000u32.into()),
            // )))
            // .fixed_width(DefiniteLength::Fraction(1.))
            // .column_driven_width()
            .table_width(self.settings.table_width_mode.into())
            .striped()
            .column_widths(widths)
            .resizable_columns(resize_behaviors, current_widths, cx)
            .header(headers)
            .disable_base_style()
            .map(|table| {
                let row_identifier_text_color = cx.theme().colors().editor_line_number;
                let selected_bg = cx.theme().colors().element_selected;
                match self.settings.rendering_with {
                    RowRenderMechanism::VariableList => {
                        table.variable_row_height_list(row_count, self.list_state.clone(), {
                            cx.processor(move |this, display_row: usize, _window, cx| {
                                // Record this display index for performance metrics
                                this.performance_metrics.rendered_indices.push(display_row);

                                let display_row = DisplayRow(display_row);
                                Self::render_single_table_row(
                                    this,
                                    cols,
                                    display_row,
                                    row_identifier_text_color,
                                    selected_bg,
                                    cx,
                                )
                                .unwrap_or_else(|| panic!("Expected to render a table row"))
                            })
                        })
                    }
                    RowRenderMechanism::UniformList => {
                        table.uniform_list("csv-table", row_count, {
                            cx.processor(move |this, range: Range<usize>, _window, cx| {
                                // Record all display indices in the range for performance metrics
                                this.performance_metrics
                                    .rendered_indices
                                    .extend(range.clone());

                                range
                                    .filter_map(|display_index| {
                                        Self::render_single_table_row(
                                            this,
                                            cols,
                                            DisplayRow(display_index),
                                            row_identifier_text_color,
                                            selected_bg,
                                            cx,
                                        )
                                    })
                                    .collect()
                            })
                        })
                    }
                }
            })
            .into_any_element()
    }

    /// Render a single row for variable_row_height_list (supports variable heights)
    fn render_single_table_row(
        this: &CsvPreviewView,
        cols: usize,
        display_row: DisplayRow,
        row_identifier_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &Context<CsvPreviewView>,
    ) -> Option<UncheckedTableRow<AnyElement>> {
        // Get the actual row index from our sorted indices
        let data_row = this.engine.d2d_mapping().get_data_row(display_row)?;
        let row = this.engine.contents.get_row(data_row)?;

        let mut elements = Vec::with_capacity(cols);
        elements.push(this.create_row_identifier_cell(
            display_row,
            data_row,
            row_identifier_text_color,
            cx,
        )?);

        // Remaining columns: actual CSV data
        for col in (0..this.engine.contents.number_of_cols).map(AnyColumn) {
            let table_cell = row.expect_get(col);

            // TODO: Introduce `<null>` cell type
            let cell_content = table_cell.display_value().cloned().unwrap_or_default();

            let display_cell_id = DisplayCellId::new(display_row, col);

            // Check if this cell is selected using display coordinates
            let is_selected = this.engine.selection.is_cell_selected(
                display_row,
                col,
                &this.engine.d2d_mapping(),
            );

            // Check if this cell is focused using display coordinates
            let is_focused = this.engine.selection.is_cell_focused(display_row, col);

            // Check if this cell is the selection anchor using display coordinates
            let is_anchor = this.engine.selection.is_cell_anchor(display_row, col);

            let cell = if let Some(ctx) = this.cell_editor.as_ref()
                && ctx.cell_to_edit == display_cell_id
            {
                div()
                    .relative()
                    .child(div().absolute().child(ctx.editor.clone()))
            } else {
                div().size_full().whitespace_nowrap().text_ellipsis().child(
                    CsvPreviewView::create_selectable_cell(
                        display_cell_id,
                        cell_content,
                        cx.entity(),
                        selected_bg,
                        is_selected,
                        is_focused,
                        is_anchor,
                        this.settings.vertical_alignment,
                        this.settings.font_type,
                        cx,
                    ),
                )
            };

            elements.push(
                div()
                    .size_full()
                    .when(this.settings.show_debug_info, |parent| {
                        parent.child(div().text_color(row_identifier_text_color).child(
                            match table_cell {
                                TableCell::Real { position: pos, .. } => {
                                    let slv = pos.start.timestamp.value;
                                    let so = pos.start.offset;
                                    let elv = pos.end.timestamp.value;
                                    let eo = pos.end.offset;
                                    format!("Pos {so}(L{slv})-{eo}(L{elv})")
                                }
                                TableCell::Virtual => "Virtual cell".into(),
                            },
                        ))
                    })
                    .text_ui(cx)
                    .child(cell)
                    .into_any_element(),
            );
        }

        Some(elements)
    }
}
