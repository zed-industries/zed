use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use fs::Fs;
use gpui::{AsyncAppContext, ModelContext, Task, WeakModel};
use project::{Project, ProjectPath};
use util::ResultExt;

use crate::assistant_panel::Conversation;

/// Ambient context about the current project.
pub struct CurrentProjectContext {
    pub enabled: bool,
    pub message: String,
    pub pending_message: Option<Task<()>>,
}

impl Default for CurrentProjectContext {
    fn default() -> Self {
        Self {
            enabled: false,
            message: String::new(),
            pending_message: None,
        }
    }
}

impl CurrentProjectContext {
    /// Updates the [`CurrentProjectContext`] for the given [`Project`].
    pub fn update(
        &mut self,
        fs: Arc<dyn Fs>,
        project: WeakModel<Project>,
        cx: &mut ModelContext<Conversation>,
    ) {
        if !self.enabled {
            self.message.clear();
            self.pending_message = None;
            cx.notify();
            return;
        }

        self.pending_message = Some(cx.spawn(|conversation, mut cx| async move {
            const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(100);
            cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;

            let Some(path_to_cargo_toml) = Self::path_to_cargo_toml(project, &mut cx).log_err()
            else {
                return;
            };

            let Some(path_to_cargo_toml) = path_to_cargo_toml
                .ok_or_else(|| anyhow!("no Cargo.toml"))
                .log_err()
            else {
                return;
            };

            let message_task = cx.background_executor().spawn(async move {
                let project_metadata = ProjectMetadata::build(fs, &path_to_cargo_toml).await?;

                anyhow::Ok(project_metadata.render_as_string())
            });

            if let Some(message) = message_task.await.log_err() {
                conversation
                    .update(&mut cx, |conversation, _cx| {
                        dbg!(&message);
                        conversation.ambient_context.current_project.message = message;
                    })
                    .log_err();
            }
        }));
    }

    fn path_to_cargo_toml(
        project: WeakModel<Project>,
        cx: &mut AsyncAppContext,
    ) -> Result<Option<PathBuf>> {
        cx.update(|cx| {
            let worktree = project.update(cx, |project, _cx| {
                project
                    .worktrees()
                    .next()
                    .ok_or_else(|| anyhow!("no worktree"))
            })??;

            let path_to_cargo_toml = worktree.update(cx, |worktree, _cx| {
                let cargo_toml = worktree.entry_for_path("Cargo.toml")?;
                Some(ProjectPath {
                    worktree_id: worktree.id(),
                    path: cargo_toml.path.clone(),
                })
            });
            let path_to_cargo_toml = path_to_cargo_toml.and_then(|path| {
                project
                    .update(cx, |project, cx| project.absolute_path(&path, cx))
                    .ok()
                    .flatten()
            });

            Ok(path_to_cargo_toml)
        })?
    }
}

#[derive(Debug)]
pub struct ProjectMetadata {
    pub name: Option<String>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub license: Option<String>,
    pub dependencies: Vec<String>,
}

impl ProjectMetadata {
    async fn build(fs: Arc<dyn Fs>, path: &Path) -> Result<Self> {
        let buffer = fs.load(path).await?;
        let cargo_toml: cargo_toml::Manifest = toml::from_str(&buffer)?;

        Ok(Self {
            name: cargo_toml
                .package
                .as_ref()
                .map(|package| package.name.clone()),
            authors: cargo_toml
                .package
                .as_ref()
                .and_then(|package| package.authors.get().ok().cloned())
                .unwrap_or_default(),
            description: cargo_toml
                .package
                .as_ref()
                .and_then(|package| package.description.as_ref())
                .and_then(|description| description.get().ok().cloned()),
            version: cargo_toml
                .package
                .as_ref()
                .and_then(|package| package.version.get().ok().cloned()),
            license: cargo_toml
                .package
                .as_ref()
                .and_then(|package| package.license.as_ref())
                .and_then(|license| license.get().ok().cloned()),
            dependencies: cargo_toml.dependencies.keys().cloned().collect(),
        })
    }

    pub fn render_as_string(&self) -> String {
        let mut prompt = "You are in a Rust project".to_string();
        if let Some(name) = self.name.as_ref() {
            prompt.push_str(&format!(" named \"{name}\""));
        }
        prompt.push_str(". ");

        if let Some(description) = self.description.as_ref() {
            prompt.push_str("It describes itself as ");
            prompt.push_str(&format!("\"{description}\""));
            prompt.push_str(". ");
        }

        if !self.dependencies.is_empty() {
            prompt.push_str("The following dependencies are installed: ");
            prompt.push_str(&self.dependencies.join(", "));
            prompt.push_str(". ");
        }

        prompt
    }
}
