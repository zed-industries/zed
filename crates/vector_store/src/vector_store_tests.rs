use std::sync::Arc;

use crate::{dot, embedding::EmbeddingProvider, VectorStore};
use anyhow::Result;
use async_trait::async_trait;
use gpui::{Task, TestAppContext};
use language::{Language, LanguageConfig, LanguageRegistry};
use project::{FakeFs, Project};
use rand::Rng;
use serde_json::json;
use unindent::Unindent;

#[gpui::test]
async fn test_vector_store(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/the-root",
        json!({
            "src": {
                "file1.rs": "
                    fn aaa() {
                        println!(\"aaaa!\");
                    }

                    fn zzzzzzzzz() {
                        println!(\"SLEEPING\");
                    }
                ".unindent(),
                "file2.rs": "
                    fn bbb() {
                        println!(\"bbbb!\");
                    }
                ".unindent(),
            }
        }),
    )
    .await;

    let languages = Arc::new(LanguageRegistry::new(Task::ready(())));
    let rust_language = Arc::new(
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".into()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_outline_query(
            r#"
            (function_item
                name: (identifier) @name
                body: (block)) @item
            "#,
        )
        .unwrap(),
    );
    languages.add(rust_language);

    let db_dir = tempdir::TempDir::new("vector-store").unwrap();
    let db_path = db_dir.path().join("db.sqlite");

    let store = cx.add_model(|_| {
        VectorStore::new(
            fs.clone(),
            db_path.to_string_lossy().to_string(),
            Arc::new(FakeEmbeddingProvider),
            languages,
        )
    });

    let project = Project::test(fs, ["/the-root".as_ref()], cx).await;
    let add_project = store.update(cx, |store, cx| store.add_project(project, cx));

    // TODO - remove
    cx.foreground()
        .advance_clock(std::time::Duration::from_secs(3));

    add_project.await.unwrap();

    let search_results = store
        .update(cx, |store, cx| store.search("aaaa".to_string(), 5, cx))
        .await
        .unwrap();

    assert_eq!(search_results[0].offset, 0);
    assert_eq!(search_results[0].name, "aaa");
}

#[test]
fn test_dot_product() {
    assert_eq!(dot(&[1., 0., 0., 0., 0.], &[0., 1., 0., 0., 0.]), 0.);
    assert_eq!(dot(&[2., 0., 0., 0., 0.], &[3., 1., 0., 0., 0.]), 6.);

    for _ in 0..100 {
        let mut rng = rand::thread_rng();
        let a: [f32; 32] = rng.gen();
        let b: [f32; 32] = rng.gen();
        assert_eq!(
            round_to_decimals(dot(&a, &b), 3),
            round_to_decimals(reference_dot(&a, &b), 3)
        );
    }

    fn round_to_decimals(n: f32, decimal_places: i32) -> f32 {
        let factor = (10.0 as f32).powi(decimal_places);
        (n * factor).round() / factor
    }

    fn reference_dot(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b.iter()).map(|(a, b)| a * b).sum()
    }
}

struct FakeEmbeddingProvider;

#[async_trait]
impl EmbeddingProvider for FakeEmbeddingProvider {
    async fn embed_batch(&self, spans: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        Ok(spans
            .iter()
            .map(|span| {
                let mut result = vec![1.0; 26];
                for letter in span.chars() {
                    let letter = letter.to_ascii_lowercase();
                    if letter as u32 >= 'a' as u32 {
                        let ix = (letter as u32) - ('a' as u32);
                        if ix < 26 {
                            result[ix as usize] += 1.0;
                        }
                    }
                }

                let norm = result.iter().map(|x| x * x).sum::<f32>().sqrt();
                for x in &mut result {
                    *x /= norm;
                }

                result
            })
            .collect())
    }
}
