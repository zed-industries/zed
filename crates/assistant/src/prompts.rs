use crate::codegen::CodegenKind;
use language::{BufferSnapshot, OffsetRangeExt, ToOffset};
use std::cmp::{self, Reverse};
use std::fmt::Write;
use std::ops::Range;

#[allow(dead_code)]
fn summarize(buffer: &BufferSnapshot, selected_range: Range<impl ToOffset>) -> String {
    #[derive(Debug)]
    struct Match {
        collapse: Range<usize>,
        keep: Vec<Range<usize>>,
    }

    let selected_range = selected_range.to_offset(buffer);
    let mut ts_matches = buffer.matches(0..buffer.len(), |grammar| {
        Some(&grammar.embedding_config.as_ref()?.query)
    });
    let configs = ts_matches
        .grammars()
        .iter()
        .map(|g| g.embedding_config.as_ref().unwrap())
        .collect::<Vec<_>>();
    let mut matches = Vec::new();
    while let Some(mat) = ts_matches.peek() {
        let config = &configs[mat.grammar_index];
        if let Some(collapse) = mat.captures.iter().find_map(|cap| {
            if Some(cap.index) == config.collapse_capture_ix {
                Some(cap.node.byte_range())
            } else {
                None
            }
        }) {
            let mut keep = Vec::new();
            for capture in mat.captures.iter() {
                if Some(capture.index) == config.keep_capture_ix {
                    keep.push(capture.node.byte_range());
                } else {
                    continue;
                }
            }
            ts_matches.advance();
            matches.push(Match { collapse, keep });
        } else {
            ts_matches.advance();
        }
    }
    matches.sort_unstable_by_key(|mat| (mat.collapse.start, Reverse(mat.collapse.end)));
    let mut matches = matches.into_iter().peekable();

    let mut summary = String::new();
    let mut offset = 0;
    let mut flushed_selection = false;
    while let Some(mat) = matches.next() {
        // Keep extending the collapsed range if the next match surrounds
        // the current one.
        while let Some(next_mat) = matches.peek() {
            if mat.collapse.start <= next_mat.collapse.start
                && mat.collapse.end >= next_mat.collapse.end
            {
                matches.next().unwrap();
            } else {
                break;
            }
        }

        if offset > mat.collapse.start {
            // Skip collapsed nodes that have already been summarized.
            offset = cmp::max(offset, mat.collapse.end);
            continue;
        }

        if offset <= selected_range.start && selected_range.start <= mat.collapse.end {
            if !flushed_selection {
                // The collapsed node ends after the selection starts, so we'll flush the selection first.
                summary.extend(buffer.text_for_range(offset..selected_range.start));
                summary.push_str("<|START|");
                if selected_range.end == selected_range.start {
                    summary.push_str(">");
                } else {
                    summary.extend(buffer.text_for_range(selected_range.clone()));
                    summary.push_str("|END|>");
                }
                offset = selected_range.end;
                flushed_selection = true;
            }

            // If the selection intersects the collapsed node, we won't collapse it.
            if selected_range.end >= mat.collapse.start {
                continue;
            }
        }

        summary.extend(buffer.text_for_range(offset..mat.collapse.start));
        for keep in mat.keep {
            summary.extend(buffer.text_for_range(keep));
        }
        offset = mat.collapse.end;
    }

    // Flush selection if we haven't already done so.
    if !flushed_selection && offset <= selected_range.start {
        summary.extend(buffer.text_for_range(offset..selected_range.start));
        summary.push_str("<|START|");
        if selected_range.end == selected_range.start {
            summary.push_str(">");
        } else {
            summary.extend(buffer.text_for_range(selected_range.clone()));
            summary.push_str("|END|>");
        }
        offset = selected_range.end;
    }

    summary.extend(buffer.text_for_range(offset..buffer.len()));
    summary
}

