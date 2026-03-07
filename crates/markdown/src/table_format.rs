use unicode_width::UnicodeWidthStr;

/// Alignment of a table column, derived from the delimiter row.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Alignment {
    /// Default left alignment (`---`).
    Left,
    /// Explicit left alignment (`:---`).
    ExplicitLeft,
    Center,
    Right,
}

/// Returns `true` if `line` looks like it could be part of a markdown pipe table.
pub fn looks_like_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') || trimmed.matches('|').count() >= 2
}

/// Format a markdown pipe table so that columns are aligned.
///
/// Returns `None` if the input does not look like a valid table (e.g. no delimiter row found,
/// or fewer than 2 rows).
pub fn format_markdown_table(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() < 2 {
        return None;
    }

    // Detect common leading indent
    let indent_len = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let indent = &lines[0][..indent_len];

    // Parse each line into cells
    let parsed: Vec<Vec<String>> = lines
        .iter()
        .map(|line| parse_row(&line[indent_len..]))
        .collect();

    // Find the delimiter row: every cell matches the pattern :?-+:?
    let delimiter_index = parsed.iter().position(|row| is_delimiter_row(row))?;

    // Extract alignments from the delimiter row
    let alignments: Vec<Alignment> = parsed[delimiter_index]
        .iter()
        .map(|cell| parse_alignment(cell))
        .collect();

    // Determine the number of columns (max across all rows)
    let column_count = parsed.iter().map(|row| row.len()).max().unwrap_or(0);
    if column_count == 0 {
        return None;
    }

    // Calculate column widths (minimum 3 to fit delimiter dashes)
    let mut widths = vec![3usize; column_count];
    for (i, row) in parsed.iter().enumerate() {
        if i == delimiter_index {
            continue;
        }
        for (j, cell) in row.iter().enumerate() {
            if j < column_count {
                widths[j] = widths[j].max(UnicodeWidthStr::width(cell.as_str()));
            }
        }
    }

    // Rebuild lines
    let mut result = Vec::with_capacity(lines.len());
    for (i, row) in parsed.iter().enumerate() {
        if i == delimiter_index {
            result.push(format_delimiter_row(&widths, &alignments, indent));
        } else {
            result.push(format_content_row(row, &widths, &alignments, indent));
        }
    }

    Some(result.join("\n"))
}

/// Parse a single table row into cells.
/// Strips leading/trailing `|`, splits on unescaped `|`, trims each cell.
fn parse_row(line: &str) -> Vec<String> {
    let trimmed = line.trim();

    // Strip leading and trailing pipes
    let inner = trimmed
        .strip_prefix('|')
        .unwrap_or(trimmed);
    let inner = inner
        .strip_suffix('|')
        .unwrap_or(inner);

    split_on_unescaped_pipe(inner)
        .iter()
        .map(|cell| cell.trim().to_string())
        .collect()
}

/// Split a string on unescaped `|` characters.
fn split_on_unescaped_pipe(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2; // skip escaped character
            continue;
        }
        if bytes[i] == b'|' {
            parts.push(&input[start..i]);
            start = i + 1;
        }
        i += 1;
    }
    parts.push(&input[start..]);
    parts
}

/// Returns `true` if every cell in the row matches the delimiter pattern `:?-+:?`.
fn is_delimiter_row(cells: &[String]) -> bool {
    if cells.is_empty() {
        return false;
    }
    cells.iter().all(|cell| {
        let trimmed = cell.trim();
        if trimmed.is_empty() {
            return false;
        }
        let inner = trimmed.strip_prefix(':').unwrap_or(trimmed);
        let inner = inner.strip_suffix(':').unwrap_or(inner);
        !inner.is_empty() && inner.chars().all(|c| c == '-')
    })
}

/// Parse the alignment from a delimiter cell like `:---`, `:---:`, `---:`.
fn parse_alignment(cell: &str) -> Alignment {
    let trimmed = cell.trim();
    let left = trimmed.starts_with(':');
    let right = trimmed.ends_with(':');
    match (left, right) {
        (true, true) => Alignment::Center,
        (false, true) => Alignment::Right,
        (true, false) => Alignment::ExplicitLeft,
        (false, false) => Alignment::Left,
    }
}

/// Format a content row with proper padding.
fn format_content_row(
    cells: &[String],
    widths: &[usize],
    alignments: &[Alignment],
    indent: &str,
) -> String {
    let mut parts = Vec::with_capacity(widths.len());
    for (j, width) in widths.iter().enumerate() {
        let cell = cells.get(j).map(|s| s.as_str()).unwrap_or("");
        let alignment = alignments.get(j).copied().unwrap_or(Alignment::Left);
        parts.push(pad_cell(cell, *width, alignment));
    }
    format!("{indent}| {} |", parts.join(" | "))
}

