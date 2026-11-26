use super::NotebookPreview;
use editor::Editor;
use feature_flags::{
    FeatureFlag as _, FeatureFlagAppExt as _, FeatureFlagStore, NotebookFeatureFlag,
};
use gpui::{Entity, TestAppContext, UpdateGlobal as _, VisualContext as _, VisualTestContext};
use project::trusted_worktrees::TrustedWorktrees;
use project::{FakeFs, Fs as _, Project};
use serde_json::{Value, json};
use settings::SettingsStore;
use std::path::Path;
use std::sync::Arc;
use util::path;
use util::rel_path::rel_path;
use workspace::item::ItemHandle;
use workspace::pane::{CloseActiveItem, ReopenClosedItem};
use workspace::{AppState, MultiWorkspace, Workspace};
use zed_actions::notebook::{
    AddCodeBlock, AddMarkdownBlock, DeleteCell, EditSource, EnterCommandMode, MoveCellDown,
    MoveCellUp, OpenNotebook,
};

const NOTEBOOK: &str = r##"{
  "nbformat_minor": 5,  "nbformat": 4,
  "metadata": { "custom": { "keep": true } },
  "cells": [
    { "cell_type": "markdown", "id": "intro", "metadata": {"tags":["note"]},
      "source": ["# Field notes\n", "![sample](attachment:sample.png)"],
      "attachments": {"sample.png":{"image/png":"iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAAAAAA6fptVAAAACklEQVR4nGP4DwABAQEAsTj2FAAAAABJRU5ErkJggg=="}} },
    { "cell_type": "code", "id": "compute", "execution_count": 7,
      "metadata": {}, "source": ["print('from source')"],
      "outputs": [{"output_type":"stream", "name":"stdout", "text":["saved result\n"]}] }
  ]
}
"##;

pub(in crate::notebook) async fn notebook_workspace(
    files: Value,
    cx: &mut TestAppContext,
) -> (Arc<FakeFs>, Entity<Workspace>, &mut VisualTestContext) {
    let fs = cx.update(|cx| {
        let state = AppState::test(cx);
        editor::init(cx);
        FeatureFlagStore::init(cx);
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .feature_flags
                    .get_or_insert_default()
                    .insert(NotebookFeatureFlag::NAME.into(), "off".into());
            });
        });
        assert!(!cx.has_flag::<NotebookFeatureFlag>());
        crate::notebook::init(cx);
        project::trusted_worktrees::init(Default::default(), cx);
        state.fs.as_fake()
    });
    fs.insert_tree(path!("/notebooks"), files).await;
    let project =
        Project::test_with_worktree_trust(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
    project.read_with(cx, |project, cx| {
        assert!(project.client().status().borrow().is_signed_out());
        assert!(TrustedWorktrees::has_restricted_worktrees(
            &project.worktree_store(),
            cx
        ));
    });
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
    (fs, workspace, cx)
}

pub(in crate::notebook) async fn open_file(
    name: &str,
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) {
    workspace
        .update_in(cx, |workspace, window, cx| {
            let worktree_id = workspace
                .project()
                .read(cx)
                .worktrees(cx)
                .next()
                .expect("worktree")
                .read(cx)
                .id();
            workspace.open_path((worktree_id, rel_path(name)), None, true, window, cx)
        })
        .await
        .expect("open project file");
    cx.run_until_parked();
}

#[gpui::test]
async fn clearing_outputs_confirms_and_only_changes_saved_result_fields(cx: &mut TestAppContext) {
    use zed_actions::notebook::ClearOutputs;
    let file = NOTEBOOK.replace(
        r#"{ "cell_type": "code", "id": "compute""#,
        r#"{ "cell_type": "code", "id": "silent", "source": [], "metadata": {}, "outputs": [], "execution_count": 0 },
    { "cell_type": "code", "id": "compute""#,
    );
    let (fs, workspace, cx) = notebook_workspace(json!({"clear.ipynb": file}), cx).await;
    open_file("clear.ipynb", &workspace, cx).await;
    let sources = cell_sources(&workspace, cx);
    let path = Path::new(path!("/notebooks/clear.ipynb"));
    cx.dispatch_action(ClearOutputs);
    cx.simulate_prompt_answer("Cancel");
    cx.run_until_parked();
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    assert_eq!(fs.load(path).await.unwrap(), file);

    cx.dispatch_action(ClearOutputs);
    cx.simulate_prompt_answer("Clear Outputs");
    cx.run_until_parked();
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    let cleared = file
        .replace(r#""execution_count": 0"#, r#""execution_count": null"#)
        .replace(r#""execution_count": 7"#, r#""execution_count": null"#)
        .replace(
            r#"[{"output_type":"stream", "name":"stdout", "text":["saved result\n"]}]"#,
            "[]",
        );
    assert_eq!(fs.load(path).await.unwrap(), cleared);
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    cx.run_until_parked();
    open_file("clear.ipynb", &workspace, cx).await;
    assert_eq!(cell_sources(&workspace, cx), sources);
    cx.dispatch_action(ClearOutputs);
    assert!(cx.pending_prompt().is_none());
}

fn cell_sources(workspace: &Entity<Workspace>, cx: &mut VisualTestContext) -> Vec<String> {
    workspace.read_with(cx, |workspace, cx| {
        let preview = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
        let notebook = preview.read(cx).notebook.as_ref().unwrap().read(cx);
        (0..)
            .map_while(|index| notebook.cell(index))
            .map(|cell| cell.current_source(cx))
            .collect()
    })
}

fn active_tab_title(workspace: &Entity<Workspace>, cx: &mut VisualTestContext) -> String {
    workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_item(cx)
            .expect("active tab")
            .tab_content_text(0, cx)
            .to_string()
    })
}

#[gpui::test]
async fn tab_return_restores_the_cell_selection_and_notebook_mode(cx: &mut TestAppContext) {
    use workspace::pane::{ActivateNextItem, ActivatePreviousItem};
    use zed_actions::notebook::EnterEditMode;

    let (_, workspace, cx) =
        notebook_workspace(json!({"alpha.ipynb": NOTEBOOK, "beta.ipynb": NOTEBOOK}), cx).await;
    open_file("alpha.ipynb", &workspace, cx).await;
    cx.dispatch_action(AddCodeBlock);
    cx.simulate_keystrokes("a b");
    cx.dispatch_action(editor::actions::SelectLeft);
    open_file("beta.ipynb", &workspace, cx).await;
    cx.dispatch_action(ActivatePreviousItem::default());
    cx.simulate_keystrokes("x");
    assert_eq!(cell_sources(&workspace, cx)[1], "ax");

    cx.dispatch_action(ActivateNextItem::default());
    cx.simulate_keystrokes("y");
    assert_eq!(
        cell_sources(&workspace, cx)[0],
        "# Field notes\n![sample](attachment:sample.png)"
    );
    cx.dispatch_action(EnterEditMode);
    cx.simulate_keystrokes("y");
    assert!(cell_sources(&workspace, cx)[0].starts_with("y# Field notes"));
}

#[gpui::test]
async fn markdown_tab_return_preserves_the_caret_and_command_mode(cx: &mut TestAppContext) {
    use workspace::pane::{ActivateNextItem, ActivatePreviousItem};
    use zed_actions::notebook::EnterEditMode;

    let (fs, workspace, cx) =
        notebook_workspace(json!({"field.ipynb": NOTEBOOK, "other.txt": "other"}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EnterEditMode);
    cx.dispatch_action(editor::actions::MoveRight);
    open_file("other.txt", &workspace, cx).await;
    cx.dispatch_action(ActivatePreviousItem::default());
    cx.simulate_keystrokes("x");
    assert_eq!(
        cell_sources(&workspace, cx)[0],
        "#x Field notes\n![sample](attachment:sample.png)"
    );

    cx.dispatch_action(EnterCommandMode);
    cx.dispatch_action(ActivateNextItem::default());
    cx.dispatch_action(ActivatePreviousItem::default());
    cx.simulate_keystrokes("y");
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    let saved = fs
        .load(Path::new(path!("/notebooks/field.ipynb")))
        .await
        .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&saved).unwrap(),
        serde_json::from_str::<Value>(&NOTEBOOK.replace("# Field notes", "#x Field notes"))
            .unwrap()
    );
    assert_eq!(
        fs.load(Path::new(path!("/notebooks/other.txt")))
            .await
            .unwrap(),
        "other"
    );
}