pub fn generate_content_prompt(
    user_prompt: String,
    language_name: Option<&str>,
    buffer: &BufferSnapshot,
    range: Range<impl ToOffset>,
    kind: CodegenKind,
) -> String {
    let range = range.to_offset(buffer);
    let mut prompt = String::new();

    // General Preamble
    if let Some(language_name) = language_name {
        writeln!(prompt, "You're an expert {language_name} engineer.\n").unwrap();
    } else {
        writeln!(prompt, "You're an expert engineer.\n").unwrap();
    }

    let mut content = String::new();
    content.extend(buffer.text_for_range(0..range.start));
    if range.start == range.end {
        content.push_str("<|START|>");
    } else {
        content.push_str("<|START|");
    }
    content.extend(buffer.text_for_range(range.clone()));
    if range.start != range.end {
        content.push_str("|END|>");
    }
    content.extend(buffer.text_for_range(range.end..buffer.len()));

    writeln!(
        prompt,
        "The file you are currently working on has the following content:"
    )
    .unwrap();
    if let Some(language_name) = language_name {
        let language_name = language_name.to_lowercase();
        writeln!(prompt, "```{language_name}\n{content}\n```").unwrap();
    } else {
        writeln!(prompt, "```\n{content}\n```").unwrap();
    }

    match kind {
        CodegenKind::Generate { position: _ } => {
            writeln!(prompt, "In particular, the user's cursor is current on the '<|START|>' span in the above outline, with no text selected.").unwrap();
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
            writeln!(prompt, "In particular, the user has selected a section of the text between the '<|START|' and '|END|>' spans.").unwrap();
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
        .with_embedding_query(
            r#"
            (
                [(line_comment) (attribute_item)]* @context
                .
                [
                    (struct_item
                        name: (_) @name)

                    (enum_item
                        name: (_) @name)

                    (impl_item
                        trait: (_)? @name
                        "for"? @name
                        type: (_) @name)

                    (trait_item
                        name: (_) @name)

                    (function_item
                        name: (_) @name
                        body: (block
                            "{" @keep
                            "}" @keep) @collapse)

                    (macro_definition
                        name: (_) @name)
                    ] @item
                )
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

        assert_eq!(
            summarize(&snapshot, Point::new(1, 4)..Point::new(1, 4)),
            indoc! {"
                struct X {
                    <|START|>a: usize,
                    b: usize,
                }

                impl X {

                    fn new() -> Self {}

                    pub fn a(&self, param: bool) -> usize {}

                    pub fn b(&self) -> usize {}
                }
            "}
        );

        assert_eq!(
            summarize(&snapshot, Point::new(8, 12)..Point::new(8, 14)),
            indoc! {"
                struct X {
                    a: usize,
                    b: usize,
                }

                impl X {

                    fn new() -> Self {
                        let <|START|a |END|>= 1;
                        let b = 2;
                        Self { a, b }
                    }

                    pub fn a(&self, param: bool) -> usize {}

                    pub fn b(&self) -> usize {}
                }
            "}
        );

        assert_eq!(
            summarize(&snapshot, Point::new(6, 0)..Point::new(6, 0)),
            indoc! {"
                struct X {
                    a: usize,
                    b: usize,
                }

                impl X {
                <|START|>
                    fn new() -> Self {}

                    pub fn a(&self, param: bool) -> usize {}

                    pub fn b(&self) -> usize {}
                }
            "}
        );

        assert_eq!(
            summarize(&snapshot, Point::new(21, 0)..Point::new(21, 0)),
            indoc! {"
                struct X {
                    a: usize,
                    b: usize,
                }

                impl X {

                    fn new() -> Self {}

                    pub fn a(&self, param: bool) -> usize {}

                    pub fn b(&self) -> usize {}
                }
                <|START|>"}
        );

        // Ensure nested functions get collapsed properly.
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
                    let a = 30;
                    fn nested() -> usize {
                        3
                    }
                    self.a + nested()
                }

                pub fn b(&self) -> usize {
                    self.b
                }
            }
        "};
        buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
        let snapshot = buffer.read(cx).snapshot();
        assert_eq!(
            summarize(&snapshot, Point::new(0, 0)..Point::new(0, 0)),
            indoc! {"
                <|START|>struct X {
                    a: usize,
                    b: usize,
                }

                impl X {

                    fn new() -> Self {}

                    pub fn a(&self, param: bool) -> usize {}

                    pub fn b(&self) -> usize {}
                }
            "}
        );
    }
}
