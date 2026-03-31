use super::*;
use acp_thread::StubAgentConnection;
use agent::ThreadStore;
use agent_ui::{
    test_support::{active_session_id, open_thread_with_connection, send_message},
    thread_metadata_store::ThreadMetadata,
};
use chrono::DateTime;
use feature_flags::FeatureFlagAppExt as _;
use fs::FakeFs;
use gpui::TestAppContext;
use pretty_assertions::assert_eq;
use project::AgentId;
use settings::SettingsStore;
use std::{path::PathBuf, sync::Arc};
use util::path_list::PathList;

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        cx.update_flags(false, vec!["agent-v2".into()]);
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });
}

#[track_caller]
fn assert_active_thread(sidebar: &Sidebar, session_id: &acp::SessionId, msg: &str) {
    assert!(
        sidebar
            .active_entry
            .as_ref()
            .is_some_and(|e| e.is_active_thread(session_id)),
        "{msg}: expected active_entry to be Thread({session_id:?}), got {:?}",
        sidebar.active_entry,
    );
}

#[track_caller]
fn assert_active_draft(sidebar: &Sidebar, workspace: &Entity<Workspace>, msg: &str) {
    assert!(
        matches!(&sidebar.active_entry, Some(ActiveEntry::Draft(ws)) if ws == workspace),
        "{msg}: expected active_entry to be Draft for workspace {:?}, got {:?}",
        workspace.entity_id(),
        sidebar.active_entry,
    );
}

fn has_thread_entry(sidebar: &Sidebar, session_id: &acp::SessionId) -> bool {
    sidebar
        .contents
        .entries
        .iter()
        .any(|entry| matches!(entry, ListEntry::Thread(t) if &t.metadata.session_id == session_id))
}

async fn init_test_project(
    worktree_path: &str,
    cx: &mut TestAppContext,
) -> Entity<project::Project> {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(worktree_path, serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
    project::Project::test(fs, [worktree_path.as_ref()], cx).await
}

fn setup_sidebar(
    multi_workspace: &Entity<MultiWorkspace>,
    cx: &mut gpui::VisualTestContext,
) -> Entity<Sidebar> {
    let multi_workspace = multi_workspace.clone();
    let sidebar =
        cx.update(|window, cx| cx.new(|cx| Sidebar::new(multi_workspace.clone(), window, cx)));
    multi_workspace.update(cx, |mw, cx| {
        mw.register_sidebar(sidebar.clone(), cx);
    });
    cx.run_until_parked();
    sidebar
}

async fn save_n_test_threads(count: u32, path_list: &PathList, cx: &mut gpui::VisualTestContext) {
    for i in 0..count {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(format!("thread-{}", i))),
            format!("Thread {}", i + 1).into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, i).unwrap(),
            None,
            path_list.clone(),
            cx,
        )
    }
    cx.run_until_parked();
}

async fn save_test_thread_metadata(
    session_id: &acp::SessionId,
    path_list: PathList,
    cx: &mut TestAppContext,
) {
    save_thread_metadata(
        session_id.clone(),
        "Test".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    )
}

async fn save_named_thread_metadata(
    session_id: &str,
    title: &str,
    path_list: &PathList,
    cx: &mut gpui::VisualTestContext,
) {
    save_thread_metadata(
        acp::SessionId::new(Arc::from(session_id)),
        SharedString::from(title.to_string()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        path_list.clone(),
        cx,
    );
    cx.run_until_parked();
}

fn save_thread_metadata(
    session_id: acp::SessionId,
    title: SharedString,
    updated_at: DateTime<Utc>,
    created_at: Option<DateTime<Utc>>,
    path_list: PathList,
    cx: &mut TestAppContext,
) {
    let metadata = ThreadMetadata {
        session_id,
        agent_id: agent::ZED_AGENT_ID.clone(),
        title,
        updated_at,
        created_at,
        folder_paths: path_list,
        archived: false,
    };
    cx.update(|cx| {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.save_manually(metadata, cx))
    });
    cx.run_until_parked();
}

fn open_and_focus_sidebar(sidebar: &Entity<Sidebar>, cx: &mut gpui::VisualTestContext) {
    let multi_workspace = sidebar.read_with(cx, |s, _| s.multi_workspace.upgrade());
    if let Some(multi_workspace) = multi_workspace {
        multi_workspace.update_in(cx, |mw, window, cx| {
            if !mw.sidebar_open() {
                mw.toggle_sidebar(window, cx);
            }
        });
    }
    cx.run_until_parked();
    sidebar.update_in(cx, |_, window, cx| {
        cx.focus_self(window);
    });
    cx.run_until_parked();
}

fn visible_entries_as_strings(
    sidebar: &Entity<Sidebar>,
    cx: &mut gpui::VisualTestContext,
) -> Vec<String> {
    sidebar.read_with(cx, |sidebar, _cx| {
        sidebar
            .contents
            .entries
            .iter()
            .enumerate()
            .map(|(ix, entry)| {
                let selected = if sidebar.selection == Some(ix) {
                    "  <== selected"
                } else {
                    ""
                };
                match entry {
                    ListEntry::ProjectHeader {
                        label,
                        path_list,
                        highlight_positions: _,
                        ..
                    } => {
                        let icon = if sidebar.collapsed_groups.contains(path_list) {
                            ">"
                        } else {
                            "v"
                        };
                        format!("{} [{}]{}", icon, label, selected)
                    }
                    ListEntry::Thread(thread) => {
                        let title = thread.metadata.title.as_ref();
                        let active = if thread.is_live { " *" } else { "" };
                        let status_str = match thread.status {
                            AgentThreadStatus::Running => " (running)",
                            AgentThreadStatus::Error => " (error)",
                            AgentThreadStatus::WaitingForConfirmation => " (waiting)",
                            _ => "",
                        };
                        let notified = if sidebar
                            .contents
                            .is_thread_notified(&thread.metadata.session_id)
                        {
                            " (!)"
                        } else {
                            ""
                        };
                        let worktree = if thread.worktrees.is_empty() {
                            String::new()
                        } else {
                            let mut seen = Vec::new();
                            let mut chips = Vec::new();
                            for wt in &thread.worktrees {
                                if !seen.contains(&wt.name) {
                                    seen.push(wt.name.clone());
                                    chips.push(format!("{{{}}}", wt.name));
                                }
                            }
                            format!(" {}", chips.join(", "))
                        };
                        format!(
                            "  {}{}{}{}{}{}",
                            title, worktree, active, status_str, notified, selected
                        )
                    }
                    ListEntry::ViewMore {
                        is_fully_expanded, ..
                    } => {
                        if *is_fully_expanded {
                            format!("  - Collapse{}", selected)
                        } else {
                            format!("  + View More{}", selected)
                        }
                    }
                    ListEntry::NewThread { worktrees, .. } => {
                        let worktree = if worktrees.is_empty() {
                            String::new()
                        } else {
                            let mut seen = Vec::new();
                            let mut chips = Vec::new();
                            for wt in worktrees {
                                if !seen.contains(&wt.name) {
                                    seen.push(wt.name.clone());
                                    chips.push(format!("{{{}}}", wt.name));
                                }
                            }
                            format!(" {}", chips.join(", "))
                        };
                        format!("  [+ New Thread{}]{}", worktree, selected)
                    }
                }
            })
            .collect()
    })
}

#[gpui::test]
async fn test_serialization_round_trip(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(3, &path_list, cx).await;

    // Set a custom width, collapse the group, and expand "View More".
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.set_width(Some(px(420.0)), cx);
        sidebar.toggle_collapse(&path_list, window, cx);
        sidebar.expanded_groups.insert(path_list.clone(), 2);
    });
    cx.run_until_parked();

    // Capture the serialized state from the first sidebar.
    let serialized = sidebar.read_with(cx, |sidebar, cx| sidebar.serialized_state(cx));
    let serialized = serialized.expect("serialized_state should return Some");

    // Create a fresh sidebar and restore into it.
    let sidebar2 =
        cx.update(|window, cx| cx.new(|cx| Sidebar::new(multi_workspace.clone(), window, cx)));
    cx.run_until_parked();

    sidebar2.update_in(cx, |sidebar, window, cx| {
        sidebar.restore_serialized_state(&serialized, window, cx);
    });
    cx.run_until_parked();

    // Assert all serialized fields match.
    let (width1, collapsed1, expanded1) = sidebar.read_with(cx, |s, _| {
        (
            s.width,
            s.collapsed_groups.clone(),
            s.expanded_groups.clone(),
        )
    });
    let (width2, collapsed2, expanded2) = sidebar2.read_with(cx, |s, _| {
        (
            s.width,
            s.collapsed_groups.clone(),
            s.expanded_groups.clone(),
        )
    });

    assert_eq!(width1, width2);
    assert_eq!(collapsed1, collapsed2);
    assert_eq!(expanded1, expanded2);
    assert_eq!(width1, px(420.0));
    assert!(collapsed1.contains(&path_list));
    assert_eq!(expanded1.get(&path_list), Some(&2));
}

#[gpui::test]
async fn test_restore_serialized_archive_view_does_not_panic(cx: &mut TestAppContext) {
    // A regression test to ensure that restoring a serialized archive view does not panic.
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, _panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);
    cx.update(|_window, cx| {
        AgentRegistryStore::init_test_global(cx, vec![]);
    });

    let serialized = serde_json::to_string(&SerializedSidebar {
        width: Some(400.0),
        collapsed_groups: Vec::new(),
        expanded_groups: Vec::new(),
        active_view: SerializedSidebarView::Archive,
    })
    .expect("serialization should succeed");

    multi_workspace.update_in(cx, |multi_workspace, window, cx| {
        if let Some(sidebar) = multi_workspace.sidebar() {
            sidebar.restore_serialized_state(&serialized, window, cx);
        }
    });
    cx.run_until_parked();

    // After the deferred `show_archive` runs, the view should be Archive.
    sidebar.read_with(cx, |sidebar, _cx| {
        assert!(
            matches!(sidebar.view, SidebarView::Archive(_)),
            "expected sidebar view to be Archive after restore, got ThreadList"
        );
    });
}

#[test]
fn test_clean_mention_links() {
    // Simple mention link
    assert_eq!(
        Sidebar::clean_mention_links("check [@Button.tsx](file:///path/to/Button.tsx)"),
        "check @Button.tsx"
    );

    // Multiple mention links
    assert_eq!(
        Sidebar::clean_mention_links(
            "look at [@foo.rs](file:///foo.rs) and [@bar.rs](file:///bar.rs)"
        ),
        "look at @foo.rs and @bar.rs"
    );

    // No mention links — passthrough
    assert_eq!(
        Sidebar::clean_mention_links("plain text with no mentions"),
        "plain text with no mentions"
    );

    // Incomplete link syntax — preserved as-is
    assert_eq!(
        Sidebar::clean_mention_links("broken [@mention without closing"),
        "broken [@mention without closing"
    );

    // Regular markdown link (no @) — not touched
    assert_eq!(
        Sidebar::clean_mention_links("see [docs](https://example.com)"),
        "see [docs](https://example.com)"
    );

    // Empty input
    assert_eq!(Sidebar::clean_mention_links(""), "");
}

#[gpui::test]
async fn test_entities_released_on_window_close(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let weak_workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().downgrade());
    let weak_sidebar = sidebar.downgrade();
    let weak_multi_workspace = multi_workspace.downgrade();

    drop(sidebar);
    drop(multi_workspace);
    cx.update(|window, _cx| window.remove_window());
    cx.run_until_parked();

    weak_multi_workspace.assert_released();
    weak_sidebar.assert_released();
    weak_workspace.assert_released();
}

#[gpui::test]
async fn test_single_workspace_no_threads(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  [+ New Thread]"]
    );
}

#[gpui::test]
async fn test_single_workspace_with_saved_threads(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-1")),
        "Fix crash in project panel".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 3, 0, 0, 0).unwrap(),
        None,
        path_list.clone(),
        cx,
    );

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-2")),
        "Add inline diff view".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Fix crash in project panel",
            "  Add inline diff view",
        ]
    );
}

#[gpui::test]
async fn test_workspace_lifecycle(cx: &mut TestAppContext) {
    let project = init_test_project("/project-a", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Single workspace with a thread
    let path_list = PathList::new(&[std::path::PathBuf::from("/project-a")]);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-a1")),
        "Thread A1".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Thread A1"]
    );

    // Add a second workspace
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.create_test_workspace(window, cx).detach();
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Thread A1",]
    );

    // Remove the second workspace
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[1].clone();
        mw.remove(&workspace, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Thread A1"]
    );
}

#[gpui::test]
async fn test_view_more_pagination(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(12, &path_list, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Thread 12",
            "  Thread 11",
            "  Thread 10",
            "  Thread 9",
            "  Thread 8",
            "  + View More",
        ]
    );
}

#[gpui::test]
async fn test_view_more_batched_expansion(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    // Create 17 threads: initially shows 5, then 10, then 15, then all 17 with Collapse
    save_n_test_threads(17, &path_list, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Initially shows 5 threads + View More
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 7); // header + 5 threads + View More
    assert!(entries.iter().any(|e| e.contains("View More")));

    // Focus and navigate to View More, then confirm to expand by one batch
    open_and_focus_sidebar(&sidebar, cx);
    for _ in 0..7 {
        cx.dispatch_action(SelectNext);
    }
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    // Now shows 10 threads + View More
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 12); // header + 10 threads + View More
    assert!(entries.iter().any(|e| e.contains("View More")));

    // Expand again by one batch
    sidebar.update_in(cx, |s, _window, cx| {
        let current = s.expanded_groups.get(&path_list).copied().unwrap_or(0);
        s.expanded_groups.insert(path_list.clone(), current + 1);
        s.update_entries(cx);
    });
    cx.run_until_parked();

    // Now shows 15 threads + View More
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 17); // header + 15 threads + View More
    assert!(entries.iter().any(|e| e.contains("View More")));

    // Expand one more time - should show all 17 threads with Collapse button
    sidebar.update_in(cx, |s, _window, cx| {
        let current = s.expanded_groups.get(&path_list).copied().unwrap_or(0);
        s.expanded_groups.insert(path_list.clone(), current + 1);
        s.update_entries(cx);
    });
    cx.run_until_parked();

    // All 17 threads shown with Collapse button
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 19); // header + 17 threads + Collapse
    assert!(!entries.iter().any(|e| e.contains("View More")));
    assert!(entries.iter().any(|e| e.contains("Collapse")));

    // Click collapse - should go back to showing 5 threads
    sidebar.update_in(cx, |s, _window, cx| {
        s.expanded_groups.remove(&path_list);
        s.update_entries(cx);
    });
    cx.run_until_parked();

    // Back to initial state: 5 threads + View More
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 7); // header + 5 threads + View More
    assert!(entries.iter().any(|e| e.contains("View More")));
}

