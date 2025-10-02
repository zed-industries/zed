use super::{HoverTarget, HoveredWord, TerminalView};
use anyhow::{Context as _, Result};
use editor::Editor;
use gpui::{App, AppContext, Context, Task, WeakEntity, Window};
use itertools::Itertools;
use project::{Entry, Metadata};
use std::path::PathBuf;
use terminal::PathLikeTarget;
use util::{
    ResultExt, debug_panic,
    paths::{PathStyle, PathWithPosition},
    rel_path::RelPath,
};
use workspace::{OpenOptions, OpenVisible, Workspace};

/// The way we found the open target. This is important to have for test assertions.
/// For example, remote projects never look in the file system.
#[cfg(test)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum OpenTargetFoundBy {
    WorktreeExact,
    WorktreeScan,
    FileSystemBackground,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BackgroundFsChecks {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone)]
enum OpenTarget {
    Worktree(PathWithPosition, Entry, #[cfg(test)] OpenTargetFoundBy),
    File(PathWithPosition, Metadata),
}

impl OpenTarget {
    fn is_file(&self) -> bool {
        match self {
            OpenTarget::Worktree(_, entry, ..) => entry.is_file(),
            OpenTarget::File(_, metadata) => !metadata.is_dir,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            OpenTarget::Worktree(_, entry, ..) => entry.is_dir(),
            OpenTarget::File(_, metadata) => metadata.is_dir,
        }
    }

    fn path(&self) -> &PathWithPosition {
        match self {
            OpenTarget::Worktree(path, ..) => path,
            OpenTarget::File(path, _) => path,
        }
    }

