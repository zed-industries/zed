use crate::{TestServer, test_server::open_channel_notes};
use call::ActiveCall;
use channel::ACKNOWLEDGE_DEBOUNCE_INTERVAL;
use client::{Collaborator, LegacyUserId, ParticipantIndex};
use collab::rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT};

use collab_ui::channel_view::ChannelView;
use collections::HashMap;
use editor::{Anchor, Editor, MultiBufferOffset, ToOffset};
use futures::future;
use gpui::{BackgroundExecutor, Context, Entity, TestAppContext, Window};
use rpc::{RECEIVE_TIMEOUT, proto::PeerId};
use serde_json::json;
use std::ops::Range;
use util::rel_path::rel_path;
use workspace::CollaboratorId;

#[gpui::test]
async fn test_core_channel_buffers(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel("zed", None, (&client_a, cx_a), &mut [(&client_b, cx_b)])
        .await;

    // Client A joins the channel buffer
    let channel_buffer_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();

    // Client A edits the buffer
    let buffer_a = channel_buffer_a.read_with(cx_a, |buffer, _| buffer.buffer());
    buffer_a.update(cx_a, |buffer, cx| {
        buffer.edit([(0..0, "hello world")], None, cx)
    });
    buffer_a.update(cx_a, |buffer, cx| {
        buffer.edit([(5..5, ", cruel")], None, cx)
    });
    buffer_a.update(cx_a, |buffer, cx| {
        buffer.edit([(0..5, "goodbye")], None, cx)
    });
    buffer_a.update(cx_a, |buffer, cx| buffer.undo(cx));
    assert_eq!(buffer_text(&buffer_a, cx_a), "hello, cruel world");
    executor.run_until_parked();

    // Client B joins the channel buffer
    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    channel_buffer_b.read_with(cx_b, |buffer, _| {
        assert_collaborators(
            buffer.collaborators(),
            &[client_a.user_id(), client_b.user_id()],
        );
    });

    // Client B sees the correct text, and then edits it
    let buffer_b = channel_buffer_b.read_with(cx_b, |buffer, _| buffer.buffer());
    assert_eq!(
        buffer_b.read_with(cx_b, |buffer, _| buffer.remote_id()),
        buffer_a.read_with(cx_a, |buffer, _| buffer.remote_id())
    );
    assert_eq!(buffer_text(&buffer_b, cx_b), "hello, cruel world");
    buffer_b.update(cx_b, |buffer, cx| {
        buffer.edit([(7..12, "beautiful")], None, cx)
    });

    // Both A and B see the new edit
    executor.run_until_parked();
    assert_eq!(buffer_text(&buffer_a, cx_a), "hello, beautiful world");
    assert_eq!(buffer_text(&buffer_b, cx_b), "hello, beautiful world");

    // Client A closes the channel buffer.
    cx_a.update(|_| drop(channel_buffer_a));
    executor.run_until_parked();

    // Client B sees that client A is gone from the channel buffer.
    channel_buffer_b.read_with(cx_b, |buffer, _| {
        assert_collaborators(buffer.collaborators(), &[client_b.user_id()]);
    });

    // Client A rejoins the channel buffer
    let _channel_buffer_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    executor.run_until_parked();

    // Sanity test, make sure we saw A rejoining
    channel_buffer_b.read_with(cx_b, |buffer, _| {
        assert_collaborators(
            buffer.collaborators(),
            &[client_a.user_id(), client_b.user_id()],
        );
    });

    // Client A loses connection.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    // Client B observes A disconnect
    channel_buffer_b.read_with(cx_b, |buffer, _| {
        assert_collaborators(buffer.collaborators(), &[client_b.user_id()]);
    });

    // TODO:
    // - Test synchronizing offline updates, what happens to A's channel buffer when A disconnects
    // - Test interaction with channel deletion while buffer is open
}

