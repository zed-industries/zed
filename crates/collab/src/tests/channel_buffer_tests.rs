use crate::{
    rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    tests::TestServer,
};
use call::ActiveCall;
use channel::Channel;
use client::ParticipantIndex;
use client::{Collaborator, UserId};
use collab_ui::channel_view::ChannelView;
use collections::HashMap;
use editor::{Anchor, Editor, ToOffset};
use futures::future;
use gpui::{executor::Deterministic, ModelHandle, TestAppContext, ViewContext};
use rpc::{proto::PeerId, RECEIVE_TIMEOUT};
use serde_json::json;
use std::{ops::Range, sync::Arc};

#[gpui::test]
async fn test_core_channel_buffers(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
    deterministic.run_until_parked();

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
    deterministic.run_until_parked();
    assert_eq!(buffer_text(&buffer_a, cx_a), "hello, beautiful world");
    assert_eq!(buffer_text(&buffer_b, cx_b), "hello, beautiful world");

    // Client A closes the channel buffer.
    cx_a.update(|_| drop(channel_buffer_a));
    deterministic.run_until_parked();

    // Client B sees that client A is gone from the channel buffer.
    channel_buffer_b.read_with(cx_b, |buffer, _| {
        assert_collaborators(&buffer.collaborators(), &[client_b.user_id()]);
    });

    // Client A rejoins the channel buffer
    let _channel_buffer_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    deterministic.run_until_parked();

    // Sanity test, make sure we saw A rejoining
    channel_buffer_b.read_with(cx_b, |buffer, _| {
        assert_collaborators(
            &buffer.collaborators(),
            &[client_a.user_id(), client_b.user_id()],
        );
    });

    // Client A loses connection.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    // Client B observes A disconnect
    channel_buffer_b.read_with(cx_b, |buffer, _| {
        assert_collaborators(&buffer.collaborators(), &[client_b.user_id()]);
    });

    // TODO:
    // - Test synchronizing offline updates, what happens to A's channel buffer when A disconnects
    // - Test interaction with channel deletion while buffer is open
}

#[gpui::test]
async fn test_channel_notes_participant_indices(
    deterministic: Arc<Deterministic>,
    mut cx_a: &mut TestAppContext,
    mut cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
    let workspace_a = client_a.build_workspace(&project_a, cx_a).root(cx_a);
    let workspace_b = client_b.build_workspace(&project_b, cx_b).root(cx_b);
    let workspace_c = client_c.build_workspace(&project_c, cx_c).root(cx_c);

    // Clients A, B, and C open the channel notes
    let channel_view_a = cx_a
        .update(|cx| ChannelView::open(channel_id, workspace_a.clone(), cx))
        .await
        .unwrap();
    let channel_view_b = cx_b
        .update(|cx| ChannelView::open(channel_id, workspace_b.clone(), cx))
        .await
        .unwrap();
    let channel_view_c = cx_c
        .update(|cx| ChannelView::open(channel_id, workspace_c.clone(), cx))
        .await
        .unwrap();

    // Clients A, B, and C all insert and select some text
    channel_view_a.update(cx_a, |notes, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.insert("a", cx);
            editor.change_selections(None, cx, |selections| {
                selections.select_ranges(vec![0..1]);
            });
        });
    });
    deterministic.run_until_parked();
    channel_view_b.update(cx_b, |notes, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.move_down(&Default::default(), cx);
            editor.insert("b", cx);
            editor.change_selections(None, cx, |selections| {
                selections.select_ranges(vec![1..2]);
            });
        });
    });
    deterministic.run_until_parked();
    channel_view_c.update(cx_c, |notes, cx| {
        notes.editor.update(cx, |editor, cx| {
            editor.move_down(&Default::default(), cx);
            editor.insert("c", cx);
            editor.change_selections(None, cx, |selections| {
                selections.select_ranges(vec![2..3]);
            });
        });
    });

    // Client A sees clients B and C without assigned colors, because they aren't
    // in a call together.
    deterministic.run_until_parked();
    channel_view_a.update(cx_a, |notes, cx| {
        notes.editor.update(cx, |editor, cx| {
            assert_remote_selections(editor, &[(None, 1..2), (None, 2..3)], cx);
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
    deterministic.run_until_parked();
    channel_view_a.update(cx_a, |notes, cx| {
        notes.editor.update(cx, |editor, cx| {
            assert_remote_selections(
                editor,
                &[(Some(ParticipantIndex(1)), 1..2), (None, 2..3)],
                cx,
            );
        });
    });
    channel_view_b.update(cx_b, |notes, cx| {
        notes.editor.update(cx, |editor, cx| {
            assert_remote_selections(
                editor,
                &[(Some(ParticipantIndex(0)), 0..1), (None, 2..3)],
                cx,
            );
        });
    });

    // Client A shares a project, and client B joins.
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    let workspace_b = client_b.build_workspace(&project_b, cx_b).root(cx_b);

    // Clients A and B open the same file.
    let editor_a = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id_a, "file.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id_a, "file.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    editor_a.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |selections| {
            selections.select_ranges(vec![0..1]);
        });
    });
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |selections| {
            selections.select_ranges(vec![2..3]);
        });
    });
    deterministic.run_until_parked();

    // Clients A and B see each other with the same colors as in the channel notes.
    editor_a.update(cx_a, |editor, cx| {
        assert_remote_selections(editor, &[(Some(ParticipantIndex(1)), 2..3)], cx);
    });
    editor_b.update(cx_b, |editor, cx| {
        assert_remote_selections(editor, &[(Some(ParticipantIndex(0)), 0..1)], cx);
    });
}

