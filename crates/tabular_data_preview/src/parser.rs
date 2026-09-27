use indexmap::{IndexMap, IndexSet};

use anyhow::Context as _;
use serde_json::value::RawValue;

use crate::{
    TabularDataPreviewPane,
    types::TableLikeContent,
    types::{LineNumber, TableCell},
};
use editor::Editor;
use gpui::{App, AppContext, Context, Entity, Subscription, Task};
use std::path::Path;
use std::time::{Duration, Instant};
use text::BufferSnapshot;
use ui::{SharedString, table_row::TableRow};

pub(crate) const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);

pub(crate) struct EditorState {
    pub editor: Entity<Editor>,
    pub _subscription: Subscription,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TabularFormat {
    Csv,
    Tsv,
    Psv,
    Ssv,
    JsonLines,
}

const TABULAR_FORMATS: &[(&str, TabularFormat)] = &[
    ("csv", TabularFormat::Csv),
    ("tsv", TabularFormat::Tsv),
    ("psv", TabularFormat::Psv),
    ("ssv", TabularFormat::Ssv),
    ("jsonl", TabularFormat::JsonLines),
    ("ndjson", TabularFormat::JsonLines),
];

impl TabularFormat {
    pub(crate) fn from_editor(editor: &Entity<Editor>, cx: &App) -> Option<Self> {
        Self::from_extension(editor_file_extension(editor, cx)?)
    }

    pub(crate) fn from_extension(ext: &str) -> Option<Self> {
        TABULAR_FORMATS
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(ext))
            .map(|(_, format)| *format)
    }

    fn parse(self, buffer_snapshot: &BufferSnapshot) -> anyhow::Result<TableLikeContent> {
        let delimiter = match self {
            Self::Csv => ',',
            Self::Tsv => '\t',
            Self::Psv => '|',
            Self::Ssv => ';',
            Self::JsonLines => return from_json_lines(buffer_snapshot),
        };
        Ok(from_buffer_with_delimiter(buffer_snapshot, delimiter))
    }
}

pub(crate) fn editor_file_extension<'a>(editor: &Entity<Editor>, cx: &'a App) -> Option<&'a str> {
    let buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
    let file = buffer.read(cx).file()?;
    Path::new(file.file_name(cx)).extension()?.to_str()
}

impl TabularDataPreviewPane {
    pub(crate) fn parse_from_active_editor(
        &mut self,
        wait_for_debounce: bool,
        cx: &mut Context<Self>,
    ) {
        let editor = self.active_editor_state.editor.clone();
        self.is_parsing = true;
        self.parsing_task = Some(self.parse_in_background(wait_for_debounce, editor, cx));
    }

    fn parse_in_background(
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

            let (buffer_snapshot, format) = view.update(cx, |_, cx| {
                let buffer_ref = editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .map(|b| b.read(cx).text_snapshot());

                let extension = editor_file_extension(&editor, cx);

                let format = extension
                    .and_then(TabularFormat::from_extension)
                    .unwrap_or_else(|| {
                        log::warn!(
                            "unrecognized tabular data extension {extension:?}, defaulting to CSV"
                        );
                        TabularFormat::Csv
                    });

                (buffer_ref, format)
            })?;

            let Some(buffer_snapshot) = buffer_snapshot else {
                view.update(cx, |view, cx| {
                    view.is_parsing = false;
                    view.parse_error = Some("Preview requires a single file".into());
                    cx.notify();
                })?;
                return Ok(());
            };

            let instant = Instant::now();
            let parsed_contents = cx
                .background_spawn(async move { format.parse(&buffer_snapshot) })
                .await;
            let parse_duration = instant.elapsed();
            let parse_end_time: Instant = Instant::now();
            log::debug!("Parsed data in {}ms", parse_duration.as_millis());
            view.update(cx, move |view, cx| {
                view.performance_metrics
                    .timings
                    .insert("Parsing", (parse_duration, Instant::now()));

                view.last_parse_end_time = Some(parse_end_time);
                view.is_parsing = false;
                let parsed_contents = match parsed_contents {
                    Ok(contents) => contents,
                    Err(error) => {
                        view.parse_error = Some(format!("{error:#}").into());
                        view.filter_sort_task = None;
                        cx.notify();
                        return;
                    }
                };

                log::debug!("Parsed {} rows", parsed_contents.rows.len());
                view.parse_error = None;
                view.engine.set_contents(parsed_contents);
                view.list_state
                    .reset_with_uniform_height(0, view.row_height);
                view.sync_column_widths(cx);
                view.apply_filter_sort(cx);
                cx.notify();
            })
        })
    }
}