#[gpui::test]
async fn test_collapse_and_expand_group(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(1, &path_list, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Thread 1"]
    );

    // Collapse
    sidebar.update_in(cx, |s, window, cx| {
        s.toggle_collapse(&path_list, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["> [my-project]"]
    );

    // Expand
    sidebar.update_in(cx, |s, window, cx| {
        s.toggle_collapse(&path_list, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Thread 1"]
    );
}

#[gpui::test]
async fn test_visible_entries_as_strings(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    let expanded_path = PathList::new(&[std::path::PathBuf::from("/expanded")]);
    let collapsed_path = PathList::new(&[std::path::PathBuf::from("/collapsed")]);

    sidebar.update_in(cx, |s, _window, _cx| {
        s.collapsed_groups.insert(collapsed_path.clone());
        s.contents
            .notified_threads
            .insert(acp::SessionId::new(Arc::from("t-5")));
        s.contents.entries = vec![
            // Expanded project header
            ListEntry::ProjectHeader {
                path_list: expanded_path.clone(),
                label: "expanded-project".into(),
                workspace: workspace.clone(),
                highlight_positions: Vec::new(),
                has_running_threads: false,
                waiting_thread_count: 0,
                is_active: true,
            },
            ListEntry::Thread(ThreadEntry {
                metadata: ThreadMetadata {
                    session_id: acp::SessionId::new(Arc::from("t-1")),
                    agent_id: AgentId::new("zed-agent"),
                    folder_paths: PathList::default(),
                    title: "Completed thread".into(),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                },
                icon: IconName::ZedAgent,
                icon_from_external_svg: None,
                status: AgentThreadStatus::Completed,
                workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                is_live: false,
                is_background: false,
                is_title_generating: false,
                highlight_positions: Vec::new(),
                worktrees: Vec::new(),
                diff_stats: DiffStats::default(),
            }),
            // Active thread with Running status
            ListEntry::Thread(ThreadEntry {
                metadata: ThreadMetadata {
                    session_id: acp::SessionId::new(Arc::from("t-2")),
                    agent_id: AgentId::new("zed-agent"),
                    folder_paths: PathList::default(),
                    title: "Running thread".into(),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                },
                icon: IconName::ZedAgent,
                icon_from_external_svg: None,
                status: AgentThreadStatus::Running,
                workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                is_live: true,
                is_background: false,
                is_title_generating: false,
                highlight_positions: Vec::new(),
                worktrees: Vec::new(),
                diff_stats: DiffStats::default(),
            }),
            // Active thread with Error status
            ListEntry::Thread(ThreadEntry {
                metadata: ThreadMetadata {
                    session_id: acp::SessionId::new(Arc::from("t-3")),
                    agent_id: AgentId::new("zed-agent"),
                    folder_paths: PathList::default(),
                    title: "Error thread".into(),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                },
                icon: IconName::ZedAgent,
                icon_from_external_svg: None,
                status: AgentThreadStatus::Error,
                workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                is_live: true,
                is_background: false,
                is_title_generating: false,
                highlight_positions: Vec::new(),
                worktrees: Vec::new(),
                diff_stats: DiffStats::default(),
            }),
            // Thread with WaitingForConfirmation status, not active
            ListEntry::Thread(ThreadEntry {
                metadata: ThreadMetadata {
                    session_id: acp::SessionId::new(Arc::from("t-4")),
                    agent_id: AgentId::new("zed-agent"),
                    folder_paths: PathList::default(),
                    title: "Waiting thread".into(),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                },
                icon: IconName::ZedAgent,
                icon_from_external_svg: None,
                status: AgentThreadStatus::WaitingForConfirmation,
                workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                is_live: false,
                is_background: false,
                is_title_generating: false,
                highlight_positions: Vec::new(),
                worktrees: Vec::new(),
                diff_stats: DiffStats::default(),
            }),
            // Background thread that completed (should show notification)
            ListEntry::Thread(ThreadEntry {
                metadata: ThreadMetadata {
                    session_id: acp::SessionId::new(Arc::from("t-5")),
                    agent_id: AgentId::new("zed-agent"),
                    folder_paths: PathList::default(),
                    title: "Notified thread".into(),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                },
                icon: IconName::ZedAgent,
                icon_from_external_svg: None,
                status: AgentThreadStatus::Completed,
                workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                is_live: true,
                is_background: true,
                is_title_generating: false,
                highlight_positions: Vec::new(),
                worktrees: Vec::new(),
                diff_stats: DiffStats::default(),
            }),
            // View More entry
            ListEntry::ViewMore {
                path_list: expanded_path.clone(),
                is_fully_expanded: false,
            },
            // Collapsed project header
            ListEntry::ProjectHeader {
                path_list: collapsed_path.clone(),
                label: "collapsed-project".into(),
                workspace: workspace.clone(),
                highlight_positions: Vec::new(),
                has_running_threads: false,
                waiting_thread_count: 0,
                is_active: false,
            },
        ];

        // Select the Running thread (index 2)
        s.selection = Some(2);
    });

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [expanded-project]",
            "  Completed thread",
            "  Running thread * (running)  <== selected",
            "  Error thread * (error)",
            "  Waiting thread (waiting)",
            "  Notified thread * (!)",
            "  + View More",
            "> [collapsed-project]",
        ]
    );

    // Move selection to the collapsed header
    sidebar.update_in(cx, |s, _window, _cx| {
        s.selection = Some(7);
    });

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx).last().cloned(),
        Some("> [collapsed-project]  <== selected".to_string()),
    );

    // Clear selection
    sidebar.update_in(cx, |s, _window, _cx| {
        s.selection = None;
    });

    // No entry should have the selected marker
    let entries = visible_entries_as_strings(&sidebar, cx);
    for entry in &entries {
        assert!(
            !entry.contains("<== selected"),
            "unexpected selection marker in: {}",
            entry
        );
    }
}

#[gpui::test]
async fn test_keyboard_select_next_and_previous(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(3, &path_list, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Entries: [header, thread3, thread2, thread1]
    // Focusing the sidebar does not set a selection; select_next/select_previous
    // handle None gracefully by starting from the first or last entry.
    open_and_focus_sidebar(&sidebar, cx);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

    // First SelectNext from None starts at index 0
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

    // Move down through remaining entries
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));

    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(3));

    // At the end, wraps back to first entry
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

    // Navigate back to the end
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(3));

    // Move back up
    cx.dispatch_action(SelectPrevious);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));

    cx.dispatch_action(SelectPrevious);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

    cx.dispatch_action(SelectPrevious);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

    // At the top, selection clears (focus returns to editor)
    cx.dispatch_action(SelectPrevious);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);
}

#[gpui::test]
async fn test_keyboard_select_first_and_last(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(3, &path_list, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    open_and_focus_sidebar(&sidebar, cx);

    // SelectLast jumps to the end
    cx.dispatch_action(SelectLast);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(3));

    // SelectFirst jumps to the beginning
    cx.dispatch_action(SelectFirst);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
}

#[gpui::test]
async fn test_keyboard_focus_in_does_not_set_selection(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Initially no selection
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

    // Open the sidebar so it's rendered, then focus it to trigger focus_in.
    // focus_in no longer sets a default selection.
    open_and_focus_sidebar(&sidebar, cx);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

    // Manually set a selection, blur, then refocus — selection should be preserved
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(0);
    });

    cx.update(|window, _cx| {
        window.blur();
    });
    cx.run_until_parked();

    sidebar.update_in(cx, |_, window, cx| {
        cx.focus_self(window);
    });
    cx.run_until_parked();
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
}

#[gpui::test]
async fn test_keyboard_confirm_on_project_header_toggles_collapse(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(1, &path_list, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Thread 1"]
    );

    // Focus the sidebar and select the header (index 0)
    open_and_focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(0);
    });

    // Confirm on project header collapses the group
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["> [my-project]  <== selected"]
    );

    // Confirm again expands the group
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]  <== selected", "  Thread 1",]
    );
}

#[gpui::test]
async fn test_keyboard_confirm_on_view_more_expands(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(8, &path_list, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Should show header + 5 threads + "View More"
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 7);
    assert!(entries.iter().any(|e| e.contains("View More")));

    // Focus sidebar (selection starts at None), then navigate down to the "View More" entry (index 6)
    open_and_focus_sidebar(&sidebar, cx);
    for _ in 0..7 {
        cx.dispatch_action(SelectNext);
    }
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(6));

    // Confirm on "View More" to expand
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    // All 8 threads should now be visible with a "Collapse" button
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 10); // header + 8 threads + Collapse button
    assert!(!entries.iter().any(|e| e.contains("View More")));
    assert!(entries.iter().any(|e| e.contains("Collapse")));
}

#[gpui::test]
async fn test_keyboard_expand_and_collapse_selected_entry(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(1, &path_list, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Thread 1"]
    );

    // Focus sidebar and manually select the header (index 0). Press left to collapse.
    open_and_focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(0);
    });

    cx.dispatch_action(SelectParent);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["> [my-project]  <== selected"]
    );

    // Press right to expand
    cx.dispatch_action(SelectChild);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]  <== selected", "  Thread 1",]
    );

    // Press right again on already-expanded header moves selection down
    cx.dispatch_action(SelectChild);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));
}

#[gpui::test]
async fn test_keyboard_collapse_from_child_selects_parent(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(1, &path_list, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Focus sidebar (selection starts at None), then navigate down to the thread (child)
    open_and_focus_sidebar(&sidebar, cx);
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Thread 1  <== selected",]
    );

    // Pressing left on a child collapses the parent group and selects it
    cx.dispatch_action(SelectParent);
    cx.run_until_parked();

    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["> [my-project]  <== selected"]
    );
}

#[gpui::test]
async fn test_keyboard_navigation_on_empty_list(cx: &mut TestAppContext) {
    let project = init_test_project("/empty-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // An empty project has the header and a new thread button.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [empty-project]", "  [+ New Thread]"]
    );

    // Focus sidebar — focus_in does not set a selection
    open_and_focus_sidebar(&sidebar, cx);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

    // First SelectNext from None starts at index 0 (header)
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

    // SelectNext moves to the new thread button
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

    // At the end, wraps back to first entry
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

    // SelectPrevious from first entry clears selection (returns to editor)
    cx.dispatch_action(SelectPrevious);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);
}

#[gpui::test]
async fn test_selection_clamps_after_entry_removal(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
    save_n_test_threads(1, &path_list, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Focus sidebar (selection starts at None), navigate down to the thread (index 1)
    open_and_focus_sidebar(&sidebar, cx);
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

    // Collapse the group, which removes the thread from the list
    cx.dispatch_action(SelectParent);
    cx.run_until_parked();

    // Selection should be clamped to the last valid index (0 = header)
    let selection = sidebar.read_with(cx, |s, _| s.selection);
    let entry_count = sidebar.read_with(cx, |s, _| s.contents.entries.len());
    assert!(
        selection.unwrap_or(0) < entry_count,
        "selection {} should be within bounds (entries: {})",
        selection.unwrap_or(0),
        entry_count,
    );
}

async fn init_test_project_with_agent_panel(
    worktree_path: &str,
    cx: &mut TestAppContext,
) -> Entity<project::Project> {
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.update_flags(false, vec!["agent-v2".into()]);
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(worktree_path, serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
    project::Project::test(fs, [worktree_path.as_ref()], cx).await
}

fn add_agent_panel(
    workspace: &Entity<Workspace>,
    cx: &mut gpui::VisualTestContext,
) -> Entity<AgentPanel> {
    workspace.update_in(cx, |workspace, window, cx| {
        let panel = cx.new(|cx| AgentPanel::test_new(workspace, window, cx));
        workspace.add_panel(panel.clone(), window, cx);
        panel
    })
}

fn setup_sidebar_with_agent_panel(
    multi_workspace: &Entity<MultiWorkspace>,
    cx: &mut gpui::VisualTestContext,
) -> (Entity<Sidebar>, Entity<AgentPanel>) {
    let sidebar = setup_sidebar(multi_workspace, cx);
    let workspace = multi_workspace.read_with(cx, |mw, _cx| mw.workspace().clone());
    let panel = add_agent_panel(&workspace, cx);
    (sidebar, panel)
}

#[gpui::test]
async fn test_parallel_threads_shown_with_live_status(cx: &mut TestAppContext) {
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    // Open thread A and keep it generating.
    let connection = StubAgentConnection::new();
    open_thread_with_connection(&panel, connection.clone(), cx);
    send_message(&panel, cx);

    let session_id_a = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id_a, path_list.clone(), cx).await;

    cx.update(|_, cx| {
        connection.send_update(
            session_id_a.clone(),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("working...".into())),
            cx,
        );
    });
    cx.run_until_parked();

    // Open thread B (idle, default response) — thread A goes to background.
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);

    let session_id_b = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id_b, path_list.clone(), cx).await;

    cx.run_until_parked();

    let mut entries = visible_entries_as_strings(&sidebar, cx);
    entries[1..].sort();
    assert_eq!(
        entries,
        vec!["v [my-project]", "  Hello *", "  Hello * (running)",]
    );
}

