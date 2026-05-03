use super::*;
use editor::{
    Editor, KillRingState,
    actions::{KillRingPickAndYank, KillRingSave, KillRingYankPop, Undo},
    test::{assert_text_with_selections, select_ranges},
};
use gpui::{Entity, TestAppContext, VisualTestContext};
use picker::Picker;
use pretty_assertions::assert_eq;
use project::{FakeFs, Project};
use settings::SettingsStore;
use std::path::Path;
use util::test::marked_text_ranges;
use workspace::{MultiWorkspace, Workspace};

struct PickerTestContext {
    cx: VisualTestContext,
    workspace: Entity<Workspace>,
    editor: Entity<Editor>,
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        assets::Assets.load_test_fonts(cx);
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        editor::init(cx);
        crate::init(cx);
    });
    zlog::init_test();
}

async fn workspace_editor(cx: &mut TestAppContext) -> PickerTestContext {
    init_test(cx);

    let root = Path::new("/root");
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(root, serde_json::json!({ "file": "" }))
        .await;

    let project = Project::test(fs, [root], cx).await;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(root.join("file"), cx)
        })
        .await
        .expect("opening test buffer should succeed");

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let workspace = cx.read(|cx| multi_workspace.read(cx).workspace().clone());
    let editor: Entity<Editor> = workspace.update_in(cx, |workspace, window, cx| {
        let editor = cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx));
        workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
        editor
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();

    PickerTestContext {
        cx: cx.clone(),
        workspace,
        editor,
    }
}

fn set_editor_state(context: &mut PickerTestContext, marked_text: &str) {
    let (text, _) = marked_text_ranges(marked_text, true);
    context
        .editor
        .update_in(&mut context.cx, |editor, window, cx| {
            editor.set_text(text, window, cx);
            select_ranges(editor, marked_text, window, cx);
        });
    context.cx.run_until_parked();
}

fn assert_editor_state(context: &mut PickerTestContext, marked_text: &str) {
    context
        .editor
        .update_in(&mut context.cx, |editor, _window, cx| {
            assert_text_with_selections(editor, marked_text, cx);
        });
}

fn save_entry(context: &mut PickerTestContext, marked_text: &str) {
    set_editor_state(context, marked_text);
    context
        .editor
        .update_in(&mut context.cx, |editor, window, cx| {
            editor.kill_ring_save(&KillRingSave, window, cx);
        });
    context.cx.run_until_parked();
}

fn save_entries(context: &mut PickerTestContext, entries_oldest_first: &[&str]) {
    for entry in entries_oldest_first {
        save_entry(context, &format!("«{}ˇ»", entry));
    }
}

fn kill_ring_texts(context: &mut PickerTestContext) -> Vec<String> {
    context.cx.read_global::<KillRingState, _>(|kill_ring, _| {
        kill_ring
            .snapshot()
            .into_iter()
            .map(|entry| entry.text().to_string())
            .collect()
    })
}

fn dispatch_pick_and_yank(context: &mut PickerTestContext) {
    context.cx.dispatch_action(KillRingPickAndYank);
    context.cx.run_until_parked();
}

fn active_modal(context: &mut PickerTestContext) -> Option<Entity<KillRingPicker>> {
    context.cx.read(|cx| {
        context
            .workspace
            .read(cx)
            .active_modal::<KillRingPicker>(cx)
    })
}

fn active_picker(context: &mut PickerTestContext) -> Entity<Picker<KillRingPickerDelegate>> {
    let modal = active_modal(context).expect("kill ring picker modal should be active");
    context.cx.read(|cx| modal.read(cx).picker.clone())
}

fn select_picker_index(context: &mut PickerTestContext, index: usize) {
    let picker = active_picker(context);
    picker.update_in(&mut context.cx, |picker, window, cx| {
        picker.set_selected_index(index, None, false, window, cx);
    });
    context.cx.run_until_parked();
}

fn confirm_picker(context: &mut PickerTestContext) {
    let picker = active_picker(context);
    picker.update_in(&mut context.cx, |picker, window, cx| {
        picker.delegate.confirm(false, window, cx);
    });
    context.cx.run_until_parked();
}

fn cancel_picker(context: &mut PickerTestContext) {
    let picker = active_picker(context);
    picker.update_in(&mut context.cx, |picker, window, cx| {
        picker.cancel(&menu::Cancel, window, cx);
    });
    context.cx.run_until_parked();
}

#[gpui::test]
async fn test_kill_ring_pick_and_yank_empty_ring_noop(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    set_editor_state(&mut context, "before ˇ after");

    dispatch_pick_and_yank(&mut context);

    assert!(active_modal(&mut context).is_none());
    assert_editor_state(&mut context, "before ˇ after");
}

#[gpui::test]
async fn test_kill_ring_pick_and_yank_opens_modal(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);

    assert!(active_modal(&mut context).is_some());
}

