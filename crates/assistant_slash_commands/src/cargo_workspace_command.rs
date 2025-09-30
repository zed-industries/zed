use anyhow::{Context as _, Result, anyhow};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use fs::Fs;
use gpui::{App, Entity, Task, WeakEntity};
use language::{BufferSnapshot, LspAdapterDelegate};
use project::{Project, ProjectPath};
use std::{
    fmt::Write,
    path::Path,
    sync::{Arc, atomic::AtomicBool},
};
use ui::prelude::*;
use util::rel_path::RelPath;
use workspace::Workspace;

pub struct CargoWorkspaceSlashCommand;

impl CargoWorkspaceSlashCommand {
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

    fn path_to_cargo_toml(project: Entity<Project>, cx: &mut App) -> Option<Arc<Path>> {
        let worktree = project.read(cx).worktrees(cx).next()?;
        let worktree = worktree.read(cx);
        let entry = worktree.entry_for_path(RelPath::new("Cargo.toml").unwrap())?;
        let path = ProjectPath {
            worktree_id: worktree.id(),
            path: entry.path.clone(),
        };
        Some(Arc::from(
            project.read(cx).absolute_path(&path, cx)?.as_path(),
        ))
    }
}

impl SlashCommand for CargoWorkspaceSlashCommand {
    fn name(&self) -> String {
        "cargo-workspace".into()
    }

    fn description(&self) -> String {
        "insert project workspace metadata".into()
    }

    fn menu_text(&self) -> String {
        "Insert Project Workspace Metadata".into()
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let output = workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            let fs = workspace.project().read(cx).fs().clone();
            let path = Self::path_to_cargo_toml(project, cx);
            let output = cx.background_spawn(async move {
                let path = path.with_context(|| "Cargo.toml not found")?;
                Self::build_message(fs, &path).await
            });

            cx.foreground_executor().spawn(async move {
                let text = output.await?;
                let range = 0..text.len();
                Ok(SlashCommandOutput {
                    text,
                    sections: vec![SlashCommandOutputSection {
                        range,
                        icon: IconName::FileTree,
                        label: "Project".into(),
                        metadata: None,
                    }],
                    run_commands_in_text: false,
                }
                .into_event_stream())
            })
        });
        output.unwrap_or_else(|error| Task::ready(Err(error)))
    }
}