    #[cfg(test)]
    fn found_by(&self) -> OpenTargetFoundBy {
        match self {
            OpenTarget::Worktree(.., found_by) => *found_by,
            OpenTarget::File(..) => OpenTargetFoundBy::FileSystemBackground,
        }
    }
}

pub(super) fn hover_path_like_target(
    workspace: &WeakEntity<Workspace>,
    hovered_word: HoveredWord,
    path_like_target: &PathLikeTarget,
    cx: &mut Context<TerminalView>,
) -> Task<()> {
    #[cfg(not(test))]
    {
        possible_hover_target(workspace, hovered_word, path_like_target, cx)
    }
    #[cfg(test)]
    {
        possible_hover_target(
            workspace,
            hovered_word,
            path_like_target,
            cx,
            BackgroundFsChecks::Enabled,
        )
    }
}

fn possible_hover_target(
    workspace: &WeakEntity<Workspace>,
    hovered_word: HoveredWord,
    path_like_target: &PathLikeTarget,
    cx: &mut Context<TerminalView>,
    #[cfg(test)] background_fs_checks: BackgroundFsChecks,
) -> Task<()> {
    let file_to_open_task = possible_open_target(
        workspace,
        path_like_target,
        cx,
        #[cfg(test)]
        background_fs_checks,
    );
    cx.spawn(async move |terminal_view, cx| {
        let file_to_open = file_to_open_task.await;
        terminal_view
            .update(cx, |terminal_view, _| match file_to_open {
                Some(OpenTarget::File(path, _) | OpenTarget::Worktree(path, ..)) => {
                    terminal_view.hover = Some(HoverTarget {
                        tooltip: path.to_string(|path| path.to_string_lossy().into_owned()),
                        hovered_word,
                    });
                }
                None => {
                    terminal_view.hover = None;
                }
            })
            .ok();
    })
}

fn possible_open_target(
    workspace: &WeakEntity<Workspace>,
    path_like_target: &PathLikeTarget,
    cx: &App,
    #[cfg(test)] background_fs_checks: BackgroundFsChecks,
) -> Task<Option<OpenTarget>> {
    let Some(workspace) = workspace.upgrade() else {
        return Task::ready(None);
    };
    // We have to check for both paths, as on Unix, certain paths with positions are valid file paths too.
    // We can be on FS remote part, without real FS, so cannot canonicalize or check for existence the path right away.
    let mut potential_paths = Vec::new();
    let cwd = path_like_target.terminal_dir.as_ref();
    let maybe_path = &path_like_target.maybe_path;
    let original_path = PathWithPosition::from_path(PathBuf::from(maybe_path));
    let path_with_position = PathWithPosition::parse_str(maybe_path);
    let worktree_candidates = workspace
        .read(cx)
        .worktrees(cx)
        .sorted_by_key(|worktree| {
            let worktree_root = worktree.read(cx).abs_path();
            match cwd.and_then(|cwd| worktree_root.strip_prefix(cwd).ok()) {
                Some(cwd_child) => cwd_child.components().count(),
                None => usize::MAX,
            }
        })
        .collect::<Vec<_>>();
    // Since we do not check paths via FS and joining, we need to strip off potential `./`, `a/`, `b/` prefixes out of it.
    const GIT_DIFF_PATH_PREFIXES: &[&str] = &["a", "b"];
    for prefix_str in GIT_DIFF_PATH_PREFIXES.iter().chain(std::iter::once(&".")) {
        if let Some(stripped) = original_path.path.strip_prefix(prefix_str).ok() {
            potential_paths.push(PathWithPosition {
                path: stripped.to_owned(),
                row: original_path.row,
                column: original_path.column,
            });
        }
        if let Some(stripped) = path_with_position.path.strip_prefix(prefix_str).ok() {
            potential_paths.push(PathWithPosition {
                path: stripped.to_owned(),
                row: path_with_position.row,
                column: path_with_position.column,
            });
        }
    }

    let insert_both_paths = original_path != path_with_position;
    potential_paths.insert(0, original_path);
    if insert_both_paths {
        potential_paths.insert(1, path_with_position);
    }

    // If we won't find paths "easily", we can traverse the entire worktree to look what ends with the potential path suffix.
    // That will be slow, though, so do the fast checks first.
    let mut worktree_paths_to_check = Vec::new();
    let mut is_cwd_in_worktree = false;
    let mut open_target = None;
    'worktree_loop: for worktree in &worktree_candidates {
        let worktree_root = worktree.read(cx).abs_path();
        let mut paths_to_check = Vec::with_capacity(potential_paths.len());
        let relative_cwd = cwd
            .and_then(|cwd| cwd.strip_prefix(&worktree_root).ok())
            .and_then(|cwd| RelPath::new(cwd, PathStyle::local()).ok())
            .and_then(|cwd_stripped| {
                (cwd_stripped.as_ref() != RelPath::empty()).then(|| {
                    is_cwd_in_worktree = true;
                    cwd_stripped
                })
            });

        for path_with_position in &potential_paths {
            let path_to_check = if worktree_root.ends_with(&path_with_position.path) {
                let root_path_with_position = PathWithPosition {
                    path: worktree_root.to_path_buf(),
                    row: path_with_position.row,
                    column: path_with_position.column,
                };
                match worktree.read(cx).root_entry() {
                    Some(root_entry) => {
                        open_target = Some(OpenTarget::Worktree(
                            root_path_with_position,
                            root_entry.clone(),
                            #[cfg(test)]
                            OpenTargetFoundBy::WorktreeExact,
                        ));
                        break 'worktree_loop;
                    }
                    None => root_path_with_position,
                }
            } else {
                PathWithPosition {
                    path: path_with_position
                        .path
                        .strip_prefix(&worktree_root)
                        .unwrap_or(&path_with_position.path)
                        .to_owned(),
                    row: path_with_position.row,
                    column: path_with_position.column,
                }
            };

            if let Ok(relative_path_to_check) =
                RelPath::new(&path_to_check.path, PathStyle::local())
                && !worktree.read(cx).is_single_file()
                && let Some(entry) = relative_cwd
                    .clone()
                    .and_then(|relative_cwd| {
                        worktree
                            .read(cx)
                            .entry_for_path(&relative_cwd.join(&relative_path_to_check))
                    })
                    .or_else(|| worktree.read(cx).entry_for_path(&relative_path_to_check))
            {
                open_target = Some(OpenTarget::Worktree(
                    PathWithPosition {
                        path: worktree.read(cx).absolutize(&entry.path),
                        row: path_to_check.row,
                        column: path_to_check.column,
                    },
                    entry.clone(),
                    #[cfg(test)]
                    OpenTargetFoundBy::WorktreeExact,
                ));
                break 'worktree_loop;
            }

            paths_to_check.push(path_to_check);
        }

        if !paths_to_check.is_empty() {
            worktree_paths_to_check.push((worktree.clone(), paths_to_check));
        }
    }

    #[cfg(not(test))]
    let enable_background_fs_checks = workspace.read(cx).project().read(cx).is_local();
    #[cfg(test)]
    let enable_background_fs_checks = background_fs_checks == BackgroundFsChecks::Enabled;

    if open_target.is_some() {
        // We we want to prefer open targets found via background fs checks over worktree matches,
        // however we can return early if either:
        //   - This is a remote project, or
        //   - If the terminal working directory is inside of at least one worktree
        if !enable_background_fs_checks || is_cwd_in_worktree {
            return Task::ready(open_target);
        }
    }

