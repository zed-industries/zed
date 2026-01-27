use crate::{RoomParticipants, TestServer, room_participants};
use call::ActiveCall;
use channel::{ChannelMembership, ChannelStore};
use client::{ChannelId, User};
use collab::{
    db::{self, UserId},
    rpc::RECONNECT_TIMEOUT,
};
use futures::future::try_join_all;
use gpui::{BackgroundExecutor, Entity, SharedString, TestAppContext};
use rpc::{
    RECEIVE_TIMEOUT,
    proto::{self, ChannelRole},
};
use std::sync::Arc;

#[gpui::test]
async fn test_core_channels(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_a_id = client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.create_channel("channel-a", None, cx)
        })
        .await
        .unwrap();
    let channel_b_id = client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.create_channel("channel-b", Some(channel_a_id), cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[
            ExpectedChannel {
                id: channel_a_id,
                name: "channel-a".into(),
                depth: 0,
            },
            ExpectedChannel {
                id: channel_b_id,
                name: "channel-b".into(),
                depth: 1,
            },
        ],
    );

    cx_b.read(|cx| {
        client_b.channel_store().read_with(cx, |channels, _| {
            assert!(channels.ordered_channels().collect::<Vec<_>>().is_empty())
        })
    });

    // Invite client B to channel A as client A.
    client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            assert!(!store.has_pending_channel_invite(channel_a_id, client_b.user_id().unwrap()));

            let invite = store.invite_member(
                channel_a_id,
                client_b.user_id().unwrap(),
                proto::ChannelRole::Member,
                cx,
            );

            // Make sure we're synchronously storing the pending invite
            assert!(store.has_pending_channel_invite(channel_a_id, client_b.user_id().unwrap()));
            invite
        })
        .await
        .unwrap();

    // Client A sees that B has been invited.
    executor.run_until_parked();
    assert_channel_invitations(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".into(),
            depth: 0,
        }],
    );

    let members = client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            assert!(!store.has_pending_channel_invite(channel_a_id, client_b.user_id().unwrap()));
            store.fuzzy_search_members(channel_a_id, "".to_string(), 10, cx)
        })
        .await
        .unwrap();
    assert_members_eq(
        &members,
        &[
            (
                client_a.user_id().unwrap(),
                proto::ChannelRole::Admin,
                proto::channel_member::Kind::Member,
            ),
            (
                client_b.user_id().unwrap(),
                proto::ChannelRole::Member,
                proto::channel_member::Kind::Invitee,
            ),
        ],
    );

    // Client B accepts the invitation.
    client_b
        .channel_store()
        .update(cx_b, |channels, cx| {
            channels.respond_to_channel_invite(channel_a_id, true, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();

    // Client B now sees that they are a member of channel A and its existing subchannels.
    assert_channel_invitations(client_b.channel_store(), cx_b, &[]);
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                id: channel_a_id,
                name: "channel-a".into(),
                depth: 0,
            },
            ExpectedChannel {
                id: channel_b_id,
                name: "channel-b".into(),
                depth: 1,
            },
        ],
    );

    let channel_c_id = client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.create_channel("channel-c", Some(channel_b_id), cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                id: channel_a_id,
                name: "channel-a".into(),
                depth: 0,
            },
            ExpectedChannel {
                id: channel_b_id,
                name: "channel-b".into(),
                depth: 1,
            },
            ExpectedChannel {
                id: channel_c_id,
                name: "channel-c".into(),
                depth: 2,
            },
        ],
    );

    // Update client B's membership to channel A to be an admin.
    client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            store.set_member_role(
                channel_a_id,
                client_b.user_id().unwrap(),
                proto::ChannelRole::Admin,
                cx,
            )
        })
        .await
        .unwrap();
    executor.run_until_parked();

    // Observe that client B is now an admin of channel A, and that
    // their admin privileges extend to subchannels of channel A.
    assert_channel_invitations(client_b.channel_store(), cx_b, &[]);
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                id: channel_a_id,
                name: "channel-a".into(),
                depth: 0,
            },
            ExpectedChannel {
                id: channel_b_id,
                name: "channel-b".into(),
                depth: 1,
            },
            ExpectedChannel {
                id: channel_c_id,
                name: "channel-c".into(),
                depth: 2,
            },
        ],
    );

    // Client A deletes the channel, deletion also deletes subchannels.
    client_a
        .channel_store()
        .update(cx_a, |channel_store, _| {
            channel_store.remove_channel(channel_b_id)
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".into(),
            depth: 0,
        }],
    );
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".into(),
            depth: 0,
        }],
    );

    // Remove client B
    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.remove_member(channel_a_id, client_b.user_id().unwrap(), cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // Client A still has their channel
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".into(),
            depth: 0,
        }],
    );

    // Client B no longer has access to the channel
    assert_channels(client_b.channel_store(), cx_b, &[]);

    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    server
        .app_state
        .db
        .rename_channel(
            db::ChannelId::from_proto(channel_a_id.0),
            UserId::from_proto(client_a.id()),
            "channel-a-renamed",
        )
        .await
        .unwrap();

    server.allow_connections();
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a-renamed".into(),
            depth: 0,
        }],
    );
}

