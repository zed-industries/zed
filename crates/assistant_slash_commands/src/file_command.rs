use anyhow::{Context as _, Result, anyhow};
use assistant_slash_command::{
    AfterCompletion, ArgumentCompletion, SlashCommand, SlashCommandContent, SlashCommandEvent,
    SlashCommandOutput, SlashCommandOutputSection, SlashCommandResult,
};
use futures::Stream;
use futures::channel::mpsc;
use fuzzy::PathMatch;
use gpui::{App, Entity, Task, WeakEntity};
use language::{BufferSnapshot, CodeLabel, HighlightId, LineEnding, LspAdapterDelegate};
use project::{PathMatchCandidateSet, Project};
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::{
    fmt::Write,
    ops::{Range, RangeInclusive},
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
};
use ui::prelude::*;
use util::ResultExt;
use workspace::Workspace;
use worktree::ChildEntriesOptions;

pub struct FileSlashCommand;

impl FileSlashCommand {
    fn search_paths(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: &Entity<Workspace>,
        cx: &mut App,
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
                    let options = ChildEntriesOptions {
                        include_files: true,
                        include_dirs: true,
                        include_ignored: false,
                    };
                    let entries = worktree.child_entries_with_options(Path::new(""), options);
                    entries.map(move |entry| {
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
                            .is_some_and(|entry| entry.is_ignored),
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
        "Insert file and/or directory".into()
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn icon(&self) -> IconName {
        IconName::File
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        cancellation_flag: Arc<AtomicBool>,
        workspace: Option<WeakEntity<Workspace>>,
        _: &mut Window,
        cx: &mut App,
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
        cx.background_spawn(async move {
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
                        after_completion: AfterCompletion::Compose,
                        replace_previous_arguments: false,
                    })
                })
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        if arguments.is_empty() {
            return Task::ready(Err(anyhow!("missing path")));
        };

        Task::ready(Ok(collect_files(
            workspace.read(cx).project().clone(),
            arguments,
            cx,
        )
        .boxed()))
    }
}

fn collect_files(
    project: Entity<Project>,
    glob_inputs: &[String],
    cx: &mut App,
) -> impl Stream<Item = Result<SlashCommandEvent>> + use<> {
    let Ok(matchers) = glob_inputs
        .iter()
        .map(|glob_input| {
            custom_path_matcher::PathMatcher::new(&[glob_input.to_owned()])
                .with_context(|| format!("invalid path {glob_input}"))
        })
        .collect::<anyhow::Result<Vec<custom_path_matcher::PathMatcher>>>()
    else {
        return futures::stream::once(async {
            anyhow::bail!("invalid path");
        })
        .boxed();
    };

    let project_handle = project.downgrade();
    let snapshots = project
        .read(cx)
        .worktrees(cx)
        .map(|worktree| worktree.read(cx).snapshot())
        .collect::<Vec<_>>();

    let (events_tx, events_rx) = mpsc::unbounded();
    cx.spawn(async move |cx| {
        for snapshot in snapshots {
            let worktree_id = snapshot.id();
            let mut directory_stack: Vec<Arc<Path>> = Vec::new();
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

                while let Some(dir) = directory_stack.last() {
                    if entry.path.starts_with(dir) {
                        break;
                    }
                    directory_stack.pop().unwrap();
                    events_tx.unbounded_send(Ok(SlashCommandEvent::EndSection))?;
                    events_tx.unbounded_send(Ok(SlashCommandEvent::Content(
                        SlashCommandContent::Text {
                            text: "\n".into(),
                            run_commands_in_text: false,
                        },
                    )))?;
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
                    if prefix_paths.is_empty() {
                        let label = if is_top_level_directory {
                            is_top_level_directory = false;
                            path_including_worktree_name.to_string_lossy().to_string()
                        } else {
                            filename
                        };
                        events_tx.unbounded_send(Ok(SlashCommandEvent::StartSection {
                            icon: IconName::Folder,
                            label: label.clone().into(),
                            metadata: None,
                        }))?;
                        events_tx.unbounded_send(Ok(SlashCommandEvent::Content(
                            SlashCommandContent::Text {
                                text: label,
                                run_commands_in_text: false,
                            },
                        )))?;
                        directory_stack.push(entry.path.clone());
                    } else {
                        // todo(windows)
                        // Potential bug: this assumes that the path separator is always `\` on Windows
                        let entry_name = format!(
                            "{}{}{}",
                            prefix_paths,
                            std::path::MAIN_SEPARATOR_STR,
                            &filename
                        );
                        events_tx.unbounded_send(Ok(SlashCommandEvent::StartSection {
                            icon: IconName::Folder,
                            label: entry_name.clone().into(),
                            metadata: None,
                        }))?;
                        events_tx.unbounded_send(Ok(SlashCommandEvent::Content(
                            SlashCommandContent::Text {
                                text: entry_name,
                                run_commands_in_text: false,
                            },
                        )))?;
                        directory_stack.push(entry.path.clone());
                    }
                    events_tx.unbounded_send(Ok(SlashCommandEvent::Content(
                        SlashCommandContent::Text {
                            text: "\n".into(),
                            run_commands_in_text: false,
                        },
                    )))?;
                } else if entry.is_file() {
                    let Some(open_buffer_task) = project_handle
                        .update(cx, |project, cx| {
                            project.open_buffer((worktree_id, &entry.path), cx)
                        })
                        .ok()
                    else {
                        continue;
                    };
                    if let Some(buffer) = open_buffer_task.await.log_err() {
                        let mut output = SlashCommandOutput::default();
                        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
                        append_buffer_to_output(
                            &snapshot,
                            Some(&path_including_worktree_name),
                            &mut output,
                        )
                        .log_err();
                        let mut buffer_events = output.into_event_stream();
                        while let Some(event) = buffer_events.next().await {
                            events_tx.unbounded_send(event)?;
                        }
                    }
                }
            }

            while directory_stack.pop().is_some() {
                events_tx.unbounded_send(Ok(SlashCommandEvent::EndSection))?;
            }
        }

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);

    events_rx.boxed()
}

pub fn codeblock_fence_for_path(
    path: Option<&Path>,
    row_range: Option<RangeInclusive<u32>>,
) -> String {
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
        write!(text, ":{}-{}", row_range.start() + 1, row_range.end() + 1).unwrap();
    }

