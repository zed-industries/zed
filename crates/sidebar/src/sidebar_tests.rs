use super::*;
use acp_thread::{AcpThread, PermissionOptions, StubAgentConnection};
use agent::ThreadStore;
use agent_ui::{
    ThreadId,
    test_support::{active_session_id, open_thread_with_connection, send_message},
    thread_metadata_store::{ThreadMetadata, WorktreePaths},
};
use chrono::DateTime;
use fs::{FakeFs, Fs};
use gpui::TestAppContext;
use pretty_assertions::assert_eq;
use project::AgentId;
use settings::SettingsStore;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{path_list::PathList, rel_path::rel_path};

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });
}

#[track_caller]
fn assert_active_thread(sidebar: &Sidebar, session_id: &acp::SessionId, msg: &str) {
    let active = sidebar.active_entry.as_ref();
    let matches = active.is_some_and(|entry| {
        // Match by session_id directly on active_entry.
        entry.session_id.as_ref() == Some(session_id)
            // Or match by finding the thread in sidebar entries.
            || sidebar.contents.entries.iter().any(|list_entry| {
                matches!(list_entry, ListEntry::Thread(t)
                    if t.metadata.session_id.as_ref() == Some(session_id)
                        && entry.matches_entry(list_entry))
            })
    });
    assert!(
        matches,
        "{msg}: expected active_entry for session {session_id:?}, got {:?}",
        active,
    );
}

#[track_caller]
fn is_active_session(sidebar: &Sidebar, session_id: &acp::SessionId) -> bool {
    let thread_id = sidebar
        .contents
        .entries
        .iter()
        .find_map(|entry| match entry {
            ListEntry::Thread(t) if t.metadata.session_id.as_ref() == Some(session_id) => {
                Some(t.metadata.thread_id)
            }
            _ => None,
        });
    match thread_id {
        Some(tid) => {
            matches!(&sidebar.active_entry, Some(ActiveEntry { thread_id, .. }) if *thread_id == tid)
        }
        // Thread not in sidebar entries — can't confirm it's active.
        None => false,
    }
}

#[track_caller]
fn assert_active_draft(sidebar: &Sidebar, workspace: &Entity<Workspace>, msg: &str) {
    assert!(
        matches!(&sidebar.active_entry, Some(ActiveEntry { workspace: ws, .. }) if ws == workspace),
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
        .any(|entry| matches!(entry, ListEntry::Thread(t) if t.metadata.session_id.as_ref() == Some(session_id)))
}

#[track_caller]
fn assert_remote_project_integration_sidebar_state(
    sidebar: &mut Sidebar,
    main_thread_id: &acp::SessionId,
    remote_thread_id: &acp::SessionId,
) {
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

    let mut saw_main_thread = false;
    let mut saw_remote_thread = false;
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
                if thread.metadata.session_id.as_ref() == Some(main_thread_id) =>
            {
                saw_main_thread = true;
            }
            ListEntry::Thread(thread)
                if thread.metadata.session_id.as_ref() == Some(remote_thread_id) =>
            {
                saw_remote_thread = true;
            }
            ListEntry::Thread(thread) => {
                let title = thread.metadata.display_title();
                panic!(
                    "unexpected sidebar thread while simulating remote project integration flicker: title=`{}`",
                    title
                );
            }
            ListEntry::ViewMore { .. } => {
                panic!(
                    "unexpected `View More` entry while simulating remote project integration flicker"
                );
            }
        }
    }

    assert!(
        saw_main_thread,
        "expected the sidebar to keep showing `Main Thread` under `project`"
    );
    assert!(
        saw_remote_thread,
        "expected the sidebar to keep showing `Worktree Thread` under `project`"
    );
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
    let sidebar = setup_sidebar_closed(multi_workspace, cx);
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.toggle_sidebar(window, cx);
    });
    cx.run_until_parked();
    sidebar
}

fn setup_sidebar_closed(
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

async fn save_n_test_threads(
    count: u32,
    project: &Entity<project::Project>,
    cx: &mut gpui::VisualTestContext,
) {
    for i in 0..count {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(format!("thread-{}", i))),
            Some(format!("Thread {}", i + 1).into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, i).unwrap(),
            None,
            project,
            cx,
        )
    }
    cx.run_until_parked();
}

async fn save_test_thread_metadata(
    session_id: &acp::SessionId,
    project: &Entity<project::Project>,
    cx: &mut TestAppContext,
) {
    save_thread_metadata(
        session_id.clone(),
        Some("Test".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        project,
        cx,
    )
}

async fn save_named_thread_metadata(
    session_id: &str,
    title: &str,
    project: &Entity<project::Project>,
    cx: &mut gpui::VisualTestContext,
) {
    save_thread_metadata(
        acp::SessionId::new(Arc::from(session_id)),
        Some(SharedString::from(title.to_string())),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        project,
        cx,
    );
    cx.run_until_parked();
}

fn save_thread_metadata(
    session_id: acp::SessionId,
    title: Option<SharedString>,
    updated_at: DateTime<Utc>,
    created_at: Option<DateTime<Utc>>,
    project: &Entity<project::Project>,
    cx: &mut TestAppContext,
) {
    cx.update(|cx| {
        let worktree_paths = project.read(cx).worktree_paths(cx);
        let remote_connection = project.read(cx).remote_connection_options(cx);
        let thread_id = ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
            .unwrap_or_else(ThreadId::new);
        let metadata = ThreadMetadata {
            thread_id,
            session_id: Some(session_id),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title,
            updated_at,
            created_at,
            worktree_paths,
            archived: false,
            remote_connection,
        };
        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.save(metadata, cx));
    });
    cx.run_until_parked();
}

fn save_thread_metadata_with_main_paths(
    session_id: &str,
    title: &str,
    folder_paths: PathList,
    main_worktree_paths: PathList,
    updated_at: DateTime<Utc>,
    cx: &mut TestAppContext,
) {
    let session_id = acp::SessionId::new(Arc::from(session_id));
    let title = SharedString::from(title.to_string());
    let thread_id = cx.update(|cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
            .unwrap_or_else(ThreadId::new)
    });
    let metadata = ThreadMetadata {
        thread_id,
        session_id: Some(session_id),
        agent_id: agent::ZED_AGENT_ID.clone(),
        title: Some(title),
        updated_at,
        created_at: None,
        worktree_paths: WorktreePaths::from_path_lists(main_worktree_paths, folder_paths).unwrap(),
        archived: false,
        remote_connection: None,
    };
    cx.update(|cx| {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.save(metadata, cx));
    });
    cx.run_until_parked();
}

fn focus_sidebar(sidebar: &Entity<Sidebar>, cx: &mut gpui::VisualTestContext) {
    sidebar.update_in(cx, |_, window, cx| {
        cx.focus_self(window);
    });
    cx.run_until_parked();
}

fn request_test_tool_authorization(
    thread: &Entity<AcpThread>,
    tool_call_id: &str,
    option_id: &str,
    cx: &mut gpui::VisualTestContext,
) {
    let tool_call_id = acp::ToolCallId::new(tool_call_id);
    let label = format!("Tool {tool_call_id}");
    let option_id = acp::PermissionOptionId::new(option_id);
    let _authorization_task = cx.update(|_, cx| {
        thread.update(cx, |thread, cx| {
            thread
                .request_tool_call_authorization(
                    acp::ToolCall::new(tool_call_id, label)
                        .kind(acp::ToolKind::Edit)
                        .into(),
                    PermissionOptions::Flat(vec![acp::PermissionOption::new(
                        option_id,
                        "Allow",
                        acp::PermissionOptionKind::AllowOnce,
                    )]),
                    cx,
                )
                .unwrap()
        })
    });
    cx.run_until_parked();
}

fn format_linked_worktree_chips(worktrees: &[ThreadItemWorktreeInfo]) -> String {
    let mut seen = Vec::new();
    let mut chips = Vec::new();
    for wt in worktrees {
        if wt.kind == ui::WorktreeKind::Main {
            continue;
        }
        if !seen.contains(&wt.name) {
            seen.push(wt.name.clone());
            chips.push(format!("{{{}}}", wt.name));
        }
    }
    if chips.is_empty() {
        String::new()
    } else {
        format!(" {}", chips.join(", "))
    }
}

fn visible_entries_as_strings(
    sidebar: &Entity<Sidebar>,
    cx: &mut gpui::VisualTestContext,
) -> Vec<String> {
    sidebar.read_with(cx, |sidebar, cx| {
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
                        key,
                        highlight_positions: _,
                        ..
                    } => {
                        let icon = if sidebar.is_group_collapsed(key, cx) {
                            ">"
                        } else {
                            "v"
                        };
                        format!("{} [{}]{}", icon, label, selected)
                    }
                    ListEntry::Thread(thread) => {
                        let title = thread.metadata.display_title();
                        let worktree = format_linked_worktree_chips(&thread.worktrees);

                        {
                            let live = if thread.is_live { " *" } else { "" };
                            let status_str = match thread.status {
                                AgentThreadStatus::Running => " (running)",
                                AgentThreadStatus::Error => " (error)",
                                AgentThreadStatus::WaitingForConfirmation => " (waiting)",
                                _ => "",
                            };
                            let notified = if sidebar
                                .contents
                                .is_thread_notified(&thread.metadata.thread_id)
                            {
                                " (!)"
                            } else {
                                ""
                            };
                            format!("  {title}{worktree}{live}{status_str}{notified}{selected}")
                        }
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
                }
            })
            .collect()
    })
}

#[gpui::test]
async fn test_serialization_round_trip(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(3, &project, cx).await;

    let project_group_key = project.read_with(cx, |project, cx| project.project_group_key(cx));

    // Set a custom width, collapse the group, and expand "View More".
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.set_width(Some(px(420.0)), cx);
        sidebar.toggle_collapse(&project_group_key, window, cx);
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
    let width1 = sidebar.read_with(cx, |s, _| s.width);
    let width2 = sidebar2.read_with(cx, |s, _| s.width);

    assert_eq!(width1, width2);
    assert_eq!(width1, px(420.0));
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
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let (_sidebar, _panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    assert_eq!(
        visible_entries_as_strings(&_sidebar, cx),
        vec!["v [my-project]"]
    );
}

#[gpui::test]
async fn test_single_workspace_with_saved_threads(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-1")),
        Some("Fix crash in project panel".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 3, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-2")),
        Some("Add inline diff view".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Single workspace with a thread
    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-a1")),
        Some("Thread A1".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project-a]",
            "  Thread A1",
        ]
    );

    // Add a second workspace
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.create_test_workspace(window, cx).detach();
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project-a]",
            "  Thread A1",
        ]
    );
}

#[gpui::test]
async fn test_view_more_pagination(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(12, &project, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Create 17 threads: initially shows 5, then 10, then 15, then all 17 with Collapse
    save_n_test_threads(17, &project, cx).await;

    let project_group_key = project.read_with(cx, |project, cx| project.project_group_key(cx));

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Initially shows 5 threads + View More
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 7); // header + 5 threads + View More
    assert!(entries.iter().any(|e| e.contains("View More")));

    // Focus and navigate to View More, then confirm to expand by one batch
    focus_sidebar(&sidebar, cx);
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
        s.expand_thread_group(&project_group_key, cx);
    });
    cx.run_until_parked();

    // Now shows 15 threads + View More
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 17); // header + 15 threads + View More
    assert!(entries.iter().any(|e| e.contains("View More")));

    // Expand one more time - should show all 17 threads with Collapse button
    sidebar.update_in(cx, |s, _window, cx| {
        s.expand_thread_group(&project_group_key, cx);
    });
    cx.run_until_parked();

    // All 17 threads shown with Collapse button
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 19); // header + 17 threads + Collapse
    assert!(!entries.iter().any(|e| e.contains("View More")));
    assert!(entries.iter().any(|e| e.contains("Collapse")));

    // Click collapse - should go back to showing 5 threads
    sidebar.update_in(cx, |s, _window, cx| {
        s.reset_thread_group_expansion(&project_group_key, cx);
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(1, &project, cx).await;

    let project_group_key = project.read_with(cx, |project, cx| project.project_group_key(cx));

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Thread 1",
        ]
    );

    // Collapse
    sidebar.update_in(cx, |s, window, cx| {
        s.toggle_collapse(&project_group_key, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "> [my-project]",
        ]
    );

    // Expand
    sidebar.update_in(cx, |s, window, cx| {
        s.toggle_collapse(&project_group_key, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Thread 1",
        ]
    );
}

#[gpui::test]
async fn test_collapse_state_survives_worktree_key_change(cx: &mut TestAppContext) {
    // When a worktree is added to a project, the project group key changes.
    // The sidebar's collapsed/expanded state is keyed by ProjectGroupKey, so
    // UI state must survive the key change.
    let (_fs, project) = init_multi_project_test(&["/project-a", "/project-b"], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(2, &project, cx).await;
    sidebar.update_in(cx, |sidebar, _window, cx| sidebar.update_entries(cx));
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Thread 2", "  Thread 1",]
    );

    // Collapse the group.
    let old_key = project.read_with(cx, |project, cx| project.project_group_key(cx));
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.toggle_collapse(&old_key, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["> [project-a]"]
    );

    // Add a second worktree — the key changes from [/project-a] to
    // [/project-a, /project-b].
    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/project-b", true, cx)
        })
        .await
        .expect("should add worktree");
    cx.run_until_parked();

    sidebar.update_in(cx, |sidebar, _window, cx| sidebar.update_entries(cx));
    cx.run_until_parked();

    // The group should still be collapsed under the new key.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["> [project-a, project-b]"]
    );
}

#[gpui::test]
async fn test_visible_entries_as_strings(cx: &mut TestAppContext) {
    use workspace::ProjectGroup;

    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    let expanded_path = PathList::new(&[std::path::PathBuf::from("/expanded")]);
    let collapsed_path = PathList::new(&[std::path::PathBuf::from("/collapsed")]);

    // Set the collapsed group state through multi_workspace
    multi_workspace.update(cx, |mw, _cx| {
        mw.test_add_project_group(ProjectGroup {
            key: ProjectGroupKey::new(None, collapsed_path.clone()),
            workspaces: Vec::new(),
            expanded: false,
            visible_thread_count: None,
        });
    });

    sidebar.update_in(cx, |s, _window, _cx| {
        let notified_thread_id = ThreadId::new();
        s.contents.notified_threads.insert(notified_thread_id);
        s.contents.entries = vec![
            // Expanded project header
            ListEntry::ProjectHeader {
                key: ProjectGroupKey::new(None, expanded_path.clone()),
                label: "expanded-project".into(),
                highlight_positions: Vec::new(),
                has_running_threads: false,
                waiting_thread_count: 0,
                is_active: true,
                has_threads: true,
            },
            ListEntry::Thread(ThreadEntry {
                metadata: ThreadMetadata {
                    thread_id: ThreadId::new(),
                    session_id: Some(acp::SessionId::new(Arc::from("t-1"))),
                    agent_id: AgentId::new("zed-agent"),
                    worktree_paths: WorktreePaths::default(),
                    title: Some("Completed thread".into()),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                    remote_connection: None,
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
                    thread_id: ThreadId::new(),
                    session_id: Some(acp::SessionId::new(Arc::from("t-2"))),
                    agent_id: AgentId::new("zed-agent"),
                    worktree_paths: WorktreePaths::default(),
                    title: Some("Running thread".into()),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                    remote_connection: None,
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
                    thread_id: ThreadId::new(),
                    session_id: Some(acp::SessionId::new(Arc::from("t-3"))),
                    agent_id: AgentId::new("zed-agent"),
                    worktree_paths: WorktreePaths::default(),
                    title: Some("Error thread".into()),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                    remote_connection: None,
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
            // remote_connection: None,
            ListEntry::Thread(ThreadEntry {
                metadata: ThreadMetadata {
                    thread_id: ThreadId::new(),
                    session_id: Some(acp::SessionId::new(Arc::from("t-4"))),
                    agent_id: AgentId::new("zed-agent"),
                    worktree_paths: WorktreePaths::default(),
                    title: Some("Waiting thread".into()),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                    remote_connection: None,
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
            // remote_connection: None,
            ListEntry::Thread(ThreadEntry {
                metadata: ThreadMetadata {
                    thread_id: notified_thread_id,
                    session_id: Some(acp::SessionId::new(Arc::from("t-5"))),
                    agent_id: AgentId::new("zed-agent"),
                    worktree_paths: WorktreePaths::default(),
                    title: Some("Notified thread".into()),
                    updated_at: Utc::now(),
                    created_at: Some(Utc::now()),
                    archived: false,
                    remote_connection: None,
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
                key: ProjectGroupKey::new(None, expanded_path.clone()),
                is_fully_expanded: false,
            },
            // Collapsed project header
            ListEntry::ProjectHeader {
                key: ProjectGroupKey::new(None, collapsed_path.clone()),
                label: "collapsed-project".into(),
                highlight_positions: Vec::new(),
                has_running_threads: false,
                waiting_thread_count: 0,
                is_active: false,
                has_threads: false,
            },
        ];

        // Select the Running thread (index 2)
        s.selection = Some(2);
    });

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(3, &project, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Entries: [header, thread3, thread2, thread1]
    // Focusing the sidebar does not set a selection; select_next/select_previous
    // handle None gracefully by starting from the first or last entry.
    focus_sidebar(&sidebar, cx);
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(3, &project, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    focus_sidebar(&sidebar, cx);

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
    focus_sidebar(&sidebar, cx);
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(1, &project, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Thread 1",
        ]
    );

    // Focus the sidebar and select the header
    focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(0);
    });

    // Confirm on project header collapses the group
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "> [my-project]  <== selected",
        ]
    );

    // Confirm again expands the group
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]  <== selected",
            "  Thread 1",
        ]
    );
}

#[gpui::test]
async fn test_keyboard_confirm_on_view_more_expands(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(8, &project, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Should show header + 5 threads + "View More"
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries.len(), 7);
    assert!(entries.iter().any(|e| e.contains("View More")));

    // Focus sidebar (selection starts at None), then navigate down to the "View More" entry (index 6)
    focus_sidebar(&sidebar, cx);
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(1, &project, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Thread 1",
        ]
    );

    // Focus sidebar and manually select the header (index 0). Press left to collapse.
    focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(0);
    });

    cx.dispatch_action(SelectParent);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "> [my-project]  <== selected",
        ]
    );

    // Press right to expand
    cx.dispatch_action(SelectChild);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]  <== selected",
            "  Thread 1",
        ]
    );

    // Press right again on already-expanded header moves selection down
    cx.dispatch_action(SelectChild);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));
}

#[gpui::test]
async fn test_keyboard_collapse_from_child_selects_parent(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(1, &project, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Focus sidebar (selection starts at None), then navigate down to the thread (child)
    focus_sidebar(&sidebar, cx);
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Thread 1  <== selected",
        ]
    );

    // Pressing left on a child collapses the parent group and selects it
    cx.dispatch_action(SelectParent);
    cx.run_until_parked();

    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "> [my-project]  <== selected",
        ]
    );
}

#[gpui::test]
async fn test_keyboard_navigation_on_empty_list(cx: &mut TestAppContext) {
    let project = init_test_project_with_agent_panel("/empty-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let (sidebar, _panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    // An empty project has only the header (no auto-created draft).
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [empty-project]"]
    );

    // Focus sidebar — focus_in does not set a selection
    focus_sidebar(&sidebar, cx);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

    // First SelectNext from None starts at index 0 (header)
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

    // SelectNext with only one entry stays at index 0
    cx.dispatch_action(SelectNext);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

    // SelectPrevious from first entry clears selection (returns to editor)
    cx.dispatch_action(SelectPrevious);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

    // SelectPrevious from None selects the last entry
    cx.dispatch_action(SelectPrevious);
    assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
}

#[gpui::test]
async fn test_selection_clamps_after_entry_removal(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_n_test_threads(1, &project, cx).await;
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Focus sidebar (selection starts at None), navigate down to the thread (index 1)
    focus_sidebar(&sidebar, cx);
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
        cx.set_global(agent_ui::MaxIdleRetainedThreads(1));
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

    // Open thread A and keep it generating.
    let connection = StubAgentConnection::new();
    open_thread_with_connection(&panel, connection.clone(), cx);
    send_message(&panel, cx);

    let session_id_a = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id_a, &project, cx).await;

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
    save_test_thread_metadata(&session_id_b, &project, cx).await;

    cx.run_until_parked();

    let mut entries = visible_entries_as_strings(&sidebar, cx);
    entries[1..].sort();
    assert_eq!(
        entries,
        vec![
            //
            "v [my-project]",
            "  Hello *",
            "  Hello * (running)",
        ]
    );
}

#[gpui::test]
async fn test_subagent_permission_request_marks_parent_sidebar_thread_waiting(
    cx: &mut TestAppContext,
) {
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let connection = StubAgentConnection::new().with_supports_load_session(true);
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);

    let parent_session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&parent_session_id, &project, cx).await;

    let subagent_session_id = acp::SessionId::new("subagent-session");
    cx.update(|_, cx| {
        let parent_thread = panel.read(cx).active_agent_thread(cx).unwrap();
        parent_thread.update(cx, |thread: &mut AcpThread, cx| {
            thread.subagent_spawned(subagent_session_id.clone(), cx);
        });
    });
    cx.run_until_parked();

    let subagent_thread = panel.read_with(cx, |panel, cx| {
        panel
            .active_conversation_view()
            .and_then(|conversation| conversation.read(cx).thread_view(&subagent_session_id, cx))
            .map(|thread_view| thread_view.read(cx).thread.clone())
            .expect("Expected subagent thread to be loaded into the conversation")
    });
    request_test_tool_authorization(&subagent_thread, "subagent-tool-call", "allow-subagent", cx);

    let parent_status = sidebar.read_with(cx, |sidebar, _cx| {
        sidebar
            .contents
            .entries
            .iter()
            .find_map(|entry| match entry {
                ListEntry::Thread(thread)
                    if thread.metadata.session_id.as_ref() == Some(&parent_session_id) =>
                {
                    Some(thread.status)
                }
                _ => None,
            })
            .expect("Expected parent thread entry in sidebar")
    });

    assert_eq!(parent_status, AgentThreadStatus::WaitingForConfirmation);
}

