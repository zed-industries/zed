use text::BufferSnapshot;
use ui::SharedString;

use crate::{row_identifiers::LineNumber, table_cell::TableCell};

/// Generic container struct of table-like data (CSV, TSV, etc)
#[derive(Default, Clone)]
pub struct TableLikeContent {
    pub headers: Vec<TableCell>,
    pub rows: Vec<Vec<TableCell>>,
    /// The source buffer snapshot for anchor resolution
    pub buffer_snapshot: Option<BufferSnapshot>,
    /// Follows the same indices as `rows`
    pub line_numbers: Vec<LineNumber>,
}

impl TableLikeContent {
    pub fn from_buffer(buffer_snapshot: BufferSnapshot) -> Self {
        let text = buffer_snapshot.text();

        if text.trim().is_empty() {
            return Self {
                headers: vec![],
                rows: vec![],
                buffer_snapshot: Some(buffer_snapshot),
                line_numbers: vec![],
            };
        }

        let (parsed_cells_with_positions, line_numbers) = Self::parse_csv_with_positions(&text);
        if parsed_cells_with_positions.is_empty() {
            return Self {
                headers: vec![],
                rows: vec![],
                buffer_snapshot: Some(buffer_snapshot),
                line_numbers: vec![],
            };
        }

        let buffer_id = buffer_snapshot.remote_id();

        // Convert to TableCell objects with buffer positions
        let headers = parsed_cells_with_positions[0]
            .iter()
            .map(|(content, start_offset, end_offset)| {
                TableCell::from_buffer_position(
                    content.clone(),
                    *start_offset,
                    *end_offset,
                    buffer_id,
                    &buffer_snapshot,
                )
            })
            .collect();

        let rows = parsed_cells_with_positions
            .into_iter()
            .skip(1)
            .map(|row| {
                row.into_iter()
                    .map(|(content, start_offset, end_offset)| {
                        TableCell::from_buffer_position(
                            content,
                            start_offset,
                            end_offset,
                            buffer_id,
                            &buffer_snapshot,
                        )
                    })
                    .collect()
            })
            .collect();

        let row_line_numbers = line_numbers.into_iter().skip(1).collect();

        Self {
            headers,
            rows,
            buffer_snapshot: Some(buffer_snapshot),
            line_numbers: row_line_numbers,
        }
    }

    /// Parse CSV and track byte positions for each cell
    fn parse_csv_with_positions(
        text: &str,
    ) -> (Vec<Vec<(SharedString, usize, usize)>>, Vec<LineNumber>) {
        let mut rows = Vec::new();
        let mut line_numbers = Vec::new();
        let mut current_row: Vec<(SharedString, usize, usize)> = Vec::new();
        let mut current_field = String::new();
        let mut field_start_offset = 0;
        let mut current_offset = 0;
        let mut in_quotes = false;
        let mut current_line = 1; // 1-based line numbering
        let mut row_start_line = 1;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            let char_byte_len = ch.len_utf8();

            match ch {
                '"' => {
                    if in_quotes {
                        if chars.peek() == Some(&'"') {
                            // Escaped quote
                            chars.next();
                            current_field.push('"');
                            current_offset += 1; // Skip the second quote
                        } else {
                            // End of quoted field
                            in_quotes = false;
                        }
                    } else {
                        // Start of quoted field
                        in_quotes = true;
                        if current_field.is_empty() {
                            field_start_offset = current_offset + char_byte_len;
                        }
                    }
                }
                ',' if !in_quotes => {
                    // Field separator
                    let field_end_offset = current_offset;
                    if current_field.is_empty() && !in_quotes {
                        field_start_offset = current_offset;
                    }
                    current_row.push((
                        current_field.trim().to_string().into(),
                        field_start_offset,
                        field_end_offset,
                    ));
                    current_field.clear();
                    field_start_offset = current_offset + char_byte_len;
                }
                '\n' => {
                    current_line += 1;
                    if !in_quotes {
                        // Row separator (only when not inside quotes)
                        let field_end_offset = current_offset;
                        if current_field.is_empty() && current_row.is_empty() {
                            field_start_offset = 0;
                        }
                        current_row.push((
                            current_field.trim().to_string().into(),
                            field_start_offset,
                            field_end_offset,
                        ));
                        current_field.clear();

                        // Only add non-empty rows
                        if !current_row.is_empty()
                            && !current_row
                                .iter()
                                .all(|(field, _, _)| field.trim().is_empty())
                        {
                            rows.push(current_row);
                            // Add line number info for this row
                            let line_info = if row_start_line == current_line - 1 {
                                LineNumber::Line(row_start_line)
                            } else {
                                LineNumber::LineRange(row_start_line, current_line - 1)
                            };
                            line_numbers.push(line_info);
                        }
                        current_row = Vec::new();
                        row_start_line = current_line;
                        field_start_offset = current_offset + char_byte_len;
                    } else {
                        // Newline inside quotes - preserve it
                        current_field.push(ch);
                    }
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        // Handle Windows line endings (\r\n) - skip the \r, let \n be handled above
                        // Don't increment current_offset yet, \n will handle it
                        continue;
                    } else {
                        // Standalone \r
                        current_line += 1;
                        if !in_quotes {
                            // Row separator (only when not inside quotes)
                            let field_end_offset = current_offset;
                            current_row.push((
                                current_field.trim().to_string().into(),
                                field_start_offset,
                                field_end_offset,
                            ));
                            current_field.clear();

                            // Only add non-empty rows
                            if !current_row.is_empty()
                                && !current_row
                                    .iter()
                                    .all(|(field, _, _)| field.trim().is_empty())
                            {
                                rows.push(current_row);
                                // Add line number info for this row
                                let line_info = if row_start_line == current_line - 1 {
                                    LineNumber::Line(row_start_line)
                                } else {
                                    LineNumber::LineRange(row_start_line, current_line - 1)
                                };
                                line_numbers.push(line_info);
                            }
                            current_row = Vec::new();
                            row_start_line = current_line;
                            field_start_offset = current_offset + char_byte_len;
                        } else {
                            // \r inside quotes - preserve it
                            current_field.push(ch);
                        }
                    }
                }
                _ => {
                    if current_field.is_empty() && !in_quotes {
                        field_start_offset = current_offset;
                    }
                    current_field.push(ch);
                }
            }

