use super::{HoverTarget, HoveredWord, TerminalView};
use anyhow::{Context as _, Result};
use editor::Editor;
use gpui::{Context, Task, TaskExt, WeakEntity, Window};
use std::path::PathBuf;
use terminal::PathLikeTarget;
use util::{ResultExt, debug_panic};
#[cfg(not(test))]
use workspace::path_link::possible_open_target;
#[cfg(test)]
use workspace::path_link::{
    BackgroundFsChecks, OpenTargetFoundBy, possible_open_target_with_fs_checks,
};
use workspace::{OpenOptions, OpenVisible, Workspace, path_link::OpenTarget};

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
    #[cfg(not(test))]
    let file_to_open_task = possible_open_target(
        workspace,
        &path_like_target.maybe_path,
        path_like_target.terminal_dir.as_deref(),
        cx,
    );
    #[cfg(test)]
    let file_to_open_task = possible_open_target_with_fs_checks(
        workspace,
        &path_like_target.maybe_path,
        path_like_target.terminal_dir.as_deref(),
        cx,
        background_fs_checks,
    );
    cx.spawn(async move |terminal_view, cx| {
        let file_to_open = file_to_open_task.await;
        terminal_view
            .update(cx, |terminal_view, _| match file_to_open {
                Some(OpenTarget::File(path, _) | OpenTarget::Worktree(path, ..)) => {
                    terminal_view.hover = Some(HoverTarget {
                        tooltip: path
                            .to_string(&|path: &PathBuf| path.to_string_lossy().into_owned()),
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
                #[cfg(not(test))]
                {
                    possible_open_target(
                        &workspace,
                        &path_like_target.maybe_path,
                        path_like_target.terminal_dir.as_deref(),
                        cx,
                    )
                }
                #[cfg(test)]
                {
                    possible_open_target_with_fs_checks(
                        &workspace,
                        &path_like_target.maybe_path,
                        path_like_target.terminal_dir.as_deref(),
                        cx,
                        background_fs_checks,
                    )
                }
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
    use gpui::{AppContext as _, TestAppContext};
    use project::Project;
    use serde_json::json;
    use std::path::{Path, PathBuf};
    use terminal::{
        HoveredWord, Point, Range, TerminalBuilder,
        terminal_settings::{AlternateScroll, CursorShape},
    };
    use util::path;
    use util::paths::PathStyle;
    use workspace::{AppState, MultiWorkspace};

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
            theme_settings::init(theme::LoadThemes::JustBase, cx);
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

        let (multi_workspace, cx) = app_cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let terminal = app_cx.new(|cx| {
            TerminalBuilder::new_display_only(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .subscribe(cx)
        });

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
                word_match: Range::new(Point::new(0, 0), Point::new(0, 0)),
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
                        WorktreeExact
                    );
                }
            )
        }

        // https://github.com/zed-industries/zed/issues/28339
        #[gpui::test]
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
