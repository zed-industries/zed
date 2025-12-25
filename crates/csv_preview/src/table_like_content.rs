use text::BufferSnapshot;
use ui::SharedString;

#[cfg(test)]
use text::{Buffer, BufferId, ReplicaId};

use crate::types::{DataCellId, DataRow, LineNumber, TableCell, TableRow};

/// Generic container struct of table-like data (CSV, TSV, etc)
#[derive(Clone)]
pub struct TableLikeContent {
    /// Number of data columns.
    /// Defines table width used to validate `TableRow` on creation
    pub number_of_cols: usize,
    pub headers: TableRow<TableCell>,
    pub rows: Vec<TableRow<TableCell>>,
    /// Follows the same indices as `rows`
    pub line_numbers: Vec<LineNumber>,
}

impl Default for TableLikeContent {
    fn default() -> Self {
        Self {
            number_of_cols: Default::default(),
            headers: TableRow::<TableCell>::empty(),
            rows: vec![],
            line_numbers: vec![],
        }
    }
}

impl TableLikeContent {
    pub fn get_cell(&self, id: &DataCellId) -> Option<&TableCell> {
        self.rows.get(*id.row)?.get(id.col)
    }
    pub fn from_buffer(buffer_snapshot: BufferSnapshot) -> Self {
        let text = buffer_snapshot.text();

        if text.trim().is_empty() {
            return Self::default();
        }

        let (parsed_cells_with_positions, line_numbers) = Self::parse_csv_with_positions(&text);
        println!("Parsed: {parsed_cells_with_positions:#?}");
        if parsed_cells_with_positions.is_empty() {
            return Self::default();
        }

        // Calculating the longest row, as CSV might have less headers than max row width
        let max_number_of_cols = parsed_cells_with_positions
            .iter()
            .map(|r| r.len())
            .max()
            .expect("Expected non-empty array to have max() value");

        // Convert to TableCell objects with buffer positions
        let raw_headers = parsed_cells_with_positions[0].clone();
        let headers = create_table_row(&buffer_snapshot, max_number_of_cols, raw_headers);

        let rows = parsed_cells_with_positions
            .into_iter()
            .skip(1)
            .map(|row| create_table_row(&buffer_snapshot, max_number_of_cols, row))
            .collect();

        let row_line_numbers = line_numbers.into_iter().skip(1).collect();

        Self {
            headers,
            rows,
            line_numbers: row_line_numbers,
            number_of_cols: max_number_of_cols,
        }
    }

    /// Parse CSV and track byte positions for each cell
    fn parse_csv_with_positions(
        text: &str,
    ) -> (
        Vec<Vec<(SharedString, std::ops::Range<usize>)>>,
        Vec<LineNumber>,
    ) {
        let mut rows = Vec::new();
        let mut line_numbers = Vec::new();
        let mut current_row: Vec<(SharedString, std::ops::Range<usize>)> = Vec::new();
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
                            // Include the opening quote in the range
                            field_start_offset = current_offset;
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
                        field_start_offset..field_end_offset,
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
                            field_start_offset..field_end_offset,
                        ));
                        current_field.clear();

                        // Only add non-empty rows
                        if !current_row.is_empty()
                            && !current_row.iter().all(|(field, _)| field.trim().is_empty())
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
                                field_start_offset..field_end_offset,
                            ));
                            current_field.clear();

                            // Only add non-empty rows
                            if !current_row.is_empty()
                                && !current_row.iter().all(|(field, _)| field.trim().is_empty())
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
                field_start_offset..field_end_offset,
            ));
        }
        if !current_row.is_empty() && !current_row.iter().all(|(field, _)| field.trim().is_empty())
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

    #[cfg(test)]
    pub fn from_str(text: String) -> Self {
        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(ReplicaId::LOCAL, buffer_id, text);
        let snapshot = buffer.snapshot();
        Self::from_buffer(snapshot)
    }

    pub(crate) fn get_row(&self, data_row: DataRow) -> Option<&TableRow<TableCell>> {
        self.rows.get(*data_row)
    }
}

fn create_table_row(
    buffer_snapshot: &BufferSnapshot,
    max_number_of_cols: usize,
    row: Vec<(SharedString, std::ops::Range<usize>)>,
) -> TableRow<TableCell> {
    let mut raw_row = row
        .into_iter()
        .map(|(content, range)| {
            TableCell::from_buffer_position(content, range.start, range.end, &buffer_snapshot)
        })
        .collect::<Vec<_>>();

    let append_elements = max_number_of_cols - raw_row.len();
    if append_elements > 0 {
        for _ in 0..append_elements {
            raw_row.push(TableCell::Virtual);
        }
    }

    TableRow::from_vec(raw_row, max_number_of_cols)
}