            current_offset += char_byte_len;
        }

        // Add the last field and row if not empty
        if !current_field.is_empty() || !current_row.is_empty() {
            let field_end_offset = current_offset;
            current_row.push((
                current_field.trim().to_string().into(),
                field_start_offset,
                field_end_offset,
            ));
        }
        if !current_row.is_empty()
            && !current_row
                .iter()
                .all(|(field, _, _)| field.trim().is_empty())
        {
            rows.push(current_row);
            // Add line number info for the last row
            let line_info = if row_start_line == current_line {
                LineNumber::Line(row_start_line)
            } else {
                LineNumber::LineRange(row_start_line, current_line)
            };
            line_numbers.push(line_info);
        }

        (rows, line_numbers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_parsing_basic() {
        let csv_data = "Name,Age,City\nJohn,30,New York\nJane,25,Los Angeles";
        let parsed = TableLikeContent::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.len(), 3);
        assert_eq!(parsed.headers[0].display_value().as_ref(), "Name");
        assert_eq!(parsed.headers[1].display_value().as_ref(), "Age");
        assert_eq!(parsed.headers[2].display_value().as_ref(), "City");

        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(parsed.rows[0][0].display_value().as_ref(), "John");
        assert_eq!(parsed.rows[0][1].display_value().as_ref(), "30");
        assert_eq!(parsed.rows[0][2].display_value().as_ref(), "New York");
    }

    #[test]
    fn test_csv_parsing_with_quotes() {
        let csv_data = r#"Name,Description
"John Doe","A person with ""special"" characters"
Jane,"Simple name""#;
        let parsed = TableLikeContent::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.len(), 2);
        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(
            parsed.rows[0][1].display_value().as_ref(),
            r#"A person with "special" characters"#
        );
    }

    #[test]
    fn test_csv_parsing_with_newlines_in_quotes() {
        let csv_data = "Name,Description,Status\n\"John\nDoe\",\"A person with\nmultiple lines\",Active\n\"Jane Smith\",\"Simple\",\"Also\nActive\"";
        let parsed = TableLikeContent::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.len(), 3);
        assert_eq!(parsed.headers[0].display_value().as_ref(), "Name");
        assert_eq!(parsed.headers[1].display_value().as_ref(), "Description");
        assert_eq!(parsed.headers[2].display_value().as_ref(), "Status");

        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(parsed.rows[0][0].display_value().as_ref(), "John\nDoe");
        assert_eq!(
            parsed.rows[0][1].display_value().as_ref(),
            "A person with\nmultiple lines"
        );
        assert_eq!(parsed.rows[0][2].display_value().as_ref(), "Active");

        assert_eq!(parsed.rows[1][0].display_value().as_ref(), "Jane Smith");
        assert_eq!(parsed.rows[1][1].display_value().as_ref(), "Simple");
        assert_eq!(parsed.rows[1][2].display_value().as_ref(), "Also\nActive");

        // Check line numbers
        assert_eq!(parsed.line_numbers.len(), 2);
        match &parsed.line_numbers[0] {
            LineNumber::LineRange(start, end) => {
                assert_eq!(*start, 2);
                assert_eq!(*end, 4);
            }
            _ => panic!("Expected LineRange for multiline row"),
        }
        match &parsed.line_numbers[1] {
            LineNumber::LineRange(start, end) => {
                assert_eq!(*start, 5);
                assert_eq!(*end, 6);
            }
            _ => panic!("Expected LineRange for second multiline row"),
        }
    }

    #[test]
    fn test_empty_csv() {
        let parsed = TableLikeContent::from_str("".to_string());
        assert!(parsed.headers.is_empty());
        assert!(parsed.rows.is_empty());
    }
}