#[gpui::test]
async fn reopening_closed_notebooks_ignores_source_view_swaps(cx: &mut TestAppContext) {
    let (_, workspace, cx) =
        notebook_workspace(json!({"field.ipynb": NOTEBOOK, "notes.txt": "notes"}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EditSource);
    cx.dispatch_action(OpenNotebook);
    open_file("notes.txt", &workspace, cx).await;
    cx.dispatch_action(CloseActiveItem::default());
    cx.dispatch_action(CloseActiveItem::default());
    assert!(workspace.read_with(cx, |workspace, cx| workspace.active_item(cx).is_none()));

    cx.dispatch_action(ReopenClosedItem);
    assert_eq!(active_tab_title(&workspace, cx), "field.ipynb");
    assert_eq!(cell_sources(&workspace, cx).len(), 2);
    cx.dispatch_action(ReopenClosedItem);
    assert_eq!(active_tab_title(&workspace, cx), "notes.txt");
    cx.dispatch_action(ReopenClosedItem);
    assert_eq!(active_tab_title(&workspace, cx), "notes.txt");
    assert_eq!(
        workspace.read_with(cx, |workspace, cx| workspace
            .active_pane()
            .read(cx)
            .items_len()),
        2
    );
}

#[gpui::test]
async fn reopening_explicitly_closed_notebook_source_restores_json(cx: &mut TestAppContext) {
    let (_, workspace, cx) =
        notebook_workspace(json!({"field.ipynb": NOTEBOOK, "notes.txt": "notes"}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EditSource);
    cx.dispatch_action(editor::actions::MoveToEnd);
    open_file("notes.txt", &workspace, cx).await;
    cx.dispatch_action(CloseActiveItem::default());
    cx.dispatch_action(CloseActiveItem::default());
    cx.dispatch_action(ReopenClosedItem);
    let source = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_item_as::<Editor>(cx)
            .expect("reopened raw notebook source")
    });
    source.update(cx, |source, cx| {
        assert_eq!(source.text(cx), NOTEBOOK);
        assert_eq!(
            source
                .selections
                .newest::<language::Point>(&source.display_snapshot(cx))
                .head(),
            language::Point::new(NOTEBOOK.lines().count() as u32, 0)
        );
    });
    cx.dispatch_action(ReopenClosedItem);
    assert_eq!(active_tab_title(&workspace, cx), "notes.txt");
}

#[gpui::test]
async fn reopening_a_notebook_reuses_source_already_open_in_the_pane(cx: &mut TestAppContext) {
    let (_, workspace, cx) = notebook_workspace(
        json!({"field.ipynb": NOTEBOOK, "older.txt": "older", "notes.txt": "notes"}),
        cx,
    )
    .await;
    open_file("field.ipynb", &workspace, cx).await;
    open_file("older.txt", &workspace, cx).await;
    cx.dispatch_action(CloseActiveItem::default());
    cx.dispatch_action(CloseActiveItem::default());
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EditSource);
    let source = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<Editor>(cx).unwrap()
    });
    open_file("notes.txt", &workspace, cx).await;
    cx.dispatch_action(ReopenClosedItem);
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.active_item_as::<Editor>(cx), Some(source));
        assert_eq!(workspace.active_pane().read(cx).items_len(), 2);
    });
    cx.dispatch_action(ReopenClosedItem);
    assert_eq!(active_tab_title(&workspace, cx), "older.txt");
}

#[gpui::test]
async fn reopening_notebook_cells_keeps_an_existing_source_tab(cx: &mut TestAppContext) {
    let (_, workspace, cx) = notebook_workspace(
        json!({"field.ipynb": NOTEBOOK, "notes.txt": "notes", "older.txt": "older"}),
        cx,
    )
    .await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EditSource);
    let source_id = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<Editor>(cx).unwrap().entity_id()
    });
    cx.dispatch_action(workspace::pane::SplitRight::default());
    open_file("notes.txt", &workspace, cx).await;
    cx.dispatch_action(workspace::pane::ActivatePreviousItem::default());
    cx.dispatch_action(OpenNotebook);
    open_file("older.txt", &workspace, cx).await;
    cx.dispatch_action(CloseActiveItem::default());
    cx.dispatch_action(CloseActiveItem::default());
    cx.dispatch_action(ReopenClosedItem);
    assert_eq!(active_tab_title(&workspace, cx), "field.ipynb");
    assert_eq!(cell_sources(&workspace, cx).len(), 2);
    cx.dispatch_action(workspace::ActivatePane(0));
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.active_pane().read(cx).items_len(), 1);
        let editor = workspace.active_item_as::<Editor>(cx).unwrap();
        assert_eq!(editor.entity_id(), source_id);
        assert_eq!(editor.read(cx).text(cx), NOTEBOOK);
    });
}

#[gpui::test]
async fn discarding_a_notebook_closes_its_tab(cx: &mut TestAppContext) {
    let (fs, workspace, cx) =
        notebook_workspace(json!({"field.ipynb": NOTEBOOK, "notes.txt": "notes"}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let saved_cells = cell_sources(&workspace, cx);
    open_file("notes.txt", &workspace, cx).await;
    cx.dispatch_action(CloseActiveItem::default());
    cx.dispatch_action(AddCodeBlock);
    cx.simulate_keystrokes("x");
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    cx.simulate_prompt_answer("Cancel");
    cx.run_until_parked();
    assert_eq!(cell_sources(&workspace, cx)[1], "x");

    cx.dispatch_action(ReopenClosedItem);
    assert_eq!(active_tab_title(&workspace, cx), "notes.txt");
    cx.dispatch_action(CloseActiveItem::default());
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    cx.simulate_prompt_answer("Don't Save");
    cx.run_until_parked();
    assert_eq!(
        workspace.read_with(cx, |workspace, cx| workspace
            .active_pane()
            .read(cx)
            .items_len()),
        0
    );
    assert_eq!(
        fs.load(Path::new(path!("/notebooks/field.ipynb")))
            .await
            .unwrap(),
        NOTEBOOK
    );
    cx.dispatch_action(ReopenClosedItem);
    assert_eq!(cell_sources(&workspace, cx), saved_cells);
}

#[gpui::test]
async fn closing_notebook_cells_keeps_an_independent_source_view(cx: &mut TestAppContext) {
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": NOTEBOOK}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EditSource);
    cx.dispatch_action(workspace::pane::SplitRight::default());
    cx.dispatch_action(OpenNotebook);
    cx.run_until_parked();
    cx.dispatch_action(AddCodeBlock);
    cx.simulate_keystrokes("x");
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    cx.run_until_parked();
    assert!(cx.pending_prompt().is_none());
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.panes().len(), 1);
        let source = workspace
            .active_item_as::<Editor>(cx)
            .expect("independent source");
        let contents: Value = serde_json::from_str(&source.read(cx).text(cx)).unwrap();
        assert_eq!(contents["cells"][1]["source"], json!(["x"]));
        assert!(source.is_dirty(cx));
    });
    assert_eq!(
        fs.load(Path::new(path!("/notebooks/field.ipynb")))
            .await
            .unwrap(),
        NOTEBOOK
    );
}

#[gpui::test]
async fn author_cells_in_an_empty_restricted_notebook(cx: &mut TestAppContext) {
    let original = r#"{"nbformat":4, "nbformat_minor":5, "metadata":{}, "cells":[ ]}"#;
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": original}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(AddCodeBlock);
    assert_eq!(cell_sources(&workspace, cx), [""]);
    cx.simulate_keystrokes("x");
    cx.dispatch_action(EnterCommandMode);
    cx.dispatch_action(AddMarkdownBlock);
    cx.simulate_keystrokes("n o t e");
    cx.dispatch_action(EnterCommandMode);
    assert_eq!(cell_sources(&workspace, cx), ["x", "note"]);
    cx.dispatch_action(editor::actions::Undo);
    assert_eq!(cell_sources(&workspace, cx), ["x"]);
    cx.dispatch_action(editor::actions::Redo);
    assert_eq!(cell_sources(&workspace, cx), ["x", "note"]);

    cx.dispatch_action(MoveCellUp);
    cx.dispatch_action(MoveCellUp);
    assert_eq!(cell_sources(&workspace, cx), ["note", "x"]);
    cx.dispatch_action(DeleteCell);
    assert_eq!(cell_sources(&workspace, cx), ["x"]);
    cx.dispatch_action(editor::actions::Undo);
    assert_eq!(cell_sources(&workspace, cx), ["note", "x"]);
    cx.dispatch_action(editor::actions::Redo);
    cx.dispatch_action(DeleteCell);
    assert!(cell_sources(&workspace, cx).is_empty());
    cx.dispatch_action(editor::actions::Undo);
    assert_eq!(cell_sources(&workspace, cx), ["x"]);
    cx.dispatch_action(settings::ActionSequence(vec![
        Box::new(editor::actions::Undo),
        Box::new(workspace::SaveAll { save_intent: None }),
    ]));

    let saved = fs
        .load(Path::new(path!("/notebooks/field.ipynb")))
        .await
        .unwrap();
    let nbformat::Notebook::V4(saved) = nbformat::parse_notebook(&saved).unwrap() else {
        panic!("expected a v4 notebook with valid cell IDs");
    };
    let [
        nbformat::v4::Cell::Markdown {
            id: markdown_id,
            source: note,
            ..
        },
        nbformat::v4::Cell::Code {
            id: code_id,
            source: code,
            outputs,
            execution_count,
            ..
        },
    ] = saved.cells.as_slice()
    else {
        panic!("expected Markdown followed by code");
    };
    assert_ne!(markdown_id, code_id);
    assert_eq!(note.concat(), "note");
    assert_eq!(code.concat(), "x");
    assert!(outputs.is_empty());
    assert_eq!(*execution_count, None);
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    assert!(workspace.read_with(cx, |workspace, cx| workspace.active_item(cx).is_none()));
    open_file("field.ipynb", &workspace, cx).await;
    assert_eq!(cell_sources(&workspace, cx), ["note", "x"]);
}