    text.push('\n');
    text
}

#[derive(Serialize, Deserialize)]
pub struct FileCommandMetadata {
    pub path: String,
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
        metadata: if is_directory {
            None
        } else {
            path.and_then(|path| {
                serde_json::to_value(FileCommandMetadata {
                    path: path.to_string_lossy().to_string(),
                })
                .ok()
            })
        },
    }
}

/// This contains a small fork of the util::paths::PathMatcher, that is stricter about the prefix
/// check. Only subpaths pass the prefix check, rather than any prefix.
mod custom_path_matcher {
    use std::{fmt::Debug as _, path::Path};

    use globset::{Glob, GlobSet, GlobSetBuilder};
    use util::paths::SanitizedPath;

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
                .iter()
                .map(|glob| Glob::new(&SanitizedPath::new(glob).to_glob_string()))
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
                    // todo(windows)
                    // Potential bug: this assumes that the path separator is always `\` on Windows
                    let with_slash = if source.ends_with(std::path::MAIN_SEPARATOR_STR) {
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
                false
            } else {
                self.glob.is_match(path_str.to_string() + separator)
            }
        }
    }
}

pub fn append_buffer_to_output(
    buffer: &BufferSnapshot,
    path: Option<&Path>,
    output: &mut SlashCommandOutput,
) -> Result<()> {
    let prev_len = output.text.len();

    let mut content = buffer.text();
    LineEnding::normalize(&mut content);
    output.text.push_str(&codeblock_fence_for_path(path, None));
    output.text.push_str(&content);
    if !output.text.ends_with('\n') {
        output.text.push('\n');
    }
    output.text.push_str("```");
    output.text.push('\n');

    let section_ix = output.sections.len();
    output.sections.insert(
        section_ix,
        build_entry_output_section(prev_len..output.text.len(), path, false, None),
    );

    output.text.push('\n');

    Ok(())
}

#[cfg(test)]
mod test {
    use assistant_slash_command::SlashCommandOutput;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use pretty_assertions::assert_eq;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use smol::stream::StreamExt;
    use util::path;

    use super::collect_files;

