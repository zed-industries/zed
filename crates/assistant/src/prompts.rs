use gpui::AppContext;
use language::{BufferSnapshot, OffsetRangeExt, ToOffset};
use std::cmp;
use std::ops::Range;
use std::{fmt::Write, iter};

use crate::codegen::CodegenKind;

fn outline_for_prompt(
    buffer: &BufferSnapshot,
    range: Range<language::Anchor>,
    cx: &AppContext,
) -> Option<String> {
    let indent = buffer
        .language_indent_size_at(0, cx)
        .chars()
        .collect::<String>();
    let outline = buffer.outline(None)?;
    let range = range.to_offset(buffer);

    let mut text = String::new();
    let mut items = outline.items.into_iter().peekable();

    let mut intersected = false;
    let mut intersection_indent = 0;
    let mut extended_range = range.clone();

    while let Some(item) = items.next() {
        let item_range = item.range.to_offset(buffer);
        if item_range.end < range.start || item_range.start > range.end {
            text.extend(iter::repeat(indent.as_str()).take(item.depth));
            text.push_str(&item.text);
            text.push('\n');
        } else {
            intersected = true;
            let is_terminal = items
                .peek()
                .map_or(true, |next_item| next_item.depth <= item.depth);
            if is_terminal {
                if item_range.start <= extended_range.start {
                    extended_range.start = item_range.start;
                    intersection_indent = item.depth;
                }
                extended_range.end = cmp::max(extended_range.end, item_range.end);
            } else {
                let name_start = item_range.start + item.name_ranges.first().unwrap().start;
                let name_end = item_range.start + item.name_ranges.last().unwrap().end;

                if range.start > name_end {
                    text.extend(iter::repeat(indent.as_str()).take(item.depth));
                    text.push_str(&item.text);
                    text.push('\n');
                } else {
                    if name_start <= extended_range.start {
                        extended_range.start = item_range.start;
                        intersection_indent = item.depth;
                    }
                    extended_range.end = cmp::max(extended_range.end, name_end);
                }
            }
        }

        if intersected
            && items.peek().map_or(true, |next_item| {
                next_item.range.start.to_offset(buffer) > range.end
            })
        {
            intersected = false;
            text.extend(iter::repeat(indent.as_str()).take(intersection_indent));
            text.extend(buffer.text_for_range(extended_range.start..range.start));
            text.push_str("<|START|");
            text.extend(buffer.text_for_range(range.clone()));
            if range.start != range.end {
                text.push_str("|END|>");
            } else {
                text.push_str(">");
            }
            text.extend(buffer.text_for_range(range.end..extended_range.end));
            text.push('\n');
        }
    }

    Some(text)
}