#[gpui::test]
async fn grouped_cell_edits_across_a_move_survive_command_undo_and_save(cx: &mut TestAppContext) {
    use zed_actions::notebook::EnterEditMode;

    let notebook = json!({
        "nbformat": 4, "nbformat_minor": 5, "metadata": {},
        "cells": [
            {"cell_type":"code", "id":"code", "metadata":{}, "outputs":[], "execution_count":1, "source":[]},
            {"cell_type":"markdown", "id":"note", "metadata":{}, "source":["neighbor"]}
        ]
    });
    let (fs, workspace, cx) =
        notebook_workspace(json!({"field.ipynb": notebook.to_string()}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let parsed_sources = |text: &str| {
        let nbformat::Notebook::V4(notebook) = nbformat::parse_notebook(text).unwrap() else {
            panic!("expected a v4 notebook");
        };
        notebook
            .cells
            .iter()
            .map(|cell| cell.source().concat())
            .collect::<Vec<_>>()
    };
    let check = |expected: [&str; 2], cx: &mut VisualTestContext| {
        assert_eq!(cell_sources(&workspace, cx), expected);
        let raw = workspace.read_with(cx, |workspace, cx| {
            let preview = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
            preview.read(cx).editor.read(cx).text(cx)
        });
        assert_eq!(parsed_sources(&raw), expected);
    };

    cx.dispatch_action(EnterEditMode);
    cx.simulate_keystrokes("a");
    let code = workspace.read_with(cx, |workspace, cx| {
        let preview = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
        let notebook = preview.read(cx).notebook.as_ref().unwrap().read(cx);
        notebook.cell(0).unwrap().editor(cx).unwrap().clone()
    });
    let first_edit = code.read_with(cx, |editor, cx| {
        editor.buffer().read(cx).last_transaction_id(cx).unwrap()
    });
    cx.dispatch_action(EnterCommandMode);
    cx.dispatch_action(MoveCellDown);
    check(["neighbor", "a"], cx);
    cx.dispatch_action(EnterEditMode);
    cx.simulate_keystrokes("b");
    code.update(cx, |editor, cx| {
        editor.group_until_transaction(first_edit, cx)
    });
    drop(code);
    check(["neighbor", "ab"], cx);
    cx.dispatch_action(editor::actions::Undo);
    check(["neighbor", ""], cx);
    cx.dispatch_action(editor::actions::Redo);
    check(["neighbor", "ab"], cx);

    cx.dispatch_action(EnterCommandMode);
    cx.dispatch_action(editor::actions::Undo);
    check(["ab", "neighbor"], cx);
    cx.dispatch_action(settings::ActionSequence(vec![
        Box::new(editor::actions::Redo),
        Box::new(workspace::SaveAll { save_intent: None }),
    ]));
    check(["neighbor", "ab"], cx);
    let saved = fs
        .load(Path::new(path!("/notebooks/field.ipynb")))
        .await
        .unwrap();
    assert_eq!(parsed_sources(&saved), ["neighbor", "ab"]);
    let stale = |cx: &mut VisualTestContext| {
        workspace.read_with(cx, |workspace, cx| {
            let preview = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
            let notebook = preview.read(cx).notebook.as_ref().unwrap().read(cx);
            let Some(crate::notebook::Cell::Code(cell)) = notebook.cell(1) else {
                panic!("missing moved code cell")
            };
            cell.read(cx).has_stale_outputs(cx)
        })
    };
    assert!(stale(cx));
    cx.dispatch_action(DeleteCell);
    cx.dispatch_action(editor::actions::Undo);
    check(["neighbor", "ab"], cx);
    assert!(stale(cx));
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    assert!(workspace.read_with(cx, |workspace, cx| workspace.active_item(cx).is_none()));
    open_file("field.ipynb", &workspace, cx).await;
    check(["neighbor", "ab"], cx);
}

#[gpui::test]
async fn structure_history_preserves_legacy_cells_through_edit_source(cx: &mut TestAppContext) {
    let original = NOTEBOOK
        .replace(r#""nbformat_minor": 5"#, r#""nbformat_minor": 4"#)
        .replace(r#" "id": "intro","#, "")
        .replace(r#" "id": "compute","#, "")
        .replace("Field notes", r"Field \u00e9")
        .replace(
            r#""outputs": ["#,
            r#""custom": {"untouched":42}, "outputs": [{"output_type":"display_data",
                "data":{"image/png":"iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAAAAAA6fptVAAAACklEQVR4nGP4DwABAQEAsTj2FAAAAABJRU5ErkJggg=="},"metadata":{}}, "#,
        );
    let path = Path::new(path!("/notebooks/field.ipynb"));
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": original}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(MoveCellDown);
    cx.dispatch_action(menu::SelectFirst);
    cx.dispatch_action(settings::ActionSequence(vec![
        Box::new(DeleteCell),
        Box::new(workspace::SaveAll { save_intent: None }),
    ]));
    let saved: Value = serde_json::from_str(&fs.load(path).await.unwrap()).unwrap();
    let mut expected: Value = serde_json::from_str(&original).unwrap();
    let code = expected["cells"].as_array_mut().unwrap().remove(1);
    assert_eq!(saved, expected);

    cx.dispatch_action(EditSource);
    cx.dispatch_action(OpenNotebook);
    cx.dispatch_action(editor::actions::Undo);
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    expected["cells"].as_array_mut().unwrap().insert(0, code);
    let restored = fs.load(path).await.unwrap();
    assert_eq!(serde_json::from_str::<Value>(&restored).unwrap(), expected);
    assert!(restored.contains(r"Field \u00e9"));

    cx.dispatch_action(editor::actions::Undo);
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    assert_eq!(fs.load(path).await.unwrap(), original);
    cx.dispatch_action(AddCodeBlock);
    cx.dispatch_action(EnterCommandMode);
    cx.dispatch_action(EditSource);
    workspace.update_in(cx, |workspace, window, cx| {
        let editor = workspace.active_item_as::<Editor>(cx).unwrap();
        editor.update(cx, |editor, cx| {
            let raw = editor.text(cx);
            let saved: Value = serde_json::from_str(&raw).unwrap();
            assert_eq!(saved["nbformat_minor"], 4);
            let cells = saved["cells"].as_array().unwrap();
            assert_eq!(cells.len(), 3);
            assert!(cells.iter().all(|cell| cell.get("id").is_none()));
            editor.set_text(raw.replace("from source", "from JSON"), window, cx);
        });
    });
    cx.dispatch_action(OpenNotebook);
    cx.dispatch_action(editor::actions::Undo);
    assert_eq!(
        cell_sources(&workspace, cx),
        [
            "# Field é\n![sample](attachment:sample.png)",
            "",
            "print('from JSON')"
        ]
    );
}

#[gpui::test]
async fn opening_a_notebook_at_a_position_reveals_its_source(cx: &mut TestAppContext) {
    use util::paths::PathWithPosition;
    use workspace::path_link::{OpenTarget, OpenTargetFoundBy};

    let (_, workspace, cx) =
        notebook_workspace(json!({"field.ipynb": NOTEBOOK, "field.json": NOTEBOOK}), cx).await;
    for (file, already_open, row, column) in [
        ("field.json", false, 9, 5),
        ("field.ipynb", false, 9, 5),
        ("field.ipynb", true, 9, 5),
        ("field.ipynb", false, 1, 1),
        ("field.ipynb", true, 1, 1),
    ] {
        let returned_item = if already_open {
            open_file(file, &workspace, cx).await;
            Some(workspace.update_in(cx, |workspace, window, cx| {
                let preview = workspace
                    .active_item_as::<NotebookPreview>(cx)
                    .expect("ordinary open shows cells");
                editor::items::navigate_to_file_position(
                    &preview,
                    row,
                    Some(column),
                    true,
                    window,
                    cx,
                )
                .item_id()
            }))
        } else {
            let target = OpenTarget::Path(
                PathWithPosition {
                    path: Path::new(path!("/notebooks")).join(file),
                    row: Some(row),
                    column: Some(column),
                },
                false,
                OpenTargetFoundBy::BackgroundPathResolution,
            );
            assert!(
                workspace
                    .update_in(cx, |_, window, cx| {
                        cx.spawn_in(window, async move |workspace, cx| {
                            editor::items::open_resolved_target(&workspace, &target, cx).await
                        })
                    })
                    .await
                    .unwrap()
            );
            None
        };
        cx.run_until_parked();
        let source = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .expect("source editor")
        });
        if let Some(returned_item) = returned_item {
            assert_eq!(returned_item, source.entity_id());
        }
        source.update(cx, |editor, cx| {
            let point = editor
                .selections
                .newest::<language::Point>(&editor.display_snapshot(cx))
                .head();
            assert_eq!(point, language::Point::new(row - 1, column - 1));
            assert_eq!(editor.text(cx), NOTEBOOK);
        });
        drop(source);
        cx.dispatch_action(workspace::pane::CloseActiveItem::default());
        cx.run_until_parked();
    }
}

#[gpui::test]
async fn test_notebook_preview_keeps_the_original_source(cx: &mut TestAppContext) {
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": NOTEBOOK}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.simulate_keystrokes("x");
    workspace.read_with(cx, |workspace, cx| {
        let preview = workspace
            .active_item_as::<NotebookPreview>(cx)
            .expect("notebook preview");
        let cells = preview
            .read(cx)
            .notebook
            .as_ref()
            .expect("editable notebook")
            .read(cx);
        assert_eq!(
            cells.cell(0).expect("Markdown").current_source(cx),
            "# Field notes\n![sample](attachment:sample.png)"
        );
        assert_eq!(
            cells.cell(1).expect("code").current_source(cx),
            "print('from source')"
        );
        assert!(cells.cell(2).is_none());
        assert!(!workspace.active_item(cx).expect("notebook").is_dirty(cx));
    });
    cx.dispatch_action(workspace::Save { save_intent: None });
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    assert!(workspace.read_with(cx, |workspace, cx| workspace.active_item(cx).is_none()));
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EditSource);
    let edited = NOTEBOOK.replace("Field notes", "Unsaved notes");
    let buffer_id = workspace.update_in(cx, |workspace, window, cx| {
        let editor = workspace
            .active_item_as::<Editor>(cx)
            .expect("source editor");
        editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), NOTEBOOK);
            editor.set_text(edited.clone(), window, cx);
            editor
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("source buffer")
                .entity_id()
        })
    });
    cx.update(|window, cx| {
        let source = workspace
            .read(cx)
            .active_item(cx)
            .expect("active source tab");
        let (_, action) = source
            .tab_extra_context_menu_actions(window, cx)
            .into_iter()
            .find(|(_, action)| action.partial_eq(&OpenNotebook))
            .expect("Open Notebook Editor in the source tab menu");
        window.dispatch_action(action, cx);
    });
    cx.run_until_parked();
    let code = workspace.read_with(cx, |workspace, cx| {
        let preview = workspace
            .active_item_as::<NotebookPreview>(cx)
            .expect("notebook preview");
        let notebook = preview
            .read(cx)
            .notebook
            .as_ref()
            .expect("editable notebook");
        assert!(
            notebook
                .read(cx)
                .cell(0)
                .expect("Markdown")
                .current_source(cx)
                .contains("Unsaved notes")
        );
        notebook
            .read(cx)
            .cell(1)
            .unwrap()
            .editor(cx)
            .unwrap()
            .clone()
    });
    code.update_in(cx, |editor, window, cx| {
        editor.set_text("modified", window, cx)
    });
    cx.run_until_parked();
    code.update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx)
    });
    cx.run_until_parked();
    cx.dispatch_action(EditSource);
    workspace.read_with(cx, |workspace, cx| {
        let editor = workspace
            .active_item_as::<Editor>(cx)
            .expect("source editor");
        assert!(editor.is_dirty(cx));
        let editor = editor.read(cx);
        let buffer = editor
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("source buffer");
        assert_eq!(buffer.entity_id(), buffer_id);
        assert_eq!(editor.text(cx), edited);
    });
    assert_eq!(
        fs.load_bytes(Path::new(path!("/notebooks/field.ipynb")))
            .await
            .expect("read notebook"),
        NOTEBOOK.as_bytes(),
    );
}

