use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Context, Result};
use assistant_slash_command::SlashCommandOutputSection;
use fs::Fs;
use gpui::{AppContext, Model, Task, WeakView};
use language::LspAdapterDelegate;
use project::{Project, ProjectPath};
use std::{
    fmt::Write,
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, ButtonLike, ElevationIndex};
use workspace::Workspace;

pub(crate) struct ProjectSlashCommand;

impl ProjectSlashCommand {
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

    fn path_to_cargo_toml(project: Model<Project>, cx: &mut AppContext) -> Option<Arc<Path>> {
        let worktree = project.read(cx).worktrees().next()?;
        let worktree = worktree.read(cx);
        let entry = worktree.entry_for_path("Cargo.toml")?;
        let path = ProjectPath {
            worktree_id: worktree.id(),
            path: entry.path.clone(),
        };
        Some(Arc::from(
            project.read(cx).absolute_path(&path, cx)?.as_path(),
        ))
    }
}

impl SlashCommand for ProjectSlashCommand {
    fn name(&self) -> String {
        "project".into()
    }

    fn description(&self) -> String {
        "insert project metadata".into()
    }

    fn menu_text(&self) -> String {
        "Insert Project Metadata".into()
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancel: Arc<AtomicBool>,
        _workspace: WeakView<Workspace>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn run(
        self: Arc<Self>,
        _argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let output = workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            let fs = workspace.project().read(cx).fs().clone();
            let path = Self::path_to_cargo_toml(project, cx);
            let output = cx.background_executor().spawn(async move {
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
                        render_placeholder: Arc::new(move |id, unfold, _cx| {
                            ButtonLike::new(id)
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ElevatedSurface)
                                .child(Icon::new(IconName::FileTree))
                                .child(Label::new("Project"))
                                .on_click(move |_, cx| unfold(cx))
                                .into_any_element()
                        }),
                    }],
                })
            })
        });
        output.unwrap_or_else(|error| Task::ready(Err(error)))
    }
}
