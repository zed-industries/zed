use ui::{
    ActiveTheme as _, AnyElement, Button, ButtonCommon as _, ButtonSize, ButtonStyle,
    Clickable as _, Context, ElementId, FluentBuilder as _, IntoElement as _, ParentElement as _,
    SharedString, Styled as _, StyledTypography as _, Tooltip, div,
};

use crate::{
    CsvPreviewView,
    settings::{FontType, RowIdentifiers},
    types::{DataRow, DisplayRow},
};

/// Line number information for CSV rows
#[derive(Debug, Clone, Copy)]
pub enum LineNumber {
    /// Single line row
    Line(usize),
    /// Multi-line row spanning from start to end line. Incluisive
    LineRange(usize, usize),
}

pub enum RowIdentDisplayMode {
    /// E.g
    /// ```
    /// 1
    /// ...
    /// 5
    /// ```
    Vertical,
    /// E.g.
    /// ```
    /// 1-5
    /// ```
    Horizontal,
}

impl LineNumber {
    pub fn display_string(&self, mode: RowIdentDisplayMode) -> String {
        match *self {
            LineNumber::Line(line) => line.to_string(),
            LineNumber::LineRange(start, end) => match mode {
                RowIdentDisplayMode::Vertical => {
                    if start + 1 == end {
                        format!("{start}\n{end}")
                    } else {
                        format!("{start}\n...\n{end}")
                    }
                }
                RowIdentDisplayMode::Horizontal => {
                    format!("{start}-{end}")
                }
            },
        }
    }
}

///// Row identifier related view operations /////
impl CsvPreviewView {
    /// Calculate the optimal width for the row identifier column (line numbers or row numbers).
    ///
    /// This ensures the column is wide enough to display the largest identifier comfortably,
    /// but not wastefully wide for small files.
    pub(crate) fn calculate_row_identifier_column_width(&self) -> f32 {
        match self.settings.numbering_type {
            RowIdentifiers::SrcLines => self.calculate_line_number_width(),
            RowIdentifiers::RowNum => self.calculate_row_number_width(),
        }
    }

    /// Calculate width needed for line numbers (can be multi-line)
    fn calculate_line_number_width(&self) -> f32 {
        // Find the maximum line number that could be displayed
        let max_line_number = self
            .engine
            .contents
            .line_numbers
            .iter()
            .map(|ln| match ln {
                LineNumber::Line(n) => *n,
                LineNumber::LineRange(_, end) => *end,
            })
            .max()
            .unwrap_or_default();

        let digit_count = if max_line_number == 0 {
            1
        } else {
            (max_line_number as f32).log10().floor() as usize + 1
        };

        // if !self.settings.multiline_cells_enabled {
        //     // Uses horizontal line numbers layout like `123-456`. Needs twice the size
        //     digit_count *= 2;
        // }

        let char_width_px = 9.0; // TODO: get real width of the characters
        let base_width = (digit_count as f32) * char_width_px;
        let padding = 20.0;
        let min_width = 60.0;
        (base_width + padding).max(min_width)
    }

    /// Calculate width needed for sequential row numbers
    fn calculate_row_number_width(&self) -> f32 {
        let max_row_number = self.engine.contents.rows.len();

        let digit_count = if max_row_number == 0 {
            1
        } else {
            (max_row_number as f32).log10().floor() as usize + 1
        };

        let char_width_px = 9.0; // TODO: get real width of the characters
        let base_width = (digit_count as f32) * char_width_px;
        let padding = 20.0;
        let min_width = 60.0;
        (base_width + padding).max(min_width)
    }

    pub(crate) fn create_row_identitifier_header(
        &self,
        cx: &mut Context<'_, CsvPreviewView>,
    ) -> AnyElement {
        // First column: row identifier (clickable to toggle between Lines and Rows)
        let row_identifier_text = match self.settings.numbering_type {
            RowIdentifiers::SrcLines => "Lines",
            RowIdentifiers::RowNum => "Rows",
        };

        let view = cx.entity();
        let value = div()
            .map(|div| match self.settings.font_type {
                FontType::Ui => div.font_ui(cx),
                FontType::Monospace => div.font_buffer(cx),
            })
            .child(
                Button::new(
                    ElementId::Name("row-identifier-toggle".into()),
                    row_identifier_text,
                )
                .style(ButtonStyle::Subtle)
                .size(ButtonSize::Compact)
                .tooltip(Tooltip::text(
                    "Toggle between: file line numbers or sequential row numbers",
                ))
                .on_click(move |_event, _window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.numbering_type = match this.settings.numbering_type {
                            RowIdentifiers::SrcLines => RowIdentifiers::RowNum,
                            RowIdentifiers::RowNum => RowIdentifiers::SrcLines,
                        };
                        cx.notify();
                    });
                }),
            )
            .into_any_element();
        value
    }

    pub(crate) fn create_row_identifier_cell(
        &self,
        display_row: DisplayRow,
        data_row: DataRow,
        row_identifier_text_color: gpui::Hsla,
        cx: &Context<'_, CsvPreviewView>,
    ) -> Option<AnyElement> {
        let row_identifier: SharedString = match self.settings.numbering_type {
            RowIdentifiers::SrcLines => self
                .engine
                .contents
                .line_numbers
                .get(*data_row)?
                .display_string(if self.settings.multiline_cells_enabled {
                    RowIdentDisplayMode::Vertical
                } else {
                    RowIdentDisplayMode::Horizontal
                })
                .into(),
            RowIdentifiers::RowNum => (*display_row + 1).to_string().into(),
        };
        // Check if this row has focus to highlight the line number
        let is_focused = self.selection.is_row_focused(display_row);

        // Use normal text color for focused row, muted color otherwise
        let text_color = if is_focused {
            cx.theme().colors().text
        } else {
            row_identifier_text_color
        };

        let value = div()
            .flex()
            .px_1()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .text_color(text_color)
            .h_full()
            .text_ui(cx)
            // Row identifiers are always centered
            .items_center()
            .justify_end()
            .map(|div| match self.settings.font_type {
                FontType::Ui => div.font_ui(cx),
                FontType::Monospace => div.font_buffer(cx),
            })
            .child(row_identifier)
            .into_any_element();
        Some(value)
    }
}