#[gpui::test]
async fn test_edit_and_save_cells_in_restricted_mode(cx: &mut TestAppContext) {
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": NOTEBOOK}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EditSource);
    cx.dispatch_action(OpenNotebook);
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.active_pane().read(cx).items_len(), 1);
    });
    let view = workspace.read_with(cx, |workspace, cx| {
        workspace
            .active_item_as::<NotebookPreview>(cx)
            .expect("notebook tab")
    });
    for (index, replacement) in [(0, "# Updated notes\n"), (1, "print('revised')\n")] {
        let editor = view.read_with(cx, |view, cx| {
            view.notebook
                .as_ref()
                .expect("notebook")
                .read(cx)
                .cell(index)
                .expect("cell")
                .editor(cx)
                .expect("editable")
                .clone()
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text(replacement, window, cx);
        });
        cx.run_until_parked();
    }
    assert!(view.read_with(cx, |view, cx| workspace::Item::is_dirty(view, cx)));
    assert_eq!(
        fs.load_bytes(Path::new(path!("/notebooks/field.ipynb")))
            .await
            .expect("read file"),
        NOTEBOOK.as_bytes()
    );
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    let saved = fs
        .load_bytes(Path::new(path!("/notebooks/field.ipynb")))
        .await
        .expect("read file");
    let mut expected: Value = serde_json::from_str(NOTEBOOK).expect("fixture");
    expected["cells"][0]["source"] = json!(["# Updated notes\n"]);
    expected["cells"][1]["source"] = json!(["print('revised')\n"]);
    assert_eq!(
        serde_json::from_slice::<Value>(&saved).expect("saved notebook"),
        expected
    );
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(
            workspace.active_item_as::<NotebookPreview>(cx),
            Some(view.clone())
        );
        assert!(!view.is_dirty(cx));
    });
    cx.dispatch_action(EditSource);
    workspace.read_with(cx, |workspace, cx| {
        let source = workspace
            .active_item_as::<Editor>(cx)
            .expect("source editor");
        assert_eq!(source.read(cx).text(cx).as_bytes(), saved);
    });
}