#[gpui::test]
async fn test_background_thread_completion_triggers_notification(cx: &mut TestAppContext) {
    let project_a = init_test_project_with_agent_panel("/project-a", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    // Open thread on workspace A and keep it generating.
    let connection_a = StubAgentConnection::new();
    open_thread_with_connection(&panel_a, connection_a.clone(), cx);
    send_message(&panel_a, cx);

    let session_id_a = active_session_id(&panel_a, cx);
    save_test_thread_metadata(&session_id_a, &project_a, cx).await;

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
        vec![
            //
            "v [project-a]",
            "  Hello * (running)",
        ]
    );

    // Complete thread A's turn (transition Running → Completed).
    connection_a.end_turn(session_id_a.clone(), acp::StopReason::EndTurn);
    cx.run_until_parked();

    // The completed background thread shows a notification indicator.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project-a]",
            "  Hello * (!)",
        ]
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    for (id, title, hour) in [
        ("t-1", "Fix crash in project panel", 3),
        ("t-2", "Add inline diff view", 2),
        ("t-3", "Refactor settings module", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            Some(title.into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            &project,
            cx,
        );
    }
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
        vec![
            //
            "v [my-project]",
            "  Add inline diff view  <== selected",
        ]
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-1")),
        Some("Fix Crash In Project Panel".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );
    cx.run_until_parked();

    // Lowercase query matches mixed-case title.
    type_in_search(&sidebar, "fix crash", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Fix Crash In Project Panel  <== selected",
        ]
    );

    // Uppercase query also matches the same title.
    type_in_search(&sidebar, "FIX CRASH", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    for (id, title, hour) in [("t-1", "Alpha thread", 2), ("t-2", "Beta thread", 1)] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            Some(title.into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            &project,
            cx,
        )
    }
    cx.run_until_parked();

    // Confirm the full list is showing.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Alpha thread",
            "  Beta thread",
        ]
    );

    // User types a search query to filter down.
    focus_sidebar(&sidebar, cx);
    type_in_search(&sidebar, "alpha", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Alpha thread  <== selected",
        ]
    );

    // User presses Escape — filter clears, full list is restored.
    // The selection index (1) now points at the first thread entry.
    cx.dispatch_action(Cancel);
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    for (id, title, hour) in [
        ("a1", "Fix bug in sidebar", 2),
        ("a2", "Add tests for editor", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            Some(title.into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            &project_a,
            cx,
        )
    }

    // Add a second workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.create_test_workspace(window, cx).detach();
    });
    cx.run_until_parked();

    let project_b = multi_workspace.read_with(cx, |mw, cx| {
        mw.workspaces().nth(1).unwrap().read(cx).project().clone()
    });

    for (id, title, hour) in [
        ("b1", "Refactor sidebar layout", 3),
        ("b2", "Fix typo in README", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            Some(title.into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            &project_b,
            cx,
        )
    }
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project-a]",
            "  Fix bug in sidebar",
            "  Add tests for editor",
        ]
    );

    // "sidebar" matches a thread in each workspace — both headers stay visible.
    type_in_search(&sidebar, "sidebar", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project-a]",
            "  Fix bug in sidebar  <== selected",
        ]
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
            //
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    for (id, title, hour) in [
        ("a1", "Fix bug in sidebar", 2),
        ("a2", "Add tests for editor", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            Some(title.into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            &project_a,
            cx,
        )
    }

    // Add a second workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.create_test_workspace(window, cx).detach();
    });
    cx.run_until_parked();

    let project_b = multi_workspace.read_with(cx, |mw, cx| {
        mw.workspaces().nth(1).unwrap().read(cx).project().clone()
    });

    for (id, title, hour) in [
        ("b1", "Refactor sidebar layout", 3),
        ("b2", "Fix typo in README", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            Some(title.into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            &project_b,
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
            //
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
        vec![
            //
            "v [alpha-project]",
            "  Fix bug in sidebar  <== selected",
        ]
    );

    // "alpha sidebar" matches the workspace name "alpha-project" (fuzzy: a-l-p-h-a-s-i-d-e-b-a-r
    // doesn't match) — but does not match either workspace name or any thread.
    // Actually let's test something simpler: a query that matches both a workspace
    // name AND some threads in that workspace. Matching threads should still appear.
    type_in_search(&sidebar, "fix", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [alpha-project]",
            "  Fix bug in sidebar  <== selected",
        ]
    );

    // A query that matches a workspace name AND a thread in that same workspace.
    // Both the header (highlighted) and all child threads should appear.
    type_in_search(&sidebar, "alpha", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
            //
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

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
            Some(title.into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, i).unwrap(),
            None,
            &project,
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
        vec![
            //
            "v [my-project]",
            "  Hidden gem thread  <== selected",
        ]
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-1")),
        Some("Important thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );
    cx.run_until_parked();

    // User focuses the sidebar and collapses the group using keyboard:
    // manually select the header, then press SelectParent to collapse.
    focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(0);
    });
    cx.dispatch_action(SelectParent);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "> [my-project]  <== selected",
        ]
    );

    // User types a search — the thread appears even though its group is collapsed.
    type_in_search(&sidebar, "important", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "> [my-project]",
            "  Important thread  <== selected",
        ]
    );
}

#[gpui::test]
async fn test_search_then_keyboard_navigate_and_confirm(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    for (id, title, hour) in [
        ("t-1", "Fix crash in panel", 3),
        ("t-2", "Fix lint warnings", 2),
        ("t-3", "Add new feature", 1),
    ] {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(id)),
            Some(title.into()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
            None,
            &project,
            cx,
        )
    }
    cx.run_until_parked();

    focus_sidebar(&sidebar, cx);

    // User types "fix" — two threads match.
    type_in_search(&sidebar, "fix", cx);
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
            //
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
            //
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.create_test_workspace(window, cx).detach();
    });
    cx.run_until_parked();

    let (workspace_0, workspace_1) = multi_workspace.read_with(cx, |mw, _| {
        (
            mw.workspaces().next().unwrap().clone(),
            mw.workspaces().nth(1).unwrap().clone(),
        )
    });

    save_thread_metadata(
        acp::SessionId::new(Arc::from("hist-1")),
        Some("Historical Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );
    cx.run_until_parked();
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Historical Thread",
        ]
    );

    // Switch to workspace 1 so we can verify the confirm switches back.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().nth(1).unwrap().clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_1
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
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_0
    );
}

#[gpui::test]
async fn test_confirm_on_historical_thread_preserves_historical_timestamp_and_order(
    cx: &mut TestAppContext,
) {
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, _panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let newer_session_id = acp::SessionId::new(Arc::from("newer-historical-thread"));
    let newer_timestamp = chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 2, 0, 0, 0).unwrap();
    save_thread_metadata(
        newer_session_id,
        Some("Newer Historical Thread".into()),
        newer_timestamp,
        Some(newer_timestamp),
        &project,
        cx,
    );

    let older_session_id = acp::SessionId::new(Arc::from("older-historical-thread"));
    let older_timestamp = chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap();
    save_thread_metadata(
        older_session_id.clone(),
        Some("Older Historical Thread".into()),
        older_timestamp,
        Some(older_timestamp),
        &project,
        cx,
    );

    cx.run_until_parked();
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    let historical_entries_before: Vec<_> = visible_entries_as_strings(&sidebar, cx)
        .into_iter()
        .filter(|entry| entry.contains("Historical Thread"))
        .collect();
    assert_eq!(
        historical_entries_before,
        vec![
            "  Newer Historical Thread".to_string(),
            "  Older Historical Thread".to_string(),
        ],
        "expected the sidebar to sort historical threads by their saved timestamp before activation"
    );

    let older_entry_index = sidebar.read_with(cx, |sidebar, _cx| {
        sidebar
            .contents
            .entries
            .iter()
            .position(|entry| {
                matches!(entry, ListEntry::Thread(thread)
                    if thread.metadata.session_id.as_ref() == Some(&older_session_id))
            })
            .expect("expected Older Historical Thread to appear in the sidebar")
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.selection = Some(older_entry_index);
        sidebar.confirm(&Confirm, window, cx);
    });
    cx.run_until_parked();

    let older_metadata = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry_by_session(&older_session_id)
            .cloned()
            .expect("expected metadata for Older Historical Thread after activation")
    });
    assert_eq!(
        older_metadata.created_at,
        Some(older_timestamp),
        "activating a historical thread should not rewrite its saved created_at timestamp"
    );

    let historical_entries_after: Vec<_> = visible_entries_as_strings(&sidebar, cx)
        .into_iter()
        .filter(|entry| entry.contains("Historical Thread"))
        .collect();
    assert_eq!(
        historical_entries_after,
        vec![
            "  Newer Historical Thread".to_string(),
            "  Older Historical Thread  <== selected".to_string(),
        ],
        "activating an older historical thread should not reorder it ahead of a newer historical thread"
    );
}

#[gpui::test]
async fn test_confirm_on_historical_thread_in_new_project_group_opens_real_thread(
    cx: &mut TestAppContext,
) {
    use workspace::ProjectGroup;

    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.set_global(agent_ui::MaxIdleRetainedThreads(1));
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let project_b_key = project_b.read_with(cx, |project, cx| project.project_group_key(cx));
    multi_workspace.update(cx, |mw, _cx| {
        mw.test_add_project_group(ProjectGroup {
            key: project_b_key.clone(),
            workspaces: Vec::new(),
            expanded: true,
            visible_thread_count: None,
        });
    });

    let session_id = acp::SessionId::new(Arc::from("historical-new-project-group"));
    save_thread_metadata(
        session_id.clone(),
        Some("Historical Thread in New Group".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap(),
        None,
        &project_b,
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    let entries_before = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(
        entries_before,
        vec![
            "v [project-a]",
            "v [project-b]",
            "  Historical Thread in New Group",
        ],
        "expected the closed project group to show the historical thread before first open"
    );

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "should start without an open workspace for the new project group"
    );

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.selection = Some(2);
        sidebar.confirm(&Confirm, window, cx);
    });

    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2,
        "confirming the historical thread should open a workspace for the new project group"
    );

    let workspace_b = multi_workspace.read_with(cx, |mw, cx| {
        mw.workspaces()
            .find(|workspace| {
                PathList::new(&workspace.read(cx).root_paths(cx))
                    == project_b_key.path_list().clone()
            })
            .cloned()
            .expect("expected workspace for project-b after opening the historical thread")
    });

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_b,
        "opening the historical thread should activate the new project's workspace"
    );

    let panel = workspace_b.read_with(cx, |workspace, cx| {
        workspace
            .panel::<AgentPanel>(cx)
            .expect("expected first-open activation to bootstrap the agent panel")
    });

    let expected_thread_id = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
            .expect("metadata should still map session id to thread id")
    });

    assert_eq!(
        panel.read_with(cx, |panel, cx| panel.active_thread_id(cx)),
        Some(expected_thread_id),
        "expected the agent panel to activate the real historical thread rather than a draft"
    );

    let entries_after = visible_entries_as_strings(&sidebar, cx);
    let matching_rows: Vec<_> = entries_after
        .iter()
        .filter(|entry| entry.contains("Historical Thread in New Group") || entry.contains("Draft"))
        .cloned()
        .collect();
    assert_eq!(
        matching_rows.len(),
        1,
        "expected only one matching row after first open into a new project group, got entries: {entries_after:?}"
    );
    assert!(
        matching_rows[0].contains("Historical Thread in New Group"),
        "expected the surviving row to be the real historical thread, got entries: {entries_after:?}"
    );
    assert!(
        !matching_rows[0].contains("Draft"),
        "expected no draft row after first open into a new project group, got entries: {entries_after:?}"
    );
}

#[gpui::test]
async fn test_click_clears_selection_and_focus_in_restores_it(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("t-1")),
        Some("Thread A".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );

    save_thread_metadata(
        acp::SessionId::new(Arc::from("t-2")),
        Some("Thread B".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );

    cx.run_until_parked();
    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Thread A",
            "  Thread B",
        ]
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
        let project_group_key = ProjectGroupKey::new(None, path_list);
        sidebar.toggle_collapse(&project_group_key, window, cx);
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

    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Hi there!".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);

    let session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id, &project, cx).await;
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Hello *",
        ]
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
        vec![
            //
            "v [my-project]",
            "  Friendly Greeting with AI *",
        ]
    );
}

#[gpui::test]
async fn test_focused_thread_tracks_user_intent(cx: &mut TestAppContext) {
    let project_a = init_test_project_with_agent_panel("/project-a", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    // Save a thread so it appears in the list.
    let connection_a = StubAgentConnection::new();
    connection_a.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel_a, connection_a, cx);
    send_message(&panel_a, cx);
    let session_id_a = active_session_id(&panel_a, cx);
    save_test_thread_metadata(&session_id_a, &project_a, cx).await;

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

    let workspace_a =
        multi_workspace.read_with(cx, |mw, _cx| mw.workspaces().next().unwrap().clone());

    // ── 1. Initial state: focused thread derived from active panel ─────
    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id_a,
            "The active panel's thread should be focused on startup",
        );
    });

    let thread_metadata_a = cx.update(|_window, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry_by_session(&session_id_a)
            .cloned()
            .expect("session_id_a should exist in metadata store")
    });
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_thread(thread_metadata_a, &workspace_a, false, window, cx);
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
        let dock = workspace.left_dock().read(cx);
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
    save_test_thread_metadata(&session_id_b, &project_b, cx).await;
    cx.run_until_parked();

    // Workspace A is currently active. Click a thread in workspace B,
    // which also triggers a workspace switch.
    let thread_metadata_b = cx.update(|_window, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry_by_session(&session_id_b)
            .cloned()
            .expect("session_id_b should exist in metadata store")
    });
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_thread(thread_metadata_b, &workspace_b, false, window, cx);
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
        let workspace = mw.workspaces().next().unwrap().clone();
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
    save_test_thread_metadata(&session_id_b2, &project_b, cx).await;
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
        let workspace = mw.workspaces().find(|w| *w == &workspace_b).cloned();
        if let Some(workspace) = workspace {
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

    // Start a thread and send a message so it has history.
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);
    let session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id, &project, cx).await;
    cx.run_until_parked();

    // Verify the thread appears in the sidebar.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project-a]",
            "  Hello *",
        ]
    );

    // The "New Thread" button should NOT be in "active/draft" state
    // because the panel has a thread with messages.
    sidebar.read_with(cx, |sidebar, _cx| {
        assert!(
            matches!(&sidebar.active_entry, Some(ActiveEntry { .. })),
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
    // project subscription. The old [project-a] key is replaced by the new
    // key since no other workspace claims it.
    let entries = visible_entries_as_strings(&sidebar, cx);
    // After adding a worktree, the thread migrates to the new group key.
    // A reconciliation draft may appear during the transition.
    assert!(
        entries.contains(&"  Hello *".to_string()),
        "thread should still be present after adding folder: {entries:?}"
    );
    assert_eq!(entries[0], "v [project-a, project-b]");

    // The "New Thread" button must still be clickable (not stuck in
    // "active/draft" state). Verify that `active_thread_is_draft` is
    // false — the panel still has the old thread with messages.
    sidebar.read_with(cx, |sidebar, _cx| {
        assert!(
            matches!(&sidebar.active_entry, Some(ActiveEntry { .. })),
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
    // non-empty thread, the panel should switch to the draft thread.
    // Drafts are not shown as sidebar rows.
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    // Create a non-empty thread (has messages).
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);

    let session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id, &project, cx).await;
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [my-project]",
            "  Hello *",
        ]
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

    // Drafts are not shown as sidebar rows, so entries stay the same.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [my-project]", "  Hello *"],
        "After Cmd-N the sidebar should not show a Draft entry"
    );

    // The panel should be on the draft and active_entry should track it.
    panel.read_with(cx, |panel, cx| {
        assert!(
            panel.active_thread_is_draft(cx),
            "panel should be showing the draft after Cmd-N",
        );
    });
    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_draft(
            sidebar,
            &workspace,
            "active_entry should be Draft after Cmd-N",
        );
    });
}

#[gpui::test]
async fn test_cmd_n_shows_new_thread_entry_in_absorbed_worktree(cx: &mut TestAppContext) {
    // When the active workspace is an absorbed git worktree, cmd-n
    // should activate the draft thread in the panel. Drafts are not
    // shown as sidebar rows.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
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
            ".git": {},
            "src": {},
        }),
    )
    .await;

    // Worktree checkout pointing back to the main repo.
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    // Switch to the worktree workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().nth(1).unwrap().clone();
        mw.activate(workspace, window, cx);
    });

    // Create a non-empty thread in the worktree workspace.
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&worktree_panel, connection, cx);
    send_message(&worktree_panel, cx);

    let session_id = active_session_id(&worktree_panel, cx);
    save_test_thread_metadata(&session_id, &worktree_project, cx).await;
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project]",
            "  Hello {wt-feature-a} *",
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

    // Drafts are not shown as sidebar rows, so entries stay the same.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project]",
            "  Hello {wt-feature-a} *"
        ],
        "After Cmd-N the sidebar should not show a Draft entry"
    );

    // The panel should be on the draft and active_entry should track it.
    worktree_panel.read_with(cx, |panel, cx| {
        assert!(
            panel.active_thread_is_draft(cx),
            "panel should be showing the draft after Cmd-N",
        );
    });
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
        .add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            false,
            git::repository::Worktree {
                path: std::path::PathBuf::from("/wt/rosewood"),
                ref_name: Some("refs/heads/rosewood".into()),
                sha: "abc".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;

    let worktree_project = project::Project::test(fs.clone(), ["/wt/rosewood".as_ref()], cx).await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_named_thread_metadata("main-t", "Unrelated Thread", &project, cx).await;
    save_named_thread_metadata("wt-t", "Fix Bug", &worktree_project, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Search for "rosewood" — should match the worktree name, not the title.
    type_in_search(&sidebar, "rosewood", cx);

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project]",
            "  Fix Bug {rosewood}  <== selected",
        ],
    );
}

#[gpui::test]
async fn test_git_worktree_added_live_updates_sidebar(cx: &mut TestAppContext) {
    let (project, fs) = init_test_project_with_git("/project", cx).await;

    project
        .update(cx, |project, cx| project.git_scans_complete(cx))
        .await;

    let worktree_project = project::Project::test(fs.clone(), ["/wt/rosewood".as_ref()], cx).await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread against a worktree path with the correct main
    // worktree association (as if the git state had been resolved).
    save_thread_metadata_with_main_paths(
        "wt-thread",
        "Worktree Thread",
        PathList::new(&[PathBuf::from("/wt/rosewood")]),
        PathList::new(&[PathBuf::from("/project")]),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        cx,
    );

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Thread is visible because its main_worktree_paths match the group.
    // The chip name is derived from the path even before git discovery.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project]", "  Worktree Thread {rosewood}"]
    );

    // Now add the worktree to the git state and trigger a rescan.
    fs.as_fake()
        .add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            git::repository::Worktree {
                path: std::path::PathBuf::from("/wt/rosewood"),
                ref_name: Some("refs/heads/rosewood".into()),
                sha: "abc".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project]",
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
            },
            "src": {},
        }),
    )
    .await;

    // Two worktree checkouts whose .git files point back to the main repo.
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-b"),
            ref_name: Some("refs/heads/feature-b".into()),
            sha: "bbb".into(),
            is_main: false,
            is_bare: false,
        },
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

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-a")),
        Some("Thread A".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project_a,
        cx,
    );
    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-b")),
        Some("Thread B".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 1).unwrap(),
        None,
        &project_b,
        cx,
    );

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Without the main repo, each worktree has its own header.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project]",
            "  Thread B {wt-feature-b}",
            "  Thread A {wt-feature-a}",
        ]
    );

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
            //
            "v [project]",
            "  Thread B {wt-feature-b}",
            "  Thread A {wt-feature-a}",
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
            ".git": {},
            "src": {},
        }),
    )
    .await;
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-b"),
            ref_name: Some("refs/heads/feature-b".into()),
            sha: "bbb".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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
    save_named_thread_metadata("thread-a", "Thread A", &project_a, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Workspace A's thread appears normally. Workspace B (threadless)
    // appears as a "New Thread" button with its worktree chip.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project]", "  Thread A {wt-feature-a}",]
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
            ".git": {},
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/project_b",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    // Worktree checkouts.
    for repo in &["project_a", "project_b"] {
        let git_path = format!("/{repo}/.git");
        for branch in &["olivetti", "selectric"] {
            fs.add_linked_worktree_for_repo(
                Path::new(&git_path),
                false,
                git::repository::Worktree {
                    path: std::path::PathBuf::from(format!("/worktrees/{repo}/{branch}/{repo}")),
                    ref_name: Some(format!("refs/heads/{branch}").into()),
                    sha: "aaa".into(),
                    is_main: false,
                    is_bare: false,
                },
            )
            .await;
        }
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
    save_named_thread_metadata("wt-thread", "Cross Worktree Thread", &project, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Should show two distinct worktree chips.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project_a, project_b]",
            "  Cross Worktree Thread {project_a:olivetti}, {project_b:selectric}",
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
            ".git": {},
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/project_b",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    for repo in &["project_a", "project_b"] {
        let git_path = format!("/{repo}/.git");
        fs.add_linked_worktree_for_repo(
            Path::new(&git_path),
            false,
            git::repository::Worktree {
                path: std::path::PathBuf::from(format!("/worktrees/{repo}/olivetti/{repo}")),
                ref_name: Some("refs/heads/olivetti".into()),
                sha: "aaa".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;
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
    save_named_thread_metadata("wt-thread", "Same Branch Thread", &project, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Both worktree paths have the name "olivetti", so only one chip.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
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
            ".git": {},
            "src": {},
        }),
    )
    .await;

    // Worktree checkout pointing back to the main repo.
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Add an agent panel to the worktree workspace so we can run a
    // thread inside it.
    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    // Switch back to the main workspace before setting up the sidebar.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().next().unwrap().clone();
        mw.activate(workspace, window, cx);
    });

    // Start a thread in the worktree workspace's panel and keep it
    // generating (don't resolve it).
    let connection = StubAgentConnection::new();
    open_thread_with_connection(&worktree_panel, connection.clone(), cx);
    send_message(&worktree_panel, cx);

    let session_id = active_session_id(&worktree_panel, cx);

    // Save metadata so the sidebar knows about this thread.
    save_test_thread_metadata(&session_id, &worktree_project, cx).await;

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
        vec!["v [project]", "  Hello {wt-feature-a} * (running)",]
    );
}