#[track_caller]
fn assert_participants_eq(participants: &[Arc<User>], expected_partitipants: &[u64]) {
    assert_eq!(
        participants.iter().map(|p| p.id).collect::<Vec<_>>(),
        expected_partitipants
    );
}

#[track_caller]
fn assert_members_eq(
    members: &[ChannelMembership],
    expected_members: &[(u64, proto::ChannelRole, proto::channel_member::Kind)],
) {
    assert_eq!(
        members
            .iter()
            .map(|member| (member.user.id, member.role, member.kind))
            .collect::<Vec<_>>(),
        expected_members
    );
}

#[gpui::test]
async fn test_joining_channel_ancestor_member(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;

    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let parent_id = server
        .make_channel("parent", None, (&client_a, cx_a), &mut [(&client_b, cx_b)])
        .await;

    let sub_id = client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.create_channel("sub_channel", Some(parent_id), cx)
        })
        .await
        .unwrap();

    let active_call_b = cx_b.read(ActiveCall::global);

    assert!(
        active_call_b
            .update(cx_b, |active_call, cx| active_call.join_channel(sub_id, cx))
            .await
            .is_ok()
    );
}

#[gpui::test]
async fn test_channel_room(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    let zed_id = server
        .make_channel(
            "zed",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    active_call_a
        .update(cx_a, |active_call, cx| active_call.join_channel(zed_id, cx))
        .await
        .unwrap();

    // Give everyone a chance to observe user A joining
    executor.run_until_parked();
    let room_a =
        cx_a.read(|cx| active_call_a.read_with(cx, |call, _| call.room().unwrap().clone()));
    cx_a.read(|cx| room_a.read_with(cx, |room, cx| assert!(room.is_connected(cx))));

    cx_a.read(|cx| {
        client_a.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_a.user_id().unwrap()],
            );
        })
    });

    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            id: zed_id,
            name: "zed".into(),
            depth: 0,
        }],
    );
    cx_b.read(|cx| {
        client_b.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_a.user_id().unwrap()],
            );
        })
    });

    cx_c.read(|cx| {
        client_c.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_a.user_id().unwrap()],
            );
        })
    });

    active_call_b
        .update(cx_b, |active_call, cx| active_call.join_channel(zed_id, cx))
        .await
        .unwrap();

    executor.run_until_parked();

    cx_a.read(|cx| {
        client_a.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
            );
        })
    });

    cx_b.read(|cx| {
        client_b.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
            );
        })
    });

    cx_c.read(|cx| {
        client_c.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
            );
        })
    });

    let room_a =
        cx_a.read(|cx| active_call_a.read_with(cx, |call, _| call.room().unwrap().clone()));
    cx_a.read(|cx| room_a.read_with(cx, |room, cx| assert!(room.is_connected(cx))));
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec![]
        }
    );

    let room_b =
        cx_b.read(|cx| active_call_b.read_with(cx, |call, _| call.room().unwrap().clone()));
    cx_b.read(|cx| room_b.read_with(cx, |room, cx| assert!(room.is_connected(cx))));
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: vec![]
        }
    );

    // Make sure that leaving and rejoining works

    active_call_a
        .update(cx_a, |active_call, cx| active_call.hang_up(cx))
        .await
        .unwrap();

    executor.run_until_parked();

    cx_a.read(|cx| {
        client_a.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_b.user_id().unwrap()],
            );
        })
    });

    cx_b.read(|cx| {
        client_b.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_b.user_id().unwrap()],
            );
        })
    });

    cx_c.read(|cx| {
        client_c.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_b.user_id().unwrap()],
            );
        })
    });

    active_call_b
        .update(cx_b, |active_call, cx| active_call.hang_up(cx))
        .await
        .unwrap();

    executor.run_until_parked();

    cx_a.read(|cx| {
        client_a.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(channels.channel_participants(zed_id), &[]);
        })
    });

    cx_b.read(|cx| {
        client_b.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(channels.channel_participants(zed_id), &[]);
        })
    });

    cx_c.read(|cx| {
        client_c.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(channels.channel_participants(zed_id), &[]);
        })
    });

    active_call_a
        .update(cx_a, |active_call, cx| active_call.join_channel(zed_id, cx))
        .await
        .unwrap();

    active_call_b
        .update(cx_b, |active_call, cx| active_call.join_channel(zed_id, cx))
        .await
        .unwrap();

    executor.run_until_parked();

    let room_a =
        cx_a.read(|cx| active_call_a.read_with(cx, |call, _| call.room().unwrap().clone()));
    cx_a.read(|cx| room_a.read_with(cx, |room, cx| assert!(room.is_connected(cx))));
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec![]
        }
    );

    let room_b =
        cx_b.read(|cx| active_call_b.read_with(cx, |call, _| call.room().unwrap().clone()));
    cx_b.read(|cx| room_b.read_with(cx, |room, cx| assert!(room.is_connected(cx))));
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: vec![]
        }
    );
}