#[gpui::test]
async fn test_restore_unsaved_notebook_after_disk_edit(cx: &mut TestAppContext) {
    use workspace::item::SerializableItem;

    let (fs, original, cx) = notebook_workspace(json!({"field.ipynb": NOTEBOOK}), cx).await;
    let db = cx.update(|_, cx| workspace::WorkspaceDb::global(cx));
    let workspace_id = db.next_id().await.expect("workspace database ID");
    let (project, app_state) =
        original.read_with(cx, |ws, _| (ws.project().clone(), ws.app_state().clone()));
    let (workspace, cx) = cx.add_window_view(|window, cx| {
        Workspace::new(Some(workspace_id), project, app_state, window, cx)
    });
    open_file("field.ipynb", &workspace, cx).await;
    let (item_id, serialized) = workspace.update_in(cx, |ws, window, cx| {
        let view = ws
            .active_item_as::<NotebookPreview>(cx)
            .expect("notebook tab");
        let cell_editor = view
            .read(cx)
            .notebook
            .as_ref()
            .expect("notebook")
            .read(cx)
            .cell(1)
            .expect("code cell")
            .editor(cx)
            .expect("editor")
            .clone();
        cell_editor.update(cx, |editor, cx| editor.set_text("unsaved = 1", window, cx));
        let id = view.entity_id().as_u64();
        let serialized = view.update(cx, |view, cx| view.serialize(ws, id, false, cx).unwrap());
        (id, serialized)
    });
    serialized.await.expect("serialize pending cell edit");
    let snapshot = cx.update(|_, cx| {
        super::persistence::NotebookPreviewDb::global(cx)
            .get_notebook(item_id, workspace_id)
            .unwrap()
            .unwrap()
            .1
            .unwrap()
    });
    assert_eq!(
        snapshot,
        NOTEBOOK.replace("print('from source')", "unsaved = 1")
    );
    cx.dispatch_action(MoveCellDown);
    cx.run_until_parked();
    cx.executor()
        .advance_clock(workspace::SERIALIZATION_THROTTLE_TIME);
    cx.run_until_parked();

    let path = Path::new(path!("/notebooks/field.ipynb"));
    assert_eq!(
        fs.load_bytes(path).await.expect("original file"),
        NOTEBOOK.as_bytes()
    );
    let disk_contents = NOTEBOOK.replace("print('from source')", "external = 2");
    fs.insert_file(path, disk_contents.as_bytes().to_vec())
        .await;
    for recover in [true, false] {
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .session
                        .get_or_insert_default()
                        .restore_unsaved_buffers = Some(recover);
                });
            });
        });
        let project = Project::test(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
        let view = cx
            .update(|window, cx| {
                NotebookPreview::deserialize(
                    project,
                    workspace.downgrade(),
                    workspace_id,
                    item_id,
                    window,
                    cx,
                )
            })
            .await
            .expect("restore notebook in a fresh project");
        view.read_with(cx, |view, cx| {
            let cell = view
                .notebook
                .as_ref()
                .expect("notebook")
                .read(cx)
                .cell(if recover { 0 } else { 1 })
                .expect("code");
            assert_eq!(
                cell.current_source(cx),
                if recover {
                    "unsaved = 1"
                } else {
                    "external = 2"
                }
            );
            assert_eq!(view.editor.is_dirty(cx), recover);
            assert_eq!(view.editor.has_conflict(cx), recover);
        });
        assert_eq!(
            fs.load_bytes(path).await.expect("unchanged file"),
            disk_contents.as_bytes()
        );
    }
}

#[gpui::test]
async fn test_splitting_notebook_source_keeps_both_editors(cx: &mut TestAppContext) {
    let (_, workspace, cx) =
        notebook_workspace(json!({"field.ipynb": NOTEBOOK, "notes.txt": "notes"}), cx).await;
    open_file("notes.txt", &workspace, cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    cx.dispatch_action(EditSource);
    cx.dispatch_action(workspace::pane::SplitRight::default());
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.panes().len(), 2);
        for pane in workspace.panes() {
            let editor = pane
                .read(cx)
                .active_item()
                .expect("tab")
                .downcast::<Editor>()
                .expect("splitting notebook source should keep the JSON editor");
            assert_eq!(editor.read(cx).text(cx), NOTEBOOK);
        }
    });
    cx.dispatch_action(workspace::ActivatePane(0));
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    cx.dispatch_action(workspace::ActivatePane(1));
    cx.dispatch_action(workspace::MoveItemToPane {
        destination: 0,
        focus: true,
        clone: false,
    });
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.panes().len(), 1);
        let editor = workspace
            .active_item_as::<Editor>(cx)
            .expect("moved source editor");
        assert_eq!(editor.read(cx).text(cx), NOTEBOOK);
    });
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    open_file("field.ipynb", &workspace, cx).await;
    assert!(workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<NotebookPreview>(cx).is_some()
    }));
}

