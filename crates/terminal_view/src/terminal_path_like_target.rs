use super::{HoverTarget, HoveredWord, TerminalView};
use anyhow::{Context as _, Result};
use editor::Editor;
use gpui::{App, AppContext, Context, Task, WeakEntity, Window};
use itertools::Itertools;
use project::{Entry, Metadata};
use std::path::PathBuf;
use terminal::PathLikeTarget;
use util::{ResultExt, debug_panic, paths::PathWithPosition};
use workspace::{OpenOptions, OpenVisible, Workspace};

#[derive(Debug, Clone)]
enum OpenTarget {
    Worktree(PathWithPosition, Entry),
    File(PathWithPosition, Metadata),
}

impl OpenTarget {
    fn is_file(&self) -> bool {
        match self {
            OpenTarget::Worktree(_, entry) => entry.is_file(),
            OpenTarget::File(_, metadata) => !metadata.is_dir,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            OpenTarget::Worktree(_, entry) => entry.is_dir(),
            OpenTarget::File(_, metadata) => metadata.is_dir,
        }
    }

    fn path(&self) -> &PathWithPosition {
        match self {
            OpenTarget::Worktree(path, _) => path,
            OpenTarget::File(path, _) => path,
        }
    }
}

pub(super) fn hover_path_like_target(
    workspace: &WeakEntity<Workspace>,
    hovered_word: HoveredWord,
    path_like_target: &PathLikeTarget,
    cx: &mut Context<TerminalView>,
) -> Task<()> {
    let file_to_open_task = possible_open_target(&workspace, &path_like_target, cx);
    cx.spawn(async move |terminal_view, cx| {
        let file_to_open = file_to_open_task.await;
        terminal_view
            .update(cx, |terminal_view, _| match file_to_open {
                Some(OpenTarget::File(path, _) | OpenTarget::Worktree(path, _)) => {
                    terminal_view.hover = Some(HoverTarget {
                        tooltip: path.to_string(|path| path.to_string_lossy().to_string()),
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
    for worktree in &worktree_candidates {
        let worktree_root = worktree.read(cx).abs_path();
        let mut paths_to_check = Vec::with_capacity(potential_paths.len());

        for path_with_position in &potential_paths {
            let path_to_check = if worktree_root.ends_with(&path_with_position.path) {
                let root_path_with_position = PathWithPosition {
                    path: worktree_root.to_path_buf(),
                    row: path_with_position.row,
                    column: path_with_position.column,
                };
                match worktree.read(cx).root_entry() {
                    Some(root_entry) => {
                        return Task::ready(Some(OpenTarget::Worktree(
                            root_path_with_position,
                            root_entry.clone(),
                        )));
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

            if path_to_check.path.is_relative() {
                if let Some(entry) = worktree.read(cx).entry_for_path(&path_to_check.path) {
                    return Task::ready(Some(OpenTarget::Worktree(
                        PathWithPosition {
                            path: worktree_root.join(&entry.path),
                            row: path_to_check.row,
                            column: path_to_check.column,
                        },
                        entry.clone(),
                    )));
                }
            }

            paths_to_check.push(path_to_check);
        }

        if !paths_to_check.is_empty() {
            worktree_paths_to_check.push((worktree.clone(), paths_to_check));
        }
    }

    // Before entire worktree traversal(s), make an attempt to do FS checks if available.
    let fs_paths_to_check = if workspace.read(cx).project().read(cx).is_local() {
        potential_paths
            .into_iter()
            .flat_map(|path_to_check| {
                let mut paths_to_check = Vec::new();
                let maybe_path = &path_to_check.path;
                if maybe_path.starts_with("~") {
                    if let Some(home_path) =
                        maybe_path
                            .strip_prefix("~")
                            .ok()
                            .and_then(|stripped_maybe_path| {
                                Some(dirs::home_dir()?.join(stripped_maybe_path))
                            })
                    {
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
                        if let Some(cwd) = &cwd {
                            paths_to_check.push(PathWithPosition {
                                path: cwd.join(maybe_path),
                                row: path_to_check.row,
                                column: path_to_check.column,
                            });
                        }
                        for worktree in &worktree_candidates {
                            paths_to_check.push(PathWithPosition {
                                path: worktree.read(cx).abs_path().join(maybe_path),
                                row: path_to_check.row,
                                column: path_to_check.column,
                            });
                        }
                    }
                }
                paths_to_check
            })
            .collect()
    } else {
        Vec::new()
    };

    let worktree_check_task = cx.spawn(async move |cx| {
        for (worktree, worktree_paths_to_check) in worktree_paths_to_check {
            let found_entry = worktree
                .update(cx, |worktree, _| {
                    let worktree_root = worktree.abs_path();
                    let mut traversal = worktree.traverse_from_path(true, true, false, "".as_ref());
                    while let Some(entry) = traversal.next() {
                        if let Some(path_in_worktree) = worktree_paths_to_check
                            .iter()
                            .find(|path_to_check| entry.path.ends_with(&path_to_check.path))
                        {
                            return Some(OpenTarget::Worktree(
                                PathWithPosition {
                                    path: worktree_root.join(&entry.path),
                                    row: path_in_worktree.row,
                                    column: path_in_worktree.column,
                                },
                                entry.clone(),
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
    });

    let fs = workspace.read(cx).project().read(cx).fs().clone();
    cx.background_spawn(async move {
        for mut path_to_check in fs_paths_to_check {
            if let Some(fs_path_to_check) = fs.canonicalize(&path_to_check.path).await.ok() {
                if let Some(metadata) = fs.metadata(&fs_path_to_check).await.ok().flatten() {
                    path_to_check.path = fs_path_to_check;
                    return Some(OpenTarget::File(path_to_check, metadata));
                }
            }
        }

        worktree_check_task.await
    })
}

pub(super) fn open_path_like_target(
    workspace: &WeakEntity<Workspace>,
    terminal_view: &mut TerminalView,
    path_like_target: &PathLikeTarget,
    window: &mut Window,
    cx: &mut Context<TerminalView>,
) {
    possibly_open_target(workspace, terminal_view, path_like_target, window, cx)
        .detach_and_log_err(cx)
}

fn possibly_open_target(
    workspace: &WeakEntity<Workspace>,
    terminal_view: &mut TerminalView,
    path_like_target: &PathLikeTarget,
    window: &mut Window,
    cx: &mut Context<TerminalView>,
) -> Task<Result<Option<OpenTarget>>> {
    if terminal_view.hover.is_none() {
        return Task::ready(Ok(None));
    }
    let workspace = workspace.clone();
    let path_like_target = path_like_target.clone();
    cx.spawn_in(window, async move |terminal_view, cx| {
        let Some(open_target) = terminal_view
            .update(cx, |_, cx| {
                possible_open_target(&workspace, &path_like_target, cx)
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
        return Ok(None);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::{Project, terminals::TerminalKind};
    use serde_json::json;
    use std::path::{Path, PathBuf};
    use terminal::{HoveredWord, alacritty_terminal::index::Point as AlacPoint};
    use util::path;
    use workspace::AppState;

    async fn init_test(
        app_cx: &mut TestAppContext,
        trees: impl IntoIterator<Item = (&str, serde_json::Value)>,
        worktree_roots: impl IntoIterator<Item = &str>,
    ) -> impl AsyncFnMut(HoveredWord, PathLikeTarget) -> (Option<HoverTarget>, Option<OpenTarget>)
    {
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

        let project = Project::test(
            fs.clone(),
            worktree_roots
                .into_iter()
                .map(Path::new)
                .collect::<Vec<_>>(),
            app_cx,
        )
        .await;

        let (workspace, cx) =
            app_cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let terminal = project
            .update_in(cx, |project, window, cx| {
                project.create_terminal(TerminalKind::Shell(None), window.window_handle(), cx)
            })
            .await
            .expect("Failed to create a terminal");

        let (terminal_a, workspace_a) = (terminal.clone(), workspace.clone());
        let (terminal_view, cx) = app_cx.add_window_view(|window, cx| {
            TerminalView::new(
                terminal_a,
                workspace_a.downgrade(),
                None,
                project.downgrade(),
                window,
                cx,
            )
        });

        async move |hovered_word: HoveredWord,
                    path_like_target: PathLikeTarget|
                    -> (Option<HoverTarget>, Option<OpenTarget>) {
            let workspace_a = workspace.clone();
            terminal_view
                .update(cx, |_, cx| {
                    hover_path_like_target(
                        &workspace_a.downgrade(),
                        hovered_word,
                        &path_like_target,
                        cx,
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
        ) -> (Option<HoverTarget>, Option<OpenTarget>),
        maybe_path: &str,
        tooltip: &str,
        terminal_dir: Option<PathBuf>,
        source_location: &str,
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
        )
        .await;

        let hover_target = hover_target.expect("Hover target should not be None");

        assert_eq!(
            hover_target.tooltip, tooltip,
            "Tooltips mismatch\n     at {source_location}:\n"
        );
        assert_eq!(
            hover_target.hovered_word.word, maybe_path,
            "Hovered word mismatch\n     at {source_location}:\n"
        );

        let open_target = open_target.expect("Open target should not be None");
        match open_target {
            OpenTarget::File(path, _) | OpenTarget::Worktree(path, _) => {
                assert_eq!(path.path.as_path(), Path::new(tooltip));
            }
        }
    }

    macro_rules! test_path_like {
        ($test_path_like:expr, $maybe_path:literal, $tooltip:literal) => {
            test_path_like_simple(
                &mut $test_path_like,
                path!($maybe_path),
                path!($tooltip),
                None,
                &format!("{}:{}", std::file!(), std::line!()),
            )
            .await
        };

        ($test_path_like:expr, $maybe_path:literal, $tooltip:literal, $cwd:literal) => {
            test_path_like_simple(
                &mut $test_path_like,
                path!($maybe_path),
                path!($tooltip),
                Some($crate::PathBuf::from(path!($cwd))),
                &format!("{}:{}", std::file!(), std::line!()),
            )
            .await
        };
    }

    #[gpui::test]
    async fn one_folder_worktree(cx: &mut TestAppContext) {
        let mut test_path_like = init_test(
            cx,
            vec![(
                path!("/test"),
                json!({
                    "lib.rs": "",
                    "test.rs": "",
                }),
            )],
            vec![path!("/test")],
        )
        .await;

        macro_rules! test { ($($args:tt),+) => { test_path_like!(test_path_like, $($args),+) } }

        test!("lib.rs", "/test/lib.rs");
        test!("test.rs", "/test/test.rs");
    }

    #[gpui::test]
    async fn mixed_worktrees(cx: &mut TestAppContext) {
        let mut test_path_like = init_test(
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
        )
        .await;

        macro_rules! test { ($($args:tt),+) => { test_path_like!(test_path_like, $($args),+) } }

        test!("file.txt", "/file.txt", "/");
        test!("lib.rs", "/test/lib.rs", "/test");
        test!("test.rs", "/test/test.rs", "/test");
        test!("file.txt", "/test/file.txt", "/test");
    }

    #[gpui::test]
    async fn worktree_file_preferred(cx: &mut TestAppContext) {
        let mut test_path_like = init_test(
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
        )
        .await;

        macro_rules! test { ($($args:tt),+) => { test_path_like!(test_path_like, $($args),+) } }

        test!("file.txt", "/test/file.txt", "/test");
    }

    mod issues {
        use super::*;

        // https://github.com/zed-industries/zed/issues/28407
        #[gpui::test]
        async fn issue_28407_siblings(cx: &mut TestAppContext) {
            let mut test_path_like = init_test(
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
            )
            .await;

            macro_rules! test { ($($args:tt),+) => { test_path_like!(test_path_like, $($args),+) } }

            test!("C.py", "/dir1/dir 2/C.py", "/dir1");
            test!("C.py", "/dir1/dir 2/C.py", "/dir1/dir 2");
            test!("C.py", "/dir1/dir 3/C.py", "/dir1/dir 3");
        }

        // https://github.com/zed-industries/zed/issues/28407
        // Note, #28407 is marked as closed, but three of the below cases are still failing.
        // See https://github.com/zed-industries/zed/issues/34027
        // See https://github.com/zed-industries/zed/issues/33498
        #[gpui::test]
        #[should_panic(expected = "Tooltips mismatch")]
        async fn issue_28407_nesting(cx: &mut TestAppContext) {
            let mut test_path_like = init_test(
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
            )
            .await;

            macro_rules! test { ($($args:tt),+) => { test_path_like!(test_path_like, $($args),+) } }

            // Failing currently
            test!("main.rs", "/project/src/main.rs", "/project");
            test!("main.rs", "/project/src/main.rs", "/project/src");
            test!("main.rs", "/project/lib/src/main.rs", "/project/lib");
            test!("main.rs", "/project/lib/src/main.rs", "/project/lib/src");

            test!("src/main.rs", "/project/src/main.rs", "/project");
            test!("src/main.rs", "/project/src/main.rs", "/project/src");
            // Failing currently
            test!("src/main.rs", "/project/lib/src/main.rs", "/project/lib");
            // Failing currently
            test!(
                "src/main.rs",
                "/project/lib/src/main.rs",
                "/project/lib/src"
            );

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
        }

        // https://github.com/zed-industries/zed/issues/28339
        #[gpui::test]
        async fn issue_28339(cx: &mut TestAppContext) {
            let mut test_path_like = init_test(
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
            )
            .await;

            macro_rules! test { ($($args:tt),+) => { test_path_like!(test_path_like, $($args),+) } }

            test!(
                "foo/./bar.txt",
                "/tmp/issue28339/foo/bar.txt",
                "/tmp/issue28339"
            );
            test!(
                "foo/../foo/bar.txt",
                "/tmp/issue28339/foo/bar.txt",
                "/tmp/issue28339"
            );
            test!(
                "foo/..///foo/bar.txt",
                "/tmp/issue28339/foo/bar.txt",
                "/tmp/issue28339"
            );
            test!(
                "issue28339/../issue28339/foo/../foo/bar.txt",
                "/tmp/issue28339/foo/bar.txt",
                "/tmp/issue28339"
            );
            test!(
                "./bar.txt",
                "/tmp/issue28339/foo/bar.txt",
                "/tmp/issue28339/foo"
            );
            test!(
                "../foo/bar.txt",
                "/tmp/issue28339/foo/bar.txt",
                "/tmp/issue28339/foo"
            );
        }

        // https://github.com/zed-industries/zed/issues/34027
        #[gpui::test]
        #[should_panic(expected = "Tooltips mismatch")]
        async fn issue_34027_non_worktree_file(cx: &mut TestAppContext) {
            let mut test_path_like = init_test(
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
            )
            .await;

            macro_rules! test { ($($args:tt),+) => { test_path_like!(test_path_like, $($args),+) } }

            test!("file.txt", "/file.txt", "/");
        }
    }
}