#[gpui::test]
async fn test_absorbed_worktree_completion_triggers_notification(cx: &mut TestAppContext) {
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().next().unwrap().clone();
        mw.activate(workspace, window, cx);
    });

    let connection = StubAgentConnection::new();
    open_thread_with_connection(&worktree_panel, connection.clone(), cx);
    send_message(&worktree_panel, cx);

    let session_id = active_session_id(&worktree_panel, cx);
    save_test_thread_metadata(&session_id, &worktree_project, cx).await;

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
        vec!["v [project]", "  Hello {wt-feature-a} * (running)",]
    );

    connection.end_turn(session_id, acp::StopReason::EndTurn);
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project]", "  Hello {wt-feature-a} * (!)",]
    );
}

#[gpui::test]
async fn test_clicking_worktree_thread_opens_workspace_when_none_exists(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Only open the main repo — no workspace for the worktree.
    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread for the worktree path (no workspace for it).
    save_named_thread_metadata("thread-wt", "WT Thread", &worktree_project, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Thread should appear under the main repo with a worktree chip.
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project]",
            "  WT Thread {wt-feature-a}",
        ],
    );

    // Only 1 workspace should exist.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
    );

    // Focus the sidebar and select the worktree thread.
    focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(1); // index 0 is header, 1 is the thread
    });

    // Confirm to open the worktree thread.
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    // A new workspace should have been created for the worktree path.
    let new_workspace = multi_workspace.read_with(cx, |mw, _| {
        assert_eq!(
            mw.workspaces().count(),
            2,
            "confirming a worktree thread without a workspace should open one",
        );
        mw.workspaces().nth(1).unwrap().clone()
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
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_named_thread_metadata("thread-wt", "WT Thread", &worktree_project, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [project]",
            "  WT Thread {wt-feature-a}",
        ],
    );

    focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(1); // index 0 is header, 1 is the thread
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
                    if thread.metadata.title.as_ref().map(|t| t.as_ref()) == Some("WT Thread")
                        && thread.worktrees.first().map(|wt| wt.name.as_ref())
                            == Some("wt-feature-a") =>
                {
                    saw_expected_thread = true;
                }
                ListEntry::Thread(thread) => {
                    let title = thread.metadata.display_title();
                    let worktree_name = thread
                        .worktrees
                        .first()
                        .map(|wt| wt.name.as_ref())
                        .unwrap_or("<none>");
                    panic!(
                        "unexpected sidebar thread while opening linked worktree thread: title=`{}`, worktree=`{}`",
                        title, worktree_name
                    );
                }
                ListEntry::ViewMore { .. } => {
                    panic!("unexpected `View More` entry while opening linked worktree thread");
                }
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
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Activate the main workspace before setting up the sidebar.
    let main_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().next().unwrap().clone();
        mw.activate(workspace.clone(), window, cx);
        workspace
    });

    save_named_thread_metadata("thread-main", "Main Thread", &main_project, cx).await;
    save_named_thread_metadata("thread-wt", "WT Thread", &worktree_project, cx).await;

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
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        main_workspace,
        "main workspace should be active initially"
    );

    // Focus the sidebar and select the absorbed worktree thread.
    focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(wt_thread_index);
    });

    // Confirm to activate the worktree thread.
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    // The worktree workspace should now be active, not the main one.
    let active_workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx)
    });
    let workspace_a =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());

    // Save a thread with path_list pointing to project-b.
    let session_id = acp::SessionId::new(Arc::from("archived-1"));
    save_test_thread_metadata(&session_id, &project_b, cx).await;

    // Ensure workspace A is active.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().next().unwrap().clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_a
    );

    // Call activate_archived_thread – should resolve saved paths and
    // switch to the workspace for project-b.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                thread_id: ThreadId::new(),
                session_id: Some(session_id.clone()),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: Some("Archived Thread".into()),
                updated_at: Utc::now(),
                created_at: None,
                worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                    "/project-b",
                )])),
                archived: false,
                remote_connection: None,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_b,
        "should have switched to the workspace matching the saved paths"
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

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b, window, cx)
    });
    let workspace_a =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());

    // Start with workspace A active.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().next().unwrap().clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_a
    );

    // No thread saved to the store – cwd is the only path hint.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                thread_id: ThreadId::new(),
                session_id: Some(acp::SessionId::new(Arc::from("unknown-session"))),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: Some("CWD Thread".into()),
                updated_at: Utc::now(),
                created_at: None,
                worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[
                    std::path::PathBuf::from("/project-b"),
                ])),
                archived: false,
                remote_connection: None,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_b,
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

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b, window, cx)
    });

    // Activate workspace B (index 1) to make it the active one.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().nth(1).unwrap().clone();
        mw.activate(workspace, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_b
    );

    // No saved thread, no cwd – should fall back to the active workspace.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                thread_id: ThreadId::new(),
                session_id: Some(acp::SessionId::new(Arc::from("no-context-session"))),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: Some("Contextless Thread".into()),
                updated_at: Utc::now(),
                created_at: None,
                worktree_paths: WorktreePaths::default(),
                archived: false,
                remote_connection: None,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspace().clone()),
        workspace_b,
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
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "should start with one workspace"
    );

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                thread_id: ThreadId::new(),
                session_id: Some(session_id.clone()),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: Some("New WS Thread".into()),
                updated_at: Utc::now(),
                created_at: None,
                worktree_paths: WorktreePaths::from_folder_paths(&path_list_b),
                archived: false,
                remote_connection: None,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
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
    let multi_workspace_b_entity = multi_workspace_b.root(cx).unwrap();

    let cx_b = &mut gpui::VisualTestContext::from_window(multi_workspace_b.into(), cx);
    let _sidebar_b = setup_sidebar(&multi_workspace_b_entity, cx_b);

    let cx_a = &mut gpui::VisualTestContext::from_window(multi_workspace_a.into(), cx);
    let sidebar = setup_sidebar(&multi_workspace_a_entity, cx_a);

    let session_id = acp::SessionId::new(Arc::from("archived-cross-window"));

    sidebar.update_in(cx_a, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                thread_id: ThreadId::new(),
                session_id: Some(session_id.clone()),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: Some("Cross Window Thread".into()),
                updated_at: Utc::now(),
                created_at: None,
                worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                    "/project-b",
                )])),
                archived: false,
                remote_connection: None,
            },
            window,
            cx,
        );
    });
    cx_a.run_until_parked();

    assert_eq!(
        multi_workspace_a
            .read_with(cx_a, |mw, _| mw.workspaces().count())
            .unwrap(),
        1,
        "should not add the other window's workspace into the current window"
    );
    assert_eq!(
        multi_workspace_b
            .read_with(cx_a, |mw, _| mw.workspaces().count())
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
                !is_active_session(&sidebar, &session_id),
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
                thread_id: ThreadId::new(),
                session_id: Some(session_id.clone()),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: Some("Cross Window Thread".into()),
                updated_at: Utc::now(),
                created_at: None,
                worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                    "/project-b",
                )])),
                archived: false,
                remote_connection: None,
            },
            window,
            cx,
        );
    });
    cx_a.run_until_parked();

    assert_eq!(
        multi_workspace_a
            .read_with(cx_a, |mw, _| mw.workspaces().count())
            .unwrap(),
        1,
        "should not add the other window's workspace into the current window"
    );
    assert_eq!(
        multi_workspace_b
            .read_with(cx_a, |mw, _| mw.workspaces().count())
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
                !is_active_session(&sidebar, &session_id),
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
    let multi_workspace_b_entity = multi_workspace_b.root(cx).unwrap();

    let cx_b = &mut gpui::VisualTestContext::from_window(multi_workspace_b.into(), cx);
    let _sidebar_b = setup_sidebar(&multi_workspace_b_entity, cx_b);

    let cx_a = &mut gpui::VisualTestContext::from_window(multi_workspace_a.into(), cx);
    let sidebar_a = setup_sidebar(&multi_workspace_a_entity, cx_a);

    let session_id = acp::SessionId::new(Arc::from("archived-current-window"));

    sidebar_a.update_in(cx_a, |sidebar, window, cx| {
        sidebar.activate_archived_thread(
            ThreadMetadata {
                thread_id: ThreadId::new(),
                session_id: Some(session_id.clone()),
                agent_id: agent::ZED_AGENT_ID.clone(),
                title: Some("Current Window Thread".into()),
                updated_at: Utc::now(),
                created_at: None,
                worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                    "/project-a",
                )])),
                archived: false,
                remote_connection: None,
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
            .read_with(cx_a, |mw, _| mw.workspaces().count())
            .unwrap(),
        1,
        "current window should continue reusing its existing workspace"
    );
    assert_eq!(
        multi_workspace_b
            .read_with(cx_a, |mw, _| mw.workspaces().count())
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
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Activate main workspace so the sidebar tracks the main panel.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = mw.workspaces().next().unwrap().clone();
        mw.activate(workspace, window, cx);
    });

    let main_workspace =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());
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
        Some("Thread 2".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &main_project,
        cx,
    );

    // Save thread 1's metadata with the worktree path and an older timestamp so
    // it sorts below thread 2. archive_thread will find it as the "next" candidate.
    let thread1_session_id = acp::SessionId::new(Arc::from("thread1-worktree-session"));
    save_thread_metadata(
        thread1_session_id,
        Some("Thread 1".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &worktree_project,
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
async fn test_archive_last_worktree_thread_removes_workspace(cx: &mut TestAppContext) {
    // When the last non-archived thread for a linked worktree is archived,
    // the linked worktree workspace should be removed from the multi-workspace.
    // The main worktree workspace should remain (it's always reachable via
    // the project header).
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

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "abc".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let _worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Save a thread for the main project.
    save_thread_metadata(
        acp::SessionId::new(Arc::from("main-thread")),
        Some("Main Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &main_project,
        cx,
    );

    // Save a thread for the linked worktree.
    let wt_thread_id = acp::SessionId::new(Arc::from("worktree-thread"));
    save_thread_metadata(
        wt_thread_id.clone(),
        Some("Worktree Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &worktree_project,
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Should have 2 workspaces.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2,
        "should start with 2 workspaces (main + linked worktree)"
    );

    // Archive the worktree thread (the only thread for /wt-feature-a).
    sidebar.update_in(cx, |sidebar: &mut Sidebar, window, cx| {
        sidebar.archive_thread(&wt_thread_id, window, cx);
    });

    // archive_thread spawns a multi-layered chain of tasks (workspace
    // removal → git persist → disk removal), each of which may spawn
    // further background work. Each run_until_parked() call drives one
    // layer of pending work.

    cx.run_until_parked();

    // The linked worktree workspace should have been removed.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "linked worktree workspace should be removed after archiving its last thread"
    );

    // The linked worktree checkout directory should also be removed from disk.
    assert!(
        !fs.is_dir(Path::new("/wt-feature-a")).await,
        "linked worktree directory should be removed from disk after archiving its last thread"
    );

    // The main thread should still be visible.
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries.iter().any(|e| e.contains("Main Thread")),
        "main thread should still be visible: {entries:?}"
    );
    assert!(
        !entries.iter().any(|e| e.contains("Worktree Thread")),
        "archived worktree thread should not be visible: {entries:?}"
    );

    // The archived thread must retain its folder_paths so it can be
    // restored to the correct workspace later.
    let wt_thread_id = cx.update(|_window, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry_by_session(&wt_thread_id)
            .unwrap()
            .thread_id
    });
    let archived_paths = cx.update(|_window, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(wt_thread_id)
            .unwrap()
            .folder_paths()
            .clone()
    });
    assert_eq!(
        archived_paths.paths(),
        &[PathBuf::from("/wt-feature-a")],
        "archived thread must retain its folder_paths for restore"
    );
}

#[gpui::test]
async fn test_restore_worktree_when_branch_has_moved(cx: &mut TestAppContext) {
    // restore_worktree_via_git should succeed when the branch has moved
    // to a different SHA since archival. The worktree stays in detached
    // HEAD and the moved branch is left untouched.
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
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "original-sha".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, _cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    multi_workspace.update_in(_cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let wt_repo = worktree_project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });
    let (staged_hash, unstaged_hash) = cx
        .update(|cx| wt_repo.update(cx, |repo, _| repo.create_archive_checkpoint()))
        .await
        .unwrap()
        .unwrap();

    // Move the branch to a different SHA.
    fs.with_git_state(Path::new("/project/.git"), false, |state| {
        state
            .refs
            .insert("refs/heads/feature-a".into(), "moved-sha".into());
    })
    .unwrap();

    let result = cx
        .spawn(|mut cx| async move {
            agent_ui::thread_worktree_archive::restore_worktree_via_git(
                &agent_ui::thread_metadata_store::ArchivedGitWorktree {
                    id: 1,
                    worktree_path: PathBuf::from("/wt-feature-a"),
                    main_repo_path: PathBuf::from("/project"),
                    branch_name: Some("feature-a".to_string()),
                    staged_commit_hash: staged_hash,
                    unstaged_commit_hash: unstaged_hash,
                    original_commit_hash: "original-sha".to_string(),
                },
                &mut cx,
            )
            .await
        })
        .await;

    assert!(
        result.is_ok(),
        "restore should succeed even when branch has moved: {:?}",
        result.err()
    );

    // The moved branch ref should be completely untouched.
    let branch_sha = fs
        .with_git_state(Path::new("/project/.git"), false, |state| {
            state.refs.get("refs/heads/feature-a").cloned()
        })
        .unwrap();
    assert_eq!(
        branch_sha.as_deref(),
        Some("moved-sha"),
        "the moved branch ref should not be modified by the restore"
    );
}

#[gpui::test]
async fn test_restore_worktree_when_branch_has_not_moved(cx: &mut TestAppContext) {
    // restore_worktree_via_git should succeed when the branch still
    // points at the same SHA as at archive time.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
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
        "/wt-feature-b",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-b",
            "src": {},
        }),
    )
    .await;
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-b"),
            ref_name: Some("refs/heads/feature-b".into()),
            sha: "original-sha".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-b".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, _cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    multi_workspace.update_in(_cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let wt_repo = worktree_project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });
    let (staged_hash, unstaged_hash) = cx
        .update(|cx| wt_repo.update(cx, |repo, _| repo.create_archive_checkpoint()))
        .await
        .unwrap()
        .unwrap();

    // refs/heads/feature-b already points at "original-sha" (set by
    // add_linked_worktree_for_repo), matching original_commit_hash.

    let result = cx
        .spawn(|mut cx| async move {
            agent_ui::thread_worktree_archive::restore_worktree_via_git(
                &agent_ui::thread_metadata_store::ArchivedGitWorktree {
                    id: 1,
                    worktree_path: PathBuf::from("/wt-feature-b"),
                    main_repo_path: PathBuf::from("/project"),
                    branch_name: Some("feature-b".to_string()),
                    staged_commit_hash: staged_hash,
                    unstaged_commit_hash: unstaged_hash,
                    original_commit_hash: "original-sha".to_string(),
                },
                &mut cx,
            )
            .await
        })
        .await;

    assert!(
        result.is_ok(),
        "restore should succeed when branch has not moved: {:?}",
        result.err()
    );
}

#[gpui::test]
async fn test_restore_worktree_when_branch_does_not_exist(cx: &mut TestAppContext) {
    // restore_worktree_via_git should succeed when the branch no longer
    // exists (e.g. it was deleted while the thread was archived). The
    // code should attempt to recreate the branch.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-d": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-d",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/wt-feature-d",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-d",
            "src": {},
        }),
    )
    .await;
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-d"),
            ref_name: Some("refs/heads/feature-d".into()),
            sha: "original-sha".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-d".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, _cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    multi_workspace.update_in(_cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let wt_repo = worktree_project.read_with(cx, |project, cx| {
        project.repositories(cx).values().next().unwrap().clone()
    });
    let (staged_hash, unstaged_hash) = cx
        .update(|cx| wt_repo.update(cx, |repo, _| repo.create_archive_checkpoint()))
        .await
        .unwrap()
        .unwrap();

    // Remove the branch ref so change_branch will fail.
    fs.with_git_state(Path::new("/project/.git"), false, |state| {
        state.refs.remove("refs/heads/feature-d");
    })
    .unwrap();

    let result = cx
        .spawn(|mut cx| async move {
            agent_ui::thread_worktree_archive::restore_worktree_via_git(
                &agent_ui::thread_metadata_store::ArchivedGitWorktree {
                    id: 1,
                    worktree_path: PathBuf::from("/wt-feature-d"),
                    main_repo_path: PathBuf::from("/project"),
                    branch_name: Some("feature-d".to_string()),
                    staged_commit_hash: staged_hash,
                    unstaged_commit_hash: unstaged_hash,
                    original_commit_hash: "original-sha".to_string(),
                },
                &mut cx,
            )
            .await
        })
        .await;

    assert!(
        result.is_ok(),
        "restore should succeed when branch does not exist: {:?}",
        result.err()
    );
}

#[gpui::test]
async fn test_restore_worktree_thread_uses_main_repo_project_group_key(cx: &mut TestAppContext) {
    // Activating an archived linked worktree thread whose directory has
    // been deleted should reuse the existing main repo workspace, not
    // create a new one. The provisional ProjectGroupKey must be derived
    // from main_worktree_paths so that find_or_create_local_workspace
    // matches the main repo workspace when the worktree path is absent.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-c": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-c",
                    },
                },
            },
            "src": {},
        }),
    )
    .await;

    fs.insert_tree(
        "/wt-feature-c",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/feature-c",
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-c"),
            ref_name: Some("refs/heads/feature-c".into()),
            sha: "original-sha".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-c".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Save thread metadata for the linked worktree.
    let wt_session_id = acp::SessionId::new(Arc::from("wt-thread-c"));
    save_thread_metadata(
        wt_session_id.clone(),
        Some("Worktree Thread C".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &worktree_project,
        cx,
    );
    cx.run_until_parked();

    let thread_id = cx.update(|_window, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry_by_session(&wt_session_id)
            .unwrap()
            .thread_id
    });

    // Archive the thread without creating ArchivedGitWorktree records.
    let store = cx.update(|_window, cx| ThreadMetadataStore::global(cx));
    cx.update(|_window, cx| {
        store.update(cx, |store, cx| store.archive(thread_id, None, cx));
    });
    cx.run_until_parked();

    // Remove the worktree workspace and delete the worktree from disk.
    let main_workspace =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());
    let remove_task = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.remove(
            vec![worktree_workspace],
            move |_this, _window, _cx| Task::ready(Ok(main_workspace)),
            window,
            cx,
        )
    });
    remove_task.await.ok();
    cx.run_until_parked();
    cx.run_until_parked();
    fs.remove_dir(
        Path::new("/wt-feature-c"),
        fs::RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await
    .unwrap();

    let workspace_count_before = multi_workspace.read_with(cx, |mw, _| mw.workspaces().count());
    assert_eq!(
        workspace_count_before, 1,
        "should have only the main workspace"
    );

    // Activate the archived thread. The worktree path is missing from
    // disk, so find_or_create_local_workspace falls back to the
    // provisional ProjectGroupKey to find a matching workspace.
    let metadata = cx.update(|_window, cx| store.read(cx).entry(thread_id).unwrap().clone());
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(metadata, window, cx);
    });
    cx.run_until_parked();

    // The provisional key should use [/project] (the main repo),
    // which matches the existing main workspace. If it incorrectly
    // used [/wt-feature-c] (the linked worktree path), no workspace
    // would match and a spurious new one would be created.
    let workspace_count_after = multi_workspace.read_with(cx, |mw, _| mw.workspaces().count());
    assert_eq!(
        workspace_count_after, 1,
        "restoring a linked worktree thread should reuse the main repo workspace, \
         not create a new one (workspace count went from {workspace_count_before} to \
         {workspace_count_after})"
    );
}

#[gpui::test]
async fn test_archive_last_worktree_thread_not_blocked_by_remote_thread_at_same_path(
    cx: &mut TestAppContext,
) {
    // A remote thread at the same path as a local linked worktree thread
    // should not prevent the local workspace from being removed when the
    // local thread is archived (the last local thread for that worktree).
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

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "abc".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let _worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Save a thread for the main project.
    save_thread_metadata(
        acp::SessionId::new(Arc::from("main-thread")),
        Some("Main Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &main_project,
        cx,
    );

    // Save a local thread for the linked worktree.
    let wt_thread_id = acp::SessionId::new(Arc::from("worktree-thread"));
    save_thread_metadata(
        wt_thread_id.clone(),
        Some("Local Worktree Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &worktree_project,
        cx,
    );

    // Save a remote thread at the same /wt-feature-a path but on a
    // different host. This should NOT count as a remaining thread for
    // the local linked worktree workspace.
    let remote_host =
        remote::RemoteConnectionOptions::Mock(remote::MockConnectionOptions { id: 99 });
    cx.update(|_window, cx| {
        let metadata = ThreadMetadata {
            thread_id: ThreadId::new(),
            session_id: Some(acp::SessionId::new(Arc::from("remote-wt-thread"))),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: Some("Remote Worktree Thread".into()),
            updated_at: chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            created_at: None,
            worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                "/wt-feature-a",
            )])),
            archived: false,
            remote_connection: Some(remote_host),
        };
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.save(metadata, cx);
        });
    });
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2,
        "should start with 2 workspaces (main + linked worktree)"
    );

    // The remote thread should NOT appear in the sidebar (it belongs
    // to a different host and no matching remote project group exists).
    let entries_before = visible_entries_as_strings(&sidebar, cx);
    assert!(
        !entries_before
            .iter()
            .any(|e| e.contains("Remote Worktree Thread")),
        "remote thread should not appear in local sidebar: {entries_before:?}"
    );

    // Archive the local worktree thread.
    sidebar.update_in(cx, |sidebar: &mut Sidebar, window, cx| {
        sidebar.archive_thread(&wt_thread_id, window, cx);
    });

    cx.run_until_parked();

    // The linked worktree workspace should be removed because the
    // only *local* thread for it was archived. The remote thread at
    // the same path should not have prevented removal.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "linked worktree workspace should be removed; the remote thread at the same path \
         should not count as a remaining local thread"
    );

    let entries = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries.iter().any(|e| e.contains("Main Thread")),
        "main thread should still be visible: {entries:?}"
    );
    assert!(
        !entries.iter().any(|e| e.contains("Local Worktree Thread")),
        "archived local worktree thread should not be visible: {entries:?}"
    );
    assert!(
        !entries.iter().any(|e| e.contains("Remote Worktree Thread")),
        "remote thread should still not appear in local sidebar: {entries:?}"
    );
}

