use crate::{
    rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    tests::{TestServer, test_server::open_channel_notes},
};
use call::ActiveCall;
use channel::ACKNOWLEDGE_DEBOUNCE_INTERVAL;
use client::{Collaborator, ParticipantIndex, UserId};
use collab_ui::channel_view::ChannelView;
use collections::HashMap;
use editor::{Anchor, Editor, ToOffset};
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
    let (project_a, worktree_id_a) = client_a.build_local_project("/root", cx_a).await;
    let project_b = client_b.build_empty_local_project(cx_b);
    let project_c = client_c.build_empty_local_project(cx_c);

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
                selections.select_ranges(vec![0..1]);
            });
        });
    });
    executor.run_until_parked();
    channel_view_b.update_in(cx_b, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.move_down(&Default::default(), window, cx);
            editor.insert("b", window, cx);
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges(vec![1..2]);
            });
        });
    });
    executor.run_until_parked();
    channel_view_c.update_in(cx_c, |notes, window, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.move_down(&Default::default(), window, cx);
            editor.insert("c", window, cx);
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges(vec![2..3]);
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
    executor.start_waiting();
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
    executor.start_waiting();
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
            selections.select_ranges(vec![0..1]);
        });
    });
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |selections| {
            selections.select_ranges(vec![2..3]);
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
    let range = Anchor::min()..Anchor::max();
    let remote_selections = snapshot
        .remote_selections_in_range(&range, hub, cx)
        .map(|s| {
            let CollaboratorId::PeerId(peer_id) = s.collaborator_id else {
                panic!("unexpected collaborator id");
            };
            let start = s.selection.start.to_offset(&snapshot.buffer_snapshot);
            let end = s.selection.end.to_offset(&snapshot.buffer_snapshot);
            let user_id = collaborators.get(&peer_id).unwrap().user_id;
            let participant_index = hub.user_participant_indices(cx).get(&user_id).copied();
            (participant_index, start..end)
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

#[track_caller]
fn assert_collaborators(collaborators: &HashMap<PeerId, Collaborator>, ids: &[Option<UserId>]) {
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