fn from_json_lines(buffer_snapshot: &BufferSnapshot) -> anyhow::Result<TableLikeContent> {
    let text = buffer_snapshot.text();
    let mut records = Vec::new();
    let mut columns = IndexSet::new();
    let mut line_numbers = Vec::new();

    for (line_index, line) in text.lines().enumerate() {
        // Ignore blank lines, but keep physical line numbers for the source gutter.
        if line.trim().is_empty() {
            continue;
        }
        let record: IndexMap<String, &RawValue> =
            serde_json::from_str(line).with_context(|| {
                format!(
                    "Cannot preview JSONL line {}: expected a JSON object",
                    line_index + 1
                )
            })?;
        columns.extend(record.keys().cloned());
        records.push(record);
        line_numbers.push(LineNumber::Line(line_index + 1));
    }

    let number_of_cols = columns.len();
    let headers = TableRow::from_vec(
        columns
            .iter()
            .map(|name| TableCell::Generated(name.clone().into()))
            .collect(),
        number_of_cols,
    );
    let rows = records
        .into_iter()
        .map(|record| {
            let cells = columns
                .iter()
                .map(|name| {
                    let Some(value) = record.get(name) else {
                        return Ok(TableCell::Virtual);
                    };
                    let raw = value.get();
                    // RawValue borrows the original text, so its span stays exact even
                    // for repeated values, escaped strings, and large numbers.
                    let start = (raw.as_ptr() as usize)
                        .checked_sub(text.as_ptr() as usize)
                        .context("JSON value is outside the source buffer")?;
                    Ok(TableCell::from_buffer_position(
                        compact_json(raw).into(),
                        start,
                        start + raw.len(),
                        buffer_snapshot,
                    ))
                })
                .collect::<anyhow::Result<Vec<_>>>()?;
            Ok(TableRow::from_vec(cells, number_of_cols))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(TableLikeContent {
        headers,
        rows,
        line_numbers,
        number_of_cols,
    })
}

fn compact_json(raw: &str) -> String {
    // Reserializing through Value can round numbers. Remove only insignificant
    // whitespace from the already validated JSON instead.
    let mut compact = String::with_capacity(raw.len());
    let mut in_string = false;
    let mut escaped = false;
    for character in raw.chars() {
        if in_string {
            compact.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
        } else if character == '"' {
            in_string = true;
            compact.push(character);
        } else if !matches!(character, ' ' | '\t' | '\r' | '\n') {
            compact.push(character);
        }
    }
    compact
}

pub fn from_buffer_with_delimiter(
    buffer_snapshot: &BufferSnapshot,
    delimiter: char,
) -> TableLikeContent {
    let text = buffer_snapshot.text();

    if text.trim().is_empty() {
        return TableLikeContent::default();
    }

    let (parsed_cells_with_positions, line_numbers) =
        parse_delimited_text_with_positions(&text, delimiter);
    if parsed_cells_with_positions.is_empty() {
        return TableLikeContent::default();
    }
    let raw_headers = parsed_cells_with_positions[0].clone();

    // Calculating the longest row, as the data might have fewer headers than max row width
    let Some(max_number_of_cols) = parsed_cells_with_positions.iter().map(|r| r.len()).max() else {
        return TableLikeContent::default();
    };

    // Convert to TableCell objects with buffer positions
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

/// Parse delimited text and track byte positions for each cell
fn parse_delimited_text_with_positions(
    text: &str,
    delimiter: char,
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
            c if c == delimiter && !in_quotes => {
                // Field separator
                let field_end_offset = current_offset;
                if current_field.is_empty() && !in_quotes {
                    field_start_offset = current_offset;
                }
                current_row.push((
                    current_field.clone().into(),
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
                        current_field.clone().into(),
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
                    // Handle Windows line endings (\r\n): account for \r byte, let \n be handled next
                    current_offset += char_byte_len;
                    continue;
                } else {
                    // Standalone \r
                    current_line += 1;
                    if !in_quotes {
                        // Row separator (only when not inside quotes)
                        let field_end_offset = current_offset;
                        current_row.push((
                            current_field.clone().into(),
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
            current_field.clone().into(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use text::{Buffer, BufferId, ReplicaId, ToOffset};

    fn snapshot(text: &str) -> anyhow::Result<BufferSnapshot> {
        let buffer_id = BufferId::new(1).context("invalid test buffer ID")?;
        Ok(Buffer::new(ReplicaId::LOCAL, buffer_id, text.to_owned())
            .snapshot()
            .clone())
    }

    fn values(row: &TableRow<TableCell>) -> Vec<Option<&str>> {
        row.as_slice()
            .iter()
            .map(|cell| cell.display_value().map(|value| value.as_str()))
            .collect()
    }

    #[test]
    fn test_json_lines_columns_and_values() -> anyhow::Result<()> {
        let input = concat!(
            "{\"b\":null,\"a\":\"null\",\"nested\":{ \"items\": [1, true] }}\n",
            "{\"b\":\"\",\"late\":123456789012345678901234567890}\n",
            "{}"
        );
        let parsed = TabularFormat::JsonLines.parse(&snapshot(input)?)?;
        assert_eq!(
            values(&parsed.headers),
            vec![Some("b"), Some("a"), Some("nested"), Some("late")]
        );
        assert_eq!(parsed.rows.len(), 3);
        let rows: Vec<_> = parsed.rows.iter().map(values).collect();
        assert_eq!(
            rows,
            vec![
                vec![
                    Some("null"),
                    Some(r#""null""#),
                    Some(r#"{"items":[1,true]}"#),
                    None
                ],
                vec![
                    Some(r#""""#),
                    None,
                    None,
                    Some("123456789012345678901234567890")
                ],
                vec![None, None, None, None],
            ]
        );
        Ok(())
    }

    #[test]
    fn test_json_lines_columns_follow_first_seen_key_order() -> anyhow::Result<()> {
        let input = concat!(
            "{}\n",
            "{\"id\":2,\"name\":\"Grace\",\"active\":true}\n",
            "{\"name\":\"Ada\",\"tags\":[],\"id\":1,\"details\":{}}\n",
            "{\"details\":{},\"active\":false,\"extra\":null}\n"
        );
        let parsed = from_json_lines(&snapshot(input)?)?;
        assert_eq!(
            values(&parsed.headers),
            vec![
                Some("id"),
                Some("name"),
                Some("active"),
                Some("tags"),
                Some("details"),
                Some("extra")
            ]
        );
        assert_eq!(
            values(parsed.rows.get(2).context("missing record")?),
            vec![
                Some("1"),
                Some(r#""Ada""#),
                None,
                Some("[]"),
                Some("{}"),
                None
            ]
        );
        Ok(())
    }

    #[test]
    fn test_json_lines_source_spans_and_line_numbers() -> anyhow::Result<()> {
        let input =
            "\r\n{\"é\": \"🦀\", \"same\": \"🦀\"}\r\n \t\r\n{\"é\": 1.234567890123456789e100}\n";
        let snapshot = snapshot(input)?;
        let parsed = from_json_lines(&snapshot)?;
        let normalized_input = snapshot.text();
        assert!(matches!(
            parsed.line_numbers.as_slice(),
            [LineNumber::Line(2), LineNumber::Line(4)]
        ));
        assert_eq!(values(&parsed.headers), vec![Some("é"), Some("same")]);
        let first_row = parsed.rows.first().context("missing first row")?;
        for (cell, expected_start) in first_row.as_slice().iter().zip([
            normalized_input
                .find("\"🦀\"")
                .context("missing first value")?,
            normalized_input
                .rfind("\"🦀\"")
                .context("missing second value")?,
        ]) {
            let TableCell::Real { position, .. } = cell else {
                anyhow::bail!("expected a source-backed cell");
            };
            assert_eq!(position.start.to_offset(&snapshot), expected_start);
            assert_eq!(
                position.end.to_offset(&snapshot),
                expected_start + "\"🦀\"".len()
            );
        }
        assert_eq!(
            values(parsed.rows.last().context("missing last row")?),
            vec![Some("1.234567890123456789e100"), None]
        );
        Ok(())
    }

    #[test]
    fn test_json_lines_compact_values_preserve_strings_and_numbers() -> anyhow::Result<()> {
        let input = r#"{"value": { "text": "a b\t\"c\\", "numbers": [ -0, 1e999, 123456789012345678901234567890 ] }}"#;
        let parsed = from_json_lines(&snapshot(input)?)?;
        assert_eq!(
            values(parsed.rows.first().context("missing row")?),
            vec![Some(
                r#"{"text":"a b\t\"c\\","numbers":[-0,1e999,123456789012345678901234567890]}"#
            )]
        );
        Ok(())
    }

    #[test]
    fn test_json_lines_empty_input_and_empty_objects() -> anyhow::Result<()> {
        for input in ["", "\n \t\r\n"] {
            let parsed = from_json_lines(&snapshot(input)?)?;
            assert!(parsed.rows.is_empty());
            assert_eq!(parsed.number_of_cols, 0);
        }
        let parsed = from_json_lines(&snapshot("{}\n{}\n")?)?;
        assert_eq!(parsed.number_of_cols, 0);
        assert_eq!(parsed.rows.len(), 2);
        assert!(parsed.rows.iter().all(|row| row.cols() == 0));
        Ok(())
    }

    #[test]
    fn test_json_lines_errors_identify_the_source_line() -> anyhow::Result<()> {
        for invalid in [
            "{",
            "{\"a\":}",
            "{} trailing",
            "{} {}",
            "[]",
            "null",
            "true",
            "42",
            "\"text\"",
        ] {
            let input = format!("{{}}\n\n{invalid}\n{{}}\n");
            let result = from_json_lines(&snapshot(&input)?);
            let Err(error) = result else {
                anyhow::bail!("accepted invalid record: {invalid}");
            };
            assert!(error.to_string().contains("line 3"), "{error:#}");
        }
        Ok(())
    }

    #[test]
    fn test_format_dispatch() -> anyhow::Result<()> {
        for extension in ["jsonl", "ndjson", "JSONL", "nDjSoN"] {
            let format = TabularFormat::from_extension(extension).context("format not detected")?;
            assert_eq!(format, TabularFormat::JsonLines);
            assert_eq!(
                format.parse(&snapshot("{\"name\":\"Ada\"}")?)?.rows.len(),
                1
            );
        }
        assert!(TabularFormat::from_extension("json").is_none());
        for (extension, delimiter) in [("csv", ','), ("tsv", '\t'), ("psv", '|'), ("ssv", ';')] {
            let input = format!("name{delimiter}age\nAda{delimiter}36");
            let format = TabularFormat::from_extension(extension).context("format not detected")?;
            let parsed = format.parse(&snapshot(&input)?)?;
            assert_eq!(values(&parsed.headers), vec![Some("name"), Some("age")]);
            assert_eq!(
                values(parsed.rows.first().context("missing row")?),
                vec![Some("Ada"), Some("36")]
            );
        }
        Ok(())
    }

    #[test]
    fn test_csv_parsing_basic() {
        let csv_data = "Name,Age,City\nJohn,30,New York\nJane,25,Los Angeles";
        let parsed = TableLikeContent::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.cols(), 3);
        assert_eq!(parsed.headers[0].display_value().unwrap().as_ref(), "Name");
        assert_eq!(parsed.headers[1].display_value().unwrap().as_ref(), "Age");
        assert_eq!(parsed.headers[2].display_value().unwrap().as_ref(), "City");

        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(parsed.rows[0][0].display_value().unwrap().as_ref(), "John");
        assert_eq!(parsed.rows[0][1].display_value().unwrap().as_ref(), "30");
        assert_eq!(
            parsed.rows[0][2].display_value().unwrap().as_ref(),
            "New York"
        );
    }

    #[test]
    fn test_csv_parsing_with_quotes() {
        let csv_data = r#"Name,Description
"John Doe","A person with ""special"" characters"
Jane,"Simple name""#;
        let parsed = TableLikeContent::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.cols(), 2);
        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(
            parsed.rows[0][1].display_value().unwrap().as_ref(),
            r#"A person with "special" characters"#
        );
    }

    #[test]
    fn test_csv_parsing_with_newlines_in_quotes() {
        let csv_data = "Name,Description,Status\n\"John\nDoe\",\"A person with\nmultiple lines\",Active\n\"Jane Smith\",\"Simple\",\"Also\nActive\"";
        let parsed = TableLikeContent::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.cols(), 3);
        assert_eq!(parsed.headers[0].display_value().unwrap().as_ref(), "Name");
        assert_eq!(
            parsed.headers[1].display_value().unwrap().as_ref(),
            "Description"
        );
        assert_eq!(
            parsed.headers[2].display_value().unwrap().as_ref(),
            "Status"
        );

        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(
            parsed.rows[0][0].display_value().unwrap().as_ref(),
            "John\nDoe"
        );
        assert_eq!(
            parsed.rows[0][1].display_value().unwrap().as_ref(),
            "A person with\nmultiple lines"
        );
        assert_eq!(
            parsed.rows[0][2].display_value().unwrap().as_ref(),
            "Active"
        );

        assert_eq!(
            parsed.rows[1][0].display_value().unwrap().as_ref(),
            "Jane Smith"
        );
        assert_eq!(
            parsed.rows[1][1].display_value().unwrap().as_ref(),
            "Simple"
        );
        assert_eq!(
            parsed.rows[1][2].display_value().unwrap().as_ref(),
            "Also\nActive"
        );

        // Check line numbers
        assert_eq!(parsed.line_numbers.len(), 2);
        match &parsed.line_numbers[0] {
            LineNumber::LineRange(start, end) => {
                assert_eq!(start, &2);
                assert_eq!(end, &4);
            }
            _ => panic!("Expected LineRange for multiline row"),
        }
        match &parsed.line_numbers[1] {
            LineNumber::LineRange(start, end) => {
                assert_eq!(start, &5);
                assert_eq!(end, &6);
            }
            _ => panic!("Expected LineRange for second multiline row"),
        }
    }

    #[test]
    fn test_empty_input() {
        let parsed = TableLikeContent::from_str("".to_string());
        assert_eq!(parsed.headers.cols(), 0);
        assert!(parsed.rows.is_empty());
    }

    #[test]
    fn test_tsv_parsing() {
        let tsv_data = "Name\tAge\tCity\nJohn\t30\tNew York\nJane\t25\tLos Angeles";
        let (parsed_cells, _) = parse_delimited_text_with_positions(tsv_data, '\t');

        assert_eq!(parsed_cells.len(), 3);
        assert_eq!(parsed_cells[0].len(), 3);
        assert_eq!(parsed_cells[0][0].0.as_ref(), "Name");
        assert_eq!(parsed_cells[0][1].0.as_ref(), "Age");
        assert_eq!(parsed_cells[0][2].0.as_ref(), "City");
        assert_eq!(parsed_cells[1][0].0.as_ref(), "John");
        assert_eq!(parsed_cells[1][1].0.as_ref(), "30");
    }

    #[test]
    fn test_psv_parsing() {
        let psv_data = "Name|Age|City\nJohn|30|New York\nJane|25|Los Angeles";
        let (parsed_cells, _) = parse_delimited_text_with_positions(psv_data, '|');

        assert_eq!(parsed_cells.len(), 3);
        assert_eq!(parsed_cells[0].len(), 3);
        assert_eq!(parsed_cells[0][0].0.as_ref(), "Name");
        assert_eq!(parsed_cells[0][1].0.as_ref(), "Age");
        assert_eq!(parsed_cells[0][2].0.as_ref(), "City");
        assert_eq!(parsed_cells[1][0].0.as_ref(), "John");
        assert_eq!(parsed_cells[1][1].0.as_ref(), "30");
    }

    #[test]
    fn test_ssv_parsing() {
        let ssv_data = "Name;Age;City\nJohn;30;New York\nJane;25;Los Angeles";
        let (parsed_cells, _) = parse_delimited_text_with_positions(ssv_data, ';');

        assert_eq!(parsed_cells.len(), 3);
        assert_eq!(parsed_cells[0].len(), 3);
        assert_eq!(parsed_cells[0][0].0.as_ref(), "Name");
        assert_eq!(parsed_cells[0][1].0.as_ref(), "Age");
        assert_eq!(parsed_cells[0][2].0.as_ref(), "City");
        assert_eq!(parsed_cells[1][0].0.as_ref(), "John");
        assert_eq!(parsed_cells[1][1].0.as_ref(), "30");
    }

    #[test]
    fn test_csv_parsing_quote_offset_handling() {
        let csv_data = r#"first,"se,cond",third"#;
        let (parsed_cells, _) = parse_delimited_text_with_positions(csv_data, ',');

        assert_eq!(parsed_cells.len(), 1); // One row
        assert_eq!(parsed_cells[0].len(), 3); // Three cells

        // first: 0..5 (no quotes)
        let (content1, range1) = &parsed_cells[0][0];
        assert_eq!(content1.as_ref(), "first");
        assert_eq!(*range1, 0..5);

        // "se,cond": 6..15 (includes quotes in range, content without quotes)
        let (content2, range2) = &parsed_cells[0][1];
        assert_eq!(content2.as_ref(), "se,cond");
        assert_eq!(*range2, 6..15);

        // third: 16..21 (no quotes)
        let (content3, range3) = &parsed_cells[0][2];
        assert_eq!(content3.as_ref(), "third");
        assert_eq!(*range3, 16..21);
    }

    #[test]
    fn test_csv_parsing_complex_quotes() {
        let csv_data = r#"id,"name with spaces","description, with commas",status
1,"John Doe","A person with ""quotes"" and, commas",active
2,"Jane Smith","Simple description",inactive"#;
        let (parsed_cells, _) = parse_delimited_text_with_positions(csv_data, ',');

        assert_eq!(parsed_cells.len(), 3); // header + 2 rows

        // Check header row
        let header_row = &parsed_cells[0];
        assert_eq!(header_row.len(), 4);

        // id: 0..2
        assert_eq!(header_row[0].0.as_ref(), "id");
        assert_eq!(header_row[0].1, 0..2);

        // "name with spaces": 3..21 (includes quotes)
        assert_eq!(header_row[1].0.as_ref(), "name with spaces");
        assert_eq!(header_row[1].1, 3..21);

        // "description, with commas": 22..48 (includes quotes)
        assert_eq!(header_row[2].0.as_ref(), "description, with commas");
        assert_eq!(header_row[2].1, 22..48);

        // status: 49..55
        assert_eq!(header_row[3].0.as_ref(), "status");
        assert_eq!(header_row[3].1, 49..55);

        // Check first data row
        let first_row = &parsed_cells[1];
        assert_eq!(first_row.len(), 4);

        // 1: 56..57
        assert_eq!(first_row[0].0.as_ref(), "1");
        assert_eq!(first_row[0].1, 56..57);

        // "John Doe": 58..68 (includes quotes)
        assert_eq!(first_row[1].0.as_ref(), "John Doe");
        assert_eq!(first_row[1].1, 58..68);

        // Content should be stripped of quotes but include escaped quotes
        assert_eq!(
            first_row[2].0.as_ref(),
            r#"A person with "quotes" and, commas"#
        );
        // The range should include the outer quotes: 69..107
        assert_eq!(first_row[2].1, 69..107);

        // active: 108..114
        assert_eq!(first_row[3].0.as_ref(), "active");
        assert_eq!(first_row[3].1, 108..114);
    }
}

impl TableLikeContent {
    #[cfg(test)]
    pub fn from_str(text: String) -> Self {
        use text::{Buffer, BufferId, ReplicaId};

        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(ReplicaId::LOCAL, buffer_id, text);
        let snapshot = buffer.snapshot();
        from_buffer_with_delimiter(&snapshot, ',')
    }
}
