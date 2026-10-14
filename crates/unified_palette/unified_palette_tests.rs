#![cfg(test)]

use super::*;
use gpui::{TestAppContext, UpdateGlobal};
use menu::Confirm;
use project::Project;
use serde_json::json;
use settings::SettingsStore;
use std::sync::Arc;
use util::{path, rel_path::rel_path};
use workspace::{AppState, MultiWorkspace};

#[test]
fn mode_from_query_strips_the_prefix() {
    assert_eq!(mode_from_query(">run"), Some((Mode::CommandPalette, "run")));
    assert_eq!(
        mode_from_query("#symbol"),
        Some((Mode::ProjectSymbols, "symbol"))
    );
    assert_eq!(
        mode_from_query("@outline"),
        Some((Mode::Outline, "outline"))
    );
    assert_eq!(mode_from_query(":42"), Some((Mode::GoToLine, "42")));
    assert_eq!(
        mode_from_query("%search"),
        Some((Mode::TextFinder, "search"))
    );
    assert_eq!(mode_from_query("?"), Some((Mode::Help, "")));
    assert_eq!(mode_from_query("file.rs"), None);
}

#[gpui::test]
async fn disabled_unified_palette_keeps_file_prefixes_and_modal_shortcuts_plain(
    cx: &mut TestAppContext,
) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        file_finder::init(cx);
        command_palette::init(cx);
        project_symbols::init(cx);
        set_unified_palette_enabled(cx, false);
    });
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query("#symbol", window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        file_finder.read_with(cx, |file_finder, cx| file_finder.query(cx)),
        "#symbol"
    );

    workspace.update_in(cx, |workspace, window, cx| {
        workspace.hide_modal(window, cx);
    });
    cx.dispatch_action(zed_actions::command_palette::Toggle);
    cx.run_until_parked();
    let command_palette = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<command_palette::CommandPalette>(cx)
            .unwrap()
    });
    assert_eq!(
        command_palette.read_with(cx, |command_palette, cx| {
            command_palette.query_with_mode_prefix(cx)
        }),
        ""
    );

    workspace.update_in(cx, |workspace, window, cx| {
        workspace.hide_modal(window, cx);
    });
    cx.dispatch_action(workspace::ToggleProjectSymbols);
    cx.run_until_parked();
    let picker = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<picker::Picker<project_symbols::ProjectSymbolsDelegate>>(cx)
            .unwrap()
    });
    assert_eq!(picker.read_with(cx, |picker, cx| picker.query(cx)), "");
}

#[gpui::test]
async fn command_shortcut_opens_existing_command_palette(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        command_palette::init(cx);
    });
    cx.update(|cx| assert!(UnifiedPaletteSettings::get(None, cx).enabled));
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(zed_actions::command_palette::Toggle);
    cx.run_until_parked();
    let command_palette = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<command_palette::CommandPalette>(cx)
            .unwrap()
    });
    assert_eq!(
        command_palette.read_with(cx, |command_palette, cx| {
            command_palette.query_with_mode_prefix(cx)
        }),
        ">"
    );

    cx.dispatch_action(zed_actions::command_palette::Toggle);
    cx.run_until_parked();
    assert!(workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<command_palette::CommandPalette>(cx)
            .is_none()
    }));
}

#[gpui::test]
async fn switching_from_regular_text_finder_does_not_add_a_mode_prefix(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        project_symbols::init(cx);
        set_unified_palette_enabled(cx, false);
        search::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(zed_actions::text_finder::Toggle);
    cx.run_until_parked();
    cx.dispatch_action(workspace::ToggleProjectSymbols);
    cx.run_until_parked();

    workspace.read_with(cx, |workspace, cx| {
        let picker = workspace
            .active_modal::<picker::Picker<project_symbols::ProjectSymbolsDelegate>>(cx)
            .unwrap();
        assert_eq!(picker.read(cx).query(cx), "");
    });
}

#[gpui::test]
async fn project_symbols_shortcut_opens_existing_picker(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        project_symbols::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(workspace::ToggleProjectSymbols);
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, cx| {
        let picker = workspace
            .active_modal::<picker::Picker<project_symbols::ProjectSymbolsDelegate>>(cx)
            .unwrap();
        assert_eq!(picker.read(cx).query(cx), "#");
    });
}

#[gpui::test]
async fn text_finder_shortcut_follows_unified_palette_setting(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        search::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(zed_actions::text_finder::Toggle);
    cx.run_until_parked();
    let text_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<search::text_finder::TextFinder>(cx)
            .unwrap()
    });
    assert_eq!(
        text_finder.read_with(cx, |text_finder, cx| text_finder.query_with_mode_prefix(cx)),
        "%"
    );
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.hide_modal(window, cx);
    });

    cx.update(|_, cx| set_unified_palette_enabled(cx, false));
    cx.dispatch_action(zed_actions::text_finder::Toggle);
    cx.run_until_parked();
    let text_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<search::text_finder::TextFinder>(cx)
            .unwrap()
    });
    assert_eq!(
        text_finder.read_with(cx, |text_finder, cx| text_finder.query_with_mode_prefix(cx)),
        ""
    );
}

