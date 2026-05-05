use gpui::Entity;
use project::Project;
use std::cmp;
use text::{Point, PointUtf16, ToOffset};

/// The number of lines to search above and below when a symbol isn't found on the exact line.
/// Selection ranges from the editor often start on a blank line before the actual code.
const NEARBY_LINE_SEARCH_RADIUS: u32 = 4;

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn find_word_bounded<'a>(text: &'a str, symbol: &'a str) -> impl Iterator<Item = usize> + 'a {
    find_word_bounded_impl(text, symbol, false)
}

fn find_word_bounded_case_insensitive<'a>(
    text: &'a str,
    symbol: &'a str,
) -> impl Iterator<Item = usize> + 'a {
    find_word_bounded_impl(text, symbol, true)
}

fn find_word_bounded_impl<'a>(
    text: &'a str,
    symbol: &'a str,
    case_insensitive: bool,
) -> impl Iterator<Item = usize> + 'a {
    let has_special_prefix =
        symbol.starts_with('$') || symbol.starts_with('@') || symbol.starts_with('#');
    let symbol_len = symbol.len();
    let text_lower = if case_insensitive {
        let lowered = text.to_lowercase();
        if lowered.len() != text.len() {
            // Byte length changed during lowercasing (e.g. Unicode case folding),
            // so byte offsets from the lowered string won't map back to the original.
            // Bail out — the caller will get an empty iterator.
            None
        } else {
            Some(lowered)
        }
    } else {
        None
    };
    let symbol_lower = if case_insensitive {
        let lowered = symbol.to_lowercase();
        if lowered.len() != symbol.len() {
            None
        } else {
            Some(lowered)
        }
    } else {
        None
    };
    // If case-insensitive was requested but lowercasing changed byte lengths, yield nothing.
    let bail_out = case_insensitive && (text_lower.is_none() || symbol_lower.is_none());

    let mut search_start = 0;
    std::iter::from_fn(move || {
        if bail_out {
            return None;
        }
        let haystack = text_lower.as_deref().unwrap_or(text);
        let needle = symbol_lower.as_deref().unwrap_or(symbol);
        while let Some(relative_offset) = haystack[search_start..].find(needle) {
            let absolute_offset = search_start + relative_offset;
            search_start = absolute_offset + symbol_len;

            let start_ok = if has_special_prefix {
                true
            } else if absolute_offset == 0 {
                true
            } else {
                let prev_char = text[..absolute_offset].chars().next_back().unwrap();
                !is_word_char(prev_char)
            };

            let end_offset = absolute_offset + symbol_len;
            let end_ok = if end_offset >= text.len() {
                true
            } else {
                let next_char = text[end_offset..].chars().next().unwrap();
                !is_word_char(next_char)
            };

            if start_ok && end_ok {
                return Some(absolute_offset);
            }
        }
        None
    })
}

/// Find the occurrence of `symbol` in `line_text` whose start is nearest to `target_column`
/// (measured in UTF-16 code units). Returns the UTF-16 column of that occurrence.
fn resolve_nearest_symbol_column_utf16(
    line_text: &str,
    symbol: &str,
    target_column: u32,
) -> Option<u32> {
    // Try case-sensitive first, fall back to case-insensitive.
    nearest_symbol_column_with(
        find_word_bounded(line_text, symbol),
        line_text,
        target_column,
    )
    .or_else(|| {
        nearest_symbol_column_with(
            find_word_bounded_case_insensitive(line_text, symbol),
            line_text,
            target_column,
        )
    })
}

fn nearest_symbol_column_with(
    matches: impl Iterator<Item = usize>,
    line_text: &str,
    target_column: u32,
) -> Option<u32> {
    let mut best: Option<u32> = None;
    let mut best_distance = u32::MAX;
    for byte_offset in matches {
        let prefix = &line_text[..byte_offset];
        let col_utf16 = prefix.encode_utf16().count() as u32;
        let distance = col_utf16.abs_diff(target_column);
        if distance < best_distance {
            best_distance = distance;
            best = Some(col_utf16);
        }
    }
    best
}

/// Extract the text content of a given row from a buffer snapshot.
fn line_text_for_row(snapshot: &text::BufferSnapshot, row: u32) -> String {
    let line_start = Point::new(row, 0).to_offset(snapshot);
    let line_end_col = snapshot.line_len(row);
    let line_end = Point::new(row, line_end_col).to_offset(snapshot);
    snapshot.text_for_range(line_start..line_end).collect()
}