    pub fn init_test(cx: &mut gpui::TestAppContext) {
        zlog::init_test();

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
            path!("/root"),
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

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;

        let result_1 =
            cx.update(|cx| collect_files(project.clone(), &["root/dir".to_string()], cx));
        let result_1 = SlashCommandOutput::from_event_stream(result_1.boxed())
            .await
            .unwrap();

        assert!(result_1.text.starts_with(path!("root/dir")));
        // 4 files + 2 directories
        assert_eq!(result_1.sections.len(), 6);

        let result_2 =
            cx.update(|cx| collect_files(project.clone(), &["root/dir/".to_string()], cx));
        let result_2 = SlashCommandOutput::from_event_stream(result_2.boxed())
            .await
            .unwrap();

        assert_eq!(result_1, result_2);

        let result =
            cx.update(|cx| collect_files(project.clone(), &["root/dir*".to_string()], cx).boxed());
        let result = SlashCommandOutput::from_event_stream(result).await.unwrap();

        assert!(result.text.starts_with(path!("root/dir")));
        // 5 files + 2 directories
        assert_eq!(result.sections.len(), 7);

        // Ensure that the project lasts until after the last await
        drop(project);
    }

    #[gpui::test]
    async fn test_file_sub_directory_rendering(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            path!("/zed"),
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

        let project = Project::test(fs, [path!("/zed").as_ref()], cx).await;

        let result =
            cx.update(|cx| collect_files(project.clone(), &["zed/assets/themes".to_string()], cx));
        let result = SlashCommandOutput::from_event_stream(result.boxed())
            .await
            .unwrap();

        // Sanity check
        assert!(result.text.starts_with(path!("zed/assets/themes\n")));
        assert_eq!(result.sections.len(), 7);

        // Ensure that full file paths are included in the real output
        assert!(
            result
                .text
                .contains(path!("zed/assets/themes/andromeda/LICENSE"))
        );
        assert!(result.text.contains(path!("zed/assets/themes/ayu/LICENSE")));
        assert!(
            result
                .text
                .contains(path!("zed/assets/themes/summercamp/LICENSE"))
        );

        assert_eq!(result.sections[5].label, "summercamp");

        // Ensure that things are in descending order, with properly relativized paths
        assert_eq!(
            result.sections[0].label,
            path!("zed/assets/themes/andromeda/LICENSE")
        );
        assert_eq!(result.sections[1].label, "andromeda");
        assert_eq!(
            result.sections[2].label,
            path!("zed/assets/themes/ayu/LICENSE")
        );
        assert_eq!(result.sections[3].label, "ayu");
        assert_eq!(
            result.sections[4].label,
            path!("zed/assets/themes/summercamp/LICENSE")
        );

        // Ensure that the project lasts until after the last await
        drop(project);
    }

    #[gpui::test]
    async fn test_file_deep_sub_directory_rendering(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            path!("/zed"),
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

        let project = Project::test(fs, [path!("/zed").as_ref()], cx).await;

        let result =
            cx.update(|cx| collect_files(project.clone(), &["zed/assets/themes".to_string()], cx));
        let result = SlashCommandOutput::from_event_stream(result.boxed())
            .await
            .unwrap();

        assert!(result.text.starts_with(path!("zed/assets/themes\n")));
        assert_eq!(result.sections[0].label, path!("zed/assets/themes/LICENSE"));
        assert_eq!(
            result.sections[1].label,
            path!("zed/assets/themes/summercamp/LICENSE")
        );
        assert_eq!(
            result.sections[2].label,
            path!("zed/assets/themes/summercamp/subdir/LICENSE")
        );
        assert_eq!(
            result.sections[3].label,
            path!("zed/assets/themes/summercamp/subdir/subsubdir/LICENSE")
        );
        assert_eq!(result.sections[4].label, "subsubdir");
        assert_eq!(result.sections[5].label, "subdir");
        assert_eq!(result.sections[6].label, "summercamp");
        assert_eq!(result.sections[7].label, path!("zed/assets/themes"));

        assert_eq!(
            result.text,
            path!(
                "zed/assets/themes\n```zed/assets/themes/LICENSE\n1\n```\n\nsummercamp\n```zed/assets/themes/summercamp/LICENSE\n1\n```\n\nsubdir\n```zed/assets/themes/summercamp/subdir/LICENSE\n1\n```\n\nsubsubdir\n```zed/assets/themes/summercamp/subdir/subsubdir/LICENSE\n3\n```\n\n"
            )
        );

        // Ensure that the project lasts until after the last await
        drop(project);
    }
}