#[gpui::test]
async fn command_prefix_opens_existing_command_palette(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        command_palette::init(cx);
        file_finder::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query(">needle", window, cx);
    });
    cx.run_until_parked();

    let command_palette = workspace.read_with(cx, |workspace, cx| {
        assert!(
            workspace
                .active_modal::<file_finder::FileFinder>(cx)
                .is_none()
        );
        workspace
            .active_modal::<command_palette::CommandPalette>(cx)
            .unwrap()
    });
    command_palette.read_with(cx, |command_palette, cx| {
        assert_eq!(command_palette.query_with_mode_prefix(cx), ">needle");
        assert_eq!(command_palette.query(cx), "needle");
    });
}

#[gpui::test]
async fn help_prefix_shows_available_modes(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        file_finder::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query("?", window, cx);
    });
    cx.run_until_parked();

    workspace.read_with(cx, |workspace, cx| {
        assert!(
            workspace
                .active_modal::<file_finder::FileFinder>(cx)
                .is_none()
        );
        let help = workspace.active_modal::<ModeHelp>(cx).unwrap();
        assert_eq!(help.read(cx).picker.read(cx).delegate.match_count(), 6);
    });

    cx.dispatch_action(Confirm);
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, cx| {
        assert!(
            workspace
                .active_modal::<file_finder::FileFinder>(cx)
                .is_some()
        );
    });
}

#[gpui::test]
async fn project_symbols_prefix_opens_existing_picker(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        file_finder::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query("#symbol", window, cx);
    });
    cx.run_until_parked();

    let picker = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<picker::Picker<project_symbols::ProjectSymbolsDelegate>>(cx)
            .unwrap()
    });
    picker.read_with(cx, |picker, cx| {
        assert_eq!(picker.query(cx), "#symbol");
        assert_eq!(picker.query_without_mode_prefix(cx), "symbol");
    });

    cx.dispatch_action(editor::actions::SelectAll);
    cx.dispatch_action(editor::actions::Backspace);
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    assert_eq!(
        file_finder.read_with(cx, |file_finder, cx| file_finder.query(cx)),
        ""
    );
}

#[gpui::test]
async fn modal_keybindings_follow_unified_palette_setting(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/root"), json!({"test.rs": "fn target() {}"}))
        .await;
    cx.update(|cx| {
        init(cx);
        outline::init(cx);
        go_to_line::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    project.read_with(cx, |project, _| {
        project.languages().add(language::rust_lang());
    });
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("test.rs")), None, true, window, cx)
        })
        .await
        .unwrap();

    cx.dispatch_action(zed_actions::outline::ToggleOutline);
    cx.run_until_parked();
    let outline_view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_modal::<outline::OutlineView>(cx).unwrap()
    });
    assert_eq!(
        outline_view.read_with(cx, |outline_view, cx| outline_view
            .query_with_mode_prefix(cx)),
        "@"
    );
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.hide_modal(window, cx);
    });

    cx.dispatch_action(editor::actions::ToggleGoToLine);
    cx.run_until_parked();
    let go_to_line = workspace.read_with(cx, |workspace, cx| {
        workspace.active_modal::<go_to_line::GoToLine>(cx).unwrap()
    });
    assert_eq!(
        go_to_line.read_with(cx, |go_to_line, cx| go_to_line.query_with_mode_prefix(cx)),
        ":"
    );
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.hide_modal(window, cx);
    });

    cx.update(|_, cx| set_unified_palette_enabled(cx, false));

    cx.dispatch_action(zed_actions::outline::ToggleOutline);
    cx.run_until_parked();
    let outline_view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_modal::<outline::OutlineView>(cx).unwrap()
    });
    assert_eq!(
        outline_view.read_with(cx, |outline_view, cx| outline_view
            .query_with_mode_prefix(cx)),
        ""
    );
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.hide_modal(window, cx);
    });

    cx.dispatch_action(editor::actions::ToggleGoToLine);
    cx.run_until_parked();
    let go_to_line = workspace.read_with(cx, |workspace, cx| {
        workspace.active_modal::<go_to_line::GoToLine>(cx).unwrap()
    });
    assert_eq!(
        go_to_line.read_with(cx, |go_to_line, cx| go_to_line.query_with_mode_prefix(cx)),
        ""
    );
}

#[gpui::test]
async fn unavailable_outline_prefix_falls_back_to_file_matching(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/root"), json!({"@target.rs": ""}))
        .await;
    cx.update(|cx| {
        init(cx);
        file_finder::init(cx);
        outline::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query("@target.rs", window, cx);
    });
    cx.run_until_parked();
    assert!(workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .is_some()
    }));

    cx.dispatch_action(Confirm);
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, cx| {
        let editor = workspace.active_item_as::<editor::Editor>(cx).unwrap();
        assert_eq!(editor.read(cx).title(cx), "@target.rs");
    });
}