#[gpui::test]
async fn test_cell_source_undo_restores_the_saved_notebook(cx: &mut TestAppContext) {
    let first_line = format!("value = '{}'", "x".repeat(1024));
    let source = format!(r#"[ "{first_line}\r\n", "sample = \u00317" ]"#);
    let mixed_markdown =
        r##"["# Field notes\r\n", "Another line\n", "![sample](attachment:sample.png)"]"##;
    let original = NOTEBOOK
        .replace(
            r##"["# Field notes\n", "![sample](attachment:sample.png)"]"##,
            mixed_markdown,
        )
        .replace(r#"["print('from source')"]"#, &source);
    let (_, workspace, cx) = notebook_workspace(json!({"field.ipynb": original}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<NotebookPreview>(cx).unwrap()
    });
    let (raw, code, markdown) = view.read_with(cx, |view, cx| {
        let notebook = view.notebook.as_ref().unwrap().read(cx);
        (
            view.editor.clone(),
            notebook.cell(1).unwrap().editor(cx).unwrap().clone(),
            notebook.cell(0).unwrap().editor(cx).unwrap().clone(),
        )
    });
    let assert_source = |expected: &str, dirty, cx: &mut VisualTestContext| {
        assert_eq!(raw.read_with(cx, |editor, cx| editor.text(cx)), expected);
        assert_eq!(
            view.read_with(cx, |view, cx| workspace::Item::is_dirty(view, cx)),
            dirty
        );
    };
    let undo = |cell: &Entity<Editor>, cx: &mut VisualTestContext| {
        cell.update_in(cx, |editor, window, cx| {
            editor.undo(&editor::actions::Undo, window, cx)
        });
        cx.run_until_parked();
    };
    let redo = |cell: &Entity<Editor>, cx: &mut VisualTestContext| {
        cell.update_in(cx, |editor, window, cx| {
            editor.redo(&editor::actions::Redo, window, cx)
        });
        cx.run_until_parked();
    };
    let group = code.update_in(cx, |editor, window, cx| {
        editor.buffer().update(cx, |buffer, cx| {
            buffer.set_group_interval(std::time::Duration::from_secs(10), cx)
        });
        editor.move_to_end(&Default::default(), window, cx);
        editor.insert("8", window, cx);
        editor.buffer().read(cx).last_transaction_id(cx).unwrap()
    });
    cx.run_until_parked();
    let edited = original.replace(&source, &format!(r#"["{first_line}\r\n","sample = 178"]"#));
    assert_source(&edited, true, cx);
    code.update_in(cx, |editor, window, cx| editor.insert("9", window, cx));
    cx.run_until_parked();
    code.update_in(cx, |editor, window, cx| {
        editor.buffer().update(cx, |buffer, cx| {
            buffer.set_group_interval(std::time::Duration::ZERO, cx)
        });
        editor.insert("0", window, cx);
    });
    cx.run_until_parked();
    code.update(cx, |editor, cx| editor.group_until_transaction(group, cx));
    let grouped = edited.replace("sample = 178", "sample = 17890");
    assert_source(&grouped, true, cx);
    undo(&code, cx);
    assert_source(&original, false, cx);
    redo(&code, cx);
    assert_source(&grouped, true, cx);

    markdown.update_in(cx, |editor, window, cx| {
        editor.set_text("other\nedit", window, cx)
    });
    cx.run_until_parked();
    undo(&code, cx);
    let other_edit = original.replace(mixed_markdown, r#"["other\r\n","edit"]"#);
    assert_source(&other_edit, true, cx);
    undo(&markdown, cx);
    assert_source(&original, false, cx);

    redo(&code, cx);
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    undo(&code, cx);
    assert_source(&original, true, cx);
    markdown.update_in(cx, |editor, window, cx| {
        editor.set_text("new branch", window, cx)
    });
    cx.run_until_parked();
    undo(&markdown, cx);
    redo(&code, cx);
    assert_source(&grouped, false, cx);
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    cx.run_until_parked();
    assert!(workspace.read_with(cx, |workspace, cx| workspace.active_item(cx).is_none()));
}

#[gpui::test]
async fn cell_undo_after_structure_undo_restores_the_exact_source(cx: &mut TestAppContext) {
    let original = r#"{"nbformat": 4, "nbformat_minor": 5, "metadata": {},
  "cells": [
    {"cell_type": "markdown", "id": "short", "metadata": {}, "source": ["a\r\n", "b\n", "c"]},
    {"cell_type": "code", "id": "long", "metadata": {}, "execution_count": null, "outputs": [],
     "source": ["neighbor = 'long enough that the short cell is the one that moves'"]}
  ]
}
"#;
    let path = Path::new(path!("/notebooks/field.ipynb"));
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": original}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let sources = cell_sources(&workspace, cx);
    let view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<NotebookPreview>(cx).unwrap()
    });
    let cell_editor = |index: usize, cx: &mut VisualTestContext| {
        view.read_with(cx, |view, cx| {
            let notebook = view.notebook.as_ref().unwrap().read(cx);
            notebook.cell(index).unwrap().editor(cx).unwrap().clone()
        })
    };
    let undo_cell = |index: usize, cx: &mut VisualTestContext| {
        cell_editor(index, cx).update_in(cx, |editor, window, cx| {
            editor.undo(&editor::actions::Undo, window, cx)
        });
        cx.run_until_parked();
    };
    let assert_source = |expected: &str, dirty, cx: &mut VisualTestContext| {
        assert_eq!(
            view.read_with(cx, |view, cx| view.editor.read(cx).text(cx)),
            expected
        );
        assert_eq!(
            view.read_with(cx, |view, cx| workspace::Item::is_dirty(view, cx)),
            dirty
        );
    };

    cell_editor(0, cx).update_in(cx, |editor, window, cx| {
        editor.move_to_end(&Default::default(), window, cx);
        editor.insert("d", window, cx);
    });
    cx.run_until_parked();
    cx.dispatch_action(MoveCellDown);
    assert!(cell_sources(&workspace, cx)[1].ends_with("cd"));
    cx.dispatch_action(editor::actions::Undo);
    assert!(cell_sources(&workspace, cx)[0].ends_with("cd"));
    undo_cell(0, cx);
    assert_source(original, false, cx);

    cell_editor(0, cx).update_in(cx, |editor, window, cx| editor.set_text("", window, cx));
    cx.run_until_parked();
    cx.dispatch_action(DeleteCell);
    assert_eq!(cell_sources(&workspace, cx).len(), 1);
    cx.dispatch_action(editor::actions::Undo);
    assert_eq!(cell_sources(&workspace, cx)[0], "");
    undo_cell(0, cx);
    assert_source(original, false, cx);

    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    assert_eq!(fs.load(path).await.unwrap(), original);
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    cx.run_until_parked();
    assert!(workspace.read_with(cx, |workspace, cx| workspace.active_item(cx).is_none()));
    open_file("field.ipynb", &workspace, cx).await;
    assert_eq!(cell_sources(&workspace, cx), sources);
}

#[gpui::test]
async fn cell_edits_keep_untouched_line_endings_through_saves_and_structure_changes(
    cx: &mut TestAppContext,
) {
    let original = r#"{"nbformat": 4, "nbformat_minor": 5, "metadata": {},
  "cells": [
    {"cell_type": "markdown", "id": "short", "metadata": {}, "source": ["a\r\n", "b\n", "c"]},
    {"cell_type": "code", "id": "long", "metadata": {}, "execution_count": null, "outputs": [],
     "source": ["neighbor = 'long enough that the short cell is the one that moves'"]}
  ]
}
"#;
    let path = Path::new(path!("/notebooks/field.ipynb"));
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": original}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<NotebookPreview>(cx).unwrap()
    });
    let short_editor = |cx: &mut VisualTestContext| {
        view.read_with(cx, |view, cx| {
            let notebook = view.notebook.as_ref().unwrap().read(cx);
            (0..)
                .map_while(|index| notebook.cell(index))
                .find(|cell| cell.id(cx).to_string() == "short")
                .unwrap()
                .editor(cx)
                .unwrap()
                .clone()
        })
    };
    let field = |text: &str| {
        let object = &text[text.find(r#""id": "short""#).unwrap()..];
        let line = object.lines().next().unwrap();
        let source = &line[line.find(r#""source": "#).unwrap() + r#""source": "#.len()..];
        source[..=source.rfind(']').unwrap()].to_owned()
    };
    let raw_field = |cx: &mut VisualTestContext| {
        field(&view.read_with(cx, |view, cx| view.editor.read(cx).text(cx)))
    };
    let append = |text: &str, cx: &mut VisualTestContext| {
        short_editor(cx).update_in(cx, |editor, window, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                buffer.set_group_interval(std::time::Duration::ZERO, cx)
            });
            editor.move_to_end(&Default::default(), window, cx);
            editor.insert(text, window, cx);
        });
        cx.run_until_parked();
    };
    let undo_cell = |cx: &mut VisualTestContext| {
        short_editor(cx).update_in(cx, |editor, window, cx| {
            editor.undo(&editor::actions::Undo, window, cx)
        });
        cx.run_until_parked();
    };
    let save = |cx: &mut VisualTestContext| {
        cx.dispatch_action(workspace::SaveAll { save_intent: None });
        cx.run_until_parked();
    };
    let short_first = |text: &str| text.find(r#""short""#) < text.find(r#""long""#);
    let edited = r#"["a\r\n","b\n","cd"]"#;

    append("d", cx);
    assert_eq!(raw_field(cx), edited);
    save(cx);
    let saved = fs.load(path).await.unwrap();
    assert_eq!(field(&saved), edited);

    cx.dispatch_action(MoveCellDown);
    save(cx);
    let saved = fs.load(path).await.unwrap();
    assert!(!short_first(&saved));
    assert_eq!(field(&saved), edited);

    append("e", cx);
    assert_eq!(raw_field(cx), r#"["a\r\n","b\n","cde"]"#);
    undo_cell(cx);
    assert_eq!(raw_field(cx), edited);

    cx.dispatch_action(editor::actions::Undo);
    save(cx);
    let saved = fs.load(path).await.unwrap();
    assert!(short_first(&saved));
    assert_eq!(field(&saved), edited);
    undo_cell(cx);
    assert_eq!(
        view.read_with(cx, |view, cx| view.editor.read(cx).text(cx)),
        original
    );
    assert!(view.read_with(cx, |view, cx| workspace::Item::is_dirty(view, cx)));

    short_editor(cx).update(cx, |editor, cx| {
        let buffer = editor.buffer().read(cx).as_singleton().unwrap();
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "x"), (2..2, "y")], None, cx)
        });
    });
    cx.run_until_parked();
    let multi_cursor = r#"["xa\r\n","yb\n","c"]"#;
    assert_eq!(raw_field(cx), multi_cursor);
    cx.dispatch_action(DeleteCell);
    assert_eq!(cell_sources(&workspace, cx).len(), 1);
    cx.dispatch_action(editor::actions::Undo);
    assert_eq!(raw_field(cx), multi_cursor);
    append("z\nw", cx);
    let typed = r#"["xa\r\n","yb\n","cz\r\n","w"]"#;
    assert_eq!(raw_field(cx), typed);
    save(cx);
    assert_eq!(field(&fs.load(path).await.unwrap()), typed);
}

#[gpui::test]
async fn batched_cell_edits_keep_line_endings_for_each_undo_step(cx: &mut TestAppContext) {
    let original = r#"{"nbformat": 4, "nbformat_minor": 5, "metadata": {},
  "cells": [{"cell_type": "markdown", "id": "short", "metadata": {}, "source": ["a\r\n", "b\n", "c"]}]
}
"#;
    let (_, workspace, cx) = notebook_workspace(json!({"field.ipynb": original}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<NotebookPreview>(cx).unwrap()
    });
    let (raw, notebook, cell) = view.read_with(cx, |view, cx| {
        let notebook = view.notebook.clone().unwrap();
        let editor = notebook
            .read(cx)
            .cell(0)
            .unwrap()
            .editor(cx)
            .unwrap()
            .clone();
        let cell = editor.read(cx).buffer().read(cx).as_singleton().unwrap();
        (view.editor.clone(), notebook, cell)
    });
    let raw_text = |cx: &mut VisualTestContext| raw.read_with(cx, |editor, cx| editor.text(cx));
    let undo = |cx: &mut VisualTestContext| {
        cell.update(cx, |cell, cx| assert!(cell.undo(cx).is_some()));
        cx.run_until_parked();
    };

    notebook.update(cx, |notebook, cx| {
        cell.update(cx, |cell, cx| {
            cell.finalize_last_transaction();
            cell.edit([(5..5, "q")], None, cx);
            cell.finalize_last_transaction();
            cell.edit([(3..4, "")], None, cx);
        });
        notebook.flush_cell_edits(cx).unwrap();
    });
    cx.run_until_parked();
    let appended = original.replace(r#"["a\r\n", "b\n", "c"]"#, r#"["a\r\n","b\n","cq"]"#);
    assert_eq!(
        raw_text(cx),
        original.replace(r#"["a\r\n", "b\n", "c"]"#, r#"["a\r\n","bcq"]"#)
    );
    undo(cx);
    assert_eq!(raw_text(cx), appended);
    undo(cx);
    assert_eq!(raw_text(cx), original);
    assert!(!view.read_with(cx, |view, cx| workspace::Item::is_dirty(view, cx)));
}

#[gpui::test]
async fn switching_to_source_preserves_cell_undo_until_the_raw_file_changes(
    cx: &mut TestAppContext,
) {
    let (_, workspace, cx) = notebook_workspace(json!({"field.ipynb": NOTEBOOK}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let code = |cx: &mut VisualTestContext| {
        workspace.read_with(cx, |workspace, cx| {
            let preview = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
            preview
                .read(cx)
                .notebook
                .as_ref()
                .unwrap()
                .read(cx)
                .cell(1)
                .unwrap()
                .editor(cx)
                .unwrap()
                .clone()
        })
    };
    code(cx).update_in(cx, |editor, window, cx| {
        editor.set_text("cell edit", window, cx)
    });
    cx.run_until_parked();
    cx.dispatch_action(EditSource);
    cx.run_until_parked();
    cx.dispatch_action(OpenNotebook);
    cx.run_until_parked();
    code(cx).update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx)
    });
    cx.run_until_parked();
    assert_eq!(
        code(cx).read_with(cx, |editor, cx| editor.text(cx)),
        "print('from source')"
    );
    assert!(!workspace.read_with(cx, |workspace, cx| {
        workspace.active_item(cx).unwrap().is_dirty(cx)
    }));
    code(cx).update_in(cx, |editor, window, cx| {
        editor.redo(&editor::actions::Redo, window, cx)
    });
    cx.run_until_parked();
    assert_eq!(
        code(cx).read_with(cx, |editor, cx| editor.text(cx)),
        "cell edit"
    );
    workspace.update_in(cx, |workspace, window, cx| {
        let item = workspace.active_item(cx).unwrap();
        editor::items::navigate_to_file_position(item.as_ref(), 5, Some(1), true, window, cx);
    });
    cx.run_until_parked();
    let positioned = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<Editor>(cx).unwrap()
    });
    positioned.update(cx, |editor, cx| {
        let point = editor
            .selections
            .newest::<language::Point>(&editor.display_snapshot(cx))
            .head();
        assert_eq!(point, language::Point::new(4, 0));
    });
    cx.dispatch_action(OpenNotebook);
    cx.run_until_parked();
    code(cx).update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx)
    });
    cx.run_until_parked();
    assert_eq!(
        code(cx).read_with(cx, |editor, cx| editor.text(cx)),
        "print('from source')"
    );
    assert!(!workspace.read_with(cx, |workspace, cx| {
        workspace.active_item(cx).unwrap().is_dirty(cx)
    }));
    code(cx).update_in(cx, |editor, window, cx| {
        editor.redo(&editor::actions::Redo, window, cx)
    });
    cx.run_until_parked();
    cx.dispatch_action(EditSource);
    cx.run_until_parked();
    let source = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<Editor>(cx).unwrap()
    });
    source.update_in(cx, |editor, window, cx| {
        let changed = editor.text(cx).replace("cell edit", "raw edit");
        editor.set_text(changed, window, cx)
    });
    cx.run_until_parked();
    cx.dispatch_action(OpenNotebook);
    cx.run_until_parked();
    code(cx).update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx)
    });
    cx.run_until_parked();
    assert_eq!(
        code(cx).read_with(cx, |editor, cx| editor.text(cx)),
        "raw edit"
    );
}

