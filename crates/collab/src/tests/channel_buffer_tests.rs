use crate::{
    rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    tests::TestServer,
};
use call::ActiveCall;
use channel::Channel;
use client::UserId;
use collab_ui::channel_view::ChannelView;
use collections::HashMap;
use futures::future;
use gpui::{executor::Deterministic, ModelHandle, TestAppContext};
use rpc::{proto, RECEIVE_TIMEOUT};
use serde_json::json;
use std::sync::Arc;

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
            &[client_b.user_id(), client_a.user_id()],
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
async fn test_channel_buffer_replica_ids(
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

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);

    // Clients A and B join a channel.
    active_call_a
        .update(cx_a, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();
    active_call_b
        .update(cx_b, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();

    // Clients A, B, and C join a channel buffer
    // C first so that the replica IDs in the project and the channel buffer are different
    let channel_buffer_c = client_c
        .channel_store()
        .update(cx_c, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    let channel_buffer_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();
    let channel_buffer_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_buffer(channel_id, cx))
        .await
        .unwrap();

    // Client B shares a project
    client_b
        .fs()
        .insert_tree("/dir", json!({ "file.txt": "contents" }))
        .await;
    let (project_b, _) = client_b.build_local_project("/dir", cx_b).await;
    let shared_project_id = active_call_b
        .update(cx_b, |call, cx| call.share_project(project_b.clone(), cx))
        .await
        .unwrap();

    // Client A joins the project
    let project_a = client_a.build_remote_project(shared_project_id, cx_a).await;
    deterministic.run_until_parked();

    // Client C is in a separate project.
    client_c.fs().insert_tree("/dir", json!({})).await;
    let (separate_project_c, _) = client_c.build_local_project("/dir", cx_c).await;

    // Note that each user has a different replica id in the projects vs the
    // channel buffer.
    channel_buffer_a.read_with(cx_a, |channel_buffer, cx| {
        assert_eq!(project_a.read(cx).replica_id(), 1);
        assert_eq!(channel_buffer.buffer().read(cx).replica_id(), 2);
    });
    channel_buffer_b.read_with(cx_b, |channel_buffer, cx| {
        assert_eq!(project_b.read(cx).replica_id(), 0);
        assert_eq!(channel_buffer.buffer().read(cx).replica_id(), 1);
    });
    channel_buffer_c.read_with(cx_c, |channel_buffer, cx| {
        // C is not in the project
        assert_eq!(channel_buffer.buffer().read(cx).replica_id(), 0);
    });

    let channel_window_a =
        cx_a.add_window(|cx| ChannelView::new(project_a.clone(), channel_buffer_a.clone(), cx));
    let channel_window_b =
        cx_b.add_window(|cx| ChannelView::new(project_b.clone(), channel_buffer_b.clone(), cx));
    let channel_window_c = cx_c.add_window(|cx| {
        ChannelView::new(separate_project_c.clone(), channel_buffer_c.clone(), cx)
    });

    let channel_view_a = channel_window_a.root(cx_a);
    let channel_view_b = channel_window_b.root(cx_b);
    let channel_view_c = channel_window_c.root(cx_c);

    // For clients A and B, the replica ids in the channel buffer are mapped
    // so that they match the same users' replica ids in their shared project.
    channel_view_a.read_with(cx_a, |view, cx| {
        assert_eq!(
            view.editor.read(cx).replica_id_map().unwrap(),
            &[(1, 0), (2, 1)].into_iter().collect::<HashMap<_, _>>()
        );
    });
    channel_view_b.read_with(cx_b, |view, cx| {
        assert_eq!(
            view.editor.read(cx).replica_id_map().unwrap(),
            &[(1, 0), (2, 1)].into_iter().collect::<HashMap<u16, u16>>(),
        )
    });

    // Client C only sees themself, as they're not part of any shared project
    channel_view_c.read_with(cx_c, |view, cx| {
        assert_eq!(
            view.editor.read(cx).replica_id_map().unwrap(),
            &[(0, 0)].into_iter().collect::<HashMap<u16, u16>>(),
        );
    });

    // Client C joins the project that clients A and B are in.
    active_call_c
        .update(cx_c, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();
    let project_c = client_c.build_remote_project(shared_project_id, cx_c).await;
    deterministic.run_until_parked();
    project_c.read_with(cx_c, |project, _| {
        assert_eq!(project.replica_id(), 2);
    });

    // For clients A and B, client C's replica id in the channel buffer is
    // now mapped to their replica id in the shared project.
    channel_view_a.read_with(cx_a, |view, cx| {
        assert_eq!(
            view.editor.read(cx).replica_id_map().unwrap(),
            &[(1, 0), (2, 1), (0, 2)]
                .into_iter()
                .collect::<HashMap<_, _>>()
        );
    });
    channel_view_b.read_with(cx_b, |view, cx| {
        assert_eq!(
            view.editor.read(cx).replica_id_map().unwrap(),
            &[(1, 0), (2, 1), (0, 2)]
                .into_iter()
                .collect::<HashMap<_, _>>(),
        )
    });
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
            &Channel {
                id: channel_id,
                name: "the-channel".to_string()
            }
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
            &Channel {
                id: channel_id,
                name: "the-channel".to_string()
            }
        );
        assert!(!buffer.is_connected());
    });
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
            assert_eq!(
                buffer_a
                    .collaborators()
                    .iter()
                    .map(|c| c.user_id)
                    .collect::<Vec<_>>(),
                vec![client_a.user_id().unwrap(), client_b.user_id().unwrap()]
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

    let channel_1_id = server
        .make_channel(
            "channel-1",
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;
    let channel_2_id = server
        .make_channel(
            "channel-2",
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
    let workspace_c = client_c.build_workspace(&project_c, cx_c).root(cx_c);

    // Client A opens the notes for channel 1.
    let channel_view_1_a = cx_a
        .update(|cx| {
            ChannelView::open(
                channel_1_id,
                workspace_a.read(cx).active_pane().clone(),
                workspace_a.clone(),
                cx,
            )
        })
        .await
        .unwrap();

    // Client B follows client A.
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace
                .toggle_follow(client_a.peer_id().unwrap(), cx)
                .unwrap()
        })
        .await
        .unwrap();

    deterministic.run_until_parked();
    workspace_b.read_with(cx_b, |workspace, _| {
        assert_eq!(
            workspace.leader_for_pane(workspace.active_pane()),
            Some(client_a.peer_id().unwrap())
        );
    });
}

#[track_caller]
fn assert_collaborators(collaborators: &[proto::Collaborator], ids: &[Option<UserId>]) {
    assert_eq!(
        collaborators
            .into_iter()
            .map(|collaborator| collaborator.user_id)
            .collect::<Vec<_>>(),
        ids.into_iter().map(|id| id.unwrap()).collect::<Vec<_>>()
    );
}

fn buffer_text(channel_buffer: &ModelHandle<language::Buffer>, cx: &mut TestAppContext) -> String {
    channel_buffer.read_with(cx, |buffer, _| buffer.text())
}