#[gpui::test]
async fn test_linked_worktree_threads_not_duplicated_across_groups(cx: &mut TestAppContext) {
    // When a multi-root workspace (e.g. [/other, /project]) shares a
    // repo with a single-root workspace (e.g. [/project]), linked
    // worktree threads from the shared repo should only appear under
    // the dedicated group [project], not under [other, project].
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });
    let fs = FakeFs::new(cx.executor());

    // Two independent repos, each with their own git history.
    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {},
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
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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

    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    // Save a thread under the linked worktree path BEFORE setting up
    // the sidebar and panels, so that reconciliation sees the [project]
    // group as non-empty and doesn't create a spurious draft there.
    let wt_session_id = acp::SessionId::new(Arc::from("wt-thread"));
    save_thread_metadata(
        wt_session_id,
        Some("Worktree Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &worktree_project,
        cx,
    );

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_only.clone(), window, cx));
    let (sidebar, _panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);
    let multi_root_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(multi_root.clone(), window, cx)
    });
    add_agent_panel(&multi_root_workspace, cx);
    cx.run_until_parked();

    // The thread should appear only under [project] (the dedicated
    // group for the /project repo), not under [other, project].
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            //
            "v [other, project]",
            "v [project]",
            "  Worktree Thread {wt-feature-a}",
        ]
    );
}

#[gpui::test]
async fn test_thread_switcher_ordering(cx: &mut TestAppContext) {
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

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
        Some("Thread C".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap()),
        &project,
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
        Some("Thread B".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap()),
        &project,
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
        Some("Thread A".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 3, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 3, 0, 0, 0).unwrap()),
        &project,
        cx,
    );

    // All three threads are now live. Thread A was opened last, so it's
    // the one being viewed. Opening each thread called record_thread_access,
    // so all three have last_accessed_at set.
    // Access order is: A (most recent), B, C (oldest).

    // ── 1. Open switcher: threads sorted by last_accessed_at ─────────────────
    focus_sidebar(&sidebar, cx);
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
            is_active_session(&sidebar, &session_id_c),
            "active_entry should be Thread({session_id_c:?})"
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
            is_active_session(&sidebar, &session_id_a),
            "active_entry should be Thread({session_id_a:?})"
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
            is_active_session(&sidebar, &session_id_b),
            "active_entry should be Thread({session_id_b:?})"
        );
    });

    // ── 3. Add a historical thread (no last_accessed_at, no message sent) ──
    // This thread was never opened in a panel — it only exists in metadata.
    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-historical")),
        Some("Historical Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap()),
        &project,
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
        Some("Old Historical Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2023, 6, 1, 0, 0, 0).unwrap(),
        Some(chrono::TimeZone::with_ymd_and_hms(&Utc, 2023, 6, 1, 0, 0, 0).unwrap()),
        &project,
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
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-to-archive")),
        Some("Thread To Archive".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project,
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
        assert_eq!(
            archived[0].session_id.as_ref().unwrap().0.as_ref(),
            "thread-to-archive"
        );
        assert!(archived[0].archived);
    });
}

#[gpui::test]
async fn test_archive_thread_active_entry_management(cx: &mut TestAppContext) {
    // Tests two archive scenarios:
    // 1. Archiving a thread in a non-active workspace leaves active_entry
    //    as the current draft.
    // 2. Archiving the thread the user is looking at falls back to a draft
    //    on the same workspace.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx)
    });
    let panel_b = add_agent_panel(&workspace_b, cx);
    cx.run_until_parked();

    // Explicitly create a draft on workspace_b so the sidebar tracks one.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.create_new_thread(&workspace_b, window, cx);
    });
    cx.run_until_parked();

    // --- Scenario 1: archive a thread in the non-active workspace ---

    // Create a thread in project-a (non-active — project-b is active).
    let connection = acp_thread::StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    agent_ui::test_support::open_thread_with_connection(&panel_a, connection, cx);
    agent_ui::test_support::send_message(&panel_a, cx);
    let thread_a = agent_ui::test_support::active_session_id(&panel_a, cx);
    cx.run_until_parked();

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&thread_a, window, cx);
    });
    cx.run_until_parked();

    // active_entry should still be a draft on workspace_b (the active one).
    sidebar.read_with(cx, |sidebar, _| {
        assert!(
            matches!(&sidebar.active_entry, Some(ActiveEntry { workspace: ws, .. }) if ws == &workspace_b),
            "expected Draft(workspace_b) after archiving non-active thread, got: {:?}",
            sidebar.active_entry,
        );
    });

    // --- Scenario 2: archive the thread the user is looking at ---

    // Create a thread in project-b (the active workspace) and verify it
    // becomes the active entry.
    let connection = acp_thread::StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    agent_ui::test_support::open_thread_with_connection(&panel_b, connection, cx);
    agent_ui::test_support::send_message(&panel_b, cx);
    let thread_b = agent_ui::test_support::active_session_id(&panel_b, cx);
    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _| {
        assert!(
            is_active_session(&sidebar, &thread_b),
            "expected active_entry to be Thread({thread_b}), got: {:?}",
            sidebar.active_entry,
        );
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&thread_b, window, cx);
    });
    cx.run_until_parked();

    // Archiving the active thread activates a draft on the same workspace
    // (via clear_base_view → activate_draft). The draft is not shown as a
    // sidebar row but active_entry tracks it.
    sidebar.read_with(cx, |sidebar, _| {
        assert!(
            matches!(&sidebar.active_entry, Some(ActiveEntry { workspace: ws, .. }) if ws == &workspace_b),
            "expected draft on workspace_b after archiving active thread, got: {:?}",
            sidebar.active_entry,
        );
    });
}

#[gpui::test]
async fn test_unarchive_only_shows_restored_thread(cx: &mut TestAppContext) {
    // Full flow: create a thread, archive it (removing the workspace),
    // then unarchive. Only the restored thread should appear — no
    // leftover drafts or previously-serialized threads.
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);
    cx.run_until_parked();

    // Create a thread and send a message so it's a real thread.
    let connection = acp_thread::StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Hello".into()),
    )]);
    agent_ui::test_support::open_thread_with_connection(&panel, connection, cx);
    agent_ui::test_support::send_message(&panel, cx);
    let session_id = agent_ui::test_support::active_session_id(&panel, cx);
    cx.run_until_parked();

    // Archive it.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&session_id, window, cx);
    });
    cx.run_until_parked();

    // Grab metadata for unarchive.
    let thread_id = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
            .expect("thread should exist")
    });
    let metadata = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(thread_id)
            .cloned()
            .expect("metadata should exist")
    });

    // Unarchive it — the draft should be replaced by the restored thread.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(metadata, window, cx);
    });
    cx.run_until_parked();

    // Only the unarchived thread should be visible — no drafts, no other threads.
    let entries = visible_entries_as_strings(&sidebar, cx);
    let thread_count = entries
        .iter()
        .filter(|e| !e.starts_with("v ") && !e.starts_with("> "))
        .count();
    assert_eq!(
        thread_count, 1,
        "expected exactly 1 thread entry (the restored one), got entries: {entries:?}"
    );
    assert!(
        !entries.iter().any(|e| e.contains("Draft")),
        "expected no drafts after restoring, got entries: {entries:?}"
    );
}

#[gpui::test]
async fn test_unarchive_first_thread_in_group_does_not_create_spurious_draft(
    cx: &mut TestAppContext,
) {
    // When a thread is unarchived into a project group that has no open
    // workspace, the sidebar opens a new workspace and loads the thread.
    // No spurious draft should appear alongside the unarchived thread.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);
    cx.run_until_parked();

    // Save an archived thread whose folder_paths point to project-b,
    // which has no open workspace.
    let session_id = acp::SessionId::new(Arc::from("archived-thread"));
    let path_list_b = PathList::new(&[std::path::PathBuf::from("/project-b")]);
    let thread_id = ThreadId::new();
    cx.update(|_, cx| {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.save(
                ThreadMetadata {
                    thread_id,
                    session_id: Some(session_id.clone()),
                    agent_id: agent::ZED_AGENT_ID.clone(),
                    title: Some("Unarchived Thread".into()),
                    updated_at: Utc::now(),
                    created_at: None,
                    worktree_paths: WorktreePaths::from_folder_paths(&path_list_b),
                    archived: true,
                    remote_connection: None,
                },
                cx,
            )
        });
    });
    cx.run_until_parked();

    // Verify no workspace for project-b exists yet.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "should start with only the project-a workspace"
    );

    // Un-archive the thread — should open project-b workspace and load it.
    let metadata = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(thread_id)
            .cloned()
            .expect("metadata should exist")
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(metadata, window, cx);
    });
    cx.run_until_parked();

    // A second workspace should have been created for project-b.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2,
        "should have opened a workspace for the unarchived thread"
    );

    // The sidebar should show the unarchived thread without a spurious draft
    // in the project-b group.
    let entries = visible_entries_as_strings(&sidebar, cx);
    let draft_count = entries.iter().filter(|e| e.contains("Draft")).count();
    // project-a gets a draft (it's the active workspace with no threads),
    // but project-b should NOT have one — only the unarchived thread.
    assert!(
        draft_count <= 1,
        "expected at most one draft (for project-a), got entries: {entries:?}"
    );
    assert!(
        entries.iter().any(|e| e.contains("Unarchived Thread")),
        "expected unarchived thread to appear, got entries: {entries:?}"
    );
}

#[gpui::test]
async fn test_unarchive_into_new_workspace_does_not_create_duplicate_real_thread(
    cx: &mut TestAppContext,
) {
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);
    cx.run_until_parked();

    let session_id = acp::SessionId::new(Arc::from("restore-into-new-workspace"));
    let path_list_b = PathList::new(&[PathBuf::from("/project-b")]);
    let original_thread_id = ThreadId::new();
    cx.update(|_, cx| {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.save(
                ThreadMetadata {
                    thread_id: original_thread_id,
                    session_id: Some(session_id.clone()),
                    agent_id: agent::ZED_AGENT_ID.clone(),
                    title: Some("Unarchived Thread".into()),
                    updated_at: Utc::now(),
                    created_at: None,
                    worktree_paths: WorktreePaths::from_folder_paths(&path_list_b),
                    archived: true,
                    remote_connection: None,
                },
                cx,
            )
        });
    });
    cx.run_until_parked();

    let metadata = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(original_thread_id)
            .cloned()
            .expect("metadata should exist before unarchive")
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(metadata, window, cx);
    });

    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2,
        "expected unarchive to open the target workspace"
    );

    let restored_workspace = multi_workspace.read_with(cx, |mw, cx| {
        mw.workspaces()
            .find(|workspace| PathList::new(&workspace.read(cx).root_paths(cx)) == path_list_b)
            .cloned()
            .expect("expected restored workspace for unarchived thread")
    });
    let restored_panel = restored_workspace.read_with(cx, |workspace, cx| {
        workspace
            .panel::<AgentPanel>(cx)
            .expect("expected unarchive to install an agent panel in the new workspace")
    });

    let restored_thread_id = restored_panel.read_with(cx, |panel, cx| panel.active_thread_id(cx));
    assert_eq!(
        restored_thread_id,
        Some(original_thread_id),
        "expected the new workspace's agent panel to target the restored archived thread id"
    );

    let session_entries = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .filter(|entry| entry.session_id.as_ref() == Some(&session_id))
            .cloned()
            .collect::<Vec<_>>()
    });
    assert_eq!(
        session_entries.len(),
        1,
        "expected exactly one metadata row for restored session after opening a new workspace, got: {session_entries:?}"
    );
    assert_eq!(
        session_entries[0].thread_id, original_thread_id,
        "expected restore into a new workspace to reuse the original thread id"
    );
    assert!(
        !session_entries[0].archived,
        "expected restored thread metadata to be unarchived, got: {:?}",
        session_entries[0]
    );

    let mapped_thread_id = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
    });
    assert_eq!(
        mapped_thread_id,
        Some(original_thread_id),
        "expected session mapping to remain stable after opening the new workspace"
    );

    let entries = visible_entries_as_strings(&sidebar, cx);
    let real_thread_rows = entries
        .iter()
        .filter(|entry| !entry.starts_with("v ") && !entry.starts_with("> "))
        .filter(|entry| !entry.contains("Draft"))
        .count();
    assert_eq!(
        real_thread_rows, 1,
        "expected exactly one visible real thread row after restore into a new workspace, got entries: {entries:?}"
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry.contains("Unarchived Thread")),
        "expected restored thread row to be visible, got entries: {entries:?}"
    );
}

#[gpui::test]
async fn test_unarchive_into_existing_workspace_replaces_draft(cx: &mut TestAppContext) {
    // When a workspace already exists with an empty draft and a thread
    // is unarchived into it, the draft should be replaced — not kept
    // alongside the loaded thread.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/my-project", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project = project::Project::test(fs.clone(), ["/my-project".as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);
    cx.run_until_parked();

    // Create a thread and send a message so it's no longer a draft.
    let connection = acp_thread::StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    agent_ui::test_support::open_thread_with_connection(&panel, connection, cx);
    agent_ui::test_support::send_message(&panel, cx);
    let session_id = agent_ui::test_support::active_session_id(&panel, cx);
    cx.run_until_parked();

    // Archive the thread — the group is left empty (no draft created).
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&session_id, window, cx);
    });
    cx.run_until_parked();

    // Un-archive the thread.
    let thread_id = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
            .expect("thread should exist in store")
    });
    let metadata = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(thread_id)
            .cloned()
            .expect("metadata should exist")
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(metadata, window, cx);
    });
    cx.run_until_parked();

    // The draft should be gone — only the unarchived thread remains.
    let entries = visible_entries_as_strings(&sidebar, cx);
    let draft_count = entries.iter().filter(|e| e.contains("Draft")).count();
    assert_eq!(
        draft_count, 0,
        "expected no drafts after unarchiving, got entries: {entries:?}"
    );
}

#[gpui::test]
async fn test_unarchive_into_inactive_existing_workspace_does_not_leave_active_draft(
    cx: &mut TestAppContext,
) {
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.set_global(agent_ui::MaxIdleRetainedThreads(1));
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let workspace_a = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx)
    });
    let _panel_b = add_agent_panel(&workspace_b, cx);
    cx.run_until_parked();

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.activate(workspace_a.clone(), window, cx);
    });
    cx.run_until_parked();

    let session_id = acp::SessionId::new(Arc::from("unarchive-into-inactive-existing-workspace"));
    let thread_id = ThreadId::new();
    cx.update(|_, cx| {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.save(
                ThreadMetadata {
                    thread_id,
                    session_id: Some(session_id.clone()),
                    agent_id: agent::ZED_AGENT_ID.clone(),
                    title: Some("Restored In Inactive Workspace".into()),
                    updated_at: Utc::now(),
                    created_at: None,
                    worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[
                        PathBuf::from("/project-b"),
                    ])),
                    archived: true,
                    remote_connection: None,
                },
                cx,
            )
        });
    });
    cx.run_until_parked();

    let metadata = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(thread_id)
            .cloned()
            .expect("archived metadata should exist before restore")
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(metadata, window, cx);
    });

    let panel_b_before_settle = workspace_b.read_with(cx, |workspace, cx| {
        workspace.panel::<AgentPanel>(cx).expect(
            "target workspace should still have an agent panel immediately after activation",
        )
    });
    let immediate_active_thread_id =
        panel_b_before_settle.read_with(cx, |panel, cx| panel.active_thread_id(cx));

    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id,
            "unarchiving into an inactive existing workspace should end on the restored thread",
        );
    });

    let panel_b = workspace_b.read_with(cx, |workspace, cx| {
        workspace
            .panel::<AgentPanel>(cx)
            .expect("target workspace should still have an agent panel")
    });
    assert_eq!(
        panel_b.read_with(cx, |panel, cx| panel.active_thread_id(cx)),
        Some(thread_id),
        "expected target panel to activate the restored thread id"
    );
    assert!(
        immediate_active_thread_id.is_none() || immediate_active_thread_id == Some(thread_id),
        "expected immediate panel state to be either still loading or already on the restored thread, got active_thread_id={immediate_active_thread_id:?}"
    );

    let entries = visible_entries_as_strings(&sidebar, cx);
    let target_rows: Vec<_> = entries
        .iter()
        .filter(|entry| entry.contains("Restored In Inactive Workspace") || entry.contains("Draft"))
        .cloned()
        .collect();
    assert_eq!(
        target_rows.len(),
        1,
        "expected only the restored row and no surviving draft in the target group, got entries: {entries:?}"
    );
    assert!(
        target_rows[0].contains("Restored In Inactive Workspace"),
        "expected the remaining row to be the restored thread, got entries: {entries:?}"
    );
    assert!(
        !target_rows[0].contains("Draft"),
        "expected no surviving draft row after unarchive into inactive existing workspace, got entries: {entries:?}"
    );
}

#[gpui::test]
async fn test_unarchive_after_removing_parent_project_group_restores_real_thread(
    cx: &mut TestAppContext,
) {
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.set_global(agent_ui::MaxIdleRetainedThreads(1));
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx)
    });
    let panel_b = add_agent_panel(&workspace_b, cx);
    cx.run_until_parked();

    let connection = acp_thread::StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    agent_ui::test_support::open_thread_with_connection(&panel_b, connection, cx);
    agent_ui::test_support::send_message(&panel_b, cx);
    let session_id = agent_ui::test_support::active_session_id(&panel_b, cx);
    save_test_thread_metadata(&session_id, &project_b, cx).await;
    cx.run_until_parked();

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&session_id, window, cx);
    });

    cx.run_until_parked();

    let archived_metadata = cx.update(|_, cx| {
        let store = ThreadMetadataStore::global(cx).read(cx);
        let thread_id = store
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
            .expect("archived thread should still exist in metadata store");
        let metadata = store
            .entry(thread_id)
            .cloned()
            .expect("archived metadata should still exist after archive");
        assert!(
            metadata.archived,
            "thread should be archived before project removal"
        );
        metadata
    });

    let group_key_b =
        project_b.read_with(cx, |project, cx| ProjectGroupKey::from_project(project, cx));
    let remove_task = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.remove_project_group(&group_key_b, window, cx)
    });
    remove_task
        .await
        .expect("remove project group task should complete");
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "removing the archived thread's parent project group should remove its workspace"
    );

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(archived_metadata.clone(), window, cx);
    });
    cx.run_until_parked();

    let restored_workspace = multi_workspace.read_with(cx, |mw, cx| {
        mw.workspaces()
            .find(|workspace| {
                PathList::new(&workspace.read(cx).root_paths(cx))
                    == PathList::new(&[PathBuf::from("/project-b")])
            })
            .cloned()
            .expect("expected unarchive to recreate the removed project workspace")
    });
    let restored_panel = restored_workspace.read_with(cx, |workspace, cx| {
        workspace
            .panel::<AgentPanel>(cx)
            .expect("expected restored workspace to bootstrap an agent panel")
    });

    let restored_thread_id = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
            .expect("session should still map to restored thread id")
    });
    assert_eq!(
        restored_panel.read_with(cx, |panel, cx| panel.active_thread_id(cx)),
        Some(restored_thread_id),
        "expected unarchive after project removal to activate the restored real thread"
    );

    sidebar.read_with(cx, |sidebar, _cx| {
        assert_active_thread(
            sidebar,
            &session_id,
            "expected sidebar active entry to track the restored thread after project removal",
        );
    });

    let entries = visible_entries_as_strings(&sidebar, cx);
    let restored_title = archived_metadata.display_title().to_string();
    let matching_rows: Vec<_> = entries
        .iter()
        .filter(|entry| entry.contains(&restored_title) || entry.contains("Draft"))
        .cloned()
        .collect();
    assert_eq!(
        matching_rows.len(),
        1,
        "expected only one restored row and no surviving draft after unarchive following project removal, got entries: {entries:?}"
    );
    assert!(
        !matching_rows[0].contains("Draft"),
        "expected no draft row after unarchive following project removal, got entries: {entries:?}"
    );
}

#[gpui::test]
async fn test_unarchive_does_not_create_duplicate_real_thread_metadata(cx: &mut TestAppContext) {
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/my-project", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project = project::Project::test(fs.clone(), ["/my-project".as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);
    cx.run_until_parked();

    let connection = acp_thread::StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    agent_ui::test_support::open_thread_with_connection(&panel, connection, cx);
    agent_ui::test_support::send_message(&panel, cx);
    let session_id = agent_ui::test_support::active_session_id(&panel, cx);
    cx.run_until_parked();

    let original_thread_id = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .find(|e| e.session_id.as_ref() == Some(&session_id))
            .map(|e| e.thread_id)
            .expect("thread should exist in store before archiving")
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&session_id, window, cx);
    });
    cx.run_until_parked();

    let metadata = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(original_thread_id)
            .cloned()
            .expect("metadata should exist after archiving")
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(metadata, window, cx);
    });
    cx.run_until_parked();

    let session_entries = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .filter(|entry| entry.session_id.as_ref() == Some(&session_id))
            .cloned()
            .collect::<Vec<_>>()
    });

    assert_eq!(
        session_entries.len(),
        1,
        "expected exactly one metadata row for the restored session, got: {session_entries:?}"
    );
    assert_eq!(
        session_entries[0].thread_id, original_thread_id,
        "expected unarchive to reuse the original thread id instead of creating a duplicate row"
    );
    assert!(
        session_entries[0].session_id.is_some(),
        "expected restored metadata to be a real thread, got: {:?}",
        session_entries[0]
    );

    let entries = visible_entries_as_strings(&sidebar, cx);
    let real_thread_rows = entries
        .iter()
        .filter(|entry| !entry.starts_with("v ") && !entry.starts_with("> "))
        .filter(|entry| !entry.contains("Draft"))
        .count();
    assert_eq!(
        real_thread_rows, 1,
        "expected exactly one visible real thread row after unarchive, got entries: {entries:?}"
    );
    assert!(
        !entries.iter().any(|entry| entry.contains("Draft")),
        "expected no draft rows after restoring, got entries: {entries:?}"
    );
}