fn resolve_symbol_column_utf16(line_text: &str, symbol: &str) -> Option<u32> {
    // Try case-sensitive first, fall back to case-insensitive.
    let byte_offset = find_word_bounded(line_text, symbol)
        .next()
        .or_else(|| find_word_bounded_case_insensitive(line_text, symbol).next())?;
    let prefix = &line_text[..byte_offset];
    Some(prefix.encode_utf16().count() as u32)
}

/// Resolve a hover/definition position from either a column or symbol name.
///
/// When `column` is `Some`, it takes priority and is treated as a character offset (0-based)
/// on the given row, which is then converted to a UTF-16 column for the LSP.
///
/// When `symbol` is `Some`, the line text is searched for the first occurrence of the symbol
/// and the UTF-16 column of that occurrence is returned.
///
/// The `row` parameter is 0-based. The `line_1based` parameter is the original 1-based line
/// number from user input, used only for error messages.
pub fn resolve_position(
    snapshot: &text::BufferSnapshot,
    row: u32,
    line_1based: u32,
    column: Option<u32>,
    symbol: Option<&str>,
) -> Result<PointUtf16, String> {
    let max_row = snapshot.max_point().row;
    if row > max_row {
        return Err(format!(
            "Line {} is out of range (file has {} lines)",
            line_1based,
            max_row + 1
        ));
    }

    // When symbol is provided, always use symbol-based search — it's the most robust
    // approach because it searches nearby lines and doesn't depend on exact line/column
    // counting. When column is also provided, it disambiguates between multiple
    // occurrences of the same symbol on a line (e.g. `$data = $data->toArray($datas)`).
    if let Some(symbol) = symbol {
        return resolve_by_symbol(snapshot, row, line_1based, max_row, symbol, column);
    }

    if let Some(column) = column {
        return resolve_by_column(snapshot, row, line_1based, max_row, column);
    }

    Err("Either `symbol` or `column` must be provided".to_string())
}

/// Resolve position by searching for a symbol name on the target line and nearby lines.
fn resolve_by_symbol(
    snapshot: &text::BufferSnapshot,
    row: u32,
    line_1based: u32,
    max_row: u32,
    symbol: &str,
    column_hint: Option<u32>,
) -> Result<PointUtf16, String> {
    // When a column hint is provided, pick the occurrence of the symbol nearest to it.
    // This disambiguates cases like `$data = $data->toArray($datas)`.
    let find_on_line = |line_text: &str| -> Option<u32> {
        if let Some(column) = column_hint {
            resolve_nearest_symbol_column_utf16(line_text, symbol, column)
        } else {
            resolve_symbol_column_utf16(line_text, symbol)
        }
    };

    // Try the exact line first
    let line_text = line_text_for_row(snapshot, row);
    if let Some(column_utf16) = find_on_line(&line_text) {
        return Ok(PointUtf16::new(row, column_utf16));
    }

    // Selection ranges often start on a blank line before the code, so search nearby.
    let search_start = row.saturating_sub(NEARBY_LINE_SEARCH_RADIUS);
    let search_end = cmp::min(row + NEARBY_LINE_SEARCH_RADIUS, max_row);
    for candidate_row in search_start..=search_end {
        if candidate_row == row {
            continue;
        }
        let candidate_text = line_text_for_row(snapshot, candidate_row);
        if let Some(column_utf16) = find_on_line(&candidate_text) {
            return Ok(PointUtf16::new(candidate_row, column_utf16));
        }
    }

    Err(format!(
        "Symbol `{}` not found on or near line {}. \
         The line content is: `{}`",
        symbol, line_1based, line_text,
    ))
}

/// Resolve position by column offset. If the column is out of range on the target line,
/// retries on line+1 since agents often land on a blank line before the code.
fn resolve_by_column(
    snapshot: &text::BufferSnapshot,
    row: u32,
    line_1based: u32,
    max_row: u32,
    column: u32,
) -> Result<PointUtf16, String> {
    let len = snapshot.line_len(row);
    let actual_row = if column <= len {
        row
    } else if row < max_row && column <= snapshot.line_len(row + 1) {
        row + 1
    } else {
        let next_info = if row < max_row {
            format!(
                ", line {} has {} characters",
                line_1based + 1,
                snapshot.line_len(row + 1)
            )
        } else {
            String::new()
        };
        return Err(format!(
            "Column {} is out of range (line {} has {} characters{})",
            column, line_1based, len, next_info,
        ));
    };

    let line_start = Point::new(actual_row, 0).to_offset(snapshot);
    let target_offset = Point::new(actual_row, column).to_offset(snapshot);
    let prefix: String = snapshot.text_for_range(line_start..target_offset).collect();
    let column_utf16 = prefix.encode_utf16().count() as u32;

    Ok(PointUtf16::new(actual_row, column_utf16))
}

