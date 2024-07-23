mod chunking;
mod embedding;
mod embedding_index;
mod project_index;
mod project_index_debug_view;
mod summary_index;
mod worktree_index;

use anyhow::{Context as _, Result};
use collections::HashMap;
pub use embedding::*;
use gpui::{AppContext, AsyncAppContext, BorrowAppContext, Context, Global, Model, WeakModel};
use project::Project;
use project_index::ProjectIndex;
use std::{path::PathBuf, sync::Arc};

pub use project_index_debug_view::ProjectIndexDebugView;

pub struct SemanticIndex {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    db_connection: heed::Env,
    project_indices: HashMap<WeakModel<Project>, Model<ProjectIndex>>,
}

impl Global for SemanticIndex {}

impl SemanticIndex {
    pub async fn new(
        db_path: PathBuf,
        embedding_provider: Arc<dyn EmbeddingProvider>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        let db_connection = cx
            .background_executor()
            .spawn(async move {
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

        Ok(SemanticIndex {
            db_connection,
            embedding_provider,
            project_indices: HashMap::default(),
        })
    }

    pub fn project_index(
        &mut self,
        project: Model<Project>,
        cx: &mut AppContext,
    ) -> Model<ProjectIndex> {
        let project_weak = project.downgrade();
        project.update(cx, move |_, cx| {
            cx.on_release(move |_, cx| {
                if cx.has_global::<SemanticIndex>() {
                    cx.update_global::<SemanticIndex, _>(|this, _| {
                        this.project_indices.remove(&project_weak);
                    })
                }
            })
            .detach();
        });

        self.project_indices
            .entry(project.downgrade())
            .or_insert_with(|| {
                cx.new_model(|cx| {
                    ProjectIndex::new(
                        project,
                        self.db_connection.clone(),
                        self.embedding_provider.clone(),
                        cx,
                    )
                })
            })
            .clone()
    }
}