#[gpui::test]
async fn test_switch_to_workspace_with_archived_thread_shows_no_active_entry(
    cx: &mut TestAppContext,
) {
    // When a thread is archived while the user is in a different workspace,
    // clear_base_view creates a draft on the archived workspace's panel.
    // Switching back to that workspace shows the draft as active_entry.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
    let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx)
    });
    let _panel_b = add_agent_panel(&workspace_b, cx);
    cx.run_until_parked();

    // Create a thread in project-a's panel (currently non-active).
    let connection = acp_thread::StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    agent_ui::test_support::open_thread_with_connection(&panel_a, connection, cx);
    agent_ui::test_support::send_message(&panel_a, cx);
    let thread_a = agent_ui::test_support::active_session_id(&panel_a, cx);
    cx.run_until_parked();

    // Archive it while project-b is active.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&thread_a, window, cx);
    });
    cx.run_until_parked();

    // Switch back to project-a. Its panel was cleared during archiving
    // (clear_base_view activated a draft), so active_entry should point
    // to the draft on workspace_a.
    let workspace_a =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.activate(workspace_a.clone(), window, cx);
    });
    cx.run_until_parked();

    sidebar.update_in(cx, |sidebar, _window, cx| {
        sidebar.update_entries(cx);
    });
    cx.run_until_parked();

    sidebar.read_with(cx, |sidebar, _| {
        assert_active_draft(
            sidebar,
            &workspace_a,
            "after switching to workspace with archived thread, active_entry should be the draft",
        );
    });
}

#[gpui::test]
async fn test_archived_threads_excluded_from_sidebar_entries(cx: &mut TestAppContext) {
    let project = init_test_project("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    save_thread_metadata(
        acp::SessionId::new(Arc::from("visible-thread")),
        Some("Visible Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );

    let archived_thread_session_id = acp::SessionId::new(Arc::from("archived-thread"));
    save_thread_metadata(
        archived_thread_session_id.clone(),
        Some("Archived Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );

    cx.update(|_, cx| {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            let thread_id = store
                .entries()
                .find(|e| e.session_id.as_ref() == Some(&archived_thread_session_id))
                .map(|e| e.thread_id)
                .unwrap();
            store.archive(thread_id, None, cx)
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
        assert_eq!(
            archived[0].session_id.as_ref().unwrap().0.as_ref(),
            "archived-thread"
        );
    });
}

#[gpui::test]
async fn test_archive_last_thread_on_linked_worktree_does_not_create_new_thread_on_worktree(
    cx: &mut TestAppContext,
) {
    // When a linked worktree has a single thread and that thread is archived,
    // the sidebar must NOT create a new thread on the same worktree (which
    // would prevent the worktree from being cleaned up on disk). Instead,
    // archive_thread switches to a sibling thread on the main workspace (or
    // creates a draft there) before archiving the metadata.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-ochre-drift"),
            ref_name: Some("refs/heads/ochre-drift".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project =
        project::Project::test(fs.clone(), ["/wt-ochre-drift".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Set up both workspaces with agent panels.
    let main_workspace =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());
    let _main_panel = add_agent_panel(&main_workspace, cx);
    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    // Activate the linked worktree workspace so the sidebar tracks it.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.activate(worktree_workspace.clone(), window, cx);
    });

    // Open a thread in the linked worktree panel and send a message
    // so it becomes the active thread.
    let connection = StubAgentConnection::new();
    open_thread_with_connection(&worktree_panel, connection.clone(), cx);
    send_message(&worktree_panel, cx);

    let worktree_thread_id = active_session_id(&worktree_panel, cx);

    // Give the thread a response chunk so it has content.
    cx.update(|_, cx| {
        connection.send_update(
            worktree_thread_id.clone(),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("done".into())),
            cx,
        );
    });

    // Save the worktree thread's metadata.
    save_thread_metadata(
        worktree_thread_id.clone(),
        Some("Ochre Drift Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &worktree_project,
        cx,
    );

    // Also save a thread on the main project so there's a sibling in the
    // group that can be selected after archiving.
    save_thread_metadata(
        acp::SessionId::new(Arc::from("main-project-thread")),
        Some("Main Project Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &main_project,
        cx,
    );

    cx.run_until_parked();

    // Verify the linked worktree thread appears with its chip.
    // The live thread title comes from the message text ("Hello"), not
    // the metadata title we saved.
    let entries_before = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries_before
            .iter()
            .any(|s| s.contains("{wt-ochre-drift}")),
        "expected worktree thread with chip before archiving, got: {entries_before:?}"
    );
    assert!(
        entries_before
            .iter()
            .any(|s| s.contains("Main Project Thread")),
        "expected main project thread before archiving, got: {entries_before:?}"
    );

    // Confirm the worktree thread is the active entry.
    sidebar.read_with(cx, |s, _| {
        assert_active_thread(
            s,
            &worktree_thread_id,
            "worktree thread should be active before archiving",
        );
    });

    // Archive the worktree thread — it's the only thread using ochre-drift.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&worktree_thread_id, window, cx);
    });

    cx.run_until_parked();

    // The archived thread should no longer appear in the sidebar.
    let entries_after = visible_entries_as_strings(&sidebar, cx);
    assert!(
        !entries_after
            .iter()
            .any(|s| s.contains("Ochre Drift Thread")),
        "archived thread should be hidden, got: {entries_after:?}"
    );

    // No "+ New Thread" entry should appear with the ochre-drift worktree
    // chip — that would keep the worktree alive and prevent cleanup.
    assert!(
        !entries_after.iter().any(|s| s.contains("{wt-ochre-drift}")),
        "no entry should reference the archived worktree, got: {entries_after:?}"
    );

    // The main project thread should still be visible.
    assert!(
        entries_after
            .iter()
            .any(|s| s.contains("Main Project Thread")),
        "main project thread should still be visible, got: {entries_after:?}"
    );
}

#[gpui::test]
async fn test_archive_last_thread_on_linked_worktree_with_no_siblings_leaves_group_empty(
    cx: &mut TestAppContext,
) {
    // When a linked worktree thread is the ONLY thread in the project group
    // (no threads on the main repo either), archiving it should leave the
    // group empty with no active entry.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-ochre-drift"),
            ref_name: Some("refs/heads/ochre-drift".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project =
        project::Project::test(fs.clone(), ["/wt-ochre-drift".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let main_workspace =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());
    let _main_panel = add_agent_panel(&main_workspace, cx);
    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    // Activate the linked worktree workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.activate(worktree_workspace.clone(), window, cx);
    });

    // Open a thread on the linked worktree — this is the ONLY thread.
    let connection = StubAgentConnection::new();
    open_thread_with_connection(&worktree_panel, connection.clone(), cx);
    send_message(&worktree_panel, cx);

    let worktree_thread_id = active_session_id(&worktree_panel, cx);

    cx.update(|_, cx| {
        connection.send_update(
            worktree_thread_id.clone(),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("done".into())),
            cx,
        );
    });

    save_thread_metadata(
        worktree_thread_id.clone(),
        Some("Ochre Drift Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &worktree_project,
        cx,
    );

    cx.run_until_parked();

    // Archive it — there are no other threads in the group.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&worktree_thread_id, window, cx);
    });

    cx.run_until_parked();

    let entries_after = visible_entries_as_strings(&sidebar, cx);

    // No entry should reference the linked worktree.
    assert!(
        !entries_after.iter().any(|s| s.contains("{wt-ochre-drift}")),
        "no entry should reference the archived worktree, got: {entries_after:?}"
    );

    // The active entry should be None — no draft is created.
    sidebar.read_with(cx, |s, _| {
        assert!(
            s.active_entry.is_none(),
            "expected no active entry after archiving the last thread, got: {:?}",
            s.active_entry,
        );
    });
}

#[gpui::test]
async fn test_unarchive_linked_worktree_thread_into_project_group_shows_only_restored_real_thread(
    cx: &mut TestAppContext,
) {
    // When an archived thread belongs to a linked worktree whose main repo is
    // already open, unarchiving should reopen the linked workspace into the
    // same project group and show only the restored real thread row.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.insert_tree(
        "/wt-ochre-drift",
        serde_json::json!({
            ".git": "gitdir: /project/.git/worktrees/ochre-drift",
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-ochre-drift"),
            ref_name: Some("refs/heads/ochre-drift".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project =
        project::Project::test(fs.clone(), ["/wt-ochre-drift".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let sidebar = setup_sidebar(&multi_workspace, cx);
    let main_workspace =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());
    let _main_panel = add_agent_panel(&main_workspace, cx);
    cx.run_until_parked();

    let session_id = acp::SessionId::new(Arc::from("linked-worktree-unarchive"));
    let original_thread_id = ThreadId::new();
    let main_paths = PathList::new(&[PathBuf::from("/project")]);
    let folder_paths = PathList::new(&[PathBuf::from("/wt-ochre-drift")]);

    cx.update(|_, cx| {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.save(
                ThreadMetadata {
                    thread_id: original_thread_id,
                    session_id: Some(session_id.clone()),
                    agent_id: agent::ZED_AGENT_ID.clone(),
                    title: Some("Unarchived Linked Thread".into()),
                    updated_at: Utc::now(),
                    created_at: None,
                    worktree_paths: WorktreePaths::from_path_lists(
                        main_paths.clone(),
                        folder_paths.clone(),
                    )
                    .expect("main and folder paths should be well-formed"),
                    archived: true,
                    remote_connection: None,
                },
                cx,
            )
        });
    });
    cx.run_until_parked();

    let metadata = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(original_thread_id)
            .cloned()
            .expect("archived linked-worktree metadata should exist before restore")
    });

    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.activate_archived_thread(metadata, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2,
        "expected unarchive to open the linked worktree workspace into the project group"
    );

    let session_entries = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entries()
            .filter(|entry| entry.session_id.as_ref() == Some(&session_id))
            .cloned()
            .collect::<Vec<_>>()
    });
    assert_eq!(
        session_entries.len(),
        1,
        "expected exactly one metadata row for restored linked worktree session, got: {session_entries:?}"
    );
    assert_eq!(
        session_entries[0].thread_id, original_thread_id,
        "expected unarchive to reuse the original linked worktree thread id"
    );
    assert!(
        !session_entries[0].archived,
        "expected restored linked worktree metadata to be unarchived, got: {:?}",
        session_entries[0]
    );

    let assert_no_extra_rows = |entries: &[String]| {
        let real_thread_rows = entries
            .iter()
            .filter(|entry| !entry.starts_with("v ") && !entry.starts_with("> "))
            .filter(|entry| !entry.contains("Draft"))
            .count();
        assert_eq!(
            real_thread_rows, 1,
            "expected exactly one visible real thread row after linked-worktree unarchive, got entries: {entries:?}"
        );
        assert!(
            !entries.iter().any(|entry| entry.contains("Draft")),
            "expected no draft rows after linked-worktree unarchive, got entries: {entries:?}"
        );
        assert!(
            !entries
                .iter()
                .any(|entry| entry.contains(DEFAULT_THREAD_TITLE)),
            "expected no default-titled real placeholder row after linked-worktree unarchive, got entries: {entries:?}"
        );
        assert!(
            entries
                .iter()
                .any(|entry| entry.contains("Unarchived Linked Thread")),
            "expected restored linked worktree thread row to be visible, got entries: {entries:?}"
        );
    };

    let entries_after_restore = visible_entries_as_strings(&sidebar, cx);
    assert_no_extra_rows(&entries_after_restore);

    // The reported bug may only appear after an extra scheduling turn.
    cx.run_until_parked();

    let entries_after_extra_turns = visible_entries_as_strings(&sidebar, cx);
    assert_no_extra_rows(&entries_after_extra_turns);
}

#[gpui::test]
async fn test_archive_thread_on_linked_worktree_selects_sibling_thread(cx: &mut TestAppContext) {
    // When a linked worktree thread is archived but the group has other
    // threads (e.g. on the main project), archive_thread should select
    // the nearest sibling.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/project",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-ochre-drift"),
            ref_name: Some("refs/heads/ochre-drift".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    let worktree_project =
        project::Project::test(fs.clone(), ["/wt-ochre-drift".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));

    let sidebar = setup_sidebar(&multi_workspace, cx);

    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    let main_workspace =
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().unwrap().clone());
    let _main_panel = add_agent_panel(&main_workspace, cx);
    let worktree_panel = add_agent_panel(&worktree_workspace, cx);

    // Activate the linked worktree workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.activate(worktree_workspace.clone(), window, cx);
    });

    // Open a thread on the linked worktree.
    let connection = StubAgentConnection::new();
    open_thread_with_connection(&worktree_panel, connection.clone(), cx);
    send_message(&worktree_panel, cx);

    let worktree_thread_id = active_session_id(&worktree_panel, cx);

    cx.update(|_, cx| {
        connection.send_update(
            worktree_thread_id.clone(),
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("done".into())),
            cx,
        );
    });

    save_thread_metadata(
        worktree_thread_id.clone(),
        Some("Ochre Drift Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &worktree_project,
        cx,
    );

    // Save a sibling thread on the main project.
    let main_thread_id = acp::SessionId::new(Arc::from("main-project-thread"));
    save_thread_metadata(
        main_thread_id,
        Some("Main Project Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &main_project,
        cx,
    );

    cx.run_until_parked();

    // Confirm the worktree thread is active.
    sidebar.read_with(cx, |s, _| {
        assert_active_thread(
            s,
            &worktree_thread_id,
            "worktree thread should be active before archiving",
        );
    });

    // Archive the worktree thread.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&worktree_thread_id, window, cx);
    });

    cx.run_until_parked();

    // The worktree workspace was removed and a draft was created on the
    // main workspace. No entry should reference the linked worktree.
    let entries_after = visible_entries_as_strings(&sidebar, cx);
    assert!(
        !entries_after.iter().any(|s| s.contains("{wt-ochre-drift}")),
        "no entry should reference the archived worktree, got: {entries_after:?}"
    );

    // The main project thread should still be visible.
    assert!(
        entries_after
            .iter()
            .any(|s| s.contains("Main Project Thread")),
        "main project thread should still be visible, got: {entries_after:?}"
    );
}

// TODO: Restore this test once linked worktree draft entries are re-implemented.
// The draft-in-sidebar approach was reverted in favor of just the + button toggle.
#[gpui::test]
#[ignore = "linked worktree draft entries not yet implemented"]
async fn test_linked_worktree_workspace_reachable_and_dismissable(cx: &mut TestAppContext) {
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

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Open the linked worktree as a separate workspace (simulates cmd-o).
    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });
    add_agent_panel(&worktree_workspace, cx);
    cx.run_until_parked();

    // Explicitly create a draft thread from the linked worktree workspace.
    // Auto-created drafts use the group's first workspace (the main one),
    // so a user-created draft is needed to make the linked worktree reachable.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.create_new_thread(&worktree_workspace, window, cx);
    });
    cx.run_until_parked();

    // Switch back to the main workspace.
    multi_workspace.update_in(cx, |mw, window, cx| {
        let main_ws = mw.workspaces().next().unwrap().clone();
        mw.activate(main_ws, window, cx);
    });
    cx.run_until_parked();

    sidebar.update_in(cx, |sidebar, _window, cx| {
        sidebar.update_entries(cx);
    });
    cx.run_until_parked();

    // The linked worktree workspace must be reachable from some sidebar entry.
    let worktree_ws_id = worktree_workspace.entity_id();
    let reachable: Vec<gpui::EntityId> = sidebar.read_with(cx, |sidebar, cx| {
        let mw = multi_workspace.read(cx);
        sidebar
            .contents
            .entries
            .iter()
            .flat_map(|entry| entry.reachable_workspaces(mw, cx))
            .map(|ws| ws.entity_id())
            .collect()
    });
    assert!(
        reachable.contains(&worktree_ws_id),
        "linked worktree workspace should be reachable, but reachable are: {reachable:?}"
    );

    // Find the draft Thread entry whose workspace is the linked worktree.
    let _ = (worktree_ws_id, sidebar, multi_workspace);
    // todo("re-implement once linked worktree draft entries exist");
}

#[gpui::test]
async fn test_linked_worktree_workspace_shows_main_worktree_threads(cx: &mut TestAppContext) {
    // When only a linked worktree workspace is open (not the main repo),
    // threads saved against the main repo should still appear in the sidebar.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    // Create the main repo with a linked worktree.
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

    fs.add_linked_worktree_for_repo(
        std::path::Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "abc".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Only open the linked worktree as a workspace — NOT the main repo.
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
        MultiWorkspace::test_new(worktree_project.clone(), window, cx)
    });
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a thread against the MAIN repo path.
    save_named_thread_metadata("main-thread", "Main Repo Thread", &main_project, cx).await;

    // Save a thread against the linked worktree path.
    save_named_thread_metadata("wt-thread", "Worktree Thread", &worktree_project, cx).await;

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // Both threads should be visible: the worktree thread by direct lookup,
    // and the main repo thread because the workspace is a linked worktree
    // and we also query the main repo path.
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries.iter().any(|e| e.contains("Main Repo Thread")),
        "expected main repo thread to be visible in linked worktree workspace, got: {entries:?}"
    );
    assert!(
        entries.iter().any(|e| e.contains("Worktree Thread")),
        "expected worktree thread to be visible, got: {entries:?}"
    );
}

async fn init_multi_project_test(
    paths: &[&str],
    cx: &mut TestAppContext,
) -> (Arc<FakeFs>, Entity<project::Project>) {
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.set_global(agent_ui::MaxIdleRetainedThreads(1));
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });
    let fs = FakeFs::new(cx.executor());
    for path in paths {
        fs.insert_tree(path, serde_json::json!({ ".git": {}, "src": {} }))
            .await;
    }
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
    let project =
        project::Project::test(fs.clone() as Arc<dyn fs::Fs>, [paths[0].as_ref()], cx).await;
    (fs, project)
}

async fn add_test_project(
    path: &str,
    fs: &Arc<FakeFs>,
    multi_workspace: &Entity<MultiWorkspace>,
    cx: &mut gpui::VisualTestContext,
) -> Entity<Workspace> {
    let project = project::Project::test(fs.clone() as Arc<dyn fs::Fs>, [path.as_ref()], cx).await;
    let workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project, window, cx)
    });
    cx.run_until_parked();
    workspace
}

#[gpui::test]
async fn test_transient_workspace_lifecycle(cx: &mut TestAppContext) {
    let (fs, project_a) =
        init_multi_project_test(&["/project-a", "/project-b", "/project-c"], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
    let _sidebar = setup_sidebar_closed(&multi_workspace, cx);

    // Sidebar starts closed. Initial workspace A is transient.
    let workspace_a = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    assert!(!multi_workspace.read_with(cx, |mw, _| mw.sidebar_open()));
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1
    );
    assert!(multi_workspace.read_with(cx, |mw, _| mw.workspace() == &workspace_a));

    // Add B — replaces A as the transient workspace.
    let workspace_b = add_test_project("/project-b", &fs, &multi_workspace, cx).await;
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1
    );
    assert!(multi_workspace.read_with(cx, |mw, _| mw.workspace() == &workspace_b));

    // Add C — replaces B as the transient workspace.
    let workspace_c = add_test_project("/project-c", &fs, &multi_workspace, cx).await;
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1
    );
    assert!(multi_workspace.read_with(cx, |mw, _| mw.workspace() == &workspace_c));
}

#[gpui::test]
async fn test_transient_workspace_retained(cx: &mut TestAppContext) {
    let (fs, project_a) = init_multi_project_test(
        &["/project-a", "/project-b", "/project-c", "/project-d"],
        cx,
    )
    .await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
    let _sidebar = setup_sidebar(&multi_workspace, cx);
    assert!(multi_workspace.read_with(cx, |mw, _| mw.sidebar_open()));

    // Add B — retained since sidebar is open.
    let workspace_a = add_test_project("/project-b", &fs, &multi_workspace, cx).await;
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2
    );

    // Switch to A — B survives. (Switching from one internal workspace, to another)
    multi_workspace.update_in(cx, |mw, window, cx| mw.activate(workspace_a, window, cx));
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2
    );

    // Close sidebar — both A and B remain retained.
    multi_workspace.update_in(cx, |mw, window, cx| mw.close_sidebar(window, cx));
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2
    );

    // Add C — added as new transient workspace. (switching from retained, to transient)
    let workspace_c = add_test_project("/project-c", &fs, &multi_workspace, cx).await;
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        3
    );
    assert!(multi_workspace.read_with(cx, |mw, _| mw.workspace() == &workspace_c));

    // Add D — replaces C as the transient workspace (Have retained and transient workspaces, transient workspace is dropped)
    let workspace_d = add_test_project("/project-d", &fs, &multi_workspace, cx).await;
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        3
    );
    assert!(multi_workspace.read_with(cx, |mw, _| mw.workspace() == &workspace_d));
}

#[gpui::test]
async fn test_transient_workspace_promotion(cx: &mut TestAppContext) {
    let (fs, project_a) =
        init_multi_project_test(&["/project-a", "/project-b", "/project-c"], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
    setup_sidebar_closed(&multi_workspace, cx);

    // Add B — replaces A as the transient workspace (A is discarded).
    let workspace_b = add_test_project("/project-b", &fs, &multi_workspace, cx).await;
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1
    );
    assert!(multi_workspace.read_with(cx, |mw, _| mw.workspace() == &workspace_b));

    // Open sidebar — promotes the transient B to retained.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.toggle_sidebar(window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1
    );
    assert!(multi_workspace.read_with(cx, |mw, _| mw.workspaces().any(|w| w == &workspace_b)));

    // Close sidebar — the retained B remains.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.toggle_sidebar(window, cx);
    });

    // Add C — added as new transient workspace.
    let workspace_c = add_test_project("/project-c", &fs, &multi_workspace, cx).await;
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2
    );
    assert!(multi_workspace.read_with(cx, |mw, _| mw.workspace() == &workspace_c));
}

