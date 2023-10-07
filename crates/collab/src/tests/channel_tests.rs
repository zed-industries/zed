use crate::{
    rpc::RECONNECT_TIMEOUT,
    tests::{room_participants, RoomParticipants, TestServer},
};
use call::ActiveCall;
use channel::{ChannelId, ChannelMembership, ChannelStore};
use client::User;
use gpui::{executor::Deterministic, ModelHandle, TestAppContext};
use rpc::{proto, RECEIVE_TIMEOUT};
use std::sync::Arc;

#[gpui::test]
async fn test_core_channels(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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

    deterministic.run_until_parked();
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[
            ExpectedChannel {
                id: channel_a_id,
                name: "channel-a".to_string(),
                depth: 0,
                user_is_admin: true,
            },
            ExpectedChannel {
                id: channel_b_id,
                name: "channel-b".to_string(),
                depth: 1,
                user_is_admin: true,
            },
        ],
    );

    client_b.channel_store().read_with(cx_b, |channels, _| {
        assert!(channels
            .channel_dag_entries()
            .collect::<Vec<_>>()
            .is_empty())
    });

    // Invite client B to channel A as client A.
    client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            assert!(!store.has_pending_channel_invite(channel_a_id, client_b.user_id().unwrap()));

            let invite = store.invite_member(channel_a_id, client_b.user_id().unwrap(), false, cx);

            // Make sure we're synchronously storing the pending invite
            assert!(store.has_pending_channel_invite(channel_a_id, client_b.user_id().unwrap()));
            invite
        })
        .await
        .unwrap();

    // Client A sees that B has been invited.
    deterministic.run_until_parked();
    assert_channel_invitations(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".to_string(),
            depth: 0,
            user_is_admin: false,
        }],
    );

    let members = client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            assert!(!store.has_pending_channel_invite(channel_a_id, client_b.user_id().unwrap()));
            store.get_channel_member_details(channel_a_id, cx)
        })
        .await
        .unwrap();
    assert_members_eq(
        &members,
        &[
            (
                client_a.user_id().unwrap(),
                true,
                proto::channel_member::Kind::Member,
            ),
            (
                client_b.user_id().unwrap(),
                false,
                proto::channel_member::Kind::Invitee,
            ),
        ],
    );

    // Client B accepts the invitation.
    client_b
        .channel_store()
        .update(cx_b, |channels, _| {
            channels.respond_to_channel_invite(channel_a_id, true)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();

    // Client B now sees that they are a member of channel A and its existing subchannels.
    assert_channel_invitations(client_b.channel_store(), cx_b, &[]);
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                id: channel_a_id,
                name: "channel-a".to_string(),
                user_is_admin: false,
                depth: 0,
            },
            ExpectedChannel {
                id: channel_b_id,
                name: "channel-b".to_string(),
                user_is_admin: false,
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

    deterministic.run_until_parked();
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                id: channel_a_id,
                name: "channel-a".to_string(),
                user_is_admin: false,
                depth: 0,
            },
            ExpectedChannel {
                id: channel_b_id,
                name: "channel-b".to_string(),
                user_is_admin: false,
                depth: 1,
            },
            ExpectedChannel {
                id: channel_c_id,
                name: "channel-c".to_string(),
                user_is_admin: false,
                depth: 2,
            },
        ],
    );

    // Update client B's membership to channel A to be an admin.
    client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            store.set_member_admin(channel_a_id, client_b.user_id().unwrap(), true, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();

    // Observe that client B is now an admin of channel A, and that
    // their admin priveleges extend to subchannels of channel A.
    assert_channel_invitations(client_b.channel_store(), cx_b, &[]);
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                id: channel_a_id,
                name: "channel-a".to_string(),
                depth: 0,
                user_is_admin: true,
            },
            ExpectedChannel {
                id: channel_b_id,
                name: "channel-b".to_string(),
                depth: 1,
                user_is_admin: true,
            },
            ExpectedChannel {
                id: channel_c_id,
                name: "channel-c".to_string(),
                depth: 2,
                user_is_admin: true,
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

    deterministic.run_until_parked();
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".to_string(),
            depth: 0,
            user_is_admin: true,
        }],
    );
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".to_string(),
            depth: 0,
            user_is_admin: true,
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

    deterministic.run_until_parked();

    // Client A still has their channel
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".to_string(),
            depth: 0,
            user_is_admin: true,
        }],
    );

    // Client B no longer has access to the channel
    assert_channels(client_b.channel_store(), cx_b, &[]);

    // When disconnected, client A sees no channels.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    assert_channels(client_a.channel_store(), cx_a, &[]);

    server.allow_connections();
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[ExpectedChannel {
            id: channel_a_id,
            name: "channel-a".to_string(),
            depth: 0,
            user_is_admin: true,
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
    expected_members: &[(u64, bool, proto::channel_member::Kind)],
) {
    assert_eq!(
        members
            .iter()
            .map(|member| (member.user.id, member.admin, member.kind))
            .collect::<Vec<_>>(),
        expected_members
    );
}

#[gpui::test]
async fn test_joining_channel_ancestor_member(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;

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

    assert!(active_call_b
        .update(cx_b, |active_call, cx| active_call.join_channel(sub_id, cx))
        .await
        .is_ok());
}

#[gpui::test]
async fn test_channel_room(
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
    deterministic.run_until_parked();

    client_a.channel_store().read_with(cx_a, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_a.user_id().unwrap()],
        );
    });

    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            id: zed_id,
            name: "zed".to_string(),
            depth: 0,
            user_is_admin: false,
        }],
    );
    client_b.channel_store().read_with(cx_b, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_a.user_id().unwrap()],
        );
    });

    client_c.channel_store().read_with(cx_c, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_a.user_id().unwrap()],
        );
    });

    active_call_b
        .update(cx_b, |active_call, cx| active_call.join_channel(zed_id, cx))
        .await
        .unwrap();

    deterministic.run_until_parked();

    client_a.channel_store().read_with(cx_a, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
        );
    });

    client_b.channel_store().read_with(cx_b, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
        );
    });

    client_c.channel_store().read_with(cx_c, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
        );
    });

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    room_a.read_with(cx_a, |room, _| assert!(room.is_connected()));
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec![]
        }
    );

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    room_b.read_with(cx_b, |room, _| assert!(room.is_connected()));
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

    deterministic.run_until_parked();

    client_a.channel_store().read_with(cx_a, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_b.user_id().unwrap()],
        );
    });

    client_b.channel_store().read_with(cx_b, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_b.user_id().unwrap()],
        );
    });

    client_c.channel_store().read_with(cx_c, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_b.user_id().unwrap()],
        );
    });

    active_call_b
        .update(cx_b, |active_call, cx| active_call.hang_up(cx))
        .await
        .unwrap();

    deterministic.run_until_parked();

    client_a.channel_store().read_with(cx_a, |channels, _| {
        assert_participants_eq(channels.channel_participants(zed_id), &[]);
    });

    client_b.channel_store().read_with(cx_b, |channels, _| {
        assert_participants_eq(channels.channel_participants(zed_id), &[]);
    });

    client_c.channel_store().read_with(cx_c, |channels, _| {
        assert_participants_eq(channels.channel_participants(zed_id), &[]);
    });

    active_call_a
        .update(cx_a, |active_call, cx| active_call.join_channel(zed_id, cx))
        .await
        .unwrap();

    active_call_b
        .update(cx_b, |active_call, cx| active_call.join_channel(zed_id, cx))
        .await
        .unwrap();

    deterministic.run_until_parked();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    room_a.read_with(cx_a, |room, _| assert!(room.is_connected()));
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec![]
        }
    );

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    room_b.read_with(cx_b, |room, _| assert!(room.is_connected()));
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: vec![]
        }
    );
}

