use language::{BufferSnapshot, Point};
use std::{fmt::Write, ops::Range};

// Token estimation for limiting context size
fn tokens_for_bytes(bytes: usize) -> usize {
    /// Typical number of string bytes per token for the purposes of limiting model input. This is
    /// intentionally low to err on the side of underestimating limits.
    const BYTES_PER_TOKEN_GUESS: usize = 3;
    bytes / BYTES_PER_TOKEN_GUESS
}

/// Special marker indicating the user's cursor position
pub const CURSOR_MARKER: &str = "<|user_cursor_is_here|>";
/// Marker indicating the file starts at this position (no content before)
pub const START_OF_FILE_MARKER: &str = "<|start_of_file|>";

/// Contains the formatted input for the language model with context around the cursor position.
#[derive(Debug)]
pub struct InputExcerpt {
    /// The range in the buffer that provides context (kept for API compatibility)
    #[allow(dead_code)]
    pub editable_range: Range<Point>,
    /// The complete formatted prompt including context and cursor marker
    pub prompt: String,
    /// Not used in simplified implementation (kept for API compatibility)
    pub speculated_output: String,
}

/// Selects an intelligent excerpt of code around the cursor position.
///
/// This function uses a multi-layered approach:
/// 1. **Tree-sitter based selection**: Finds the smallest syntax node (function, loop, etc.)
///    that contains the cursor and fits within the token limit
/// 2. **Line-based expansion**: If there's room, expands to include complete lines
/// 3. **Context addition**: Adds surrounding context for better predictions
///
/// The result is a well-balanced excerpt that gives the model enough context to make
/// intelligent completions at the cursor position.
///
/// # Arguments
/// * `position` - The cursor position in the buffer
/// * `path` - The file path (for display in the prompt)
/// * `snapshot` - The buffer snapshot
/// * `editable_region_token_limit` - Max tokens for the main context region
/// * `context_token_limit` - Additional tokens for surrounding context
pub fn excerpt_for_cursor_position(
    position: Point,
    path: &str,
    snapshot: &BufferSnapshot,
    editable_region_token_limit: usize,
    context_token_limit: usize,
) -> InputExcerpt {
    let mut scope_range = position..position;
    let mut remaining_edit_tokens = editable_region_token_limit;

    // Try to find a suitable syntax scope using tree-sitter
    while let Some(parent) = snapshot.syntax_ancestor(scope_range.clone()) {
        let parent_tokens = tokens_for_bytes(parent.byte_range().len());
        let parent_point_range = Point::new(
            parent.start_position().row as u32,
            parent.start_position().column as u32,
        )
            ..Point::new(
                parent.end_position().row as u32,
                parent.end_position().column as u32,
            );
        if parent_point_range == scope_range {
            break;
        } else if parent_tokens <= editable_region_token_limit {
            scope_range = parent_point_range;
            remaining_edit_tokens = editable_region_token_limit - parent_tokens;
        } else {
            break;
        }
    }

    // Expand the range to include complete lines and nearby context
    let main_range = expand_range(snapshot, scope_range, remaining_edit_tokens);
    let context_range = expand_range(snapshot, main_range.clone(), context_token_limit);

    let mut prompt = String::new();

    // Build the prompt with file path
    writeln!(&mut prompt, "```{path}").unwrap();
    if context_range.start == Point::zero() {
        writeln!(&mut prompt, "{START_OF_FILE_MARKER}").unwrap();
    }

    // Add all context with cursor marker at the appropriate position
    for chunk in snapshot.chunks(context_range.start..position, false) {
        prompt.push_str(chunk.text);
    }

    // Add cursor marker
    prompt.push_str(CURSOR_MARKER);

    // Add text after cursor
    for chunk in snapshot.chunks(position..context_range.end, false) {
        prompt.push_str(chunk.text);
    }
    write!(prompt, "\n```").unwrap();

    InputExcerpt {
        editable_range: main_range,
        prompt,
        speculated_output: String::new(), // Not used in simplified implementation
    }
}

fn expand_range(
    snapshot: &BufferSnapshot,
    range: Range<Point>,
    mut remaining_tokens: usize,
) -> Range<Point> {
    let mut expanded_range = range.clone();
    // Start at beginning of line
    expanded_range.start.column = 0;
    // End at end of line
    expanded_range.end.column = snapshot.line_len(expanded_range.end.row);

    loop {
        let mut expanded = false;

        // Try to expand upward
        if remaining_tokens > 0 && expanded_range.start.row > 0 {
            expanded_range.start.row -= 1;
            let line_tokens =
                tokens_for_bytes(snapshot.line_len(expanded_range.start.row) as usize);
            remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
            expanded = true;
        }

        // Try to expand downward
        if remaining_tokens > 0 && expanded_range.end.row < snapshot.max_point().row {
            expanded_range.end.row += 1;
            expanded_range.end.column = snapshot.line_len(expanded_range.end.row);
            let line_tokens = tokens_for_bytes(expanded_range.end.column as usize);
            remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
            expanded = true;
        }

        if !expanded {
            break;
        }
    }

    expanded_range
}

pub fn prompt_for_outline(snapshot: &BufferSnapshot) -> String {
    use std::borrow::Cow;

    let mut input_outline = String::new();

    writeln!(
        input_outline,
        "```{}",
        snapshot
            .file()
            .map_or(Cow::Borrowed("untitled"), |file| file
                .path()
                .to_string_lossy())
    )
    .unwrap();

    let outline = snapshot.outline(None);
    for item in &outline.items {
        let spacing = " ".repeat(item.depth);
        writeln!(input_outline, "{}{}", spacing, item.text).unwrap();
    }

    writeln!(input_outline, "```").unwrap();

    input_outline
}