#[gpui::test]
async fn test_legacy_thread_with_canonical_path_opens_main_repo_workspace(cx: &mut TestAppContext) {
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

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "abc".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Only a linked worktree workspace is open — no workspace for /project.
    let worktree_project = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
        MultiWorkspace::test_new(worktree_project.clone(), window, cx)
    });
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save a legacy thread: folder_paths = main repo, main_worktree_paths = empty.
    let legacy_session = acp::SessionId::new(Arc::from("legacy-main-thread"));
    cx.update(|_, cx| {
        let metadata = ThreadMetadata {
            thread_id: ThreadId::new(),
            session_id: Some(legacy_session.clone()),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: Some("Legacy Main Thread".into()),
            updated_at: chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            created_at: None,
            worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                "/project",
            )])),
            archived: false,
            remote_connection: None,
        };
        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.save(metadata, cx));
    });
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // The legacy thread should appear in the sidebar under the project group.
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert!(
        entries.iter().any(|e| e.contains("Legacy Main Thread")),
        "legacy thread should be visible: {entries:?}",
    );

    // Verify only 1 workspace before clicking.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
    );

    // Focus and select the legacy thread, then confirm.
    focus_sidebar(&sidebar, cx);
    let thread_index = sidebar.read_with(cx, |sidebar, _| {
        sidebar
            .contents
            .entries
            .iter()
            .position(|e| e.session_id().is_some_and(|id| id == &legacy_session))
            .expect("legacy thread should be in entries")
    });
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = Some(thread_index);
    });
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    let new_workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    let new_path_list =
        new_workspace.read_with(cx, |_, cx| workspace_path_list(&new_workspace, cx));
    assert_eq!(
        new_path_list,
        PathList::new(&[PathBuf::from("/project")]),
        "the new workspace should be for the main repo, not the linked worktree",
    );
}

#[gpui::test]
async fn test_linked_worktree_workspace_reachable_after_adding_unrelated_project(
    cx: &mut TestAppContext,
) {
    // Regression test for a property-test finding:
    //   AddLinkedWorktree { project_group_index: 0 }
    //   AddProject { use_worktree: true }
    //   AddProject { use_worktree: false }
    // After these three steps, the linked-worktree workspace was not
    // reachable from any sidebar entry.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);

        cx.observe_new(
            |workspace: &mut Workspace,
             window: Option<&mut Window>,
             cx: &mut gpui::Context<Workspace>| {
                if let Some(window) = window {
                    let panel = cx.new(|cx| AgentPanel::test_new(workspace, window, cx));
                    workspace.add_panel(panel, window, cx);
                }
            },
        )
        .detach();
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
        project::Project::test(fs.clone() as Arc<dyn fs::Fs>, ["/my-project".as_ref()], cx).await;
    project.update(cx, |p, cx| p.git_scans_complete(cx)).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Step 1: Create a linked worktree for the main project.
    let worktree_name = "wt-0";
    let worktree_path = "/worktrees/wt-0";

    fs.insert_tree(
        worktree_path,
        serde_json::json!({
            ".git": "gitdir: /my-project/.git/worktrees/wt-0",
            "src": {},
        }),
    )
    .await;
    fs.insert_tree(
        "/my-project/.git/worktrees/wt-0",
        serde_json::json!({
            "commondir": "../../",
            "HEAD": "ref: refs/heads/wt-0",
        }),
    )
    .await;
    fs.add_linked_worktree_for_repo(
        Path::new("/my-project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from(worktree_path),
            ref_name: Some(format!("refs/heads/{}", worktree_name).into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    let main_workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    let main_project = main_workspace.read_with(cx, |ws, _| ws.project().clone());
    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    cx.run_until_parked();

    // Step 2: Open the linked worktree as its own workspace.
    let worktree_project =
        project::Project::test(fs.clone() as Arc<dyn fs::Fs>, [worktree_path.as_ref()], cx).await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });
    cx.run_until_parked();

    // Step 3: Add an unrelated project.
    fs.insert_tree(
        "/other-project",
        serde_json::json!({
            ".git": {},
            "src": {},
        }),
    )
    .await;
    let other_project = project::Project::test(
        fs.clone() as Arc<dyn fs::Fs>,
        ["/other-project".as_ref()],
        cx,
    )
    .await;
    other_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(other_project.clone(), window, cx);
    });
    cx.run_until_parked();

    // Force a full sidebar rebuild with all groups expanded.
    sidebar.update_in(cx, |sidebar, _window, cx| {
        if let Some(mw) = sidebar.multi_workspace.upgrade() {
            mw.update(cx, |mw, _cx| mw.test_expand_all_groups());
        }
        sidebar.update_entries(cx);
    });
    cx.run_until_parked();

    // The linked-worktree workspace must be reachable from at least one
    // sidebar entry — otherwise the user has no way to navigate to it.
    let worktree_ws_id = worktree_workspace.entity_id();
    let (all_ids, reachable_ids) = sidebar.read_with(cx, |sidebar, cx| {
        let mw = multi_workspace.read(cx);

        let all: HashSet<gpui::EntityId> = mw.workspaces().map(|ws| ws.entity_id()).collect();
        let reachable: HashSet<gpui::EntityId> = sidebar
            .contents
            .entries
            .iter()
            .flat_map(|entry| entry.reachable_workspaces(mw, cx))
            .map(|ws| ws.entity_id())
            .collect();
        (all, reachable)
    });

    let unreachable = &all_ids - &reachable_ids;
    eprintln!("{}", visible_entries_as_strings(&sidebar, cx).join("\n"));

    assert!(
        unreachable.is_empty(),
        "workspaces not reachable from any sidebar entry: {:?}\n\
         (linked-worktree workspace id: {:?})",
        unreachable,
        worktree_ws_id,
    );
}

#[gpui::test]
async fn test_startup_failed_restoration_shows_no_draft(cx: &mut TestAppContext) {
    // Empty project groups no longer auto-create drafts via reconciliation.
    // A fresh startup with no restorable thread should show only the header.
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, _panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    let _workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(
        entries,
        vec!["v [my-project]"],
        "empty group should show only the header, no auto-created draft"
    );
}

#[gpui::test]
async fn test_startup_successful_restoration_no_spurious_draft(cx: &mut TestAppContext) {
    // Rule 5: When the app starts and the AgentPanel successfully loads
    // a thread, no spurious draft should appear.
    let project = init_test_project_with_agent_panel("/my-project", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    // Create and send a message to make a real thread.
    let connection = StubAgentConnection::new();
    connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel, connection, cx);
    send_message(&panel, cx);
    let session_id = active_session_id(&panel, cx);
    save_test_thread_metadata(&session_id, &project, cx).await;
    cx.run_until_parked();

    // Should show the thread, NOT a spurious draft.
    let entries = visible_entries_as_strings(&sidebar, cx);
    assert_eq!(entries, vec!["v [my-project]", "  Hello *"]);

    // active_entry should be Thread, not Draft.
    sidebar.read_with(cx, |sidebar, _| {
        assert_active_thread(sidebar, &session_id, "should be on the thread, not a draft");
    });
}

#[gpui::test]
async fn test_project_header_click_restores_last_viewed(cx: &mut TestAppContext) {
    // Rule 9: Clicking a project header should restore whatever the
    // user was last looking at in that group, not create new drafts
    // or jump to the first entry.
    let project_a = init_test_project_with_agent_panel("/project-a", cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, cx);

    // Create two threads in project-a.
    let conn1 = StubAgentConnection::new();
    conn1.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel_a, conn1, cx);
    send_message(&panel_a, cx);
    let thread_a1 = active_session_id(&panel_a, cx);
    save_test_thread_metadata(&thread_a1, &project_a, cx).await;

    let conn2 = StubAgentConnection::new();
    conn2.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
        acp::ContentChunk::new("Done".into()),
    )]);
    open_thread_with_connection(&panel_a, conn2, cx);
    send_message(&panel_a, cx);
    let thread_a2 = active_session_id(&panel_a, cx);
    save_test_thread_metadata(&thread_a2, &project_a, cx).await;
    cx.run_until_parked();

    // The user is now looking at thread_a2.
    sidebar.read_with(cx, |sidebar, _| {
        assert_active_thread(sidebar, &thread_a2, "should be on thread_a2");
    });

    // Add project-b and switch to it.
    let fs = cx.update(|_window, cx| <dyn fs::Fs>::global(cx));
    fs.as_fake()
        .insert_tree("/project-b", serde_json::json!({ "src": {} }))
        .await;
    let project_b =
        project::Project::test(fs.clone() as Arc<dyn Fs>, ["/project-b".as_ref()], cx).await;
    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx)
    });
    let _panel_b = add_agent_panel(&workspace_b, cx);
    cx.run_until_parked();

    // Now switch BACK to project-a by activating its workspace.
    let workspace_a = multi_workspace.read_with(cx, |mw, cx| {
        mw.workspaces()
            .find(|ws| {
                ws.read(cx)
                    .project()
                    .read(cx)
                    .visible_worktrees(cx)
                    .any(|wt| {
                        wt.read(cx)
                            .abs_path()
                            .to_string_lossy()
                            .contains("project-a")
                    })
            })
            .unwrap()
            .clone()
    });
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.activate(workspace_a.clone(), window, cx);
    });
    cx.run_until_parked();

    // The panel should still show thread_a2 (the last thing the user
    // was viewing in project-a), not a draft or thread_a1.
    sidebar.read_with(cx, |sidebar, _| {
        assert_active_thread(
            sidebar,
            &thread_a2,
            "switching back to project-a should restore thread_a2",
        );
    });

    // No spurious draft entries should have been created in
    // project-a's group (project-b may have a placeholder).
    let entries = visible_entries_as_strings(&sidebar, cx);
    // Find project-a's section and check it has no drafts.
    let project_a_start = entries
        .iter()
        .position(|e| e.contains("project-a"))
        .unwrap();
    let project_a_end = entries[project_a_start + 1..]
        .iter()
        .position(|e| e.starts_with("v "))
        .map(|i| i + project_a_start + 1)
        .unwrap_or(entries.len());
    let project_a_drafts = entries[project_a_start..project_a_end]
        .iter()
        .filter(|e| e.contains("Draft"))
        .count();
    assert_eq!(
        project_a_drafts, 0,
        "switching back to project-a should not create drafts in its group"
    );
}

#[gpui::test]
async fn test_activating_workspace_with_draft_does_not_create_extras(cx: &mut TestAppContext) {
    // When a workspace has a draft (from the panel's load fallback)
    // and the user activates it (e.g. by clicking the placeholder or
    // the project header), no extra drafts should be created.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project-a", serde_json::json!({ ".git": {}, "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ ".git": {}, "src": {} }))
        .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    let project_a =
        project::Project::test(fs.clone() as Arc<dyn Fs>, ["/project-a".as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);
    let workspace_a = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    let _panel_a = add_agent_panel(&workspace_a, cx);
    cx.run_until_parked();

    // Add project-b with its own workspace and agent panel.
    let project_b =
        project::Project::test(fs.clone() as Arc<dyn Fs>, ["/project-b".as_ref()], cx).await;
    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_b.clone(), window, cx)
    });
    let _panel_b = add_agent_panel(&workspace_b, cx);
    cx.run_until_parked();

    // Explicitly create a draft on workspace_b so the sidebar tracks one.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.create_new_thread(&workspace_b, window, cx);
    });
    cx.run_until_parked();

    // Count project-b's drafts.
    let count_b_drafts = |cx: &mut gpui::VisualTestContext| {
        let entries = visible_entries_as_strings(&sidebar, cx);
        entries
            .iter()
            .skip_while(|e| !e.contains("project-b"))
            .take_while(|e| !e.starts_with("v ") || e.contains("project-b"))
            .filter(|e| e.contains("Draft"))
            .count()
    };
    let drafts_before = count_b_drafts(cx);

    // Switch away from project-b, then back.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.activate(workspace_a.clone(), window, cx);
    });
    cx.run_until_parked();
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.activate(workspace_b.clone(), window, cx);
    });
    cx.run_until_parked();

    let drafts_after = count_b_drafts(cx);
    assert_eq!(
        drafts_before, drafts_after,
        "activating workspace should not create extra drafts"
    );

    // The draft should be highlighted as active after switching back.
    sidebar.read_with(cx, |sidebar, _| {
        assert_active_draft(
            sidebar,
            &workspace_b,
            "draft should be active after switching back to its workspace",
        );
    });
}

#[gpui::test]
async fn test_non_archive_thread_paths_migrate_on_worktree_add_and_remove(cx: &mut TestAppContext) {
    // Historical threads (not open in any agent panel) should have their
    // worktree paths updated when a folder is added to or removed from the
    // project.
    let (_fs, project) = init_multi_project_test(&["/project-a", "/project-b"], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Save two threads directly into the metadata store (not via the agent
    // panel), so they are purely historical — no open views hold them.
    // Use different timestamps so sort order is deterministic.
    save_thread_metadata(
        acp::SessionId::new(Arc::from("hist-1")),
        Some("Historical 1".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );
    save_thread_metadata(
        acp::SessionId::new(Arc::from("hist-2")),
        Some("Historical 2".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 1).unwrap(),
        None,
        &project,
        cx,
    );
    cx.run_until_parked();
    sidebar.update_in(cx, |sidebar, _window, cx| sidebar.update_entries(cx));
    cx.run_until_parked();

    // Sanity-check: both threads exist under the initial key [/project-a].
    let old_key_paths = PathList::new(&[PathBuf::from("/project-a")]);
    cx.update(|_window, cx| {
        let store = ThreadMetadataStore::global(cx).read(cx);
        assert_eq!(
            store
                .entries_for_main_worktree_path(&old_key_paths, None)
                .count(),
            2,
            "should have 2 historical threads under old key before worktree add"
        );
    });

    // Add a second worktree to the project.
    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/project-b", true, cx)
        })
        .await
        .expect("should add worktree");
    cx.run_until_parked();

    // The historical threads should now be indexed under the new combined
    // key [/project-a, /project-b].
    let new_key_paths = PathList::new(&[PathBuf::from("/project-a"), PathBuf::from("/project-b")]);
    cx.update(|_window, cx| {
        let store = ThreadMetadataStore::global(cx).read(cx);
        assert_eq!(
            store
                .entries_for_main_worktree_path(&old_key_paths, None)
                .count(),
            0,
            "should have 0 historical threads under old key after worktree add"
        );
        assert_eq!(
            store
                .entries_for_main_worktree_path(&new_key_paths, None)
                .count(),
            2,
            "should have 2 historical threads under new key after worktree add"
        );
    });

    // Sidebar should show threads under the new header.
    sidebar.update_in(cx, |sidebar, _window, cx| sidebar.update_entries(cx));
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project-a, project-b]",
            "  Historical 2",
            "  Historical 1",
        ]
    );

    // Now remove the second worktree.
    let worktree_id = project.read_with(cx, |project, cx| {
        project
            .visible_worktrees(cx)
            .find(|wt| wt.read(cx).abs_path().as_ref() == Path::new("/project-b"))
            .map(|wt| wt.read(cx).id())
            .expect("should find project-b worktree")
    });
    project.update(cx, |project, cx| {
        project.remove_worktree(worktree_id, cx);
    });
    cx.run_until_parked();

    // Historical threads should migrate back to the original key.
    cx.update(|_window, cx| {
        let store = ThreadMetadataStore::global(cx).read(cx);
        assert_eq!(
            store
                .entries_for_main_worktree_path(&new_key_paths, None)
                .count(),
            0,
            "should have 0 historical threads under new key after worktree remove"
        );
        assert_eq!(
            store
                .entries_for_main_worktree_path(&old_key_paths, None)
                .count(),
            2,
            "should have 2 historical threads under old key after worktree remove"
        );
    });

    sidebar.update_in(cx, |sidebar, _window, cx| sidebar.update_entries(cx));
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec!["v [project-a]", "  Historical 2", "  Historical 1",]
    );
}

#[gpui::test]
async fn test_worktree_add_only_regroups_threads_for_changed_workspace(cx: &mut TestAppContext) {
    // When two workspaces share the same project group (same main path)
    // but have different folder paths (main repo vs linked worktree),
    // adding a worktree to the main workspace should regroup only that
    // workspace and its threads into the new project group. Threads for the
    // linked worktree workspace should remain under the original group.
    agent_ui::test_support::init_test(cx);
    cx.update(|cx| {
        cx.set_global(agent_ui::MaxIdleRetainedThreads(1));
        ThreadStore::init_global(cx);
        ThreadMetadataStore::init_global(cx);
        language_model::LanguageModelRegistry::test(cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project", serde_json::json!({ ".git": {}, "src": {} }))
        .await;
    fs.insert_tree("/project-b", serde_json::json!({ ".git": {}, "src": {} }))
        .await;
    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: std::path::PathBuf::from("/wt-feature"),
            ref_name: Some("refs/heads/feature".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Workspace A: main repo at /project.
    let main_project =
        project::Project::test(fs.clone() as Arc<dyn fs::Fs>, ["/project".as_ref()], cx).await;
    // Workspace B: linked worktree of the same repo (same group, different folder).
    let worktree_project =
        project::Project::test(fs.clone() as Arc<dyn fs::Fs>, ["/wt-feature".as_ref()], cx).await;

    main_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(main_project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx);
    });
    cx.run_until_parked();

    // Save a thread for each workspace's folder paths.
    let time_main = chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 1).unwrap();
    let time_wt = chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 2).unwrap();
    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-main")),
        Some("Main Thread".into()),
        time_main,
        Some(time_main),
        &main_project,
        cx,
    );
    save_thread_metadata(
        acp::SessionId::new(Arc::from("thread-wt")),
        Some("Worktree Thread".into()),
        time_wt,
        Some(time_wt),
        &worktree_project,
        cx,
    );
    cx.run_until_parked();

    let folder_paths_main = PathList::new(&[PathBuf::from("/project")]);
    let folder_paths_wt = PathList::new(&[PathBuf::from("/wt-feature")]);

    // Sanity-check: each thread is indexed under its own folder paths, but
    // both appear under the shared sidebar group keyed by the main worktree.
    cx.update(|_window, cx| {
        let store = ThreadMetadataStore::global(cx).read(cx);
        assert_eq!(
            store.entries_for_path(&folder_paths_main, None).count(),
            1,
            "one thread under [/project]"
        );
        assert_eq!(
            store.entries_for_path(&folder_paths_wt, None).count(),
            1,
            "one thread under [/wt-feature]"
        );
    });
    sidebar.update_in(cx, |sidebar, _window, cx| sidebar.update_entries(cx));
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  Worktree Thread {wt-feature}",
            "  Main Thread",
        ]
    );

    // Add /project-b to the main project only.
    main_project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/project-b", true, cx)
        })
        .await
        .expect("should add worktree");
    cx.run_until_parked();

    // Main Thread (folder paths [/project]) should be regrouped to
    // [/project, /project-b]. Worktree Thread should remain under the
    // original [/project] group.
    let folder_paths_main_b =
        PathList::new(&[PathBuf::from("/project"), PathBuf::from("/project-b")]);
    cx.update(|_window, cx| {
        let store = ThreadMetadataStore::global(cx).read(cx);
        assert_eq!(
            store.entries_for_path(&folder_paths_main, None).count(),
            0,
            "main thread should no longer be under old folder paths [/project]"
        );
        assert_eq!(
            store.entries_for_path(&folder_paths_main_b, None).count(),
            1,
            "main thread should now be under [/project, /project-b]"
        );
        assert_eq!(
            store.entries_for_path(&folder_paths_wt, None).count(),
            1,
            "worktree thread should remain unchanged under [/wt-feature]"
        );
    });

    sidebar.update_in(cx, |sidebar, _window, cx| sidebar.update_entries(cx));
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&sidebar, cx),
        vec![
            "v [project]",
            "  Worktree Thread {wt-feature}",
            "v [project, project-b]",
            "  Main Thread",
        ]
    );
}

#[gpui::test]
async fn test_linked_worktree_workspace_reachable_after_adding_worktree_to_project(
    cx: &mut TestAppContext,
) {
    // When a linked worktree is opened as its own workspace and then a new
    // folder is added to the main project group, the linked worktree
    // workspace must still be reachable from some sidebar entry.
    let (_fs, project) = init_multi_project_test(&["/my-project"], cx).await;
    let fs = _fs.clone();

    // Set up git worktree infrastructure.
    fs.insert_tree(
        "/my-project/.git/worktrees/wt-0",
        serde_json::json!({
            "commondir": "../../",
            "HEAD": "ref: refs/heads/wt-0",
        }),
    )
    .await;
    fs.insert_tree(
        "/worktrees/wt-0",
        serde_json::json!({
            ".git": "gitdir: /my-project/.git/worktrees/wt-0",
            "src": {},
        }),
    )
    .await;
    fs.add_linked_worktree_for_repo(
        Path::new("/my-project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/worktrees/wt-0"),
            ref_name: Some("refs/heads/wt-0".into()),
            sha: "aaa".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    // Re-scan so the main project discovers the linked worktree.
    project.update(cx, |p, cx| p.git_scans_complete(cx)).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Open the linked worktree as its own workspace.
    let worktree_project = project::Project::test(
        fs.clone() as Arc<dyn fs::Fs>,
        ["/worktrees/wt-0".as_ref()],
        cx,
    )
    .await;
    worktree_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx);
    });
    cx.run_until_parked();

    // Both workspaces should be reachable.
    let workspace_count = multi_workspace.read_with(cx, |mw, _| mw.workspaces().count());
    assert_eq!(workspace_count, 2, "should have 2 workspaces");

    // Add a new folder to the main project, changing the project group key.
    fs.insert_tree(
        "/other-project",
        serde_json::json!({ ".git": {}, "src": {} }),
    )
    .await;
    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree("/other-project", true, cx)
        })
        .await
        .expect("should add worktree");
    cx.run_until_parked();

    sidebar.update_in(cx, |sidebar, _window, cx| sidebar.update_entries(cx));
    cx.run_until_parked();

    // The linked worktree workspace must still be reachable.
    let entries = visible_entries_as_strings(&sidebar, cx);
    let mw_workspaces: Vec<_> = multi_workspace.read_with(cx, |mw, _| {
        mw.workspaces().map(|ws| ws.entity_id()).collect()
    });
    sidebar.read_with(cx, |sidebar, cx| {
        let multi_workspace = multi_workspace.read(cx);
        let reachable: std::collections::HashSet<gpui::EntityId> = sidebar
            .contents
            .entries
            .iter()
            .flat_map(|entry| entry.reachable_workspaces(multi_workspace, cx))
            .map(|ws| ws.entity_id())
            .collect();
        let all: std::collections::HashSet<gpui::EntityId> =
            mw_workspaces.iter().copied().collect();
        let unreachable = &all - &reachable;
        assert!(
            unreachable.is_empty(),
            "all workspaces should be reachable after adding folder; \
             unreachable: {:?}, entries: {:?}",
            unreachable,
            entries,
        );
    });
}

mod property_test {
    use super::*;
    use gpui::proptest::prelude::*;

    struct UnopenedWorktree {
        path: String,
        main_workspace_path: String,
    }