#[gpui::test]
async fn test_channel_notes_participant_indices(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);
    cx_c.update(editor::init);

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    client_a
        .fs()
        .insert_tree("/root", json!({"file.txt": "123"}))
        .await;
    let (project_a, worktree_id_a) = client_a.build_local_project_with_trust("/root", cx_a).await;
    let project_b = client_b.build_empty_local_project(false, cx_b);
    let project_c = client_c.build_empty_local_project(false, cx_c);

    let (workspace_a, mut cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, mut cx_b) = client_b.build_workspace(&project_b, cx_b);
    let (workspace_c, cx_c) = client_c.build_workspace(&project_c, cx_c);

    // Clients A, B, and C open the channel notes
    let channel_view_a = cx_a
        .update(|window, cx| ChannelView::open(channel_id, None, workspace_a.clone(), window, cx))
        .await
        .unwrap();
    let channel_view_b = cx_b
        .update(|window, cx| ChannelView::open(channel_id, None, workspace_b.clone(), window, cx))
        .await
        .unwrap();
    let channel_view_c = cx_c
        .update(|window, cx| ChannelView::open(channel_id, None, workspace_c.clone(), window, cx))
        .await
        .unwrap();

    // Clients A, B, and C all insert and select some text
    channel_view_a.update_in(cx_a, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.insert("a", window, cx);
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges(vec![MultiBufferOffset(0)..MultiBufferOffset(1)]);
            });
        });
    });
    executor.run_until_parked();
    channel_view_b.update_in(cx_b, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.move_down(&Default::default(), window, cx);
            editor.insert("b", window, cx);
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges(vec![MultiBufferOffset(1)..MultiBufferOffset(2)]);
            });
        });
    });
    executor.run_until_parked();
    channel_view_c.update_in(cx_c, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.move_down(&Default::default(), window, cx);
            editor.insert("c", window, cx);
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges(vec![MultiBufferOffset(2)..MultiBufferOffset(3)]);
            });
        });
    });

    // Client A sees clients B and C without assigned colors, because they aren't
    // in a call together.
    executor.run_until_parked();
    channel_view_a.update_in(cx_a, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            assert_remote_selections(editor, &[(None, 1..2), (None, 2..3)], window, cx);
        });
    });

    // Clients A and B join the same call.
    for (call, cx) in [(&active_call_a, &mut cx_a), (&active_call_b, &mut cx_b)] {
        call.update(*cx, |call, cx| call.join_channel(channel_id, cx))
            .await
            .unwrap();
    }

    // Clients A and B see each other with two different assigned colors. Client C
    // still doesn't have a color.
    executor.run_until_parked();
    channel_view_a.update_in(cx_a, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            assert_remote_selections(
                editor,
                &[(Some(ParticipantIndex(1)), 1..2), (None, 2..3)],
                window,
                cx,
            );
        });
    });
    channel_view_b.update_in(cx_b, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            assert_remote_selections(
                editor,
                &[(Some(ParticipantIndex(0)), 0..1), (None, 2..3)],
                window,
                cx,
            );
        });
    });

    // Client A shares a project, and client B joins.
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    // Clients A and B open the same file.
    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id_a, rel_path("file.txt")),
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id_a, rel_path("file.txt")),
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |selections| {
            selections.select_ranges(vec![MultiBufferOffset(0)..MultiBufferOffset(1)]);
        });
    });
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |selections| {
            selections.select_ranges(vec![MultiBufferOffset(2)..MultiBufferOffset(3)]);
        });
    });
    executor.run_until_parked();

    // Clients A and B see each other with the same colors as in the channel notes.
    editor_a.update_in(cx_a, |editor, window, cx| {
        assert_remote_selections(editor, &[(Some(ParticipantIndex(1)), 2..3)], window, cx);
    });
    editor_b.update_in(cx_b, |editor, window, cx| {
        assert_remote_selections(editor, &[(Some(ParticipantIndex(0)), 0..1)], window, cx);
    });
}

