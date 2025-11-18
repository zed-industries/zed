mod declaration;
mod declaration_scoring;
mod excerpt;
mod imports;
mod outline;
mod reference;
mod syntax_index;
pub mod text_similarity;

use std::{path::Path, sync::Arc};

use cloud_llm_client::predict_edits_v3;
use collections::HashMap;
use gpui::{App, AppContext as _, Entity, Task};
use language::BufferSnapshot;
use text::{Point, ToOffset as _};

pub use declaration::*;
pub use declaration_scoring::*;
pub use excerpt::*;
pub use imports::*;
pub use reference::*;
pub use syntax_index::*;

pub use predict_edits_v3::Line;

#[derive(Clone, Debug, PartialEq)]
pub struct EditPredictionContextOptions {
    pub use_imports: bool,
    pub excerpt: EditPredictionExcerptOptions,
    pub score: EditPredictionScoreOptions,
    pub max_retrieved_declarations: u8,
}

#[derive(Clone, Debug)]
pub struct EditPredictionContext {
    pub excerpt: EditPredictionExcerpt,
    pub excerpt_text: EditPredictionExcerptText,
    pub cursor_point: Point,
    pub declarations: Vec<ScoredDeclaration>,
}

impl EditPredictionContext {
    pub fn gather_context_in_background(
        cursor_point: Point,
        buffer: BufferSnapshot,
        options: EditPredictionContextOptions,
        syntax_index: Option<Entity<SyntaxIndex>>,
        cx: &mut App,
    ) -> Task<Option<Self>> {
        let parent_abs_path = project::File::from_dyn(buffer.file()).and_then(|f| {
            let mut path = f.worktree.read(cx).absolutize(&f.path);
            if path.pop() { Some(path) } else { None }
        });

        if let Some(syntax_index) = syntax_index {
            let index_state =
                syntax_index.read_with(cx, |index, _cx| Arc::downgrade(index.state()));
            cx.background_spawn(async move {
                let parent_abs_path = parent_abs_path.as_deref();
                let index_state = index_state.upgrade()?;
                let index_state = index_state.lock().await;
                Self::gather_context(
                    cursor_point,
                    &buffer,
                    parent_abs_path,
                    &options,
                    Some(&index_state),
                )
            })
        } else {
            cx.background_spawn(async move {
                let parent_abs_path = parent_abs_path.as_deref();
                Self::gather_context(cursor_point, &buffer, parent_abs_path, &options, None)
            })
        }
    }

    pub fn gather_context(
        cursor_point: Point,
        buffer: &BufferSnapshot,
        parent_abs_path: Option<&Path>,
        options: &EditPredictionContextOptions,
        index_state: Option<&SyntaxIndexState>,
    ) -> Option<Self> {
        let imports = if options.use_imports {
            Imports::gather(&buffer, parent_abs_path)
        } else {
            Imports::default()
        };
        Self::gather_context_with_references_fn(
            cursor_point,
            buffer,
            &imports,
            options,
            index_state,
            references_in_excerpt,
        )
    }

    pub fn gather_context_with_references_fn(
        cursor_point: Point,
        buffer: &BufferSnapshot,
        imports: &Imports,
        options: &EditPredictionContextOptions,
        index_state: Option<&SyntaxIndexState>,
        get_references: impl FnOnce(
            &EditPredictionExcerpt,
            &EditPredictionExcerptText,
            &BufferSnapshot,
        ) -> HashMap<Identifier, Vec<Reference>>,
    ) -> Option<Self> {
        let excerpt = EditPredictionExcerpt::select_from_buffer(
            cursor_point,
            buffer,
            &options.excerpt,
            index_state,
        )?;
        let excerpt_text = excerpt.text(buffer);

        let declarations = if options.max_retrieved_declarations > 0
            && let Some(index_state) = index_state
        {
            let excerpt_occurrences =
                text_similarity::Occurrences::within_string(&excerpt_text.body);

            let adjacent_start = Point::new(cursor_point.row.saturating_sub(2), 0);
            let adjacent_end = Point::new(cursor_point.row + 1, 0);
            let adjacent_occurrences = text_similarity::Occurrences::within_string(
                &buffer
                    .text_for_range(adjacent_start..adjacent_end)
                    .collect::<String>(),
            );

            let cursor_offset_in_file = cursor_point.to_offset(buffer);

            let references = get_references(&excerpt, &excerpt_text, buffer);

            let mut declarations = scored_declarations(
                &options.score,
                &index_state,
                &excerpt,
                &excerpt_occurrences,
                &adjacent_occurrences,
                &imports,
                references,
                cursor_offset_in_file,
                buffer,
            );
            // TODO [zeta2] if we need this when we ship, we should probably do it in a smarter way
            declarations.truncate(options.max_retrieved_declarations as usize);
            declarations
        } else {
            vec![]
        };

        Some(Self {
            excerpt,
            excerpt_text,
            cursor_point,
            declarations,
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
                    EditPredictionContextOptions {
                        use_imports: true,
                        excerpt: EditPredictionExcerptOptions {
                            max_bytes: 60,
                            min_bytes: 10,
                            target_before_cursor_over_total_bytes: 0.5,
                        },
                        score: EditPredictionScoreOptions {
                            omit_excerpt_overlaps: true,
                        },
                        max_retrieved_declarations: u8::MAX,
                    },
                    Some(index.clone()),
                    cx,
                )
            })
            .await
            .unwrap();

        let mut snippet_identifiers = context
            .declarations
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

        let file_indexing_parallelism = 2;
        let index = cx.new(|cx| SyntaxIndex::new(&project, file_indexing_parallelism, cx));
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