#[gpui::test]
async fn test_background_thread_completion_triggers_notification(cx: &mut TestAppContext) {
    let project_a = init_test_project_with_agent_panel("/project-a", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let path_list_a = PathList::new(&[std::path::PathBuf::from("/project-a")]);

    // Open thread on workspace A and keep it generating.
    let connection_a = StubAgentConnection::new();
    open_thread_with_connection(&panel_a, connection_a.clone(), cx);
    send_message(&panel_a, cx);

    let session_id_a = active_session_id(&panel_a, cx);
    save_test_thread_metadata(&session_id_a, path_list_a.clone(), cx).await;

    cx.update(|_, cx| {
        connection_a.send_update(
            session_id_a.clone(),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("chunk".into())),
            cx,
        );
    });
    cx.run_until_parked();

    // Add a second workspace and activate it (making workspace A the background).
    let fs = cx.update(|_, cx| <dyn fs::Fs>::global(cx));
    let project_b = project::Project::test(fs, [], cx).await;
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b, window, cx);
    });
    cx.run_until_parked();

    // Thread A is still running; no notification yet.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Hello * (running)",]
    );

    // Complete thread A's turn (transition Running → Completed).
    connection_a.end_turn(session_id_a.clone(), acp::StopReason::EndTurn);
    cx.run_until_parked();

    // The completed background thread shows a notification indicator.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Hello * (!)",]
    );
}

fn type_in_search(sidebar: &Entity<Sidebar>, query: &str, cx: &mut gpui::VisualTestContext) {
    sidebar.update_in(cx, |sidebar, window, cx| {
        window.focus(&sidebar.filter_editor.focus_handle(cx), cx);
        sidebar.filter_editor.update(cx, |editor, cx| {
            editor.set_text(query, window, cx);
        });
    });
    cx.run_until_parked();
}

#[gpui::test]
async fn test_search_narrows_visible_threads_to_matches(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    for (id, title, hour) in [
        ("t-1", "Fix crash in project panel", 3),
        ("t-2", "Add inline diff view", 2),
        ("t-3", "Refactor settings module", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            title.into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            path_list.clone(),
            cx,
        );
    }
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Fix crash in project panel",
            "  Add inline diff view",
            "  Refactor settings module",
        ]
    );

    // User types "diff" in the search box — only the matching thread remains,
    // with its workspace header preserved for context.
    type_in_search(&sidebar, "diff", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Add inline diff view  <== selected",]
    );

    // User changes query to something with no matches — list is empty.
    type_in_search(&sidebar, "nonexistent", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        Vec::<String>::new()
    );
}

#[gpui::test]
async fn test_search_matches_regardless_of_case(cx: &mut TestAppContext) {
    // Scenario: A user remembers a thread title but not the exact casing.
    // Search should match case-insensitively so they can still find it.
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-1")),
        "Fix Crash In Project Panel".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    );
    cx.run_until_parked();

    // Lowercase query matches mixed-case title.
    type_in_search(&sidebar, "fix crash", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Fix Crash In Project Panel  <== selected",
        ]
    );

    // Uppercase query also matches the same title.
    type_in_search(&sidebar, "FIX CRASH", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Fix Crash In Project Panel  <== selected",
        ]
    );
}

#[gpui::test]
async fn test_escape_clears_search_and_restores_full_list(cx: &mut TestAppContext) {
    // Scenario: A user searches, finds what they need, then presses Escape
    // to dismiss the filter and see the full list again.
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    for (id, title, hour) in [("t-1", "Alpha thread", 2), ("t-2", "Beta thread", 1)] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            title.into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            path_list.clone(),
            cx,
        )
    }
    cx.run_until_parked();

    // Confirm the full list is showing.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Alpha thread", "  Beta thread",]
    );

    // User types a search query to filter down.
    open_and_focus_sidebar(&sidebar, cx);
    type_in_search(&sidebar, "alpha", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Alpha thread  <== selected",]
    );

    // User presses Escape — filter clears, full list is restored.
    // The selection index (1) now points at the first thread entry.
    cx.dispatch_action(Cancel);
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Alpha thread  <== selected",
            "  Beta thread",
        ]
    );
}

#[gpui::test]
async fn test_search_only_shows_workspace_headers_with_matches(cx: &mut TestAppContext) {
    let project_a = init_test_project("/project-a", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list_a = PathList::new(&[std::path::PathBuf::from("/project-a")]);

    for (id, title, hour) in [
        ("a1", "Fix bug in sidebar", 2),
        ("a2", "Add tests for editor", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            title.into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            path_list_a.clone(),
            cx,
        )
    }

    // Add a second workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.create_test_workspace(window, cx).detach();
    });
    cx.run_until_parked();

    let path_list_b = PathList::new::<std::path::PathBuf>(&[]);

    for (id, title, hour) in [
        ("b1", "Refactor sidebar layout", 3),
        ("b2", "Fix typo in README", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            title.into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            path_list_b.clone(),
            cx,
        )
    }
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project-a]",
            "  Fix bug in sidebar",
            "  Add tests for editor",
        ]
    );

    // "sidebar" matches a thread in each workspace — both headers stay visible.
    type_in_search(&sidebar, "sidebar", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Fix bug in sidebar  <== selected",]
    );

    // "typo" only matches in the second workspace — the first header disappears.
    type_in_search(&sidebar, "typo", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        Vec::<String>::new()
    );

    // "project-a" matches the first workspace name — the header appears
    // with all child threads included.
    type_in_search(&sidebar, "project-a", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project-a]",
            "  Fix bug in sidebar  <== selected",
            "  Add tests for editor",
        ]
    );
}

#[gpui::test]
async fn test_search_matches_workspace_name(cx: &mut TestAppContext) {
    let project_a = init_test_project("/alpha-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list_a = PathList::new(&[std::path::PathBuf::from("/alpha-project")]);

    for (id, title, hour) in [
        ("a1", "Fix bug in sidebar", 2),
        ("a2", "Add tests for editor", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            title.into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            path_list_a.clone(),
            cx,
        )
    }

    // Add a second workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.create_test_workspace(window, cx).detach();
    });
    cx.run_until_parked();

    let path_list_b = PathList::new::<std::path::PathBuf>(&[]);

    for (id, title, hour) in [
        ("b1", "Refactor sidebar layout", 3),
        ("b2", "Fix typo in README", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            title.into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            path_list_b.clone(),
            cx,
        )
    }
    cx.run_until_parked();

    // "alpha" matches the workspace name "alpha-project" but no thread titles.
    // The workspace header should appear with all child threads included.
    type_in_search(&sidebar, "alpha", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [alpha-project]",
            "  Fix bug in sidebar  <== selected",
            "  Add tests for editor",
        ]
    );

    // "sidebar" matches thread titles in both workspaces but not workspace names.
    // Both headers appear with their matching threads.
    type_in_search(&sidebar, "sidebar", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [alpha-project]", "  Fix bug in sidebar  <== selected",]
    );

    // "alpha sidebar" matches the workspace name "alpha-project" (fuzzy: a-l-p-h-a-s-i-d-e-b-a-r
    // doesn't match) — but does not match either workspace name or any thread.
    // Actually let's test something simpler: a query that matches both a workspace
    // name AND some threads in that workspace. Matching threads should still appear.
    type_in_search(&sidebar, "fix", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [alpha-project]", "  Fix bug in sidebar  <== selected",]
    );

    // A query that matches a workspace name AND a thread in that same workspace.
    // Both the header (highlighted) and all child threads should appear.
    type_in_search(&sidebar, "alpha", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [alpha-project]",
            "  Fix bug in sidebar  <== selected",
            "  Add tests for editor",
        ]
    );

    // Now search for something that matches only a workspace name when there
    // are also threads with matching titles — the non-matching workspace's
    // threads should still appear if their titles match.
    type_in_search(&sidebar, "alp", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [alpha-project]",
            "  Fix bug in sidebar  <== selected",
            "  Add tests for editor",
        ]
    );
}

#[gpui::test]
async fn test_search_finds_threads_hidden_behind_view_more(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    // Create 8 threads. The oldest one has a unique name and will be
    // behind View More (only 5 shown by default).
    for i in 0..8u32 {
        let title = if i == 0 {
            "Hidden gem thread".to_string()
        } else {
            format!("Thread {}", i + 1)
        };
        save_thread_metadata(
            acp::SessionId::new(Arc::from(format!("thread-{}", i))),
            title.into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, i).unwrap(),
            None,
            path_list.clone(),
            cx,
        )
    }
    cx.run_until_parked();

    // Confirm the thread is not visible and View More is shown.
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries.iter().any(|e| e.contains("View More")),
        "should have View More button"
    );
    assert!(
        !entries.iter().any(|e| e.contains("Hidden gem")),
        "Hidden gem should be behind View More"
    );

    // User searches for the hidden thread — it appears, and View More is gone.
    type_in_search(&sidebar, "hidden gem", cx);
    let filtered = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(
        filtered,
        vec!["v [my-project]", "  Hidden gem thread  <== selected",]
    );
    assert!(
        !filtered.iter().any(|e| e.contains("View More")),
        "View More should not appear when filtering"
    );
}

#[gpui::test]
async fn test_search_finds_threads_inside_collapsed_groups(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-1")),
        "Important thread".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    );
    cx.run_until_parked();

    // User focuses the sidebar and collapses the group using keyboard:
    // manually select the header, then press SelectParent to collapse.
    open_and_focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(0);
    });
    cx.dispatch_action(SelectParent);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["> [my-project]  <== selected"]
    );

    // User types a search — the thread appears even though its group is collapsed.
    type_in_search(&sidebar, "important", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["> [my-project]", "  Important thread  <== selected",]
    );
}

#[gpui::test]
async fn test_search_then_keyboard_navigate_and_confirm(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    for (id, title, hour) in [
        ("t-1", "Fix crash in panel", 3),
        ("t-2", "Fix lint warnings", 2),
        ("t-3", "Add new feature", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            title.into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            path_list.clone(),
            cx,
        )
    }
    cx.run_until_parked();

    open_and_focus_sidebar(&sidebar, cx);

    // User types "fix" — two threads match.
    type_in_search(&sidebar, "fix", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Fix crash in panel  <== selected",
            "  Fix lint warnings",
        ]
    );

    // Selection starts on the first matching thread. User presses
    // SelectNext to move to the second match.
    cx.dispatch_action(SelectNext);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Fix crash in panel",
            "  Fix lint warnings  <== selected",
        ]
    );

    // User can also jump back with SelectPrevious.
    cx.dispatch_action(SelectPrevious);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [my-project]",
            "  Fix crash in panel  <== selected",
            "  Fix lint warnings",
        ]
    );
}

#[gpui::test]
async fn test_confirm_on_historical_thread_activates_workspace(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.create_test_workspace(window, cx).detach();
    });
    cx.run_until_parked();

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("hist-1")),
        "Historical Thread".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    );
    cx.run_until_parked();
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Historical Thread",]
    );

    // Switch to workspace 1 so we can verify the confirm switches back.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[1].clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        1
    );

    // Confirm on the historical (non-live) thread at index 1.
    // Before a previous fix, the workspace field was Option<usize> and
    // historical threads had None, so activate_thread early-returned
    // without switching the workspace.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.selection = Some(1);
        sidebar.confirm(&Confirm, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        0
    );
}

#[gpui::test]
async fn test_click_clears_selection_and_focus_in_restores_it(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("t-1")),
        "Thread A".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        path_list.clone(),
        cx,
    );

    save_thread_metadata(
        acp::SessionId::new(Arc::from("t-2")),
        "Thread B".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    );

    cx.run_until_parked();
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Thread A", "  Thread B",]
    );

    // Keyboard confirm preserves selection.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.selection = Some(1);
        sidebar.confirm(&Confirm, window, cx);
    });
    assert_eq!(
        sidebar.read_with(cx, |sidebar, _| sidebar.selection),
        Some(1)
    );

    // Click handlers clear selection to None so no highlight lingers
    // after a click regardless of focus state. The hover style provides
    // visual feedback during mouse interaction instead.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.selection = None;
        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        sidebar.toggle_collapse(&path_list, window, cx);
    });
    assert_eq!(sidebar.read_with(cx, |sidebar, _| sidebar.selection), None);

    // When the user tabs back into the sidebar, focus_in no longer
    // restores selection — it stays None.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.focus_in(window, cx);
    });
    assert_eq!(sidebar.read_with(cx, |sidebar, _| sidebar.selection), None);
}

#[gpui::test]
async fn test_thread_title_update_propagates_to_sidebar(cx: &mut TestAppContext) {
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Hi there!".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);

    let session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id, path_list.clone(), cx).await;
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Hello *"]
    );

    // Simulate the agent generating a title. The notification chain is:
    // AcpThread::set_title emits TitleUpdated →
    // ConnectionView::handle_thread_event calls cx.notify() →
    // AgentPanel observer fires and emits AgentPanelEvent →
    // Sidebar subscription calls update_entries / rebuild_contents.
    //
    // Before the fix, handle_thread_event did NOT call cx.notify() for
    // TitleUpdated, so the AgentPanel observer never fired and the
    // sidebar kept showing the old title.
    let thread = panel.read_with(cx, |panel, cx| panel.active_agent_thread(cx).unwrap());
    thread.update(cx, |thread, cx| {
        thread
            .set_title("Friendly Greeting with AI".into(), cx)
            .detach();
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Friendly Greeting with AI *"]
    );
}

