use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result};
use gpui::{App, Entity, EventEmitter, Task};
use project::{Project, ProjectEntryId, ProjectItem, ProjectPath};

use crate::pdf_engine::{PdfDocument, PdfEngine, create_default_engine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfItemEvent {
    PageChanged,
    Reloaded,
}

pub struct PdfItem {
    project_path: ProjectPath,
    abs_path: Option<PathBuf>,
    file_name: String,
    document: Arc<dyn PdfDocument>,
    engine: Arc<dyn PdfEngine>,
}

impl EventEmitter<PdfItemEvent> for PdfItem {}

impl PdfItem {
    pub fn new(
        project_path: ProjectPath,
        abs_path: Option<PathBuf>,
        file_name: String,
        document: Arc<dyn PdfDocument>,
        engine: Arc<dyn PdfEngine>,
    ) -> Self {
        Self {
            project_path,
            abs_path,
            file_name,
            document,
            engine,
        }
    }

    pub fn document(&self) -> Arc<dyn PdfDocument> {
        self.document.clone()
    }

    pub fn engine(&self) -> &Arc<dyn PdfEngine> {
        &self.engine
    }

    pub fn abs_path(&self) -> Option<&PathBuf> {
        self.abs_path.as_ref()
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn project_path(&self) -> &ProjectPath {
        &self.project_path
    }
}

impl ProjectItem for PdfItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        let ext = path.path.extension()?.to_ascii_lowercase();
        if ext != "pdf" {
            return None;
        }

        let project = project.clone();
        let path = path.clone();

        Some(cx.spawn(async move |cx| {
            let (abs_path, file_name, fs) = project.update(cx, |project, cx| {
                let abs_path = project.absolute_path(&path, cx);
                let file_name = path
                    .path
                    .file_name()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "document.pdf".to_string());
                let fs = project.fs().clone();
                (abs_path, file_name, fs)
            });

            let Some(abs_path) = abs_path else {
                return Err(anyhow::anyhow!("File not found or not local"));
            };

            let bytes = fs.load_bytes(&abs_path).await.context("Failed to load PDF file bytes")?;

            let engine = create_default_engine();
            let document = engine.load_document(&abs_path, &bytes)?;

            let item = cx.update(|cx| {
                use gpui::AppContext as _;
                cx.new(|_cx| {
                    PdfItem::new(path, Some(abs_path), file_name, document, engine)
                })
            });

            Ok(item)
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
