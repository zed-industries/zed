mod chunking;
mod embedding;
mod embedding_index;
mod indexing;
mod project_index;
mod project_index_debug_view;
mod summary_backlog;
mod summary_index;
mod worktree_index;

use anyhow::{Context as _, Result};
use collections::HashMap;
use fs::Fs;
use gpui::{App, AppContext as _, AsyncApp, BorrowAppContext, Context, Entity, Global, WeakEntity};
use language::LineEnding;
use project::{Project, Worktree};
use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt as _;
use workspace::Workspace;

pub use embedding::*;
pub use project_index::{LoadedSearchResult, ProjectIndex, SearchResult, Status};
pub use project_index_debug_view::ProjectIndexDebugView;
pub use summary_index::FileSummary;

pub struct SemanticDb {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    db_connection: Option<heed::Env>,
    project_indices: HashMap<WeakEntity<Project>, Entity<ProjectIndex>>,
}

impl Global for SemanticDb {}

impl SemanticDb {
    pub async fn new(
        db_path: PathBuf,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let db_connection = cx
            .background_spawn(async move {
                std::fs::create_dir_all(&db_path)?;
                unsafe {
                    heed::EnvOpenOptions::new()
                        .map_size(1024 * 1024 * 1024)
                        .max_dbs(3000)
                        .open(db_path)
                }
            })
            .await
            .context("opening database connection")?;

        cx.update(|cx| {
            cx.observe_new(
                |workspace: &mut Workspace, _window, cx: &mut Context<Workspace>| {
                    let project = workspace.project().clone();

                    if cx.has_global::<SemanticDb>() {
                        cx.update_global::<SemanticDb, _>(|this, cx| {
                            this.create_project_index(project, cx);
                        })
                    } else {
                        log::info!("No SemanticDb, skipping project index")
                    }
                },
            )
            .detach();
        })
        .ok();

        Ok(SemanticDb {
            db_connection: Some(db_connection),
            embedding_provider,
            project_indices: HashMap::default(),
        })
    }

    pub async fn load_results(
        mut results: Vec<SearchResult>,
        fs: &Arc<dyn Fs>,
        cx: &AsyncApp,
    ) -> Result<Vec<LoadedSearchResult>> {
        let mut max_scores_by_path = HashMap::<_, (f32, usize)>::default();
        for result in &results {
            let (score, query_index) = max_scores_by_path
                .entry((result.worktree.clone(), result.path.clone()))
                .or_default();
            if result.score > *score {
                *score = result.score;
                *query_index = result.query_index;
            }
        }

        results.sort_by(|a, b| {
            let max_score_a = max_scores_by_path[&(a.worktree.clone(), a.path.clone())].0;
            let max_score_b = max_scores_by_path[&(b.worktree.clone(), b.path.clone())].0;
            max_score_b
                .partial_cmp(&max_score_a)
                .unwrap_or(Ordering::Equal)
                .then_with(|| a.worktree.entity_id().cmp(&b.worktree.entity_id()))
                .then_with(|| a.path.cmp(&b.path))
                .then_with(|| a.range.start.cmp(&b.range.start))
        });

        let mut last_loaded_file: Option<(Entity<Worktree>, Arc<Path>, PathBuf, String)> = None;
        let mut loaded_results = Vec::<LoadedSearchResult>::new();
        for result in results {
            let full_path;
            let file_content;
            if let Some(last_loaded_file) =
                last_loaded_file
                    .as_ref()
                    .filter(|(last_worktree, last_path, _, _)| {
                        last_worktree == &result.worktree && last_path == &result.path
                    })
            {
                full_path = last_loaded_file.2.clone();
                file_content = &last_loaded_file.3;
            } else {
                let output = result.worktree.read_with(cx, |worktree, _cx| {
                    let entry_abs_path = worktree.abs_path().join(&result.path);
                    let mut entry_full_path = PathBuf::from(worktree.root_name());
                    entry_full_path.push(&result.path);
                    let file_content = async {
                        let entry_abs_path = entry_abs_path;
                        fs.load(&entry_abs_path).await
                    };
                    (entry_full_path, file_content)
                })?;
                full_path = output.0;
                let Some(content) = output.1.await.log_err() else {
                    continue;
                };
                last_loaded_file = Some((
                    result.worktree.clone(),
                    result.path.clone(),
                    full_path.clone(),
                    content,
                ));
                file_content = &last_loaded_file.as_ref().unwrap().3;
            };

            let query_index = max_scores_by_path[&(result.worktree.clone(), result.path.clone())].1;

            let mut range_start = result.range.start.min(file_content.len());
            let mut range_end = result.range.end.min(file_content.len());
            while !file_content.is_char_boundary(range_start) {
                range_start += 1;
            }
            while !file_content.is_char_boundary(range_end) {
                range_end += 1;
            }

            let start_row = file_content[0..range_start].matches('\n').count() as u32;
            let mut end_row = file_content[0..range_end].matches('\n').count() as u32;
            let start_line_byte_offset = file_content[0..range_start]
                .rfind('\n')
                .map(|pos| pos + 1)
                .unwrap_or_default();
            let mut end_line_byte_offset = range_end;
            if file_content[..end_line_byte_offset].ends_with('\n') {
                end_row -= 1;
            } else {
                end_line_byte_offset = file_content[range_end..]
                    .find('\n')
                    .map(|pos| range_end + pos + 1)
                    .unwrap_or_else(|| file_content.len());
            }
            let mut excerpt_content =
                file_content[start_line_byte_offset..end_line_byte_offset].to_string();
            LineEnding::normalize(&mut excerpt_content);

            if let Some(prev_result) = loaded_results.last_mut() {
                if prev_result.full_path == full_path {
                    if *prev_result.row_range.end() + 1 == start_row {
                        prev_result.row_range = *prev_result.row_range.start()..=end_row;
                        prev_result.excerpt_content.push_str(&excerpt_content);
                        continue;
                    }
                }
            }

            loaded_results.push(LoadedSearchResult {
                path: result.path,
                full_path,
                excerpt_content,
                row_range: start_row..=end_row,
                query_index,
            });
        }

        for result in &mut loaded_results {
            while result.excerpt_content.ends_with("\n\n") {
                result.excerpt_content.pop();
                result.row_range =
                    *result.row_range.start()..=result.row_range.end().saturating_sub(1)
            }
        }

        Ok(loaded_results)
    }

