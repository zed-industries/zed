use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use fs::Fs;
use gpui::{AsyncAppContext, ModelContext, Task, WeakModel};
use project::{Project, ProjectPath};
use util::ResultExt;

use crate::ambient_context::ContextUpdated;
use crate::assistant_panel::Conversation;
use crate::{LanguageModelRequestMessage, Role};

/// Ambient context about the current project.
pub struct CurrentProjectContext {
    pub enabled: bool,
    pub message: String,
    pub pending_message: Option<Task<()>>,
}

#[allow(clippy::derivable_impls)]
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
    /// Returns the [`CurrentProjectContext`] as a message to the language model.
    pub fn to_message(&self) -> Option<LanguageModelRequestMessage> {
        self.enabled.then(|| LanguageModelRequestMessage {
            role: Role::System,
            content: self.message.clone(),
        })
    }

    /// Updates the [`CurrentProjectContext`] for the given [`Project`].
    pub fn update(
        &mut self,
        fs: Arc<dyn Fs>,
        project: WeakModel<Project>,
        cx: &mut ModelContext<Conversation>,
    ) -> ContextUpdated {
        if !self.enabled {
            self.message.clear();
            self.pending_message = None;
            cx.notify();
            return ContextUpdated::Disabled;
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

            let message_task = cx
                .background_executor()
                .spawn(async move { Self::build_message(fs, &path_to_cargo_toml).await });

            if let Some(message) = message_task.await.log_err() {
                conversation
                    .update(&mut cx, |conversation, cx| {
                        conversation.ambient_context.current_project.message = message;
                        conversation.count_remaining_tokens(cx);
                        cx.notify();
                    })
                    .log_err();
            }
        }));

        ContextUpdated::Updating
    }

    async fn build_message(fs: Arc<dyn Fs>, path_to_cargo_toml: &Path) -> Result<String> {
        let buffer = fs.load(path_to_cargo_toml).await?;
        let cargo_toml: cargo_toml::Manifest = toml::from_str(&buffer)?;

        let mut message = String::new();
        writeln!(message, "You are in a Rust project.")?;

        if let Some(workspace) = cargo_toml.workspace {
            writeln!(
                message,
                "The project is a Cargo workspace with the following members:"
            )?;
            for member in workspace.members {
                writeln!(message, "- {member}")?;
            }

            if !workspace.default_members.is_empty() {
                writeln!(message, "The default members are:")?;
                for member in workspace.default_members {
                    writeln!(message, "- {member}")?;
                }
            }

            if !workspace.dependencies.is_empty() {
                writeln!(
                    message,
                    "The following workspace dependencies are installed:"
                )?;
                for dependency in workspace.dependencies.keys() {
                    writeln!(message, "- {dependency}")?;
                }
            }
        } else if let Some(package) = cargo_toml.package {
            writeln!(
                message,
                "The project name is \"{name}\".",
                name = package.name
            )?;

            let description = package
                .description
                .as_ref()
                .and_then(|description| description.get().ok().cloned());
            if let Some(description) = description.as_ref() {
                writeln!(message, "It describes itself as \"{description}\".")?;
            }

            if !cargo_toml.dependencies.is_empty() {
                writeln!(message, "The following dependencies are installed:")?;
                for dependency in cargo_toml.dependencies.keys() {
                    writeln!(message, "- {dependency}")?;
                }
            }
        }

        Ok(message)
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
