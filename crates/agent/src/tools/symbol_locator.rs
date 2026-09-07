use std::cmp;
use std::fmt;

use gpui::{App, AsyncApp, Entity};
use language::{Buffer, Location};
use project::{CodeAction, Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use text::ToPoint as _;
use text::{Anchor, Point};

/// The number of lines to search above and below when a symbol isn't found on the exact line.
/// Agents often land on a blank line before the actual code due to selection ranges.
const NEARBY_LINE_SEARCH_RADIUS: u32 = 4;

/// Identifies a specific symbol (declaration or usage) in the source code.
///
/// Use the file path, line number, and symbol name from file outlines, grep results, or other tool outputs to populate these fields.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SymbolLocator {
    /// The relative path of the file containing the symbol (e.g. "crates/editor/src/editor.rs").
    pub file_path: String,

    /// The 1-based line number where the symbol appears. Use the line numbers from file outlines or grep results.
    pub line: u32,

    /// The name of the symbol (function name, type name, variable name, etc.)
    pub symbol_name: String,
}

pub struct PendingCodeActions {
    pub actions: Vec<CodeAction>,
    pub buffer: Entity<Buffer>,
}

pub type CodeActionStore = Entity<Option<PendingCodeActions>>;

pub struct ResolvedSymbol {
    pub buffer: Entity<Buffer>,
    pub position: Anchor,
    pub line_text: String,
    pub truncated: bool,
}

pub const MAX_LINE_DISPLAY_LEN: usize = 200;

pub struct LocationDisplay {
    pub path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub snippet: String,
    pub truncated: bool,
}

impl LocationDisplay {
    pub fn from_location(location: &Location, cx: &App) -> Self {
        let snapshot = location.buffer.read(cx).snapshot();
        let range =
            location.range.start.to_point(&snapshot)..location.range.end.to_point(&snapshot);
        let path = location
            .buffer
            .read(cx)
            .file()
            .map(|f| f.full_path(cx).display().to_string())
            .unwrap_or_else(|| "<untitled>".to_string());

        let start_line = range.start.row + 1;
        let end_line = range.end.row + 1;

        let line_len = snapshot.line_len(range.start.row);
        let truncated = line_len as usize > MAX_LINE_DISPLAY_LEN;
        let snippet: String = snapshot
            .text_for_range(Point::new(range.start.row, 0)..Point::new(range.start.row, line_len))
            .flat_map(|chunk| chunk.chars())
            .skip_while(|c| c.is_whitespace())
            .take(MAX_LINE_DISPLAY_LEN)
            .collect::<String>();
        let snippet = snippet.trim_end().to_string();

        Self {
            path,
            start_line,
            end_line,
            snippet,
            truncated,
        }
    }
}

impl fmt::Display for LocationDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let truncated_label = if self.truncated { " (truncated)" } else { "" };
        if self.start_line == self.end_line {
            writeln!(f, "{}#L{}{truncated_label}", self.path, self.start_line)?;
        } else {
            writeln!(
                f,
                "{}#L{}-{}{truncated_label}",
                self.path, self.start_line, self.end_line
            )?;
        }
        writeln!(f, "```")?;
        writeln!(f, "{}", self.snippet)?;
        write!(f, "```")
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '$' | '@' | '#' | '-')
}

/// Find the first occurrence of `symbol` as a complete token in `line_text`.
/// Returns the byte offset of the match, or None.
/// A match is "complete" when the characters immediately before and after are
/// not identifier characters (alphanumeric, `_`, `$`, `@`, `#`, `-`).
fn find_symbol_on_line(line_text: &str, symbol: &str) -> Option<usize> {
    let symbol_len = symbol.len();
    if symbol_len == 0 || symbol_len > line_text.len() {
        return None;
    }
    let mut start = 0;
    while let Some(offset) = line_text[start..].find(symbol) {
        let abs = start + offset;
        let before_ok = abs == 0
            || !line_text[..abs]
                .chars()
                .next_back()
                .is_some_and(is_identifier_char);
        let end = abs + symbol_len;
        let after_ok =
            end >= line_text.len() || !line_text[end..].chars().next().is_some_and(is_identifier_char);
        if before_ok && after_ok {
            return Some(abs);
        }
        start = abs + symbol_len;
    }
    None
}

/// Extract the text content of a given row from a buffer snapshot.
fn line_text_for_row(snapshot: &text::BufferSnapshot, row: u32) -> String {
    let line_start = Point::new(row, 0);
    let line_len = snapshot.line_len(row);
    let line_end = Point::new(row, line_len);
    snapshot
        .text_for_range(line_start..line_end)
        .collect::<String>()
}