pub fn generate_content_prompt(
    user_prompt: String,
    language_name: Option<&str>,
    buffer: &BufferSnapshot,
    range: Range<language::Anchor>,
    cx: &AppContext,
    kind: CodegenKind,
) -> String {
    let mut prompt = String::new();

    // General Preamble
    if let Some(language_name) = language_name {
        writeln!(prompt, "You're an expert {language_name} engineer.\n").unwrap();
    } else {
        writeln!(prompt, "You're an expert engineer.\n").unwrap();
    }

    let outline = outline_for_prompt(buffer, range.clone(), cx);
    if let Some(outline) = outline {
        writeln!(
            prompt,
            "The file you are currently working on has the following outline:"
        )
        .unwrap();
        if let Some(language_name) = language_name {
            let language_name = language_name.to_lowercase();
            writeln!(prompt, "```{language_name}\n{outline}\n```").unwrap();
        } else {
            writeln!(prompt, "```\n{outline}\n```").unwrap();
        }
    }

    // Assume for now that we are just generating
    if range.clone().start == range.end {
        writeln!(prompt, "In particular, the user's cursor is current on the '<|START|>' span in the above outline, with no text selected.").unwrap();
    } else {
        writeln!(prompt, "In particular, the user has selected a section of the text between the '<|START|' and '|END|>' spans.").unwrap();
    }

    match kind {
        CodegenKind::Generate { position: _ } => {
            writeln!(
                prompt,
                "Assume the cursor is located where the `<|START|` marker is."
            )
            .unwrap();
            writeln!(
                prompt,
                "Text can't be replaced, so assume your answer will be inserted at the cursor."
            )
            .unwrap();
            writeln!(
                prompt,
                "Generate text based on the users prompt: {user_prompt}"
            )
            .unwrap();
        }
        CodegenKind::Transform { range: _ } => {
            writeln!(
                prompt,
                "Modify the users code selected text based upon the users prompt: {user_prompt}"
            )
            .unwrap();
            writeln!(
                prompt,
                "You MUST reply with only the adjusted code (within the '<|START|' and '|END|>' spans), not the entire file."
            )
            .unwrap();
        }
    }

    if let Some(language_name) = language_name {
        writeln!(prompt, "Your answer MUST always be valid {language_name}").unwrap();
    }
    writeln!(prompt, "Always wrap your response in a Markdown codeblock").unwrap();
    writeln!(prompt, "Never make remarks about the output.").unwrap();

    prompt
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use std::sync::Arc;

    use gpui::AppContext;
    use indoc::indoc;
    use language::{language_settings, tree_sitter_rust, Buffer, Language, LanguageConfig, Point};
    use settings::SettingsStore;

    pub(crate) fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(
            r#"
                (call_expression) @indent
                (field_expression) @indent
                (_ "(" ")" @end) @indent
                (_ "{" "}" @end) @indent
                "#,
        )
        .unwrap()
        .with_outline_query(
            r#"
                (struct_item
                    "struct" @context
                    name: (_) @name) @item
                (enum_item
                    "enum" @context
                    name: (_) @name) @item
                (enum_variant
                    name: (_) @name) @item
                (field_declaration
                    name: (_) @name) @item
                (impl_item
                    "impl" @context
                    trait: (_)? @name
                    "for"? @context
                    type: (_) @name) @item
                (function_item
                    "fn" @context
                    name: (_) @name) @item
                (mod_item
                    "mod" @context
                    name: (_) @name) @item
                "#,
        )
        .unwrap()
    }

    #[gpui::test]
    fn test_outline_for_prompt(cx: &mut AppContext) {
        cx.set_global(SettingsStore::test(cx));
        language_settings::init(cx);
        let text = indoc! {"
            struct X {
                a: usize,
                b: usize,
            }

            impl X {

                fn new() -> Self {
                    let a = 1;
                    let b = 2;
                    Self { a, b }
                }

                pub fn a(&self, param: bool) -> usize {
                    self.a
                }

                pub fn b(&self) -> usize {
                    self.b
                }
            }
        "};
        let buffer =
            cx.add_model(|cx| Buffer::new(0, 0, text).with_language(Arc::new(rust_lang()), cx));
        let snapshot = buffer.read(cx).snapshot();

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(1, 4))..snapshot.anchor_before(Point::new(1, 4)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    <||>a: usize
                    b
                impl X
                    fn new
                    fn a
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(8, 12))..snapshot.anchor_before(Point::new(8, 14)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X
                    fn new() -> Self {
                        let <|a |>= 1;
                        let b = 2;
                        Self { a, b }
                    }
                    fn a
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(6, 0))..snapshot.anchor_before(Point::new(6, 0)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X
                <||>
                    fn new
                    fn a
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(8, 12))..snapshot.anchor_before(Point::new(13, 9)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X
                    fn new() -> Self {
                        let <|a = 1;
                        let b = 2;
                        Self { a, b }
                    }

                    pub f|>n a(&self, param: bool) -> usize {
                        self.a
                    }
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(5, 6))..snapshot.anchor_before(Point::new(12, 0)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X<| {

                    fn new() -> Self {
                        let a = 1;
                        let b = 2;
                        Self { a, b }
                    }
                |>
                    fn a
                    fn b
            "})
        );

        let outline = outline_for_prompt(
            &snapshot,
            snapshot.anchor_before(Point::new(18, 8))..snapshot.anchor_before(Point::new(18, 8)),
            cx,
        );
        assert_eq!(
            outline.as_deref(),
            Some(indoc! {"
                struct X
                    a
                    b
                impl X
                    fn new
                    fn a
                    pub fn b(&self) -> usize {
                        <||>self.b
                    }
            "})
        );
    }
}
