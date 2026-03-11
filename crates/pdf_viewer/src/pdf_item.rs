use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, BackgroundExecutor, Entity, EventEmitter, Task};
use project::{Project, ProjectEntryId, ProjectItem, ProjectPath};

pub struct PdfItem {
    project_path: ProjectPath,
    abs_path: PathBuf,
    pdf_bytes: Arc<[u8]>,
}

pub enum PdfItemEvent {
    Reloaded,
}

impl EventEmitter<PdfItemEvent> for PdfItem {}

impl PdfItem {
    pub fn abs_path(&self) -> &Path {
        &self.abs_path
    }

    pub fn file_name(&self) -> &str {
        self.abs_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("document.pdf")
    }

    pub fn pdf_bytes(&self) -> &Arc<[u8]> {
        &self.pdf_bytes
    }

    pub fn project_path(&self) -> &ProjectPath {
        &self.project_path
    }
}

pub fn is_pdf_file(path: &ProjectPath) -> bool {
    path.path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
}

fn load_pdf_bytes(abs_path: PathBuf, background: BackgroundExecutor) -> Task<Result<Arc<[u8]>>> {
    background.spawn(async move {
        let bytes = std::fs::read(&abs_path)
            .with_context(|| format!("Failed to read PDF: {}", abs_path.display()))?;
        Ok(Arc::from(bytes))
    })
}

impl ProjectItem for PdfItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        if !is_pdf_file(path) {
            return None;
        }

        let worktree = project.read(cx).worktree_for_id(path.worktree_id, cx)?;
        let abs_path = worktree
            .read(cx)
            .abs_path()
            .join(path.path.as_std_path());
        let project_path = path.clone();
        let background = cx.background_executor().clone();

        Some(cx.spawn(async move |cx| {
            let pdf_bytes = load_pdf_bytes(abs_path.clone(), background).await?;

            let entity = cx.update(|cx| {
                cx.new(|_| PdfItem {
                    project_path,
                    abs_path,
                    pdf_bytes,
                })
            });
            Ok(entity)
        }))
    }

    fn entry_id(&self, _cx: &App) -> Option<ProjectEntryId> {
        None
    }

    fn project_path(&self, _cx: &App) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }

    fn is_dirty(&self) -> bool {
        false
    }
}