#[gpui::test]
async fn test_unpreviewable_files_open_in_the_editor(cx: &mut TestAppContext) {
    let malformed = "{not valid JSON";
    let unsupported = r#"{"nbformat":99,"nbformat_minor":0,"cells":[],"metadata":{}}"#;
    let unsupported_output =
        NOTEBOOK.replace(r#""output_type":"stream""#, r#""output_type":"unknown""#);
    let invalid_heading = r#"{"nbformat":3,"worksheets":[{"cells":[{"cell_type":"heading","level":7,"metadata":{},"source":"Title"}]}]}"#;
    let (_, workspace, cx) = notebook_workspace(
        json!({"broken.ipynb": malformed, "future.ipynb": unsupported, "output.ipynb": unsupported_output, "heading.ipynb": invalid_heading, "document.json": NOTEBOOK}),
        cx,
    )
    .await;
    for (name, source) in [
        ("broken.ipynb", malformed),
        ("future.ipynb", unsupported),
        ("output.ipynb", unsupported_output.as_str()),
        ("heading.ipynb", invalid_heading),
        ("document.json", NOTEBOOK),
    ] {
        open_file(name, &workspace, cx).await;
        let editor_id = workspace.read_with(cx, |workspace, cx| {
            let editor = workspace.active_item_as::<Editor>(cx).expect("raw editor");
            assert_eq!(editor.read(cx).text(cx), source);
            editor.entity_id()
        });
        cx.dispatch_action(OpenNotebook);
        workspace.read_with(cx, |workspace, cx| {
            let editor = workspace
                .active_item_as::<Editor>(cx)
                .expect("still editing raw source");
            assert_eq!(editor.entity_id(), editor_id);
        });
    }
    open_file("broken.ipynb", &workspace, cx).await;
    workspace.update_in(cx, |workspace, window, cx| {
        let editor = workspace.active_item_as::<Editor>(cx).expect("raw editor");
        editor.update(cx, |editor, cx| editor.set_text(NOTEBOOK, window, cx));
    });
    cx.dispatch_action(OpenNotebook);
    workspace.read_with(cx, |workspace, cx| {
        assert!(workspace.active_item_as::<NotebookPreview>(cx).is_some());
    });
}

#[gpui::test]
async fn an_unrelated_cell_edit_does_not_unblock_save(cx: &mut TestAppContext) {
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": NOTEBOOK}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<NotebookPreview>(cx).unwrap()
    });
    let (raw, code, markdown) = view.read_with(cx, |view, cx| {
        let notebook = view.notebook.as_ref().unwrap().read(cx);
        (
            view.editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap(),
            notebook.cell(1).unwrap().editor(cx).unwrap().clone(),
            notebook.cell(0).unwrap().editor(cx).unwrap().clone(),
        )
    });
    code.update_in(cx, |editor, window, cx| {
        editor.set_text("first edit", window, cx)
    });
    cx.run_until_parked();
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    raw.update(cx, |buffer, _| {
        let transaction = buffer.peek_undo_stack().unwrap().transaction_id();
        assert!(buffer.forget_transaction(transaction).is_some());
    });
    code.update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx)
    });
    cx.run_until_parked();
    assert!(!raw.read_with(cx, |buffer, _| buffer.is_dirty()));
    assert!(view.read_with(cx, |view, cx| workspace::Item::is_dirty(view, cx)));
    markdown.update_in(cx, |editor, window, cx| {
        editor.set_text("other edit", window, cx)
    });
    cx.run_until_parked();
    let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());
    let save = view.update_in(cx, |view, window, cx| {
        workspace::Item::save(view, Default::default(), project, window, cx)
    });
    assert!(save.await.is_err());
    assert_eq!(
        fs.load_bytes(Path::new(path!("/notebooks/field.ipynb")))
            .await
            .unwrap(),
        NOTEBOOK
            .replace("print('from source')", "first edit")
            .as_bytes()
    );
}