#[gpui::test]
async fn test_focused_thread_tracks_user_intent(cx: &mut TestAppContext) {
    let project_a = init_test_project_with_agent_panel("/project-a", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let path_list_a = PathList::new(&[std::path::PathBuf::from("/project-a")]);

    // Save a thread so it appears in the list.
    let connection_a = StubAgentConnection::new();
    connection_a.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel_a, connection_a, cx);
    send_message(&panel_a, cx);
    let session_id_a = active_session_id(&panel_a, cx);
    save_test_thread_metadata(&session_id_a, path_list_a.clone(), cx).await;

    // Add a second workspace with its own agent panel.
    let fs = cx.update(|_, cx| <dyn fs::Fs>::global(cx));
    fs.as_fake()
        .insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    let project_b = project::Project::test(fs, ["/project-b".as_ref()], cx).await;
    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx)
    });
    let panel_b = add_agent_panel(&workspace_b, cx);
    cx.run_until_parked();

    let workspace_a = multi_workspace.read_with(cx, |mw, _cx| mw.workspaces()[0].clone());

    // ── 1. Initial state: focused thread derived from active panel ─────
    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_a,
            "The active panel's thread should be focused on startup",
        );
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_thread(
            ThreadMetadata {
                session_id: session_id_a.clone(),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "Test".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: PathList::default(),
                archived: false,
            },
            &workspace_a,
            window,
            cx,
        );
    });
    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_a,
            "After clicking a thread, it should be the focused thread",
        );
        assert!(
            has_thread_entry(sidebar, &session_id_a),
            "The clicked thread should be present in the entries"
        );
    });

    workspace_a.read_with(cx, |workspace, cx| {
        assert!(
            workspace.panel::<AgentPanel>(cx).is_some(),
            "Agent panel should exist"
        );
        let dock = workspace.right_dock().read(cx);
        assert!(
            dock.is_open(),
            "Clicking a thread should open the agent panel dock"
        );
    });

    let connection_b = StubAgentConnection::new();
    connection_b.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Thread B".into()),
    )]);
    open_thread_with_connection(&panel_b, connection_b, cx);
    send_message(&panel_b, cx);
    let session_id_b = active_session_id(&panel_b, cx);
    let path_list_b = PathList::new(&[std::path::PathBuf::from("/project-b")]);
    save_test_thread_metadata(&session_id_b, path_list_b.clone(), cx).await;
    cx.run_until_parked();

    // Workspace A is currently active. Click a thread in workspace B,
    // which also triggers a workspace switch.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_thread(
            ThreadMetadata {
                session_id: session_id_b.clone(),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "Thread B".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: PathList::default(),
                archived: false,
            },
            &workspace_b,
            window,
            cx,
        );
    });
    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_b,
            "Clicking a thread in another workspace should focus that thread",
        );
        assert!(
            has_thread_entry(sidebar, &session_id_b),
            "The cross-workspace thread should be present in the entries"
        );
    });

    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[0].clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_a,
            "Switching workspace should seed focused_thread from the new active panel",
        );
        assert!(
            has_thread_entry(sidebar, &session_id_a),
            "The seeded thread should be present in the entries"
        );
    });

    let connection_b2 = StubAgentConnection::new();
    connection_b2.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new(DEFAULT_THREAD_TITLE.into()),
    )]);
    open_thread_with_connection(&panel_b, connection_b2, cx);
    send_message(&panel_b, cx);
    let session_id_b2 = active_session_id(&panel_b, cx);
    save_test_thread_metadata(&session_id_b2, path_list_b.clone(), cx).await;
    cx.run_until_parked();

    // Panel B is not the active workspace's panel (workspace A is
    // active), so opening a thread there should not change focused_thread.
    // This prevents running threads in background workspaces from causing
    // the selection highlight to jump around.
    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_a,
            "Opening a thread in a non-active panel should not change focused_thread",
        );
    });

    workspace_b.update_in(cx, |workspace, window, cx| {
        workspace.focus_handle(cx).focus(window, cx);
    });
    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_a,
            "Defocusing the sidebar should not change focused_thread",
        );
    });

    // Switching workspaces via the multi_workspace (simulates clicking
    // a workspace header) should clear focused_thread.
    multi_workspace.update_in(cx, |mw, window, cx| {
        if let Some(index) = mw.workspaces().iter().position(|w| w == &workspace_b) {
            let workspace = mw.workspaces()[index].clone();
            mw.activate(workspace, window, cx);
        }
    });
    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_b2,
            "Switching workspace should seed focused_thread from the new active panel",
        );
        assert!(
            has_thread_entry(sidebar, &session_id_b2),
            "The seeded thread should be present in the entries"
        );
    });

    // ── 8. Focusing the agent panel thread keeps focused_thread ────
    // Workspace B still has session_id_b2 loaded in the agent panel.
    // Clicking into the thread (simulated by focusing its view) should
    // keep focused_thread since it was already seeded on workspace switch.
    panel_b.update_in(cx, |panel, window, cx| {
        if let Some(thread_view) = panel.active_conversation_view() {
            thread_view.read(cx).focus_handle(cx).focus(window, cx);
        }
    });
    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_b2,
            "Focusing the agent panel thread should set focused_thread",
        );
        assert!(
            has_thread_entry(sidebar, &session_id_b2),
            "The focused thread should be present in the entries"
        );
    });
}

#[gpui::test]
async fn test_new_thread_button_works_after_adding_folder(cx: &mut TestAppContext) {
    let project = init_test_project_with_agent_panel("/project-a", cx).await;
    let fs = cx.update(|cx| <dyn fs::Fs>::global(cx));
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let path_list_a = PathList::new(&[std::path::PathBuf::from("/project-a")]);

    // Start a thread and send a message so it has history.
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);
    let session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id, path_list_a.clone(), cx).await;
    cx.run_until_parked();

    // Verify the thread appears in the sidebar.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Hello *",]
    );

    // The "New Thread" button should NOT be in "active/draft" state
    // because the panel has a thread with messages.
    sidebar.read_with(cx, |sidebar, _cx| {
        assert!(
            matches!(&sidebar.active_entry, Some(ActiveEntry::Thread { .. })),
            "Panel has a thread with messages, so active_entry should be Thread, got {:?}",
            sidebar.active_entry,
        );
    });

    // Now add a second folder to the workspace, changing the path_list.
    fs.as_fake()
        .insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/project-b", true, cx)
        })
        .await
        .expect("should add worktree");
    cx.run_until_parked();

    // The workspace path_list is now [project-a, project-b]. The active
    // thread's metadata was re-saved with the new paths by the agent panel's
    // project subscription, so it stays visible under the updated group.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a, project-b]", "  Hello *",]
    );

    // The "New Thread" button must still be clickable (not stuck in
    // "active/draft" state). Verify that `active_thread_is_draft` is
    // false — the panel still has the old thread with messages.
    sidebar.read_with(cx, |sidebar, _cx| {
        assert!(
            matches!(&sidebar.active_entry, Some(ActiveEntry::Thread { .. })),
            "After adding a folder the panel still has a thread with messages, \
                 so active_entry should be Thread, got {:?}",
            sidebar.active_entry,
        );
    });

    // Actually click "New Thread" by calling create_new_thread and
    // verify a new draft is created.
    let workspace = multi_workspace.read_with(cx, |mw, _cx| mw.workspace().clone());
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.create_new_thread(&workspace, window, cx);
    });
    cx.run_until_parked();

    // After creating a new thread, the panel should now be in draft
    // state (no messages on the new thread).
    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_draft(
            sidebar,
            &workspace,
            "After creating a new thread active_entry should be Draft",
        );
    });
}

#[gpui::test]
async fn test_cmd_n_shows_new_thread_entry(cx: &mut TestAppContext) {
    // When the user presses Cmd-N (NewThread action) while viewing a
    // non-empty thread, the sidebar should show the "New Thread" entry.
    // This exercises the same code path as the workspace action handler
    // (which bypasses the sidebar's create_new_thread method).
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    // Create a non-empty thread (has messages).
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);

    let session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id, path_list.clone(), cx).await;
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Hello *"]
    );

    // Simulate cmd-n
    let workspace = multi_workspace.read_with(cx, |mw, _cx| mw.workspace().clone());
    panel.update_in(cx, |panel, window, cx| {
        panel.new_thread(&NewThread, window, cx);
    });
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.focus_panel::<AgentPanel>(window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  [+ New Thread]", "  Hello *"],
        "After Cmd-N the sidebar should show a highlighted New Thread entry"
    );

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_draft(
            sidebar,
            &workspace,
            "active_entry should be Draft after Cmd-N",
        );
    });
}

#[gpui::test]
async fn test_draft_with_server_session_shows_as_draft(cx: &mut TestAppContext) {
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    // Create a saved thread so the workspace has history.
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);
    let saved_session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&saved_session_id, path_list.clone(), cx).await;
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Hello *"]
    );

    // Open a new draft thread via a server connection. This gives the
    // conversation a parent_id (session assigned by the server) but
    // no messages have been sent, so active_thread_is_draft() is true.
    let draft_connection = StubAgentConnection::new();
    open_thread_with_connection(&panel, draft_connection, cx);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  [+ New Thread]", "  Hello *"],
        "Draft with a server session should still show as [+ New Thread]"
    );

    let workspace = multi_workspace.read_with(cx, |mw, _cx| mw.workspace().clone());
    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_draft(
            sidebar,
            &workspace,
            "Draft with server session should be Draft, not Thread",
        );
    });
}

#[gpui::test]
async fn test_cmd_n_shows_new_thread_entry_in_absorbed_worktree(cx: &mut TestAppContext) {
    // When the active workspace is an absorbed git worktree, cmd-n
    // should still show the "New Thread" entry under the main repo's
    // header and highlight it as active.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.update_flags(false, vec!["agent-v2".into()]);
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    // Main repo with a linked worktree.
    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    // Worktree checkout pointing back to the main repo.
    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;

    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    // Switch to the worktree workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[1].clone();
        mw.activate(workspace, window, cx);
    });

    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Create a non-empty thread in the worktree workspace.
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&worktree_panel, connection, cx);
    send_message(&worktree_panel, cx);

    let session_id = active_session_id(&worktree_panel, cx);
    let wt_path_list = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    save_test_thread_metadata(&session_id, wt_path_list, cx).await;
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  Hello {wt-feature-a} *"
        ]
    );

    // Simulate Cmd-N in the worktree workspace.
    worktree_panel.update_in(cx, |panel, window, cx| {
        panel.new_thread(&NewThread, window, cx);
    });
    worktree_workspace.update_in(cx, |workspace, window, cx| {
        workspace.focus_panel::<AgentPanel>(window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  [+ New Thread {wt-feature-a}]",
            "  Hello {wt-feature-a} *"
        ],
        "After Cmd-N in an absorbed worktree, the sidebar should show \
             a highlighted New Thread entry under the main repo header"
    );

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_draft(
            sidebar,
            &worktree_workspace,
            "active_entry should be Draft after Cmd-N",
        );
    });
}

async fn init_test_project_with_git(
    worktree_path: &str,
    cx: &mut TestAppContext,
) -> (Entity<project::Project>, Arc<dyn fs::Fs>) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        worktree_path,
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
    let project = project::Project::test(fs.clone(), [worktree_path.as_ref()], cx).await;
    (project, fs)
}

#[gpui::test]
async fn test_search_matches_worktree_name(cx: &mut TestAppContext) {
    let (project, fs) = init_test_project_with_git("/project", cx).await;

    fs.as_fake()
        .with_git_state(std::path::Path::new("/project/.git"), false, |state| {
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from("/wt/rosewood"),
                ref_name: Some("refs/heads/rosewood".into()),
                sha: "abc".into(),
            });
        })
        .unwrap();

    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let main_paths = PathList::new(&[std::path::PathBuf::from("/project")]);
    let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt/rosewood")]);
    save_named_thread_metadata("main-t", "Unrelated Thread", &main_paths, cx).await;
    save_named_thread_metadata("wt-t", "Fix Bug", &wt_paths, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Search for "rosewood" — should match the worktree name, not the title.
    type_in_search(&sidebar, "rosewood", cx);

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project]", "  Fix Bug {rosewood}  <== selected"],
    );
}

#[gpui::test]
async fn test_git_worktree_added_live_updates_sidebar(cx: &mut TestAppContext) {
    let (project, fs) = init_test_project_with_git("/project", cx).await;

    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread against a worktree path that doesn't exist yet.
    let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt/rosewood")]);
    save_named_thread_metadata("wt-thread", "Worktree Thread", &wt_paths, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Thread is not visible yet — no worktree knows about this path.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project]", "  [+ New Thread]"]
    );

    // Now add the worktree to the git state and trigger a rescan.
    fs.as_fake()
        .with_git_state(std::path::Path::new("/project/.git"), true, |state| {
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from("/wt/rosewood"),
                ref_name: Some("refs/heads/rosewood".into()),
                sha: "abc".into(),
            });
        })
        .unwrap();

    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  Worktree Thread {rosewood}",
        ]
    );
}

#[gpui::test]
async fn test_two_worktree_workspaces_absorbed_when_main_added(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    // Create the main repo directory (not opened as a workspace yet).
    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                    "feature-b": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-b",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    // Two worktree checkouts whose .git files point back to the main repo.
    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/wt-feature-b",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-b",
            "src": {},
        }),
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/wt-feature-b".as_ref()], cx).await;

    project_a.update(cx, |p, cx| p.git_scans_complete(cx)).await;
    project_b.update(cx, |p, cx| p.git_scans_complete(cx)).await;

    // Open both worktrees as workspaces — no main repo yet.
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx);
    });
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let paths_a = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    let paths_b = PathList::new(&[std::path::PathBuf::from("/wt-feature-b")]);
    save_named_thread_metadata("thread-a", "Thread A", &paths_a, cx).await;
    save_named_thread_metadata("thread-b", "Thread B", &paths_b, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Without the main repo, each worktree has its own header.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  Thread A {wt-feature-a}",
            "  Thread B {wt-feature-b}",
        ]
    );

    // Configure the main repo to list both worktrees before opening
    // it so the initial git scan picks them up.
    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-b"),
            ref_name: Some("refs/heads/feature-b".into()),
            sha: "bbb".into(),
        });
    })
    .unwrap();

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(main_project.clone(), window, cx);
    });
    cx.run_until_parked();

    // Both worktree workspaces should now be absorbed under the main
    // repo header, with worktree chips.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  Thread A {wt-feature-a}",
            "  Thread B {wt-feature-b}",
        ]
    );
}

#[gpui::test]
async fn test_threadless_workspace_shows_new_thread_with_worktree_chip(cx: &mut TestAppContext) {
    // When a group has two workspaces — one with threads and one
    // without — the threadless workspace should appear as a
    // "New Thread" button with its worktree chip.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    // Main repo with two linked worktrees.
    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                    "feature-b": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-b",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/wt-feature-b",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-b",
            "src": {},
        }),
    )
    .await;

    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-b"),
            ref_name: Some("refs/heads/feature-b".into()),
            sha: "bbb".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Workspace A: worktree feature-a (has threads).
    let project_a = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
    project_a.update(cx, |p, cx| p.git_scans_complete(cx)).await;

    // Workspace B: worktree feature-b (no threads).
    let project_b = project::Project::test(fs.clone(), ["/wt-feature-b".as_ref()], cx).await;
    project_b.update(cx, |p, cx| p.git_scans_complete(cx)).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx);
    });
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Only save a thread for workspace A.
    let paths_a = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    save_named_thread_metadata("thread-a", "Thread A", &paths_a, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Workspace A's thread appears normally. Workspace B (threadless)
    // appears as a "New Thread" button with its worktree chip.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread {wt-feature-b}]",
            "  Thread A {wt-feature-a}",
        ]
    );
}

