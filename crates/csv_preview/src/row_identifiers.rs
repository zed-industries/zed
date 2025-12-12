use ui::{
    AnyElement, Button, ButtonCommon as _, ButtonSize, ButtonStyle, Clickable as _, Context,
    ElementId, FluentBuilder as _, IntoElement as _, ParentElement as _, SharedString, Styled as _,
    StyledTypography as _, Tooltip, div,
};

use crate::{CsvPreviewView, settings::FontType, settings::RowIdentifiers};

/// Line number information for CSV rows
#[derive(Debug, Clone, Copy)]
pub enum LineNumber {
    /// Single line row
    Line(usize),
    /// Multi-line row spanning from start to end line. Incluisive
    LineRange(usize, usize),
}

impl LineNumber {
    pub fn display_string(&self) -> String {
        match *self {
            LineNumber::Line(line) => line.to_string(),
            LineNumber::LineRange(start, end) => {
                if start + 1 == end {
                    format!("{start}\n{end}")
                } else {
                    format!("{start}\n...\n{end}")
                }
            }
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
            .contents
            .line_numbers
            .iter()
            .map(|ln| match ln {
                LineNumber::Line(n) => *n,
                LineNumber::LineRange(_, end) => *end,
            })
            .max()
            .unwrap_or(1);

        let digit_count = if max_line_number == 0 {
            1
        } else {
            (max_line_number as f32).log10().floor() as usize + 1
        };

        let char_width_px = 9.0; // TODO: get real width of the characters
        let base_width = (digit_count as f32) * char_width_px;
        let padding = 20.0;
        let min_width = 60.0;
        (base_width + padding).max(min_width)
    }

    /// Calculate width needed for sequential row numbers
    fn calculate_row_number_width(&self) -> f32 {
        let max_row_number = self.contents.rows.len();

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
        display_index: usize,
        row_identifier_text_color: gpui::Hsla,
        cx: &Context<'_, CsvPreviewView>,
        row_index: usize,
    ) -> Option<AnyElement> {
        let row_identifier: SharedString = match self.settings.numbering_type {
            RowIdentifiers::SrcLines => self
                .contents
                .line_numbers
                .get(row_index)?
                .display_string()
                .into(),
            RowIdentifiers::RowNum => (display_index + 1).to_string().into(),
        };
        let value = div()
            .flex()
            .child(row_identifier)
            .text_color(row_identifier_text_color)
            .h_full()
            // Row identifiers are always centered
            .items_center()
            .map(|div| match self.settings.font_type {
                FontType::Ui => div.font_ui(cx),
                FontType::Monospace => div.font_buffer(cx),
            })
            .into_any_element();
        Some(value)
    }
}