#[gpui::test]
async fn test_channel_jumping(executor: BackgroundExecutor, cx_a: &mut TestAppContext) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;

    let zed_id = server
        .make_channel("zed", None, (&client_a, cx_a), &mut [])
        .await;
    let rust_id = server
        .make_channel("rust", None, (&client_a, cx_a), &mut [])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);

    active_call_a
        .update(cx_a, |active_call, cx| active_call.join_channel(zed_id, cx))
        .await
        .unwrap();

    // Give everything a chance to observe user A joining
    executor.run_until_parked();

    cx_a.read(|cx| {
        client_a.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(zed_id),
                &[client_a.user_id().unwrap()],
            );
            assert_participants_eq(channels.channel_participants(rust_id), &[]);
        })
    });

    active_call_a
        .update(cx_a, |active_call, cx| {
            active_call.join_channel(rust_id, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    cx_a.read(|cx| {
        client_a.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(channels.channel_participants(zed_id), &[]);
            assert_participants_eq(
                channels.channel_participants(rust_id),
                &[client_a.user_id().unwrap()],
            );
        })
    });
}

#[gpui::test]
async fn test_permissions_update_while_invited(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let rust_id = server
        .make_channel("rust", None, (&client_a, cx_a), &mut [])
        .await;

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.invite_member(
                rust_id,
                client_b.user_id().unwrap(),
                proto::ChannelRole::Member,
                cx,
            )
        })
        .await
        .unwrap();

    executor.run_until_parked();

    assert_channel_invitations(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            depth: 0,
            id: rust_id,
            name: "rust".into(),
        }],
    );
    assert_channels(client_b.channel_store(), cx_b, &[]);

    // Update B's invite before they've accepted it
    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.set_member_role(
                rust_id,
                client_b.user_id().unwrap(),
                proto::ChannelRole::Admin,
                cx,
            )
        })
        .await
        .unwrap();

    executor.run_until_parked();

    assert_channel_invitations(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            depth: 0,
            id: rust_id,
            name: "rust".into(),
        }],
    );
    assert_channels(client_b.channel_store(), cx_b, &[]);
}

#[gpui::test]
async fn test_channel_rename(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let rust_id = server
        .make_channel("rust", None, (&client_a, cx_a), &mut [(&client_b, cx_b)])
        .await;

    // Rename the channel
    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.rename(rust_id, "#rust-archive", cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // Client A sees the channel with its new name.
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[ExpectedChannel {
            depth: 0,
            id: rust_id,
            name: "rust-archive".into(),
        }],
    );

    // Client B sees the channel with its new name.
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            depth: 0,
            id: rust_id,
            name: "rust-archive".into(),
        }],
    );
}

#[gpui::test]
async fn test_call_from_channel(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    let channel_id = server
        .make_channel(
            "x",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    active_call_a
        .update(cx_a, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();

    // Client A calls client B while in the channel.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();

    // Client B accepts the call.
    executor.run_until_parked();
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    // Client B sees that they are now in the channel
    executor.run_until_parked();
    cx_b.read(|cx| {
        active_call_b.read_with(cx, |call, cx| {
            assert_eq!(call.channel_id(cx), Some(channel_id));
        })
    });
    cx_b.read(|cx| {
        client_b.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(channel_id),
                &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
            );
        })
    });

    // Clients A and C also see that client B is in the channel.
    cx_a.read(|cx| {
        client_a.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(channel_id),
                &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
            );
        })
    });
    cx_c.read(|cx| {
        client_c.channel_store().read_with(cx, |channels, _| {
            assert_participants_eq(
                channels.channel_participants(channel_id),
                &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
            );
        })
    });
}