    pub fn project_index(
        &mut self,
        project: Entity<Project>,
        _cx: &mut App,
    ) -> Option<Entity<ProjectIndex>> {
        self.project_indices.get(&project.downgrade()).cloned()
    }

    pub fn remaining_summaries(
        &self,
        project: &WeakEntity<Project>,
        cx: &mut App,
    ) -> Option<usize> {
        self.project_indices.get(project).map(|project_index| {
            project_index.update(cx, |project_index, cx| {
                project_index.remaining_summaries(cx)
            })
        })
    }

    pub fn create_project_index(
        &mut self,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Entity<ProjectIndex> {
        let project_index = cx.new(|cx| {
            ProjectIndex::new(
                project.clone(),
                self.db_connection.clone().unwrap(),
                self.embedding_provider.clone(),
                cx,
            )
        });

        let project_weak = project.downgrade();
        self.project_indices
            .insert(project_weak.clone(), project_index.clone());

        cx.observe_release(&project, move |_, cx| {
            if cx.has_global::<SemanticDb>() {
                cx.update_global::<SemanticDb, _>(|this, _| {
                    this.project_indices.remove(&project_weak);
                })
            }
        })
        .detach();

        project_index
    }
}

impl Drop for SemanticDb {
    fn drop(&mut self) {
        self.db_connection.take().unwrap().prepare_for_closing();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use chunking::Chunk;
    use embedding_index::{ChunkedFile, EmbeddingIndex};
    use feature_flags::FeatureFlagAppExt;
    use fs::FakeFs;
    use futures::{FutureExt, future::BoxFuture};
    use gpui::TestAppContext;
    use indexing::IndexingEntrySet;
    use language::language_settings::AllLanguageSettings;
    use project::{Project, ProjectEntryId};
    use serde_json::json;
    use settings::SettingsStore;
    use smol::channel;
    use std::{future, path::Path, sync::Arc};
    use util::separator;

    fn init_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();

        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            language::init(cx);
            cx.update_flags(false, vec![]);
            Project::init_settings(cx);
            SettingsStore::update(cx, |store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
            });
        });
    }

    pub struct TestEmbeddingProvider {
        batch_size: usize,
        compute_embedding: Box<dyn Fn(&str) -> Result<Embedding> + Send + Sync>,
    }

    impl TestEmbeddingProvider {
        pub fn new(
            batch_size: usize,
            compute_embedding: impl 'static + Fn(&str) -> Result<Embedding> + Send + Sync,
        ) -> Self {
            Self {
                batch_size,
                compute_embedding: Box::new(compute_embedding),
            }
        }
    }

    impl EmbeddingProvider for TestEmbeddingProvider {
        fn embed<'a>(
            &'a self,
            texts: &'a [TextToEmbed<'a>],
        ) -> BoxFuture<'a, Result<Vec<Embedding>>> {
            let embeddings = texts
                .iter()
                .map(|to_embed| (self.compute_embedding)(to_embed.text))
                .collect();
            future::ready(embeddings).boxed()
        }

        fn batch_size(&self) -> usize {
            self.batch_size
        }
    }

    #[gpui::test]
    async fn test_search(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        init_test(cx);

        cx.update(|cx| {
            // This functionality is staff-flagged.
            cx.update_flags(true, vec![]);
        });

        let temp_dir = tempfile::tempdir().unwrap();

        let mut semantic_index = SemanticDb::new(
            temp_dir.path().into(),
            Arc::new(TestEmbeddingProvider::new(16, |text| {
                let mut embedding = vec![0f32; 2];
                // if the text contains garbage, give it a 1 in the first dimension
                if text.contains("garbage in") {
                    embedding[0] = 0.9;
                } else {
                    embedding[0] = -0.9;
                }

                if text.contains("garbage out") {
                    embedding[1] = 0.9;
                } else {
                    embedding[1] = -0.9;
                }

                Ok(Embedding::new(embedding))
            })),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let fs = FakeFs::new(cx.executor());
        let project_path = Path::new("/fake_project");

        fs.insert_tree(
            project_path,
            json!({
                "fixture": {
                    "main.rs": include_str!("../fixture/main.rs"),
                    "needle.md": include_str!("../fixture/needle.md"),
                }
            }),
        )
        .await;

        let project = Project::test(fs, [project_path], cx).await;

        let project_index = cx.update(|cx| {
            let language_registry = project.read(cx).languages().clone();
            let node_runtime = project.read(cx).node_runtime().unwrap().clone();
            languages::init(language_registry, node_runtime, cx);
            semantic_index.create_project_index(project.clone(), cx)
        });

        cx.run_until_parked();
        while cx
            .update(|cx| semantic_index.remaining_summaries(&project.downgrade(), cx))
            .unwrap()
            > 0
        {
            cx.run_until_parked();
        }

        let results = cx
            .update(|cx| {
                let project_index = project_index.read(cx);
                let query = "garbage in, garbage out";
                project_index.search(vec![query.into()], 4, cx)
            })
            .await
            .unwrap();

        assert!(
            results.len() > 1,
            "should have found some results, but only found {:?}",
            results
        );

        for result in &results {
            println!("result: {:?}", result.path);
            println!("score: {:?}", result.score);
        }

        // Find result that is greater than 0.5
        let search_result = results.iter().find(|result| result.score > 0.9).unwrap();

        assert_eq!(
            search_result.path.to_string_lossy(),
            separator!("fixture/needle.md")
        );

        let content = cx
            .update(|cx| {
                let worktree = search_result.worktree.read(cx);
                let entry_abs_path = worktree.abs_path().join(&search_result.path);
                let fs = project.read(cx).fs().clone();
                cx.background_spawn(async move { fs.load(&entry_abs_path).await.unwrap() })
            })
            .await;

        let range = search_result.range.clone();
        let content = content[range.clone()].to_owned();

        assert!(content.contains("garbage in, garbage out"));
    }

    #[gpui::test]
    async fn test_embed_files(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let provider = Arc::new(TestEmbeddingProvider::new(3, |text| {
            if text.contains('g') {
                Err(anyhow!("cannot embed text containing a 'g' character"))
            } else {
                Ok(Embedding::new(
                    ('a'..='z')
                        .map(|char| text.chars().filter(|c| *c == char).count() as f32)
                        .collect(),
                ))
            }
        }));

        let (indexing_progress_tx, _) = channel::unbounded();
        let indexing_entries = Arc::new(IndexingEntrySet::new(indexing_progress_tx));

        let (chunked_files_tx, chunked_files_rx) = channel::unbounded::<ChunkedFile>();
        chunked_files_tx
            .send_blocking(ChunkedFile {
                path: Path::new("test1.md").into(),
                mtime: None,
                handle: indexing_entries.insert(ProjectEntryId::from_proto(0)),
                text: "abcdefghijklmnop".to_string(),
                chunks: [0..4, 4..8, 8..12, 12..16]
                    .into_iter()
                    .map(|range| Chunk {
                        range,
                        digest: Default::default(),
                    })
                    .collect(),
            })
            .unwrap();
        chunked_files_tx
            .send_blocking(ChunkedFile {
                path: Path::new("test2.md").into(),
                mtime: None,
                handle: indexing_entries.insert(ProjectEntryId::from_proto(1)),
                text: "qrstuvwxyz".to_string(),
                chunks: [0..4, 4..8, 8..10]
                    .into_iter()
                    .map(|range| Chunk {
                        range,
                        digest: Default::default(),
                    })
                    .collect(),
            })
            .unwrap();
        chunked_files_tx.close();

        let embed_files_task =
            cx.update(|cx| EmbeddingIndex::embed_files(provider.clone(), chunked_files_rx, cx));
        embed_files_task.task.await.unwrap();

        let embedded_files_rx = embed_files_task.files;
        let mut embedded_files = Vec::new();
        while let Ok((embedded_file, _)) = embedded_files_rx.recv().await {
            embedded_files.push(embedded_file);
        }

        assert_eq!(embedded_files.len(), 1);
        assert_eq!(embedded_files[0].path.as_ref(), Path::new("test2.md"));
        assert_eq!(
            embedded_files[0]
                .chunks
                .iter()
                .map(|embedded_chunk| { embedded_chunk.embedding.clone() })
                .collect::<Vec<Embedding>>(),
            vec![
                (provider.compute_embedding)("qrst").unwrap(),
                (provider.compute_embedding)("uvwx").unwrap(),
                (provider.compute_embedding)("yz").unwrap(),
            ],
        );
    }

    #[gpui::test]
    async fn test_load_search_results(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project_path = Path::new("/fake_project");

        let file1_content = "one\ntwo\nthree\nfour\nfive\n";
        let file2_content = "aaa\nbbb\nccc\nddd\neee\n";

        fs.insert_tree(
            project_path,
            json!({
                "file1.txt": file1_content,
                "file2.txt": file2_content,
            }),
        )
        .await;

        let fs = fs as Arc<dyn Fs>;
        let project = Project::test(fs.clone(), [project_path], cx).await;
        let worktree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());

        // chunk that is already newline-aligned
        let search_results = vec![SearchResult {
            worktree: worktree.clone(),
            path: Path::new("file1.txt").into(),
            range: 0..file1_content.find("four").unwrap(),
            score: 0.5,
            query_index: 0,
        }];
        assert_eq!(
            SemanticDb::load_results(search_results, &fs, &cx.to_async())
                .await
                .unwrap(),
            &[LoadedSearchResult {
                path: Path::new("file1.txt").into(),
                full_path: "fake_project/file1.txt".into(),
                excerpt_content: "one\ntwo\nthree\n".into(),
                row_range: 0..=2,
                query_index: 0,
            }]
        );

        // chunk that is *not* newline-aligned
        let search_results = vec![SearchResult {
            worktree: worktree.clone(),
            path: Path::new("file1.txt").into(),
            range: file1_content.find("two").unwrap() + 1..file1_content.find("four").unwrap() + 2,
            score: 0.5,
            query_index: 0,
        }];
        assert_eq!(
            SemanticDb::load_results(search_results, &fs, &cx.to_async())
                .await
                .unwrap(),
            &[LoadedSearchResult {
                path: Path::new("file1.txt").into(),
                full_path: "fake_project/file1.txt".into(),
                excerpt_content: "two\nthree\nfour\n".into(),
                row_range: 1..=3,
                query_index: 0,
            }]
        );

        // chunks that are adjacent

        let search_results = vec![
            SearchResult {
                worktree: worktree.clone(),
                path: Path::new("file1.txt").into(),
                range: file1_content.find("two").unwrap()..file1_content.len(),
                score: 0.6,
                query_index: 0,
            },
            SearchResult {
                worktree: worktree.clone(),
                path: Path::new("file1.txt").into(),
                range: 0..file1_content.find("two").unwrap(),
                score: 0.5,
                query_index: 1,
            },
            SearchResult {
                worktree: worktree.clone(),
                path: Path::new("file2.txt").into(),
                range: 0..file2_content.len(),
                score: 0.8,
                query_index: 1,
            },
        ];
        assert_eq!(
            SemanticDb::load_results(search_results, &fs, &cx.to_async())
                .await
                .unwrap(),
            &[
                LoadedSearchResult {
                    path: Path::new("file2.txt").into(),
                    full_path: "fake_project/file2.txt".into(),
                    excerpt_content: file2_content.into(),
                    row_range: 0..=4,
                    query_index: 1,
                },
                LoadedSearchResult {
                    path: Path::new("file1.txt").into(),
                    full_path: "fake_project/file1.txt".into(),
                    excerpt_content: file1_content.into(),
                    row_range: 0..=4,
                    query_index: 0,
                }
            ]
        );
    }
}