    // Before entire worktree traversal(s), make an attempt to do FS checks if available.
    let fs_paths_to_check =
        if enable_background_fs_checks {
            let fs_cwd_paths_to_check = cwd
                .iter()
                .flat_map(|cwd| {
                    let mut paths_to_check = Vec::new();
                    for path_to_check in &potential_paths {
                        let maybe_path = &path_to_check.path;
                        if path_to_check.path.is_relative() {
                            paths_to_check.push(PathWithPosition {
                                path: cwd.join(&maybe_path),
                                row: path_to_check.row,
                                column: path_to_check.column,
                            });
                        }
                    }
                    paths_to_check
                })
                .collect::<Vec<_>>();
            fs_cwd_paths_to_check
                .into_iter()
                .chain(
                    potential_paths
                        .into_iter()
                        .flat_map(|path_to_check| {
                            let mut paths_to_check = Vec::new();
                            let maybe_path = &path_to_check.path;
                            if maybe_path.starts_with("~") {
                                if let Some(home_path) = maybe_path.strip_prefix("~").ok().and_then(
                                    |stripped_maybe_path| {
                                        Some(dirs::home_dir()?.join(stripped_maybe_path))
                                    },
                                ) {
                                    paths_to_check.push(PathWithPosition {
                                        path: home_path,
                                        row: path_to_check.row,
                                        column: path_to_check.column,
                                    });
                                }
                            } else {
                                paths_to_check.push(PathWithPosition {
                                    path: maybe_path.clone(),
                                    row: path_to_check.row,
                                    column: path_to_check.column,
                                });
                                if maybe_path.is_relative() {
                                    for worktree in &worktree_candidates {
                                        if !worktree.read(cx).is_single_file() {
                                            paths_to_check.push(PathWithPosition {
                                                path: worktree.read(cx).abs_path().join(maybe_path),
                                                row: path_to_check.row,
                                                column: path_to_check.column,
                                            });
                                        }
                                    }
                                }
                            }
                            paths_to_check
                        })
                        .collect::<Vec<_>>(),
                )
                .collect()
        } else {
            Vec::new()
        };

    let fs = workspace.read(cx).project().read(cx).fs().clone();
    let background_fs_checks_task = cx.background_spawn(async move {
        for mut path_to_check in fs_paths_to_check {
            if let Some(fs_path_to_check) = fs.canonicalize(&path_to_check.path).await.ok()
                && let Some(metadata) = fs.metadata(&fs_path_to_check).await.ok().flatten()
            {
                if open_target
                    .as_ref()
                    .map(|open_target| open_target.path().path != fs_path_to_check)
                    .unwrap_or(true)
                {
                    path_to_check.path = fs_path_to_check;
                    return Some(OpenTarget::File(path_to_check, metadata));
                }

                break;
            }
        }

        open_target
    });

    cx.spawn(async move |cx| {
        background_fs_checks_task.await.or_else(|| {
            for (worktree, worktree_paths_to_check) in worktree_paths_to_check {
                let found_entry = worktree
                    .update(cx, |worktree, _| -> Option<OpenTarget> {
                        let traversal =
                            worktree.traverse_from_path(true, true, false, RelPath::empty());
                        for entry in traversal {
                            if let Some(path_in_worktree) =
                                worktree_paths_to_check.iter().find(|path_to_check| {
                                    RelPath::new(&path_to_check.path, PathStyle::local())
                                        .is_ok_and(|path| entry.path.ends_with(&path))
                                })
                            {
                                return Some(OpenTarget::Worktree(
                                    PathWithPosition {
                                        path: worktree.absolutize(&entry.path),
                                        row: path_in_worktree.row,
                                        column: path_in_worktree.column,
                                    },
                                    entry.clone(),
                                    #[cfg(test)]
                                    OpenTargetFoundBy::WorktreeScan,
                                ));
                            }
                        }
                        None
                    })
                    .ok()?;
                if let Some(found_entry) = found_entry {
                    return Some(found_entry);
                }
            }
            None
        })
    })
}

pub(super) fn open_path_like_target(
    workspace: &WeakEntity<Workspace>,
    terminal_view: &mut TerminalView,
    path_like_target: &PathLikeTarget,
    window: &mut Window,
    cx: &mut Context<TerminalView>,
) {
    #[cfg(not(test))]
    {
        possibly_open_target(workspace, terminal_view, path_like_target, window, cx)
            .detach_and_log_err(cx)
    }
    #[cfg(test)]
    {
        possibly_open_target(
            workspace,
            terminal_view,
            path_like_target,
            window,
            cx,
            BackgroundFsChecks::Enabled,
        )
        .detach_and_log_err(cx)
    }
}