#[gpui::test]
async fn test_channel_jumping(deterministic: Arc<Deterministic>, cx_a: &mut TestAppContext) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
    deterministic.run_until_parked();

    client_a.channel_store().read_with(cx_a, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(zed_id),
            &[client_a.user_id().unwrap()],
        );
        assert_participants_eq(channels.channel_participants(rust_id), &[]);
    });

    active_call_a
        .update(cx_a, |active_call, cx| {
            active_call.join_channel(rust_id, cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();

    client_a.channel_store().read_with(cx_a, |channels, _| {
        assert_participants_eq(channels.channel_participants(zed_id), &[]);
        assert_participants_eq(
            channels.channel_participants(rust_id),
            &[client_a.user_id().unwrap()],
        );
    });
}

#[gpui::test]
async fn test_permissions_update_while_invited(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let rust_id = server
        .make_channel("rust", None, (&client_a, cx_a), &mut [])
        .await;

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.invite_member(rust_id, client_b.user_id().unwrap(), false, cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();

    assert_channel_invitations(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            depth: 0,
            id: rust_id,
            name: "rust".to_string(),
            user_is_admin: false,
        }],
    );
    assert_channels(client_b.channel_store(), cx_b, &[]);

    // Update B's invite before they've accepted it
    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.set_member_admin(rust_id, client_b.user_id().unwrap(), true, cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();

    assert_channel_invitations(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            depth: 0,
            id: rust_id,
            name: "rust".to_string(),
            user_is_admin: false,
        }],
    );
    assert_channels(client_b.channel_store(), cx_b, &[]);
}

