use crate::{
    CURSOR_MARKER, EDITABLE_REGION_END_MARKER, EDITABLE_REGION_START_MARKER, START_OF_FILE_MARKER,
    tokens_for_bytes,
};
use language::{BufferSnapshot, Point};
use std::{fmt::Write, ops::Range};

#[derive(Debug)]
pub struct InputExcerpt {
    pub editable_range: Range<Point>,
    pub prompt: String,
    pub speculated_output: String,
}

pub fn excerpt_for_cursor_position(
    position: Point,
    path: &str,
    snapshot: &BufferSnapshot,
    editable_region_token_limit: usize,
    context_token_limit: usize,
) -> InputExcerpt {
    let mut scope_range = position..position;
    let mut remaining_edit_tokens = editable_region_token_limit;

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

    let editable_range = expand_range(snapshot, scope_range, remaining_edit_tokens);
    let context_range = expand_range(snapshot, editable_range.clone(), context_token_limit);

    let mut prompt = String::new();
    let mut speculated_output = String::new();

    writeln!(&mut prompt, "```{path}").unwrap();
    if context_range.start == Point::zero() {
        writeln!(&mut prompt, "{START_OF_FILE_MARKER}").unwrap();
    }

    for chunk in snapshot.chunks(context_range.start..editable_range.start, false) {
        prompt.push_str(chunk.text);
    }

    push_editable_range(position, snapshot, editable_range.clone(), &mut prompt);
    push_editable_range(
        position,
        snapshot,
        editable_range.clone(),
        &mut speculated_output,
    );

    for chunk in snapshot.chunks(editable_range.end..context_range.end, false) {
        prompt.push_str(chunk.text);
    }
    write!(prompt, "\n```").unwrap();

    InputExcerpt {
        editable_range,
        prompt,
        speculated_output,
    }
}

fn push_editable_range(
    cursor_position: Point,
    snapshot: &BufferSnapshot,
    editable_range: Range<Point>,
    prompt: &mut String,
) {
    writeln!(prompt, "{EDITABLE_REGION_START_MARKER}").unwrap();
    for chunk in snapshot.chunks(editable_range.start..cursor_position, false) {
        prompt.push_str(chunk.text);
    }
    prompt.push_str(CURSOR_MARKER);
    for chunk in snapshot.chunks(cursor_position..editable_range.end, false) {
        prompt.push_str(chunk.text);
    }
    write!(prompt, "\n{EDITABLE_REGION_END_MARKER}").unwrap();
}

fn expand_range(
    snapshot: &BufferSnapshot,
    range: Range<Point>,
    mut remaining_tokens: usize,
) -> Range<Point> {
    let mut expanded_range = range.clone();
    expanded_range.start.column = 0;
    expanded_range.end.column = snapshot.line_len(expanded_range.end.row);
    loop {
        let mut expanded = false;

        if remaining_tokens > 0 && expanded_range.start.row > 0 {
            expanded_range.start.row -= 1;
            let line_tokens =
                tokens_for_bytes(snapshot.line_len(expanded_range.start.row) as usize);
            remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
            expanded = true;
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{App, AppContext};
    use indoc::indoc;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher};
    use std::sync::Arc;

    #[gpui::test]
    fn test_excerpt_for_cursor_position(cx: &mut App) {
        let text = indoc! {r#"
            fn foo() {
                let x = 42;
                println!("Hello, world!");
            }

            fn bar() {
                let x = 42;
                let mut sum = 0;
                for i in 0..x {
                    sum += i;
                }
                println!("Sum: {}", sum);
                return sum;
            }

            fn generate_random_numbers() -> Vec<i32> {
                let mut rng = rand::thread_rng();
                let mut numbers = Vec::new();
                for _ in 0..5 {
                    numbers.push(rng.gen_range(1..101));
                }
                numbers
            }
        "#};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let snapshot = buffer.read(cx).snapshot();

        // Ensure we try to fit the largest possible syntax scope, resorting to line-based expansion
        // when a larger scope doesn't fit the editable region.
        let excerpt = excerpt_for_cursor_position(Point::new(12, 5), "main.rs", &snapshot, 50, 32);
        assert_eq!(
            excerpt.prompt,
            indoc! {r#"
            ```main.rs
                let x = 42;
                println!("Hello, world!");
            <|editable_region_start|>
            }

            fn bar() {
                let x = 42;
                let mut sum = 0;
                for i in 0..x {
                    sum += i;
                }
                println!("Sum: {}", sum);
                r<|user_cursor_is_here|>eturn sum;
            }

            fn generate_random_numbers() -> Vec<i32> {
            <|editable_region_end|>
                let mut rng = rand::thread_rng();
                let mut numbers = Vec::new();
            ```"#}
        );

        // The `bar` function won't fit within the editable region, so we resort to line-based expansion.
        let excerpt = excerpt_for_cursor_position(Point::new(12, 5), "main.rs", &snapshot, 40, 32);
        assert_eq!(
            excerpt.prompt,
            indoc! {r#"
            ```main.rs
            fn bar() {
                let x = 42;
                let mut sum = 0;
            <|editable_region_start|>
                for i in 0..x {
                    sum += i;
                }
                println!("Sum: {}", sum);
                r<|user_cursor_is_here|>eturn sum;
            }

            fn generate_random_numbers() -> Vec<i32> {
                let mut rng = rand::thread_rng();
            <|editable_region_end|>
                let mut numbers = Vec::new();
                for _ in 0..5 {
                    numbers.push(rng.gen_range(1..101));
            ```"#}
        );
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
    }
}