#[track_caller]
fn assert_remote_selections(
    editor: &mut Editor,
    expected_selections: &[(Option<ParticipantIndex>, Range<usize>)],
    cx: &mut ViewContext<Editor>,
) {
    let snapshot = editor.snapshot(cx);
    let range = Anchor::min()..Anchor::max();
    let remote_selections = snapshot
        .remote_selections_in_range(&range, editor.collaboration_hub().unwrap(), cx)
        .map(|s| {
            let start = s.selection.start.to_offset(&snapshot.buffer_snapshot);
            let end = s.selection.end.to_offset(&snapshot.buffer_snapshot);
            (s.participant_index, start..end)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        remote_selections, expected_selections,
        "incorrect remote selections"
    );
}

#[gpui::test]
async fn test_multiple_handles_to_channel_buffer(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
    let channel_buffer_model_id = channel_buffer.id();
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
    assert_ne!(channel_buffer.id(), channel_buffer_model_id);
    channel_buffer.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, _| {
            assert_eq!(buffer.text(), "hello");
        })
    });
}

#[gpui::test]
async fn test_channel_buffer_disconnect(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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

    channel_buffer_a.update(cx_a, |buffer, _| {
        assert_eq!(
            buffer.channel().as_ref(),
            &channel(channel_id, "the-channel")
        );
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
    channel_buffer_b.update(cx_b, |buffer, _| {
        assert_eq!(
            buffer.channel().as_ref(),
            &channel(channel_id, "the-channel")
        );
        assert!(!buffer.is_connected());
    });
}

fn channel(id: u64, name: &'static str) -> Channel {
    Channel {
        id,
        name: name.to_string(),
        has_note_changed: false,
    }
}

#[gpui::test]
async fn test_rejoin_channel_buffer(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
    client_c.override_establish_connection(|_, cx| cx.spawn(|_| future::pending()));

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

#[gpui::test(iterations = 10)]
async fn test_following_to_channel_notes_without_a_shared_project(
    deterministic: Arc<Deterministic>,
    mut cx_a: &mut TestAppContext,
    mut cx_b: &mut TestAppContext,
    mut cx_c: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let client_c = server.create_client(cx_c, "user_c").await;

    cx_a.update(editor::init);
    cx_b.update(editor::init);
    cx_c.update(editor::init);
    cx_a.update(collab_ui::channel_view::init);
    cx_b.update(collab_ui::channel_view::init);
    cx_c.update(collab_ui::channel_view::init);

    let channel_1_id = server
        .make_channel(
            "channel-1",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;
    let channel_2_id = server
        .make_channel(
            "channel-2",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    // Clients A, B, and C join a channel.
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);
    for (call, cx) in [
        (&active_call_a, &mut cx_a),
        (&active_call_b, &mut cx_b),
        (&active_call_c, &mut cx_c),
    ] {
        call.update(*cx, |call, cx| call.join_channel(channel_1_id, cx))
            .await
            .unwrap();
    }
    deterministic.run_until_parked();

    // Clients A, B, and C all open their own unshared projects.
    client_a.fs().insert_tree("/a", json!({})).await;
    client_b.fs().insert_tree("/b", json!({})).await;
    client_c.fs().insert_tree("/c", json!({})).await;
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    let (project_b, _) = client_b.build_local_project("/b", cx_b).await;
    let (project_c, _) = client_b.build_local_project("/c", cx_c).await;
    let workspace_a = client_a.build_workspace(&project_a, cx_a).root(cx_a);
    let workspace_b = client_b.build_workspace(&project_b, cx_b).root(cx_b);
    let _workspace_c = client_c.build_workspace(&project_c, cx_c).root(cx_c);

    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    // Client A opens the notes for channel 1.
    let channel_view_1_a = cx_a
        .update(|cx| ChannelView::open(channel_1_id, workspace_a.clone(), cx))
        .await
        .unwrap();
    channel_view_1_a.update(cx_a, |notes, cx| {
        assert_eq!(notes.channel(cx).name, "channel-1");
        notes.editor.update(cx, |editor, cx| {
            editor.insert("Hello from A.", cx);
            editor.change_selections(None, cx, |selections| {
                selections.select_ranges(vec![3..4]);
            });
        });
    });

    // Client B follows client A.
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace
                .toggle_follow(client_a.peer_id().unwrap(), cx)
                .unwrap()
        })
        .await
        .unwrap();

    // Client B is taken to the notes for channel 1, with the same
    // text selected as client A.
    deterministic.run_until_parked();
    let channel_view_1_b = workspace_b.read_with(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.leader_for_pane(workspace.active_pane()),
            Some(client_a.peer_id().unwrap())
        );
        workspace
            .active_item(cx)
            .expect("no active item")
            .downcast::<ChannelView>()
            .expect("active item is not a channel view")
    });
    channel_view_1_b.read_with(cx_b, |notes, cx| {
        assert_eq!(notes.channel(cx).name, "channel-1");
        let editor = notes.editor.read(cx);
        assert_eq!(editor.text(cx), "Hello from A.");
        assert_eq!(editor.selections.ranges::<usize>(cx), &[3..4]);
    });

    // Client A opens the notes for channel 2.
    let channel_view_2_a = cx_a
        .update(|cx| ChannelView::open(channel_2_id, workspace_a.clone(), cx))
        .await
        .unwrap();
    channel_view_2_a.read_with(cx_a, |notes, cx| {
        assert_eq!(notes.channel(cx).name, "channel-2");
    });

    // Client B is taken to the notes for channel 2.
    deterministic.run_until_parked();
    let channel_view_2_b = workspace_b.read_with(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.leader_for_pane(workspace.active_pane()),
            Some(client_a.peer_id().unwrap())
        );
        workspace
            .active_item(cx)
            .expect("no active item")
            .downcast::<ChannelView>()
            .expect("active item is not a channel view")
    });
    channel_view_2_b.read_with(cx_b, |notes, cx| {
        assert_eq!(notes.channel(cx).name, "channel-2");
    });
}