#[gpui::test]
async fn test_lost_channel_creation(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    let channel_id = server
        .make_channel("x", None, (&client_a, cx_a), &mut [])
        .await;

    // Invite a member
    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.invite_member(
                channel_id,
                client_b.user_id().unwrap(),
                proto::ChannelRole::Member,
                cx,
            )
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // Sanity check, B has the invitation
    assert_channel_invitations(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            depth: 0,
            id: channel_id,
            name: "x".into(),
        }],
    );

    // A creates a subchannel while the invite is still pending.
    let subchannel_id = client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.create_channel("subchannel", Some(channel_id), cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // Make sure A sees their new channel
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[
            ExpectedChannel {
                depth: 0,
                id: channel_id,
                name: "x".into(),
            },
            ExpectedChannel {
                depth: 1,
                id: subchannel_id,
                name: "subchannel".into(),
            },
        ],
    );

    // Client B accepts the invite
    client_b
        .channel_store()
        .update(cx_b, |channel_store, cx| {
            channel_store.respond_to_channel_invite(channel_id, true, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // Client B should now see the channel
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                depth: 0,
                id: channel_id,
                name: "x".into(),
            },
            ExpectedChannel {
                depth: 1,
                id: subchannel_id,
                name: "subchannel".into(),
            },
        ],
    );
}

#[gpui::test]
async fn test_channel_link_notifications(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    let user_b = client_b.user_id().unwrap();
    let user_c = client_c.user_id().unwrap();

    let channels = server
        .make_channel_tree(&[("zed", None)], (&client_a, cx_a))
        .await;
    let zed_channel = channels[0];

    try_join_all(client_a.channel_store().update(cx_a, |channel_store, cx| {
        [
            channel_store.set_channel_visibility(zed_channel, proto::ChannelVisibility::Public, cx),
            channel_store.invite_member(zed_channel, user_b, proto::ChannelRole::Member, cx),
            channel_store.invite_member(zed_channel, user_c, proto::ChannelRole::Guest, cx),
        ]
    }))
    .await
    .unwrap();

    executor.run_until_parked();

    client_b
        .channel_store()
        .update(cx_b, |channel_store, cx| {
            channel_store.respond_to_channel_invite(zed_channel, true, cx)
        })
        .await
        .unwrap();

    client_c
        .channel_store()
        .update(cx_c, |channel_store, cx| {
            channel_store.respond_to_channel_invite(zed_channel, true, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // we have an admin (a), member (b) and guest (c) all part of the zed channel.

    // create a new private channel, make it public, and move it under the previous one, and verify it shows for b and not c
    let active_channel = client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.create_channel("active", Some(zed_channel), cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // the new channel shows for b and not c
    assert_channels_list_shape(
        client_a.channel_store(),
        cx_a,
        &[(zed_channel, 0), (active_channel, 1)],
    );
    assert_channels_list_shape(
        client_b.channel_store(),
        cx_b,
        &[(zed_channel, 0), (active_channel, 1)],
    );
    assert_channels_list_shape(client_c.channel_store(), cx_c, &[(zed_channel, 0)]);

    let vim_channel = client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.create_channel("vim", Some(zed_channel), cx)
        })
        .await
        .unwrap();

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.set_channel_visibility(vim_channel, proto::ChannelVisibility::Public, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // the new channel shows for b and c
    assert_channels_list_shape(
        client_a.channel_store(),
        cx_a,
        &[(zed_channel, 0), (active_channel, 1), (vim_channel, 1)],
    );
    assert_channels_list_shape(
        client_b.channel_store(),
        cx_b,
        &[(zed_channel, 0), (active_channel, 1), (vim_channel, 1)],
    );
    assert_channels_list_shape(
        client_c.channel_store(),
        cx_c,
        &[(zed_channel, 0), (vim_channel, 1)],
    );

    let helix_channel = client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.create_channel("helix", Some(zed_channel), cx)
        })
        .await
        .unwrap();

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.move_channel(helix_channel, vim_channel, cx)
        })
        .await
        .unwrap();

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.set_channel_visibility(
                helix_channel,
                proto::ChannelVisibility::Public,
                cx,
            )
        })
        .await
        .unwrap();
    cx_a.run_until_parked();

    // the new channel shows for b and c
    assert_channels_list_shape(
        client_b.channel_store(),
        cx_b,
        &[
            (zed_channel, 0),
            (active_channel, 1),
            (vim_channel, 1),
            (helix_channel, 2),
        ],
    );
    assert_channels_list_shape(
        client_c.channel_store(),
        cx_c,
        &[(zed_channel, 0), (vim_channel, 1), (helix_channel, 2)],
    );
}

