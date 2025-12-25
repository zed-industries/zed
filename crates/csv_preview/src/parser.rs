use crate::{
    CsvPreviewView,
    types::TableLikeContent,
    types::{LineNumber, TableCell, TableRow},
};
use editor::{Editor, EditorEvent};
use gpui::{AppContext, Context, Entity, ListAlignment, ListState, Subscription, Task};
use std::time::{Duration, Instant};
use text::BufferSnapshot;
use ui::{SharedString, px};

pub(crate) const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);

pub(crate) struct EditorState {
    pub editor: Entity<Editor>,
    pub _subscription: Subscription,
}

impl CsvPreviewView {
    pub(crate) fn set_editor(&mut self, editor: Entity<Editor>, cx: &mut Context<Self>) {
        if let Some(active) = &self.active_editor_state
            && active.editor == editor
        {
            return;
        }

        let subscription = cx.subscribe(&editor, |this, _editor, event: &EditorEvent, cx| {
            match event {
                EditorEvent::Edited { .. }
                | EditorEvent::DirtyChanged
                | EditorEvent::ExcerptsEdited { .. } => {
                    println!("Event which triggered reparsing: {event:?}");
                    this.parse_csv_from_active_editor(true, cx);
                }
                EditorEvent::BufferEdited | EditorEvent::Reparsed(_) if this.cell_edited_flag => {
                    println!("CSV Cell edited. Event: {event:?}");
                    // Clearing
                    this.cell_edited_flag = false;
                    this.parse_csv_from_active_editor(true, cx);
                }
                _ => {
                    println!("Other event: {event:?}");
                }
            };
        });

        self.active_editor_state = Some(EditorState {
            editor,
            _subscription: subscription,
        });

        self.parse_csv_from_active_editor(false, cx);
    }

    pub(crate) fn parse_csv_from_active_editor(
        &mut self,
        wait_for_debounce: bool,
        cx: &mut Context<Self>,
    ) {
        self.parsing_task = Some(self.parse_csv_in_background(
            wait_for_debounce,
            self.editor_state().editor.clone(),
            cx,
        ));
    }

    fn parse_csv_in_background(
        &mut self,
        wait_for_debounce: bool,
        editor: Entity<Editor>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        cx.spawn(async move |view, cx| {
            if wait_for_debounce {
                // Smart debouncing: check if cooldown period has already passed
                let now = Instant::now();
                let should_wait = view.update(cx, |view, _| {
                    if let Some(last_end) = view.last_parse_end_time {
                        let cooldown_until = last_end + REPARSE_DEBOUNCE;
                        if now < cooldown_until {
                            Some(cooldown_until - now)
                        } else {
                            None // Cooldown already passed, parse immediately
                        }
                    } else {
                        None // First parse, no debounce
                    }
                })?;

                if let Some(wait_duration) = should_wait {
                    cx.background_executor().timer(wait_duration).await;
                }
            }

            let buffer_snapshot = view.update(cx, |_, cx| {
                editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .map(|b| b.read(cx).text_snapshot())
            })?;

            let Some(buffer_snapshot) = buffer_snapshot else {
                return Ok(());
            };

            let instant = Instant::now();
            let parsed_csv = cx
                .background_spawn(async move { from_buffer(buffer_snapshot) })
                .await;
            let parse_duration = instant.elapsed();
            let parse_end_time: Instant = Instant::now();
            log::debug!("Parsed CSV in {}ms", parse_duration.as_millis());
            view.update(cx, move |view, cx| {
                view.performance_metrics
                    .timings
                    .insert("Parsing", (parse_duration, Instant::now()));

                view.list_state = ListState::new(parsed_csv.rows.len(), ListAlignment::Top, px(1.));
                view.engine.contents = parsed_csv;
                view.last_parse_end_time = Some(parse_end_time);
                view.performance_metrics.record("Filters recalc", || {
                    view.engine.calculate_available_filters();
                });

                view.apply_filter_sort();
                cx.notify();
            })
        })
    }
}

///// CSV parsing /////
pub fn from_buffer(buffer_snapshot: BufferSnapshot) -> TableLikeContent {
    let text = buffer_snapshot.text();

    if text.trim().is_empty() {
        return TableLikeContent::default();
    }

    let (parsed_cells_with_positions, line_numbers) = parse_csv_with_positions(&text);
    if parsed_cells_with_positions.is_empty() {
        return TableLikeContent::default();
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

    TableLikeContent {
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
    if !current_row.is_empty() && !current_row.iter().all(|(field, _)| field.trim().is_empty()) {
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

impl TableLikeContent {
    #[cfg(test)]
    pub fn from_str(text: String) -> Self {
        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(ReplicaId::LOCAL, buffer_id, text);
        let snapshot = buffer.snapshot();
        Self::from_buffer(snapshot)
    }
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
