mod declaration;
mod declaration_scoring;
mod excerpt;
mod outline;
mod reference;
mod syntax_index;
mod text_similarity;

use gpui::{App, AppContext as _, Entity, Task};
use language::BufferSnapshot;
use text::{Point, ToOffset as _};

pub use declaration::*;
pub use declaration_scoring::*;
pub use excerpt::*;
pub use reference::*;
pub use syntax_index::*;

#[derive(Debug)]
pub struct EditPredictionContext {
    pub excerpt: EditPredictionExcerpt,
    pub excerpt_text: EditPredictionExcerptText,
    pub cursor_offset_in_excerpt: usize,
    pub snippets: Vec<ScoredSnippet>,
}

impl EditPredictionContext {
    pub fn gather_context_in_background(
        cursor_point: Point,
        buffer: BufferSnapshot,
        excerpt_options: EditPredictionExcerptOptions,
        syntax_index: Option<Entity<SyntaxIndex>>,
        cx: &mut App,
    ) -> Task<Option<Self>> {
        if let Some(syntax_index) = syntax_index {
            let index_state = syntax_index.read_with(cx, |index, _cx| index.state().clone());
            cx.background_spawn(async move {
                let index_state = index_state.lock().await;
                Self::gather_context(cursor_point, &buffer, &excerpt_options, Some(&index_state))
            })
        } else {
            cx.background_spawn(async move {
                Self::gather_context(cursor_point, &buffer, &excerpt_options, None)
            })
        }
    }

    pub fn gather_context(
        cursor_point: Point,
        buffer: &BufferSnapshot,
        excerpt_options: &EditPredictionExcerptOptions,
        index_state: Option<&SyntaxIndexState>,
    ) -> Option<Self> {
        let excerpt = EditPredictionExcerpt::select_from_buffer(
            cursor_point,
            buffer,
            excerpt_options,
            index_state,
        )?;
        let excerpt_text = excerpt.text(buffer);
        let cursor_offset_in_file = cursor_point.to_offset(buffer);
        // TODO fix this to not need saturating_sub
        let cursor_offset_in_excerpt = cursor_offset_in_file.saturating_sub(excerpt.range.start);

        let snippets = if let Some(index_state) = index_state {
            let references = references_in_excerpt(&excerpt, &excerpt_text, buffer);

            scored_snippets(
                &index_state,
                &excerpt,
                &excerpt_text,
                references,
                cursor_offset_in_file,
                buffer,
            )
        } else {
            vec![]
        };

        Some(Self {
            excerpt,
            excerpt_text,
            cursor_offset_in_excerpt,
            snippets,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use gpui::{Entity, TestAppContext};
    use indoc::indoc;
    use language::{Language, LanguageConfig, LanguageId, LanguageMatcher, tree_sitter_rust};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    use crate::{EditPredictionExcerptOptions, SyntaxIndex};

    #[gpui::test]
    async fn test_call_site(cx: &mut TestAppContext) {
        let (project, index, _rust_lang_id) = init_test(cx).await;

        let buffer = project
            .update(cx, |project, cx| {
                let project_path = project.find_project_path("c.rs", cx).unwrap();
                project.open_buffer(project_path, cx)
            })
            .await
            .unwrap();

        cx.run_until_parked();

        // first process_data call site
        let cursor_point = language::Point::new(8, 21);
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let context = cx
            .update(|cx| {
                EditPredictionContext::gather_context_in_background(
                    cursor_point,
                    buffer_snapshot,
                    EditPredictionExcerptOptions {
                        max_bytes: 60,
                        min_bytes: 10,
                        target_before_cursor_over_total_bytes: 0.5,
                    },
                    Some(index),
                    cx,
                )
            })
            .await
            .unwrap();

        let mut snippet_identifiers = context
            .snippets
            .iter()
            .map(|snippet| snippet.identifier.name.as_ref())
            .collect::<Vec<_>>();
        snippet_identifiers.sort();
        assert_eq!(snippet_identifiers, vec!["main", "process_data"]);
        drop(buffer);
    }

    async fn init_test(
        cx: &mut TestAppContext,
    ) -> (Entity<Project>, Entity<SyntaxIndex>, LanguageId) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a.rs": indoc! {r#"
                    fn main() {
                        let x = 1;
                        let y = 2;
                        let z = add(x, y);
                        println!("Result: {}", z);
                    }

                    fn add(a: i32, b: i32) -> i32 {
                        a + b
                    }
                "#},
                "b.rs": indoc! {"
                    pub struct Config {
                        pub name: String,
                        pub value: i32,
                    }

                    impl Config {
                        pub fn new(name: String, value: i32) -> Self {
                            Config { name, value }
                        }
                    }
                "},
                "c.rs": indoc! {r#"
                    use std::collections::HashMap;

                    fn main() {
                        let args: Vec<String> = std::env::args().collect();
                        let data: Vec<i32> = args[1..]
                            .iter()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        let result = process_data(data);
                        println!("{:?}", result);
                    }

                    fn process_data(data: Vec<i32>) -> HashMap<i32, usize> {
                        let mut counts = HashMap::new();
                        for value in data {
                            *counts.entry(value).or_insert(0) += 1;
                        }
                        counts
                    }

                    #[cfg(test)]
                    mod tests {
                        use super::*;

                        #[test]
                        fn test_process_data() {
                            let data = vec![1, 2, 2, 3];
                            let result = process_data(data);
                            assert_eq!(result.get(&2), Some(&2));
                        }
                    }
                "#}
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let lang = rust_lang();
        let lang_id = lang.id();
        language_registry.add(Arc::new(lang));

        let index = cx.new(|cx| SyntaxIndex::new(&project, cx));
        cx.run_until_parked();

        (project, index, lang_id)
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
        .with_highlights_query(include_str!("../../languages/src/rust/highlights.scm"))
        .unwrap()
        .with_outline_query(include_str!("../../languages/src/rust/outline.scm"))
        .unwrap()
    }
}
