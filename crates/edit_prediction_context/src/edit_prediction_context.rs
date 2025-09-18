mod declaration;
mod declaration_scoring;
mod excerpt;
mod outline;
mod reference;
mod syntax_index;
mod text_similarity;

use cloud_llm_client::predict_edits_v3::{self, Signature};
use collections::HashMap;
pub use declaration::{BufferDeclaration, Declaration, FileDeclaration, Identifier};
pub use declaration_scoring::SnippetStyle;
pub use excerpt::{EditPredictionExcerpt, EditPredictionExcerptOptions, EditPredictionExcerptText};

use gpui::{App, AppContext as _, Entity, Task};
use language::BufferSnapshot;
pub use reference::references_in_excerpt;
pub use syntax_index::SyntaxIndex;
use text::{Point, ToOffset as _};

use crate::{
    declaration::DeclarationId,
    declaration_scoring::{ScoredSnippet, scored_snippets},
    syntax_index::SyntaxIndexState,
};

#[derive(Debug)]
pub struct EditPredictionContext {
    pub excerpt: EditPredictionExcerpt,
    pub excerpt_text: EditPredictionExcerptText,
    pub snippets: Vec<ScoredSnippet>,
}

impl EditPredictionContext {
    pub fn gather(
        cursor_point: Point,
        buffer: BufferSnapshot,
        excerpt_options: EditPredictionExcerptOptions,
        syntax_index: Entity<SyntaxIndex>,
        cx: &mut App,
    ) -> Task<Option<Self>> {
        let index_state = syntax_index.read_with(cx, |index, _cx| index.state().clone());
        cx.background_spawn(async move {
            let index_state = index_state.lock().await;
            Self::gather_context(cursor_point, buffer, excerpt_options, &index_state)
        })
    }

    fn gather_context(
        cursor_point: Point,
        buffer: BufferSnapshot,
        excerpt_options: EditPredictionExcerptOptions,
        index_state: &SyntaxIndexState,
    ) -> Option<Self> {
        let excerpt = EditPredictionExcerpt::select_from_buffer(
            cursor_point,
            &buffer,
            &excerpt_options,
            Some(index_state),
        )?;
        let excerpt_text = excerpt.text(&buffer);
        let references = references_in_excerpt(&excerpt, &excerpt_text, &buffer);
        let cursor_offset = cursor_point.to_offset(&buffer);

        let snippets = scored_snippets(
            &index_state,
            &excerpt,
            &excerpt_text,
            references,
            cursor_offset,
            &buffer,
        );

        Some(Self {
            excerpt,
            excerpt_text,
            snippets,
        })
    }

    pub fn cloud_request(
        cursor_point: Point,
        buffer: BufferSnapshot,
        excerpt_options: EditPredictionExcerptOptions,
        syntax_index: Entity<SyntaxIndex>,
        cx: &mut App,
    ) -> Task<Option<predict_edits_v3::Body>> {
        let index_state = syntax_index.read_with(cx, |index, _cx| index.state().clone());
        cx.background_spawn(async move {
            let index_state = index_state.lock().await;
            Self::gather_context(cursor_point, buffer, excerpt_options, &index_state)
                .map(|context| context.into_cloud_request(&index_state))
        })
    }

    pub fn into_cloud_request(self, index: &SyntaxIndexState) -> predict_edits_v3::Body {
        let mut signatures = Vec::new();
        let mut declaration_to_signature_index = HashMap::default();
        let mut referenced_declarations = Vec::new();
        let excerpt_parent = self
            .excerpt
            .parent_declarations
            .last()
            .and_then(|(parent, _)| {
                add_signature(
                    *parent,
                    &mut declaration_to_signature_index,
                    &mut signatures,
                    index,
                )
            });
        for snippet in self.snippets {
            let parent_index = snippet.declaration.parent().and_then(|parent| {
                add_signature(
                    parent,
                    &mut declaration_to_signature_index,
                    &mut signatures,
                    index,
                )
            });
            let (text, text_is_truncated) = snippet.declaration.item_text();
            referenced_declarations.push(predict_edits_v3::ReferencedDeclaration {
                text: text.into(),
                text_is_truncated,
                signature_range: snippet.declaration.signature_range_in_item_text(),
                parent_index,
                score_components: snippet.score_components,
                signature_score: snippet.scores.signature,
                declaration_score: snippet.scores.declaration,
            });
        }
        predict_edits_v3::Body {
            excerpt: self.excerpt_text.body,
            referenced_declarations,
            signatures,
            excerpt_parent,
            // todo!
            events: vec![],
            can_collect_data: false,
            diagnostic_groups: None,
            git_info: None,
        }
    }
}

fn add_signature(
    declaration_id: DeclarationId,
    declaration_to_signature_index: &mut HashMap<DeclarationId, usize>,
    signatures: &mut Vec<Signature>,
    index: &SyntaxIndexState,
) -> Option<usize> {
    if let Some(signature_index) = declaration_to_signature_index.get(&declaration_id) {
        return Some(*signature_index);
    }
    let Some(parent_declaration) = index.declaration(declaration_id) else {
        log::error!("bug: missing parent declaration");
        return None;
    };
    let parent_index = parent_declaration.parent().and_then(|parent| {
        add_signature(parent, declaration_to_signature_index, signatures, index)
    });
    let (text, text_is_truncated) = parent_declaration.signature_text();
    let signature_index = signatures.len();
    signatures.push(Signature {
        text: text.into(),
        text_is_truncated,
        parent_index,
    });
    declaration_to_signature_index.insert(declaration_id, signature_index);
    Some(signature_index)
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
                EditPredictionContext::gather(
                    cursor_point,
                    buffer_snapshot,
                    EditPredictionExcerptOptions {
                        max_bytes: 60,
                        min_bytes: 10,
                        target_before_cursor_over_total_bytes: 0.5,
                    },
                    index,
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
