use super::{diagnostics_command::write_single_file_diagnostics, SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Context as _, Result};
use assistant_slash_command::{AfterCompletion, ArgumentCompletion, SlashCommandOutputSection};
use fuzzy::PathMatch;
use gpui::{AppContext, Model, Task, View, WeakView};
use language::{BufferSnapshot, CodeLabel, HighlightId, LineEnding, LspAdapterDelegate};
use project::{PathMatchCandidateSet, Project};
use std::{
    fmt::Write,
    ops::Range,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};
use ui::prelude::*;
use util::ResultExt;
use workspace::Workspace;

pub(crate) struct FileSlashCommand;

impl FileSlashCommand {
    fn search_paths(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: &View<Workspace>,
        cx: &mut AppContext,
    ) -> Task<Vec<PathMatch>> {
        if query.is_empty() {
            let workspace = workspace.read(cx);
            let project = workspace.project().read(cx);
            let entries = workspace.recent_navigation_history(Some(10), cx);

            let entries = entries
                .into_iter()
                .map(|entries| (entries.0, false))
                .chain(project.worktrees(cx).flat_map(|worktree| {
                    let worktree = worktree.read(cx);
                    let id = worktree.id();
                    worktree.child_entries(Path::new("")).map(move |entry| {
                        (
                            project::ProjectPath {
                                worktree_id: id,
                                path: entry.path.clone(),
                            },
                            entry.kind.is_dir(),
                        )
                    })
                }))
                .collect::<Vec<_>>();

            let path_prefix: Arc<str> = Arc::default();
            Task::ready(
                entries
                    .into_iter()
                    .filter_map(|(entry, is_dir)| {
                        let worktree = project.worktree_for_id(entry.worktree_id, cx)?;
                        let mut full_path = PathBuf::from(worktree.read(cx).root_name());
                        full_path.push(&entry.path);
                        Some(PathMatch {
                            score: 0.,
                            positions: Vec::new(),
                            worktree_id: entry.worktree_id.to_usize(),
                            path: full_path.into(),
                            path_prefix: path_prefix.clone(),
                            distance_to_relative_ancestor: 0,
                            is_dir,
                        })
                    })
                    .collect(),
            )
        } else {
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
                        candidates: project::Candidates::Entries,
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
    }
}

impl SlashCommand for FileSlashCommand {
    fn name(&self) -> String {
        "file".into()
    }

    fn description(&self) -> String {
        "insert file".into()
    }

