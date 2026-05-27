use std::ops::Range;

use language::{BufferSnapshot, Point};

/// Find the eval region enclosing `cursor_point` using the language's
/// eval.scm query. Returns the tightest `@eval` capture containing the cursor.
pub fn find_eval_at(buffer: &BufferSnapshot, cursor_point: Point) -> Option<Range<Point>> {
    let cursor_offset = buffer.point_to_offset(cursor_point);

    let mut syntax_matches = buffer.matches(0..buffer.len(), |grammar| {
        grammar.eval_config.as_ref().map(|c| &c.query)
    });

    let configs: Vec<_> = syntax_matches
        .grammars()
        .iter()
        .map(|grammar| grammar.eval_config.as_ref())
        .collect();

    let mut best: Option<Range<usize>> = None;

    while let Some(mat) = syntax_matches.peek() {
        if let Some(config) = &configs[mat.grammar_index] {
            for capture in mat.captures.iter() {
                if capture.index == config.eval_capture_ix {
                    let range = capture.node.byte_range();
                    if range.start <= cursor_offset && cursor_offset <= range.end {
                        let is_tighter =
                            best.as_ref().map(|b| range.len() < b.len()).unwrap_or(true);
                        if is_tighter {
                            best = Some(range);
                        }
                    }
                }
            }
        }
        syntax_matches.advance();
    }

    best.map(|r| {
        let start = buffer.offset_to_point(r.start);
        let end = buffer.offset_to_point(r.end);
        start..end
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext as _, TestAppContext};
    use language::{Buffer, Language, LanguageRegistry};
    use std::sync::Arc;

    fn rust_language_with_eval_query() -> Arc<Language> {
        let language = match Arc::try_unwrap(language::rust_lang()) {
            Ok(language) => language,
            Err(_) => panic!("rust_lang should be uniquely owned in this test"),
        };

        let language = language
            .with_eval_query("(function_item) @eval\n(block) @eval")
            .expect("rust eval query should parse");
        Arc::new(language)
    }

    fn bash_language_with_eval_query() -> Arc<Language> {
        let language = Language::new(
            language::LanguageConfig {
                name: "Bash".into(),
                line_comments: vec!["# ".into()],
                ..Default::default()
            },
            Some(tree_sitter_bash::LANGUAGE.into()),
        )
        .with_eval_query(include_str!("../../grammars/src/bash/eval.scm"))
        .expect("bash eval query should parse");
        Arc::new(language)
    }

    fn snapshot_for(
        text: &str,
        language: Arc<Language>,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut TestAppContext,
    ) -> BufferSnapshot {
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(text, cx);
            if let Some(language_registry) = language_registry {
                buffer.set_language_registry(language_registry);
            }
            buffer.set_language(Some(language), cx);
            buffer
        });
        cx.executor().run_until_parked();
        buffer.read_with(cx, |buffer, _| buffer.snapshot())
    }

    fn text_for_range(buffer: &BufferSnapshot, range: Range<Point>) -> String {
        buffer.text_for_range(range.start..range.end).collect()
    }

    #[gpui::test]
    fn find_eval_at_returns_tightest_capture(cx: &mut TestAppContext) {
        let snapshot = snapshot_for(
            "fn outer() {\n    if true {\n        println!(\"hi\");\n    }\n}\n",
            rust_language_with_eval_query(),
            None,
            cx,
        );

        let range = find_eval_at(&snapshot, Point::new(2, 10))
            .expect("expected cursor inside nested block to match an eval capture");

        assert_eq!(
            text_for_range(&snapshot, range),
            "{\n        println!(\"hi\");\n    }"
        );
    }

    #[gpui::test]
    fn find_eval_at_returns_injected_bash_command_in_markdown(cx: &mut TestAppContext) {
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor.clone()));
        let markdown_language = language::markdown_lang();
        registry.add(markdown_language.clone());
        registry.add(bash_language_with_eval_query());

        let snapshot = snapshot_for(
            "before\n\n```Bash\ncd ~/develop/zed\n\n# One-time setup:\n# git remote add upstream https://github.com/zed-industries/zed.git\n\ngit fetch upstream\ngit checkout main\n```\n\nafter\n",
            markdown_language,
            Some(registry),
            cx,
        );

        let range = find_eval_at(&snapshot, Point::new(3, 0))
            .expect("expected cursor inside bash command to match an eval capture");

        assert_eq!(text_for_range(&snapshot, range), "cd ~/develop/zed");
    }

    #[gpui::test]
    fn find_eval_at_ignores_injected_bash_comment_in_markdown(cx: &mut TestAppContext) {
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor.clone()));
        let markdown_language = language::markdown_lang();
        registry.add(markdown_language.clone());
        registry.add(bash_language_with_eval_query());

        let snapshot = snapshot_for(
            "before\n\n```Bash\ncd ~/develop/zed\n\n# One-time setup:\n# git remote add upstream https://github.com/zed-industries/zed.git\n\ngit fetch upstream\ngit checkout main\n```\n\nafter\n",
            markdown_language,
            Some(registry),
            cx,
        );

        assert!(find_eval_at(&snapshot, Point::new(5, 0)).is_none());
    }
}