#[gpui::test]
async fn test_kill_ring_picker_lists_entries_newest_first(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first", "second", "third"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);

    let picker = active_picker(&mut context);
    let (match_count, entry_texts) = context.cx.read(|cx| {
        let picker = picker.read(cx);
        (
            picker.delegate.match_count(),
            picker
                .delegate
                .matches
                .iter()
                .map(|entry_match| {
                    picker.delegate.entries[entry_match.candidate_id]
                        .text()
                        .to_string()
                })
                .collect::<Vec<_>>(),
        )
    });
    assert_eq!(match_count, 3);
    assert_eq!(entry_texts, vec!["third", "second", "first"]);
}

#[gpui::test]
async fn test_kill_ring_picker_multiline_entry_renders_single_line(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entry(&mut context, "«one\ntwoˇ»");
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);

    let picker = active_picker(&mut context);
    let rendered_preview = context
        .cx
        .read(|cx| picker.read(cx).delegate.matches[0].string.clone());
    assert!(rendered_preview.contains('⏎'));
    assert!(!rendered_preview.contains('\n'));
}

#[gpui::test]
async fn test_kill_ring_picker_preview_is_transactional(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    assert_editor_state(&mut context, "firstˇ");

    context
        .editor
        .update_in(&mut context.cx, |editor, window, cx| {
            editor.undo(&Undo, window, cx);
        });
    assert_editor_state(&mut context, "ˇ");
}

#[gpui::test]
async fn test_kill_ring_picker_navigate_replaces_preview(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first", "second"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    assert_editor_state(&mut context, "secondˇ");

    select_picker_index(&mut context, 1);

    assert_editor_state(&mut context, "firstˇ");
}

#[gpui::test]
async fn test_kill_ring_picker_confirm_commits_insertion(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    confirm_picker(&mut context);

    assert!(active_modal(&mut context).is_none());
    assert_editor_state(&mut context, "firstˇ");
}

#[gpui::test]
async fn test_kill_ring_picker_confirm_seeds_yank_pop(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first", "second", "third"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    select_picker_index(&mut context, 1);
    confirm_picker(&mut context);
    assert_editor_state(&mut context, "secondˇ");

    context
        .editor
        .update_in(&mut context.cx, |editor, window, cx| {
            editor.kill_ring_yank_pop(&KillRingYankPop, window, cx);
        });

    assert_editor_state(&mut context, "firstˇ");
}

#[gpui::test]
async fn test_kill_ring_picker_escape_reverts_preview(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first"]);
    set_editor_state(&mut context, "before ˇ after");

    dispatch_pick_and_yank(&mut context);
    cancel_picker(&mut context);

    assert!(active_modal(&mut context).is_none());
    assert_editor_state(&mut context, "before ˇ after");
}

#[gpui::test]
async fn test_kill_ring_picker_fuzzy_filter(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["alpha", "beta", "gamma"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    let picker = active_picker(&mut context);
    picker.update_in(&mut context.cx, |picker, window, cx| {
        picker.update_matches("alp".to_string(), window, cx);
    });
    context.cx.run_until_parked();

    let matches = context.cx.read(|cx| {
        picker
            .read(cx)
            .delegate
            .matches
            .iter()
            .map(|entry_match| entry_match.string.clone())
            .collect::<Vec<_>>()
    });
    assert_eq!(matches, vec!["alpha"]);
}

#[gpui::test]
async fn test_kill_ring_picker_multi_cursor_insertion(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["x"]);
    set_editor_state(&mut context, "ˇone\nˇtwo\nˇthree");

    dispatch_pick_and_yank(&mut context);
    confirm_picker(&mut context);

    assert_editor_state(&mut context, "xˇone\nxˇtwo\nxˇthree");
}

#[gpui::test]
async fn test_kill_ring_picker_editor_lifecycle(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    let picker = active_picker(&mut context);
    picker.update_in(&mut context.cx, |picker, window, cx| {
        picker.delegate.active_editor = gpui::WeakEntity::new_invalid();
        picker.delegate.dismissed(window, cx);
    });
}

#[gpui::test]
async fn test_kill_ring_picker_does_not_mutate_kill_ring(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first", "second", "third"]);
    let before = kill_ring_texts(&mut context);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    select_picker_index(&mut context, 1);
    cancel_picker(&mut context);

    assert_eq!(kill_ring_texts(&mut context), before);
}

#[gpui::test]
async fn test_kill_ring_picker_focus_loss_reverts_preview(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    context
        .editor
        .update_in(&mut context.cx, |editor, window, cx| {
            editor.focus_handle(cx).focus(window, cx);
        });
    context.cx.run_until_parked();

    assert!(active_modal(&mut context).is_none());
    assert_editor_state(&mut context, "ˇ");
}

#[gpui::test]
async fn test_kill_ring_picker_dismiss_after_editor_drop_no_panic(cx: &mut TestAppContext) {
    let mut context = workspace_editor(cx).await;
    save_entries(&mut context, &["first"]);
    set_editor_state(&mut context, "ˇ");

    dispatch_pick_and_yank(&mut context);
    let picker = active_picker(&mut context);
    picker.update_in(&mut context.cx, |picker, window, cx| {
        picker.delegate.active_editor = gpui::WeakEntity::new_invalid();
        picker.delegate.dismissed(window, cx);
    });
}