#[gpui::test]
async fn test_channel_membership_notifications(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_c").await;

    let user_b = client_b.user_id().unwrap();

    let channels = server
        .make_channel_tree(
            &[("zed", None), ("vim", Some("zed")), ("opensource", None)],
            (&client_a, cx_a),
        )
        .await;
    let zed_channel = channels[0];
    let vim_channel = channels[1];
    let opensource_channel = channels[2];

    try_join_all(client_a.channel_store().update(cx_a, |channel_store, cx| {
        [
            channel_store.set_channel_visibility(zed_channel, proto::ChannelVisibility::Public, cx),
            channel_store.set_channel_visibility(vim_channel, proto::ChannelVisibility::Public, cx),
            channel_store.invite_member(zed_channel, user_b, proto::ChannelRole::Admin, cx),
            channel_store.invite_member(opensource_channel, user_b, proto::ChannelRole::Member, cx),
        ]
    }))
    .await
    .unwrap();

    executor.run_until_parked();

    client_b
        .channel_store()
        .update(cx_b, |channel_store, cx| {
            channel_store.respond_to_channel_invite(zed_channel, true, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // we have an admin (a), and a guest (b) with access to all of zed, and membership in vim.
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                depth: 0,
                id: zed_channel,
                name: "zed".into(),
            },
            ExpectedChannel {
                depth: 1,
                id: vim_channel,
                name: "vim".into(),
            },
        ],
    );

    client_b.channel_store().update(cx_b, |channel_store, _| {
        channel_store.is_channel_admin(zed_channel)
    });

    client_b
        .channel_store()
        .update(cx_b, |channel_store, cx| {
            channel_store.respond_to_channel_invite(opensource_channel, true, cx)
        })
        .await
        .unwrap();

    cx_a.run_until_parked();

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.set_member_role(opensource_channel, user_b, ChannelRole::Admin, cx)
        })
        .await
        .unwrap();

    cx_a.run_until_parked();

    client_b.channel_store().update(cx_b, |channel_store, _| {
        channel_store.is_channel_admin(opensource_channel)
    });
}

#[gpui::test]
async fn test_guest_access(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channels = server
        .make_channel_tree(
            &[("channel-a", None), ("channel-b", Some("channel-a"))],
            (&client_a, cx_a),
        )
        .await;
    let channel_a = channels[0];
    let channel_b = channels[1];

    let active_call_b = cx_b.read(ActiveCall::global);

    // Non-members should not be allowed to join
    assert!(
        active_call_b
            .update(cx_b, |call, cx| call.join_channel(channel_a, cx))
            .await
            .is_err()
    );

    // Make channels A and B public
    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.set_channel_visibility(channel_a, proto::ChannelVisibility::Public, cx)
        })
        .await
        .unwrap();
    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.set_channel_visibility(channel_b, proto::ChannelVisibility::Public, cx)
        })
        .await
        .unwrap();

    // Client B joins channel A as a guest
    active_call_b
        .update(cx_b, |call, cx| call.join_channel(channel_a, cx))
        .await
        .unwrap();

    executor.run_until_parked();
    assert_channels_list_shape(
        client_a.channel_store(),
        cx_a,
        &[(channel_a, 0), (channel_b, 1)],
    );
    assert_channels_list_shape(
        client_b.channel_store(),
        cx_b,
        &[(channel_a, 0), (channel_b, 1)],
    );

    client_a.channel_store().update(cx_a, |channel_store, _| {
        let participants = channel_store.channel_participants(channel_a);
        assert_eq!(participants.len(), 1);
        assert_eq!(participants[0].id, client_b.user_id().unwrap());
    });
}