    struct TestState {
        fs: Arc<FakeFs>,
        thread_counter: u32,
        workspace_counter: u32,
        worktree_counter: u32,
        saved_thread_ids: Vec<acp::SessionId>,
        unopened_worktrees: Vec<UnopenedWorktree>,
    }

    impl TestState {
        fn new(fs: Arc<FakeFs>) -> Self {
            Self {
                fs,
                thread_counter: 0,
                workspace_counter: 1,
                worktree_counter: 0,
                saved_thread_ids: Vec::new(),
                unopened_worktrees: Vec::new(),
            }
        }

        fn next_metadata_only_thread_id(&mut self) -> acp::SessionId {
            let id = self.thread_counter;
            self.thread_counter += 1;
            acp::SessionId::new(Arc::from(format!("prop-thread-{id}")))
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
        SaveThread { project_group_index: usize },
        SaveWorktreeThread { worktree_index: usize },
        ToggleAgentPanel,
        CreateDraftThread,
        AddProject { use_worktree: bool },
        ArchiveThread { index: usize },
        SwitchToThread { index: usize },
        SwitchToProjectGroup { index: usize },
        AddLinkedWorktree { project_group_index: usize },
        AddWorktreeToProject { project_group_index: usize },
        RemoveWorktreeFromProject { project_group_index: usize },
    }

    // Distribution (out of 24 slots):
    //   SaveThread:                5 slots (~21%)
    //   SaveWorktreeThread:        2 slots (~8%)
    //   ToggleAgentPanel:          1 slot  (~4%)
    //   CreateDraftThread:         1 slot  (~4%)
    //   AddProject:                1 slot  (~4%)
    //   ArchiveThread:             2 slots (~8%)
    //   SwitchToThread:            2 slots (~8%)
    //   SwitchToProjectGroup:      2 slots (~8%)
    //   AddLinkedWorktree:         4 slots (~17%)
    //   AddWorktreeToProject:      2 slots (~8%)
    //   RemoveWorktreeFromProject: 2 slots (~8%)
    const DISTRIBUTION_SLOTS: u32 = 24;

    impl TestState {
        fn generate_operation(&self, raw: u32, project_group_count: usize) -> Operation {
            let extra = (raw / DISTRIBUTION_SLOTS) as usize;

            match raw % DISTRIBUTION_SLOTS {
                0..=4 => Operation::SaveThread {
                    project_group_index: extra % project_group_count,
                },
                5..=6 if !self.unopened_worktrees.is_empty() => Operation::SaveWorktreeThread {
                    worktree_index: extra % self.unopened_worktrees.len(),
                },
                5..=6 => Operation::SaveThread {
                    project_group_index: extra % project_group_count,
                },
                7 => Operation::ToggleAgentPanel,
                8 => Operation::CreateDraftThread,
                9 => Operation::AddProject {
                    use_worktree: !self.unopened_worktrees.is_empty(),
                },
                10..=11 if !self.saved_thread_ids.is_empty() => Operation::ArchiveThread {
                    index: extra % self.saved_thread_ids.len(),
                },
                10..=11 => Operation::AddProject {
                    use_worktree: !self.unopened_worktrees.is_empty(),
                },
                12..=13 if !self.saved_thread_ids.is_empty() => Operation::SwitchToThread {
                    index: extra % self.saved_thread_ids.len(),
                },
                12..=13 => Operation::SwitchToProjectGroup {
                    index: extra % project_group_count,
                },
                14..=15 => Operation::SwitchToProjectGroup {
                    index: extra % project_group_count,
                },
                16..=19 if project_group_count > 0 => Operation::AddLinkedWorktree {
                    project_group_index: extra % project_group_count,
                },
                16..=19 => Operation::SaveThread {
                    project_group_index: extra % project_group_count,
                },
                20..=21 if project_group_count > 0 => Operation::AddWorktreeToProject {
                    project_group_index: extra % project_group_count,
                },
                20..=21 => Operation::SaveThread {
                    project_group_index: extra % project_group_count,
                },
                22..=23 if project_group_count > 0 => Operation::RemoveWorktreeFromProject {
                    project_group_index: extra % project_group_count,
                },
                22..=23 => Operation::SaveThread {
                    project_group_index: extra % project_group_count,
                },
                _ => unreachable!(),
            }
        }
    }