#[track_caller]
fn assert_remote_selections(
    editor: &mut Editor,
    expected_selections: &[(Option<ParticipantIndex>, Range<usize>)],
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let snapshot = editor.snapshot(window, cx);
    let hub = editor.collaboration_hub().unwrap();
    let collaborators = hub.collaborators(cx);
    let range = Anchor::Min..Anchor::Max;
    let remote_selections = snapshot
        .remote_selections_in_range(&range, hub, cx)
        .map(|s| {
            let CollaboratorId::PeerId(peer_id) = s.collaborator_id else {
                panic!("unexpected collaborator id");
            };
            let start = s.selection.start.to_offset(snapshot.buffer_snapshot());
            let end = s.selection.end.to_offset(snapshot.buffer_snapshot());
            let user_id = collaborators.get(&peer_id).unwrap().user_id;
            let participant_index = hub.user_participant_indices(cx).get(&user_id).copied();
            (participant_index, start.0..end.0)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        remote_selections, expected_selections,
        "incorrect remote selections"
    );
}

#[gpui::test]
async fn test_multiple_handles_to_channel_buffer(
    deterministic: BackgroundExecutor,
    cx_a: &mut TestAppContext,
) {
    let mut server = TestServer::start(deterministic.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;

    let channel_id = server
        .make_channel("the-channel", None, (&client_a, cx_a), &mut [])
        .await;

    let channel_buffer_1 = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx));
    let channel_buffer_2 = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx));
    let channel_buffer_3 = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx));

    // All concurrent tasks for opening a channel buffer return the same model handle.
    let (channel_buffer, channel_buffer_2, channel_buffer_3) =
        future::try_join3(channel_buffer_1, channel_buffer_2, channel_buffer_3)
            .await
            .unwrap();
    let channel_buffer_entity_id = channel_buffer.entity_id();
    assert_eq!(channel_buffer, channel_buffer_2);
    assert_eq!(channel_buffer, channel_buffer_3);

    channel_buffer.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "hello")], None, cx);
        })
    });
    deterministic.run_until_parked();

    cx_a.update(|_| {
        drop(channel_buffer);
        drop(channel_buffer_2);
        drop(channel_buffer_3);
    });
    deterministic.run_until_parked();

    // The channel buffer can be reopened after dropping it.
    let channel_buffer = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    assert_ne!(channel_buffer.entity_id(), channel_buffer_entity_id);
    channel_buffer.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, _| {
            assert_eq!(buffer.text(), "hello");
        })
    });
}

#[gpui::test]
async fn test_channel_buffer_disconnect(
    deterministic: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(deterministic.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b)],
        )
        .await;

    let channel_buffer_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();

    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();

    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    channel_buffer_a.update(cx_a, |buffer, cx| {
        assert_eq!(buffer.channel(cx).unwrap().name, "the-channel");
        assert!(!buffer.is_connected());
    });

    deterministic.run_until_parked();

    server.allow_connections();
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    deterministic.run_until_parked();

    client_a
        .channel_store()
        .update(cx_a, |channel_store, _| {
            channel_store.remove_channel(channel_id)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();

    // Channel buffer observed the deletion
    channel_buffer_b.update(cx_b, |buffer, cx| {
        assert!(buffer.channel(cx).is_none());
        assert!(!buffer.is_connected());
    });
}

#[gpui::test]
async fn test_rejoin_channel_buffer(
    deterministic: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(deterministic.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b)],
        )
        .await;

    let channel_buffer_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();

    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "1")], None, cx);
        })
    });
    deterministic.run_until_parked();

    // Client A disconnects.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());

    // Both clients make an edit.
    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(1..1, "2")], None, cx);
        })
    });
    channel_buffer_b.update(cx_b, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "0")], None, cx);
        })
    });

    // Both clients see their own edit.
    deterministic.run_until_parked();
    channel_buffer_a.read_with(cx_a, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "12");
    });
    channel_buffer_b.read_with(cx_b, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "01");
    });

    // Client A reconnects. Both clients see each other's edits, and see
    // the same collaborators.
    server.allow_connections();
    deterministic.advance_clock(RECEIVE_TIMEOUT);
    channel_buffer_a.read_with(cx_a, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "012");
    });
    channel_buffer_b.read_with(cx_b, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "012");
    });

    channel_buffer_a.read_with(cx_a, |buffer_a, _| {
        channel_buffer_b.read_with(cx_b, |buffer_b, _| {
            assert_eq!(buffer_a.collaborators(), buffer_b.collaborators());
        });
    });
}

#[gpui::test]
async fn test_channel_buffers_and_server_restarts(
    deterministic: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(deterministic.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    let channel_buffer_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    let _channel_buffer_c = client_c
        .channel_store()
        .update(cx_c, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();

    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "1")], None, cx);
        })
    });
    deterministic.run_until_parked();

    // Client C can't reconnect.
    client_c.override_establish_connection(|_, cx| cx.spawn(async |_| future::pending().await));

    // Server stops.
    server.reset().await;
    deterministic.advance_clock(RECEIVE_TIMEOUT);

    // While the server is down, both clients make an edit.
    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(1..1, "2")], None, cx);
        })
    });
    channel_buffer_b.update(cx_b, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "0")], None, cx);
        })
    });

    // Server restarts.
    server.start().await.unwrap();
    deterministic.advance_clock(CLEANUP_TIMEOUT);

    // Clients reconnects. Clients A and B see each other's edits, and see
    // that client C has disconnected.
    channel_buffer_a.read_with(cx_a, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "012");
    });
    channel_buffer_b.read_with(cx_b, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "012");
    });

    channel_buffer_a.read_with(cx_a, |buffer_a, _| {
        channel_buffer_b.read_with(cx_b, |buffer_b, _| {
            assert_collaborators(
                buffer_a.collaborators(),
                &[client_a.user_id(), client_b.user_id()],
            );
            assert_eq!(buffer_a.collaborators(), buffer_b.collaborators());
        });
    });
}