#[gpui::test]
async fn batched_cell_history_saves_the_visible_source(cx: &mut TestAppContext) {
    let original = NOTEBOOK.replace("print('from source')", "sample = 1");
    let (fs, workspace, cx) = notebook_workspace(json!({"field.ipynb": original}), cx).await;
    open_file("field.ipynb", &workspace, cx).await;
    let editors = |cx: &mut VisualTestContext| {
        workspace.read_with(cx, |workspace, cx| {
            let view = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
            let view = view.read(cx);
            let code = view
                .notebook
                .as_ref()
                .unwrap()
                .read(cx)
                .cell(1)
                .unwrap()
                .editor(cx)
                .unwrap()
                .clone();
            (code, view.editor.clone())
        })
    };
    let (code, raw) = editors(cx);
    code.update_in(cx, |editor, window, cx| {
        editor.move_to_end(&Default::default(), window, cx);
        editor.insert("1", window, cx);
        editor.finalize_last_transaction(cx);
    });
    cx.run_until_parked();
    code.update_in(cx, |editor, window, cx| editor.insert("2", window, cx));
    cx.run_until_parked();
    code.update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx);
        editor.undo(&editor::actions::Undo, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        code.read_with(cx, |editor, cx| editor.text(cx)),
        "sample = 1"
    );
    assert_eq!(raw.read_with(cx, |editor, cx| editor.text(cx)), original);
    assert!(!raw.read_with(cx, |editor, cx| workspace::Item::is_dirty(editor, cx)));
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    assert_eq!(
        fs.load_bytes(Path::new(path!("/notebooks/field.ipynb")))
            .await
            .unwrap(),
        original.as_bytes()
    );
    code.update_in(cx, |editor, window, cx| {
        editor.redo(&editor::actions::Redo, window, cx);
        editor.redo(&editor::actions::Redo, window, cx);
    });
    cx.run_until_parked();
    code.update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx)
    });
    cx.run_until_parked();
    let saved = original.replace("sample = 1", "sample = 11");
    assert_eq!(
        code.read_with(cx, |editor, cx| editor.text(cx)),
        "sample = 11"
    );
    assert_eq!(raw.read_with(cx, |editor, cx| editor.text(cx)), saved);
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    assert_eq!(
        fs.load_bytes(Path::new(path!("/notebooks/field.ipynb")))
            .await
            .unwrap(),
        saved.as_bytes()
    );
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    open_file("field.ipynb", &workspace, cx).await;
    let (code, raw) = editors(cx);
    assert_eq!(
        code.read_with(cx, |editor, cx| editor.text(cx)),
        "sample = 11"
    );
    assert_eq!(raw.read_with(cx, |editor, cx| editor.text(cx)), saved);
    code.update_in(cx, |editor, window, cx| {
        editor.move_to_end(&Default::default(), window, cx);
        editor.insert("2", window, cx);
        editor.finalize_last_transaction(cx);
    });
    cx.run_until_parked();
    code.update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx);
        editor.insert("3", window, cx);
        editor.finalize_last_transaction(cx);
        editor.insert("4", window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        code.read_with(cx, |editor, cx| editor.text(cx)),
        "sample = 1134"
    );
    assert_eq!(
        raw.read_with(cx, |editor, cx| editor.text(cx)),
        original.replace("sample = 1", "sample = 1134")
    );
    code.update_in(cx, |editor, window, cx| {
        editor.undo(&editor::actions::Undo, window, cx)
    });
    cx.run_until_parked();
    assert_eq!(
        code.read_with(cx, |editor, cx| editor.text(cx)),
        "sample = 113"
    );
    assert_eq!(
        raw.read_with(cx, |editor, cx| editor.text(cx)),
        original.replace("sample = 1", "sample = 113")
    );
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    open_file("field.ipynb", &workspace, cx).await;
    assert_eq!(
        editors(cx).0.read_with(cx, |editor, cx| editor.text(cx)),
        "sample = 113"
    );
    let (code, _) = editors(cx);
    let view = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<NotebookPreview>(cx).unwrap()
    });
    let save = view.update_in(cx, |view, window, cx| {
        let project = view.editor.read(cx).project().unwrap().clone();
        code.update(cx, |editor, cx| {
            editor.move_to_end(&Default::default(), window, cx);
            editor.insert("5", window, cx);
        });
        workspace::Item::save(view, Default::default(), project, window, cx)
    });
    save.await.unwrap();
    assert_eq!(
        fs.load_bytes(Path::new(path!("/notebooks/field.ipynb")))
            .await
            .unwrap(),
        original.replace("sample = 1", "sample = 1135").as_bytes()
    );
    let save_as = view.update_in(cx, |view, window, cx| {
        let project = view.editor.read(cx).project().unwrap().clone();
        let mut path = workspace::Item::active_project_path(view, cx).unwrap();
        path.path = rel_path("copy.ipynb").into();
        code.update(cx, |editor, cx| editor.insert("6", window, cx));
        workspace::Item::save_as(view, project, path, window, cx)
    });
    save_as.await.unwrap();
    assert_eq!(
        fs.load_bytes(Path::new(path!("/notebooks/copy.ipynb")))
            .await
            .unwrap(),
        original.replace("sample = 1", "sample = 11356").as_bytes()
    );
}

#[gpui::test]
async fn save_all_in_an_action_sequence_includes_a_new_cell_edit(cx: &mut TestAppContext) {
    let original = NOTEBOOK.replace("print('from source')", "sample = 1");
    let (fs, workspace, cx) = notebook_workspace(
        json!({"field.ipynb": original, "control.txt": "sample = 1"}),
        cx,
    )
    .await;
    for name in ["control.txt", "field.ipynb"] {
        open_file(name, &workspace, cx).await;
        let editor = workspace.read_with(cx, |workspace, cx| {
            workspace.active_item_as::<Editor>(cx).unwrap_or_else(|| {
                let preview = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
                preview
                    .read(cx)
                    .notebook
                    .as_ref()
                    .unwrap()
                    .read(cx)
                    .cell(1)
                    .unwrap()
                    .editor(cx)
                    .unwrap()
                    .clone()
            })
        });
        cx.focus(&editor);
        editor.update_in(cx, |editor, window, cx| {
            editor.move_to_end(&Default::default(), window, cx)
        });
        cx.dispatch_action(settings::ActionSequence(vec![
            Box::new(editor::actions::Newline),
            Box::new(workspace::SaveAll { save_intent: None }),
        ]));
        cx.run_until_parked();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "sample = 1\n"
        );
        let expected = if name == "field.ipynb" {
            original.replace("sample = 1", r#"sample = 1\n"#)
        } else {
            format!("sample = 1{}", language::LineEnding::default().as_str())
        };
        assert_eq!(
            fs.load_bytes(&Path::new(path!("/notebooks")).join(name))
                .await
                .unwrap(),
            expected.as_bytes()
        );
    }
}

#[gpui::test]
async fn saved_rich_outputs_render_passively_and_save_unchanged(cx: &mut TestAppContext) {
    use crate::outputs::Output;
    use gpui::{px, size};

    let outputs = r#"[
      {"output_type":"display_data", "metadata":{}, "data":{"image/jpeg":"/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDABALDA4MChAODQ4SERATGCgaGBYWGDEjJR0oOjM9PDkzODdASFxOQERXRTc4UG1RV19iZ2hnPk1xeXBkeFxlZ2P/wAALCAABAAIBAREA/8QAFAABAAAAAAAAAAAAAAAAAAAABf/EABQQAQAAAAAAAAAAAAAAAAAAAAD/2gAIAQEAAD8AMf/Z"}},
      {"output_type":"display_data", "metadata":{}, "data":{"application/json":{"marker":[1, 2.50]}}},
      {"output_type":"execute_result", "execution_count":7, "metadata":{}, "data":{"text/plain":"plain",
        "text/markdown":["**Summary** ", "![remote](https://example.com/remote.png) [site](https://example.com)"]}},
      {"output_type":"execute_result", "execution_count":7, "metadata":{}, "data":{"text/plain":"frame",
        "text/html":"<table></table>", "application/vnd.dataresource+json":{
          "schema":{"fields":[{"name":"id","type":"integer"}]}, "data":[{"id":1}]}}},
      {"output_type":"display_data", "metadata":{}, "data":{"text/plain":"html fallback",
        "text/html":"<img src='https://example.com/remote.png'>"}}
    ]"#;
    let file = NOTEBOOK.replace(
        r#"[{"output_type":"stream", "name":"stdout", "text":["saved result\n"]}]"#,
        outputs,
    );
    assert!(file.contains(outputs));
    let (fs, workspace, cx) = notebook_workspace(json!({"rich.ipynb": file}), cx).await;
    cx.update(|_, cx| {
        cx.set_http_client(http_client::FakeHttpClient::create(|request| async move {
            panic!("saved output fetched {}", request.uri())
        }));
    });
    cx.simulate_resize(size(px(1200.), px(1600.)));
    let check_outputs = |cx: &mut VisualTestContext| {
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            let preview = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
            let notebook = preview.read(cx).notebook.as_ref().unwrap().read(cx);
            let cell = (0..)
                .map_while(|index| notebook.cell(index))
                .find_map(|cell| match cell {
                    crate::notebook::Cell::Code(cell)
                        if !cell.read(cx).output_views().is_empty() =>
                    {
                        Some(cell)
                    }
                    _ => None,
                })
                .expect("code cell with saved outputs");
            let [
                Output::Image { .. },
                Output::Json { .. },
                Output::Markdown { .. },
                Output::Table { .. },
                Output::Plain { content, .. },
            ] = cell.read(cx).output_views()
            else {
                panic!("expected JPEG, JSON, Markdown, table, and plain fallback for HTML")
            };
            assert_eq!(content.read(cx).full_text(cx), "html fallback\n");
        })
    };

    open_file("rich.ipynb", &workspace, cx).await;
    check_outputs(cx);
    cx.dispatch_action(AddCodeBlock);
    cx.simulate_keystrokes("x");
    cx.dispatch_action(workspace::SaveAll { save_intent: None });
    cx.run_until_parked();
    let saved = fs
        .load(Path::new(path!("/notebooks/rich.ipynb")))
        .await
        .unwrap();
    assert!(
        saved.contains(outputs),
        "saved outputs keep their raw bytes"
    );
    assert!(cell_sources(&workspace, cx).contains(&"x".to_string()));
    let saved_cells = serde_json::from_str::<Value>(&saved).unwrap()["cells"].clone();
    assert_eq!(saved_cells.as_array().map(Vec::len), Some(3));

    cx.dispatch_action(workspace::pane::CloseActiveItem::default());
    cx.run_until_parked();
    open_file("rich.ipynb", &workspace, cx).await;
    check_outputs(cx);
}