#[gpui::test]
async fn test_channel_buffer_changes(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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

    // Client A makes an edit, and client B should see that the note has changed.
    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "1")], None, cx);
        })
    });
    deterministic.run_until_parked();

    let has_buffer_changed = cx_b.read(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_channel_buffer_changed(channel_id)
            .unwrap()
    });

    assert!(has_buffer_changed);

    // Opening the buffer should clear the changed flag.
    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    deterministic.run_until_parked();

    let has_buffer_changed = cx_b.read(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_channel_buffer_changed(channel_id)
            .unwrap()
    });

    assert!(!has_buffer_changed);

    // Editing the channel while the buffer is open shuold not show that the buffer has changed.
    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "2")], None, cx);
        })
    });
    deterministic.run_until_parked();

    let has_buffer_changed = cx_b.read(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_channel_buffer_changed(channel_id)
            .unwrap()
    });

    assert!(!has_buffer_changed);

    // Closing the buffer should re-enable change tracking
    cx_b.update(|_| {
        drop(channel_buffer_b);
    });

    deterministic.run_until_parked();

    channel_buffer_a.update(cx_a, |buffer, cx| {
        buffer.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "3")], None, cx);
        })
    });
    deterministic.run_until_parked();

    let has_buffer_changed = cx_b.read(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_channel_buffer_changed(channel_id)
            .unwrap()
    });

    assert!(has_buffer_changed);
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
        ids.into_iter().map(|id| id.unwrap()).collect::<Vec<_>>()
    );
}

fn buffer_text(channel_buffer: &ModelHandle<language::Buffer>, cx: &mut TestAppContext) -> String {
    channel_buffer.read_with(cx, |buffer, _| buffer.text())
}