#[gpui::test]
async fn test_channel_buffer_changes(
    deterministic: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let (server, client_a, client_b, channel_id) = TestServer::start2(cx_a, cx_b).await;
    let (_, cx_a) = client_a.build_test_workspace(cx_a).await;
    let (workspace_b, cx_b) = client_b.build_test_workspace(cx_b).await;
    let channel_store_b = client_b.channel_store().clone();

    // Editing the channel notes should set them to dirty
    open_channel_notes(channel_id, cx_a).await.unwrap();
    cx_a.simulate_keystrokes("1");
    channel_store_b.read_with(cx_b, |channel_store, _| {
        assert!(channel_store.has_channel_buffer_changed(channel_id))
    });

    // Opening the buffer should clear the changed flag.
    open_channel_notes(channel_id, cx_b).await.unwrap();
    channel_store_b.read_with(cx_b, |channel_store, _| {
        assert!(!channel_store.has_channel_buffer_changed(channel_id))
    });

    // Editing the channel while the buffer is open should not show that the buffer has changed.
    cx_a.simulate_keystrokes("2");
    channel_store_b.read_with(cx_b, |channel_store, _| {
        assert!(!channel_store.has_channel_buffer_changed(channel_id))
    });

    // Test that the server is tracking things correctly, and we retain our 'not changed'
    // state across a disconnect
    deterministic.advance_clock(ACKNOWLEDGE_DEBOUNCE_INTERVAL);
    server
        .simulate_long_connection_interruption(client_b.peer_id().unwrap(), deterministic.clone());

    // Re-subscribe to channels after reconnection (simulates collab panel re-rendering)
    client_b.initialize_channel_store(cx_b);
    deterministic.run_until_parked();

    channel_store_b.read_with(cx_b, |channel_store, _| {
        assert!(!channel_store.has_channel_buffer_changed(channel_id))
    });

    // Closing the buffer should re-enable change tracking
    cx_b.update(|window, cx| {
        workspace_b.update(cx, |workspace, cx| {
            workspace.close_all_items_and_panes(&Default::default(), window, cx)
        });
    });
    deterministic.run_until_parked();

    cx_a.simulate_keystrokes("3");
    channel_store_b.read_with(cx_b, |channel_store, _| {
        assert!(channel_store.has_channel_buffer_changed(channel_id))
    });
}

#[gpui::test]
async fn test_channel_buffer_changes_persist(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
) {
    let (mut server, client_a, client_b, channel_id) = TestServer::start2(cx_a, cx_b).await;
    let (_, cx_a) = client_a.build_test_workspace(cx_a).await;
    let (_, cx_b) = client_b.build_test_workspace(cx_b).await;

    // a) edits the notes
    open_channel_notes(channel_id, cx_a).await.unwrap();
    cx_a.simulate_keystrokes("1");
    // b) opens them to observe the current version
    open_channel_notes(channel_id, cx_b).await.unwrap();

    // On boot the client should get the correct state.
    let client_b2 = server.create_client(cx_b2, "user_b").await;
    let channel_store_b2 = client_b2.channel_store().clone();
    channel_store_b2.read_with(cx_b2, |channel_store, _| {
        assert!(!channel_store.has_channel_buffer_changed(channel_id))
    });
}