/// Format the delimiter row with dashes and alignment markers.
fn format_delimiter_row(widths: &[usize], alignments: &[Alignment], indent: &str) -> String {
    let mut parts = Vec::with_capacity(widths.len());
    for (j, width) in widths.iter().enumerate() {
        let alignment = alignments.get(j).copied().unwrap_or(Alignment::Left);
        let dashes = match alignment {
            Alignment::Left => format!("{:-<width$}", "", width = *width),
            Alignment::ExplicitLeft => {
                let inner_width = width.saturating_sub(1);
                format!(":{:-<width$}", "", width = inner_width)
            }
            Alignment::Center => {
                let inner_width = width.saturating_sub(2);
                format!(":{:-<width$}:", "", width = inner_width)
            }
            Alignment::Right => {
                let inner_width = width.saturating_sub(1);
                format!("{:-<width$}:", "", width = inner_width)
            }
        };
        parts.push(dashes);
    }
    format!("{indent}| {} |", parts.join(" | "))
}

/// Pad a cell's content according to its column width and alignment.
fn pad_cell(content: &str, width: usize, alignment: Alignment) -> String {
    let content_width = UnicodeWidthStr::width(content);
    if content_width >= width {
        return content.to_string();
    }
    let padding = width - content_width;
    match alignment {
        Alignment::Left | Alignment::ExplicitLeft => format!("{content}{}", " ".repeat(padding)),
        Alignment::Right => format!("{}{content}", " ".repeat(padding)),
        Alignment::Center => {
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            format!("{}{content}{}", " ".repeat(left_pad), " ".repeat(right_pad))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_alignment() {
        let input = "| Name | Age | City |\n| --- | --- | --- |\n| Alice | 30 | New York |\n| Bob | 25 | LA |";
        let expected = "| Name  | Age | City     |\n| ----- | --- | -------- |\n| Alice | 30  | New York |\n| Bob   | 25  | LA       |";
        assert_eq!(format_markdown_table(input).unwrap(), expected);
    }

    #[test]
    fn test_alignment_specifiers() {
        let input = "| Left | Center | Right |\n| :--- | :---: | ---: |\n| a | b | c |\n| longer | text | here |";
        let expected = "| Left   | Center | Right |\n| :----- | :----: | ----: |\n| a      |   b    |     c |\n| longer |  text  |  here |";
        assert_eq!(format_markdown_table(input).unwrap(), expected);
    }

    #[test]
    fn test_unicode_width() {
        let input = "| Name | Greeting |\n| --- | --- |\n| Alice | Hello |\n| 太郎 | こんにちは |";
        let result = format_markdown_table(input).unwrap();
        // "太郎" is width 4, "こんにちは" is width 10
        let expected = "| Name  | Greeting   |\n| ----- | ---------- |\n| Alice | Hello      |\n| 太郎  | こんにちは |";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_indented_table() {
        let input = "    | A | B |\n    | --- | --- |\n    | 1 | 2 |";
        let expected = "    | A   | B   |\n    | --- | --- |\n    | 1   | 2   |";
        assert_eq!(format_markdown_table(input).unwrap(), expected);
    }

    #[test]
    fn test_escaped_pipes() {
        let input = "| Expression | Result |\n| --- | --- |\n| a \\| b | true |\n| c | false |";
        let result = format_markdown_table(input).unwrap();
        assert!(result.contains("a \\| b"));
    }

    #[test]
    fn test_no_delimiter_row() {
        let input = "| A | B |\n| C | D |";
        assert!(format_markdown_table(input).is_none());
    }

    #[test]
    fn test_single_line_not_a_table() {
        let input = "| A | B |";
        assert!(format_markdown_table(input).is_none());
    }

    #[test]
    fn test_without_leading_trailing_pipes() {
        let input = "Name | Age\n--- | ---\nAlice | 30\nBob | 25";
        let expected = "| Name  | Age |\n| ----- | --- |\n| Alice | 30  |\n| Bob   | 25  |";
        assert_eq!(format_markdown_table(input).unwrap(), expected);
    }

    #[test]
    fn test_uneven_columns() {
        let input = "| A | B | C |\n| --- | --- | --- |\n| 1 | 2 |\n| x | y | z |";
        let result = format_markdown_table(input).unwrap();
        // Row with 2 cells should get an empty third column padded to min width
        assert!(result.contains("|     |"), "expected empty padded cell, got:\n{result}");
    }

    #[test]
    fn test_looks_like_table_row() {
        assert!(looks_like_table_row("| A | B |"));
        assert!(looks_like_table_row("  | A | B |"));
        assert!(looks_like_table_row("A | B | C"));
        assert!(!looks_like_table_row("no pipes here"));
        assert!(!looks_like_table_row("one | pipe"));
    }

    #[test]
    fn test_minimum_delimiter_width() {
        let input = "| A | B |\n| --- | --- |\n| x | y |";
        let result = format_markdown_table(input).unwrap();
        // Delimiter cells should be at least 3 chars wide
        assert!(result.contains("---"));
    }
}