// TODO: Fix
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_csv_parsing_basic() {
//         let csv_data = "Name,Age,City\nJohn,30,New York\nJane,25,Los Angeles";
//         let parsed = TableLikeContent::from_str(csv_data.to_string());

//         assert_eq!(parsed.headers.len(), 3);
//         assert_eq!(parsed.headers[0].display_value().as_ref(), "Name");
//         assert_eq!(parsed.headers[1].display_value().as_ref(), "Age");
//         assert_eq!(parsed.headers[2].display_value().as_ref(), "City");

//         assert_eq!(parsed.rows.len(), 2);
//         assert_eq!(parsed.rows[0][0].display_value().as_ref(), "John");
//         assert_eq!(parsed.rows[0][1].display_value().as_ref(), "30");
//         assert_eq!(parsed.rows[0][2].display_value().as_ref(), "New York");
//     }

//     #[test]
//     fn test_csv_parsing_with_quotes() {
//         let csv_data = r#"Name,Description
// "John Doe","A person with ""special"" characters"
// Jane,"Simple name""#;
//         let parsed = TableLikeContent::from_str(csv_data.to_string());

//         assert_eq!(parsed.headers.len(), 2);
//         assert_eq!(parsed.rows.len(), 2);
//         assert_eq!(
//             parsed.rows[0][1].display_value().as_ref(),
//             r#"A person with "special" characters"#
//         );
//     }

//     #[test]
//     fn test_csv_parsing_with_newlines_in_quotes() {
//         let csv_data = "Name,Description,Status\n\"John\nDoe\",\"A person with\nmultiple lines\",Active\n\"Jane Smith\",\"Simple\",\"Also\nActive\"";
//         let parsed = TableLikeContent::from_str(csv_data.to_string());

//         assert_eq!(parsed.headers.len(), 3);
//         assert_eq!(parsed.headers[0].display_value().as_ref(), "Name");
//         assert_eq!(parsed.headers[1].display_value().as_ref(), "Description");
//         assert_eq!(parsed.headers[2].display_value().as_ref(), "Status");

//         assert_eq!(parsed.rows.len(), 2);
//         assert_eq!(parsed.rows[0][0].display_value().as_ref(), "John\nDoe");
//         assert_eq!(
//             parsed.rows[0][1].display_value().as_ref(),
//             "A person with\nmultiple lines"
//         );
//         assert_eq!(parsed.rows[0][2].display_value().as_ref(), "Active");

//         assert_eq!(parsed.rows[1][0].display_value().as_ref(), "Jane Smith");
//         assert_eq!(parsed.rows[1][1].display_value().as_ref(), "Simple");
//         assert_eq!(parsed.rows[1][2].display_value().as_ref(), "Also\nActive");

//         // Check line numbers
//         assert_eq!(parsed.line_numbers.len(), 2);
//         match &parsed.line_numbers[0] {
//             LineNumber::LineRange(start, end) => {
//                 assert_eq!(*start, 2);
//                 assert_eq!(*end, 4);
//             }
//             _ => panic!("Expected LineRange for multiline row"),
//         }
//         match &parsed.line_numbers[1] {
//             LineNumber::LineRange(start, end) => {
//                 assert_eq!(*start, 5);
//                 assert_eq!(*end, 6);
//             }
//             _ => panic!("Expected LineRange for second multiline row"),
//         }
//     }

//     #[test]
//     fn test_empty_csv() {
//         let parsed = TableLikeContent::from_str("".to_string());
//         assert!(parsed.headers.is_empty());
//         assert!(parsed.rows.is_empty());
//     }

//     #[test]
//     fn test_csv_parsing_quote_offset_handling() {
//         let csv_data = r#"first,"se,cond",third"#;
//         let (parsed_cells, _) = TableLikeContent::parse_csv_with_positions(csv_data);

//         assert_eq!(parsed_cells.len(), 1); // One row
//         assert_eq!(parsed_cells[0].len(), 3); // Three cells

//         // first: 0..5 (no quotes)
//         let (content1, range1) = &parsed_cells[0][0];
//         assert_eq!(content1.as_ref(), "first");
//         assert_eq!(*range1, 0..5);

//         // "se,cond": 6..15 (includes quotes in range, content without quotes)
//         let (content2, range2) = &parsed_cells[0][1];
//         assert_eq!(content2.as_ref(), "se,cond");
//         assert_eq!(*range2, 6..15);

//         // third: 16..21 (no quotes)
//         let (content3, range3) = &parsed_cells[0][2];
//         assert_eq!(content3.as_ref(), "third");
//         assert_eq!(*range3, 16..21);
//     }

//     #[test]
//     fn test_csv_parsing_complex_quotes() {
//         let csv_data = r#"id,"name with spaces","description, with commas",status
// 1,"John Doe","A person with ""quotes"" and, commas",active
// 2,"Jane Smith","Simple description",inactive"#;
//         let (parsed_cells, _) = TableLikeContent::parse_csv_with_positions(csv_data);

//         assert_eq!(parsed_cells.len(), 3); // header + 2 rows

//         // Check header row
//         let header_row = &parsed_cells[0];
//         assert_eq!(header_row.len(), 4);

//         // id: 0..2
//         assert_eq!(header_row[0].0.as_ref(), "id");
//         assert_eq!(header_row[0].1, 0..2);

//         // "name with spaces": 3..21 (includes quotes)
//         assert_eq!(header_row[1].0.as_ref(), "name with spaces");
//         assert_eq!(header_row[1].1, 3..21);

//         // "description, with commas": 22..48 (includes quotes)
//         assert_eq!(header_row[2].0.as_ref(), "description, with commas");
//         assert_eq!(header_row[2].1, 22..48);

//         // status: 49..55
//         assert_eq!(header_row[3].0.as_ref(), "status");
//         assert_eq!(header_row[3].1, 49..55);

//         // Check first data row
//         let first_row = &parsed_cells[1];
//         assert_eq!(first_row.len(), 4);

//         // 1: 56..57
//         assert_eq!(first_row[0].0.as_ref(), "1");
//         assert_eq!(first_row[0].1, 56..57);

//         // "John Doe": 58..68 (includes quotes)
//         assert_eq!(first_row[1].0.as_ref(), "John Doe");
//         assert_eq!(first_row[1].1, 58..68);

//         // Content should be stripped of quotes but include escaped quotes
//         assert_eq!(
//             first_row[2].0.as_ref(),
//             r#"A person with "quotes" and, commas"#
//         );
//         // The range should include the outer quotes: 69..107
//         assert_eq!(first_row[2].1, 69..107);

//         // active: 108..114
//         assert_eq!(first_row[3].0.as_ref(), "active");
//         assert_eq!(first_row[3].1, 108..114);
//     }
// }
