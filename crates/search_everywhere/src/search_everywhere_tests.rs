use super::*;
use gpui::{TestAppContext, VisualTestContext};
use serde_json::json;
use util::{path, rel_path::rel_path};
use workspace::{AppState, MultiWorkspace};

fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
    cx.update(|cx| {
        let state = AppState::test(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        super::init(cx);
        state
    })
}

async fn setup(cx: &mut TestAppContext) -> (Entity<Workspace>, &mut VisualTestContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "first.txt": "alpha needle\n",
                "second.txt": "    beta\n",
                "src": { "main.rs": "fn main() {}\n" },
            }),
        )
        .await;
    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    (workspace, cx)
}

async fn open_file(path: &str, workspace: &Entity<Workspace>, cx: &mut VisualTestContext) {
    let worktree_id = workspace.update(cx, |workspace, cx| {
        workspace
            .project()
            .read(cx)
            .worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .id()
    });
    let project_path = ProjectPath {
        worktree_id,
        path: rel_path(path).into(),
    };
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(project_path, None, true, window, cx)
        })
        .await
        .unwrap();
}

#[track_caller]
fn active_picker(
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Option<Entity<Picker<SearchEverywhereDelegate>>> {
    workspace.update(cx, |workspace, cx| {
        workspace
            .active_modal::<SearchEverywhere>(cx)
            .map(|modal| modal.read(cx).picker.clone())
    })
}

fn open(
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<Picker<SearchEverywhereDelegate>> {
    cx.dispatch_action(Toggle);
    cx.run_until_parked();
    active_picker(workspace, cx).expect("search everywhere is not open")
}

fn type_query(
    picker: &Entity<Picker<SearchEverywhereDelegate>>,
    query: &str,
    cx: &mut VisualTestContext,
) {
    picker.update_in(cx, |picker, window, cx| picker.set_query(query, window, cx));
    cx.run_until_parked();
}

fn labels(
    picker: &Entity<Picker<SearchEverywhereDelegate>>,
    cx: &mut VisualTestContext,
) -> Vec<String> {
    picker.read_with(cx, |picker, _| {
        let delegate = &picker.delegate;
        delegate
            .matches
            .iter()
            .map(|search_match| match search_match {
                SearchMatch::File(path_match) => {
                    format!("file:{}", path_match.path.as_unix_str())
                }
                SearchMatch::Symbol { symbol, .. } => format!("symbol:{}", symbol.name),
                SearchMatch::Action { action_ix, .. } => {
                    format!("action:{}", delegate.actions[*action_ix].name)
                }
                SearchMatch::Text(text_match) => format!(
                    "text:{}:{}",
                    text_match.project_path.path.as_unix_str(),
                    text_match.line
                ),
                SearchMatch::More(group) => format!("more:{}", group.label()),
            })
            .collect()
    })
}

fn current_tab(
    picker: &Entity<Picker<SearchEverywhereDelegate>>,
    cx: &mut VisualTestContext,
) -> SearchTab {
    picker.read_with(cx, |picker, _| picker.delegate.tab)
}

/// Regression: opening with a non-empty navigation history used to read the
/// workspace while it was being updated and panic.
#[gpui::test]
async fn test_opens_with_recent_files(cx: &mut TestAppContext) {
    let (workspace, cx) = setup(cx).await;
    open_file("first.txt", &workspace, cx).await;
    open_file("second.txt", &workspace, cx).await;

    let picker = open(&workspace, cx);
    assert_eq!(
        labels(&picker, cx),
        vec!["file:second.txt", "file:first.txt"],
        "empty query lists recent files, most recent first"
    );

    // Double Shift again closes it.
    cx.dispatch_action(Toggle);
    cx.run_until_parked();
    assert!(active_picker(&workspace, cx).is_none());
}

#[gpui::test]
async fn test_files_tabs_and_prefixes(cx: &mut TestAppContext) {
    let (workspace, cx) = setup(cx).await;
    let picker = open(&workspace, cx);

    type_query(&picker, "main", cx);
    assert!(
        labels(&picker, cx).contains(&"file:src/main.rs".to_string()),
        "{:?}",
        labels(&picker, cx)
    );

    cx.dispatch_action(NextTab);
    cx.run_until_parked();
    assert_eq!(current_tab(&picker, cx), SearchTab::Classes);
    cx.dispatch_action(PreviousTab);
    cx.dispatch_action(PreviousTab);
    cx.run_until_parked();
    assert_eq!(current_tab(&picker, cx), SearchTab::Text);

    // In the Text tab, `#` is searched literally rather than switching tabs.
    type_query(&picker, "#x", cx);
    assert_eq!(current_tab(&picker, cx), SearchTab::Text);

    // `/f ` switches to Files and strips the prefix.
    cx.dispatch_action(NextTab);
    cx.run_until_parked();
    assert_eq!(current_tab(&picker, cx), SearchTab::All);
    type_query(&picker, "/f first", cx);
    assert_eq!(current_tab(&picker, cx), SearchTab::Files);
    assert_eq!(picker.read_with(cx, |picker, cx| picker.query(cx)), "first");
    assert_eq!(labels(&picker, cx), vec!["file:first.txt"]);
}

#[gpui::test]
async fn test_text_search(cx: &mut TestAppContext) {
    let (workspace, cx) = setup(cx).await;
    let picker = open(&workspace, cx);

    type_query(&picker, "/t ", cx);
    assert_eq!(current_tab(&picker, cx), SearchTab::Text);
    type_query(&picker, "needle", cx);
    cx.executor().advance_clock(TEXT_SEARCH_DEBOUNCE * 2);
    cx.run_until_parked();
    assert_eq!(labels(&picker, cx), vec!["text:first.txt:alpha needle"]);
}

#[gpui::test]
async fn test_restores_last_tab_and_query(cx: &mut TestAppContext) {
    let (workspace, cx) = setup(cx).await;
    let picker = open(&workspace, cx);
    type_query(&picker, "/f second", cx);
    cx.dispatch_action(Toggle);
    cx.run_until_parked();

    let picker = open(&workspace, cx);
    assert_eq!(current_tab(&picker, cx), SearchTab::Files);
    assert_eq!(
        picker.read_with(cx, |picker, cx| picker.query(cx)),
        "second"
    );
    assert_eq!(labels(&picker, cx), vec!["file:second.txt"]);
}

#[gpui::test]
async fn test_line_suffix_opens_file_at_line(cx: &mut TestAppContext) {
    let (workspace, cx) = setup(cx).await;
    let picker = open(&workspace, cx);
    type_query(&picker, "main.rs:1:4", cx);
    assert_eq!(
        picker.read_with(cx, |picker, _| picker.delegate.line_position),
        Some(LinePosition {
            row: 1,
            column: Some(4)
        })
    );
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    let editor = workspace
        .update(cx, |workspace, cx| workspace.active_item_as::<Editor>(cx))
        .expect("an editor is open");
    let cursor = editor.update(cx, |editor, cx| {
        editor
            .selections
            .newest::<Point>(&editor.display_snapshot(cx))
            .head()
    });
    assert_eq!(cursor, Point::new(0, 3));
}