#[gpui::test]
async fn test_multi_worktree_thread_shows_multiple_chips(cx: &mut TestAppContext) {
    // A thread created in a workspace with roots from different git
    // worktrees should show a chip for each distinct worktree name.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    // Two main repos.
    fs.insert_tree(
        "/project_a",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "olivetti": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/olivetti",
                    },
                    "selectric": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/selectric",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/project_b",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "olivetti": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/olivetti",
                    },
                    "selectric": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/selectric",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    // Worktree checkouts.
    for (repo, branch) in &[
        ("project_a", "olivetti"),
        ("project_a", "selectric"),
        ("project_b", "olivetti"),
        ("project_b", "selectric"),
    ] {
        let worktree_path = format!("/worktrees/{repo}/{branch}/{repo}");
        let gitdir = format!("gitdir: /{repo}/.git/worktrees/{branch}");
        fs.insert_tree(
            &worktree_path,
            serde_json::json!({
                ".git": gitdir,
                "src": {},
            }),
        )
        .await;
    }

    // Register linked worktrees.
    for repo in &["project_a", "project_b"] {
        let git_path = format!("/{repo}/.git");
        fs.with_git_state(std::path::Path::new(&git_path), false, |state| {
            for branch in &["olivetti", "selectric"] {
                state.worktrees.push(git::repository::Worktree {
                    path: std::path::PathBuf::from(format!("/worktrees/{repo}/{branch}/{repo}")),
                    ref_name: Some(format!("refs/heads/{branch}").into()),
                    sha: "aaa".into(),
                });
            }
        })
        .unwrap();
    }

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Open a workspace with the worktree checkout paths as roots
    // (this is the workspace the thread was created in).
    let project = project::Project::test(
        fs.clone(),
        [
            "/worktrees/project_a/olivetti/project_a".as_ref(),
            "/worktrees/project_b/selectric/project_b".as_ref(),
        ],
        cx,
    )
    .await;
    project.update(cx, |p, cx| p.git_scans_complete(cx)).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread under the same paths as the workspace roots.
    let thread_paths = PathList::new(&[
        std::path::PathBuf::from("/worktrees/project_a/olivetti/project_a"),
        std::path::PathBuf::from("/worktrees/project_b/selectric/project_b"),
    ]);
    save_named_thread_metadata("wt-thread", "Cross Worktree Thread", &thread_paths, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Should show two distinct worktree chips.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project_a, project_b]",
            "  Cross Worktree Thread {olivetti}, {selectric}",
        ]
    );
}

#[gpui::test]
async fn test_same_named_worktree_chips_are_deduplicated(cx: &mut TestAppContext) {
    // When a thread's roots span multiple repos but share the same
    // worktree name (e.g. both in "olivetti"), only one chip should
    // appear.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project_a",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "olivetti": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/olivetti",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/project_b",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "olivetti": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/olivetti",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    for repo in &["project_a", "project_b"] {
        let worktree_path = format!("/worktrees/{repo}/olivetti/{repo}");
        let gitdir = format!("gitdir: /{repo}/.git/worktrees/olivetti");
        fs.insert_tree(
            &worktree_path,
            serde_json::json!({
                ".git": gitdir,
                "src": {},
            }),
        )
        .await;

        let git_path = format!("/{repo}/.git");
        fs.with_git_state(std::path::Path::new(&git_path), false, |state| {
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from(format!("/worktrees/{repo}/olivetti/{repo}")),
                ref_name: Some("refs/heads/olivetti".into()),
                sha: "aaa".into(),
            });
        })
        .unwrap();
    }

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project = project::Project::test(
        fs.clone(),
        [
            "/worktrees/project_a/olivetti/project_a".as_ref(),
            "/worktrees/project_b/olivetti/project_b".as_ref(),
        ],
        cx,
    )
    .await;
    project.update(cx, |p, cx| p.git_scans_complete(cx)).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Thread with roots in both repos' "olivetti" worktrees.
    let thread_paths = PathList::new(&[
        std::path::PathBuf::from("/worktrees/project_a/olivetti/project_a"),
        std::path::PathBuf::from("/worktrees/project_b/olivetti/project_b"),
    ]);
    save_named_thread_metadata("wt-thread", "Same Branch Thread", &thread_paths, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Both worktree paths have the name "olivetti", so only one chip.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project_a, project_b]",
            "  Same Branch Thread {olivetti}",
        ]
    );
}

#[gpui::test]
async fn test_absorbed_worktree_running_thread_shows_live_status(cx: &mut TestAppContext) {
    // When a worktree workspace is absorbed under the main repo, a
    // running thread in the worktree's agent panel should still show
    // live status (spinner + "(running)") in the sidebar.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.update_flags(false, vec!["agent-v2".into()]);
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    // Main repo with a linked worktree.
    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    // Worktree checkout pointing back to the main repo.
    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;

    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    // Create the MultiWorkspace with both projects.
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Add an agent panel to the worktree workspace so we can run a
    // thread inside it.
    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    // Switch back to the main workspace before setting up the sidebar.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[0].clone();
        mw.activate(workspace, window, cx);
    });

    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Start a thread in the worktree workspace's panel and keep it
    // generating (don't resolve it).
    let connection = StubAgentConnection::new();
    open_thread_with_connection(&worktree_panel, connection.clone(), cx);
    send_message(&worktree_panel, cx);

    let session_id = active_session_id(&worktree_panel, cx);

    // Save metadata so the sidebar knows about this thread.
    let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    save_test_thread_metadata(&session_id, wt_paths, cx).await;

    // Keep the thread generating by sending a chunk without ending
    // the turn.
    cx.update(|_, cx| {
        connection.send_update(
            session_id.clone(),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("working...".into())),
            cx,
        );
    });
    cx.run_until_parked();

    // The worktree thread should be absorbed under the main project
    // and show live running status.
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(
        entries,
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  Hello {wt-feature-a} * (running)",
        ]
    );
}

#[gpui::test]
async fn test_absorbed_worktree_completion_triggers_notification(cx: &mut TestAppContext) {
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.update_flags(false, vec!["agent-v2".into()]);
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;

    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[0].clone();
        mw.activate(workspace, window, cx);
    });

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let connection = StubAgentConnection::new();
    open_thread_with_connection(&worktree_panel, connection.clone(), cx);
    send_message(&worktree_panel, cx);

    let session_id = active_session_id(&worktree_panel, cx);
    let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    save_test_thread_metadata(&session_id, wt_paths, cx).await;

    cx.update(|_, cx| {
        connection.send_update(
            session_id.clone(),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("working...".into())),
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  Hello {wt-feature-a} * (running)",
        ]
    );

    connection.end_turn(session_id, acp::StopReason::EndTurn);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  Hello {wt-feature-a} * (!)",
        ]
    );
}

#[gpui::test]
async fn test_clicking_worktree_thread_opens_workspace_when_none_exists(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;

    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Only open the main repo — no workspace for the worktree.
    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread for the worktree path (no workspace for it).
    let paths_wt = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    save_named_thread_metadata("thread-wt", "WT Thread", &paths_wt, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Thread should appear under the main repo with a worktree chip.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  WT Thread {wt-feature-a}"
        ],
    );

    // Only 1 workspace should exist.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().len()),
        1,
    );

    // Focus the sidebar and select the worktree thread.
    open_and_focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(2); // index 0 is header, 1 is new thread, 2 is the thread
    });

    // Confirm to open the worktree thread.
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    // A new workspace should have been created for the worktree path.
    let new_workspace = multi_workspace.read_with(cx, |mw, _| {
        assert_eq!(
            mw.workspaces().len(),
            2,
            "confirming a worktree thread without a workspace should open one",
        );
        mw.workspaces()[1].clone()
    });

    let new_path_list =
        new_workspace.read_with(cx, |_, cx| workspace_path_list(&new_workspace, cx));
    assert_eq!(
        new_path_list,
        PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]),
        "the new workspace should have been opened for the worktree path",
    );
}

#[gpui::test]
async fn test_clicking_worktree_thread_does_not_briefly_render_as_separate_project(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;

    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let paths_wt = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    save_named_thread_metadata("thread-wt", "WT Thread", &paths_wt, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  WT Thread {wt-feature-a}"
        ],
    );

    open_and_focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(2);
    });

    let assert_sidebar_state = |sidebar: &mut Sidebar, _cx: &mut Context<Sidebar>| {
        let mut project_headers = sidebar.contents.entries.iter().filter_map(|entry| {
            if let ListEntry::ProjectHeader { label, .. } = entry {
                Some(label.as_ref())
            } else {
                None
            }
        });

        let Some(project_header) = project_headers.next() else {
            panic!("expected exactly one sidebar project header named `project`, found none");
        };
        assert_eq!(
            project_header, "project",
            "expected the only sidebar project header to be `project`"
        );
        if let Some(unexpected_header) = project_headers.next() {
            panic!(
                "expected exactly one sidebar project header named `project`, found extra header `{unexpected_header}`"
            );
        }

        let mut saw_expected_thread = false;
        for entry in &sidebar.contents.entries {
            match entry {
                ListEntry::ProjectHeader { label, .. } => {
                    assert_eq!(
                        label.as_ref(),
                        "project",
                        "expected the only sidebar project header to be `project`"
                    );
                }
                ListEntry::Thread(thread)
                    if thread.metadata.title.as_ref() == "WT Thread"
                        && thread.worktrees.first().map(|wt| wt.name.as_ref())
                            == Some("wt-feature-a") =>
                {
                    saw_expected_thread = true;
                }
                ListEntry::Thread(thread) => {
                    let title = thread.metadata.title.as_ref();
                    let worktree_name = thread
                        .worktrees
                        .first()
                        .map(|wt| wt.name.as_ref())
                        .unwrap_or("<none>");
                    panic!(
                        "unexpected sidebar thread while opening linked worktree thread: title=`{title}`, worktree=`{worktree_name}`"
                    );
                }
                ListEntry::ViewMore { .. } => {
                    panic!("unexpected `View More` entry while opening linked worktree thread");
                }
                ListEntry::NewThread { .. } => {}
            }
        }

        assert!(
            saw_expected_thread,
            "expected the sidebar to keep showing `WT Thread {{wt-feature-a}}` under `project`"
        );
    };

    sidebar
        .update(cx, |_, cx| cx.observe_self(assert_sidebar_state))
        .detach();

    let window = cx.windows()[0];
    cx.update_window(window, |_, window, cx| {
        window.dispatch_action(Confirm.boxed_clone(), cx);
    })
    .unwrap();

    cx.run_until_parked();

    sidebar.update(cx, assert_sidebar_state);
}

#[gpui::test]
async fn test_clicking_absorbed_worktree_thread_activates_worktree_workspace(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;

    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Activate the main workspace before setting up the sidebar.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[0].clone();
        mw.activate(workspace, window, cx);
    });

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let paths_main = PathList::new(&[std::path::PathBuf::from("/project")]);
    let paths_wt = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    save_named_thread_metadata("thread-main", "Main Thread", &paths_main, cx).await;
    save_named_thread_metadata("thread-wt", "WT Thread", &paths_wt, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // The worktree workspace should be absorbed under the main repo.
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0], "v [project]");
    assert!(entries.contains(&"  Main Thread".to_string()));
    assert!(entries.contains(&"  WT Thread {wt-feature-a}".to_string()));

    let wt_thread_index = entries
        .iter()
        .position(|e| e.contains("WT Thread"))
        .expect("should find the worktree thread entry");

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        0,
        "main workspace should be active initially"
    );

    // Focus the sidebar and select the absorbed worktree thread.
    open_and_focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(wt_thread_index);
    });

    // Confirm to activate the worktree thread.
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    // The worktree workspace should now be active, not the main one.
    let active_workspace = multi_workspace.read_with(cx, |mw, _| {
        mw.workspaces()[mw.active_workspace_index()].clone()
    });
    assert_eq!(
        active_workspace, worktree_workspace,
        "clicking an absorbed worktree thread should activate the worktree workspace"
    );
}

#[gpui::test]
async fn test_activate_archived_thread_with_saved_paths_activates_matching_workspace(
    cx: &mut TestAppContext,
) {
    // Thread has saved metadata in ThreadStore. A matching workspace is
    // already open. Expected: activates the matching workspace.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b, window, cx);
    });

    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread with path_list pointing to project-b.
    let path_list_b = PathList::new(&[std::path::PathBuf::from("/project-b")]);
    let session_id = acp::SessionId::new(Arc::from("archived-1"));
    save_test_thread_metadata(&session_id, path_list_b.clone(), cx).await;

    // Ensure workspace A is active.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[0].clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        0
    );

    // Call activate_archived_thread – should resolve saved paths and
    // switch to the workspace for project-b.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                session_id: session_id.clone(),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "Archived Thread".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: PathList::new(&[PathBuf::from("/project-b")]),
                archived: false,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        1,
        "should have activated the workspace matching the saved path_list"
    );
}

#[gpui::test]
async fn test_activate_archived_thread_cwd_fallback_with_matching_workspace(
    cx: &mut TestAppContext,
) {
    // Thread has no saved metadata but session_info has cwd. A matching
    // workspace is open. Expected: uses cwd to find and activate it.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b, window, cx);
    });

    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Start with workspace A active.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[0].clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        0
    );

    // No thread saved to the store – cwd is the only path hint.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                session_id: acp::SessionId::new(Arc::from("unknown-session")),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "CWD Thread".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: PathList::new(&[std::path::PathBuf::from("/project-b")]),
                archived: false,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        1,
        "should have activated the workspace matching the cwd"
    );
}

