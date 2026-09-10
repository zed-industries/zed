use super::*;
use crate::Project;
use crate::debugger::breakpoint_store::ActiveStackFrame;
use fs::FakeFs;
use gpui::TestAppContext;
use settings::SettingsStore;

async fn session(cx: &mut TestAppContext) -> Entity<Session> {
    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
    });
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(Path::new("/project"), serde_json::json!({"main.rs": ""}))
        .await;
    let project = Project::test(fs, [Path::new("/project")], cx).await;
    cx.update(|cx| {
        Session::new(
            project.read(cx).breakpoint_store().clone(),
            SessionId(1),
            None,
            None,
            DebugAdapterName("test".into()),
            Default::default(),
            Default::default(),
            None,
            None,
            None,
            cx,
        )
    })
}

fn stopped(thread: i64, all_threads: bool) -> StoppedEvent {
    StoppedEvent {
        reason: dap::StoppedEventReason::Breakpoint,
        description: None,
        thread_id: Some(thread),
        preserve_focus_hint: None,
        text: None,
        all_threads_stopped: Some(all_threads),
        hit_breakpoint_ids: None,
    }
}

#[gpui::test]
async fn delayed_continue_preserves_new_stop(cx: &mut TestAppContext) {
    let session = session(cx).await;
    session.update(cx, |session, cx| {
        session.handle_stopped_event(stopped(1, true), cx);
        let token = session.execution_order.begin(1);
        let revision = session.breakpoint_store.read(cx).active_position_revision();
        let response = Session::on_continue_response(ThreadId(1), token, revision);
        session
            .active_snapshot
            .thread_states
            .continue_thread(ThreadId(1));
        // Force the event handler to run before the outstanding response callback.
        session.handle_stopped_event(stopped(1, true), cx);
        response(
            session,
            Ok(dap::ContinueResponse {
                all_threads_continued: Some(true),
            }),
            cx,
        );
        assert_eq!(session.thread_status(ThreadId(1)), ThreadStatus::Stopped);
        assert_eq!(session.thread_status(ThreadId(2)), ThreadStatus::Stopped);
    });
}

#[gpui::test]
async fn delayed_all_threads_continue_preserves_other_thread_stop(cx: &mut TestAppContext) {
    let session = session(cx).await;
    session.update(cx, |session, cx| {
        session.handle_stopped_event(stopped(1, true), cx);
        let token = session.execution_order.begin(1);
        let revision = session.breakpoint_store.read(cx).active_position_revision();
        let response = Session::on_continue_response(ThreadId(1), token, revision);
        session.handle_stopped_event(stopped(2, false), cx);
        response(
            session,
            Ok(dap::ContinueResponse {
                all_threads_continued: Some(true),
            }),
            cx,
        );
        assert_eq!(session.thread_status(ThreadId(1)), ThreadStatus::Running);
        assert_eq!(session.thread_status(ThreadId(2)), ThreadStatus::Stopped);
    });
}

#[gpui::test]
async fn delayed_step_preserves_new_active_frame(cx: &mut TestAppContext) {
    let session = session(cx).await;
    session.update(cx, |session, cx| {
        let token = session.execution_order.begin(1);
        let revision = session.breakpoint_store.read(cx).active_position_revision();
        let response = Session::on_step_response::<NextCommand>(ThreadId(1), token, revision);
        let frame = ActiveStackFrame {
            session_id: session.session_id(),
            thread_id: ThreadId(2),
            stack_frame_id: 2,
            path: Path::new("/project/main.rs").into(),
            position: text::Anchor::min_for_buffer(text::BufferId::new(1).unwrap()),
        };
        session
            .breakpoint_store
            .update(cx, |store, cx| store.set_active_position(frame.clone(), cx));
        response(session, Ok(()), cx);
        assert!(session.breakpoint_store.read(cx).active_position() == Some(&frame));
    });
}

#[gpui::test]
async fn delayed_failures_do_not_stop_a_newer_execution(cx: &mut TestAppContext) {
    let session = session(cx).await;
    session.update(cx, |session, cx| {
        let token = session.execution_order.begin(1);
        let revision = session.breakpoint_store.read(cx).active_position_revision();
        let continued = Session::on_continue_response(ThreadId(1), token, revision);
        let stepped = Session::on_step_response::<NextCommand>(ThreadId(1), token, revision);
        session.execution_order.begin(1);
        session
            .active_snapshot
            .thread_states
            .continue_thread(ThreadId(1));
        assert!(continued(session, Err(anyhow!("old Continue failed")), cx).is_none());
        assert_eq!(session.thread_status(ThreadId(1)), ThreadStatus::Running);
        assert!(stepped(session, Err(anyhow!("old Step failed")), cx).is_none());
        assert_eq!(session.thread_status(ThreadId(1)), ThreadStatus::Running);
    });
}