#[gpui::test]
async fn test_channel_buffer_operations_lost_on_reconnect(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b)],
        )
        .await;

    // Both clients open the channel buffer.
    let channel_buffer_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();

    // Step 1: Client A makes an initial edit that syncs to B.
    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "a")], None, cx);
        })
    });
    executor.run_until_parked();

    // Verify both clients see "a".
    channel_buffer_a.read_with(cx_a, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "a");
    });
    channel_buffer_b.read_with(cx_b, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "a");
    });

    // Step 2: Disconnect client A. Do NOT advance past RECONNECT_TIMEOUT
    // so that the buffer stays in `opened_buffers` for rejoin.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.run_until_parked();

    // Step 3: While disconnected, client A makes an offline edit ("b").
    // on_buffer_update fires but client.send() fails because transport is down.
    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(1..1, "b")], None, cx);
        })
    });
    executor.run_until_parked();

    // Client A sees "ab" locally; B still sees "a".
    channel_buffer_a.read_with(cx_a, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "ab");
    });
    channel_buffer_b.read_with(cx_b, |buffer, cx| {
        assert_eq!(buffer.buffer().read(cx).text(), "a");
    });

    // Step 4: Reconnect and make a racing edit in parallel.
    //
    // The race condition occurs when:
    // 1. Transport reconnects, handle_connect captures version V (with "b") and sends RejoinChannelBuffers
    // 2. DURING the async gap (awaiting response), user makes edit "c"
    // 3. on_buffer_update sends UpdateChannelBuffer (succeeds because transport is up)
    // 4. Server receives BOTH messages concurrently (FuturesUnordered)
    // 5. If UpdateChannelBuffer commits first, server version is inflated to include "c"
    // 6. RejoinChannelBuffers reads inflated version and sends it back
    // 7. Client's serialize_ops(inflated_version) filters out "b" (offline edit)
    //    because the inflated version's timestamp covers "b"'s timestamp

    // Get the buffer handle for spawning
    let buffer_for_edit = channel_buffer_a.read_with(cx_a, |buffer, _| buffer.buffer());

    // Spawn the edit task - it will wait for executor to run it
    let edit_task = cx_a.spawn({
        let buffer = buffer_for_edit;
        async move |mut cx| {
            let _ = buffer.update(&mut cx, |buffer, cx| {
                buffer.edit([(2..2, "c")], None, cx);
            });
        }
    });

    // Allow connections so reconnect can succeed
    server.allow_connections();

    // Advance clock to trigger reconnection attempt
    executor.advance_clock(RECEIVE_TIMEOUT);

    // Run the edit task - this races with handle_connect
    edit_task.detach();

    // Let everything settle.
    executor.run_until_parked();

    // Step 7: Read final buffer text from both clients.
    let text_a = channel_buffer_a.read_with(cx_a, |buffer, cx| buffer.buffer().read(cx).text());
    let text_b = channel_buffer_b.read_with(cx_b, |buffer, cx| buffer.buffer().read(cx).text());

    // Both clients must see the same text containing all three edits.
    assert_eq!(
        text_a, text_b,
        "Client A and B diverged! A sees {:?}, B sees {:?}. \
         Operations were lost during reconnection.",
        text_a, text_b
    );
    assert!(
        text_a.contains('a'),
        "Initial edit 'a' missing from final text {:?}",
        text_a
    );
    assert!(
        text_a.contains('b'),
        "Offline edit 'b' missing from final text {:?}. \
         This is the reconnection race bug: the offline operation was \
         filtered out by serialize_ops because the server_version was \
         inflated by a racing UpdateChannelBuffer.",
        text_a
    );
    assert!(
        text_a.contains('c'),
        "Racing edit 'c' missing from final text {:?}",
        text_a
    );

    // Step 8: Verify the invariant directly — every operation known to
    // client A must be observed by client B's version. If any operation
    // in A's history is not covered by B's version, it was lost.
    channel_buffer_a.read_with(cx_a, |buf_a, cx_a_inner| {
        let buffer_a = buf_a.buffer().read(cx_a_inner);
        let ops_a = buffer_a.operations();
        channel_buffer_b.read_with(cx_b, |buf_b, cx_b_inner| {
            let buffer_b = buf_b.buffer().read(cx_b_inner);
            let version_b = buffer_b.version();
            for (lamport, _op) in ops_a.iter() {
                assert!(
                    version_b.observed(*lamport),
                    "Operation with lamport timestamp {:?} from client A \
                     is NOT observed by client B's version. This operation \
                     was lost during reconnection.",
                    lamport
                );
            }
        });
    });
}