#[gpui::test]
async fn test_activate_archived_thread_no_paths_no_cwd_uses_active_workspace(
    cx: &mut TestAppContext,
) {
    // Thread has no saved metadata and no cwd. Expected: falls back to
    // the currently active workspace.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b, window, cx);
    });

    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Activate workspace B (index 1) to make it the active one.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[1].clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        1
    );

    // No saved thread, no cwd – should fall back to the active workspace.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                session_id: acp::SessionId::new(Arc::from("no-context-session")),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "Contextless Thread".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: PathList::default(),
                archived: false,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
        1,
        "should have stayed on the active workspace when no path info is available"
    );
}

#[gpui::test]
async fn test_activate_archived_thread_saved_paths_opens_new_workspace(cx: &mut TestAppContext) {
    // Thread has saved metadata pointing to a path with no open workspace.
    // Expected: opens a new workspace for that path.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread with path_list pointing to project-b – which has no
    // open workspace.
    let path_list_b = PathList::new(&[std::path::PathBuf::from("/project-b")]);
    let session_id = acp::SessionId::new(Arc::from("archived-new-ws"));

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().len()),
        1,
        "should start with one workspace"
    );

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                session_id: session_id.clone(),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "New WS Thread".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: path_list_b,
                archived: false,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().len()),
        2,
        "should have opened a second workspace for the archived thread's saved paths"
    );
}

#[gpui::test]
async fn test_activate_archived_thread_reuses_workspace_in_another_window(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let multi_workspace_a =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
    let multi_workspace_b =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project_b, window, cx));

    let multi_workspace_a_entity = multi_workspace_a.root(cx).unwrap();

    let cx_a = &mut gpui::VisualTestContext::from_window(multi_workspace_a.into(), cx);
    let sidebar = setup_sidebar(&multi_workspace_a_entity, cx_a);

    let session_id = acp::SessionId::new(Arc::from("archived-cross-window"));

    sidebar.update_in(cx_a, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                session_id: session_id.clone(),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "Cross Window Thread".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: PathList::new(&[PathBuf::from("/project-b")]),
                archived: false,
            },
            window,
            cx,
        );
    });
    cx_a.run_until_parked();

    assert_eq!(
        multi_workspace_a
            .read_with(cx_a, |mw, _| mw.workspaces().len())
            .unwrap(),
        1,
        "should not add the other window's workspace into the current window"
    );
    assert_eq!(
        multi_workspace_b
            .read_with(cx_a, |mw, _| mw.workspaces().len())
            .unwrap(),
        1,
        "should reuse the existing workspace in the other window"
    );
    assert!(
        cx_a.read(|cx| cx.active_window().unwrap()) == *multi_workspace_b,
        "should activate the window that already owns the matching workspace"
    );
    sidebar.read_with(cx_a, |sidebar, _| {
            assert!(
                !matches!(&sidebar.active_entry, Some(ActiveEntry::Thread { session_id: id, .. }) if id == &session_id),
                "source window's sidebar should not eagerly claim focus for a thread opened in another window"
            );
        });
}

#[gpui::test]
async fn test_activate_archived_thread_reuses_workspace_in_another_window_with_target_sidebar(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let multi_workspace_a =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
    let multi_workspace_b =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project_b.clone(), window, cx));

    let multi_workspace_a_entity = multi_workspace_a.root(cx).unwrap();
    let multi_workspace_b_entity = multi_workspace_b.root(cx).unwrap();

    let cx_a = &mut gpui::VisualTestContext::from_window(multi_workspace_a.into(), cx);
    let sidebar_a = setup_sidebar(&multi_workspace_a_entity, cx_a);

    let cx_b = &mut gpui::VisualTestContext::from_window(multi_workspace_b.into(), cx);
    let sidebar_b = setup_sidebar(&multi_workspace_b_entity, cx_b);
    let workspace_b = multi_workspace_b_entity.read_with(cx_b, |mw, _| mw.workspace().clone());
    let _panel_b = add_agent_panel(&workspace_b, cx_b);

    let session_id = acp::SessionId::new(Arc::from("archived-cross-window-with-sidebar"));

    sidebar_a.update_in(cx_a, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                session_id: session_id.clone(),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "Cross Window Thread".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: PathList::new(&[PathBuf::from("/project-b")]),
                archived: false,
            },
            window,
            cx,
        );
    });
    cx_a.run_until_parked();

    assert_eq!(
        multi_workspace_a
            .read_with(cx_a, |mw, _| mw.workspaces().len())
            .unwrap(),
        1,
        "should not add the other window's workspace into the current window"
    );
    assert_eq!(
        multi_workspace_b
            .read_with(cx_a, |mw, _| mw.workspaces().len())
            .unwrap(),
        1,
        "should reuse the existing workspace in the other window"
    );
    assert!(
        cx_a.read(|cx| cx.active_window().unwrap()) == *multi_workspace_b,
        "should activate the window that already owns the matching workspace"
    );
    sidebar_a.read_with(cx_a, |sidebar, _| {
            assert!(
                !matches!(&sidebar.active_entry, Some(ActiveEntry::Thread { session_id: id, .. }) if id == &session_id),
                "source window's sidebar should not eagerly claim focus for a thread opened in another window"
            );
        });
    sidebar_b.read_with(cx_b, |sidebar, _| {
        assert_active_thread(
            sidebar,
            &session_id,
            "target window's sidebar should eagerly focus the activated archived thread",
        );
    });
}

#[gpui::test]
async fn test_activate_archived_thread_prefers_current_window_for_matching_paths(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_b = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;

    let multi_workspace_b =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project_b, window, cx));
    let multi_workspace_a =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

    let multi_workspace_a_entity = multi_workspace_a.root(cx).unwrap();

    let cx_a = &mut gpui::VisualTestContext::from_window(multi_workspace_a.into(), cx);
    let sidebar_a = setup_sidebar(&multi_workspace_a_entity, cx_a);

    let session_id = acp::SessionId::new(Arc::from("archived-current-window"));

    sidebar_a.update_in(cx_a, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                session_id: session_id.clone(),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: "Current Window Thread".into(),
                updated_at: Utc::now(),
                created_at: None,
                folder_paths: PathList::new(&[PathBuf::from("/project-a")]),
                archived: false,
            },
            window,
            cx,
        );
    });
    cx_a.run_until_parked();

    assert!(
        cx_a.read(|cx| cx.active_window().unwrap()) == *multi_workspace_a,
        "should keep activation in the current window when it already has a matching workspace"
    );
    sidebar_a.read_with(cx_a, |sidebar, _| {
        assert_active_thread(
            sidebar,
            &session_id,
            "current window's sidebar should eagerly focus the activated archived thread",
        );
    });
    assert_eq!(
        multi_workspace_a
            .read_with(cx_a, |mw, _| mw.workspaces().len())
            .unwrap(),
        1,
        "current window should continue reusing its existing workspace"
    );
    assert_eq!(
        multi_workspace_b
            .read_with(cx_a, |mw, _| mw.workspaces().len())
            .unwrap(),
        1,
        "other windows should not be activated just because they also match the saved paths"
    );
}

#[gpui::test]
async fn test_archive_thread_uses_next_threads_own_workspace(cx: &mut TestAppContext) {
    // Regression test: archive_thread previously always loaded the next thread
    // through group_workspace (the main workspace's ProjectHeader), even when
    // the next thread belonged to an absorbed linked-worktree workspace. That
    // caused the worktree thread to be loaded in the main panel, which bound it
    // to the main project and corrupted its stored folder_paths.
    //
    // The fix: use next.workspace (ThreadEntryWorkspace::Open) when available,
    // falling back to group_workspace only for Closed workspaces.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.update_flags(false, vec!["agent-v2".into()]);
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;

    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Activate main workspace so the sidebar tracks the main panel.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces()[0].clone();
        mw.activate(workspace, window, cx);
    });

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let main_workspace = multi_workspace.read_with(cx, |mw, _| mw.workspaces()[0].clone());
    let main_panel = add_agent_panel(&main_workspace, cx);
    let _worktree_panel = add_agent_panel(&worktree_workspace, cx);

    // Open Thread 2 in the main panel and keep it running.
    let connection = StubAgentConnection::new();
    open_thread_with_connection(&main_panel, connection.clone(), cx);
    send_message(&main_panel, cx);

    let thread2_session_id = active_session_id(&main_panel, cx);

    cx.update(|_, cx| {
        connection.send_update(
            thread2_session_id.clone(),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("working...".into())),
            cx,
        );
    });

    // Save thread 2's metadata with a newer timestamp so it sorts above thread 1.
    save_thread_metadata(
        thread2_session_id.clone(),
        "Thread 2".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        PathList::new(&[std::path::PathBuf::from("/project")]),
        cx,
    );

    // Save thread 1's metadata with the worktree path and an older timestamp so
    // it sorts below thread 2. archive_thread will find it as the "next" candidate.
    let thread1_session_id = acp::SessionId::new(Arc::from("thread1-worktree-session"));
    save_thread_metadata(
        thread1_session_id,
        "Thread 1".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]),
        cx,
    );

    cx.run_until_parked();

    // Verify the sidebar absorbed thread 1 under [project] with the worktree chip.
    let entries_before = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries_before.iter().any(|s| s.contains("{wt-feature-a}")),
        "Thread 1 should appear with the linked-worktree chip before archiving: {:?}",
        entries_before
    );

    // The sidebar should track T2 as the focused thread (derived from the
    // main panel's active view).
    sidebar.read_with(cx, |s, _| {
        assert_active_thread(
            s,
            &thread2_session_id,
            "focused thread should be Thread 2 before archiving",
        );
    });

    // Archive thread 2.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&thread2_session_id, window, cx);
    });

    cx.run_until_parked();

    // The main panel's active thread must still be thread 2.
    let main_active = main_panel.read_with(cx, |panel, cx| {
        panel
            .active_agent_thread(cx)
            .map(|t| t.read(cx).session_id().clone())
    });
    assert_eq!(
        main_active,
        Some(thread2_session_id.clone()),
        "main panel should not have been taken over by loading the linked-worktree thread T1; \
             before the fix, archive_thread used group_workspace instead of next.workspace, \
             causing T1 to be loaded in the wrong panel"
    );

    // Thread 1 should still appear in the sidebar with its worktree chip
    // (Thread 2 was archived so it is gone from the list).
    let entries_after = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries_after.iter().any(|s| s.contains("{wt-feature-a}")),
        "T1 should still carry its linked-worktree chip after archiving T2: {:?}",
        entries_after
    );
}

#[gpui::test]
async fn test_linked_worktree_threads_not_duplicated_across_groups(cx: &mut TestAppContext) {
    // When a multi-root workspace (e.g. [/other, /project]) shares a
    // repo with a single-root workspace (e.g. [/project]), linked
    // worktree threads from the shared repo should only appear under
    // the dedicated group [project], not under [other, project].
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    // Two independent repos, each with their own git history.
    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-a": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-a",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/wt-feature-a",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-a",
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/other",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    // Register the linked worktree in the main repo.
    fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
        state.worktrees.push(git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
        });
    })
    .unwrap();

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Workspace 1: just /project.
    let project_only = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    project_only
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    // Workspace 2: /other and /project together (multi-root).
    let multi_root =
        project::Project::test(fs.clone(), ["/other".as_ref(), "/project".as_ref()], cx).await;
    multi_root
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_only.clone(), window, cx));
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(multi_root.clone(), window, cx);
    });
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread under the linked worktree path.
    let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
    save_named_thread_metadata("wt-thread", "Worktree Thread", &wt_paths, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // The thread should appear only under [project] (the dedicated
    // group for the /project repo), not under [other, project].
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  [+ New Thread]",
            "  Worktree Thread {wt-feature-a}",
            "v [other, project]",
            "  [+ New Thread]",
        ]
    );
}

