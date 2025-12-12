use crate::{CsvPreviewView, NumberingType};

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

impl CsvPreviewView {
    /// Calculate the optimal width for the row identifier column (line numbers or row numbers).
    ///
    /// This ensures the column is wide enough to display the largest identifier comfortably,
    /// but not wastefully wide for small files.
    pub(crate) fn calculate_row_identifier_column_width(&self) -> f32 {
        match self.settings.numbering_type {
            NumberingType::Lines => self.calculate_line_number_width(),
            NumberingType::Rows => self.calculate_row_number_width(),
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
}