#[gpui::test]
async fn test_invite_access(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channels = server
        .make_channel_tree(
            &[("channel-a", None), ("channel-b", Some("channel-a"))],
            (&client_a, cx_a),
        )
        .await;
    let channel_a_id = channels[0];
    let channel_b_id = channels[0];

    let active_call_b = cx_b.read(ActiveCall::global);

    // should not be allowed to join
    assert!(
        active_call_b
            .update(cx_b, |call, cx| call.join_channel(channel_b_id, cx))
            .await
            .is_err()
    );

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.invite_member(
                channel_a_id,
                client_b.user_id().unwrap(),
                ChannelRole::Member,
                cx,
            )
        })
        .await
        .unwrap();

    active_call_b
        .update(cx_b, |call, cx| call.join_channel(channel_b_id, cx))
        .await
        .unwrap();

    executor.run_until_parked();

    client_b.channel_store().update(cx_b, |channel_store, _| {
        assert!(channel_store.channel_for_id(channel_b_id).is_some());
        assert!(channel_store.channel_for_id(channel_a_id).is_some());
    });

    client_a.channel_store().update(cx_a, |channel_store, _| {
        let participants = channel_store.channel_participants(channel_b_id);
        assert_eq!(participants.len(), 1);
        assert_eq!(participants[0].id, client_b.user_id().unwrap());
    })
}

#[gpui::test]
async fn test_leave_channel(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let (_server, _client_a, client_b, channel_id) = TestServer::start2(cx_a, cx_b).await;

    client_b
        .channel_store()
        .update(cx_b, |channel_store, cx| {
            channel_store.remove_member(channel_id, client_b.user_id().unwrap(), cx)
        })
        .await
        .unwrap();

    cx_a.run_until_parked();

    assert_eq!(
        client_b
            .channel_store()
            .read_with(cx_b, |store, _| store.channels().count()),
        0
    );
}

#[gpui::test]
async fn test_channel_moving(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    _cx_b: &mut TestAppContext,
    _cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;

    let channels = server
        .make_channel_tree(
            &[
                ("channel-a", None),
                ("channel-b", Some("channel-a")),
                ("channel-c", Some("channel-b")),
                ("channel-d", Some("channel-c")),
            ],
            (&client_a, cx_a),
        )
        .await;
    let channel_a_id = channels[0];
    let channel_b_id = channels[1];
    let channel_c_id = channels[2];
    let channel_d_id = channels[3];

    // Current shape:
    // a - b - c - d
    assert_channels_list_shape(
        client_a.channel_store(),
        cx_a,
        &[
            (channel_a_id, 0),
            (channel_b_id, 1),
            (channel_c_id, 2),
            (channel_d_id, 3),
        ],
    );

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.move_channel(channel_d_id, channel_b_id, cx)
        })
        .await
        .unwrap();

    // Current shape:
    //       /- d
    // a - b -- c
    assert_channels_list_shape(
        client_a.channel_store(),
        cx_a,
        &[
            (channel_a_id, 0),
            (channel_b_id, 1),
            (channel_c_id, 2),
            (channel_d_id, 2),
        ],
    );
}

#[derive(Debug, PartialEq)]
struct ExpectedChannel {
    depth: usize,
    id: ChannelId,
    name: SharedString,
}

#[track_caller]
fn assert_channel_invitations(
    channel_store: &Entity<ChannelStore>,
    cx: &TestAppContext,
    expected_channels: &[ExpectedChannel],
) {
    let actual = cx.read(|cx| {
        channel_store.read_with(cx, |store, _| {
            store
                .channel_invitations()
                .iter()
                .map(|channel| ExpectedChannel {
                    depth: 0,
                    name: channel.name.clone(),
                    id: channel.id,
                })
                .collect::<Vec<_>>()
        })
    });
    assert_eq!(actual, expected_channels);
}

#[track_caller]
fn assert_channels(
    channel_store: &Entity<ChannelStore>,
    cx: &TestAppContext,
    expected_channels: &[ExpectedChannel],
) {
    let actual = cx.read(|cx| {
        channel_store.read_with(cx, |store, _| {
            store
                .ordered_channels()
                .map(|(depth, channel)| ExpectedChannel {
                    depth,
                    name: channel.name.clone(),
                    id: channel.id,
                })
                .collect::<Vec<_>>()
        })
    });
    pretty_assertions::assert_eq!(actual, expected_channels);
}

#[track_caller]
fn assert_channels_list_shape(
    channel_store: &Entity<ChannelStore>,
    cx: &TestAppContext,
    expected_channels: &[(ChannelId, usize)],
) {
    let actual = cx.read(|cx| {
        channel_store.read_with(cx, |store, _| {
            store
                .ordered_channels()
                .map(|(depth, channel)| (channel.id, depth))
                .collect::<Vec<_>>()
        })
    });
    pretty_assertions::assert_eq!(actual, expected_channels);
}
