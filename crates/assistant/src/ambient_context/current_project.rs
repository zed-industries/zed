use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use fs::Fs;
use gpui::{AsyncWindowContext, Model, Task};
use project::{Project, ProjectPath};

/// Ambient context about the current project.
pub struct CurrentProjectContext {
    pub metadata: ProjectMetadata,
}

impl CurrentProjectContext {
    /// Loads the [`CurrentProjectContext`] for the given [`Project`].
    pub fn load(
        fs: Arc<dyn Fs>,
        project: Model<Project>,
        cx: &mut AsyncWindowContext,
    ) -> Result<Task<Result<Self>>> {
        let path_to_cargo_toml = cx.update(|cx| {
            let worktree = project
                .read(cx)
                .worktrees()
                .next()
                .ok_or_else(|| anyhow!("no worktree"))?;

            let path_to_cargo_toml = worktree.update(cx, |worktree, _cx| {
                let cargo_toml = worktree.entry_for_path("Cargo.toml")?;
                Some(ProjectPath {
                    worktree_id: worktree.id(),
                    path: cargo_toml.path.clone(),
                })
            });
            let path_to_cargo_toml =
                path_to_cargo_toml.and_then(|path| project.read(cx).absolute_path(&path, cx));

            anyhow::Ok(path_to_cargo_toml)
        })??;

        let path_to_cargo_toml = path_to_cargo_toml.ok_or_else(|| anyhow!("no Cargo.toml"))?;

        Ok(cx.spawn(|_cx| async move {
            let project_metadata = ProjectMetadata::build(fs, &path_to_cargo_toml).await?;

            anyhow::Ok(CurrentProjectContext {
                metadata: project_metadata,
            })
        }))
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