/// Validate the common input fields shared by all LSP tools.
pub fn validate_lsp_tool_input(
    path: &str,
    line: u32,
    symbol: Option<&str>,
    column: Option<u32>,
) -> Result<(), String> {
    if path.is_empty() {
        return Err("Path must not be empty".to_string());
    }
    if symbol.is_none() && column.is_none() {
        return Err("Either `symbol` or `column` must be provided".to_string());
    }
    if symbol.is_some_and(|s| s.is_empty()) && column.is_none() {
        return Err("Symbol must not be empty (or provide `column` instead)".to_string());
    }
    if line == 0 {
        return Err("Line number must be 1-based (starting from 1)".to_string());
    }
    Ok(())
}

/// Open a buffer for the given path string, returning a helpful error if the path
/// isn't found in the project.
pub fn open_buffer_for_path(
    project: &mut Project,
    path: &str,
    cx: &mut gpui::Context<Project>,
) -> Result<gpui::Task<anyhow::Result<Entity<language::Buffer>>>, String> {
    let Some(project_path) = project.find_project_path(path, cx) else {
        let root_names: Vec<&str> = project.worktree_root_names(cx).collect();
        return Err(format!(
            "Could not find path `{path}` in project. \
             The path must start with one of the project's root directories: {}. \
             For example: `{}/...`",
            root_names.join(", "),
            root_names.first().unwrap_or(&"root"),
        ));
    };
    Ok(project.open_buffer(project_path, cx))
}

