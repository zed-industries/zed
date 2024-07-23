use super::{
    file_command::{build_entry_output_section, codeblock_fence_for_path},
    SlashCommand, SlashCommandOutput,
};
use anyhow::{anyhow, Context, Result};
use assistant_slash_command::{ArgumentCompletion, SlashCommandOutputSection};
use fuzzy::PathMatch;
use gpui::{AppContext, Task, View, WeakView};
use language::LspAdapterDelegate;
use project::PathMatchCandidateSet;
use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};
use ui::prelude::*;
use workspace::Workspace;

pub(crate) struct DirectorySlashCommand;

impl DirectorySlashCommand {
    fn search_paths(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: &View<Workspace>,
        cx: &mut AppContext,
    ) -> Task<Vec<PathMatch>> {
        let worktrees = workspace.read(cx).visible_worktrees(cx).collect::<Vec<_>>();
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                PathMatchCandidateSet {
                    snapshot: worktree.snapshot(),
                    include_ignored: worktree
                        .root_entry()
                        .map_or(false, |entry| entry.is_ignored),
                    include_root_name: true,
                    candidates: project::Candidates::Directories,
                }
            })
            .collect::<Vec<_>>();

        let executor = cx.background_executor().clone();
        cx.foreground_executor().spawn(async move {
            fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                query.as_str(),
                None,
                false,
                100,
                &cancellation_flag,
                executor,
            )
            .await
        })
    }

    fn add_directory_contents(
        &self,
        base_path: &Path,
        current_path: &Path,
        text: &mut String,
        sections: &mut Vec<SlashCommandOutputSection<usize>>,
        directory_stack: &mut Vec<(PathBuf, String, usize)>,
    ) -> Result<()> {
        let mut entries: Vec<_> = fs::read_dir(current_path)?.collect::<Result<_, _>>()?;
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let path = entry.path();
            let relative_path = path.strip_prefix(base_path).unwrap_or(&path);

            if path.is_dir() {
                let entry_start = text.len();
                text.push_str(&relative_path.to_string_lossy());
                text.push('\n');
                directory_stack.push((
                    path.clone(),
                    relative_path.to_string_lossy().to_string(),
                    entry_start,
                ));
                self.add_directory_contents(base_path, &path, text, sections, directory_stack)?;
            } else if path.is_file() {
                if let Ok(content) = fs::read_to_string(&path) {
                    let start = text.len();
                    text.push_str(&codeblock_fence_for_path(Some(relative_path), None));
                    text.push_str(&content);
                    if !text.ends_with('\n') {
                        text.push('\n');
                    }
                    writeln!(text, "```").unwrap();
                    sections.push(build_entry_output_section(
                        start..text.len(),
                        Some(relative_path),
                        false,
                        None,
                    ));
                    text.push('\n');
                }
            }
        }

        Ok(())
    }
}

impl SlashCommand for DirectorySlashCommand {
    fn name(&self) -> String {
        "directory".into()
    }

    fn description(&self) -> String {
        "insert contents of all files under a directory".into()
    }

    fn menu_text(&self) -> String {
        "Insert Directory Contents".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let Some(workspace) = workspace.and_then(|workspace| workspace.upgrade()) else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let paths = self.search_paths(query, cancellation_flag, &workspace, cx);
        cx.background_executor().spawn(async move {
            Ok(paths
                .await
                .into_iter()
                .map(|path_match| {
                    let full_path = format!(
                        "{}{}",
                        path_match.path_prefix,
                        path_match.path.to_string_lossy()
                    );

                    ArgumentCompletion {
                        label: full_path.clone(),
                        new_text: full_path,
                        run_command: true,
                    }
                })
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let Some(argument) = argument else {
            return Task::ready(Err(anyhow!("missing path")));
        };

        let project = workspace.read(cx).project().clone();
        let directory_path = PathBuf::from(argument);

        cx.spawn(|mut cx| async move {
            let full_path = project
                .update(&mut cx, |project, cx| {
                    for worktree in project.worktrees() {
                        let worktree = worktree.read(cx);
                        let root_path = worktree.abs_path();
                        // Check if the directory_path starts with the root name
                        if let Some(root_name) = root_path.file_name() {
                            if directory_path.starts_with(root_name) {
                                let potential_full_path = root_path.join(
                                    directory_path
                                        .strip_prefix(root_name)
                                        .unwrap_or(&directory_path),
                                );
                                if potential_full_path.is_dir() {
                                    return Ok(potential_full_path);
                                }
                            }
                        }
                        // If not, try joining directly
                        let potential_full_path = root_path.join(&directory_path);
                        if potential_full_path.is_dir() {
                            return Ok(potential_full_path);
                        }
                    }
                    Err(anyhow!("Directory not found in any worktree"))
                })
                .context("Failed to locate directory")?;

            let mut text = String::new();
            let mut sections = Vec::new();
            let mut directory_stack = Vec::new();
            // Unwrap the Result to get the PathBuf
            let full_path = full_path?;

            self.add_directory_contents(
                &full_path,
                &full_path,
                &mut text,
                &mut sections,
                &mut directory_stack,
            )
            .context("Failed to add directory contents")?;

            while let Some((_, entry_name, start)) = directory_stack.pop() {
                sections.push(build_entry_output_section(
                    start..text.len(),
                    Some(&PathBuf::from(entry_name)),
                    true,
                    None,
                ));
            }

            if sections.is_empty() {
                return Err(anyhow!("No files found in the specified directory"));
            }

            Ok(SlashCommandOutput {
                text,
                sections,
                run_commands_in_text: false,
            })
        })
    }
}
