use crate::CsvPreviewView;

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
    /// Calculate the optimal width for the line number column based on the total number of rows.
    ///
    /// This ensures the column is wide enough to display the largest line number comfortably,
    /// but not wastefully wide for small files.
    pub(crate) fn calculate_line_number_column_width(&self) -> f32 {
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
        let min_width = 60.;
        (base_width + padding).max(min_width)
    }
}