#[gpui::test]
async fn test_thread_switcher_ordering(cx: &mut TestAppContext) {
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    let switcher_ids =
        |sidebar: &Entity<Sidebar>, cx: &mut gpui::VisualTestContext| -> Vec<acp::SessionId> {
            sidebar.read_with(cx, |sidebar, cx| {
                let switcher = sidebar
                    .thread_switcher
                    .as_ref()
                    .expect("switcher should be open");
                switcher
                    .read(cx)
                    .entries()
                    .iter()
                    .map(|e| e.session_id.clone())
                    .collect()
            })
        };

    let switcher_selected_id =
        |sidebar: &Entity<Sidebar>, cx: &mut gpui::VisualTestContext| -> acp::SessionId {
            sidebar.read_with(cx, |sidebar, cx| {
                let switcher = sidebar
                    .thread_switcher
                    .as_ref()
                    .expect("switcher should be open");
                let s = switcher.read(cx);
                s.selected_entry()
                    .expect("should have selection")
                    .session_id
                    .clone()
            })
        };

    // ── Setup: create three threads with distinct created_at times ──────
    // Thread C (oldest), Thread B, Thread A (newest) — by created_at.
    // We send messages in each so they also get last_message_sent_or_queued timestamps.
    let connection_c = StubAgentConnection::new();
    connection_c.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done C".into()),
    )]);
    open_thread_with_connection(&panel, connection_c, cx);
    send_message(&panel, cx);
    let session_id_c = active_session_id(&panel, cx);
    save_thread_metadata(
        session_id_c.clone(),
        "Thread C".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap()),
        path_list.clone(),
        cx,
    );

    let connection_b = StubAgentConnection::new();
    connection_b.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done B".into()),
    )]);
    open_thread_with_connection(&panel, connection_b, cx);
    send_message(&panel, cx);
    let session_id_b = active_session_id(&panel, cx);
    save_thread_metadata(
        session_id_b.clone(),
        "Thread B".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap()),
        path_list.clone(),
        cx,
    );

    let connection_a = StubAgentConnection::new();
    connection_a.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done A".into()),
    )]);
    open_thread_with_connection(&panel, connection_a, cx);
    send_message(&panel, cx);
    let session_id_a = active_session_id(&panel, cx);
    save_thread_metadata(
        session_id_a.clone(),
        "Thread A".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 3, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 3, 0, 0, 0).unwrap()),
        path_list.clone(),
        cx,
    );

    // All three threads are now live. Thread A was opened last, so it's
    // the one being viewed. Opening each thread called record_thread_access,
    // so all three have last_accessed_at set.
    // Access order is: A (most recent), B, C (oldest).

    // ── 1. Open switcher: threads sorted by last_accessed_at ───────────
    open_and_focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.on_toggle_thread_switcher(&ToggleThreadSwitcher::default(), window, cx);
    });
    cx.run_until_parked();

    // All three have last_accessed_at, so they sort by access time.
    // A was accessed most recently (it's the currently viewed thread),
    // then B, then C.
    assert_eq!(
        switcher_ids(&sidebar, cx),
        vec![
            session_id_a.clone(),
            session_id_b.clone(),
            session_id_c.clone()
        ],
    );
    // First ctrl-tab selects the second entry (B).
    assert_eq!(switcher_selected_id(&sidebar, cx), session_id_b);

    // Dismiss the switcher without confirming.
    sidebar.update_in(cx, |sidebar, _window, cx| {
        sidebar.dismiss_thread_switcher(cx);
    });
    cx.run_until_parked();

    // ── 2. Confirm on Thread C: it becomes most-recently-accessed ──────
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.on_toggle_thread_switcher(&ToggleThreadSwitcher::default(), window, cx);
    });
    cx.run_until_parked();

    // Cycle twice to land on Thread C (index 2).
    sidebar.read_with(cx, |sidebar, cx| {
        let switcher = sidebar.thread_switcher.as_ref().unwrap();
        assert_eq!(switcher.read(cx).selected_index(), 1);
    });
    sidebar.update_in(cx, |sidebar, _window, cx| {
        sidebar
            .thread_switcher
            .as_ref()
            .unwrap()
            .update(cx, |s, cx| s.cycle_selection(cx));
    });
    cx.run_until_parked();
    assert_eq!(switcher_selected_id(&sidebar, cx), session_id_c);

    assert!(sidebar.update(cx, |sidebar, _cx| sidebar.thread_last_accessed.is_empty()));

    // Confirm on Thread C.
    sidebar.update_in(cx, |sidebar, window, cx| {
        let switcher = sidebar.thread_switcher.as_ref().unwrap();
        let focus = switcher.focus_handle(cx);
        focus.dispatch_action(&menu::Confirm, window, cx);
    });
    cx.run_until_parked();

    // Switcher should be dismissed after confirm.
    sidebar.read_with(cx, |sidebar, _cx| {
        assert!(
            sidebar.thread_switcher.is_none(),
            "switcher should be dismissed"
        );
    });

    sidebar.update(cx, |sidebar, _cx| {
        let last_accessed = sidebar
            .thread_last_accessed
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(last_accessed.len(), 1);
        assert!(last_accessed.contains(&session_id_c));
        assert!(
            sidebar
                .active_entry
                .as_ref()
                .expect("active_entry should be set")
                .is_active_thread(&session_id_c)
        );
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.on_toggle_thread_switcher(&ToggleThreadSwitcher::default(), window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        switcher_ids(&sidebar, cx),
        vec![
            session_id_c.clone(),
            session_id_a.clone(),
            session_id_b.clone()
        ],
    );

    // Confirm on Thread A.
    sidebar.update_in(cx, |sidebar, window, cx| {
        let switcher = sidebar.thread_switcher.as_ref().unwrap();
        let focus = switcher.focus_handle(cx);
        focus.dispatch_action(&menu::Confirm, window, cx);
    });
    cx.run_until_parked();

    sidebar.update(cx, |sidebar, _cx| {
        let last_accessed = sidebar
            .thread_last_accessed
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(last_accessed.len(), 2);
        assert!(last_accessed.contains(&session_id_c));
        assert!(last_accessed.contains(&session_id_a));
        assert!(
            sidebar
                .active_entry
                .as_ref()
                .expect("active_entry should be set")
                .is_active_thread(&session_id_a)
        );
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.on_toggle_thread_switcher(&ToggleThreadSwitcher::default(), window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        switcher_ids(&sidebar, cx),
        vec![
            session_id_a.clone(),
            session_id_c.clone(),
            session_id_b.clone(),
        ],
    );

    sidebar.update_in(cx, |sidebar, _window, cx| {
        let switcher = sidebar.thread_switcher.as_ref().unwrap();
        switcher.update(cx, |switcher, cx| switcher.cycle_selection(cx));
    });
    cx.run_until_parked();

    // Confirm on Thread B.
    sidebar.update_in(cx, |sidebar, window, cx| {
        let switcher = sidebar.thread_switcher.as_ref().unwrap();
        let focus = switcher.focus_handle(cx);
        focus.dispatch_action(&menu::Confirm, window, cx);
    });
    cx.run_until_parked();

    sidebar.update(cx, |sidebar, _cx| {
        let last_accessed = sidebar
            .thread_last_accessed
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(last_accessed.len(), 3);
        assert!(last_accessed.contains(&session_id_c));
        assert!(last_accessed.contains(&session_id_a));
        assert!(last_accessed.contains(&session_id_b));
        assert!(
            sidebar
                .active_entry
                .as_ref()
                .expect("active_entry should be set")
                .is_active_thread(&session_id_b)
        );
    });

    // ── 3. Add a historical thread (no last_accessed_at, no message sent) ──
    // This thread was never opened in a panel — it only exists in metadata.
    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-historical")),
        "Historical Thread".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap()),
        path_list.clone(),
        cx,
    );

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.on_toggle_thread_switcher(&ToggleThreadSwitcher::default(), window, cx);
    });
    cx.run_until_parked();

    // Historical Thread has no last_accessed_at and no last_message_sent_or_queued,
    // so it falls to tier 3 (sorted by created_at). It should appear after all
    // accessed threads, even though its created_at (June 2024) is much later
    // than the others.
    //
    // But the live threads (A, B, C) each had send_message called which sets
    // last_message_sent_or_queued. So for the accessed threads (tier 1) the
    // sort key is last_accessed_at; for Historical Thread (tier 3) it's created_at.
    let session_id_hist = acp::SessionId::new(Arc::from("thread-historical"));

    let ids = switcher_ids(&sidebar, cx);
    assert_eq!(
        ids,
        vec![
            session_id_b.clone(),
            session_id_a.clone(),
            session_id_c.clone(),
            session_id_hist.clone()
        ],
    );

    sidebar.update_in(cx, |sidebar, _window, cx| {
        sidebar.dismiss_thread_switcher(cx);
    });
    cx.run_until_parked();

    // ── 4. Add another historical thread with older created_at ─────────
    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-old-historical")),
        "Old Historical Thread".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2023, 6, 1, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2023, 6, 1, 0, 0, 0).unwrap()),
        path_list,
        cx,
    );

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.on_toggle_thread_switcher(&ToggleThreadSwitcher::default(), window, cx);
    });
    cx.run_until_parked();

    // Both historical threads have no access or message times. They should
    // appear after accessed threads, sorted by created_at (newest first).
    let session_id_old_hist = acp::SessionId::new(Arc::from("thread-old-historical"));
    let ids = switcher_ids(&sidebar, cx);
    assert_eq!(
        ids,
        vec![
            session_id_b,
            session_id_a,
            session_id_c,
            session_id_hist,
            session_id_old_hist,
        ],
    );

    sidebar.update_in(cx, |sidebar, _window, cx| {
        sidebar.dismiss_thread_switcher(cx);
    });
    cx.run_until_parked();
}

#[gpui::test]
async fn test_archive_thread_keeps_metadata_but_hides_from_sidebar(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-to-archive")),
        "Thread To Archive".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    let entries = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries.iter().any(|e| e.contains("Thread To Archive")),
        "expected thread to be visible before archiving, got: {entries:?}"
    );

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(
            &acp::SessionId::new(Arc::from("thread-to-archive")),
            window,
            cx,
        );
    });
    cx.run_until_parked();

    let entries = visible_entries_as_strings(&sidebar, cx);
    assert!(
        !entries.iter().any(|e| e.contains("Thread To Archive")),
        "expected thread to be hidden after archiving, got: {entries:?}"
    );

    cx.update(|_, cx| {
        let store = ThreadMetadataStore::global(cx);
        let archived: Vec<_> = store.read(cx).archived_entries().collect();
        assert_eq!(archived.len(), 1);
        assert_eq!(archived[0].session_id.0.as_ref(), "thread-to-archive");
        assert!(archived[0].archived);
    });
}

#[gpui::test]
async fn test_archived_threads_excluded_from_sidebar_entries(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("visible-thread")),
        "Visible Thread".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        path_list.clone(),
        cx,
    );

    let archived_thread_session_id = acp::SessionId::new(Arc::from("archived-thread"));
    save_thread_metadata(
        archived_thread_session_id.clone(),
        "Archived Thread".into(),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        path_list,
        cx,
    );

    cx.update(|_, cx| {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.archive(&archived_thread_session_id, cx)
        })
    });
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    let entries = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries.iter().any(|e| e.contains("Visible Thread")),
        "expected visible thread in sidebar, got: {entries:?}"
    );
    assert!(
        !entries.iter().any(|e| e.contains("Archived Thread")),
        "expected archived thread to be hidden from sidebar, got: {entries:?}"
    );

    cx.update(|_, cx| {
        let store = ThreadMetadataStore::global(cx);
        let all: Vec<_> = store.read(cx).entries().collect();
        assert_eq!(
            all.len(),
            2,
            "expected 2 total entries in the store, got: {}",
            all.len()
        );

        let archived: Vec<_> = store.read(cx).archived_entries().collect();
        assert_eq!(archived.len(), 1);
        assert_eq!(archived[0].session_id.0.as_ref(), "archived-thread");
    });
}

mod property_test {
    use super::*;
    use gpui::EntityId;

    struct UnopenedWorktree {
        path: String,
    }

    struct TestState {
        fs: Arc<FakeFs>,
        thread_counter: u32,
        workspace_counter: u32,
        worktree_counter: u32,
        saved_thread_ids: Vec<acp::SessionId>,
        workspace_paths: Vec<String>,
        main_repo_indices: Vec<usize>,
        unopened_worktrees: Vec<UnopenedWorktree>,
    }

    impl TestState {
        fn new(fs: Arc<FakeFs>, initial_workspace_path: String) -> Self {
            Self {
                fs,
                thread_counter: 0,
                workspace_counter: 1,
                worktree_counter: 0,
                saved_thread_ids: Vec::new(),
                workspace_paths: vec![initial_workspace_path],
                main_repo_indices: vec![0],
                unopened_worktrees: Vec::new(),
            }
        }

        fn next_thread_id(&mut self) -> acp::SessionId {
            let id = self.thread_counter;
            self.thread_counter += 1;
            let session_id = acp::SessionId::new(Arc::from(format!("prop-thread-{id}")));
            self.saved_thread_ids.push(session_id.clone());
            session_id
        }

        fn remove_thread(&mut self, index: usize) -> acp::SessionId {
            self.saved_thread_ids.remove(index)
        }

        fn next_workspace_path(&mut self) -> String {
            let id = self.workspace_counter;
            self.workspace_counter += 1;
            format!("/prop-project-{id}")
        }

        fn next_worktree_name(&mut self) -> String {
            let id = self.worktree_counter;
            self.worktree_counter += 1;
            format!("wt-{id}")
        }
    }

    #[derive(Debug)]
    enum Operation {
        SaveThread { workspace_index: usize },
        SaveWorktreeThread { worktree_index: usize },
        DeleteThread { index: usize },
        ToggleAgentPanel,
        CreateDraftThread,
        AddWorkspace,
        OpenWorktreeAsWorkspace { worktree_index: usize },
        RemoveWorkspace { index: usize },
        SwitchWorkspace { index: usize },
        AddLinkedWorktree { workspace_index: usize },
    }

    // Distribution (out of 20 slots):
    //   SaveThread:              5 slots (~23%)
    //   SaveWorktreeThread:      2 slots (~9%)
    //   DeleteThread:            2 slots (~9%)
    //   ToggleAgentPanel:        2 slots (~9%)
    //   CreateDraftThread:       2 slots (~9%)
    //   AddWorkspace:            1 slot  (~5%)
    //   OpenWorktreeAsWorkspace: 1 slot  (~5%)
    //   RemoveWorkspace:         1 slot  (~5%)
    //   SwitchWorkspace:         2 slots (~9%)
    //   AddLinkedWorktree:       4 slots (~18%)
    const DISTRIBUTION_SLOTS: u32 = 22;

    impl TestState {
        fn generate_operation(&self, raw: u32) -> Operation {
            let extra = (raw / DISTRIBUTION_SLOTS) as usize;
            let workspace_count = self.workspace_paths.len();

            match raw % DISTRIBUTION_SLOTS {
                0..=4 => Operation::SaveThread {
                    workspace_index: extra % workspace_count,
                },
                5..=6 if !self.unopened_worktrees.is_empty() => Operation::SaveWorktreeThread {
                    worktree_index: extra % self.unopened_worktrees.len(),
                },
                5..=6 => Operation::SaveThread {
                    workspace_index: extra % workspace_count,
                },
                7..=8 if !self.saved_thread_ids.is_empty() => Operation::DeleteThread {
                    index: extra % self.saved_thread_ids.len(),
                },
                7..=8 => Operation::SaveThread {
                    workspace_index: extra % workspace_count,
                },
                9..=10 => Operation::ToggleAgentPanel,
                11..=12 => Operation::CreateDraftThread,
                13 if !self.unopened_worktrees.is_empty() => Operation::OpenWorktreeAsWorkspace {
                    worktree_index: extra % self.unopened_worktrees.len(),
                },
                13 => Operation::AddWorkspace,
                14 if workspace_count > 1 => Operation::RemoveWorkspace {
                    index: extra % workspace_count,
                },
                14 => Operation::AddWorkspace,
                15..=16 => Operation::SwitchWorkspace {
                    index: extra % workspace_count,
                },
                17..=21 if !self.main_repo_indices.is_empty() => {
                    let main_index = self.main_repo_indices[extra % self.main_repo_indices.len()];
                    Operation::AddLinkedWorktree {
                        workspace_index: main_index,
                    }
                }
                17..=21 => Operation::SaveThread {
                    workspace_index: extra % workspace_count,
                },
                _ => unreachable!(),
            }
        }
    }