    fn menu_text(&self) -> String {
        "Insert File".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        cancellation_flag: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let Some(workspace) = workspace.and_then(|workspace| workspace.upgrade()) else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let paths = self.search_paths(
            arguments.last().cloned().unwrap_or_default(),
            cancellation_flag,
            &workspace,
            cx,
        );
        let comment_id = cx.theme().syntax().highlight_id("comment").map(HighlightId);
        cx.background_executor().spawn(async move {
            Ok(paths
                .await
                .into_iter()
                .filter_map(|path_match| {
                    let text = format!(
                        "{}{}",
                        path_match.path_prefix,
                        path_match.path.to_string_lossy()
                    );

                    let mut label = CodeLabel::default();
                    let file_name = path_match.path.file_name()?.to_string_lossy();
                    let label_text = if path_match.is_dir {
                        format!("{}/ ", file_name)
                    } else {
                        format!("{} ", file_name)
                    };

                    label.push_str(label_text.as_str(), None);
                    label.push_str(&text, comment_id);
                    label.filter_range = 0..file_name.len();

                    Some(ArgumentCompletion {
                        label,
                        new_text: text,
                        after_completion: if path_match.is_dir {
                            AfterCompletion::Compose
                        } else {
                            AfterCompletion::Run
                        },
                        replace_previous_arguments: false,
                    })
                })
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        if arguments.is_empty() {
            return Task::ready(Err(anyhow!("missing path")));
        };

        let task = collect_files(workspace.read(cx).project().clone(), arguments, cx);

        cx.foreground_executor().spawn(async move {
            let output = task.await?;
            Ok(SlashCommandOutput {
                text: output.completion_text,
                sections: output
                    .files
                    .into_iter()
                    .map(|file| {
                        build_entry_output_section(
                            file.range_in_text,
                            Some(&file.path),
                            file.entry_type == EntryType::Directory,
                            None,
                        )
                    })
                    .collect(),
                run_commands_in_text: true,
            })
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum EntryType {
    File,
    Directory,
}

#[derive(Clone, PartialEq, Debug)]
struct FileCommandOutput {
    completion_text: String,
    files: Vec<OutputFile>,
}

#[derive(Clone, PartialEq, Debug)]
struct OutputFile {
    range_in_text: Range<usize>,
    path: PathBuf,
    entry_type: EntryType,
}

fn collect_files(
    project: Model<Project>,
    glob_inputs: &[String],
    cx: &mut AppContext,
) -> Task<Result<FileCommandOutput>> {
    let Ok(matchers) = glob_inputs
        .into_iter()
        .map(|glob_input| {
            custom_path_matcher::PathMatcher::new(&[glob_input.to_owned()])
                .with_context(|| format!("invalid path {glob_input}"))
        })
        .collect::<anyhow::Result<Vec<custom_path_matcher::PathMatcher>>>()
    else {
        return Task::ready(Err(anyhow!("invalid path")));
    };

    let project_handle = project.downgrade();
    let snapshots = project
        .read(cx)
        .worktrees(cx)
        .map(|worktree| worktree.read(cx).snapshot())
        .collect::<Vec<_>>();

    cx.spawn(|mut cx| async move {
        let mut text = String::new();
        let mut ranges = Vec::new();
        for snapshot in snapshots {
            let worktree_id = snapshot.id();
            let mut directory_stack: Vec<(Arc<Path>, String, usize)> = Vec::new();
            let mut folded_directory_names_stack = Vec::new();
            let mut is_top_level_directory = true;

            for entry in snapshot.entries(false, 0) {
                let mut path_including_worktree_name = PathBuf::new();
                path_including_worktree_name.push(snapshot.root_name());
                path_including_worktree_name.push(&entry.path);

                if !matchers
                    .iter()
                    .any(|matcher| matcher.is_match(&path_including_worktree_name))
                {
                    continue;
                }

                while let Some((dir, _, _)) = directory_stack.last() {
                    if entry.path.starts_with(dir) {
                        break;
                    }
                    let (_, entry_name, start) = directory_stack.pop().unwrap();
                    ranges.push(OutputFile {
                        range_in_text: start..text.len().saturating_sub(1),
                        path: PathBuf::from(entry_name),
                        entry_type: EntryType::Directory,
                    });
                }

                let filename = entry
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string();

                if entry.is_dir() {
                    // Auto-fold directories that contain no files
                    let mut child_entries = snapshot.child_entries(&entry.path);
                    if let Some(child) = child_entries.next() {
                        if child_entries.next().is_none() && child.kind.is_dir() {
                            if is_top_level_directory {
                                is_top_level_directory = false;
                                folded_directory_names_stack.push(
                                    path_including_worktree_name.to_string_lossy().to_string(),
                                );
                            } else {
                                folded_directory_names_stack.push(filename.to_string());
                            }
                            continue;
                        }
                    } else {
                        // Skip empty directories
                        folded_directory_names_stack.clear();
                        continue;
                    }
                    let prefix_paths = folded_directory_names_stack.drain(..).as_slice().join("/");
                    let entry_start = text.len();
                    if prefix_paths.is_empty() {
                        if is_top_level_directory {
                            text.push_str(&path_including_worktree_name.to_string_lossy());
                            is_top_level_directory = false;
                        } else {
                            text.push_str(&filename);
                        }
                        directory_stack.push((entry.path.clone(), filename, entry_start));
                    } else {
                        let entry_name = format!("{}/{}", prefix_paths, &filename);
                        text.push_str(&entry_name);
                        directory_stack.push((entry.path.clone(), entry_name, entry_start));
                    }
                    text.push('\n');
                } else if entry.is_file() {
                    let Some(open_buffer_task) = project_handle
                        .update(&mut cx, |project, cx| {
                            project.open_buffer((worktree_id, &entry.path), cx)
                        })
                        .ok()
                    else {
                        continue;
                    };
                    if let Some(buffer) = open_buffer_task.await.log_err() {
                        let buffer_snapshot =
                            cx.read_model(&buffer, |buffer, _| buffer.snapshot())?;
                        let prev_len = text.len();
                        collect_file_content(
                            &mut text,
                            &buffer_snapshot,
                            path_including_worktree_name.to_string_lossy().to_string(),
                        );
                        text.push('\n');
                        if !write_single_file_diagnostics(
                            &mut text,
                            Some(&path_including_worktree_name),
                            &buffer_snapshot,
                        ) {
                            text.pop();
                        }
                        ranges.push(OutputFile {
                            range_in_text: prev_len..text.len(),
                            path: path_including_worktree_name,
                            entry_type: EntryType::File,
                        });
                        text.push('\n');
                    }
                }
            }

            while let Some((dir, entry, start)) = directory_stack.pop() {
                if directory_stack.is_empty() {
                    let mut root_path = PathBuf::new();
                    root_path.push(snapshot.root_name());
                    root_path.push(&dir);
                    ranges.push(OutputFile {
                        range_in_text: start..text.len(),
                        path: root_path,
                        entry_type: EntryType::Directory,
                    });
                } else {
                    ranges.push(OutputFile {
                        range_in_text: start..text.len(),
                        path: PathBuf::from(entry.as_str()),
                        entry_type: EntryType::Directory,
                    });
                }
            }
        }
        Ok(FileCommandOutput {
            completion_text: text,
            files: ranges,
        })
    })
}

fn collect_file_content(buffer: &mut String, snapshot: &BufferSnapshot, filename: String) {
    let mut content = snapshot.text();
    LineEnding::normalize(&mut content);
    buffer.reserve(filename.len() + content.len() + 9);
    buffer.push_str(&codeblock_fence_for_path(
        Some(&PathBuf::from(filename)),
        None,
    ));
    buffer.push_str(&content);
    if !buffer.ends_with('\n') {
        buffer.push('\n');
    }
    buffer.push_str("```");
}

pub fn codeblock_fence_for_path(path: Option<&Path>, row_range: Option<Range<u32>>) -> String {
    let mut text = String::new();
    write!(text, "```").unwrap();

    if let Some(path) = path {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            write!(text, "{} ", extension).unwrap();
        }

        write!(text, "{}", path.display()).unwrap();
    } else {
        write!(text, "untitled").unwrap();
    }

    if let Some(row_range) = row_range {
        write!(text, ":{}-{}", row_range.start + 1, row_range.end + 1).unwrap();
    }

    text.push('\n');
    text
}

pub fn build_entry_output_section(
    range: Range<usize>,
    path: Option<&Path>,
    is_directory: bool,
    line_range: Option<Range<u32>>,
) -> SlashCommandOutputSection<usize> {
    let mut label = if let Some(path) = path {
        path.to_string_lossy().to_string()
    } else {
        "untitled".to_string()
    };
    if let Some(line_range) = line_range {
        write!(label, ":{}-{}", line_range.start, line_range.end).unwrap();
    }

    let icon = if is_directory {
        IconName::Folder
    } else {
        IconName::File
    };

    SlashCommandOutputSection {
        range,
        icon,
        label: label.into(),
    }
}

/// This contains a small fork of the util::paths::PathMatcher, that is stricter about the prefix
/// check. Only subpaths pass the prefix check, rather than any prefix.
mod custom_path_matcher {
    use std::{fmt::Debug as _, path::Path};

    use globset::{Glob, GlobSet, GlobSetBuilder};

    #[derive(Clone, Debug, Default)]
    pub struct PathMatcher {
        sources: Vec<String>,
        sources_with_trailing_slash: Vec<String>,
        glob: GlobSet,
    }

    impl std::fmt::Display for PathMatcher {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.sources.fmt(f)
        }
    }

    impl PartialEq for PathMatcher {
        fn eq(&self, other: &Self) -> bool {
            self.sources.eq(&other.sources)
        }
    }

    impl Eq for PathMatcher {}

    impl PathMatcher {
        pub fn new(globs: &[String]) -> Result<Self, globset::Error> {
            let globs = globs
                .into_iter()
                .map(|glob| Glob::new(&glob))
                .collect::<Result<Vec<_>, _>>()?;
            let sources = globs.iter().map(|glob| glob.glob().to_owned()).collect();
            let sources_with_trailing_slash = globs
                .iter()
                .map(|glob| glob.glob().to_string() + std::path::MAIN_SEPARATOR_STR)
                .collect();
            let mut glob_builder = GlobSetBuilder::new();
            for single_glob in globs {
                glob_builder.add(single_glob);
            }
            let glob = glob_builder.build()?;
            Ok(PathMatcher {
                glob,
                sources,
                sources_with_trailing_slash,
            })
        }

        pub fn is_match<P: AsRef<Path>>(&self, other: P) -> bool {
            let other_path = other.as_ref();
            self.sources
                .iter()
                .zip(self.sources_with_trailing_slash.iter())
                .any(|(source, with_slash)| {
                    let as_bytes = other_path.as_os_str().as_encoded_bytes();
                    let with_slash = if source.ends_with("/") {
                        source.as_bytes()
                    } else {
                        with_slash.as_bytes()
                    };

                    as_bytes.starts_with(with_slash) || as_bytes.ends_with(source.as_bytes())
                })
                || self.glob.is_match(other_path)
                || self.check_with_end_separator(other_path)
        }

        fn check_with_end_separator(&self, path: &Path) -> bool {
            let path_str = path.to_string_lossy();
            let separator = std::path::MAIN_SEPARATOR_STR;
            if path_str.ends_with(separator) {
                return false;
            } else {
                self.glob.is_match(path_str.to_string() + separator)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;

    use crate::slash_command::file_command::collect_files;

    pub fn init_test(cx: &mut gpui::TestAppContext) {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::try_init().ok();
        }

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            // release_channel::init(SemanticVersion::default(), cx);
            language::init(cx);
            Project::init_settings(cx);
        });
    }

    #[gpui::test]
    async fn test_file_exact_matching(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            "/root",
            json!({
                "dir": {
                    "subdir": {
                       "file_0": "0"
                    },
                    "file_1": "1",
                    "file_2": "2",
                    "file_3": "3",
                },
                "dir.rs": "4"
            }),
        )
        .await;

        let project = Project::test(fs, ["/root".as_ref()], cx).await;

        let result_1 = cx
            .update(|cx| collect_files(project.clone(), &["root/dir".to_string()], cx))
            .await
            .unwrap();

        assert!(result_1.completion_text.starts_with("root/dir"));
        // 4 files + 2 directories
        assert_eq!(6, result_1.files.len());

        let result_2 = cx
            .update(|cx| collect_files(project.clone(), &["root/dir/".to_string()], cx))
            .await
            .unwrap();

        assert_eq!(result_1, result_2);

        let result = cx
            .update(|cx| collect_files(project.clone(), &["root/dir*".to_string()], cx))
            .await
            .unwrap();

        assert!(result.completion_text.starts_with("root/dir"));
        // 5 files + 2 directories
        assert_eq!(7, result.files.len());

        // Ensure that the project lasts until after the last await
        drop(project);
    }

    #[gpui::test]
    async fn test_file_sub_directory_rendering(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            "/zed",
            json!({
                "assets": {
                    "dir1": {
                        ".gitkeep": ""
                    },
                    "dir2": {
                        ".gitkeep": ""
                    },
                    "themes": {
                        "ayu": {
                            "LICENSE": "1",
                        },
                        "andromeda": {
                            "LICENSE": "2",
                        },
                        "summercamp": {
                            "LICENSE": "3",
                        },
                    },
                },
            }),
        )
        .await;

        let project = Project::test(fs, ["/zed".as_ref()], cx).await;

        let result = cx
            .update(|cx| collect_files(project.clone(), &["zed/assets/themes".to_string()], cx))
            .await
            .unwrap();

        // Sanity check
        assert!(result.completion_text.starts_with("zed/assets/themes\n"));
        assert_eq!(7, result.files.len());

        // Ensure that full file paths are included in the real output
        assert!(result
            .completion_text
            .contains("zed/assets/themes/andromeda/LICENSE"));
        assert!(result
            .completion_text
            .contains("zed/assets/themes/ayu/LICENSE"));
        assert!(result
            .completion_text
            .contains("zed/assets/themes/summercamp/LICENSE"));

        assert_eq!("summercamp", result.files[5].path.to_string_lossy());

        // Ensure that things are in descending order, with properly relativized paths
        assert_eq!(
            "zed/assets/themes/andromeda/LICENSE",
            result.files[0].path.to_string_lossy()
        );
        assert_eq!("andromeda", result.files[1].path.to_string_lossy());
        assert_eq!(
            "zed/assets/themes/ayu/LICENSE",
            result.files[2].path.to_string_lossy()
        );
        assert_eq!("ayu", result.files[3].path.to_string_lossy());
        assert_eq!(
            "zed/assets/themes/summercamp/LICENSE",
            result.files[4].path.to_string_lossy()
        );

        // Ensure that the project lasts until after the last await
        drop(project);
    }

    #[gpui::test]
    async fn test_file_deep_sub_directory_rendering(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            "/zed",
            json!({
                "assets": {
                    "themes": {
                        "LICENSE": "1",
                        "summercamp": {
                            "LICENSE": "1",
                            "subdir": {
                                "LICENSE": "1",
                                "subsubdir": {
                                    "LICENSE": "3",
                                }
                            }
                        },
                    },
                },
            }),
        )
        .await;

        let project = Project::test(fs, ["/zed".as_ref()], cx).await;

        let result = cx
            .update(|cx| collect_files(project.clone(), &["zed/assets/themes".to_string()], cx))
            .await
            .unwrap();

        assert!(result.completion_text.starts_with("zed/assets/themes\n"));
        assert_eq!(
            "zed/assets/themes/LICENSE",
            result.files[0].path.to_string_lossy()
        );
        assert_eq!(
            "zed/assets/themes/summercamp/LICENSE",
            result.files[1].path.to_string_lossy()
        );
        assert_eq!(
            "zed/assets/themes/summercamp/subdir/LICENSE",
            result.files[2].path.to_string_lossy()
        );
        assert_eq!(
            "zed/assets/themes/summercamp/subdir/subsubdir/LICENSE",
            result.files[3].path.to_string_lossy()
        );
        assert_eq!("subsubdir", result.files[4].path.to_string_lossy());
        assert_eq!("subdir", result.files[5].path.to_string_lossy());
        assert_eq!("summercamp", result.files[6].path.to_string_lossy());
        assert_eq!("zed/assets/themes", result.files[7].path.to_string_lossy());

        // Ensure that the project lasts until after the last await
        drop(project);
    }
}
