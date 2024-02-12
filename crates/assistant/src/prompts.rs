use ai::models::LanguageModel;
use ai::prompts::base::{PromptArguments, PromptChain, PromptPriority, PromptTemplate};
use ai::prompts::file_context::FileContext;
use ai::prompts::generate::GenerateInlineContent;
use ai::prompts::preamble::EngineerPreamble;
use ai::prompts::repository_context::{PromptCodeSnippet, RepositoryContext};
use ai::providers::open_ai::OpenAiLanguageModel;
use language::{BufferSnapshot, OffsetRangeExt, ToOffset};
use std::cmp::{self, Reverse};
use std::ops::Range;
use std::sync::Arc;

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
                summary.push_str("<|S|");
                if selected_range.end == selected_range.start {
                    summary.push_str(">");
                } else {
                    summary.extend(buffer.text_for_range(selected_range.clone()));
                    summary.push_str("|E|>");
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
        summary.push_str("<|S|");
        if selected_range.end == selected_range.start {
            summary.push_str(">");
        } else {
            summary.extend(buffer.text_for_range(selected_range.clone()));
            summary.push_str("|E|>");
        }
        offset = selected_range.end;
    }

    summary.extend(buffer.text_for_range(offset..buffer.len()));
    summary
}

pub fn generate_content_prompt(
    user_prompt: String,
    language_name: Option<&str>,
    buffer: BufferSnapshot,
    range: Range<usize>,
    search_results: Vec<PromptCodeSnippet>,
    model: &str,
    project_name: Option<String>,
) -> anyhow::Result<String> {
    // Using new Prompt Templates
    let openai_model: Arc<dyn LanguageModel> = Arc::new(OpenAiLanguageModel::load(model));
    let lang_name = if let Some(language_name) = language_name {
        Some(language_name.to_string())
    } else {
        None
    };

    let args = PromptArguments {
        model: openai_model,
        language_name: lang_name.clone(),
        project_name,
        snippets: search_results.clone(),
        reserved_tokens: 1000,
        buffer: Some(buffer),
        selected_range: Some(range),
        user_prompt: Some(user_prompt.clone()),
    };

    let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
        (PromptPriority::Mandatory, Box::new(EngineerPreamble {})),
        (
            PromptPriority::Ordered { order: 1 },
            Box::new(RepositoryContext {}),
        ),
        (
            PromptPriority::Ordered { order: 0 },
            Box::new(FileContext {}),
        ),
        (
            PromptPriority::Mandatory,
            Box::new(GenerateInlineContent {}),
        ),
    ];
    let chain = PromptChain::new(args, templates);
    let (prompt, _) = chain.generate(true)?;

    anyhow::Ok(prompt)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use gpui::{AppContext, Context};
    use indoc::indoc;
    use language::{
        language_settings, tree_sitter_rust, Buffer, BufferId, Language, LanguageConfig,
        LanguageMatcher, Point,
    };
    use settings::SettingsStore;
    use std::sync::Arc;

    pub(crate) fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
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
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
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
        let buffer = cx.new_model(|cx| {
            Buffer::new(0, BufferId::new(1).unwrap(), text).with_language(Arc::new(rust_lang()), cx)
        });
        let snapshot = buffer.read(cx).snapshot();

        assert_eq!(
            summarize(&snapshot, Point::new(1, 4)..Point::new(1, 4)),
            indoc! {"
                struct X {
                    <|S|>a: usize,
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
                        let <|S|a |E|>= 1;
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
                <|S|>
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
                <|S|>"}
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
                <|S|>struct X {
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