#[gpui::test]
async fn test_channel_rename(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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

    deterministic.run_until_parked();

    // Client A sees the channel with its new name.
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[ExpectedChannel {
            depth: 0,
            id: rust_id,
            name: "rust-archive".to_string(),
            user_is_admin: true,
        }],
    );

    // Client B sees the channel with its new name.
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            depth: 0,
            id: rust_id,
            name: "rust-archive".to_string(),
            user_is_admin: false,
        }],
    );
}

#[gpui::test]
async fn test_call_from_channel(
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
    deterministic.run_until_parked();
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    // Client B sees that they are now in the channel
    deterministic.run_until_parked();
    active_call_b.read_with(cx_b, |call, cx| {
        assert_eq!(call.channel_id(cx), Some(channel_id));
    });
    client_b.channel_store().read_with(cx_b, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(channel_id),
            &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
        );
    });

    // Clients A and C also see that client B is in the channel.
    client_a.channel_store().read_with(cx_a, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(channel_id),
            &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
        );
    });
    client_c.channel_store().read_with(cx_c, |channels, _| {
        assert_participants_eq(
            channels.channel_participants(channel_id),
            &[client_a.user_id().unwrap(), client_b.user_id().unwrap()],
        );
    });
}

#[gpui::test]
async fn test_lost_channel_creation(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
            channel_store.invite_member(channel_id, client_b.user_id().unwrap(), false, cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();

    // Sanity check, B has the invitation
    assert_channel_invitations(
        client_b.channel_store(),
        cx_b,
        &[ExpectedChannel {
            depth: 0,
            id: channel_id,
            name: "x".to_string(),
            user_is_admin: false,
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

    deterministic.run_until_parked();

    // Make sure A sees their new channel
    assert_channels(
        client_a.channel_store(),
        cx_a,
        &[
            ExpectedChannel {
                depth: 0,
                id: channel_id,
                name: "x".to_string(),
                user_is_admin: true,
            },
            ExpectedChannel {
                depth: 1,
                id: subchannel_id,
                name: "subchannel".to_string(),
                user_is_admin: true,
            },
        ],
    );

    // Client B accepts the invite
    client_b
        .channel_store()
        .update(cx_b, |channel_store, _| {
            channel_store.respond_to_channel_invite(channel_id, true)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();

    // Client B should now see the channel
    assert_channels(
        client_b.channel_store(),
        cx_b,
        &[
            ExpectedChannel {
                depth: 0,
                id: channel_id,
                name: "x".to_string(),
                user_is_admin: false,
            },
            ExpectedChannel {
                depth: 1,
                id: subchannel_id,
                name: "subchannel".to_string(),
                user_is_admin: false,
            },
        ],
    );
}

#[gpui::test]
async fn test_channel_moving(
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
            channel_store.move_channel(channel_d_id, channel_c_id, channel_b_id, cx)
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

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.link_channel(channel_d_id, channel_c_id, cx)
        })
        .await
        .unwrap();

    // Current shape for A:
    //      /------\
    // a - b -- c -- d
    assert_channels_list_shape(
        client_a.channel_store(),
        cx_a,
        &[
            (channel_a_id, 0),
            (channel_b_id, 1),
            (channel_c_id, 2),
            (channel_d_id, 3),
            (channel_d_id, 2),
        ],
    );

    let b_channels = server
        .make_channel_tree(
            &[
                ("channel-mu", None),
                ("channel-gamma", Some("channel-mu")),
                ("channel-epsilon", Some("channel-mu")),
            ],
            (&client_b, cx_b),
        )
        .await;
    let channel_mu_id = b_channels[0];
    let channel_ga_id = b_channels[1];
    let channel_ep_id = b_channels[2];

    // Current shape for B:
    //    /- ep
    // mu -- ga
    assert_channels_list_shape(
        client_b.channel_store(),
        cx_b,
        &[(channel_mu_id, 0), (channel_ep_id, 1), (channel_ga_id, 1)],
    );

    client_a
        .add_admin_to_channel((&client_b, cx_b), channel_b_id, cx_a)
        .await;

    // Current shape for B:
    //    /- ep
    // mu -- ga
    //  /---------\
    // b  -- c  -- d
    assert_channels_list_shape(
        client_b.channel_store(),
        cx_b,
        &[
            // New channels from a
            (channel_b_id, 0),
            (channel_c_id, 1),
            (channel_d_id, 2),
            (channel_d_id, 1),
            // B's old channels
            (channel_mu_id, 0),
            (channel_ep_id, 1),
            (channel_ga_id, 1),
        ],
    );

    client_b
        .add_admin_to_channel((&client_c, cx_c), channel_ep_id, cx_b)
        .await;

    // Current shape for C:
    // - ep
    assert_channels_list_shape(client_c.channel_store(), cx_c, &[(channel_ep_id, 0)]);

    client_b
        .channel_store()
        .update(cx_b, |channel_store, cx| {
            channel_store.link_channel(channel_b_id, channel_ep_id, cx)
        })
        .await
        .unwrap();

    // Current shape for B:
    //              /---------\
    //    /- ep -- b  -- c  -- d
    // mu -- ga
    assert_channels_list_shape(
        client_b.channel_store(),
        cx_b,
        &[
            (channel_mu_id, 0),
            (channel_ep_id, 1),
            (channel_b_id, 2),
            (channel_c_id, 3),
            (channel_d_id, 4),
            (channel_d_id, 3),
            (channel_ga_id, 1),
        ],
    );

    // Current shape for C:
    //        /---------\
    // ep -- b  -- c  -- d
    assert_channels_list_shape(
        client_c.channel_store(),
        cx_c,
        &[
            (channel_ep_id, 0),
            (channel_b_id, 1),
            (channel_c_id, 2),
            (channel_d_id, 3),
            (channel_d_id, 2),
        ],
    );

    client_b
        .channel_store()
        .update(cx_b, |channel_store, cx| {
            channel_store.link_channel(channel_ga_id, channel_b_id, cx)
        })
        .await
        .unwrap();

    // Current shape for B:
    //              /---------\
    //    /- ep -- b  -- c  -- d
    //   /          \
    // mu ---------- ga
    assert_channels_list_shape(
        client_b.channel_store(),
        cx_b,
        &[
            (channel_mu_id, 0),
            (channel_ep_id, 1),
            (channel_b_id, 2),
            (channel_c_id, 3),
            (channel_d_id, 4),
            (channel_d_id, 3),
            (channel_ga_id, 3),
            (channel_ga_id, 1),
        ],
    );

    // Current shape for A:
    //      /------\
    // a - b -- c -- d
    //      \-- ga
    assert_channels_list_shape(
        client_a.channel_store(),
        cx_a,
        &[
            (channel_a_id, 0),
            (channel_b_id, 1),
            (channel_c_id, 2),
            (channel_d_id, 3),
            (channel_d_id, 2),
            (channel_ga_id, 2),
        ],
    );

    // Current shape for C:
    //        /-------\
    // ep -- b -- c -- d
    //        \-- ga
    assert_channels_list_shape(
        client_c.channel_store(),
        cx_c,
        &[
            (channel_ep_id, 0),
            (channel_b_id, 1),
            (channel_c_id, 2),
            (channel_d_id, 3),
            (channel_d_id, 2),
            (channel_ga_id, 2),
        ],
    );
}

#[derive(Debug, PartialEq)]
struct ExpectedChannel {
    depth: usize,
    id: ChannelId,
    name: String,
    user_is_admin: bool,
}

#[track_caller]
fn assert_channel_invitations(
    channel_store: &ModelHandle<ChannelStore>,
    cx: &TestAppContext,
    expected_channels: &[ExpectedChannel],
) {
    let actual = channel_store.read_with(cx, |store, _| {
        store
            .channel_invitations()
            .iter()
            .map(|channel| ExpectedChannel {
                depth: 0,
                name: channel.name.clone(),
                id: channel.id,
                user_is_admin: store.is_user_admin(channel.id),
            })
            .collect::<Vec<_>>()
    });
    assert_eq!(actual, expected_channels);
}

#[track_caller]
fn assert_channels(
    channel_store: &ModelHandle<ChannelStore>,
    cx: &TestAppContext,
    expected_channels: &[ExpectedChannel],
) {
    let actual = channel_store.read_with(cx, |store, _| {
        store
            .channel_dag_entries()
            .map(|(depth, channel)| ExpectedChannel {
                depth,
                name: channel.name.clone(),
                id: channel.id,
                user_is_admin: store.is_user_admin(channel.id),
            })
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(actual, expected_channels);
}

#[track_caller]
fn assert_channels_list_shape(
    channel_store: &ModelHandle<ChannelStore>,
    cx: &TestAppContext,
    expected_channels: &[(u64, usize)],
) {
    cx.foreground().run_until_parked();

    let actual = channel_store.read_with(cx, |store, _| {
        store
            .channel_dag_entries()
            .map(|(depth, channel)| (channel.id, depth))
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(actual, expected_channels);
}