/// Format a display string for error messages from the combination of symbol/column inputs.
pub fn format_display_symbol(symbol: Option<&str>, column: Option<u32>) -> String {
    if let Some(symbol) = symbol {
        symbol.to_string()
    } else if let Some(column) = column {
        format!("column {column}")
    } else {
        "symbol".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_symbol_column_utf16_ascii() {
        let line = "    fn hello_world() {";
        assert_eq!(resolve_symbol_column_utf16(line, "fn"), Some(4));
        assert_eq!(resolve_symbol_column_utf16(line, "hello_world"), Some(7));
        assert_eq!(resolve_symbol_column_utf16(line, "nonexistent"), None);
    }

    #[test]
    fn test_resolve_symbol_column_utf16_first_occurrence() {
        let line = "foo(foo, bar)";
        assert_eq!(resolve_symbol_column_utf16(line, "foo"), Some(0));
    }

    #[test]
    fn test_resolve_symbol_column_utf16_unicode() {
        let line = "let café = 42;";
        assert_eq!(resolve_symbol_column_utf16(line, "café"), Some(4));
        assert_eq!(resolve_symbol_column_utf16(line, "42"), Some(11));
    }

    #[test]
    fn test_resolve_symbol_column_utf16_surrogate_pairs() {
        let line = "let 😀 = x;";
        assert_eq!(resolve_symbol_column_utf16(line, "="), Some(7));
    }

    #[test]
    fn test_resolve_symbol_column_utf16_at_start() {
        let line = "println!(\"hello\");";
        assert_eq!(resolve_symbol_column_utf16(line, "println"), Some(0));
    }

    #[test]
    fn test_resolve_nearest_symbol_first_occurrence() {
        let line = "$data = $data->toArray($datas);";
        // Column 0 is nearest to the first "$data" at column 0
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "$data", 0),
            Some(0)
        );
    }

    #[test]
    fn test_resolve_nearest_symbol_second_occurrence() {
        let line = "$data = $data->toArray($datas);";
        // Column 10 is nearest to the second "$data" at column 8
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "$data", 10),
            Some(8)
        );
    }

    #[test]
    fn test_resolve_nearest_symbol_third_occurrence() {
        let line = "$data = $data->toArray($datas);";
        // "$datas" at column 23 is NOT a word-bounded match for "$data" because the
        // character after "$data" is 's', which is a word character. With column hint
        // 23, the nearest valid word-bounded match is the second "$data" at column 8.
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "$data", 23),
            Some(8)
        );
    }

    #[test]
    fn test_resolve_nearest_symbol_exact_match() {
        let line = "foo(bar, foo, baz, foo)";
        // Exactly at the second "foo" (column 9)
        assert_eq!(resolve_nearest_symbol_column_utf16(line, "foo", 9), Some(9));
        // Exactly at the third "foo" (column 19)
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "foo", 19),
            Some(19)
        );
        // Midway between second (9) and third (19), ties go to the closer one
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "foo", 13),
            Some(9)
        );
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "foo", 15),
            Some(19)
        );
    }

    #[test]
    fn test_resolve_nearest_symbol_no_match() {
        let line = "let x = 42;";
        assert_eq!(resolve_nearest_symbol_column_utf16(line, "foo", 5), None);
    }

    #[test]
    fn test_resolve_nearest_symbol_single_occurrence() {
        let line = "let x = foo();";
        // With only one occurrence, column hint doesn't matter
        assert_eq!(resolve_nearest_symbol_column_utf16(line, "foo", 0), Some(8));
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "foo", 100),
            Some(8)
        );
    }

    #[test]
    fn test_resolve_symbol_column_utf16_word_boundary() {
        let line = "public function captureReservedPayment(Payment $payment, Decimal $captureAmount): void";
        // Should match standalone "Payment" (column 39), NOT the one inside "captureReservedPayment"
        assert_eq!(resolve_symbol_column_utf16(line, "Payment"), Some(39));
    }

    #[test]
    fn test_resolve_symbol_column_utf16_case_insensitive_fallback() {
        let line = "                'payment_token_value' => $e->paymentToken->token->value,";
        // "PaymentToken" doesn't match case-sensitively, but matches "paymentToken" case-insensitively
        assert_eq!(resolve_symbol_column_utf16(line, "PaymentToken"), Some(45));
    }

    #[test]
    fn test_resolve_symbol_column_utf16_case_sensitive_preferred() {
        let line = "let PaymentToken = paymentToken;";
        // Both match, but case-sensitive "PaymentToken" at column 4 should be preferred
        assert_eq!(resolve_symbol_column_utf16(line, "PaymentToken"), Some(4));
    }

    #[test]
    fn test_resolve_nearest_symbol_case_insensitive_fallback() {
        let line = "                'payment_token_value' => $e->paymentToken->token->value,";
        // "PaymentToken" doesn't match case-sensitively, falls back to case-insensitive
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "PaymentToken", 45),
            Some(45)
        );
    }

    #[test]
    fn test_resolve_symbol_column_utf16_word_boundary_php_variable() {
        let line = "public function captureReservedPayment(Payment $payment, Decimal $captureAmount): void";
        // PHP variable $payment should match at column 47
        assert_eq!(resolve_symbol_column_utf16(line, "$payment"), Some(47));
    }

    #[test]
    fn test_resolve_symbol_column_utf16_word_boundary_at_start() {
        let line = "Payment::create($data);";
        assert_eq!(resolve_symbol_column_utf16(line, "Payment"), Some(0));
    }

    #[test]
    fn test_resolve_symbol_column_utf16_word_boundary_at_end() {
        let line = "use App\\Models\\Payment";
        assert_eq!(resolve_symbol_column_utf16(line, "Payment"), Some(15));
    }

    #[test]
    fn test_resolve_symbol_column_utf16_no_false_partial_match() {
        let line = "let payment_processor = PaymentProcessor::new();";
        // "Payment" appears as a substring of "PaymentProcessor" but not as a standalone word
        // It should NOT match "payment_processor" (lowercase) nor "PaymentProcessor" (word continues)
        assert_eq!(resolve_symbol_column_utf16(line, "Payment"), None);
    }

    #[test]
    fn test_resolve_nearest_symbol_word_boundary() {
        let line = "public function captureReservedPayment(Payment $payment): void";
        // Column 39 should match the standalone "Payment" at column 39
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "Payment", 39),
            Some(39)
        );
        // Column 0 should also match the standalone "Payment" at column 39 (only word-bounded match)
        assert_eq!(
            resolve_nearest_symbol_column_utf16(line, "Payment", 0),
            Some(39)
        );
    }
}