    fn save_thread_to_path(
        state: &mut TestState,
        path_list: PathList,
        cx: &mut gpui::VisualTestContext,
    ) {
        let session_id = state.next_thread_id();
        let title: SharedString = format!("Thread {}", session_id).into();
        let updated_at = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2024, 1, 1, 0, 0, 0)
            .unwrap()
            + chrono::Duration::seconds(state.thread_counter as i64);
        save_thread_metadata(session_id, title, updated_at, None, path_list, cx);
    }

    async fn perform_operation(
        operation: Operation,
        state: &mut TestState,
        multi_workspace: &Entity<MultiWorkspace>,
        _sidebar: &Entity<Sidebar>,
        cx: &mut gpui::VisualTestContext,
    ) {
        match operation {
            Operation::SaveThread { workspace_index } => {
                let workspace =
                    multi_workspace.read_with(cx, |mw, _| mw.workspaces()[workspace_index].clone());
                let path_list = workspace
                    .read_with(cx, |workspace, cx| PathList::new(&workspace.root_paths(cx)));
                save_thread_to_path(state, path_list, cx);
            }
            Operation::SaveWorktreeThread { worktree_index } => {
                let worktree = &state.unopened_worktrees[worktree_index];
                let path_list = PathList::new(&[std::path::PathBuf::from(&worktree.path)]);
                save_thread_to_path(state, path_list, cx);
            }
            Operation::DeleteThread { index } => {
                let session_id = state.remove_thread(index);
                cx.update(|_, cx| {
                    ThreadMetadataStore::global(cx)
                        .update(cx, |store, cx| store.delete(session_id, cx));
                });
            }
            Operation::ToggleAgentPanel => {
                let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
                let panel_open =
                    workspace.read_with(cx, |_, cx| AgentPanel::is_visible(&workspace, cx));
                workspace.update_in(cx, |workspace, window, cx| {
                    if panel_open {
                        workspace.close_panel::<AgentPanel>(window, cx);
                    } else {
                        workspace.open_panel::<AgentPanel>(window, cx);
                    }
                });
            }
            Operation::CreateDraftThread => {
                let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
                let panel =
                    workspace.read_with(cx, |workspace, cx| workspace.panel::<AgentPanel>(cx));
                if let Some(panel) = panel {
                    let connection = StubAgentConnection::new();
                    open_thread_with_connection(&panel, connection, cx);
                    cx.run_until_parked();
                }
                workspace.update_in(cx, |workspace, window, cx| {
                    workspace.focus_panel::<AgentPanel>(window, cx);
                });
            }
            Operation::AddWorkspace => {
                let path = state.next_workspace_path();
                state
                    .fs
                    .insert_tree(
                        &path,
                        serde_json::json!({
                            ".git": {},
                            "src": {},
                        }),
                    )
                    .await;
                let project = project::Project::test(
                    state.fs.clone() as Arc<dyn fs::Fs>,
                    [path.as_ref()],
                    cx,
                )
                .await;
                project.update(cx, |p, cx| p.git_scans_complete(cx)).await;
                let workspace = multi_workspace.update_in(cx, |mw, window, cx| {
                    mw.test_add_workspace(project.clone(), window, cx)
                });
                add_agent_panel(&workspace, cx);
                let new_index = state.workspace_paths.len();
                state.workspace_paths.push(path);
                state.main_repo_indices.push(new_index);
            }
            Operation::OpenWorktreeAsWorkspace { worktree_index } => {
                let worktree = state.unopened_worktrees.remove(worktree_index);
                let project = project::Project::test(
                    state.fs.clone() as Arc<dyn fs::Fs>,
                    [worktree.path.as_ref()],
                    cx,
                )
                .await;
                project.update(cx, |p, cx| p.git_scans_complete(cx)).await;
                let workspace = multi_workspace.update_in(cx, |mw, window, cx| {
                    mw.test_add_workspace(project.clone(), window, cx)
                });
                add_agent_panel(&workspace, cx);
                state.workspace_paths.push(worktree.path);
            }
            Operation::RemoveWorkspace { index } => {
                let removed = multi_workspace.update_in(cx, |mw, window, cx| {
                    let workspace = mw.workspaces()[index].clone();
                    mw.remove(&workspace, window, cx)
                });
                if removed {
                    state.workspace_paths.remove(index);
                    state.main_repo_indices.retain(|i| *i != index);
                    for i in &mut state.main_repo_indices {
                        if *i > index {
                            *i -= 1;
                        }
                    }
                }
            }
            Operation::SwitchWorkspace { index } => {
                let workspace =
                    multi_workspace.read_with(cx, |mw, _| mw.workspaces()[index].clone());
                multi_workspace.update_in(cx, |mw, window, cx| {
                    mw.activate(workspace, window, cx);
                });
            }
            Operation::AddLinkedWorktree { workspace_index } => {
                let main_path = state.workspace_paths[workspace_index].clone();
                let dot_git = format!("{}/.git", main_path);
                let worktree_name = state.next_worktree_name();
                let worktree_path = format!("/worktrees/{}", worktree_name);

                state.fs
                    .insert_tree(
                        &worktree_path,
                        serde_json::json!({
                            ".git": format!("gitdir: {}/.git/worktrees/{}", main_path, worktree_name),
                            "src": {},
                        }),
                    )
                    .await;

                // Also create the worktree metadata dir inside the main repo's .git
                state
                    .fs
                    .insert_tree(
                        &format!("{}/.git/worktrees/{}", main_path, worktree_name),
                        serde_json::json!({
                            "commondir": "../../",
                            "HEAD": format!("ref: refs/heads/{}", worktree_name),
                        }),
                    )
                    .await;

                let dot_git_path = std::path::Path::new(&dot_git);
                let worktree_pathbuf = std::path::PathBuf::from(&worktree_path);
                state
                    .fs
                    .with_git_state(dot_git_path, false, |git_state| {
                        git_state.worktrees.push(git::repository::Worktree {
                            path: worktree_pathbuf,
                            ref_name: Some(format!("refs/heads/{}", worktree_name).into()),
                            sha: "aaa".into(),
                        });
                    })
                    .unwrap();

                // Re-scan the main workspace's project so it discovers the new worktree.
                let main_workspace =
                    multi_workspace.read_with(cx, |mw, _| mw.workspaces()[workspace_index].clone());
                let main_project = main_workspace.read_with(cx, |ws, _| ws.project().clone());
                main_project
                    .update(cx, |p, cx| p.git_scans_complete(cx))
                    .await;

                state.unopened_worktrees.push(UnopenedWorktree {
                    path: worktree_path,
                });
            }
        }
    }

    fn update_sidebar(sidebar: &Entity<Sidebar>, cx: &mut gpui::VisualTestContext) {
        sidebar.update_in(cx, |sidebar, _window, cx| {
            sidebar.collapsed_groups.clear();
            let path_lists: Vec<PathList> = sidebar
                .contents
                .entries
                .iter()
                .filter_map(|entry| match entry {
                    ListEntry::ProjectHeader { path_list, .. } => Some(path_list.clone()),
                    _ => None,
                })
                .collect();
            for path_list in path_lists {
                sidebar.expanded_groups.insert(path_list, 10_000);
            }
            sidebar.update_entries(cx);
        });
    }

    fn validate_sidebar_properties(sidebar: &Sidebar, cx: &App) -> anyhow::Result<()> {
        verify_every_workspace_in_multiworkspace_is_shown(sidebar, cx)?;
        verify_all_threads_are_shown(sidebar, cx)?;
        verify_active_state_matches_current_workspace(sidebar, cx)?;
        Ok(())
    }

    fn verify_every_workspace_in_multiworkspace_is_shown(
        sidebar: &Sidebar,
        cx: &App,
    ) -> anyhow::Result<()> {
        let Some(multi_workspace) = sidebar.multi_workspace.upgrade() else {
            anyhow::bail!("sidebar should still have an associated multi-workspace");
        };

        let workspaces = multi_workspace.read(cx).workspaces().to_vec();

        // Workspaces with no root paths are not shown because the
        // sidebar skips empty path lists. All other workspaces should
        // appear — either via a Thread entry or a NewThread entry for
        // threadless workspaces.
        let expected_workspaces: HashSet<EntityId> = workspaces
            .iter()
            .filter(|ws| !workspace_path_list(ws, cx).paths().is_empty())
            .map(|ws| ws.entity_id())
            .collect();

        let sidebar_workspaces: HashSet<EntityId> = sidebar
            .contents
            .entries
            .iter()
            .filter_map(|entry| entry.workspace().map(|ws| ws.entity_id()))
            .collect();

        let missing = &expected_workspaces - &sidebar_workspaces;
        let stray = &sidebar_workspaces - &expected_workspaces;

        anyhow::ensure!(
            missing.is_empty() && stray.is_empty(),
            "sidebar workspaces don't match multi-workspace.\n\
             Only in multi-workspace (missing): {:?}\n\
             Only in sidebar (stray): {:?}",
            missing,
            stray,
        );

        Ok(())
    }

    fn verify_all_threads_are_shown(sidebar: &Sidebar, cx: &App) -> anyhow::Result<()> {
        let Some(multi_workspace) = sidebar.multi_workspace.upgrade() else {
            anyhow::bail!("sidebar should still have an associated multi-workspace");
        };
        let workspaces = multi_workspace.read(cx).workspaces().to_vec();
        let thread_store = ThreadMetadataStore::global(cx);

        let sidebar_thread_ids: HashSet<acp::SessionId> = sidebar
            .contents
            .entries
            .iter()
            .filter_map(|entry| entry.session_id().cloned())
            .collect();

        let mut metadata_thread_ids: HashSet<acp::SessionId> = HashSet::default();
        for workspace in &workspaces {
            let path_list = workspace_path_list(workspace, cx);
            if path_list.paths().is_empty() {
                continue;
            }
            for metadata in thread_store.read(cx).entries_for_path(&path_list) {
                metadata_thread_ids.insert(metadata.session_id.clone());
            }
            for snapshot in root_repository_snapshots(workspace, cx) {
                for linked_worktree in snapshot.linked_worktrees() {
                    let worktree_path_list =
                        PathList::new(std::slice::from_ref(&linked_worktree.path));
                    for metadata in thread_store.read(cx).entries_for_path(&worktree_path_list) {
                        metadata_thread_ids.insert(metadata.session_id.clone());
                    }
                }
            }
        }

        anyhow::ensure!(
            sidebar_thread_ids == metadata_thread_ids,
            "sidebar threads don't match metadata store: sidebar has {:?}, store has {:?}",
            sidebar_thread_ids,
            metadata_thread_ids,
        );
        Ok(())
    }

    fn verify_active_state_matches_current_workspace(
        sidebar: &Sidebar,
        cx: &App,
    ) -> anyhow::Result<()> {
        let Some(multi_workspace) = sidebar.multi_workspace.upgrade() else {
            anyhow::bail!("sidebar should still have an associated multi-workspace");
        };

        let active_workspace = multi_workspace.read(cx).workspace();

        // 1. active_entry must always be Some after rebuild_contents.
        let entry = sidebar
            .active_entry
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("active_entry must always be Some"))?;

        // 2. The entry's workspace must agree with the multi-workspace's
        //    active workspace.
        anyhow::ensure!(
            entry.workspace().entity_id() == active_workspace.entity_id(),
            "active_entry workspace ({:?}) != active workspace ({:?})",
            entry.workspace().entity_id(),
            active_workspace.entity_id(),
        );

        // 3. The entry must match the agent panel's current state.
        let panel = active_workspace.read(cx).panel::<AgentPanel>(cx).unwrap();
        if panel.read(cx).active_thread_is_draft(cx) {
            anyhow::ensure!(
                matches!(entry, ActiveEntry::Draft(_)),
                "panel shows a draft but active_entry is {:?}",
                entry,
            );
        } else if let Some(session_id) = panel
            .read(cx)
            .active_conversation_view()
            .and_then(|cv| cv.read(cx).parent_id(cx))
        {
            anyhow::ensure!(
                matches!(entry, ActiveEntry::Thread { session_id: id, .. } if id == &session_id),
                "panel has session {:?} but active_entry is {:?}",
                session_id,
                entry,
            );
        }

        // 4. Exactly one entry in sidebar contents must be uniquely
        //    identified by the active_entry.
        let matching_count = sidebar
            .contents
            .entries
            .iter()
            .filter(|e| entry.matches_entry(e))
            .count();
        anyhow::ensure!(
            matching_count == 1,
            "expected exactly 1 sidebar entry matching active_entry {:?}, found {}",
            entry,
            matching_count,
        );

        Ok(())
    }

    #[gpui::property_test]
    async fn test_sidebar_invariants(
        #[strategy = gpui::proptest::collection::vec(0u32..DISTRIBUTION_SLOTS * 10, 1..5)]
        raw_operations: Vec<u32>,
        cx: &mut TestAppContext,
    ) {
        agent_ui::test_support::init_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec!["agent-v2".into()]);
            ThreadStore::init_global(cx);
            ThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/my-project",
            serde_json::json!({
                ".git": {},
                "src": {},
            }),
        )
        .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        let project =
            project::Project::test(fs.clone() as Arc<dyn fs::Fs>, ["/my-project".as_ref()], cx)
                .await;
        project.update(cx, |p, cx| p.git_scans_complete(cx)).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let (sidebar, _panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

        let mut state = TestState::new(fs, "/my-project".to_string());
        let mut executed: Vec<String> = Vec::new();

        for &raw_op in &raw_operations {
            let operation = state.generate_operation(raw_op);
            executed.push(format!("{:?}", operation));
            perform_operation(operation, &mut state, &multi_workspace, &sidebar, cx).await;
            cx.run_until_parked();

            update_sidebar(&sidebar, cx);
            cx.run_until_parked();

            let result =
                sidebar.read_with(cx, |sidebar, cx| validate_sidebar_properties(sidebar, cx));
            if let Err(err) = result {
                let log = executed.join("\n  ");
                panic!(
                    "Property violation after step {}:\n{err}\n\nOperations:\n  {log}",
                    executed.len(),
                );
            }
        }
    }
}
