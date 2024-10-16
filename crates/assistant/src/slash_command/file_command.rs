use super::{
    buffer_to_output, SlashCommand, SlashCommandEvent, SlashCommandOutputSection,
    SlashCommandResult,
};
// use super::diagnostics_command::collect_buffer_diagnostics;
use anyhow::{anyhow, Context as _, Result};
use assistant_slash_command::{AfterCompletion, ArgumentCompletion, SlashCommandContentType};
use futures::{
    channel::mpsc,
    stream::{self, StreamExt},
};
use fuzzy::PathMatch;
use gpui::{AppContext, Model, Task, View, WeakView};
use language::{BufferSnapshot, CodeLabel, HighlightId, LspAdapterDelegate};
use project::{PathMatchCandidateSet, Project};
use std::{
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
        "Insert file".into()
    }

    fn menu_text(&self) -> String {
        self.description()
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
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        if arguments.is_empty() {
            return Task::ready(Err(anyhow!("missing path")));
        };

        collect_files(workspace.read(cx).project().clone(), arguments, cx)
    }
}

fn collect_files(
    project: Model<Project>,
    glob_inputs: &[String],
    cx: &mut AppContext,
) -> Task<SlashCommandResult> {
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

    let (events_tx, events_rx) = mpsc::unbounded();
    cx.spawn(|mut cx| async move {
        for snapshot in snapshots {
            let worktree_id = snapshot.id();
            let mut folded_directory_names = Vec::new();
            let mut is_top_level_directory = true;
            let mut directory_stack = Vec::new();

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

                let filename = entry
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string();

                while !directory_stack.is_empty()
                    && !entry.path.starts_with(directory_stack.last().unwrap())
                {
                    log::info!("end dir");
                    directory_stack.pop();
                    events_tx.unbounded_send(SlashCommandEvent::EndSection { metadata: None })?;
                }

                if entry.is_dir() {
                    log::info!("start dir");
                    let mut child_entries = snapshot.child_entries(&entry.path);
                    if let Some(child) = child_entries.next() {
                        if child_entries.next().is_none() && child.kind.is_dir() {
                            if is_top_level_directory {
                                is_top_level_directory = false;
                                folded_directory_names.push(
                                    path_including_worktree_name.to_string_lossy().to_string(),
                                );
                            } else {
                                folded_directory_names.push(filename.to_string())
                            }
                        }
                    } else {
                        // Skip empty directories
                        folded_directory_names.clear();
                        continue;
                    }

                    let prefix_paths = folded_directory_names.drain(..).as_slice().join("/");
                    let dirname = if prefix_paths.is_empty() {
                        if is_top_level_directory {
                            is_top_level_directory = false;
                            path_including_worktree_name.to_string_lossy().to_string()
                        } else {
                            filename
                        }
                    } else {
                        format!("{}/{}", prefix_paths, &filename)
                    };
                    events_tx.unbounded_send(SlashCommandEvent::StartSection {
                        icon: IconName::Folder,
                        label: dirname.clone().into(),
                        metadata: None,
                    })?;
                    events_tx.unbounded_send(SlashCommandEvent::Content(
                        SlashCommandContentType::Text {
                            text: dirname,
                            run_commands_in_text: false,
                        },
                    ))?;
                    directory_stack.push(entry.path.clone());
                } else if entry.is_file() {
                    let open_buffer_task = project_handle
                        .update(&mut cx, |project, cx| {
                            project.open_buffer((worktree_id, &entry.path), cx)
                        })
                        .ok();
                    let Some(open_buffer_task) = open_buffer_task else {
                        continue;
                    };
                    if let Some(buffer) = open_buffer_task.await.log_err() {
                        let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot())?;
                        let events_from_buffer =
                            buffer_to_output(&snapshot, Some(&path_including_worktree_name))?;
                        for event in events_from_buffer {
                            events_tx.unbounded_send(event)?;
                        }
                    }
                }
            }

            // Close any remaining open directories
            while !directory_stack.is_empty() {
                log::info!("end dir");
                directory_stack.pop();
                events_tx.unbounded_send(SlashCommandEvent::EndSection { metadata: None })?;
            }
        }
        anyhow::Ok(())
    })
    .detach();
    Task::ready(Ok(events_rx.boxed()))
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
    use futures::StreamExt;
    use gpui::TestAppContext;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;

    use crate::slash_command::file_command::collect_files;
    use assistant_slash_command::{SlashCommandContentType, SlashCommandEvent};

    pub fn init_test(cx: &mut gpui::TestAppContext) {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::try_init().ok();
        }

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
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

        let mut result_1 = cx
            .update(|cx| collect_files(project.clone(), &["root/dir".to_string()], cx))
            .await
            .unwrap();

        let mut events_1 = Vec::new();
        while let Some(event) = result_1.next().await {
            events_1.push(event);
        }

        // Check first event is StartSection with "root/dir"
        assert!(matches!(&events_1[0],
            SlashCommandEvent::StartSection { label, .. } if label.starts_with("root/dir")));

        // 4 files + 2 directories
        assert_eq!(events_1.len(), 18); // 2 events per section (start/end) + content events

        let mut result_2 = cx
            .update(|cx| collect_files(project.clone(), &["root/dir/".to_string()], cx))
            .await
            .unwrap();

        let mut events_2 = Vec::new();
        while let Some(event) = result_2.next().await {
            events_2.push(event);
        }

        assert_eq!(events_1.len(), events_2.len());

        let mut result = cx
            .update(|cx| collect_files(project.clone(), &["root/dir*".to_string()], cx))
            .await
            .unwrap();

        let mut events = Vec::new();
        while let Some(event) = result.next().await {
            events.push(event);
        }

        // Check first event is StartSection with "root/dir"
        assert!(matches!(&events[0],
            SlashCommandEvent::StartSection { label, .. } if label.starts_with("root/dir")));

        // 5 files + 2 directories
        assert_eq!(events.len(), 21); // 2 events per section (start/end) + content events

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

        let mut result = cx
            .update(|cx| collect_files(project.clone(), &["zed/assets/themes".to_string()], cx))
            .await
            .unwrap();

        let mut events = Vec::new();
        while let Some(event) = result.next().await {
            events.push(event);
        }

        // Check first event is StartSection with themes path
        assert!(matches!(&events[0],
            SlashCommandEvent::StartSection { label, .. } if label.starts_with("zed/assets/themes")));

        // Check we have the right number of sections (7 sections)
        let section_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(e, SlashCommandEvent::StartSection { .. }))
            .collect();
        assert_eq!(section_events.len(), 7);

        // Check content is included in the events
        let content_events: Vec<_> = events
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    SlashCommandEvent::Content(SlashCommandContentType::Text { .. })
                )
            })
            .collect();
        let content = content_events.iter().fold(String::new(), |mut acc, e| {
            if let SlashCommandEvent::Content(SlashCommandContentType::Text { text, .. }) = e {
                acc.push_str(text);
            }
            acc
        });

        assert!(content.contains("zed/assets/themes/andromeda/LICENSE"));
        assert!(content.contains("zed/assets/themes/ayu/LICENSE"));
        assert!(content.contains("zed/assets/themes/summercamp/LICENSE"));

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

        let mut result = cx
            .update(|cx| collect_files(project.clone(), &["zed/assets/themes".to_string()], cx))
            .await
            .unwrap();

        let mut events = Vec::new();
        while let Some(event) = result.next().await {
            events.push(event);
        }

        // Check content of events with pattern matching
        let mut i = 0;
        // Check we get all expected events
        let events_str = events
            .iter()
            .map(|e| match e {
                SlashCommandEvent::StartSection { label, .. } => format!("StartSection: {}", label),
                SlashCommandEvent::Content(SlashCommandContentType::Text { text, .. }) => {
                    format!("Content: {}", text)
                }
                SlashCommandEvent::Content(SlashCommandContentType::Image { .. }) => {
                    "Content: Image".to_string()
                }
                SlashCommandEvent::EndSection { .. } => "EndSection".to_string(),
                _ => "Unknown event".to_string(),
            })
            .collect::<Vec<_>>()
            .join("\n");

        for event in &events {
            match event {
                SlashCommandEvent::StartSection { label, .. } => {
                    match i {
                        0 => assert!(label.starts_with("zed/assets/themes")),
                        2 => assert!(label.starts_with("summercamp")),
                        4 => assert!(label.starts_with("subdir")),
                        6 => assert!(label.starts_with("subsubdir")),
                        _ => (),
                    }
                    i += 1;
                }
                SlashCommandEvent::Content(SlashCommandContentType::Text { text, .. }) => match i {
                    1 => assert!(
                        text.contains("zed/assets/themes"),
                        "Expected text to contain 'LICENSE' but got: {}",
                        text
                    ),
                    2 => assert!(
                        text.contains("LICENSE"),
                        "Expected text to contain 'LICENSE' but got: {}",
                        text
                    ),
                    4 => assert!(
                        text.contains("summercamp/LICENSE"),
                        "Expected text to contain 'summercamp/LICENSE' but got: {}",
                        text
                    ),
                    6 => assert!(
                        text.contains("subdir/LICENSE"),
                        "Expected text to contain 'subdir/LICENSE' but got: {}",
                        text
                    ),
                    8 => assert!(
                        text.contains("subsubdir/LICENSE"),
                        "Expected text to contain 'subsubdir/LICENSE' but got: {}",
                        text
                    ),
                    _ => (),
                },
                _ => (),
            }
        }

        drop(project);
    }
}
