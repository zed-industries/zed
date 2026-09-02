use std::path::{Path, PathBuf};

use acp_thread::MentionUri;
use agent::ThreadStore;
use gpui::{Action as _, AppContext as _, TestAppContext, VisualContext as _, VisualTestContext};
use language_model::LanguageModelRegistry;
use project::{FakeFs, Project};
use serde_json::json;
use workspace::MultiWorkspace;
use zed_actions::agent::AddSelectionToThread;

use crate::{AgentPanel, AgentThreadSource, thread_metadata_store::ThreadMetadataStore};

/// End-to-end guard for `AddSelectionToThread` from the debugger dock: with
/// selections in both the active file editor and the debug console, the
/// console (the focused surface) must win.
///
/// The console and the session terminal live in the `DebugPanel` dock and are
/// never `workspace.active_item()`; before the fix, selection resolution fell
/// back to the active editor or a stale cached source, so the wrong text was
/// attached to the thread.
#[gpui::test]
async fn test_add_selection_from_debug_console_beats_active_editor(cx: &mut TestAppContext) {
    // Combined init: debugger_ui's test fixtures (which set up the settings
    // store, themes, DAP adapter registry) plus the agent panel stack.
    debugger_ui::tests::init_test(cx);
    cx.update(|cx| {
        db::AppDatabase::test_new();
        ThreadMetadataStore::init_global(cx);
        ThreadStore::init_global(cx);
        LanguageModelRegistry::test(cx);
        crate::agent_panel::init(cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        prompt_store::init(cx);
    });

    let fs = FakeFs::new(cx.executor());
    cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
    fs.insert_tree(
        "/project",
        json!({ "file.rs": "line one\nline two\nline three\n" }),
    )
    .await;
    let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

    let multi_workspace =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let workspace = multi_workspace
        .read_with(cx, |mw, _cx| mw.workspace().clone())
        .unwrap();

    // Load the debug panel into the same workspace (mirrors
    // `debugger_ui::tests::init_test_workspace`, but for this window).
    let debug_panel = multi_workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |_workspace, cx| {
                cx.spawn_in(window, async move |this, cx| {
                    debugger_ui::debugger_panel::DebugPanel::load(this, cx).await
                })
            })
        })
        .unwrap()
        .await
        .expect("Failed to load debug panel");
    multi_workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.add_panel(debug_panel, window, cx);
            });
        })
        .unwrap();
    cx.run_until_parked();

    let cx = &mut VisualTestContext::from_window(*multi_workspace, cx);
    let panel = workspace.update_in(cx, |workspace, window, cx| {
        let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
        workspace.add_panel(panel.clone(), window, cx);
        panel
    });

    // An active native-thread conversation so the selection is inserted as a
    // mention into the message editor.
    panel.update_in(cx, |panel, window, cx| {
        panel.activate_new_thread(false, AgentThreadSource::AgentPanel, window, cx);
    });
    cx.run_until_parked();

    // Start a debug session and give its console some output.
    let session = debugger_ui::tests::start_debug_session(&multi_workspace, cx, |_| {})
        .expect("debug session should start");
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    client
        .fake_event(dap::messages::Events::Output(dap::OutputEvent {
            category: None,
            output: "console text".to_string(),
            data: None,
            variables_reference: None,
            source: None,
            line: None,
            column: None,
            group: None,
            location_reference: None,
        }))
        .await;
    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();

    // Select text in the file editor (the would-be `active_item()` decoy).
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_paths(
                vec![PathBuf::from("/project/file.rs")],
                workspace::OpenOptions::default(),
                None,
                window,
                cx,
            )
        })
        .await;
    cx.run_until_parked();
    let file_editor = workspace.update(cx, |workspace, cx| {
        workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<editor::Editor>(cx))
            .expect("opened file should be an editor")
    });
    file_editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |selections| {
            selections.select_ranges([text::Point::new(1, 0)..text::Point::new(2, 4)]);
        });
    });

    // Then select text in the debug console and focus it.
    let console_editor = workspace.update(cx, |workspace, cx| {
        workspace
            .panel::<debugger_ui::debugger_panel::DebugPanel>(cx)
            .and_then(|panel| panel.read(cx).console_editor(cx))
            .expect("debug console editor should be reachable")
    });
    console_editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |selections| {
            selections.select_ranges([text::Point::new(0, 0)..text::Point::new(0, 12)]);
        });
    });
    cx.focus(&console_editor);
    cx.run_until_parked();

    workspace.update_in(cx, |_, window, cx| {
        window.dispatch_action(AddSelectionToThread.boxed_clone(), cx);
    });
    cx.run_until_parked();

    // The thread's message editor must carry the *console* selection mention
    // (its buffer has no project path) and must not carry the file selection.
    let mentions = panel.read_with(cx, |panel, cx| {
        let conversation_view = panel
            .active_conversation_view()
            .expect("native thread should be active");
        let thread = conversation_view
            .read(cx)
            .active_thread()
            .expect("thread view should exist");
        thread
            .read(cx)
            .message_editor
            .read(cx)
            .mention_set()
            .read(cx)
            .mentions()
    });

    assert!(
        mentions
            .iter()
            .any(|uri| matches!(uri, MentionUri::Selection { abs_path: None, .. })),
        "expected a console-buffer selection mention, got {mentions:?}"
    );
    assert!(
        !mentions.iter().any(|uri| matches!(
            uri,
            MentionUri::Selection {
                abs_path: Some(path),
                ..
            } if path == Path::new("/project/file.rs")
        )),
        "file editor selection must not shadow the focused console selection, got {mentions:?}"
    );
}