fn possibly_open_target(
    workspace: &WeakEntity<Workspace>,
    terminal_view: &mut TerminalView,
    path_like_target: &PathLikeTarget,
    window: &mut Window,
    cx: &mut Context<TerminalView>,
    #[cfg(test)] background_fs_checks: BackgroundFsChecks,
) -> Task<Result<Option<OpenTarget>>> {
    if terminal_view.hover.is_none() {
        return Task::ready(Ok(None));
    }
    let workspace = workspace.clone();
    let path_like_target = path_like_target.clone();
    cx.spawn_in(window, async move |terminal_view, cx| {
        let Some(open_target) = terminal_view
            .update(cx, |_, cx| {
                possible_open_target(
                    &workspace,
                    &path_like_target,
                    cx,
                    #[cfg(test)]
                    background_fs_checks,
                )
            })?
            .await
        else {
            return Ok(None);
        };

        let path_to_open = open_target.path();
        let opened_items = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![path_to_open.path.clone()],
                    OpenOptions {
                        visible: Some(OpenVisible::OnlyDirectories),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .context("workspace update")?
            .await;
        if opened_items.len() != 1 {
            debug_panic!(
                "Received {} items for one path {path_to_open:?}",
                opened_items.len(),
            );
        }

        if let Some(opened_item) = opened_items.first() {
            if open_target.is_file() {
                if let Some(Ok(opened_item)) = opened_item {
                    if let Some(row) = path_to_open.row {
                        let col = path_to_open.column.unwrap_or(0);
                        if let Some(active_editor) = opened_item.downcast::<Editor>() {
                            active_editor
                                .downgrade()
                                .update_in(cx, |editor, window, cx| {
                                    editor.go_to_singleton_buffer_point(
                                        language::Point::new(
                                            row.saturating_sub(1),
                                            col.saturating_sub(1),
                                        ),
                                        window,
                                        cx,
                                    )
                                })
                                .log_err();
                        }
                    }
                    return Ok(Some(open_target));
                }
            } else if open_target.is_dir() {
                workspace.update(cx, |workspace, cx| {
                    workspace.project().update(cx, |_, cx| {
                        cx.emit(project::Event::ActivateProjectPanel);
                    })
                })?;
                return Ok(Some(open_target));
            }
        }
        Ok(None)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::Project;
    use serde_json::json;
    use std::path::{Path, PathBuf};
    use terminal::{HoveredWord, alacritty_terminal::index::Point as AlacPoint};
    use util::path;
    use workspace::AppState;

    async fn init_test(
        app_cx: &mut TestAppContext,
        trees: impl IntoIterator<Item = (&str, serde_json::Value)>,
        worktree_roots: impl IntoIterator<Item = &str>,
    ) -> impl AsyncFnMut(
        HoveredWord,
        PathLikeTarget,
        BackgroundFsChecks,
    ) -> (Option<HoverTarget>, Option<OpenTarget>) {
        let fs = app_cx.update(AppState::test).fs.as_fake().clone();

        app_cx.update(|cx| {
            terminal::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            Project::init_settings(cx);
            language::init(cx);
            editor::init(cx);
        });

        for (path, tree) in trees {
            fs.insert_tree(path, tree).await;
        }

        let project: gpui::Entity<Project> = Project::test(
            fs.clone(),
            worktree_roots.into_iter().map(Path::new),
            app_cx,
        )
        .await;

        let (workspace, cx) =
            app_cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let cwd = std::env::current_dir().expect("Failed to get working directory");
        let terminal = project
            .update(cx, |project: &mut Project, cx| {
                project.create_terminal_shell(Some(cwd), cx)
            })
            .await
            .expect("Failed to create a terminal");

        let workspace_a = workspace.clone();
        let (terminal_view, cx) = app_cx.add_window_view(|window, cx| {
            TerminalView::new(
                terminal,
                workspace_a.downgrade(),
                None,
                project.downgrade(),
                window,
                cx,
            )
        });

        async move |hovered_word: HoveredWord,
                    path_like_target: PathLikeTarget,
                    background_fs_checks: BackgroundFsChecks|
                    -> (Option<HoverTarget>, Option<OpenTarget>) {
            let workspace_a = workspace.clone();
            terminal_view
                .update(cx, |_, cx| {
                    possible_hover_target(
                        &workspace_a.downgrade(),
                        hovered_word,
                        &path_like_target,
                        cx,
                        background_fs_checks,
                    )
                })
                .await;

            let hover_target =
                terminal_view.read_with(cx, |terminal_view, _| terminal_view.hover.clone());

            let open_target = terminal_view
                .update_in(cx, |terminal_view, window, cx| {
                    possibly_open_target(
                        &workspace.downgrade(),
                        terminal_view,
                        &path_like_target,
                        window,
                        cx,
                        background_fs_checks,
                    )
                })
                .await
                .expect("Failed to possibly open target");

            (hover_target, open_target)
        }
    }

    async fn test_path_like_simple(
        test_path_like: &mut impl AsyncFnMut(
            HoveredWord,
            PathLikeTarget,
            BackgroundFsChecks,
        ) -> (Option<HoverTarget>, Option<OpenTarget>),
        maybe_path: &str,
        tooltip: &str,
        terminal_dir: Option<PathBuf>,
        background_fs_checks: BackgroundFsChecks,
        mut open_target_found_by: OpenTargetFoundBy,
        file: &str,
        line: u32,
    ) {
        let (hover_target, open_target) = test_path_like(
            HoveredWord {
                word: maybe_path.to_string(),
                word_match: AlacPoint::default()..=AlacPoint::default(),
                id: 0,
            },
            PathLikeTarget {
                maybe_path: maybe_path.to_string(),
                terminal_dir,
            },
            background_fs_checks,
        )
        .await;

        let Some(hover_target) = hover_target else {
            assert!(
                hover_target.is_some(),
                "Hover target should not be `None` at {file}:{line}:"
            );
            return;
        };

        assert_eq!(
            hover_target.tooltip, tooltip,
            "Tooltip mismatch at {file}:{line}:"
        );
        assert_eq!(
            hover_target.hovered_word.word, maybe_path,
            "Hovered word mismatch at {file}:{line}:"
        );

        let Some(open_target) = open_target else {
            assert!(
                open_target.is_some(),
                "Open target should not be `None` at {file}:{line}:"
            );
            return;
        };

        assert_eq!(
            open_target.path().path,
            Path::new(tooltip),
            "Open target path mismatch at {file}:{line}:"
        );

        if background_fs_checks == BackgroundFsChecks::Disabled
            && open_target_found_by == OpenTargetFoundBy::FileSystemBackground
        {
            open_target_found_by = OpenTargetFoundBy::WorktreeScan;
        }

        assert_eq!(
            open_target.found_by(),
            open_target_found_by,
            "Open target found by mismatch at {file}:{line}:"
        );
    }

    macro_rules! none_or_some_pathbuf {
        (None) => {
            None
        };
        ($cwd:literal) => {
            Some($crate::PathBuf::from(path!($cwd)))
        };
    }

    macro_rules! test_path_like {
        (
            $test_path_like:expr,
            $maybe_path:literal,
            $tooltip:literal,
            $cwd:tt,
            $found_by:expr
        ) => {{
            test_path_like!(
                $test_path_like,
                $maybe_path,
                $tooltip,
                $cwd,
                BackgroundFsChecks::Enabled,
                $found_by
            );
            test_path_like!(
                $test_path_like,
                $maybe_path,
                $tooltip,
                $cwd,
                BackgroundFsChecks::Disabled,
                $found_by
            );
        }};

        (
            $test_path_like:expr,
            $maybe_path:literal,
            $tooltip:literal,
            $cwd:tt,
            $background_fs_checks:path,
            $found_by:expr
        ) => {
            test_path_like_simple(
                &mut $test_path_like,
                path!($maybe_path),
                path!($tooltip),
                none_or_some_pathbuf!($cwd),
                $background_fs_checks,
                $found_by,
                std::file!(),
                std::line!(),
            )
            .await
        };
    }

    // Note the arms of `test`, `test_local`, and `test_remote` should be collapsed once macro
    // metavariable expressions (#![feature(macro_metavar_expr)]) are stabilized.
    // See https://github.com/rust-lang/rust/issues/83527
    #[doc = "test_path_likes!(<cx>, <trees>, <worktrees>, { $(<tests>;)+ })"]
    macro_rules! test_path_likes {
        ($cx:expr, $trees:expr, $worktrees:expr, { $($tests:expr;)+ }) => { {
            let mut test_path_like = init_test($cx, $trees, $worktrees).await;
            #[doc ="test!(<hovered maybe_path>, <expected tooltip>, <terminal cwd> "]
            #[doc ="\\[, found by \\])"]
            #[allow(unused_macros)]
            macro_rules! test {
                ($maybe_path:literal, $tooltip:literal, $cwd:tt) => {
                    test_path_like!(
                        test_path_like,
                        $maybe_path,
                        $tooltip,
                        $cwd,
                        OpenTargetFoundBy::WorktreeExact
                    )
                };
                ($maybe_path:literal, $tooltip:literal, $cwd:tt, $found_by:ident) => {
                    test_path_like!(
                        test_path_like,
                        $maybe_path,
                        $tooltip,
                        $cwd,
                        OpenTargetFoundBy::$found_by
                    )
                }
            }
            #[doc ="test_local!(<hovered maybe_path>, <expected tooltip>, <terminal cwd> "]
            #[doc ="\\[, found by \\])"]
            #[allow(unused_macros)]
            macro_rules! test_local {
                ($maybe_path:literal, $tooltip:literal, $cwd:tt) => {
                    test_path_like!(
                        test_path_like,
                        $maybe_path,
                        $tooltip,
                        $cwd,
                        BackgroundFsChecks::Enabled,
                        OpenTargetFoundBy::WorktreeExact
                    )
                };
                ($maybe_path:literal, $tooltip:literal, $cwd:tt, $found_by:ident) => {
                    test_path_like!(
                        test_path_like,
                        $maybe_path,
                        $tooltip,
                        $cwd,
                        BackgroundFsChecks::Enabled,
                        OpenTargetFoundBy::$found_by
                    )
                }
            }
            #[doc ="test_remote!(<hovered maybe_path>, <expected tooltip>, <terminal cwd> "]
            #[doc ="\\[, found by \\])"]
            #[allow(unused_macros)]
            macro_rules! test_remote {
                ($maybe_path:literal, $tooltip:literal, $cwd:tt) => {
                    test_path_like!(
                        test_path_like,
                        $maybe_path,
                        $tooltip,
                        $cwd,
                        BackgroundFsChecks::Disabled,
                        OpenTargetFoundBy::WorktreeExact
                    )
                };
                ($maybe_path:literal, $tooltip:literal, $cwd:tt, $found_by:ident) => {
                    test_path_like!(
                        test_path_like,
                        $maybe_path,
                        $tooltip,
                        $cwd,
                        BackgroundFsChecks::Disabled,
                        OpenTargetFoundBy::$found_by
                    )
                }
            }
            $($tests);+
        } }
    }

    #[gpui::test]
    async fn one_folder_worktree(cx: &mut TestAppContext) {
        test_path_likes!(
            cx,
            vec![(
                path!("/test"),
                json!({
                    "lib.rs": "",
                    "test.rs": "",
                }),
            )],
            vec![path!("/test")],
            {
                test!("lib.rs", "/test/lib.rs", None);
                test!("/test/lib.rs", "/test/lib.rs", None);
                test!("test.rs", "/test/test.rs", None);
                test!("/test/test.rs", "/test/test.rs", None);
            }
        )
    }

    #[gpui::test]
    async fn mixed_worktrees(cx: &mut TestAppContext) {
        test_path_likes!(
            cx,
            vec![
                (
                    path!("/"),
                    json!({
                        "file.txt": "",
                    }),
                ),
                (
                    path!("/test"),
                    json!({
                        "lib.rs": "",
                        "test.rs": "",
                        "file.txt": "",
                    }),
                ),
            ],
            vec![path!("/file.txt"), path!("/test")],
            {
                test!("file.txt", "/file.txt", "/");
                test!("/file.txt", "/file.txt", "/");

                test!("lib.rs", "/test/lib.rs", "/test");
                test!("test.rs", "/test/test.rs", "/test");
                test!("file.txt", "/test/file.txt", "/test");

                test!("/test/lib.rs", "/test/lib.rs", "/test");
                test!("/test/test.rs", "/test/test.rs", "/test");
                test!("/test/file.txt", "/test/file.txt", "/test");
            }
        )
    }

    #[gpui::test]
    async fn worktree_file_preferred(cx: &mut TestAppContext) {
        test_path_likes!(
            cx,
            vec![
                (
                    path!("/"),
                    json!({
                        "file.txt": "",
                    }),
                ),
                (
                    path!("/test"),
                    json!({
                        "file.txt": "",
                    }),
                ),
            ],
            vec![path!("/test")],
            {
                test!("file.txt", "/test/file.txt", "/test");
            }
        )
    }

    mod issues {
        use super::*;

        // https://github.com/zed-industries/zed/issues/28407
        #[gpui::test]
        async fn issue_28407_siblings(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/dir1"),
                    json!({
                        "dir 2": {
                            "C.py": ""
                        },
                        "dir 3": {
                            "C.py": ""
                        },
                    }),
                )],
                vec![path!("/dir1")],
                {
                    test!("C.py", "/dir1/dir 2/C.py", "/dir1", WorktreeScan);
                    test!("C.py", "/dir1/dir 2/C.py", "/dir1/dir 2");
                    test!("C.py", "/dir1/dir 3/C.py", "/dir1/dir 3");
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/28407
        // See https://github.com/zed-industries/zed/issues/34027
        // See https://github.com/zed-industries/zed/issues/33498
        #[gpui::test]
        async fn issue_28407_nesting(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/project"),
                    json!({
                        "lib": {
                            "src": {
                                "main.rs": "",
                                "only_in_lib.rs": ""
                            },
                        },
                        "src": {
                            "main.rs": ""
                        },
                    }),
                )],
                vec![path!("/project")],
                {
                    test!("main.rs", "/project/src/main.rs", "/project/src");
                    test!("main.rs", "/project/lib/src/main.rs", "/project/lib/src");

                    test!("src/main.rs", "/project/src/main.rs", "/project");
                    test!("src/main.rs", "/project/src/main.rs", "/project/src");
                    test!("src/main.rs", "/project/lib/src/main.rs", "/project/lib");

                    test!("lib/src/main.rs", "/project/lib/src/main.rs", "/project");
                    test!(
                        "lib/src/main.rs",
                        "/project/lib/src/main.rs",
                        "/project/src"
                    );
                    test!(
                        "lib/src/main.rs",
                        "/project/lib/src/main.rs",
                        "/project/lib"
                    );
                    test!(
                        "lib/src/main.rs",
                        "/project/lib/src/main.rs",
                        "/project/lib/src"
                    );
                    test!(
                        "src/only_in_lib.rs",
                        "/project/lib/src/only_in_lib.rs",
                        "/project/lib/src",
                        WorktreeScan
                    );
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/28339
        // Note: These could all be found by WorktreeExact if we used
        // `fs::normalize_path(&maybe_path)`
        #[gpui::test]
        async fn issue_28339(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/tmp"),
                    json!({
                        "issue28339": {
                            "foo": {
                                "bar.txt": ""
                            },
                        },
                    }),
                )],
                vec![path!("/tmp")],
                {
                    test_local!(
                        "foo/./bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339",
                        WorktreeExact
                    );
                    test_local!(
                        "foo/../foo/bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339",
                        WorktreeExact
                    );
                    test_local!(
                        "foo/..///foo/bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339",
                        WorktreeExact
                    );
                    test_local!(
                        "issue28339/../issue28339/foo/../foo/bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339",
                        WorktreeExact
                    );
                    test_local!(
                        "./bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339/foo",
                        WorktreeExact
                    );
                    test_local!(
                        "../foo/bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339/foo",
                        FileSystemBackground
                    );
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/28339
        // Note: These could all be found by WorktreeExact if we used
        // `fs::normalize_path(&maybe_path)`
        #[gpui::test]
        #[should_panic(expected = "Hover target should not be `None`")]
        async fn issue_28339_remote(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/tmp"),
                    json!({
                        "issue28339": {
                            "foo": {
                                "bar.txt": ""
                            },
                        },
                    }),
                )],
                vec![path!("/tmp")],
                {
                    test_remote!(
                        "foo/./bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339"
                    );
                    test_remote!(
                        "foo/../foo/bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339"
                    );
                    test_remote!(
                        "foo/..///foo/bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339"
                    );
                    test_remote!(
                        "issue28339/../issue28339/foo/../foo/bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339"
                    );
                    test_remote!(
                        "./bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339/foo"
                    );
                    test_remote!(
                        "../foo/bar.txt",
                        "/tmp/issue28339/foo/bar.txt",
                        "/tmp/issue28339/foo"
                    );
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/34027
        #[gpui::test]
        async fn issue_34027(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/tmp/issue34027"),
                    json!({
                        "test.txt": "",
                        "foo": {
                            "test.txt": "",
                        }
                    }),
                ),],
                vec![path!("/tmp/issue34027")],
                {
                    test!("test.txt", "/tmp/issue34027/test.txt", "/tmp/issue34027");
                    test!(
                        "test.txt",
                        "/tmp/issue34027/foo/test.txt",
                        "/tmp/issue34027/foo"
                    );
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/34027
        #[gpui::test]
        async fn issue_34027_siblings(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/test"),
                    json!({
                        "sub1": {
                            "file.txt": "",
                        },
                        "sub2": {
                            "file.txt": "",
                        }
                    }),
                ),],
                vec![path!("/test")],
                {
                    test!("file.txt", "/test/sub1/file.txt", "/test/sub1");
                    test!("file.txt", "/test/sub2/file.txt", "/test/sub2");
                    test!("sub1/file.txt", "/test/sub1/file.txt", "/test/sub1");
                    test!("sub2/file.txt", "/test/sub2/file.txt", "/test/sub2");
                    test!("sub1/file.txt", "/test/sub1/file.txt", "/test/sub2");
                    test!("sub2/file.txt", "/test/sub2/file.txt", "/test/sub1");
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/34027
        #[gpui::test]
        async fn issue_34027_nesting(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/test"),
                    json!({
                        "sub1": {
                            "file.txt": "",
                            "subsub1": {
                                "file.txt": "",
                            }
                        },
                        "sub2": {
                            "file.txt": "",
                            "subsub1": {
                                "file.txt": "",
                            }
                        }
                    }),
                ),],
                vec![path!("/test")],
                {
                    test!(
                        "file.txt",
                        "/test/sub1/subsub1/file.txt",
                        "/test/sub1/subsub1"
                    );
                    test!(
                        "file.txt",
                        "/test/sub2/subsub1/file.txt",
                        "/test/sub2/subsub1"
                    );
                    test!(
                        "subsub1/file.txt",
                        "/test/sub1/subsub1/file.txt",
                        "/test",
                        WorktreeScan
                    );
                    test!(
                        "subsub1/file.txt",
                        "/test/sub1/subsub1/file.txt",
                        "/test",
                        WorktreeScan
                    );
                    test!(
                        "subsub1/file.txt",
                        "/test/sub1/subsub1/file.txt",
                        "/test/sub1"
                    );
                    test!(
                        "subsub1/file.txt",
                        "/test/sub2/subsub1/file.txt",
                        "/test/sub2"
                    );
                    test!(
                        "subsub1/file.txt",
                        "/test/sub1/subsub1/file.txt",
                        "/test/sub1/subsub1",
                        WorktreeScan
                    );
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/34027
        #[gpui::test]
        async fn issue_34027_non_worktree_local_file(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![
                    (
                        path!("/"),
                        json!({
                            "file.txt": "",
                        }),
                    ),
                    (
                        path!("/test"),
                        json!({
                            "file.txt": "",
                        }),
                    ),
                ],
                vec![path!("/test")],
                {
                    // Note: Opening a non-worktree file adds that file as a single file worktree.
                    test_local!("file.txt", "/file.txt", "/", FileSystemBackground);
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/34027
        #[gpui::test]
        async fn issue_34027_non_worktree_remote_file(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![
                    (
                        path!("/"),
                        json!({
                            "file.txt": "",
                        }),
                    ),
                    (
                        path!("/test"),
                        json!({
                            "file.txt": "",
                        }),
                    ),
                ],
                vec![path!("/test")],
                {
                    // Note: Opening a non-worktree file adds that file as a single file worktree.
                    test_remote!("file.txt", "/test/file.txt", "/");
                    test_remote!("/test/file.txt", "/test/file.txt", "/");
                }
            )
        }

        // See https://github.com/zed-industries/zed/issues/34027
        #[gpui::test]
        #[should_panic(expected = "Tooltip mismatch")]
        async fn issue_34027_gaps(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/project"),
                    json!({
                        "lib": {
                            "src": {
                                "main.rs": ""
                            },
                        },
                        "src": {
                            "main.rs": ""
                        },
                    }),
                )],
                vec![path!("/project")],
                {
                    test!("main.rs", "/project/src/main.rs", "/project");
                    test!("main.rs", "/project/lib/src/main.rs", "/project/lib");
                }
            )
        }

        // See https://github.com/zed-industries/zed/issues/34027
        #[gpui::test]
        #[should_panic(expected = "Tooltip mismatch")]
        async fn issue_34027_overlap(cx: &mut TestAppContext) {
            test_path_likes!(
                cx,
                vec![(
                    path!("/project"),
                    json!({
                        "lib": {
                            "src": {
                                "main.rs": ""
                            },
                        },
                        "src": {
                            "main.rs": ""
                        },
                    }),
                )],
                vec![path!("/project")],
                {
                    // Finds "/project/src/main.rs"
                    test!(
                        "src/main.rs",
                        "/project/lib/src/main.rs",
                        "/project/lib/src"
                    );
                }
            )
        }
    }
}