#[gpui::test]
async fn outline_prefix_opens_existing_outline_view(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({"test.rs": "fn unrelated() {}\nfn target() {}"}),
        )
        .await;

    cx.update(|cx| {
        init(cx);
        file_finder::init(cx);
        outline::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    project.read_with(cx, |project, _| {
        project.languages().add(language::rust_lang());
    });
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let editor = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("test.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<editor::Editor>()
        .unwrap();

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query("@target", window, cx);
    });
    cx.run_until_parked();

    let outline_view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_modal::<outline::OutlineView>(cx).unwrap()
    });
    assert_eq!(
        outline_view.read_with(cx, |outline_view, cx| {
            outline_view.query_with_mode_prefix(cx)
        }),
        "@target"
    );
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    let selected_point = editor.update_in(cx, |editor, _, cx| {
        editor
            .selections
            .last::<language::Point>(&editor.display_snapshot(cx))
            .range()
            .start
    });
    assert_eq!(selected_point, language::Point::new(1, 0));
}

#[gpui::test]
async fn go_to_line_prefix_navigates_to_requested_line(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    let contents = (1..=130)
        .map(|line| format!("line {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/root"), json!({"test.rs": contents}))
        .await;

    cx.update(|cx| {
        init(cx);
        file_finder::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let editor = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("test.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<editor::Editor>()
        .unwrap();

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query(":123", window, cx);
    });
    cx.run_until_parked();

    let go_to_line = workspace.read_with(cx, |workspace, cx| {
        workspace.active_modal::<go_to_line::GoToLine>(cx).unwrap()
    });
    assert_eq!(
        go_to_line.read_with(cx, |go_to_line, cx| go_to_line.query_with_mode_prefix(cx)),
        ":123"
    );

    cx.dispatch_action(editor::actions::SelectAll);
    cx.dispatch_action(editor::actions::Backspace);
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    assert_eq!(
        file_finder.read_with(cx, |file_finder, cx| file_finder.query(cx)),
        ""
    );
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query(":123", window, cx);
    });
    cx.run_until_parked();

    let go_to_line = workspace.read_with(cx, |workspace, cx| {
        workspace.active_modal::<go_to_line::GoToLine>(cx).unwrap()
    });
    assert_eq!(
        go_to_line.read_with(cx, |go_to_line, cx| go_to_line.query_with_mode_prefix(cx)),
        ":123"
    );
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    let selected_point = editor.update_in(cx, |editor, _, cx| {
        editor
            .selections
            .last::<language::Point>(&editor.display_snapshot(cx))
            .range()
            .start
    });
    assert_eq!(selected_point, language::Point::new(122, 0));
}

#[gpui::test]
async fn quick_search_prefix_opens_text_finder_and_preserves_query(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        init(cx);
        file_finder::init(cx);
        search::init(cx);
    });
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query("%needle", window, cx);
    });
    cx.run_until_parked();

    workspace.read_with(cx, |workspace, cx| {
        assert!(
            workspace
                .active_modal::<search::text_finder::TextFinder>(cx)
                .is_some()
        );
    });
    let text_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<search::text_finder::TextFinder>(cx)
            .unwrap()
    });
    assert_eq!(
        text_finder.read_with(cx, |text_finder, cx| text_finder.query(cx)),
        "needle"
    );
    assert_eq!(
        text_finder.read_with(cx, |text_finder, cx| {
            text_finder.query_with_mode_prefix(cx)
        }),
        "%needle"
    );

    cx.dispatch_action(workspace::ToggleFileFinder::default());
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    assert_eq!(
        file_finder.read_with(cx, |file_finder, cx| file_finder.query(cx)),
        "needle"
    );
    file_finder.update_in(cx, |file_finder, window, cx| {
        file_finder.set_query("%needle", window, cx);
    });
    cx.run_until_parked();
    cx.dispatch_action(editor::actions::SelectAll);
    cx.dispatch_action(editor::actions::Backspace);
    cx.run_until_parked();
    let file_finder = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_modal::<file_finder::FileFinder>(cx)
            .unwrap()
    });
    assert_eq!(
        file_finder.read_with(cx, |file_finder, cx| file_finder.query(cx)),
        ""
    );
}

fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
    cx.update(|cx| {
        let state = AppState::test(cx);
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        workspace::init(state.clone(), cx);
        set_unified_palette_enabled(cx, true);
        state
    })
}

fn set_unified_palette_enabled(cx: &mut App, enabled: bool) {
    SettingsStore::update_global(cx, |store, cx| {
        store.update_user_settings(cx, |content| content.unified_palette = Some(enabled));
    });
}