// Reproduces a crash observed in production: a client that kept its channel
// buffer alive across a server-side epoch snapshot ends up holding
// insertions that no longer exist anywhere else, and its selection
// broadcasts crash every other client rendering the channel notes with
// "invalid anchor" in `BufferSnapshot::offset_for_anchor`.
//
// The honest sequence, all through real client/server code:
//
// 1. Client B opens the channel notes (epoch 0, replica 8) and inserts text.
// 2. B loses its connection. After RECONNECT_TIMEOUT the server removes B's
//    collaborator row; B was the last collaborator, so the server snapshots
//    the buffer: the text is baked into `base_text`, the operations are
//    discarded, and the epoch advances to 1. B's insertion now exists only
//    in B's in-memory buffer.
// 3. B reconnects. `RejoinChannelBuffers` refuses B's buffer ("epoch has
//    changed"), so B's `ChannelBuffer` disconnects but stays alive: B is now
//    a zombie whose notes view still shows the old buffer.
// 4. Client C joins the notes in epoch 1 and is assigned replica 8 -- B's
//    old replica id, recycled. C's edits advance every other client's
//    version watermark for replica 8 past the lamport timestamp of B's
//    snapshotted insertion.
// 5. B parks its cursor inside its old insertion. Its selection broadcast
//    carries an anchor for insertion (8, small_lamport). On client A,
//    `Buffer::can_apply_op` gates `UpdateSelections` on
//    `version.observed(anchor.timestamp())`, and `clock::Global::observed`
//    is a per-replica high-watermark, so C's recycled-replica edits make the
//    check pass even though the insertion doesn't exist on A. The selection
//    is stored in `remote_selections` and crashes A's editor during
//    `layout_selections`.
#[gpui::test]
async fn test_channel_notes_selection_from_zombie_pre_epoch_buffer(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    // Epoch 0: client B is the only collaborator in the channel notes, and
    // inserts some text. B is the first joiner, so it gets replica 8
    // (FIRST_COLLAB_ID).
    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    let buffer_b = channel_buffer_b.read_with(cx_b, |channel_buffer, _| channel_buffer.buffer());
    buffer_b.update(cx_b, |buffer, cx| {
        buffer.edit([(0..0, "zombie epoch text")], None, cx)
    });
    executor.run_until_parked();

    let replica_id_b = channel_buffer_b.read_with(cx_b, |channel_buffer, cx| {
        assert!(channel_buffer.is_connected());
        channel_buffer.replica_id(cx)
    });

    // B parks a cursor inside its insertion and records the anchors. They
    // reference the insertion B just made: (replica 8, some small lamport).
    let (zombie_selections, zombie_anchor_timestamp) = buffer_b.read_with(cx_b, |buffer, _| {
        let snapshot = buffer.snapshot();
        let cursor = snapshot.anchor_after(3);
        assert_eq!(cursor.timestamp().replica_id, replica_id_b);
        (
            Arc::from([text::Selection {
                id: 1,
                start: cursor,
                end: cursor,
                reversed: false,
                goal: text::SelectionGoal::None,
            }]),
            cursor.timestamp(),
        )
    });

    // B loses its connection long enough for the server to remove its
    // collaborator row. B was the last collaborator, so the server
    // snapshots the buffer: epoch 0 -> 1, operations discarded.
    server.forbid_connections();
    server.disconnect_client(client_b.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    let db_channel_id = collab::db::ChannelId::from_proto(channel_id.0);
    assert!(
        server
            .app_state
            .db
            .get_channel_buffer_collaborators(db_channel_id)
            .await
            .unwrap()
            .is_empty(),
        "server should have dropped b and snapshotted the buffer"
    );

    // B reconnects. The rejoin fails because the epoch changed, so B's
    // channel buffer disconnects but stays alive with its pre-snapshot
    // contents: B is now a zombie replica.
    server.allow_connections();
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    executor.run_until_parked();
    channel_buffer_b.read_with(cx_b, |channel_buffer, _| {
        assert!(
            !channel_buffer.is_connected(),
            "b's buffer should have failed to rejoin across the epoch change"
        );
    });

    // Epoch 1: client C joins the notes first and gets B's recycled replica
    // id. C edits enough to push its lamport clock past the timestamp of B's
    // snapshotted insertion.
    let channel_buffer_c = client_c
        .channel_store()
        .update(cx_c, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    channel_buffer_c.read_with(cx_c, |channel_buffer, cx| {
        assert_eq!(
            channel_buffer.replica_id(cx),
            replica_id_b,
            "c should be assigned b's recycled replica id"
        );
        assert_eq!(
            channel_buffer.buffer().read(cx).text(),
            "zombie epoch text",
            "the snapshot should have baked b's text into the new epoch's base text"
        );
    });
    let buffer_c = channel_buffer_c.read_with(cx_c, |channel_buffer, _| channel_buffer.buffer());
    for _ in 0..zombie_anchor_timestamp.value {
        buffer_c.update(cx_c, |buffer, cx| buffer.edit([(0..0, "c")], None, cx));
    }
    executor.run_until_parked();

    // Client A opens the channel notes in a real window.
    let (_workspace_a, cx_a) = client_a.build_test_workspace(cx_a).await;
    let channel_view_a = open_channel_notes(channel_id, cx_a).await.unwrap();
    cx_a.run_until_parked();

    // A's watermark for the recycled replica now covers the zombie anchor's
    // timestamp, but the insertion itself was discarded by the snapshot.
    channel_view_a.update_in(cx_a, |notes, _, cx| {
        notes.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).as_singleton().unwrap().read(cx);
            assert!(
                buffer.version().observed(zombie_anchor_timestamp),
                "a's version watermark should cover the zombie anchor's timestamp"
            );
        });
    });

    // The zombie parks its cursor inside its old insertion, exactly as the
    // editor does on every cursor move in B's stale notes view, and the
    // buffer emits the broadcast operation. We capture it from the buffer's
    // own event and deliver it over B's connection.
    //
    // We deliver the captured op explicitly rather than relying on
    // `on_buffer_update` to send it, because the deterministic test harness
    // lands the zombie in the muted variant: B's client-side
    // buffer-disconnect timer (RECONNECT_TIMEOUT after B noticed the drop)
    // and the server's snapshot timer (RECONNECT_TIMEOUT after the server
    // noticed) are both armed at the same tick, so the buffer is
    // disconnected before `handle_connect`'s failed rejoin runs, leaving
    // `rejoining` stuck true and muting `on_buffer_update`. In production
    // these two clocks are skewed by asymmetric detection and human
    // reconnect timing, so a zombie also reaches the unmuted variant and
    // sends these exact bytes itself; any client that merely *applied* the
    // poisoned set likewise rebroadcasts it via `serialize_ops` on its next
    // rejoin. The wire message, and the crash it causes, are identical
    // regardless of which path emits it.
    let captured_selection_op = Rc::new(RefCell::new(None));
    let _subscription = cx_b.update(|cx| {
        cx.subscribe(&buffer_b, {
            let captured_selection_op = captured_selection_op.clone();
            move |_, event, _| {
                if let language::BufferEvent::Operation {
                    operation,
                    is_local: true,
                } = event
                    && matches!(operation, language::Operation::UpdateSelections { .. })
                {
                    *captured_selection_op.borrow_mut() =
                        Some(language::proto::serialize_operation(operation));
                }
            }
        })
    });
    buffer_b.update(cx_b, |buffer, cx| {
        buffer.set_active_selections(
            zombie_selections,
            false,
            language::CursorShape::default(),
            cx,
        );
    });
    let zombie_selection_op = captured_selection_op.borrow_mut().take().unwrap();
    client_b
        .client()
        .send(rpc::proto::UpdateChannelBuffer {
            channel_id: channel_id.0,
            operations: vec![zombie_selection_op],
        })
        .unwrap();

    // Client A receives the selection. It passes `can_resolve` thanks to the
    // recycled replica's watermark, and rendering the channel notes panics
    // in `offset_for_anchor` via `layout_selections`.
    cx_a.run_until_parked();

    channel_view_a.update_in(cx_a, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            // Resolve remote selections the same way `layout_selections`
            // does during rendering. With the bug present, this panics with
            // "invalid anchor".
            let snapshot = editor.snapshot(window, cx);
            let hub = editor.collaboration_hub().unwrap();
            let range = Anchor::Min..Anchor::Max;
            for remote_selection in snapshot.remote_selections_in_range(&range, hub, cx) {
                let start = remote_selection
                    .selection
                    .start
                    .to_offset(snapshot.buffer_snapshot());
                let end = remote_selection
                    .selection
                    .end
                    .to_offset(snapshot.buffer_snapshot());
                assert!(start <= end);
            }
        });
    });
}

