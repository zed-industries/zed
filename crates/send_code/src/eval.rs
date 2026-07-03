use std::ops::Range;

use language::{BufferSnapshot, Point};

/// Find the eval region enclosing `cursor_point` using the language's
/// eval.scm query. Prefers the least-indented enclosing capture, then the
/// tightest capture at that indentation.
pub fn find_eval_at(buffer: &BufferSnapshot, cursor_point: Point) -> Option<Range<Point>> {
    let cursor_offset = buffer.point_to_offset(cursor_point);

    let mut syntax_matches = buffer.matches(0..buffer.len(), |grammar| {
        grammar.eval_config.as_ref().map(|config| &config.query)
    });

    let configs: Vec<_> = syntax_matches
        .grammars()
        .iter()
        .map(|grammar| grammar.eval_config.as_ref())
        .collect();

    let mut best: Option<(Range<usize>, Point)> = None;

    while let Some(mat) = syntax_matches.peek() {
        if let Some(config) = &configs[mat.grammar_index] {
            for capture in mat.captures.iter() {
                if capture.index == config.eval_capture_ix {
                    let range = capture.node.byte_range();
                    if range.start <= cursor_offset && cursor_offset <= range.end {
                        let start = buffer.offset_to_point(range.start);
                        let is_tighter = best.as_ref().is_none_or(|(best_range, best_start)| {
                            start.column < best_start.column
                                || (start.column == best_start.column
                                    && range.len() < best_range.len())
                        });
                        if is_tighter {
                            best = Some((range, start));
                        }
                    }
                }
            }
        }
        syntax_matches.advance();
    }

    best.map(|(range, start)| {
        let end = buffer.offset_to_point(range.end);
        start..end
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext as _, TestAppContext};
    use language::{
        Buffer, Language, LanguageConfig, LanguageMatcher, LanguageQueries, LanguageRegistry,
        LoadedLanguage,
    };
    use std::borrow::Cow;
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

    fn python_language_with_eval_query() -> Arc<Language> {
        let language = Language::new(
            language::LanguageConfig {
                name: "Python".into(),
                line_comments: vec!["# ".into()],
                ..Default::default()
            },
            Some(tree_sitter_python::LANGUAGE.into()),
        )
        .with_eval_query(include_str!("../../grammars/src/python/eval.scm"))
        .expect("python eval query should parse");
        Arc::new(language)
    }

    fn r_language_with_eval_query() -> Arc<Language> {
        let language = Language::new(
            language::LanguageConfig {
                name: "R".into(),
                line_comments: vec!["# ".into()],
                ..Default::default()
            },
            Some(tree_sitter_r::LANGUAGE.into()),
        )
        .with_highlights_query(include_str!("../../grammars/src/r/highlights.scm"))
        .expect("r highlights query should parse")
        .with_eval_query(include_str!("../../grammars/src/r/eval.scm"))
        .expect("r eval query should parse");
        Arc::new(language)
    }

    fn register_available_r_language(registry: &LanguageRegistry) {
        registry.register_native_grammars([("r", tree_sitter_r::LANGUAGE)]);

        let config = LanguageConfig {
            name: "R".into(),
            code_fence_block_name: Some("r".into()),
            grammar: Some("r".into()),
            matcher: LanguageMatcher {
                path_suffixes: vec!["r".to_string(), "R".to_string()],
                ..Default::default()
            },
            line_comments: vec!["# ".into()],
            ..Default::default()
        };

        registry.register_language(
            config.name.clone(),
            config.grammar.clone(),
            config.matcher.clone(),
            false,
            None,
            Arc::new(move || {
                Ok(LoadedLanguage {
                    config: config.clone(),
                    queries: LanguageQueries {
                        highlights: Some(Cow::Borrowed(include_str!(
                            "../../grammars/src/r/highlights.scm"
                        ))),
                        eval: Some(Cow::Borrowed(include_str!("../../grammars/src/r/eval.scm"))),
                        ..Default::default()
                    },
                    context_provider: None,
                    toolchain_provider: None,
                    manifest_name: None,
                })
            }),
        );
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
    fn find_eval_at_prefers_least_indented_capture(cx: &mut TestAppContext) {
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
            "fn outer() {\n    if true {\n        println!(\"hi\");\n    }\n}"
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

    #[gpui::test]
    fn find_eval_at_returns_python_class_when_cursor_is_inside_method(cx: &mut TestAppContext) {
        let snapshot = snapshot_for(
            "class Counter:\n    def __init__(self):\n        self.value = 0\n\n    def increment(self):\n        self.value += 1\n        return self.value\n\ncounter = Counter()\n",
            python_language_with_eval_query(),
            None,
            cx,
        );

        let range = find_eval_at(&snapshot, Point::new(4, 8))
            .expect("expected cursor inside method to match the enclosing class");

        assert_eq!(
            text_for_range(&snapshot, range),
            "class Counter:\n    def __init__(self):\n        self.value = 0\n\n    def increment(self):\n        self.value += 1\n        return self.value"
        );
    }

    #[gpui::test]
    fn find_eval_at_returns_injected_r_expression_in_markdown(cx: &mut TestAppContext) {
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor.clone()));
        let markdown_language = language::markdown_lang();
        registry.add(markdown_language.clone());
        registry.add(r_language_with_eval_query());

        let snapshot = snapshot_for(
            "before\n\n```r\nx <- 1:5\n\n# This comment should no-op.\nmean(x)\n\ndf\n```\n\nafter\n",
            markdown_language,
            Some(registry),
            cx,
        );

        let range = find_eval_at(&snapshot, Point::new(3, 0))
            .expect("expected cursor inside r assignment to match an eval capture");

        assert_eq!(text_for_range(&snapshot, range), "x <- 1:5");
        assert!(find_eval_at(&snapshot, Point::new(5, 0)).is_none());

        let range = find_eval_at(&snapshot, Point::new(8, 0))
            .expect("expected top-level r identifier to match an eval capture");

        assert_eq!(text_for_range(&snapshot, range), "df");
    }

    #[gpui::test]
    fn find_eval_at_returns_registered_r_expression_in_markdown(cx: &mut TestAppContext) {
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor.clone()));
        let markdown_language = language::markdown_lang();
        registry.add(markdown_language.clone());
        register_available_r_language(&registry);

        let _load_r = registry.language_for_name("R");
        cx.executor().run_until_parked();

        let snapshot = snapshot_for(
            "before\n\n```r\nx <- 1:5\n\n# This comment should no-op.\nmean(x)\n\ndf\n```\n\nafter\n",
            markdown_language,
            Some(registry),
            cx,
        );

        let range = find_eval_at(&snapshot, Point::new(3, 0))
            .expect("expected cursor inside registered r language to match an eval capture");

        assert_eq!(text_for_range(&snapshot, range), "x <- 1:5");
        assert!(find_eval_at(&snapshot, Point::new(5, 0)).is_none());

        let range = find_eval_at(&snapshot, Point::new(8, 0))
            .expect("expected top-level registered r identifier to match an eval capture");

        assert_eq!(text_for_range(&snapshot, range), "df");
    }
}