impl SymbolLocator {
    /// Resolves this locator into a concrete buffer and position.
    ///
    /// Opens the file at `file_path`, then searches for `symbol_name` on the
    /// specified `line` using token-boundary matching. If not found on the exact
    /// line, searches nearby lines (±4 lines) as agents often land on blank lines
    /// before the actual code.
    pub async fn resolve(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<ResolvedSymbol, String> {
        let Self {
            file_path,
            line,
            symbol_name,
        } = self;

        let open_buffer_task = project.update(cx, |project, cx| {
            let Some(project_path) = project.find_project_path(file_path, cx) else {
                return Err(format!("Could not find path '{file_path}' in project",));
            };
            Ok(project.open_buffer(project_path, cx))
        })?;

        let buffer = open_buffer_task
            .await
            .map_err(|e| format!("Failed to open '{}': {e}", self.file_path))?;

        let (position, line_text, truncated) = buffer.read_with(cx, |buffer, _cx| {
            let snapshot = buffer.snapshot();
            let row = line.saturating_sub(1);

            if row > snapshot.max_point().row {
                let line_count = snapshot.max_point().row + 1;
                return Err(format!(
                    "Line {line} is beyond the end of '{file_path}' (file has {line_count} lines)",
                ));
            }

            let max_row = snapshot.max_point().row;

            // Try the exact line first
            let target_line_text = line_text_for_row(&snapshot, row);
            if let Some(byte_offset) = find_symbol_on_line(&target_line_text, symbol_name) {
                let position = snapshot.anchor_before(Point::new(row, byte_offset as u32));
                let display_text = format_line_display(&target_line_text);
                let truncated = target_line_text.len() > MAX_LINE_DISPLAY_LEN;
                return Ok((position, display_text, truncated));
            }

            // Search nearby lines (agents often land on blank lines before code)
            let search_start = row.saturating_sub(NEARBY_LINE_SEARCH_RADIUS);
            let search_end = cmp::min(row + NEARBY_LINE_SEARCH_RADIUS, max_row);
            for candidate_row in search_start..=search_end {
                if candidate_row == row {
                    continue;
                }
                let candidate_text = line_text_for_row(&snapshot, candidate_row);
                if let Some(byte_offset) = find_symbol_on_line(&candidate_text, symbol_name) {
                    let position =
                        snapshot.anchor_before(Point::new(candidate_row, byte_offset as u32));
                    let display_text = format_line_display(&candidate_text);
                    let truncated = candidate_text.len() > MAX_LINE_DISPLAY_LEN;
                    return Ok((position, display_text, truncated));
                }
            }

            let preview: String = target_line_text
                .chars()
                .skip_while(|c| c.is_whitespace())
                .take(MAX_LINE_DISPLAY_LEN)
                .collect();
            Err(format!(
                "Symbol '{symbol_name}' not found on line {line} of '{file_path}'. \
                 Line content: '{}'",
                preview.trim_end()
            ))
        })?;

        Ok(ResolvedSymbol {
            buffer,
            position,
            line_text,
            truncated,
        })
    }
}

fn format_line_display(line_text: &str) -> String {
    let truncated = if line_text.len() > MAX_LINE_DISPLAY_LEN {
        &line_text[..MAX_LINE_DISPLAY_LEN]
    } else {
        line_text
    };
    truncated.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_boundary_prevents_partial_match() {
        let line = "let payment_processor = PaymentProcessor::new();";
        // "Payment" is a substring of "PaymentProcessor" but not a standalone word
        assert_eq!(find_symbol_on_line(line, "Payment"), None);
    }

    #[test]
    fn test_word_boundary_matches_standalone() {
        let line = "pub fn captureReservedPayment(Payment $payment): void";
        // "Payment" as a standalone word at the parameter type position
        let offset = find_symbol_on_line(line, "Payment");
        assert!(offset.is_some());
        assert_eq!(&line[offset.unwrap()..offset.unwrap() + 7], "Payment");
    }

    #[test]
    fn test_special_prefix_php_variable() {
        let line = "public function capture(Payment $payment): void";
        let offset = find_symbol_on_line(line, "$payment");
        assert!(offset.is_some());
        assert_eq!(&line[offset.unwrap()..offset.unwrap() + 8], "$payment");
    }

    #[test]
    fn test_no_match_returns_none() {
        let line = "let x = 42;";
        assert_eq!(find_symbol_on_line(line, "nonexistent"), None);
    }
}