// Reproduces the same crash as the test above, but distilled to the wire
// level: the poisoned `UpdateSelections` is crafted directly, standing in
// for any diverged replica (the zombie above, or a victim of a historical
// op-loss bug) whose buffer contains an insertion nobody else has.
//
// The receiving side admits the poisoned selection because
// `Buffer::can_apply_op` gates `UpdateSelections` on
// `version.observed(anchor.timestamp())`, and `clock::Global::observed` is a
// per-replica high-watermark: once a *later* op from the same replica is
// applied, the watermark also covers the missing insertion's timestamp, so
// the gap is undetectable. The selection is then stored in
// `remote_selections` and crashes the editor during `layout_selections`.
#[gpui::test]
async fn test_channel_notes_selection_anchored_in_unseen_insertion(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let (_server, client_a, client_b, channel_id) = TestServer::start2(cx_a, cx_b).await;
    let (_workspace_a, cx_a) = client_a.build_test_workspace(cx_a).await;

    // Client A opens the channel notes in a real window, and writes some text.
    let channel_view_a = open_channel_notes(channel_id, cx_a).await.unwrap();
    channel_view_a.update_in(cx_a, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.insert("hello from a", window, cx);
        });
    });
    cx_a.run_until_parked();

    // Client B joins the channel buffer as a collaborator.
    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    cx_b.run_until_parked();

    let (buffer_id, replica_id_b, version_b) =
        channel_buffer_b.read_with(cx_b, |channel_buffer, cx| {
            let buffer = channel_buffer.buffer().read(cx);
            assert_eq!(buffer.text(), "hello from a");
            (buffer.remote_id(), buffer.replica_id(), buffer.version())
        });

    // Simulate client B being a diverged replica, exactly as observed on the
    // wire in the production crash:
    // - B has an insertion at (replica_b, 100) that was never delivered to
    //   anyone else (we never deliver it here).
    // - B's later edit at (replica_b, 200) *is* delivered, advancing every
    //   other replica's watermark for replica_b past 100.
    // - B then broadcasts selections anchored inside the missing insertion.
    let phantom_insertion = clock::Lamport {
        replica_id: replica_id_b,
        value: 100,
    };
    let delivered_edit = clock::Lamport {
        replica_id: replica_id_b,
        value: 200,
    };
    let selection_timestamp = clock::Lamport {
        replica_id: replica_id_b,
        value: 201,
    };

    let edit_op = language::Operation::Buffer(text::Operation::Edit(text::EditOperation {
        timestamp: delivered_edit,
        version: version_b,
        ranges: vec![text::FullOffset(0)..text::FullOffset(0)],
        new_text: vec!["B: ".into()],
    }));
    let selection_op = language::Operation::UpdateSelections {
        selections: Arc::from([text::Selection {
            id: 1,
            start: text::Anchor::new(phantom_insertion, 1, text::Bias::Left, buffer_id),
            end: text::Anchor::new(phantom_insertion, 2, text::Bias::Left, buffer_id),
            reversed: false,
            goal: text::SelectionGoal::None,
        }]),
        lamport_timestamp: selection_timestamp,
        line_mode: false,
        cursor_shape: language::CursorShape::default(),
    };

    client_b
        .client()
        .send(rpc::proto::UpdateChannelBuffer {
            channel_id: channel_id.0,
            operations: vec![
                language::proto::serialize_operation(&edit_op),
                language::proto::serialize_operation(&selection_op),
            ],
        })
        .unwrap();

    // Client A receives both operations. The edit applies cleanly and
    // advances A's watermark for B's replica to 200, so the selection op
    // passes `can_resolve` and is stored even though the insertion at
    // (replica_b, 100) doesn't exist on A. Rendering the channel notes then
    // panics in `offset_for_anchor` via `layout_selections`.
    cx_a.run_until_parked();

    channel_view_a.update_in(cx_a, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "B: hello from a");

            // Resolve remote selections the same way `layout_selections`
            // does during rendering. With the bug present, this panics with
            // "invalid anchor".
            let snapshot = editor.snapshot(window, cx);
            let hub = editor.collaboration_hub().unwrap();
            let range = Anchor::Min..Anchor::Max;
            for remote_selection in snapshot.remote_selections_in_range(&range, hub, cx) {
                let start = remote_selection
                    .selection
                    .start
                    .to_offset(snapshot.buffer_snapshot());
                let end = remote_selection
                    .selection
                    .end
                    .to_offset(snapshot.buffer_snapshot());
                assert!(start <= end);
            }
        });
    });
}

#[track_caller]
fn assert_collaborators(
    collaborators: &HashMap<PeerId, Collaborator>,
    ids: &[Option<LegacyUserId>],
) {
    let mut user_ids = collaborators
        .values()
        .map(|collaborator| collaborator.user_id)
        .collect::<Vec<_>>();
    user_ids.sort();
    assert_eq!(
        user_ids,
        ids.iter().map(|id| id.unwrap()).collect::<Vec<_>>()
    );
}

fn buffer_text(channel_buffer: &Entity<language::Buffer>, cx: &mut TestAppContext) -> String {
    channel_buffer.read_with(cx, |buffer, _| buffer.text())
}