    fn save_thread_to_path_with_main(
        state: &mut TestState,
        path_list: PathList,
        main_worktree_paths: PathList,
        cx: &mut gpui::VisualTestContext,
    ) {
        let session_id = state.next_metadata_only_thread_id();
        let title: SharedString = format!("Thread {}", session_id).into();
        let updated_at = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2024, 1, 1, 0, 0, 0)
            .unwrap()
            + chrono::Duration::seconds(state.thread_counter as i64);
        let metadata = ThreadMetadata {
            thread_id: ThreadId::new(),
            session_id: Some(session_id),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: Some(title),
            updated_at,
            created_at: None,
            worktree_paths: WorktreePaths::from_path_lists(main_worktree_paths, path_list).unwrap(),
            archived: false,
            remote_connection: None,
        };
        cx.update(|_, cx| {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| store.save(metadata, cx))
        });
        cx.run_until_parked();
    }

    async fn perform_operation(
        operation: Operation,
        state: &mut TestState,
        multi_workspace: &Entity<MultiWorkspace>,
        sidebar: &Entity<Sidebar>,
        cx: &mut gpui::VisualTestContext,
    ) {
        match operation {
            Operation::SaveThread {
                project_group_index,
            } => {
                // Find a workspace for this project group and create a real
                // thread via its agent panel.
                let (workspace, project) = multi_workspace.read_with(cx, |mw, cx| {
                    let keys = mw.project_group_keys();
                    let key = &keys[project_group_index];
                    let ws = mw
                        .workspaces_for_project_group(key, cx)
                        .and_then(|ws| ws.first().cloned())
                        .unwrap_or_else(|| mw.workspace().clone());
                    let project = ws.read(cx).project().clone();
                    (ws, project)
                });

                let panel =
                    workspace.read_with(cx, |workspace, cx| workspace.panel::<AgentPanel>(cx));
                if let Some(panel) = panel {
                    let connection = StubAgentConnection::new();
                    connection.set_next_prompt_updates(vec![
                        acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                            "Done".into(),
                        )),
                    ]);
                    open_thread_with_connection(&panel, connection, cx);
                    send_message(&panel, cx);
                    let session_id = active_session_id(&panel, cx);
                    state.saved_thread_ids.push(session_id.clone());

                    let title: SharedString = format!("Thread {}", state.thread_counter).into();
                    state.thread_counter += 1;
                    let updated_at =
                        chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2024, 1, 1, 0, 0, 0)
                            .unwrap()
                            + chrono::Duration::seconds(state.thread_counter as i64);
                    save_thread_metadata(session_id, Some(title), updated_at, None, &project, cx);
                }
            }
            Operation::SaveWorktreeThread { worktree_index } => {
                let worktree = &state.unopened_worktrees[worktree_index];
                let path_list = PathList::new(&[std::path::PathBuf::from(&worktree.path)]);
                let main_worktree_paths =
                    PathList::new(&[std::path::PathBuf::from(&worktree.main_workspace_path)]);
                save_thread_to_path_with_main(state, path_list, main_worktree_paths, cx);
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
                    panel.update_in(cx, |panel, window, cx| {
                        panel.new_thread(&NewThread, window, cx);
                    });
                    cx.run_until_parked();
                }
                workspace.update_in(cx, |workspace, window, cx| {
                    workspace.focus_panel::<AgentPanel>(window, cx);
                });
            }
            Operation::AddProject { use_worktree } => {
                let path = if use_worktree {
                    // Open an existing linked worktree as a project (simulates Cmd+O
                    // on a worktree directory).
                    state.unopened_worktrees.remove(0).path
                } else {
                    // Create a brand new project.
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
                    path
                };
                let project = project::Project::test(
                    state.fs.clone() as Arc<dyn fs::Fs>,
                    [path.as_ref()],
                    cx,
                )
                .await;
                project.update(cx, |p, cx| p.git_scans_complete(cx)).await;
                multi_workspace.update_in(cx, |mw, window, cx| {
                    mw.test_add_workspace(project.clone(), window, cx)
                });
            }

            Operation::ArchiveThread { index } => {
                let session_id = state.saved_thread_ids[index].clone();
                sidebar.update_in(cx, |sidebar: &mut Sidebar, window, cx| {
                    sidebar.archive_thread(&session_id, window, cx);
                });
                cx.run_until_parked();
                state.saved_thread_ids.remove(index);
            }
            Operation::SwitchToThread { index } => {
                let session_id = state.saved_thread_ids[index].clone();
                // Find the thread's position in the sidebar entries and select it.
                let thread_index = sidebar.read_with(cx, |sidebar, _| {
                    sidebar.contents.entries.iter().position(|entry| {
                        matches!(
                            entry,
                            ListEntry::Thread(t) if t.metadata.session_id.as_ref() == Some(&session_id)
                        )
                    })
                });
                if let Some(ix) = thread_index {
                    sidebar.update_in(cx, |sidebar, window, cx| {
                        sidebar.selection = Some(ix);
                        sidebar.confirm(&Confirm, window, cx);
                    });
                    cx.run_until_parked();
                }
            }
            Operation::SwitchToProjectGroup { index } => {
                let workspace = multi_workspace.read_with(cx, |mw, cx| {
                    let keys = mw.project_group_keys();
                    let key = &keys[index];
                    mw.workspaces_for_project_group(key, cx)
                        .and_then(|ws| ws.first().cloned())
                        .unwrap_or_else(|| mw.workspace().clone())
                });
                multi_workspace.update_in(cx, |mw, window, cx| {
                    mw.activate(workspace, window, cx);
                });
            }
            Operation::AddLinkedWorktree {
                project_group_index,
            } => {
                // Get the main worktree path from the project group key.
                let main_path = multi_workspace.read_with(cx, |mw, _| {
                    let keys = mw.project_group_keys();
                    let key = &keys[project_group_index];
                    key.path_list()
                        .paths()
                        .first()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                });
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
                    .add_linked_worktree_for_repo(
                        dot_git_path,
                        false,
                        git::repository::Worktree {
                            path: worktree_pathbuf,
                            ref_name: Some(format!("refs/heads/{}", worktree_name).into()),
                            sha: "aaa".into(),
                            is_main: false,
                            is_bare: false,
                        },
                    )
                    .await;

                // Re-scan the main workspace's project so it discovers the new worktree.
                let main_workspace = multi_workspace.read_with(cx, |mw, cx| {
                    let keys = mw.project_group_keys();
                    let key = &keys[project_group_index];
                    mw.workspaces_for_project_group(key, cx)
                        .and_then(|ws| ws.first().cloned())
                        .unwrap()
                });
                let main_project = main_workspace.read_with(cx, |ws, _| ws.project().clone());
                main_project
                    .update(cx, |p, cx| p.git_scans_complete(cx))
                    .await;

                state.unopened_worktrees.push(UnopenedWorktree {
                    path: worktree_path,
                    main_workspace_path: main_path.clone(),
                });
            }
            Operation::AddWorktreeToProject {
                project_group_index,
            } => {
                let workspace = multi_workspace.read_with(cx, |mw, cx| {
                    let keys = mw.project_group_keys();
                    let key = &keys[project_group_index];
                    mw.workspaces_for_project_group(key, cx)
                        .and_then(|ws| ws.first().cloned())
                });
                let Some(workspace) = workspace else { return };
                let project = workspace.read_with(cx, |ws, _| ws.project().clone());

                let new_path = state.next_workspace_path();
                state
                    .fs
                    .insert_tree(&new_path, serde_json::json!({ ".git": {}, "src": {} }))
                    .await;

                let result = project
                    .update(cx, |project, cx| {
                        project.find_or_create_worktree(&new_path, true, cx)
                    })
                    .await;
                if result.is_err() {
                    return;
                }
                cx.run_until_parked();
            }
            Operation::RemoveWorktreeFromProject {
                project_group_index,
            } => {
                let workspace = multi_workspace.read_with(cx, |mw, cx| {
                    let keys = mw.project_group_keys();
                    let key = &keys[project_group_index];
                    mw.workspaces_for_project_group(key, cx)
                        .and_then(|ws| ws.first().cloned())
                });
                let Some(workspace) = workspace else { return };
                let project = workspace.read_with(cx, |ws, _| ws.project().clone());

                let worktree_count = project.read_with(cx, |p, cx| p.visible_worktrees(cx).count());
                if worktree_count <= 1 {
                    return;
                }

                let worktree_id = project.read_with(cx, |p, cx| {
                    p.visible_worktrees(cx).last().map(|wt| wt.read(cx).id())
                });
                if let Some(worktree_id) = worktree_id {
                    project.update(cx, |project, cx| {
                        project.remove_worktree(worktree_id, cx);
                    });
                    cx.run_until_parked();
                }
            }
        }
    }

    fn update_sidebar(sidebar: &Entity<Sidebar>, cx: &mut gpui::VisualTestContext) {
        sidebar.update_in(cx, |sidebar, _window, cx| {
            if let Some(mw) = sidebar.multi_workspace.upgrade() {
                mw.update(cx, |mw, _cx| mw.test_expand_all_groups());
            }
            sidebar.update_entries(cx);
        });
    }

    fn validate_sidebar_properties(sidebar: &Sidebar, cx: &App) -> anyhow::Result<()> {
        verify_every_group_in_multiworkspace_is_shown(sidebar, cx)?;
        verify_no_duplicate_threads(sidebar)?;
        verify_all_threads_are_shown(sidebar, cx)?;
        verify_active_state_matches_current_workspace(sidebar, cx)?;
        verify_all_workspaces_are_reachable(sidebar, cx)?;
        verify_workspace_group_key_integrity(sidebar, cx)?;
        Ok(())
    }

    fn verify_no_duplicate_threads(sidebar: &Sidebar) -> anyhow::Result<()> {
        let mut seen: HashSet<acp::SessionId> = HashSet::default();
        let mut duplicates: Vec<(acp::SessionId, String)> = Vec::new();

        for entry in &sidebar.contents.entries {
            if let Some(session_id) = entry.session_id() {
                if !seen.insert(session_id.clone()) {
                    let title = match entry {
                        ListEntry::Thread(thread) => thread.metadata.display_title().to_string(),
                        _ => "<unknown>".to_string(),
                    };
                    duplicates.push((session_id.clone(), title));
                }
            }
        }

        anyhow::ensure!(
            duplicates.is_empty(),
            "threads appear more than once in sidebar: {:?}",
            duplicates,
        );
        Ok(())
    }

    fn verify_every_group_in_multiworkspace_is_shown(
        sidebar: &Sidebar,
        cx: &App,
    ) -> anyhow::Result<()> {
        let Some(multi_workspace) = sidebar.multi_workspace.upgrade() else {
            anyhow::bail!("sidebar should still have an associated multi-workspace");
        };

        let mw = multi_workspace.read(cx);

        // Every project group key in the multi-workspace that has a
        // non-empty path list should appear as a ProjectHeader in the
        // sidebar.
        let all_keys = mw.project_group_keys();
        let expected_keys: HashSet<&ProjectGroupKey> = all_keys
            .iter()
            .filter(|k| !k.path_list().paths().is_empty())
            .collect();

        let sidebar_keys: HashSet<&ProjectGroupKey> = sidebar
            .contents
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::ProjectHeader { key, .. } => Some(key),
                _ => None,
            })
            .collect();

        let missing = &expected_keys - &sidebar_keys;
        let stray = &sidebar_keys - &expected_keys;

        anyhow::ensure!(
            missing.is_empty() && stray.is_empty(),
            "sidebar project groups don't match multi-workspace.\n\
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
        let workspaces = multi_workspace
            .read(cx)
            .workspaces()
            .cloned()
            .collect::<Vec<_>>();
        let thread_store = ThreadMetadataStore::global(cx);

        let sidebar_thread_ids: HashSet<acp::SessionId> = sidebar
            .contents
            .entries
            .iter()
            .filter_map(|entry| entry.session_id().cloned())
            .collect();

        let mut metadata_thread_ids: HashSet<acp::SessionId> = HashSet::default();

        // Query using the same approach as the sidebar: iterate project
        // group keys, then do main + legacy queries per group.
        let mw = multi_workspace.read(cx);
        let mut workspaces_by_group: HashMap<ProjectGroupKey, Vec<Entity<Workspace>>> =
            HashMap::default();
        for workspace in &workspaces {
            let key = workspace.read(cx).project_group_key(cx);
            workspaces_by_group
                .entry(key)
                .or_default()
                .push(workspace.clone());
        }

        for group_key in mw.project_group_keys() {
            let path_list = group_key.path_list().clone();
            if path_list.paths().is_empty() {
                continue;
            }

            let group_workspaces = workspaces_by_group
                .get(&group_key)
                .map(|ws| ws.as_slice())
                .unwrap_or_default();

            // Main code path queries (run for all groups, even without workspaces).
            // Skip drafts (session_id: None) — they are not shown in the
            // sidebar entries.
            for metadata in thread_store
                .read(cx)
                .entries_for_main_worktree_path(&path_list, None)
            {
                if let Some(sid) = metadata.session_id.clone() {
                    metadata_thread_ids.insert(sid);
                }
            }
            for metadata in thread_store.read(cx).entries_for_path(&path_list, None) {
                if let Some(sid) = metadata.session_id.clone() {
                    metadata_thread_ids.insert(sid);
                }
            }

            // Legacy: per-workspace queries for different root paths.
            let covered_paths: HashSet<std::path::PathBuf> = group_workspaces
                .iter()
                .flat_map(|ws| {
                    ws.read(cx)
                        .root_paths(cx)
                        .into_iter()
                        .map(|p| p.to_path_buf())
                })
                .collect();

            for workspace in group_workspaces {
                let ws_path_list = workspace_path_list(workspace, cx);
                if ws_path_list != path_list {
                    for metadata in thread_store.read(cx).entries_for_path(&ws_path_list, None) {
                        if let Some(sid) = metadata.session_id.clone() {
                            metadata_thread_ids.insert(sid);
                        }
                    }
                }
            }

            for workspace in group_workspaces {
                for snapshot in root_repository_snapshots(workspace, cx) {
                    let repo_path_list =
                        PathList::new(&[snapshot.original_repo_abs_path.to_path_buf()]);
                    if repo_path_list != path_list {
                        continue;
                    }
                    for linked_worktree in snapshot.linked_worktrees() {
                        if covered_paths.contains(&*linked_worktree.path) {
                            continue;
                        }
                        let worktree_path_list =
                            PathList::new(std::slice::from_ref(&linked_worktree.path));
                        for metadata in thread_store
                            .read(cx)
                            .entries_for_path(&worktree_path_list, None)
                        {
                            if let Some(sid) = metadata.session_id.clone() {
                                metadata_thread_ids.insert(sid);
                            }
                        }
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

        // 1. active_entry should be Some when the panel has content.
        //    It may be None when the panel is uninitialized (no drafts,
        //    no threads), which is fine.
        //    It may also temporarily point at a different workspace
        //    when the workspace just changed and the new panel has no
        //    content yet.
        let panel = active_workspace.read(cx).panel::<AgentPanel>(cx).unwrap();
        let panel_has_content = panel.read(cx).active_thread_id(cx).is_some()
            || panel.read(cx).active_conversation_view().is_some();

        let Some(entry) = sidebar.active_entry.as_ref() else {
            if panel_has_content {
                anyhow::bail!("active_entry is None but panel has content (draft or thread)");
            }
            return Ok(());
        };

        // If the entry workspace doesn't match the active workspace
        // and the panel has no content, this is a transient state that
        // will resolve when the panel gets content.
        if entry.workspace().entity_id() != active_workspace.entity_id() && !panel_has_content {
            return Ok(());
        }

        // 2. The entry's workspace must agree with the multi-workspace's
        //    active workspace.
        anyhow::ensure!(
            entry.workspace().entity_id() == active_workspace.entity_id(),
            "active_entry workspace ({:?}) != active workspace ({:?})",
            entry.workspace().entity_id(),
            active_workspace.entity_id(),
        );

        // 3. The entry must match the agent panel's current state.
        if panel.read(cx).active_thread_id(cx).is_some() {
            anyhow::ensure!(
                matches!(entry, ActiveEntry { .. }),
                "panel shows a tracked draft but active_entry is {:?}",
                entry,
            );
        } else if let Some(thread_id) = panel
            .read(cx)
            .active_conversation_view()
            .map(|cv| cv.read(cx).parent_id())
        {
            anyhow::ensure!(
                matches!(entry, ActiveEntry { thread_id: tid, .. } if *tid == thread_id),
                "panel has thread {:?} but active_entry is {:?}",
                thread_id,
                entry,
            );
        }

        // 4. Exactly one entry in sidebar contents must be uniquely
        //    identified by the active_entry — unless the panel is showing
        //    a draft, which is represented by the + button's active state
        //    rather than a sidebar row.
        // TODO: Make this check more complete
        let is_draft = panel.read(cx).active_thread_is_draft(cx)
            || panel.read(cx).active_conversation_view().is_none();
        if is_draft {
            return Ok(());
        }
        let matching_count = sidebar
            .contents
            .entries
            .iter()
            .filter(|e| entry.matches_entry(e))
            .count();
        if matching_count != 1 {
            let thread_entries: Vec<_> = sidebar
                .contents
                .entries
                .iter()
                .filter_map(|e| match e {
                    ListEntry::Thread(t) => Some(format!(
                        "tid={:?} sid={:?}",
                        t.metadata.thread_id, t.metadata.session_id
                    )),
                    _ => None,
                })
                .collect();
            let store = agent_ui::thread_metadata_store::ThreadMetadataStore::global(cx).read(cx);
            let store_entries: Vec<_> = store
                .entries()
                .map(|m| {
                    format!(
                        "tid={:?} sid={:?} archived={} paths={:?}",
                        m.thread_id,
                        m.session_id,
                        m.archived,
                        m.folder_paths()
                    )
                })
                .collect();
            anyhow::bail!(
                "expected exactly 1 sidebar entry matching active_entry {:?}, found {}. sidebar threads: {:?}. store: {:?}",
                entry,
                matching_count,
                thread_entries,
                store_entries,
            );
        }

        Ok(())
    }

    /// Every workspace in the multi-workspace should be "reachable" from
    /// the sidebar — meaning there is at least one entry (thread, draft,
    /// new-thread, or project header) that, when clicked, would activate
    /// that workspace.
    fn verify_all_workspaces_are_reachable(sidebar: &Sidebar, cx: &App) -> anyhow::Result<()> {
        let Some(multi_workspace) = sidebar.multi_workspace.upgrade() else {
            anyhow::bail!("sidebar should still have an associated multi-workspace");
        };

        let multi_workspace = multi_workspace.read(cx);

        let reachable_workspaces: HashSet<gpui::EntityId> = sidebar
            .contents
            .entries
            .iter()
            .flat_map(|entry| entry.reachable_workspaces(multi_workspace, cx))
            .map(|ws| ws.entity_id())
            .collect();

        let all_workspace_ids: HashSet<gpui::EntityId> = multi_workspace
            .workspaces()
            .map(|ws| ws.entity_id())
            .collect();

        let unreachable = &all_workspace_ids - &reachable_workspaces;

        anyhow::ensure!(
            unreachable.is_empty(),
            "The following workspaces are not reachable from any sidebar entry: {:?}",
            unreachable,
        );

        Ok(())
    }

    fn verify_workspace_group_key_integrity(sidebar: &Sidebar, cx: &App) -> anyhow::Result<()> {
        let Some(multi_workspace) = sidebar.multi_workspace.upgrade() else {
            anyhow::bail!("sidebar should still have an associated multi-workspace");
        };
        multi_workspace
            .read(cx)
            .assert_project_group_key_integrity(cx)
    }

    #[gpui::property_test(config = ProptestConfig {
        cases: 20,
        ..Default::default()
    })]
    async fn test_sidebar_invariants(
        #[strategy = gpui::proptest::collection::vec(0u32..DISTRIBUTION_SLOTS * 10, 1..10)]
        raw_operations: Vec<u32>,
        cx: &mut TestAppContext,
    ) {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT_PROPTEST_DB: AtomicUsize = AtomicUsize::new(0);

        agent_ui::test_support::init_test(cx);
        cx.update(|cx| {
            cx.set_global(db::AppDatabase::test_new());
            cx.set_global(agent_ui::MaxIdleRetainedThreads(1));
            cx.set_global(agent_ui::thread_metadata_store::TestMetadataDbName(
                format!(
                    "PROPTEST_THREAD_METADATA_{}",
                    NEXT_PROPTEST_DB.fetch_add(1, Ordering::SeqCst)
                ),
            ));

            ThreadStore::init_global(cx);
            ThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);

            // Auto-add an AgentPanel to every workspace so that implicitly
            // created workspaces (e.g. from thread activation) also have one.
            cx.observe_new(
                |workspace: &mut Workspace,
                 window: Option<&mut Window>,
                 cx: &mut gpui::Context<Workspace>| {
                    if let Some(window) = window {
                        let panel = cx.new(|cx| AgentPanel::test_new(workspace, window, cx));
                        workspace.add_panel(panel, window, cx);
                    }
                },
            )
            .detach();
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
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let mut state = TestState::new(fs);
        let mut executed: Vec<String> = Vec::new();

        for &raw_op in &raw_operations {
            let project_group_count =
                multi_workspace.read_with(cx, |mw, _| mw.project_group_keys().len());
            let operation = state.generate_operation(raw_op, project_group_count);
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

#[gpui::test]
async fn test_remote_project_integration_does_not_briefly_render_as_separate_project(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);

    cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    let app_state = cx.update(|cx| {
        let app_state = workspace::AppState::test(cx);
        workspace::init(app_state.clone(), cx);
        app_state
    });

    // Set up the remote server side.
    let server_fs = FakeFs::new(server_cx.executor());
    server_fs
        .insert_tree(
            "/project",
            serde_json::json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
    server_fs.set_branch_name(Path::new("/project/.git"), Some("main"));

    // Create the linked worktree checkout path on the remote server,
    // but do not yet register it as a git-linked worktree. The real
    // regrouping update in this test should happen only after the
    // sidebar opens the closed remote thread.
    server_fs
        .insert_tree(
            "/project-wt-1",
            serde_json::json!({
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;

    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    let (original_opts, server_session, _) = remote::RemoteClient::fake_server(cx, server_cx);

    server_cx.update(remote_server::HeadlessProject::init);
    let server_executor = server_cx.executor();
    let _headless = server_cx.new(|cx| {
        remote_server::HeadlessProject::new(
            remote_server::HeadlessAppState {
                session: server_session,
                fs: server_fs.clone(),
                http_client: Arc::new(http_client::BlockedHttpClient),
                node_runtime: node_runtime::NodeRuntime::unavailable(),
                languages: Arc::new(language::LanguageRegistry::new(server_executor.clone())),
                extension_host_proxy: Arc::new(extension::ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });

    // Connect the client side and build a remote project.
    let remote_client = remote::RemoteClient::connect_mock(original_opts.clone(), cx).await;
    let project = cx.update(|cx| {
        let project_client = client::Client::new(
            Arc::new(clock::FakeSystemClock::new()),
            http_client::FakeHttpClient::with_404_response(),
            cx,
        );
        let user_store = cx.new(|cx| client::UserStore::new(project_client.clone(), cx));
        project::Project::remote(
            remote_client,
            project_client,
            node_runtime::NodeRuntime::unavailable(),
            user_store,
            app_state.languages.clone(),
            app_state.fs.clone(),
            false,
            cx,
        )
    });

    // Open the remote worktree.
    project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(Path::new("/project"), true, cx)
        })
        .await
        .expect("should open remote worktree");
    cx.run_until_parked();

    // Verify the project is remote.
    project.read_with(cx, |project, cx| {
        assert!(!project.is_local(), "project should be remote");
        assert!(
            project.remote_connection_options(cx).is_some(),
            "project should have remote connection options"
        );
    });

    cx.update(|cx| <dyn fs::Fs>::set_global(app_state.fs.clone(), cx));

    // Create MultiWorkspace with the remote project.
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    cx.run_until_parked();

    // Save a thread for the main remote workspace (folder_paths match
    // the open workspace, so it will be classified as Open).
    let main_thread_id = acp::SessionId::new(Arc::from("main-thread"));
    save_thread_metadata(
        main_thread_id.clone(),
        Some("Main Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        None,
        &project,
        cx,
    );
    cx.run_until_parked();

    // Save a thread whose folder_paths point to a linked worktree path
    // that doesn't have an open workspace ("/project-wt-1"), but whose
    // main_worktree_paths match the project group key so it appears
    // in the sidebar under the same remote group. This simulates a
    // linked worktree workspace that was closed.
    let remote_thread_id = acp::SessionId::new(Arc::from("remote-thread"));
    let (main_worktree_paths, remote_connection) = project.read_with(cx, |p, cx| {
        (
            p.project_group_key(cx).path_list().clone(),
            p.remote_connection_options(cx),
        )
    });
    cx.update(|_window, cx| {
        let metadata = ThreadMetadata {
            thread_id: ThreadId::new(),
            session_id: Some(remote_thread_id.clone()),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: Some("Worktree Thread".into()),
            updated_at: chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 1).unwrap(),
            created_at: None,
            worktree_paths: WorktreePaths::from_path_lists(
                main_worktree_paths,
                PathList::new(&[PathBuf::from("/project-wt-1")]),
            )
            .unwrap(),
            archived: false,
            remote_connection,
        };
        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.save(metadata, cx));
    });
    cx.run_until_parked();

    focus_sidebar(&sidebar, cx);
    sidebar.update_in(cx, |sidebar, _window, _cx| {
        sidebar.selection = sidebar.contents.entries.iter().position(|entry| {
            matches!(
                entry,
                ListEntry::Thread(thread) if thread.metadata.session_id.as_ref() == Some(&remote_thread_id)
            )
        });
    });

    let saw_separate_project_header = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let saw_separate_project_header_for_observer = saw_separate_project_header.clone();

    sidebar
        .update(cx, |_, cx| {
            cx.observe_self(move |sidebar, _cx| {
                let mut project_headers = sidebar.contents.entries.iter().filter_map(|entry| {
                    if let ListEntry::ProjectHeader { label, .. } = entry {
                        Some(label.as_ref())
                    } else {
                        None
                    }
                });

                let Some(project_header) = project_headers.next() else {
                    saw_separate_project_header_for_observer
                        .store(true, std::sync::atomic::Ordering::SeqCst);
                    return;
                };

                if project_header != "project" || project_headers.next().is_some() {
                    saw_separate_project_header_for_observer
                        .store(true, std::sync::atomic::Ordering::SeqCst);
                }
            })
        })
        .detach();

    multi_workspace.update(cx, |multi_workspace, cx| {
        let workspace = multi_workspace.workspace().clone();
        workspace.update(cx, |workspace: &mut Workspace, cx| {
            let remote_client = workspace
                .project()
                .read(cx)
                .remote_client()
                .expect("main remote project should have a remote client");
            remote_client.update(cx, |remote_client: &mut remote::RemoteClient, cx| {
                remote_client.force_server_not_running(cx);
            });
        });
    });
    cx.run_until_parked();

    let (server_session_2, connect_guard_2) =
        remote::RemoteClient::fake_server_with_opts(&original_opts, cx, server_cx);
    let _headless_2 = server_cx.new(|cx| {
        remote_server::HeadlessProject::new(
            remote_server::HeadlessAppState {
                session: server_session_2,
                fs: server_fs.clone(),
                http_client: Arc::new(http_client::BlockedHttpClient),
                node_runtime: node_runtime::NodeRuntime::unavailable(),
                languages: Arc::new(language::LanguageRegistry::new(server_executor.clone())),
                extension_host_proxy: Arc::new(extension::ExtensionHostProxy::new()),
                startup_time: std::time::Instant::now(),
            },
            false,
            cx,
        )
    });
    drop(connect_guard_2);

    let window = cx.windows()[0];
    cx.update_window(window, |_, window, cx| {
        window.dispatch_action(Confirm.boxed_clone(), cx);
    })
    .unwrap();

    cx.run_until_parked();

    let new_workspace = multi_workspace.read_with(cx, |mw, _| {
        assert_eq!(
            mw.workspaces().count(),
            2,
            "confirming a closed remote thread should open a second workspace"
        );
        mw.workspaces()
            .find(|workspace| workspace.entity_id() != mw.workspace().entity_id())
            .unwrap()
            .clone()
    });

    server_fs
        .add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            git::repository::Worktree {
                path: PathBuf::from("/project-wt-1"),
                ref_name: Some("refs/heads/feature-wt".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

    server_cx.run_until_parked();
    cx.run_until_parked();
    server_cx.run_until_parked();
    cx.run_until_parked();

    let entries_after_update = visible_entries_as_strings(&sidebar, cx);
    let group_after_update = new_workspace.read_with(cx, |workspace, cx| {
        workspace.project().read(cx).project_group_key(cx)
    });

    assert_eq!(
        group_after_update,
        project.read_with(cx, |project, cx| ProjectGroupKey::from_project(project, cx)),
        "expected the remote worktree workspace to be grouped under the main remote project after the real update; \
         final sidebar entries: {:?}",
        entries_after_update,
    );

    sidebar.update(cx, |sidebar, _cx| {
        assert_remote_project_integration_sidebar_state(
            sidebar,
            &main_thread_id,
            &remote_thread_id,
        );
    });

    assert!(
        !saw_separate_project_header.load(std::sync::atomic::Ordering::SeqCst),
        "sidebar briefly rendered the remote worktree as a separate project during the real remote open/update sequence; \
         final group: {:?}; final sidebar entries: {:?}",
        group_after_update,
        entries_after_update,
    );
}

#[gpui::test]
async fn test_archive_removes_worktree_even_when_workspace_paths_diverge(cx: &mut TestAppContext) {
    // When the thread's folder_paths don't exactly match any workspace's
    // root paths (e.g. because a folder was added to the workspace after
    // the thread was created), workspace_to_remove is None. But the linked
    // worktree workspace still needs to be removed so that its worktree
    // entities are released, allowing git worktree removal to proceed.
    //
    // With the fix, archive_thread scans roots_to_archive for any linked
    // worktree workspaces and includes them in the removal set, even when
    // the thread's folder_paths don't match the workspace's root paths.
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
            "src": {
                "main.rs": "fn main() {}",
            },
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/project/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-a"),
            ref_name: Some("refs/heads/feature-a".into()),
            sha: "abc".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

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
    let sidebar = setup_sidebar(&multi_workspace, cx);

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(worktree_project.clone(), window, cx)
    });

    // Save thread metadata using folder_paths that DON'T match the
    // workspace's root paths. This simulates the case where the workspace's
    // paths diverged (e.g. a folder was added after thread creation).
    // This causes workspace_to_remove to be None because
    // workspace_for_paths can't find a workspace with these exact paths.
    let wt_thread_id = acp::SessionId::new(Arc::from("worktree-thread"));
    save_thread_metadata_with_main_paths(
        "worktree-thread",
        "Worktree Thread",
        PathList::new(&[
            PathBuf::from("/wt-feature-a"),
            PathBuf::from("/nonexistent"),
        ]),
        PathList::new(&[PathBuf::from("/project"), PathBuf::from("/nonexistent")]),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        cx,
    );

    // Also save a main thread so the sidebar has something to show.
    save_thread_metadata(
        acp::SessionId::new(Arc::from("main-thread")),
        Some("Main Thread".into()),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        None,
        &main_project,
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        2,
        "should start with 2 workspaces (main + linked worktree)"
    );

    // Archive the worktree thread.
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&wt_thread_id, window, cx);
    });

    cx.run_until_parked();

    // The linked worktree workspace should have been removed, even though
    // workspace_to_remove was None (paths didn't match).
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "linked worktree workspace should be removed after archiving, \
         even when folder_paths don't match workspace root paths"
    );

    // The thread should still be archived (not unarchived due to an error).
    let still_archived = cx.update(|_, cx| {
        ThreadMetadataStore::global(cx)
            .read(cx)
            .entry_by_session(&wt_thread_id)
            .map(|t| t.archived)
    });
    assert_eq!(
        still_archived,
        Some(true),
        "thread should still be archived (not rolled back due to error)"
    );

    // The linked worktree directory should be removed from disk.
    assert!(
        !fs.is_dir(Path::new("/wt-feature-a")).await,
        "linked worktree directory should be removed from disk"
    );
}

#[gpui::test]
async fn test_archive_mixed_workspace_closes_only_archived_worktree_items(cx: &mut TestAppContext) {
    // When a workspace contains both a worktree being archived and other
    // worktrees that should remain, only the editor items referencing the
    // archived worktree should be closed — the workspace itself must be
    // preserved.
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/main-repo",
        serde_json::json!({
            ".git": {
                "worktrees": {
                    "feature-b": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/feature-b",
                    },
                },
            },
            "src": {
                "lib.rs": "pub fn hello() {}",
            },
        }),
    )
    .await;

    fs.insert_tree(
        "/wt-feature-b",
        serde_json::json!({
            ".git": "gitdir: /main-repo/.git/worktrees/feature-b",
            "src": {
                "main.rs": "fn main() { hello(); }",
            },
        }),
    )
    .await;

    fs.add_linked_worktree_for_repo(
        Path::new("/main-repo/.git"),
        false,
        git::repository::Worktree {
            path: PathBuf::from("/wt-feature-b"),
            ref_name: Some("refs/heads/feature-b".into()),
            sha: "def".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

    // Create a single project that contains BOTH the main repo and the
    // linked worktree — this makes it a "mixed" workspace.
    let mixed_project = project::Project::test(
        fs.clone(),
        ["/main-repo".as_ref(), "/wt-feature-b".as_ref()],
        cx,
    )
    .await;

    mixed_project
        .update(cx, |p, cx| p.git_scans_complete(cx))
        .await;

    let (multi_workspace, cx) = cx
        .add_window_view(|window, cx| MultiWorkspace::test_new(mixed_project.clone(), window, cx));
    let sidebar = setup_sidebar(&multi_workspace, cx);

    // Open editor items in both worktrees so we can verify which ones
    // get closed.
    let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

    let worktree_ids: Vec<(WorktreeId, Arc<Path>)> = workspace.read_with(cx, |ws, cx| {
        ws.project()
            .read(cx)
            .visible_worktrees(cx)
            .map(|wt| (wt.read(cx).id(), wt.read(cx).abs_path()))
            .collect()
    });

    let main_repo_wt_id = worktree_ids
        .iter()
        .find(|(_, path)| path.ends_with("main-repo"))
        .map(|(id, _)| *id)
        .expect("should find main-repo worktree");

    let feature_b_wt_id = worktree_ids
        .iter()
        .find(|(_, path)| path.ends_with("wt-feature-b"))
        .map(|(id, _)| *id)
        .expect("should find wt-feature-b worktree");

    // Open files from both worktrees.
    let main_repo_path = project::ProjectPath {
        worktree_id: main_repo_wt_id,
        path: Arc::from(rel_path("src/lib.rs")),
    };
    let feature_b_path = project::ProjectPath {
        worktree_id: feature_b_wt_id,
        path: Arc::from(rel_path("src/main.rs")),
    };

    workspace
        .update_in(cx, |ws, window, cx| {
            ws.open_path(main_repo_path.clone(), None, true, window, cx)
        })
        .await
        .expect("should open main-repo file");
    workspace
        .update_in(cx, |ws, window, cx| {
            ws.open_path(feature_b_path.clone(), None, true, window, cx)
        })
        .await
        .expect("should open feature-b file");

    cx.run_until_parked();

    // Verify both items are open.
    let open_paths_before: Vec<project::ProjectPath> = workspace.read_with(cx, |ws, cx| {
        ws.panes()
            .iter()
            .flat_map(|pane| {
                pane.read(cx)
                    .items()
                    .filter_map(|item| item.project_path(cx))
            })
            .collect()
    });
    assert!(
        open_paths_before
            .iter()
            .any(|pp| pp.worktree_id == main_repo_wt_id),
        "main-repo file should be open"
    );
    assert!(
        open_paths_before
            .iter()
            .any(|pp| pp.worktree_id == feature_b_wt_id),
        "feature-b file should be open"
    );

    // Save thread metadata for the linked worktree with deliberately
    // mismatched folder_paths to trigger the scan-based detection.
    save_thread_metadata_with_main_paths(
        "feature-b-thread",
        "Feature B Thread",
        PathList::new(&[
            PathBuf::from("/wt-feature-b"),
            PathBuf::from("/nonexistent"),
        ]),
        PathList::new(&[PathBuf::from("/main-repo"), PathBuf::from("/nonexistent")]),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
        cx,
    );

    // Save another thread that references only the main repo (not the
    // linked worktree) so archiving the feature-b thread's worktree isn't
    // blocked by another unarchived thread referencing the same path.
    save_thread_metadata_with_main_paths(
        "other-thread",
        "Other Thread",
        PathList::new(&[PathBuf::from("/main-repo")]),
        PathList::new(&[PathBuf::from("/main-repo")]),
        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
        cx,
    );
    cx.run_until_parked();

    multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
    cx.run_until_parked();

    // There should still be exactly 1 workspace.
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "should have 1 workspace (the mixed workspace)"
    );

    // Archive the feature-b thread.
    let fb_session_id = acp::SessionId::new(Arc::from("feature-b-thread"));
    sidebar.update_in(cx, |sidebar, window, cx| {
        sidebar.archive_thread(&fb_session_id, window, cx);
    });

    cx.run_until_parked();

    // The workspace should still exist (it's "mixed" — has non-archived worktrees).
    assert_eq!(
        multi_workspace.read_with(cx, |mw, _| mw.workspaces().count()),
        1,
        "mixed workspace should be preserved"
    );

    // Only the feature-b editor item should have been closed.
    let open_paths_after: Vec<project::ProjectPath> = workspace.read_with(cx, |ws, cx| {
        ws.panes()
            .iter()
            .flat_map(|pane| {
                pane.read(cx)
                    .items()
                    .filter_map(|item| item.project_path(cx))
            })
            .collect()
    });
    assert!(
        open_paths_after
            .iter()
            .any(|pp| pp.worktree_id == main_repo_wt_id),
        "main-repo file should still be open"
    );
    assert!(
        !open_paths_after
            .iter()
            .any(|pp| pp.worktree_id == feature_b_wt_id),
        "feature-b file should have been closed"
    );
}

#[test]
fn test_worktree_info_branch_names_for_main_worktrees() {
    let folder_paths = PathList::new(&[PathBuf::from("/projects/myapp")]);
    let worktree_paths = WorktreePaths::from_folder_paths(&folder_paths);

    let branch_by_path: HashMap<PathBuf, SharedString> =
        [(PathBuf::from("/projects/myapp"), "feature-x".into())]
            .into_iter()
            .collect();

    let infos = worktree_info_from_thread_paths(&worktree_paths, &branch_by_path);
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].kind, ui::WorktreeKind::Main);
    assert_eq!(infos[0].branch_name, Some(SharedString::from("feature-x")));
    assert_eq!(infos[0].name, SharedString::from("myapp"));
}

#[test]
fn test_worktree_info_branch_names_for_linked_worktrees() {
    let main_paths = PathList::new(&[PathBuf::from("/projects/myapp")]);
    let folder_paths = PathList::new(&[PathBuf::from("/projects/myapp-feature")]);
    let worktree_paths =
        WorktreePaths::from_path_lists(main_paths, folder_paths).expect("same length");

    let branch_by_path: HashMap<PathBuf, SharedString> = [(
        PathBuf::from("/projects/myapp-feature"),
        "feature-branch".into(),
    )]
    .into_iter()
    .collect();

    let infos = worktree_info_from_thread_paths(&worktree_paths, &branch_by_path);
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].kind, ui::WorktreeKind::Linked);
    assert_eq!(
        infos[0].branch_name,
        Some(SharedString::from("feature-branch"))
    );
}

#[test]
fn test_worktree_info_missing_branch_returns_none() {
    let folder_paths = PathList::new(&[PathBuf::from("/projects/myapp")]);
    let worktree_paths = WorktreePaths::from_folder_paths(&folder_paths);

    let branch_by_path: HashMap<PathBuf, SharedString> = HashMap::new();

    let infos = worktree_info_from_thread_paths(&worktree_paths, &branch_by_path);
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].kind, ui::WorktreeKind::Main);
    assert_eq!(infos[0].branch_name, None);
    assert_eq!(infos[0].name, SharedString::from("myapp"));
}
