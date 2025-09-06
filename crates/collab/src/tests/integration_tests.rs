use crate::{
    rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    tests::{
        RoomParticipants, TestClient, TestServer, channel_id, following_tests::join_channel,
        room_participants, rust_lang,
    },
};
use anyhow::{Result, anyhow};
use assistant_context::ContextStore;
use assistant_slash_command::SlashCommandWorkingSet;
use buffer_diff::{DiffHunkSecondaryStatus, DiffHunkStatus, assert_hunks};
use call::{ActiveCall, ParticipantLocation, Room, room};
use client::{RECEIVE_TIMEOUT, User};
use collections::{HashMap, HashSet};
use fs::{FakeFs, Fs as _, RemoveOptions};
use futures::{StreamExt as _, channel::mpsc};
use git::status::{FileStatus, StatusCode, TrackedStatus, UnmergedStatus, UnmergedStatusCode};
use gpui::{
    App, BackgroundExecutor, Entity, Modifiers, MouseButton, MouseDownEvent, TestAppContext,
    UpdateGlobal, px, size,
};
use language::{
    Diagnostic, DiagnosticEntry, DiagnosticSourceKind, FakeLspAdapter, Language, LanguageConfig,
    LanguageMatcher, LineEnding, OffsetRangeExt, Point, Rope,
    language_settings::{
        AllLanguageSettings, Formatter, FormatterList, PrettierSettings, SelectedFormatter,
    },
    tree_sitter_rust, tree_sitter_typescript,
};
use lsp::{LanguageServerId, OneOf};
use parking_lot::Mutex;
use pretty_assertions::assert_eq;
use project::{
    DiagnosticSummary, HoverBlockKind, Project, ProjectPath,
    lsp_store::{FormatTrigger, LspFormatTarget},
    search::{SearchQuery, SearchResult},
};
use prompt_store::PromptBuilder;
use rand::prelude::*;
use serde_json::json;
use settings::SettingsStore;
use std::{
    cell::{Cell, RefCell},
    env, future, mem,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
    time::Duration,
};
use unindent::Unindent as _;
use util::{path, uri};
use workspace::Pane;

#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}

#[gpui::test(iterations = 10)]
async fn test_database_failure_during_client_reconnection(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client = server.create_client(cx, "user_a").await;

    // Keep disconnecting the client until a database failure prevents it from
    // reconnecting.
    server.test_db.set_query_failure_probability(0.3);
    loop {
        server.disconnect_client(client.peer_id().unwrap());
        executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
        if !client.status().borrow().is_connected() {
            break;
        }
    }

    // Make the database healthy again and ensure the client can finally connect.
    server.test_db.set_query_failure_probability(0.);
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    assert!(
        matches!(*client.status().borrow(), client::Status::Connected { .. }),
        "status was {:?}",
        *client.status().borrow()
    );
}

#[gpui::test(iterations = 10)]
async fn test_basic_calls(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;

    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);

    // Call user B from client A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: vec!["user_b".to_string()]
        }
    );

    // User B receives the call.

    let mut incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
    let call_b = incoming_call_b.next().await.unwrap().unwrap();
    assert_eq!(call_b.calling_user.github_login, "user_a");

    // User B connects via another client and also receives a ring on the newly-connected client.
    let _client_b2 = server.create_client(cx_b2, "user_b").await;
    let active_call_b2 = cx_b2.read(ActiveCall::global);

    let mut incoming_call_b2 = active_call_b2.read_with(cx_b2, |call, _| call.incoming());
    executor.run_until_parked();
    let call_b2 = incoming_call_b2.next().await.unwrap().unwrap();
    assert_eq!(call_b2.calling_user.github_login, "user_a");

    // User B joins the room using the first client.
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    assert!(incoming_call_b.next().await.unwrap().is_none());

    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: Default::default()
        }
    );

    // Call user C from client B.

    let mut incoming_call_c = active_call_c.read_with(cx_c, |call, _| call.incoming());
    active_call_b
        .update(cx_b, |call, cx| {
            call.invite(client_c.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec!["user_c".to_string()]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: vec!["user_c".to_string()]
        }
    );

    // User C receives the call, but declines it.
    let call_c = incoming_call_c.next().await.unwrap().unwrap();
    assert_eq!(call_c.calling_user.github_login, "user_b");
    active_call_c.update(cx_c, |call, cx| call.decline_incoming(cx).unwrap());
    assert!(incoming_call_c.next().await.unwrap().is_none());

    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: Default::default()
        }
    );

    // Call user C again from user A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_c.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec!["user_c".to_string()]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: vec!["user_c".to_string()]
        }
    );

    // User C accepts the call.
    let call_c = incoming_call_c.next().await.unwrap().unwrap();
    assert_eq!(call_c.calling_user.github_login, "user_a");
    active_call_c
        .update(cx_c, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    assert!(incoming_call_c.next().await.unwrap().is_none());

    let room_c = active_call_c.read_with(cx_c, |call, _| call.room().unwrap().clone());

    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string(), "user_c".to_string()],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string(), "user_c".to_string()],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: vec!["user_a".to_string(), "user_b".to_string()],
            pending: Default::default()
        }
    );

    // User A shares their screen
    let display = gpui::TestScreenCaptureSource::new();
    let events_b = active_call_events(cx_b);
    let events_c = active_call_events(cx_c);
    cx_a.set_screen_capture_sources(vec![display]);
    let screen_a = cx_a
        .update(|cx| cx.screen_capture_sources())
        .await
        .unwrap()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    active_call_a
        .update(cx_a, |call, cx| {
            call.room()
                .unwrap()
                .update(cx, |room, cx| room.share_screen(screen_a, cx))
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // User B observes the remote screen sharing track.
    assert_eq!(events_b.borrow().len(), 1);
    let event_b = events_b.borrow().first().unwrap().clone();
    if let call::room::Event::RemoteVideoTracksChanged { participant_id } = event_b {
        assert_eq!(participant_id, client_a.peer_id().unwrap());

        room_b.read_with(cx_b, |room, _| {
            assert_eq!(
                room.remote_participants()[&client_a.user_id().unwrap()]
                    .video_tracks
                    .len(),
                1
            );
        });
    } else {
        panic!("unexpected event")
    }

    // User C observes the remote screen sharing track.
    assert_eq!(events_c.borrow().len(), 1);
    let event_c = events_c.borrow().first().unwrap().clone();
    if let call::room::Event::RemoteVideoTracksChanged { participant_id } = event_c {
        assert_eq!(participant_id, client_a.peer_id().unwrap());

        room_c.read_with(cx_c, |room, _| {
            assert_eq!(
                room.remote_participants()[&client_a.user_id().unwrap()]
                    .video_tracks
                    .len(),
                1
            );
        });
    } else {
        panic!("unexpected event")
    }

    // User A leaves the room.
    active_call_a
        .update(cx_a, |call, cx| {
            let hang_up = call.hang_up(cx);
            assert!(call.room().is_none());
            hang_up
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_c".to_string()],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: Default::default()
        }
    );

    // User B gets disconnected from the LiveKit server, which causes them
    // to automatically leave the room. User C leaves the room as well because
    // nobody else is in there.
    server
        .test_livekit_server
        .disconnect_client(client_b.user_id().unwrap().to_string())
        .await;
    executor.run_until_parked();

    active_call_b.read_with(cx_b, |call, _| assert!(call.room().is_none()));

    active_call_c.read_with(cx_c, |call, _| assert!(call.room().is_none()));
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: Default::default(),
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: Default::default(),
            pending: Default::default()
        }
    );
}

#[gpui::test(iterations = 10)]
async fn test_calling_multiple_users_simultaneously(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_d: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;

    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    let client_d = server.create_client(cx_d, "user_d").await;
    server
        .make_contacts(&mut [
            (&client_a, cx_a),
            (&client_b, cx_b),
            (&client_c, cx_c),
            (&client_d, cx_d),
        ])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);
    let active_call_d = cx_d.read(ActiveCall::global);

    // Simultaneously call user B and user C from client A.
    let b_invite = active_call_a.update(cx_a, |call, cx| {
        call.invite(client_b.user_id().unwrap(), None, cx)
    });
    let c_invite = active_call_a.update(cx_a, |call, cx| {
        call.invite(client_c.user_id().unwrap(), None, cx)
    });
    b_invite.await.unwrap();
    c_invite.await.unwrap();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: vec!["user_b".to_string(), "user_c".to_string()]
        }
    );

    // Call client D from client A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_d.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: vec![
                "user_b".to_string(),
                "user_c".to_string(),
                "user_d".to_string()
            ]
        }
    );

    // Accept the call on all clients simultaneously.
    let accept_b = active_call_b.update(cx_b, |call, cx| call.accept_incoming(cx));
    let accept_c = active_call_c.update(cx_c, |call, cx| call.accept_incoming(cx));
    let accept_d = active_call_d.update(cx_d, |call, cx| call.accept_incoming(cx));
    accept_b.await.unwrap();
    accept_c.await.unwrap();
    accept_d.await.unwrap();

    executor.run_until_parked();

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());

    let room_c = active_call_c.read_with(cx_c, |call, _| call.room().unwrap().clone());

    let room_d = active_call_d.read_with(cx_d, |call, _| call.room().unwrap().clone());
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec![
                "user_b".to_string(),
                "user_c".to_string(),
                "user_d".to_string(),
            ],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec![
                "user_a".to_string(),
                "user_c".to_string(),
                "user_d".to_string(),
            ],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: vec![
                "user_a".to_string(),
                "user_b".to_string(),
                "user_d".to_string(),
            ],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_d, cx_d),
        RoomParticipants {
            remote: vec![
                "user_a".to_string(),
                "user_b".to_string(),
                "user_c".to_string(),
            ],
            pending: Default::default()
        }
    );
}

#[gpui::test(iterations = 10)]
async fn test_joining_channels_and_calling_multiple_users_simultaneously(
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
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    let channel_1 = server
        .make_channel(
            "channel1",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    let channel_2 = server
        .make_channel(
            "channel2",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);

    // Simultaneously join channel 1 and then channel 2
    active_call_a
        .update(cx_a, |call, cx| call.join_channel(channel_1, cx))
        .detach();
    let join_channel_2 = active_call_a.update(cx_a, |call, cx| call.join_channel(channel_2, cx));

    join_channel_2.await.unwrap();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();

    assert_eq!(channel_id(&room_a, cx_a), Some(channel_2));

    // Leave the room
    active_call_a
        .update(cx_a, |call, cx| call.hang_up(cx))
        .await
        .unwrap();

    // Initiating invites and then joining a channel should fail gracefully
    let b_invite = active_call_a.update(cx_a, |call, cx| {
        call.invite(client_b.user_id().unwrap(), None, cx)
    });
    let c_invite = active_call_a.update(cx_a, |call, cx| {
        call.invite(client_c.user_id().unwrap(), None, cx)
    });

    let join_channel = active_call_a.update(cx_a, |call, cx| call.join_channel(channel_1, cx));

    b_invite.await.unwrap();
    c_invite.await.unwrap();
    join_channel.await.unwrap();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();

    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: vec!["user_b".to_string(), "user_c".to_string()]
        }
    );

    assert_eq!(channel_id(&room_a, cx_a), None);

    // Leave the room
    active_call_a
        .update(cx_a, |call, cx| call.hang_up(cx))
        .await
        .unwrap();

    // Simultaneously join channel 1 and call user B and user C from client A.
    let join_channel = active_call_a.update(cx_a, |call, cx| call.join_channel(channel_1, cx));

    let b_invite = active_call_a.update(cx_a, |call, cx| {
        call.invite(client_b.user_id().unwrap(), None, cx)
    });
    let c_invite = active_call_a.update(cx_a, |call, cx| {
        call.invite(client_c.user_id().unwrap(), None, cx)
    });

    join_channel.await.unwrap();
    b_invite.await.unwrap();
    c_invite.await.unwrap();

    active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();
}

#[gpui::test(iterations = 10)]
async fn test_room_uniqueness(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_a2: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let _client_a2 = server.create_client(cx_a2, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let _client_b2 = server.create_client(cx_b2, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_a2 = cx_a2.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_b2 = cx_b2.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);

    // Call user B from client A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();

    // Ensure a new room can't be created given user A just created one.
    active_call_a2
        .update(cx_a2, |call, cx| {
            call.invite(client_c.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap_err();

    active_call_a2.read_with(cx_a2, |call, _| assert!(call.room().is_none()));

    // User B receives the call from user A.

    let mut incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
    let call_b1 = incoming_call_b.next().await.unwrap().unwrap();
    assert_eq!(call_b1.calling_user.github_login, "user_a");

    // Ensure calling users A and B from client C fails.
    active_call_c
        .update(cx_c, |call, cx| {
            call.invite(client_a.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap_err();
    active_call_c
        .update(cx_c, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap_err();

    // Ensure User B can't create a room while they still have an incoming call.
    active_call_b2
        .update(cx_b2, |call, cx| {
            call.invite(client_c.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap_err();

    active_call_b2.read_with(cx_b2, |call, _| assert!(call.room().is_none()));

    // User B joins the room and calling them after they've joined still fails.
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    active_call_c
        .update(cx_c, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap_err();

    // Ensure User B can't create a room while they belong to another room.
    active_call_b2
        .update(cx_b2, |call, cx| {
            call.invite(client_c.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap_err();

    active_call_b2.read_with(cx_b2, |call, _| assert!(call.room().is_none()));

    // Client C can successfully call client B after client B leaves the room.
    active_call_b
        .update(cx_b, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();
    active_call_c
        .update(cx_c, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    let call_b2 = incoming_call_b.next().await.unwrap().unwrap();
    assert_eq!(call_b2.calling_user.github_login, "user_c");
}

#[gpui::test(iterations = 10)]
async fn test_client_disconnecting_from_room(
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

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    // Call user B from client A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());

    // User B receives the call and joins the room.

    let mut incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
    incoming_call_b.next().await.unwrap().unwrap();
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: Default::default()
        }
    );

    // User A automatically reconnects to the room upon disconnection.
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT);
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: Default::default()
        }
    );

    // When user A disconnects, both client A and B clear their room on the active call.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    active_call_a.read_with(cx_a, |call, _| assert!(call.room().is_none()));

    active_call_b.read_with(cx_b, |call, _| assert!(call.room().is_none()));
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: Default::default(),
            pending: Default::default()
        }
    );

    // Allow user A to reconnect to the server.
    server.allow_connections();
    executor.advance_clock(RECONNECT_TIMEOUT);

    // Call user B again from client A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());

    // User B receives the call and joins the room.

    let mut incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
    incoming_call_b.next().await.unwrap().unwrap();
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: Default::default()
        }
    );

    // User B gets disconnected from the LiveKit server, which causes it
    // to automatically leave the room.
    server
        .test_livekit_server
        .disconnect_client(client_b.user_id().unwrap().to_string())
        .await;
    executor.run_until_parked();
    active_call_a.update(cx_a, |call, _| assert!(call.room().is_none()));
    active_call_b.update(cx_b, |call, _| assert!(call.room().is_none()));
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: Default::default()
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: Default::default(),
            pending: Default::default()
        }
    );
}

#[gpui::test(iterations = 10)]
async fn test_server_restarts(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_d: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    client_a
        .fs()
        .insert_tree("/a", json!({ "a.txt": "a-contents" }))
        .await;

    // Invite client B to collaborate on a project
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;

    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    let client_d = server.create_client(cx_d, "user_d").await;
    server
        .make_contacts(&mut [
            (&client_a, cx_a),
            (&client_b, cx_b),
            (&client_c, cx_c),
            (&client_d, cx_d),
        ])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);
    let active_call_d = cx_d.read(ActiveCall::global);

    // User A calls users B, C, and D.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), Some(project_a.clone()), cx)
        })
        .await
        .unwrap();
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_c.user_id().unwrap(), Some(project_a.clone()), cx)
        })
        .await
        .unwrap();
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_d.user_id().unwrap(), Some(project_a.clone()), cx)
        })
        .await
        .unwrap();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());

    // User B receives the call and joins the room.

    let mut incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
    assert!(incoming_call_b.next().await.unwrap().is_some());
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());

    // User C receives the call and joins the room.

    let mut incoming_call_c = active_call_c.read_with(cx_c, |call, _| call.incoming());
    assert!(incoming_call_c.next().await.unwrap().is_some());
    active_call_c
        .update(cx_c, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    let room_c = active_call_c.read_with(cx_c, |call, _| call.room().unwrap().clone());

    // User D receives the call but doesn't join the room yet.

    let mut incoming_call_d = active_call_d.read_with(cx_d, |call, _| call.incoming());
    assert!(incoming_call_d.next().await.unwrap().is_some());

    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string(), "user_c".to_string()],
            pending: vec!["user_d".to_string()]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string(), "user_c".to_string()],
            pending: vec!["user_d".to_string()]
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: vec!["user_a".to_string(), "user_b".to_string()],
            pending: vec!["user_d".to_string()]
        }
    );

    // The server is torn down.
    server.reset().await;

    // Users A and B reconnect to the call. User C has troubles reconnecting, so it leaves the room.
    client_c.override_establish_connection(|_, cx| cx.spawn(async |_| future::pending().await));
    executor.advance_clock(RECONNECT_TIMEOUT);
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string(), "user_c".to_string()],
            pending: vec!["user_d".to_string()]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string(), "user_c".to_string()],
            pending: vec!["user_d".to_string()]
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: vec![],
            pending: vec![]
        }
    );

    // User D is notified again of the incoming call and accepts it.
    assert!(incoming_call_d.next().await.unwrap().is_some());
    active_call_d
        .update(cx_d, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    executor.run_until_parked();

    let room_d = active_call_d.read_with(cx_d, |call, _| call.room().unwrap().clone());
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec![
                "user_b".to_string(),
                "user_c".to_string(),
                "user_d".to_string(),
            ],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec![
                "user_a".to_string(),
                "user_c".to_string(),
                "user_d".to_string(),
            ],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: vec![],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_d, cx_d),
        RoomParticipants {
            remote: vec![
                "user_a".to_string(),
                "user_b".to_string(),
                "user_c".to_string(),
            ],
            pending: vec![]
        }
    );

    // The server finishes restarting, cleaning up stale connections.
    server.start().await.unwrap();
    executor.advance_clock(CLEANUP_TIMEOUT);
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string(), "user_d".to_string()],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string(), "user_d".to_string()],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: vec![],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_d, cx_d),
        RoomParticipants {
            remote: vec!["user_a".to_string(), "user_b".to_string()],
            pending: vec![]
        }
    );

    // User D hangs up.
    active_call_d
        .update(cx_d, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_c, cx_c),
        RoomParticipants {
            remote: vec![],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_d, cx_d),
        RoomParticipants {
            remote: vec![],
            pending: vec![]
        }
    );

    // User B calls user D again.
    active_call_b
        .update(cx_b, |call, cx| {
            call.invite(client_d.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();

    // User D receives the call but doesn't join the room yet.

    let mut incoming_call_d = active_call_d.read_with(cx_d, |call, _| call.incoming());
    assert!(incoming_call_d.next().await.unwrap().is_some());
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec!["user_d".to_string()]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: vec!["user_d".to_string()]
        }
    );

    // The server is torn down.
    server.reset().await;

    // Users A and B have troubles reconnecting, so they leave the room.
    client_a.override_establish_connection(|_, cx| cx.spawn(async |_| future::pending().await));
    client_b.override_establish_connection(|_, cx| cx.spawn(async |_| future::pending().await));
    client_c.override_establish_connection(|_, cx| cx.spawn(async |_| future::pending().await));
    executor.advance_clock(RECONNECT_TIMEOUT);
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec![],
            pending: vec![]
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec![],
            pending: vec![]
        }
    );

    // User D is notified again of the incoming call but doesn't accept it.
    assert!(incoming_call_d.next().await.unwrap().is_some());

    // The server finishes restarting, cleaning up stale connections and canceling the
    // call to user D because the room has become empty.
    server.start().await.unwrap();
    executor.advance_clock(CLEANUP_TIMEOUT);
    assert!(incoming_call_d.next().await.unwrap().is_none());
}

#[gpui::test(iterations = 10)]
async fn test_calls_on_multiple_connections(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b1: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b1 = server.create_client(cx_b1, "user_b").await;
    let client_b2 = server.create_client(cx_b2, "user_b").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b1, cx_b1)])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b1 = cx_b1.read(ActiveCall::global);
    let active_call_b2 = cx_b2.read(ActiveCall::global);

    let mut incoming_call_b1 = active_call_b1.read_with(cx_b1, |call, _| call.incoming());

    let mut incoming_call_b2 = active_call_b2.read_with(cx_b2, |call, _| call.incoming());
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // Call user B from client A, ensuring both clients for user B ring.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User B declines the call on one of the two connections, causing both connections
    // to stop ringing.
    active_call_b2.update(cx_b2, |call, cx| call.decline_incoming(cx).unwrap());
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // Call user B again from client A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User B accepts the call on one of the two connections, causing both connections
    // to stop ringing.
    active_call_b2
        .update(cx_b2, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // User B disconnects the client that is not on the call. Everything should be fine.
    client_b1.disconnect(&cx_b1.to_async());
    executor.advance_clock(RECEIVE_TIMEOUT);
    client_b1
        .connect(false, &cx_b1.to_async())
        .await
        .into_response()
        .unwrap();

    // User B hangs up, and user A calls them again.
    active_call_b2
        .update(cx_b2, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User A cancels the call, causing both connections to stop ringing.
    active_call_a
        .update(cx_a, |call, cx| {
            call.cancel_invite(client_b1.user_id().unwrap(), cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // User A calls user B again.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User A hangs up, causing both connections to stop ringing.
    active_call_a
        .update(cx_a, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // User A calls user B again.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User A disconnects, causing both connections to stop ringing.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // User A reconnects automatically, then calls user B again.
    server.allow_connections();
    executor.advance_clock(RECONNECT_TIMEOUT);
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User B disconnects all clients, causing user A to no longer see a pending call for them.
    server.forbid_connections();
    server.disconnect_client(client_b1.peer_id().unwrap());
    server.disconnect_client(client_b2.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    active_call_a.read_with(cx_a, |call, _| assert!(call.room().is_none()));
}

#[gpui::test(iterations = 10)]
async fn test_unshare_project(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            "/a",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;

    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let worktree_a = project_a.read_with(cx_a, |project, cx| project.worktrees(cx).next().unwrap());
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    executor.run_until_parked();

    assert!(worktree_a.read_with(cx_a, |tree, _| tree.has_update_observer()));

    project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // When client B leaves the room, the project becomes read-only.
    active_call_b
        .update(cx_b, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();

    assert!(project_b.read_with(cx_b, |project, cx| project.is_disconnected(cx)));

    // Client C opens the project.
    let project_c = client_c.join_remote_project(project_id, cx_c).await;

    // When client A unshares the project, client C's project becomes read-only.
    project_a
        .update(cx_a, |project, cx| project.unshare(cx))
        .unwrap();
    executor.run_until_parked();

    assert!(worktree_a.read_with(cx_a, |tree, _| !tree.has_update_observer()));

    assert!(project_c.read_with(cx_c, |project, cx| project.is_disconnected(cx)));

    // Client C can open the project again after client A re-shares.
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_c2 = client_c.join_remote_project(project_id, cx_c).await;
    executor.run_until_parked();

    assert!(worktree_a.read_with(cx_a, |tree, _| tree.has_update_observer()));
    project_c2
        .update(cx_c, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // When client A (the host) leaves the room, the project gets unshared and guests are notified.
    active_call_a
        .update(cx_a, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));

    project_c2.read_with(cx_c, |project, cx| {
        assert!(project.is_disconnected(cx));
        assert!(project.collaborators().is_empty());
    });
}

#[gpui::test(iterations = 10)]
async fn test_project_reconnect(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    cx_b.update(editor::init);

    client_a
        .fs()
        .insert_tree(
            path!("/root-1"),
            json!({
                "dir1": {
                    "a.txt": "a",
                    "b.txt": "b",
                    "subdir1": {
                        "c.txt": "c",
                        "d.txt": "d",
                        "e.txt": "e",
                    }
                },
                "dir2": {
                    "v.txt": "v",
                },
                "dir3": {
                    "w.txt": "w",
                    "x.txt": "x",
                    "y.txt": "y",
                },
                "dir4": {
                    "z.txt": "z",
                },
            }),
        )
        .await;
    client_a
        .fs()
        .insert_tree(
            path!("/root-2"),
            json!({
                "2.txt": "2",
            }),
        )
        .await;
    client_a
        .fs()
        .insert_tree(
            path!("/root-3"),
            json!({
                "3.txt": "3",
            }),
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let (project_a1, _) = client_a
        .build_local_project(path!("/root-1/dir1"), cx_a)
        .await;
    let (project_a2, _) = client_a.build_local_project(path!("/root-2"), cx_a).await;
    let (project_a3, _) = client_a.build_local_project(path!("/root-3"), cx_a).await;
    let worktree_a1 =
        project_a1.read_with(cx_a, |project, cx| project.worktrees(cx).next().unwrap());
    let project1_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a1.clone(), cx))
        .await
        .unwrap();
    let project2_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a2.clone(), cx))
        .await
        .unwrap();
    let project3_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a3.clone(), cx))
        .await
        .unwrap();

    let project_b1 = client_b.join_remote_project(project1_id, cx_b).await;
    let project_b2 = client_b.join_remote_project(project2_id, cx_b).await;
    let project_b3 = client_b.join_remote_project(project3_id, cx_b).await;
    executor.run_until_parked();

    let worktree1_id = worktree_a1.read_with(cx_a, |worktree, _| {
        assert!(worktree.has_update_observer());
        worktree.id()
    });
    let (worktree_a2, _) = project_a1
        .update(cx_a, |p, cx| {
            p.find_or_create_worktree(path!("/root-1/dir2"), true, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();

    let worktree2_id = worktree_a2.read_with(cx_a, |tree, _| {
        assert!(tree.has_update_observer());
        tree.id()
    });
    executor.run_until_parked();

    project_b1.read_with(cx_b, |project, cx| {
        assert!(project.worktree_for_id(worktree2_id, cx).is_some())
    });

    let buffer_a1 = project_a1
        .update(cx_a, |p, cx| p.open_buffer((worktree1_id, "a.txt"), cx))
        .await
        .unwrap();
    let buffer_b1 = project_b1
        .update(cx_b, |p, cx| p.open_buffer((worktree1_id, "a.txt"), cx))
        .await
        .unwrap();

    // Drop client A's connection.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT);

    project_a1.read_with(cx_a, |project, _| {
        assert!(project.is_shared());
        assert_eq!(project.collaborators().len(), 1);
    });

    project_b1.read_with(cx_b, |project, cx| {
        assert!(!project.is_disconnected(cx));
        assert_eq!(project.collaborators().len(), 1);
    });

    worktree_a1.read_with(cx_a, |tree, _| assert!(tree.has_update_observer()));

    // While client A is disconnected, add and remove files from client A's project.
    client_a
        .fs()
        .insert_tree(
            path!("/root-1/dir1/subdir2"),
            json!({
                "f.txt": "f-contents",
                "g.txt": "g-contents",
                "h.txt": "h-contents",
                "i.txt": "i-contents",
            }),
        )
        .await;
    client_a
        .fs()
        .remove_dir(
            path!("/root-1/dir1/subdir1").as_ref(),
            RemoveOptions {
                recursive: true,
                ..Default::default()
            },
        )
        .await
        .unwrap();

    // While client A is disconnected, add and remove worktrees from client A's project.
    project_a1.update(cx_a, |project, cx| {
        project.remove_worktree(worktree2_id, cx)
    });
    let (worktree_a3, _) = project_a1
        .update(cx_a, |p, cx| {
            p.find_or_create_worktree(path!("/root-1/dir3"), true, cx)
        })
        .await
        .unwrap();
    worktree_a3
        .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;

    let worktree3_id = worktree_a3.read_with(cx_a, |tree, _| {
        assert!(!tree.has_update_observer());
        tree.id()
    });
    executor.run_until_parked();

    // While client A is disconnected, close project 2
    cx_a.update(|_| drop(project_a2));

    // While client A is disconnected, mutate a buffer on both the host and the guest.
    buffer_a1.update(cx_a, |buf, cx| buf.edit([(0..0, "W")], None, cx));
    buffer_b1.update(cx_b, |buf, cx| buf.edit([(1..1, "Z")], None, cx));
    executor.run_until_parked();

    // Client A reconnects. Their project is re-shared, and client B re-joins it.
    server.allow_connections();
    client_a
        .connect(false, &cx_a.to_async())
        .await
        .into_response()
        .unwrap();
    executor.run_until_parked();

    project_a1.read_with(cx_a, |project, cx| {
        assert!(project.is_shared());
        assert!(worktree_a1.read(cx).has_update_observer());
        assert_eq!(
            worktree_a1
                .read(cx)
                .snapshot()
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec![
                path!("a.txt"),
                path!("b.txt"),
                path!("subdir2"),
                path!("subdir2/f.txt"),
                path!("subdir2/g.txt"),
                path!("subdir2/h.txt"),
                path!("subdir2/i.txt")
            ]
        );
        assert!(worktree_a3.read(cx).has_update_observer());
        assert_eq!(
            worktree_a3
                .read(cx)
                .snapshot()
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec!["w.txt", "x.txt", "y.txt"]
        );
    });

    project_b1.read_with(cx_b, |project, cx| {
        assert!(!project.is_disconnected(cx));
        assert_eq!(
            project
                .worktree_for_id(worktree1_id, cx)
                .unwrap()
                .read(cx)
                .snapshot()
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec![
                path!("a.txt"),
                path!("b.txt"),
                path!("subdir2"),
                path!("subdir2/f.txt"),
                path!("subdir2/g.txt"),
                path!("subdir2/h.txt"),
                path!("subdir2/i.txt")
            ]
        );
        assert!(project.worktree_for_id(worktree2_id, cx).is_none());
        assert_eq!(
            project
                .worktree_for_id(worktree3_id, cx)
                .unwrap()
                .read(cx)
                .snapshot()
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec!["w.txt", "x.txt", "y.txt"]
        );
    });

    project_b2.read_with(cx_b, |project, cx| assert!(project.is_disconnected(cx)));

    project_b3.read_with(cx_b, |project, cx| assert!(!project.is_disconnected(cx)));

    buffer_a1.read_with(cx_a, |buffer, _| assert_eq!(buffer.text(), "WaZ"));

    buffer_b1.read_with(cx_b, |buffer, _| assert_eq!(buffer.text(), "WaZ"));

    // Drop client B's connection.
    server.forbid_connections();
    server.disconnect_client(client_b.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT);

    // While client B is disconnected, add and remove files from client A's project
    client_a
        .fs()
        .insert_file(path!("/root-1/dir1/subdir2/j.txt"), "j-contents".into())
        .await;
    client_a
        .fs()
        .remove_file(
            path!("/root-1/dir1/subdir2/i.txt").as_ref(),
            Default::default(),
        )
        .await
        .unwrap();

    // While client B is disconnected, add and remove worktrees from client A's project.
    let (worktree_a4, _) = project_a1
        .update(cx_a, |p, cx| {
            p.find_or_create_worktree(path!("/root-1/dir4"), true, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();

    let worktree4_id = worktree_a4.read_with(cx_a, |tree, _| {
        assert!(tree.has_update_observer());
        tree.id()
    });
    project_a1.update(cx_a, |project, cx| {
        project.remove_worktree(worktree3_id, cx)
    });
    executor.run_until_parked();

    // While client B is disconnected, mutate a buffer on both the host and the guest.
    buffer_a1.update(cx_a, |buf, cx| buf.edit([(1..1, "X")], None, cx));
    buffer_b1.update(cx_b, |buf, cx| buf.edit([(2..2, "Y")], None, cx));
    executor.run_until_parked();

    // While disconnected, close project 3
    cx_a.update(|_| drop(project_a3));

    // Client B reconnects. They re-join the room and the remaining shared project.
    server.allow_connections();
    client_b
        .connect(false, &cx_b.to_async())
        .await
        .into_response()
        .unwrap();
    executor.run_until_parked();

    project_b1.read_with(cx_b, |project, cx| {
        assert!(!project.is_disconnected(cx));
        assert_eq!(
            project
                .worktree_for_id(worktree1_id, cx)
                .unwrap()
                .read(cx)
                .snapshot()
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec![
                path!("a.txt"),
                path!("b.txt"),
                path!("subdir2"),
                path!("subdir2/f.txt"),
                path!("subdir2/g.txt"),
                path!("subdir2/h.txt"),
                path!("subdir2/j.txt")
            ]
        );
        assert!(project.worktree_for_id(worktree2_id, cx).is_none());
        assert_eq!(
            project
                .worktree_for_id(worktree4_id, cx)
                .unwrap()
                .read(cx)
                .snapshot()
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec!["z.txt"]
        );
    });

    project_b3.read_with(cx_b, |project, cx| assert!(project.is_disconnected(cx)));

    buffer_a1.read_with(cx_a, |buffer, _| assert_eq!(buffer.text(), "WXaYZ"));

    buffer_b1.read_with(cx_b, |buffer, _| assert_eq!(buffer.text(), "WXaYZ"));
}

#[gpui::test(iterations = 10)]
async fn test_active_call_events(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    client_a.fs().insert_tree("/a", json!({})).await;
    client_b.fs().insert_tree("/b", json!({})).await;

    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    let (project_b, _) = client_b.build_local_project("/b", cx_b).await;

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    executor.run_until_parked();

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let events_a = active_call_events(cx_a);
    let events_b = active_call_events(cx_b);

    let project_a_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(mem::take(&mut *events_a.borrow_mut()), vec![]);
    assert_eq!(
        mem::take(&mut *events_b.borrow_mut()),
        vec![room::Event::RemoteProjectShared {
            owner: Arc::new(User {
                id: client_a.user_id().unwrap(),
                github_login: "user_a".into(),
                avatar_uri: "avatar_a".into(),
                name: None,
            }),
            project_id: project_a_id,
            worktree_root_names: vec!["a".to_string()],
        }]
    );

    let project_b_id = active_call_b
        .update(cx_b, |call, cx| call.share_project(project_b.clone(), cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        mem::take(&mut *events_a.borrow_mut()),
        vec![room::Event::RemoteProjectShared {
            owner: Arc::new(User {
                id: client_b.user_id().unwrap(),
                github_login: "user_b".into(),
                avatar_uri: "avatar_b".into(),
                name: None,
            }),
            project_id: project_b_id,
            worktree_root_names: vec!["b".to_string()]
        }]
    );
    assert_eq!(mem::take(&mut *events_b.borrow_mut()), vec![]);

    // Sharing a project twice is idempotent.
    let project_b_id_2 = active_call_b
        .update(cx_b, |call, cx| call.share_project(project_b.clone(), cx))
        .await
        .unwrap();
    assert_eq!(project_b_id_2, project_b_id);
    executor.run_until_parked();
    assert_eq!(mem::take(&mut *events_a.borrow_mut()), vec![]);
    assert_eq!(mem::take(&mut *events_b.borrow_mut()), vec![]);

    // Unsharing a project should dispatch the RemoteProjectUnshared event.
    active_call_a
        .update(cx_a, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();

    assert_eq!(
        mem::take(&mut *events_a.borrow_mut()),
        vec![room::Event::RoomLeft { channel_id: None }]
    );
    assert_eq!(
        mem::take(&mut *events_b.borrow_mut()),
        vec![room::Event::RemoteProjectUnshared {
            project_id: project_a_id,
        }]
    );
}

fn active_call_events(cx: &mut TestAppContext) -> Rc<RefCell<Vec<room::Event>>> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let active_call = cx.read(ActiveCall::global);
    cx.update({
        let events = events.clone();
        |cx| {
            cx.subscribe(&active_call, move |_, event, _| {
                events.borrow_mut().push(event.clone())
            })
            .detach()
        }
    });
    events
}

#[gpui::test]
async fn test_mute_deafen(
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
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);

    // User A calls user B, B answers.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    executor.run_until_parked();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());

    room_a.read_with(cx_a, |room, _| assert!(!room.is_muted()));
    room_b.read_with(cx_b, |room, _| assert!(!room.is_muted()));

    // Users A and B are both unmuted.
    assert_eq!(
        participant_audio_state(&room_a, cx_a),
        &[ParticipantAudioState {
            user_id: client_b.user_id().unwrap(),
            is_muted: false,
            audio_tracks_playing: vec![true],
        }]
    );
    assert_eq!(
        participant_audio_state(&room_b, cx_b),
        &[ParticipantAudioState {
            user_id: client_a.user_id().unwrap(),
            is_muted: false,
            audio_tracks_playing: vec![true],
        }]
    );

    // User A mutes
    room_a.update(cx_a, |room, cx| room.toggle_mute(cx));
    executor.run_until_parked();

    // User A hears user B, but B doesn't hear A.
    room_a.read_with(cx_a, |room, _| assert!(room.is_muted()));
    room_b.read_with(cx_b, |room, _| assert!(!room.is_muted()));
    assert_eq!(
        participant_audio_state(&room_a, cx_a),
        &[ParticipantAudioState {
            user_id: client_b.user_id().unwrap(),
            is_muted: false,
            audio_tracks_playing: vec![true],
        }]
    );
    assert_eq!(
        participant_audio_state(&room_b, cx_b),
        &[ParticipantAudioState {
            user_id: client_a.user_id().unwrap(),
            is_muted: true,
            audio_tracks_playing: vec![true],
        }]
    );

    // User A deafens
    room_a.update(cx_a, |room, cx| room.toggle_deafen(cx));
    executor.run_until_parked();

    // User A does not hear user B.
    room_a.read_with(cx_a, |room, _| assert!(room.is_muted()));
    room_b.read_with(cx_b, |room, _| assert!(!room.is_muted()));
    assert_eq!(
        participant_audio_state(&room_a, cx_a),
        &[ParticipantAudioState {
            user_id: client_b.user_id().unwrap(),
            is_muted: false,
            audio_tracks_playing: vec![false],
        }]
    );
    assert_eq!(
        participant_audio_state(&room_b, cx_b),
        &[ParticipantAudioState {
            user_id: client_a.user_id().unwrap(),
            is_muted: true,
            audio_tracks_playing: vec![true],
        }]
    );

    // User B calls user C, C joins.
    active_call_b
        .update(cx_b, |call, cx| {
            call.invite(client_c.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    active_call_c
        .update(cx_c, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    executor.run_until_parked();

    // User A does not hear users B or C.
    assert_eq!(
        participant_audio_state(&room_a, cx_a),
        &[
            ParticipantAudioState {
                user_id: client_b.user_id().unwrap(),
                is_muted: false,
                audio_tracks_playing: vec![false],
            },
            ParticipantAudioState {
                user_id: client_c.user_id().unwrap(),
                is_muted: false,
                audio_tracks_playing: vec![false],
            }
        ]
    );
    assert_eq!(
        participant_audio_state(&room_b, cx_b),
        &[
            ParticipantAudioState {
                user_id: client_a.user_id().unwrap(),
                is_muted: true,
                audio_tracks_playing: vec![true],
            },
            ParticipantAudioState {
                user_id: client_c.user_id().unwrap(),
                is_muted: false,
                audio_tracks_playing: vec![true],
            }
        ]
    );

    #[derive(PartialEq, Eq, Debug)]
    struct ParticipantAudioState {
        user_id: u64,
        is_muted: bool,
        audio_tracks_playing: Vec<bool>,
    }

    fn participant_audio_state(
        room: &Entity<Room>,
        cx: &TestAppContext,
    ) -> Vec<ParticipantAudioState> {
        room.read_with(cx, |room, _| {
            room.remote_participants()
                .iter()
                .map(|(user_id, participant)| ParticipantAudioState {
                    user_id: *user_id,
                    is_muted: participant.muted,
                    audio_tracks_playing: participant
                        .audio_tracks
                        .values()
                        .map(|(track, _)| track.enabled())
                        .collect(),
                })
                .collect::<Vec<_>>()
        })
    }
}

#[gpui::test(iterations = 10)]
async fn test_room_location(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    client_a.fs().insert_tree("/a", json!({})).await;
    client_b.fs().insert_tree("/b", json!({})).await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let a_notified = Rc::new(Cell::new(false));
    cx_a.update({
        let notified = a_notified.clone();
        |cx| {
            cx.observe(&active_call_a, move |_, _| notified.set(true))
                .detach()
        }
    });

    let b_notified = Rc::new(Cell::new(false));
    cx_b.update({
        let b_notified = b_notified.clone();
        |cx| {
            cx.observe(&active_call_b, move |_, _| b_notified.set(true))
                .detach()
        }
    });

    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let (project_b, _) = client_b.build_local_project("/b", cx_b).await;

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();
    assert!(a_notified.take());
    assert_eq!(
        participant_locations(&room_a, cx_a),
        vec![("user_b".to_string(), ParticipantLocation::External)]
    );
    assert!(b_notified.take());
    assert_eq!(
        participant_locations(&room_b, cx_b),
        vec![("user_a".to_string(), ParticipantLocation::UnsharedProject)]
    );

    let project_a_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(a_notified.take());
    assert_eq!(
        participant_locations(&room_a, cx_a),
        vec![("user_b".to_string(), ParticipantLocation::External)]
    );
    assert!(b_notified.take());
    assert_eq!(
        participant_locations(&room_b, cx_b),
        vec![(
            "user_a".to_string(),
            ParticipantLocation::SharedProject {
                project_id: project_a_id
            }
        )]
    );

    let project_b_id = active_call_b
        .update(cx_b, |call, cx| call.share_project(project_b.clone(), cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(a_notified.take());
    assert_eq!(
        participant_locations(&room_a, cx_a),
        vec![("user_b".to_string(), ParticipantLocation::External)]
    );
    assert!(b_notified.take());
    assert_eq!(
        participant_locations(&room_b, cx_b),
        vec![(
            "user_a".to_string(),
            ParticipantLocation::SharedProject {
                project_id: project_a_id
            }
        )]
    );

    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(a_notified.take());
    assert_eq!(
        participant_locations(&room_a, cx_a),
        vec![(
            "user_b".to_string(),
            ParticipantLocation::SharedProject {
                project_id: project_b_id
            }
        )]
    );
    assert!(b_notified.take());
    assert_eq!(
        participant_locations(&room_b, cx_b),
        vec![(
            "user_a".to_string(),
            ParticipantLocation::SharedProject {
                project_id: project_a_id
            }
        )]
    );

    active_call_b
        .update(cx_b, |call, cx| call.set_location(None, cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert!(a_notified.take());
    assert_eq!(
        participant_locations(&room_a, cx_a),
        vec![("user_b".to_string(), ParticipantLocation::External)]
    );
    assert!(b_notified.take());
    assert_eq!(
        participant_locations(&room_b, cx_b),
        vec![(
            "user_a".to_string(),
            ParticipantLocation::SharedProject {
                project_id: project_a_id
            }
        )]
    );

    fn participant_locations(
        room: &Entity<Room>,
        cx: &TestAppContext,
    ) -> Vec<(String, ParticipantLocation)> {
        room.read_with(cx, |room, _| {
            room.remote_participants()
                .values()
                .map(|participant| {
                    (
                        participant.user.github_login.to_string(),
                        participant.location,
                    )
                })
                .collect()
        })
    }
}

#[gpui::test(iterations = 10)]
async fn test_propagate_saves_and_fs_changes(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let rust = Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));
    let javascript = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["js".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));
    for client in [&client_a, &client_b, &client_c] {
        client.language_registry().add(rust.clone());
        client.language_registry().add(javascript.clone());
    }

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "file1.rs": "",
                "file2": ""
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;

    let worktree_a = project_a.read_with(cx_a, |p, cx| p.worktrees(cx).next().unwrap());
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Join that worktree as clients B and C.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let project_c = client_c.join_remote_project(project_id, cx_c).await;

    let worktree_b = project_b.read_with(cx_b, |p, cx| p.worktrees(cx).next().unwrap());

    let worktree_c = project_c.read_with(cx_c, |p, cx| p.worktrees(cx).next().unwrap());

    // Open and edit a buffer as both guests B and C.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "file1.rs"), cx))
        .await
        .unwrap();
    let buffer_c = project_c
        .update(cx_c, |p, cx| p.open_buffer((worktree_id, "file1.rs"), cx))
        .await
        .unwrap();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.language().unwrap().name(), "Rust".into());
    });

    buffer_c.read_with(cx_c, |buffer, _| {
        assert_eq!(buffer.language().unwrap().name(), "Rust".into());
    });
    buffer_b.update(cx_b, |buf, cx| buf.edit([(0..0, "i-am-b, ")], None, cx));
    buffer_c.update(cx_c, |buf, cx| buf.edit([(0..0, "i-am-c, ")], None, cx));

    // Open and edit that buffer as the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "file1.rs"), cx))
        .await
        .unwrap();

    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buf, _| assert_eq!(buf.text(), "i-am-c, i-am-b, "));
    buffer_a.update(cx_a, |buf, cx| {
        buf.edit([(buf.len()..buf.len(), "i-am-a")], None, cx)
    });

    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buf, _| {
        assert_eq!(buf.text(), "i-am-c, i-am-b, i-am-a");
    });

    buffer_b.read_with(cx_b, |buf, _| {
        assert_eq!(buf.text(), "i-am-c, i-am-b, i-am-a");
    });

    buffer_c.read_with(cx_c, |buf, _| {
        assert_eq!(buf.text(), "i-am-c, i-am-b, i-am-a");
    });

    // Edit the buffer as the host and concurrently save as guest B.
    let save_b = project_b.update(cx_b, |project, cx| {
        project.save_buffer(buffer_b.clone(), cx)
    });
    buffer_a.update(cx_a, |buf, cx| buf.edit([(0..0, "hi-a, ")], None, cx));
    save_b.await.unwrap();
    assert_eq!(
        client_a.fs().load("/a/file1.rs".as_ref()).await.unwrap(),
        "hi-a, i-am-c, i-am-b, i-am-a"
    );

    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buf, _| assert!(!buf.is_dirty()));

    buffer_b.read_with(cx_b, |buf, _| assert!(!buf.is_dirty()));

    buffer_c.read_with(cx_c, |buf, _| assert!(!buf.is_dirty()));

    // Make changes on host's file system, see those changes on guest worktrees.
    client_a
        .fs()
        .rename(
            path!("/a/file1.rs").as_ref(),
            path!("/a/file1.js").as_ref(),
            Default::default(),
        )
        .await
        .unwrap();
    client_a
        .fs()
        .rename(
            path!("/a/file2").as_ref(),
            path!("/a/file3").as_ref(),
            Default::default(),
        )
        .await
        .unwrap();
    client_a
        .fs()
        .insert_file(path!("/a/file4"), "4".into())
        .await;
    executor.run_until_parked();

    worktree_a.read_with(cx_a, |tree, _| {
        assert_eq!(
            tree.paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["file1.js", "file3", "file4"]
        )
    });

    worktree_b.read_with(cx_b, |tree, _| {
        assert_eq!(
            tree.paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["file1.js", "file3", "file4"]
        )
    });

    worktree_c.read_with(cx_c, |tree, _| {
        assert_eq!(
            tree.paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["file1.js", "file3", "file4"]
        )
    });

    // Ensure buffer files are updated as well.

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.file().unwrap().path().to_str(), Some("file1.js"));
        assert_eq!(buffer.language().unwrap().name(), "JavaScript".into());
    });

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.file().unwrap().path().to_str(), Some("file1.js"));
        assert_eq!(buffer.language().unwrap().name(), "JavaScript".into());
    });

    buffer_c.read_with(cx_c, |buffer, _| {
        assert_eq!(buffer.file().unwrap().path().to_str(), Some("file1.js"));
        assert_eq!(buffer.language().unwrap().name(), "JavaScript".into());
    });

    let new_buffer_a = project_a
        .update(cx_a, |p, cx| p.create_buffer(cx))
        .await
        .unwrap();

    let new_buffer_id = new_buffer_a.read_with(cx_a, |buffer, _| buffer.remote_id());
    let new_buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer_by_id(new_buffer_id, cx))
        .await
        .unwrap();

    new_buffer_b.read_with(cx_b, |buffer, _| {
        assert!(buffer.file().is_none());
    });

    new_buffer_a.update(cx_a, |buffer, cx| {
        buffer.edit([(0..0, "ok")], None, cx);
    });
    project_a
        .update(cx_a, |project, cx| {
            let path = ProjectPath {
                path: Arc::from(Path::new("file3.rs")),
                worktree_id: worktree_a.read(cx).id(),
            };

            project.save_buffer_as(new_buffer_a.clone(), path, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    new_buffer_b.read_with(cx_b, |buffer_b, _| {
        assert_eq!(
            buffer_b.file().unwrap().path().as_ref(),
            Path::new("file3.rs")
        );

        new_buffer_a.read_with(cx_a, |buffer_a, _| {
            assert_eq!(buffer_b.saved_mtime(), buffer_a.saved_mtime());
            assert_eq!(buffer_b.saved_version(), buffer_a.saved_version());
        });
    });
}

#[gpui::test(iterations = 10)]
async fn test_git_diff_base_change(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            "/dir",
            json!({
            ".git": {},
            "sub": {
                ".git": {},
                "b.txt": "
                    one
                    two
                    three
                ".unindent(),
            },
            "a.txt": "
                    one
                    two
                    three
                ".unindent(),
            }),
        )
        .await;

    let (project_local, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| {
            call.share_project(project_local.clone(), cx)
        })
        .await
        .unwrap();

    let project_remote = client_b.join_remote_project(project_id, cx_b).await;

    let staged_text = "
        one
        three
    "
    .unindent();

    let committed_text = "
        one
        TWO
        three
    "
    .unindent();

    let new_committed_text = "
        one
        TWO_HUNDRED
        three
    "
    .unindent();

    let new_staged_text = "
        one
        two
    "
    .unindent();

    client_a.fs().set_index_for_repo(
        Path::new("/dir/.git"),
        &[("a.txt".into(), staged_text.clone())],
    );
    client_a.fs().set_head_for_repo(
        Path::new("/dir/.git"),
        &[("a.txt".into(), committed_text.clone())],
        "deadbeef",
    );

    // Create the buffer
    let buffer_local_a = project_local
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();
    let local_unstaged_diff_a = project_local
        .update(cx_a, |p, cx| {
            p.open_unstaged_diff(buffer_local_a.clone(), cx)
        })
        .await
        .unwrap();

    // Wait for it to catch up to the new diff
    executor.run_until_parked();
    local_unstaged_diff_a.read_with(cx_a, |diff, cx| {
        let buffer = buffer_local_a.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(staged_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &diff.base_text_string().unwrap(),
            &[(1..2, "", "two\n", DiffHunkStatus::added_none())],
        );
    });

    // Create remote buffer
    let remote_buffer_a = project_remote
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();
    let remote_unstaged_diff_a = project_remote
        .update(cx_b, |p, cx| {
            p.open_unstaged_diff(remote_buffer_a.clone(), cx)
        })
        .await
        .unwrap();

    // Wait remote buffer to catch up to the new diff
    executor.run_until_parked();
    remote_unstaged_diff_a.read_with(cx_b, |diff, cx| {
        let buffer = remote_buffer_a.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(staged_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &diff.base_text_string().unwrap(),
            &[(1..2, "", "two\n", DiffHunkStatus::added_none())],
        );
    });

    // Open uncommitted changes on the guest, without opening them on the host first
    let remote_uncommitted_diff_a = project_remote
        .update(cx_b, |p, cx| {
            p.open_uncommitted_diff(remote_buffer_a.clone(), cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    remote_uncommitted_diff_a.read_with(cx_b, |diff, cx| {
        let buffer = remote_buffer_a.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(committed_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &diff.base_text_string().unwrap(),
            &[(
                1..2,
                "TWO\n",
                "two\n",
                DiffHunkStatus::modified(DiffHunkSecondaryStatus::HasSecondaryHunk),
            )],
        );
    });

    // Update the index text of the open buffer
    client_a.fs().set_index_for_repo(
        Path::new("/dir/.git"),
        &[("a.txt".into(), new_staged_text.clone())],
    );
    client_a.fs().set_head_for_repo(
        Path::new("/dir/.git"),
        &[("a.txt".into(), new_committed_text.clone())],
        "deadbeef",
    );

    // Wait for buffer_local_a to receive it
    executor.run_until_parked();
    local_unstaged_diff_a.read_with(cx_a, |diff, cx| {
        let buffer = buffer_local_a.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(new_staged_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &diff.base_text_string().unwrap(),
            &[(2..3, "", "three\n", DiffHunkStatus::added_none())],
        );
    });

    // Guest receives index text update
    remote_unstaged_diff_a.read_with(cx_b, |diff, cx| {
        let buffer = remote_buffer_a.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(new_staged_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &diff.base_text_string().unwrap(),
            &[(2..3, "", "three\n", DiffHunkStatus::added_none())],
        );
    });

    remote_uncommitted_diff_a.read_with(cx_b, |diff, cx| {
        let buffer = remote_buffer_a.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(new_committed_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &diff.base_text_string().unwrap(),
            &[(
                1..2,
                "TWO_HUNDRED\n",
                "two\n",
                DiffHunkStatus::modified(DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk),
            )],
        );
    });

    // Nested git dir
    let staged_text = "
        one
        three
    "
    .unindent();

    let new_staged_text = "
        one
        two
    "
    .unindent();

    client_a.fs().set_index_for_repo(
        Path::new("/dir/sub/.git"),
        &[("b.txt".into(), staged_text.clone())],
    );

    // Create the buffer
    let buffer_local_b = project_local
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "sub/b.txt"), cx))
        .await
        .unwrap();
    let local_unstaged_diff_b = project_local
        .update(cx_a, |p, cx| {
            p.open_unstaged_diff(buffer_local_b.clone(), cx)
        })
        .await
        .unwrap();

    // Wait for it to catch up to the new diff
    executor.run_until_parked();
    local_unstaged_diff_b.read_with(cx_a, |diff, cx| {
        let buffer = buffer_local_b.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(staged_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &diff.base_text_string().unwrap(),
            &[(1..2, "", "two\n", DiffHunkStatus::added_none())],
        );
    });

    // Create remote buffer
    let remote_buffer_b = project_remote
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "sub/b.txt"), cx))
        .await
        .unwrap();
    let remote_unstaged_diff_b = project_remote
        .update(cx_b, |p, cx| {
            p.open_unstaged_diff(remote_buffer_b.clone(), cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();
    remote_unstaged_diff_b.read_with(cx_b, |diff, cx| {
        let buffer = remote_buffer_b.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(staged_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &staged_text,
            &[(1..2, "", "two\n", DiffHunkStatus::added_none())],
        );
    });

    // Updatet the staged text
    client_a.fs().set_index_for_repo(
        Path::new("/dir/sub/.git"),
        &[("b.txt".into(), new_staged_text.clone())],
    );

    // Wait for buffer_local_b to receive it
    executor.run_until_parked();
    local_unstaged_diff_b.read_with(cx_a, |diff, cx| {
        let buffer = buffer_local_b.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(new_staged_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &new_staged_text,
            &[(2..3, "", "three\n", DiffHunkStatus::added_none())],
        );
    });

    remote_unstaged_diff_b.read_with(cx_b, |diff, cx| {
        let buffer = remote_buffer_b.read(cx);
        assert_eq!(
            diff.base_text_string().as_deref(),
            Some(new_staged_text.as_str())
        );
        assert_hunks(
            diff.hunks_in_row_range(0..4, buffer, cx),
            buffer,
            &new_staged_text,
            &[(2..3, "", "three\n", DiffHunkStatus::added_none())],
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_git_branch_name(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            "/dir",
            json!({
            ".git": {},
            }),
        )
        .await;

    let (project_local, _worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| {
            call.share_project(project_local.clone(), cx)
        })
        .await
        .unwrap();

    let project_remote = client_b.join_remote_project(project_id, cx_b).await;
    client_a
        .fs()
        .set_branch_name(Path::new("/dir/.git"), Some("branch-1"));

    // Wait for it to catch up to the new branch
    executor.run_until_parked();

    #[track_caller]
    fn assert_branch(branch_name: Option<impl Into<String>>, project: &Project, cx: &App) {
        let branch_name = branch_name.map(Into::into);
        let repositories = project.repositories(cx).values().collect::<Vec<_>>();
        assert_eq!(repositories.len(), 1);
        let repository = repositories[0].clone();
        assert_eq!(
            repository
                .read(cx)
                .branch
                .as_ref()
                .map(|branch| branch.name().to_owned()),
            branch_name
        )
    }

    // Smoke test branch reading

    project_local.read_with(cx_a, |project, cx| {
        assert_branch(Some("branch-1"), project, cx)
    });

    project_remote.read_with(cx_b, |project, cx| {
        assert_branch(Some("branch-1"), project, cx)
    });

    client_a
        .fs()
        .set_branch_name(Path::new("/dir/.git"), Some("branch-2"));

    // Wait for buffer_local_a to receive it
    executor.run_until_parked();

    // Smoke test branch reading

    project_local.read_with(cx_a, |project, cx| {
        assert_branch(Some("branch-2"), project, cx)
    });

    project_remote.read_with(cx_b, |project, cx| {
        assert_branch(Some("branch-2"), project, cx)
    });

    let project_remote_c = client_c.join_remote_project(project_id, cx_c).await;
    executor.run_until_parked();

    project_remote_c.read_with(cx_c, |project, cx| {
        assert_branch(Some("branch-2"), project, cx)
    });
}

#[gpui::test]
async fn test_git_status_sync(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            path!("/dir"),
            json!({
                ".git": {},
                "a.txt": "a",
                "b.txt": "b",
                "c.txt": "c",
            }),
        )
        .await;

    // Initially, a.txt is uncommitted, but present in the index,
    // and b.txt is unmerged.
    client_a.fs().set_head_for_repo(
        path!("/dir/.git").as_ref(),
        &[("b.txt".into(), "B".into()), ("c.txt".into(), "c".into())],
        "deadbeef",
    );
    client_a.fs().set_index_for_repo(
        path!("/dir/.git").as_ref(),
        &[
            ("a.txt".into(), "".into()),
            ("b.txt".into(), "B".into()),
            ("c.txt".into(), "c".into()),
        ],
    );
    client_a.fs().set_unmerged_paths_for_repo(
        path!("/dir/.git").as_ref(),
        &[(
            "b.txt".into(),
            UnmergedStatus {
                first_head: UnmergedStatusCode::Updated,
                second_head: UnmergedStatusCode::Deleted,
            },
        )],
    );

    const A_STATUS_START: FileStatus = FileStatus::Tracked(TrackedStatus {
        index_status: StatusCode::Added,
        worktree_status: StatusCode::Modified,
    });
    const B_STATUS_START: FileStatus = FileStatus::Unmerged(UnmergedStatus {
        first_head: UnmergedStatusCode::Updated,
        second_head: UnmergedStatusCode::Deleted,
    });

    let (project_local, _worktree_id) = client_a.build_local_project(path!("/dir"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| {
            call.share_project(project_local.clone(), cx)
        })
        .await
        .unwrap();

    let project_remote = client_b.join_remote_project(project_id, cx_b).await;

    // Wait for it to catch up to the new status
    executor.run_until_parked();

    #[track_caller]
    fn assert_status(
        file: impl AsRef<Path>,
        status: Option<FileStatus>,
        project: &Project,
        cx: &App,
    ) {
        let file = file.as_ref();
        let repos = project
            .repositories(cx)
            .values()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(repos.len(), 1);
        let repo = repos.into_iter().next().unwrap();
        assert_eq!(
            repo.read(cx)
                .status_for_path(&file.into())
                .map(|entry| entry.status),
            status
        );
    }

    project_local.read_with(cx_a, |project, cx| {
        assert_status("a.txt", Some(A_STATUS_START), project, cx);
        assert_status("b.txt", Some(B_STATUS_START), project, cx);
        assert_status("c.txt", None, project, cx);
    });

    project_remote.read_with(cx_b, |project, cx| {
        assert_status("a.txt", Some(A_STATUS_START), project, cx);
        assert_status("b.txt", Some(B_STATUS_START), project, cx);
        assert_status("c.txt", None, project, cx);
    });

    const A_STATUS_END: FileStatus = FileStatus::Tracked(TrackedStatus {
        index_status: StatusCode::Added,
        worktree_status: StatusCode::Unmodified,
    });
    const B_STATUS_END: FileStatus = FileStatus::Tracked(TrackedStatus {
        index_status: StatusCode::Deleted,
        worktree_status: StatusCode::Added,
    });
    const C_STATUS_END: FileStatus = FileStatus::Tracked(TrackedStatus {
        index_status: StatusCode::Unmodified,
        worktree_status: StatusCode::Modified,
    });

    // Delete b.txt from the index, mark conflict as resolved,
    // and modify c.txt in the working copy.
    client_a.fs().set_index_for_repo(
        path!("/dir/.git").as_ref(),
        &[("a.txt".into(), "a".into()), ("c.txt".into(), "c".into())],
    );
    client_a
        .fs()
        .set_unmerged_paths_for_repo(path!("/dir/.git").as_ref(), &[]);
    client_a
        .fs()
        .atomic_write(path!("/dir/c.txt").into(), "CC".into())
        .await
        .unwrap();

    // Wait for buffer_local_a to receive it
    executor.run_until_parked();

    // Smoke test status reading
    project_local.read_with(cx_a, |project, cx| {
        assert_status("a.txt", Some(A_STATUS_END), project, cx);
        assert_status("b.txt", Some(B_STATUS_END), project, cx);
        assert_status("c.txt", Some(C_STATUS_END), project, cx);
    });

    project_remote.read_with(cx_b, |project, cx| {
        assert_status("a.txt", Some(A_STATUS_END), project, cx);
        assert_status("b.txt", Some(B_STATUS_END), project, cx);
        assert_status("c.txt", Some(C_STATUS_END), project, cx);
    });

    // And synchronization while joining
    let project_remote_c = client_c.join_remote_project(project_id, cx_c).await;
    executor.run_until_parked();

    project_remote_c.read_with(cx_c, |project, cx| {
        assert_status("a.txt", Some(A_STATUS_END), project, cx);
        assert_status("b.txt", Some(B_STATUS_END), project, cx);
        assert_status("c.txt", Some(C_STATUS_END), project, cx);
    });

    // Now remove the original git repository and check that collaborators are notified.
    client_a
        .fs()
        .remove_dir(path!("/dir/.git").as_ref(), RemoveOptions::default())
        .await
        .unwrap();

    executor.run_until_parked();
    project_remote.update(cx_b, |project, cx| {
        pretty_assertions::assert_eq!(
            project.git_store().read(cx).repo_snapshots(cx),
            HashMap::default()
        );
    });
    project_remote_c.update(cx_c, |project, cx| {
        pretty_assertions::assert_eq!(
            project.git_store().read(cx).repo_snapshots(cx),
            HashMap::default()
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_fs_operations(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            path!("/dir"),
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/dir"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let worktree_a = project_a.read_with(cx_a, |project, cx| project.worktrees(cx).next().unwrap());
    let worktree_b = project_b.read_with(cx_b, |project, cx| project.worktrees(cx).next().unwrap());

    let entry = project_b
        .update(cx_b, |project, cx| {
            project.create_entry((worktree_id, "c.txt"), false, cx)
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();

    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["a.txt", "b.txt", "c.txt"]
        );
    });

    worktree_b.read_with(cx_b, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["a.txt", "b.txt", "c.txt"]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            project.rename_entry(entry.id, Path::new("d.txt"), cx)
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();

    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["a.txt", "b.txt", "d.txt"]
        );
    });

    worktree_b.read_with(cx_b, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["a.txt", "b.txt", "d.txt"]
        );
    });

    let dir_entry = project_b
        .update(cx_b, |project, cx| {
            project.create_entry((worktree_id, "DIR"), true, cx)
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();

    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["DIR", "a.txt", "b.txt", "d.txt"]
        );
    });

    worktree_b.read_with(cx_b, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["DIR", "a.txt", "b.txt", "d.txt"]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            project.create_entry((worktree_id, "DIR/e.txt"), false, cx)
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();

    project_b
        .update(cx_b, |project, cx| {
            project.create_entry((worktree_id, "DIR/SUBDIR"), true, cx)
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();

    project_b
        .update(cx_b, |project, cx| {
            project.create_entry((worktree_id, "DIR/SUBDIR/f.txt"), false, cx)
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();

    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            [
                path!("DIR"),
                path!("DIR/SUBDIR"),
                path!("DIR/SUBDIR/f.txt"),
                path!("DIR/e.txt"),
                path!("a.txt"),
                path!("b.txt"),
                path!("d.txt")
            ]
        );
    });

    worktree_b.read_with(cx_b, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            [
                path!("DIR"),
                path!("DIR/SUBDIR"),
                path!("DIR/SUBDIR/f.txt"),
                path!("DIR/e.txt"),
                path!("a.txt"),
                path!("b.txt"),
                path!("d.txt")
            ]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            project.copy_entry(entry.id, None, Path::new("f.txt"), cx)
        })
        .await
        .unwrap()
        .unwrap();

    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            [
                path!("DIR"),
                path!("DIR/SUBDIR"),
                path!("DIR/SUBDIR/f.txt"),
                path!("DIR/e.txt"),
                path!("a.txt"),
                path!("b.txt"),
                path!("d.txt"),
                path!("f.txt")
            ]
        );
    });

    worktree_b.read_with(cx_b, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            [
                path!("DIR"),
                path!("DIR/SUBDIR"),
                path!("DIR/SUBDIR/f.txt"),
                path!("DIR/e.txt"),
                path!("a.txt"),
                path!("b.txt"),
                path!("d.txt"),
                path!("f.txt")
            ]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            project.delete_entry(dir_entry.id, false, cx).unwrap()
        })
        .await
        .unwrap();
    executor.run_until_parked();

    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["a.txt", "b.txt", "d.txt", "f.txt"]
        );
    });

    worktree_b.read_with(cx_b, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["a.txt", "b.txt", "d.txt", "f.txt"]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            project.delete_entry(entry.id, false, cx).unwrap()
        })
        .await
        .unwrap();

    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["a.txt", "b.txt", "f.txt"]
        );
    });

    worktree_b.read_with(cx_b, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            ["a.txt", "b.txt", "f.txt"]
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_local_settings(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // As client A, open a project that contains some local settings files
    client_a
        .fs()
        .insert_tree(
            "/dir",
            json!({
                ".zed": {
                    "settings.json": r#"{ "tab_size": 2 }"#
                },
                "a": {
                    ".zed": {
                        "settings.json": r#"{ "tab_size": 8 }"#
                    },
                    "a.txt": "a-contents",
                },
                "b": {
                    "b.txt": "b-contents",
                }
            }),
        )
        .await;
    let (project_a, _) = client_a.build_local_project("/dir", cx_a).await;
    executor.run_until_parked();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    executor.run_until_parked();

    // As client B, join that project and observe the local settings.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let worktree_b = project_b.read_with(cx_b, |project, cx| project.worktrees(cx).next().unwrap());
    executor.run_until_parked();
    cx_b.read(|cx| {
        let store = cx.global::<SettingsStore>();
        assert_eq!(
            store
                .local_settings(worktree_b.read(cx).id())
                .collect::<Vec<_>>(),
            &[
                (Path::new("").into(), r#"{"tab_size":2}"#.to_string()),
                (Path::new("a").into(), r#"{"tab_size":8}"#.to_string()),
            ]
        )
    });

    // As client A, update a settings file. As Client B, see the changed settings.
    client_a
        .fs()
        .insert_file("/dir/.zed/settings.json", r#"{}"#.into())
        .await;
    executor.run_until_parked();
    cx_b.read(|cx| {
        let store = cx.global::<SettingsStore>();
        assert_eq!(
            store
                .local_settings(worktree_b.read(cx).id())
                .collect::<Vec<_>>(),
            &[
                (Path::new("").into(), r#"{}"#.to_string()),
                (Path::new("a").into(), r#"{"tab_size":8}"#.to_string()),
            ]
        )
    });

    // As client A, create and remove some settings files. As client B, see the changed settings.
    client_a
        .fs()
        .remove_file("/dir/.zed/settings.json".as_ref(), Default::default())
        .await
        .unwrap();
    client_a
        .fs()
        .create_dir("/dir/b/.zed".as_ref())
        .await
        .unwrap();
    client_a
        .fs()
        .insert_file("/dir/b/.zed/settings.json", r#"{"tab_size": 4}"#.into())
        .await;
    executor.run_until_parked();
    cx_b.read(|cx| {
        let store = cx.global::<SettingsStore>();
        assert_eq!(
            store
                .local_settings(worktree_b.read(cx).id())
                .collect::<Vec<_>>(),
            &[
                (Path::new("a").into(), r#"{"tab_size":8}"#.to_string()),
                (Path::new("b").into(), r#"{"tab_size":4}"#.to_string()),
            ]
        )
    });

    // As client B, disconnect.
    server.forbid_connections();
    server.disconnect_client(client_b.peer_id().unwrap());

    // As client A, change and remove settings files while client B is disconnected.
    client_a
        .fs()
        .insert_file("/dir/a/.zed/settings.json", r#"{"hard_tabs":true}"#.into())
        .await;
    client_a
        .fs()
        .remove_file("/dir/b/.zed/settings.json".as_ref(), Default::default())
        .await
        .unwrap();
    executor.run_until_parked();

    // As client B, reconnect and see the changed settings.
    server.allow_connections();
    executor.advance_clock(RECEIVE_TIMEOUT);
    cx_b.read(|cx| {
        let store = cx.global::<SettingsStore>();
        assert_eq!(
            store
                .local_settings(worktree_b.read(cx).id())
                .collect::<Vec<_>>(),
            &[(Path::new("a").into(), r#"{"hard_tabs":true}"#.to_string()),]
        )
    });
}

#[gpui::test(iterations = 10)]
async fn test_buffer_conflict_after_save(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            path!("/dir"),
            json!({
                "a.txt": "a-contents",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/dir"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open a buffer as client B
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    buffer_b.update(cx_b, |buf, cx| buf.edit([(0..0, "world ")], None, cx));

    buffer_b.read_with(cx_b, |buf, _| {
        assert!(buf.is_dirty());
        assert!(!buf.has_conflict());
    });

    project_b
        .update(cx_b, |project, cx| {
            project.save_buffer(buffer_b.clone(), cx)
        })
        .await
        .unwrap();

    buffer_b.read_with(cx_b, |buffer_b, _| assert!(!buffer_b.is_dirty()));

    buffer_b.read_with(cx_b, |buf, _| {
        assert!(!buf.has_conflict());
    });

    buffer_b.update(cx_b, |buf, cx| buf.edit([(0..0, "hello ")], None, cx));

    buffer_b.read_with(cx_b, |buf, _| {
        assert!(buf.is_dirty());
        assert!(!buf.has_conflict());
    });
}

#[gpui::test(iterations = 10)]
async fn test_buffer_reloading(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            path!("/dir"),
            json!({
                "a.txt": "a\nb\nc",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/dir"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open a buffer as client B
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    buffer_b.read_with(cx_b, |buf, _| {
        assert!(!buf.is_dirty());
        assert!(!buf.has_conflict());
        assert_eq!(buf.line_ending(), LineEnding::Unix);
    });

    let new_contents = Rope::from("d\ne\nf");
    client_a
        .fs()
        .save(
            path!("/dir/a.txt").as_ref(),
            &new_contents,
            LineEnding::Windows,
        )
        .await
        .unwrap();

    executor.run_until_parked();

    buffer_b.read_with(cx_b, |buf, _| {
        assert_eq!(buf.text(), new_contents.to_string());
        assert!(!buf.is_dirty());
        assert!(!buf.has_conflict());
        assert_eq!(buf.line_ending(), LineEnding::Windows);
    });
}

#[gpui::test(iterations = 10)]
async fn test_editing_while_guest_opens_buffer(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(path!("/dir"), json!({ "a.txt": "a-contents" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/dir"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open a buffer as client A
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // Start opening the same buffer as client B
    let open_buffer = project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx));
    let buffer_b = cx_b.executor().spawn(open_buffer);

    // Edit the buffer as client A while client B is still opening it.
    cx_b.executor().simulate_random_delay().await;
    buffer_a.update(cx_a, |buf, cx| buf.edit([(0..0, "X")], None, cx));
    cx_b.executor().simulate_random_delay().await;
    buffer_a.update(cx_a, |buf, cx| buf.edit([(1..1, "Y")], None, cx));

    let text = buffer_a.read_with(cx_a, |buf, _| buf.text());
    let buffer_b = buffer_b.await.unwrap();
    executor.run_until_parked();

    buffer_b.read_with(cx_b, |buf, _| assert_eq!(buf.text(), text));
}

#[gpui::test(iterations = 10)]
async fn test_leaving_worktree_while_opening_buffer(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree("/dir", json!({ "a.txt": "a-contents" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // See that a guest has joined as client A.
    executor.run_until_parked();

    project_a.read_with(cx_a, |p, _| assert_eq!(p.collaborators().len(), 1));

    // Begin opening a buffer as client B, but leave the project before the open completes.
    let open_buffer = project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx));
    let buffer_b = cx_b.executor().spawn(open_buffer);
    cx_b.update(|_| drop(project_b));
    drop(buffer_b);

    // See that the guest has left.
    executor.run_until_parked();

    project_a.read_with(cx_a, |p, _| assert!(p.collaborators().is_empty()));
}

#[gpui::test(iterations = 10)]
async fn test_canceling_buffer_opening(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            "/dir",
            json!({
                "a.txt": "abc",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // Open a buffer as client B but cancel after a random amount of time.
    let buffer_b = project_b.update(cx_b, |p, cx| {
        p.open_buffer_by_id(buffer_a.read_with(cx_a, |a, _| a.remote_id()), cx)
    });
    executor.simulate_random_delay().await;
    drop(buffer_b);

    // Try opening the same buffer again as client B, and ensure we can
    // still do it despite the cancellation above.
    let buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_by_id(buffer_a.read_with(cx_a, |a, _| a.remote_id()), cx)
        })
        .await
        .unwrap();

    buffer_b.read_with(cx_b, |buf, _| assert_eq!(buf.text(), "abc"));
}

#[gpui::test(iterations = 10)]
async fn test_leaving_project(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            "/a",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b1 = client_b.join_remote_project(project_id, cx_b).await;
    let project_c = client_c.join_remote_project(project_id, cx_c).await;

    // Client A sees that a guest has joined.
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });

    project_b1.read_with(cx_b, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });

    project_c.read_with(cx_c, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });

    // Client B opens a buffer.
    let buffer_b1 = project_b1
        .update(cx_b, |project, cx| {
            let worktree_id = project.worktrees(cx).next().unwrap().read(cx).id();
            project.open_buffer((worktree_id, "a.txt"), cx)
        })
        .await
        .unwrap();

    buffer_b1.read_with(cx_b, |buffer, _| assert_eq!(buffer.text(), "a-contents"));

    // Drop client B's project and ensure client A and client C observe client B leaving.
    cx_b.update(|_| drop(project_b1));
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });

    project_c.read_with(cx_c, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });

    // Client B re-joins the project and can open buffers as before.
    let project_b2 = client_b.join_remote_project(project_id, cx_b).await;
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });

    project_b2.read_with(cx_b, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });

    project_c.read_with(cx_c, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });

    let buffer_b2 = project_b2
        .update(cx_b, |project, cx| {
            let worktree_id = project.worktrees(cx).next().unwrap().read(cx).id();
            project.open_buffer((worktree_id, "a.txt"), cx)
        })
        .await
        .unwrap();

    buffer_b2.read_with(cx_b, |buffer, _| assert_eq!(buffer.text(), "a-contents"));

    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });

    // Drop client B's connection and ensure client A and client C observe client B leaving.
    client_b.disconnect(&cx_b.to_async());
    executor.advance_clock(RECONNECT_TIMEOUT);

    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });

    project_b2.read_with(cx_b, |project, cx| {
        assert!(project.is_disconnected(cx));
    });

    project_c.read_with(cx_c, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });

    // Client B can't join the project, unless they re-join the room.
    cx_b.spawn(|cx| {
        Project::in_room(
            project_id,
            client_b.app_state.client.clone(),
            client_b.user_store().clone(),
            client_b.language_registry().clone(),
            FakeFs::new(cx.background_executor().clone()),
            cx,
        )
    })
    .await
    .unwrap_err();

    // Simulate connection loss for client C and ensure client A observes client C leaving the project.
    client_c.wait_for_current_user(cx_c).await;
    server.forbid_connections();
    server.disconnect_client(client_c.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 0);
    });

    project_b2.read_with(cx_b, |project, cx| {
        assert!(project.is_disconnected(cx));
    });

    project_c.read_with(cx_c, |project, cx| {
        assert!(project.is_disconnected(cx));
    });
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_diagnostics(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a.language_registry().add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));
    let mut fake_language_servers = client_a
        .language_registry()
        .register_fake_lsp("Rust", Default::default());

    // Share a project as client A
    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "a.rs": "let one = two",
                "other.rs": "",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;

    // Cause the language server to start.
    let _buffer = project_a
        .update(cx_a, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/a/other.rs"), cx)
        })
        .await
        .unwrap();

    // Simulate a language server reporting errors for a file.
    let mut fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;
    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        &lsp::PublishDiagnosticsParams {
            uri: lsp::Uri::from_file_path(path!("/a/a.rs")).unwrap(),
            version: None,
            diagnostics: vec![lsp::Diagnostic {
                severity: Some(lsp::DiagnosticSeverity::WARNING),
                range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 7)),
                message: "message 0".to_string(),
                ..Default::default()
            }],
        },
    );

    // Client A shares the project and, simultaneously, the language server
    // publishes a diagnostic. This is done to ensure that the server always
    // observes the latest diagnostics for a worktree.
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        &lsp::PublishDiagnosticsParams {
            uri: lsp::Uri::from_file_path(path!("/a/a.rs")).unwrap(),
            version: None,
            diagnostics: vec![lsp::Diagnostic {
                severity: Some(lsp::DiagnosticSeverity::ERROR),
                range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 7)),
                message: "message 1".to_string(),
                ..Default::default()
            }],
        },
    );

    // Join the worktree as client B.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Wait for server to see the diagnostics update.
    executor.run_until_parked();

    // Ensure client B observes the new diagnostics.

    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(false, cx).collect::<Vec<_>>(),
            &[(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("a.rs")),
                },
                LanguageServerId(0),
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 0,
                },
            )]
        )
    });

    // Join project as client C and observe the diagnostics.
    let project_c = client_c.join_remote_project(project_id, cx_c).await;
    executor.run_until_parked();
    let project_c_diagnostic_summaries =
        Rc::new(RefCell::new(project_c.read_with(cx_c, |project, cx| {
            project.diagnostic_summaries(false, cx).collect::<Vec<_>>()
        })));
    project_c.update(cx_c, |_, cx| {
        let summaries = project_c_diagnostic_summaries.clone();
        cx.subscribe(&project_c, {
            move |p, _, event, cx| {
                if let project::Event::DiskBasedDiagnosticsFinished { .. } = event {
                    *summaries.borrow_mut() = p.diagnostic_summaries(false, cx).collect();
                }
            }
        })
        .detach();
    });

    executor.run_until_parked();
    assert_eq!(
        project_c_diagnostic_summaries.borrow().as_slice(),
        &[(
            ProjectPath {
                worktree_id,
                path: Arc::from(Path::new("a.rs")),
            },
            LanguageServerId(0),
            DiagnosticSummary {
                error_count: 1,
                warning_count: 0,
            },
        )]
    );

    // Simulate a language server reporting more errors for a file.
    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        &lsp::PublishDiagnosticsParams {
            uri: lsp::Uri::from_file_path(path!("/a/a.rs")).unwrap(),
            version: None,
            diagnostics: vec![
                lsp::Diagnostic {
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 7)),
                    message: "message 1".to_string(),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    severity: Some(lsp::DiagnosticSeverity::WARNING),
                    range: lsp::Range::new(lsp::Position::new(0, 10), lsp::Position::new(0, 13)),
                    message: "message 2".to_string(),
                    ..Default::default()
                },
            ],
        },
    );

    // Clients B and C get the updated summaries
    executor.run_until_parked();

    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(false, cx).collect::<Vec<_>>(),
            [(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("a.rs")),
                },
                LanguageServerId(0),
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 1,
                },
            )]
        );
    });

    project_c.read_with(cx_c, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(false, cx).collect::<Vec<_>>(),
            [(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("a.rs")),
                },
                LanguageServerId(0),
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 1,
                },
            )]
        );
    });

    // Open the file with the errors on client B. They should be present.
    let open_buffer = project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx));
    let buffer_b = cx_b.executor().spawn(open_buffer).await.unwrap();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(0, 4)..Point::new(0, 7),
                    diagnostic: Diagnostic {
                        group_id: 2,
                        message: "message 1".to_string(),
                        severity: lsp::DiagnosticSeverity::ERROR,
                        is_primary: true,
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(0, 10)..Point::new(0, 13),
                    diagnostic: Diagnostic {
                        group_id: 3,
                        severity: lsp::DiagnosticSeverity::WARNING,
                        message: "message 2".to_string(),
                        is_primary: true,
                        source_kind: DiagnosticSourceKind::Pushed,
                        ..Diagnostic::default()
                    }
                }
            ]
        );
    });

    // Simulate a language server reporting no errors for a file.
    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        &lsp::PublishDiagnosticsParams {
            uri: lsp::Uri::from_file_path(path!("/a/a.rs")).unwrap(),
            version: None,
            diagnostics: Vec::new(),
        },
    );
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(false, cx).collect::<Vec<_>>(),
            []
        )
    });

    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(false, cx).collect::<Vec<_>>(),
            []
        )
    });

    project_c.read_with(cx_c, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(false, cx).collect::<Vec<_>>(),
            []
        )
    });
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_lsp_progress_updates_and_diagnostics_ordering(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            disk_based_diagnostics_progress_token: Some("the-disk-based-token".into()),
            disk_based_diagnostics_sources: vec!["the-disk-based-diagnostics-source".into()],
            ..Default::default()
        },
    );

    let file_names = &["one.rs", "two.rs", "three.rs", "four.rs", "five.rs"];
    client_a
        .fs()
        .insert_tree(
            path!("/test"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = 2;",
                "three.rs": "const THREE: usize = 3;",
                "four.rs": "const FOUR: usize = 3;",
                "five.rs": "const FIVE: usize = 3;",
            }),
        )
        .await;

    let (project_a, worktree_id) = client_a.build_local_project(path!("/test"), cx_a).await;

    // Share a project as client A
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Join the project as client B and open all three files.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let guest_buffers = futures::future::try_join_all(file_names.iter().map(|file_name| {
        project_b.update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, file_name), cx)
        })
    }))
    .await
    .unwrap();

    // Simulate a language server reporting errors for a file.
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server
        .request::<lsp::request::WorkDoneProgressCreate>(lsp::WorkDoneProgressCreateParams {
            token: lsp::NumberOrString::String("the-disk-based-token".to_string()),
        })
        .await
        .into_response()
        .unwrap();
    fake_language_server.notify::<lsp::notification::Progress>(&lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-disk-based-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Begin(
            lsp::WorkDoneProgressBegin {
                title: "Progress Began".into(),
                ..Default::default()
            },
        )),
    });
    for file_name in file_names {
        fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
            &lsp::PublishDiagnosticsParams {
                uri: lsp::Uri::from_file_path(Path::new(path!("/test")).join(file_name)).unwrap(),
                version: None,
                diagnostics: vec![lsp::Diagnostic {
                    severity: Some(lsp::DiagnosticSeverity::WARNING),
                    source: Some("the-disk-based-diagnostics-source".into()),
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                    message: "message one".to_string(),
                    ..Default::default()
                }],
            },
        );
    }
    fake_language_server.notify::<lsp::notification::Progress>(&lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-disk-based-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::End(
            lsp::WorkDoneProgressEnd { message: None },
        )),
    });

    // When the "disk base diagnostics finished" message is received, the buffers'
    // diagnostics are expected to be present.
    let disk_based_diagnostics_finished = Arc::new(AtomicBool::new(false));
    project_b.update(cx_b, {
        let project_b = project_b.clone();
        let disk_based_diagnostics_finished = disk_based_diagnostics_finished.clone();
        move |_, cx| {
            cx.subscribe(&project_b, move |_, _, event, cx| {
                if let project::Event::DiskBasedDiagnosticsFinished { .. } = event {
                    disk_based_diagnostics_finished.store(true, SeqCst);
                    for (buffer, _) in &guest_buffers {
                        assert_eq!(
                            buffer
                                .read(cx)
                                .snapshot()
                                .diagnostics_in_range::<_, usize>(0..5, false)
                                .count(),
                            1,
                            "expected a diagnostic for buffer {:?}",
                            buffer.read(cx).file().unwrap().path(),
                        );
                    }
                }
            })
            .detach();
        }
    });

    executor.run_until_parked();
    assert!(disk_based_diagnostics_finished.load(SeqCst));
}

#[gpui::test(iterations = 10)]
async fn test_reloading_buffer_manually(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(path!("/a"), json!({ "a.rs": "let one = 1;" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let open_buffer = project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx));
    let buffer_b = cx_b.executor().spawn(open_buffer).await.unwrap();
    buffer_b.update(cx_b, |buffer, cx| {
        buffer.edit([(4..7, "six")], None, cx);
        buffer.edit([(10..11, "6")], None, cx);
        assert_eq!(buffer.text(), "let six = 6;");
        assert!(buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| assert_eq!(buffer.text(), "let six = 6;"));

    client_a
        .fs()
        .save(
            path!("/a/a.rs").as_ref(),
            &Rope::from("let seven = 7;"),
            LineEnding::Unix,
        )
        .await
        .unwrap();
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| assert!(buffer.has_conflict()));

    buffer_b.read_with(cx_b, |buffer, _| assert!(buffer.has_conflict()));

    project_b
        .update(cx_b, |project, cx| {
            project.reload_buffers(HashSet::from_iter([buffer_b.clone()]), true, cx)
        })
        .await
        .unwrap();

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "let seven = 7;");
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "let seven = 7;");
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });

    buffer_a.update(cx_a, |buffer, cx| {
        // Undoing on the host is a no-op when the reload was initiated by the guest.
        buffer.undo(cx);
        assert_eq!(buffer.text(), "let seven = 7;");
        assert!(!buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });
    buffer_b.update(cx_b, |buffer, cx| {
        // Undoing on the guest rolls back the buffer to before it was reloaded but the conflict gets cleared.
        buffer.undo(cx);
        assert_eq!(buffer.text(), "let six = 6;");
        assert!(buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });
}

#[gpui::test(iterations = 10)]
async fn test_formatting_buffer(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a
        .language_registry()
        .register_fake_lsp("Rust", FakeLspAdapter::default());

    // Here we insert a fake tree with a directory that exists on disk. This is needed
    // because later we'll invoke a command, which requires passing a working directory
    // that points to a valid location on disk.
    let directory = env::current_dir().unwrap();
    client_a
        .fs()
        .insert_tree(&directory, json!({ "a.rs": "let one = \"two\"" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(&directory, cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx))
        .await
        .unwrap();

    let _handle = project_b.update(cx_b, |project, cx| {
        project.register_buffer_with_language_servers(&buffer_b, cx)
    });
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.set_request_handler::<lsp::request::Formatting, _, _>(|_, _| async move {
        Ok(Some(vec![
            lsp::TextEdit {
                range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 4)),
                new_text: "h".to_string(),
            },
            lsp::TextEdit {
                range: lsp::Range::new(lsp::Position::new(0, 7), lsp::Position::new(0, 7)),
                new_text: "y".to_string(),
            },
        ]))
    });

    project_b
        .update(cx_b, |project, cx| {
            project.format(
                HashSet::from_iter([buffer_b.clone()]),
                LspFormatTarget::Buffers,
                true,
                FormatTrigger::Save,
                cx,
            )
        })
        .await
        .unwrap();

    // The edits from the LSP are applied, and a final newline is added.
    assert_eq!(
        buffer_b.read_with(cx_b, |buffer, _| buffer.text()),
        "let honey = \"two\"\n"
    );

    // There is no `awk` command on Windows.
    #[cfg(not(target_os = "windows"))]
    {
        // Ensure buffer can be formatted using an external command. Notice how the
        // host's configuration is honored as opposed to using the guest's settings.
        cx_a.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, |file| {
                    file.defaults.formatter = Some(SelectedFormatter::List(FormatterList::Single(
                        Formatter::External {
                            command: "awk".into(),
                            arguments: Some(
                                vec!["{sub(/two/,\"{buffer_path}\")}1".to_string()].into(),
                            ),
                        },
                    )));
                });
            });
        });

        executor.allow_parking();
        project_b
            .update(cx_b, |project, cx| {
                project.format(
                    HashSet::from_iter([buffer_b.clone()]),
                    LspFormatTarget::Buffers,
                    true,
                    FormatTrigger::Save,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(
            buffer_b.read_with(cx_b, |buffer, _| buffer.text()),
            format!("let honey = \"{}/a.rs\"\n", directory.to_str().unwrap())
        );
    }
}

#[gpui::test(iterations = 10)]
async fn test_prettier_formatting_buffer(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let test_plugin = "test_plugin";

    client_a.language_registry().add(Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
    )));
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            prettier_plugins: vec![test_plugin],
            ..Default::default()
        },
    );

    // Here we insert a fake tree with a directory that exists on disk. This is needed
    // because later we'll invoke a command, which requires passing a working directory
    // that points to a valid location on disk.
    let directory = env::current_dir().unwrap();
    let buffer_text = "let one = \"two\"";
    client_a
        .fs()
        .insert_tree(&directory, json!({ "a.ts": buffer_text }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(&directory, cx_a).await;
    let prettier_format_suffix = project::TEST_PRETTIER_FORMAT_SUFFIX;
    let open_buffer = project_a.update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.ts"), cx));
    let buffer_a = cx_a.executor().spawn(open_buffer).await.unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let (buffer_b, _) = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, "a.ts"), cx)
        })
        .await
        .unwrap();

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |file| {
                file.defaults.formatter = Some(SelectedFormatter::Auto);
                file.defaults.prettier = Some(PrettierSettings {
                    allowed: true,
                    ..PrettierSettings::default()
                });
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |file| {
                file.defaults.formatter = Some(SelectedFormatter::List(FormatterList::Single(
                    Formatter::LanguageServer { name: None },
                )));
                file.defaults.prettier = Some(PrettierSettings {
                    allowed: true,
                    ..PrettierSettings::default()
                });
            });
        });
    });
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.set_request_handler::<lsp::request::Formatting, _, _>(|_, _| async move {
        panic!(
            "Unexpected: prettier should be preferred since it's enabled and language supports it"
        )
    });

    project_b
        .update(cx_b, |project, cx| {
            project.format(
                HashSet::from_iter([buffer_b.clone()]),
                LspFormatTarget::Buffers,
                true,
                FormatTrigger::Save,
                cx,
            )
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_eq!(
        buffer_b.read_with(cx_b, |buffer, _| buffer.text()),
        buffer_text.to_string() + "\n" + prettier_format_suffix,
        "Prettier formatting was not applied to client buffer after client's request"
    );

    project_a
        .update(cx_a, |project, cx| {
            project.format(
                HashSet::from_iter([buffer_a.clone()]),
                LspFormatTarget::Buffers,
                true,
                FormatTrigger::Manual,
                cx,
            )
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_eq!(
        buffer_b.read_with(cx_b, |buffer, _| buffer.text()),
        buffer_text.to_string() + "\n" + prettier_format_suffix + "\n" + prettier_format_suffix,
        "Prettier formatting was not applied to client buffer after host's request"
    );
}

#[gpui::test(iterations = 10)]
async fn test_definition(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let capabilities = lsp::ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        type_definition_provider: Some(lsp::TypeDefinitionProviderCapability::Simple(true)),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/root"),
            json!({
                "dir-1": {
                    "a.rs": "const ONE: usize = b::TWO + b::THREE;",
                },
                "dir-2": {
                    "b.rs": "const TWO: c::T2 = 2;\nconst THREE: usize = 3;",
                    "c.rs": "type T2 = usize;",
                }
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a
        .build_local_project(path!("/root/dir-1"), cx_a)
        .await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open the file on client B.
    let (buffer_b, _handle) = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, "a.rs"), cx)
        })
        .await
        .unwrap();

    // Request the definition of a symbol as the guest.
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.set_request_handler::<lsp::request::GotoDefinition, _, _>(
        |_, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                lsp::Location::new(
                    lsp::Uri::from_file_path(path!("/root/dir-2/b.rs")).unwrap(),
                    lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                ),
            )))
        },
    );
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let definitions_1 = project_b
        .update(cx_b, |p, cx| p.definitions(&buffer_b, 23, cx))
        .await
        .unwrap()
        .unwrap();
    cx_b.read(|cx| {
        assert_eq!(
            definitions_1.len(),
            1,
            "Unexpected definitions: {definitions_1:?}"
        );
        assert_eq!(project_b.read(cx).worktrees(cx).count(), 2);
        let target_buffer = definitions_1[0].target.buffer.read(cx);
        assert_eq!(
            target_buffer.text(),
            "const TWO: c::T2 = 2;\nconst THREE: usize = 3;"
        );
        assert_eq!(
            definitions_1[0].target.range.to_point(target_buffer),
            Point::new(0, 6)..Point::new(0, 9)
        );
    });

    // Try getting more definitions for the same buffer, ensuring the buffer gets reused from
    // the previous call to `definition`.
    fake_language_server.set_request_handler::<lsp::request::GotoDefinition, _, _>(
        |_, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                lsp::Location::new(
                    lsp::Uri::from_file_path(path!("/root/dir-2/b.rs")).unwrap(),
                    lsp::Range::new(lsp::Position::new(1, 6), lsp::Position::new(1, 11)),
                ),
            )))
        },
    );

    let definitions_2 = project_b
        .update(cx_b, |p, cx| p.definitions(&buffer_b, 33, cx))
        .await
        .unwrap()
        .unwrap();
    cx_b.read(|cx| {
        assert_eq!(definitions_2.len(), 1);
        assert_eq!(project_b.read(cx).worktrees(cx).count(), 2);
        let target_buffer = definitions_2[0].target.buffer.read(cx);
        assert_eq!(
            target_buffer.text(),
            "const TWO: c::T2 = 2;\nconst THREE: usize = 3;"
        );
        assert_eq!(
            definitions_2[0].target.range.to_point(target_buffer),
            Point::new(1, 6)..Point::new(1, 11)
        );
    });
    assert_eq!(
        definitions_1[0].target.buffer,
        definitions_2[0].target.buffer
    );

    fake_language_server.set_request_handler::<lsp::request::GotoTypeDefinition, _, _>(
        |req, _| async move {
            assert_eq!(
                req.text_document_position_params.position,
                lsp::Position::new(0, 7)
            );
            Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                lsp::Location::new(
                    lsp::Uri::from_file_path(path!("/root/dir-2/c.rs")).unwrap(),
                    lsp::Range::new(lsp::Position::new(0, 5), lsp::Position::new(0, 7)),
                ),
            )))
        },
    );

    let type_definitions = project_b
        .update(cx_b, |p, cx| p.type_definitions(&buffer_b, 7, cx))
        .await
        .unwrap()
        .unwrap();
    cx_b.read(|cx| {
        assert_eq!(
            type_definitions.len(),
            1,
            "Unexpected type definitions: {type_definitions:?}"
        );
        let target_buffer = type_definitions[0].target.buffer.read(cx);
        assert_eq!(target_buffer.text(), "type T2 = usize;");
        assert_eq!(
            type_definitions[0].target.range.to_point(target_buffer),
            Point::new(0, 5)..Point::new(0, 7)
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_references(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let capabilities = lsp::ServerCapabilities {
        references_provider: Some(lsp::OneOf::Left(true)),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "my-fake-lsp-adapter",
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "my-fake-lsp-adapter",
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/root"),
            json!({
                "dir-1": {
                    "one.rs": "const ONE: usize = 1;",
                    "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                },
                "dir-2": {
                    "three.rs": "const THREE: usize = two::TWO + one::ONE;",
                }
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a
        .build_local_project(path!("/root/dir-1"), cx_a)
        .await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open the file on client B.
    let (buffer_b, _handle) = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, "one.rs"), cx)
        })
        .await
        .unwrap();

    // Request references to a symbol as the guest.
    let fake_language_server = fake_language_servers.next().await.unwrap();
    let (lsp_response_tx, rx) = mpsc::unbounded::<Result<Option<Vec<lsp::Location>>>>();
    fake_language_server.set_request_handler::<lsp::request::References, _, _>({
        let rx = Arc::new(Mutex::new(Some(rx)));
        move |params, _| {
            assert_eq!(
                params.text_document_position.text_document.uri.as_str(),
                uri!("file:///root/dir-1/one.rs")
            );
            let rx = rx.clone();
            async move {
                let mut response_rx = rx.lock().take().unwrap();
                let result = response_rx.next().await.unwrap();
                *rx.lock() = Some(response_rx);
                result
            }
        }
    });
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let references = project_b.update(cx_b, |p, cx| p.references(&buffer_b, 7, cx));

    // User is informed that a request is pending.
    executor.run_until_parked();
    project_b.read_with(cx_b, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert_eq!(status.name.0, "my-fake-lsp-adapter");
        assert_eq!(
            status.pending_work.values().next().unwrap().message,
            Some("Finding references...".into())
        );
    });

    // Cause the language server to respond.
    lsp_response_tx
        .unbounded_send(Ok(Some(vec![
            lsp::Location {
                uri: lsp::Uri::from_file_path(path!("/root/dir-1/two.rs")).unwrap(),
                range: lsp::Range::new(lsp::Position::new(0, 24), lsp::Position::new(0, 27)),
            },
            lsp::Location {
                uri: lsp::Uri::from_file_path(path!("/root/dir-1/two.rs")).unwrap(),
                range: lsp::Range::new(lsp::Position::new(0, 35), lsp::Position::new(0, 38)),
            },
            lsp::Location {
                uri: lsp::Uri::from_file_path(path!("/root/dir-2/three.rs")).unwrap(),
                range: lsp::Range::new(lsp::Position::new(0, 37), lsp::Position::new(0, 40)),
            },
        ])))
        .unwrap();

    let references = references.await.unwrap().unwrap();
    executor.run_until_parked();
    project_b.read_with(cx_b, |project, cx| {
        // User is informed that a request is no longer pending.
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert!(status.pending_work.is_empty());

        assert_eq!(references.len(), 3);
        assert_eq!(project.worktrees(cx).count(), 2);

        let two_buffer = references[0].buffer.read(cx);
        let three_buffer = references[2].buffer.read(cx);
        assert_eq!(
            two_buffer.file().unwrap().path().as_ref(),
            Path::new("two.rs")
        );
        assert_eq!(references[1].buffer, references[0].buffer);
        assert_eq!(
            three_buffer.file().unwrap().full_path(cx),
            Path::new(path!("/root/dir-2/three.rs"))
        );

        assert_eq!(references[0].range.to_offset(two_buffer), 24..27);
        assert_eq!(references[1].range.to_offset(two_buffer), 35..38);
        assert_eq!(references[2].range.to_offset(three_buffer), 37..40);
    });

    let references = project_b.update(cx_b, |p, cx| p.references(&buffer_b, 7, cx));

    // User is informed that a request is pending.
    executor.run_until_parked();
    project_b.read_with(cx_b, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert_eq!(status.name.0, "my-fake-lsp-adapter");
        assert_eq!(
            status.pending_work.values().next().unwrap().message,
            Some("Finding references...".into())
        );
    });

    // Cause the LSP request to fail.
    lsp_response_tx
        .unbounded_send(Err(anyhow!("can't find references")))
        .unwrap();
    assert_eq!(references.await.unwrap().unwrap(), []);

    // User is informed that the request is no longer pending.
    executor.run_until_parked();
    project_b.read_with(cx_b, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert!(status.pending_work.is_empty());
    });
}

#[gpui::test(iterations = 10)]
async fn test_project_search(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            "/root",
            json!({
                "dir-1": {
                    "a": "hello world",
                    "b": "goodnight moon",
                    "c": "a world of goo",
                    "d": "world champion of clown world",
                },
                "dir-2": {
                    "e": "disney world is fun",
                }
            }),
        )
        .await;
    let (project_a, _) = client_a.build_local_project("/root/dir-1", cx_a).await;
    let (worktree_2, _) = project_a
        .update(cx_a, |p, cx| {
            p.find_or_create_worktree("/root/dir-2", true, cx)
        })
        .await
        .unwrap();
    worktree_2
        .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Perform a search as the guest.
    let mut results = HashMap::default();
    let search_rx = project_b.update(cx_b, |project, cx| {
        project.search(
            SearchQuery::text(
                "world",
                false,
                false,
                false,
                Default::default(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
            cx,
        )
    });
    while let Ok(result) = search_rx.recv().await {
        match result {
            SearchResult::Buffer { buffer, ranges } => {
                results.entry(buffer).or_insert(ranges);
            }
            SearchResult::LimitReached => {
                panic!(
                    "Unexpectedly reached search limit in tests. If you do want to assert limit-reached, change this panic call."
                )
            }
        };
    }

    let mut ranges_by_path = results
        .into_iter()
        .map(|(buffer, ranges)| {
            buffer.read_with(cx_b, |buffer, cx| {
                let path = buffer.file().unwrap().full_path(cx);
                let offset_ranges = ranges
                    .into_iter()
                    .map(|range| range.to_offset(buffer))
                    .collect::<Vec<_>>();
                (path, offset_ranges)
            })
        })
        .collect::<Vec<_>>();
    ranges_by_path.sort_by_key(|(path, _)| path.clone());

    assert_eq!(
        ranges_by_path,
        &[
            (PathBuf::from("dir-1/a"), vec![6..11]),
            (PathBuf::from("dir-1/c"), vec![2..7]),
            (PathBuf::from("dir-1/d"), vec![0..5, 24..29]),
            (PathBuf::from("dir-2/e"), vec![7..12]),
        ]
    );
}

#[gpui::test(iterations = 10)]
async fn test_document_highlights(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            path!("/root-1"),
            json!({
                "main.rs": "fn double(number: i32) -> i32 { number + number }",
            }),
        )
        .await;

    client_a.language_registry().add(rust_lang());
    let capabilities = lsp::ServerCapabilities {
        document_highlight_provider: Some(lsp::OneOf::Left(true)),
        ..lsp::ServerCapabilities::default()
    };
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    let (project_a, worktree_id) = client_a.build_local_project(path!("/root-1"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open the file on client B.
    let (buffer_b, _handle) = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    // Request document highlights as the guest.
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.set_request_handler::<lsp::request::DocumentHighlightRequest, _, _>(
        |params, _| async move {
            assert_eq!(
                params
                    .text_document_position_params
                    .text_document
                    .uri
                    .as_str(),
                uri!("file:///root-1/main.rs")
            );
            assert_eq!(
                params.text_document_position_params.position,
                lsp::Position::new(0, 34)
            );
            Ok(Some(vec![
                lsp::DocumentHighlight {
                    kind: Some(lsp::DocumentHighlightKind::WRITE),
                    range: lsp::Range::new(lsp::Position::new(0, 10), lsp::Position::new(0, 16)),
                },
                lsp::DocumentHighlight {
                    kind: Some(lsp::DocumentHighlightKind::READ),
                    range: lsp::Range::new(lsp::Position::new(0, 32), lsp::Position::new(0, 38)),
                },
                lsp::DocumentHighlight {
                    kind: Some(lsp::DocumentHighlightKind::READ),
                    range: lsp::Range::new(lsp::Position::new(0, 41), lsp::Position::new(0, 47)),
                },
            ]))
        },
    );
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let highlights = project_b
        .update(cx_b, |p, cx| p.document_highlights(&buffer_b, 34, cx))
        .await
        .unwrap();

    buffer_b.read_with(cx_b, |buffer, _| {
        let snapshot = buffer.snapshot();

        let highlights = highlights
            .into_iter()
            .map(|highlight| (highlight.kind, highlight.range.to_offset(&snapshot)))
            .collect::<Vec<_>>();
        assert_eq!(
            highlights,
            &[
                (lsp::DocumentHighlightKind::WRITE, 10..16),
                (lsp::DocumentHighlightKind::READ, 32..38),
                (lsp::DocumentHighlightKind::READ, 41..47)
            ]
        )
    });
}

#[gpui::test(iterations = 10)]
async fn test_lsp_hover(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            path!("/root-1"),
            json!({
                "main.rs": "use std::collections::HashMap;",
            }),
        )
        .await;

    client_a.language_registry().add(rust_lang());
    let language_server_names = ["rust-analyzer", "CrabLang-ls"];
    let capabilities_1 = lsp::ServerCapabilities {
        hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
        ..lsp::ServerCapabilities::default()
    };
    let capabilities_2 = lsp::ServerCapabilities {
        hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
        ..lsp::ServerCapabilities::default()
    };
    let mut language_servers = [
        client_a.language_registry().register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: language_server_names[0],
                capabilities: capabilities_1.clone(),
                ..FakeLspAdapter::default()
            },
        ),
        client_a.language_registry().register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: language_server_names[1],
                capabilities: capabilities_2.clone(),
                ..FakeLspAdapter::default()
            },
        ),
    ];
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: language_server_names[0],
            capabilities: capabilities_1,
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: language_server_names[1],
            capabilities: capabilities_2,
            ..FakeLspAdapter::default()
        },
    );

    let (project_a, worktree_id) = client_a.build_local_project(path!("/root-1"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open the file as the guest
    let (buffer_b, _handle) = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    let mut servers_with_hover_requests = HashMap::default();
    for i in 0..language_server_names.len() {
        let new_server = language_servers[i].next().await.unwrap_or_else(|| {
            panic!(
                "Failed to get language server #{i} with name {}",
                &language_server_names[i]
            )
        });
        let new_server_name = new_server.server.name();
        assert!(
            !servers_with_hover_requests.contains_key(&new_server_name),
            "Unexpected: initialized server with the same name twice. Name: `{new_server_name}`"
        );
        match new_server_name.as_ref() {
            "CrabLang-ls" => {
                servers_with_hover_requests.insert(
                    new_server_name.clone(),
                    new_server.set_request_handler::<lsp::request::HoverRequest, _, _>(
                        move |params, _| {
                            assert_eq!(
                                params
                                    .text_document_position_params
                                    .text_document
                                    .uri
                                    .as_str(),
                                uri!("file:///root-1/main.rs")
                            );
                            let name = new_server_name.clone();
                            async move {
                                Ok(Some(lsp::Hover {
                                    contents: lsp::HoverContents::Scalar(
                                        lsp::MarkedString::String(format!("{name} hover")),
                                    ),
                                    range: None,
                                }))
                            }
                        },
                    ),
                );
            }
            "rust-analyzer" => {
                servers_with_hover_requests.insert(
                    new_server_name.clone(),
                    new_server.set_request_handler::<lsp::request::HoverRequest, _, _>(
                        |params, _| async move {
                            assert_eq!(
                                params
                                    .text_document_position_params
                                    .text_document
                                    .uri
                                    .as_str(),
                                uri!("file:///root-1/main.rs")
                            );
                            assert_eq!(
                                params.text_document_position_params.position,
                                lsp::Position::new(0, 22)
                            );
                            Ok(Some(lsp::Hover {
                                contents: lsp::HoverContents::Array(vec![
                                    lsp::MarkedString::String("Test hover content.".to_string()),
                                    lsp::MarkedString::LanguageString(lsp::LanguageString {
                                        language: "Rust".to_string(),
                                        value: "let foo = 42;".to_string(),
                                    }),
                                ]),
                                range: Some(lsp::Range::new(
                                    lsp::Position::new(0, 22),
                                    lsp::Position::new(0, 29),
                                )),
                            }))
                        },
                    ),
                );
            }
            unexpected => panic!("Unexpected server name: {unexpected}"),
        }
    }
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    // Request hover information as the guest.
    let mut hovers = project_b
        .update(cx_b, |p, cx| p.hover(&buffer_b, 22, cx))
        .await
        .unwrap();
    assert_eq!(
        hovers.len(),
        2,
        "Expected two hovers from both language servers, but got: {hovers:?}"
    );

    let _: Vec<()> = futures::future::join_all(servers_with_hover_requests.into_values().map(
        |mut hover_request| async move {
            hover_request
                .next()
                .await
                .expect("All hover requests should have been triggered")
        },
    ))
    .await;

    hovers.sort_by_key(|hover| hover.contents.len());
    let first_hover = hovers.first().cloned().unwrap();
    assert_eq!(
        first_hover.contents,
        vec![project::HoverBlock {
            text: "CrabLang-ls hover".to_string(),
            kind: HoverBlockKind::Markdown,
        },]
    );
    let second_hover = hovers.last().cloned().unwrap();
    assert_eq!(
        second_hover.contents,
        vec![
            project::HoverBlock {
                text: "Test hover content.".to_string(),
                kind: HoverBlockKind::Markdown,
            },
            project::HoverBlock {
                text: "let foo = 42;".to_string(),
                kind: HoverBlockKind::Code {
                    language: "Rust".to_string()
                },
            }
        ]
    );
    buffer_b.read_with(cx_b, |buffer, _| {
        let snapshot = buffer.snapshot();
        assert_eq!(second_hover.range.unwrap().to_offset(&snapshot), 22..29);
    });
}

#[gpui::test(iterations = 10)]
async fn test_project_symbols(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                workspace_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/code"),
            json!({
                "crate-1": {
                    "one.rs": "const ONE: usize = 1;",
                },
                "crate-2": {
                    "two.rs": "const TWO: usize = 2; const THREE: usize = 3;",
                },
                "private": {
                    "passwords.txt": "the-password",
                }
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a
        .build_local_project(path!("/code/crate-1"), cx_a)
        .await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Cause the language server to start.
    let _buffer = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, "one.rs"), cx)
        })
        .await
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.set_request_handler::<lsp::WorkspaceSymbolRequest, _, _>(
        |_, _| async move {
            Ok(Some(lsp::WorkspaceSymbolResponse::Flat(vec![
                #[allow(deprecated)]
                lsp::SymbolInformation {
                    name: "TWO".into(),
                    location: lsp::Location {
                        uri: lsp::Uri::from_file_path(path!("/code/crate-2/two.rs")).unwrap(),
                        range: lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                    },
                    kind: lsp::SymbolKind::CONSTANT,
                    tags: None,
                    container_name: None,
                    deprecated: None,
                },
            ])))
        },
    );

    // Request the definition of a symbol as the guest.
    let symbols = project_b
        .update(cx_b, |p, cx| p.symbols("two", cx))
        .await
        .unwrap();
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name, "TWO");

    // Open one of the returned symbols.
    let buffer_b_2 = project_b
        .update(cx_b, |project, cx| {
            project.open_buffer_for_symbol(&symbols[0], cx)
        })
        .await
        .unwrap();

    buffer_b_2.read_with(cx_b, |buffer, cx| {
        assert_eq!(
            buffer.file().unwrap().full_path(cx),
            Path::new(path!("/code/crate-2/two.rs"))
        );
    });

    // Attempt to craft a symbol and violate host's privacy by opening an arbitrary file.
    let mut fake_symbol = symbols[0].clone();
    fake_symbol.path.path = Path::new(path!("/code/secrets")).into();
    let error = project_b
        .update(cx_b, |project, cx| {
            project.open_buffer_for_symbol(&fake_symbol, cx)
        })
        .await
        .unwrap_err();
    assert!(error.to_string().contains("invalid symbol signature"));
}

#[gpui::test(iterations = 10)]
async fn test_open_buffer_while_getting_definition_pointing_to_it(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    mut rng: StdRng,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let capabilities = lsp::ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/root"),
            json!({
                "a.rs": "const ONE: usize = b::TWO;",
                "b.rs": "const TWO: usize = 2",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/root"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let (buffer_b1, _lsp) = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer_with_lsp((worktree_id, "a.rs"), cx)
        })
        .await
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.set_request_handler::<lsp::request::GotoDefinition, _, _>(
        |_, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                lsp::Location::new(
                    lsp::Uri::from_file_path(path!("/root/b.rs")).unwrap(),
                    lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                ),
            )))
        },
    );

    let definitions;
    let buffer_b2;
    if rng.random() {
        cx_a.run_until_parked();
        cx_b.run_until_parked();
        definitions = project_b.update(cx_b, |p, cx| p.definitions(&buffer_b1, 23, cx));
        (buffer_b2, _) = project_b
            .update(cx_b, |p, cx| {
                p.open_buffer_with_lsp((worktree_id, "b.rs"), cx)
            })
            .await
            .unwrap();
    } else {
        (buffer_b2, _) = project_b
            .update(cx_b, |p, cx| {
                p.open_buffer_with_lsp((worktree_id, "b.rs"), cx)
            })
            .await
            .unwrap();
        cx_a.run_until_parked();
        cx_b.run_until_parked();
        definitions = project_b.update(cx_b, |p, cx| p.definitions(&buffer_b1, 23, cx));
    }

    let definitions = definitions.await.unwrap().unwrap();
    assert_eq!(
        definitions.len(),
        1,
        "Unexpected definitions: {definitions:?}"
    );
    assert_eq!(definitions[0].target.buffer, buffer_b2);
}

#[gpui::test(iterations = 10)]
async fn test_contacts(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_d: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    let client_d = server.create_client(cx_d, "user_d").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);
    let _active_call_d = cx_d.read(ActiveCall::global);

    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_b".to_string(), "online", "free")
        ]
    );
    assert_eq!(contacts(&client_d, cx_d), []);

    server.disconnect_client(client_c.peer_id().unwrap());
    server.forbid_connections();
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "free"),
            ("user_c".to_string(), "offline", "free")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_c".to_string(), "offline", "free")
        ]
    );
    assert_eq!(contacts(&client_c, cx_c), []);
    assert_eq!(contacts(&client_d, cx_d), []);

    server.allow_connections();
    client_c
        .connect(false, &cx_c.to_async())
        .await
        .into_response()
        .unwrap();

    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_b".to_string(), "online", "free")
        ]
    );
    assert_eq!(contacts(&client_d, cx_d), []);

    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "busy")
        ]
    );
    assert_eq!(contacts(&client_d, cx_d), []);

    // Client B and client D become contacts while client B is being called.
    server
        .make_contacts(&mut [(&client_b, cx_b), (&client_d, cx_d)])
        .await;
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "free"),
            ("user_d".to_string(), "online", "free"),
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_d, cx_d),
        [("user_b".to_string(), "online", "busy")]
    );

    active_call_b.update(cx_b, |call, cx| call.decline_incoming(cx).unwrap());
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "free"),
            ("user_d".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_b".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_d, cx_d),
        [("user_b".to_string(), "online", "free")]
    );

    active_call_c
        .update(cx_c, |call, cx| {
            call.invite(client_a.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "busy"),
            ("user_d".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_d, cx_d),
        [("user_b".to_string(), "online", "free")]
    );

    active_call_a
        .update(cx_a, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "busy"),
            ("user_d".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_d, cx_d),
        [("user_b".to_string(), "online", "free")]
    );

    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "busy"),
            ("user_d".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_d, cx_d),
        [("user_b".to_string(), "online", "busy")]
    );

    active_call_a
        .update(cx_a, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_c".to_string(), "online", "free"),
            ("user_d".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "free"),
            ("user_b".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_d, cx_d),
        [("user_b".to_string(), "online", "free")]
    );

    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_a, cx_a),
        [
            ("user_b".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_c".to_string(), "online", "free"),
            ("user_d".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_d, cx_d),
        [("user_b".to_string(), "online", "busy")]
    );

    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    assert_eq!(contacts(&client_a, cx_a), []);
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "offline", "free"),
            ("user_c".to_string(), "online", "free"),
            ("user_d".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "offline", "free"),
            ("user_b".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_d, cx_d),
        [("user_b".to_string(), "online", "free")]
    );

    // Test removing a contact
    client_b
        .user_store()
        .update(cx_b, |store, cx| {
            store.remove_contact(client_c.user_id().unwrap(), cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "offline", "free"),
            ("user_d".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [("user_a".to_string(), "offline", "free"),]
    );

    fn contacts(
        client: &TestClient,
        cx: &TestAppContext,
    ) -> Vec<(String, &'static str, &'static str)> {
        client.user_store().read_with(cx, |store, _| {
            store
                .contacts()
                .iter()
                .map(|contact| {
                    (
                        contact.user.github_login.clone().to_string(),
                        if contact.online { "online" } else { "offline" },
                        if contact.busy { "busy" } else { "free" },
                    )
                })
                .collect()
        })
    }
}

#[gpui::test(iterations = 10)]
async fn test_contact_requests(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_a2: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_c2: &mut TestAppContext,
) {
    // Connect to a server as 3 clients.
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_a2 = server.create_client(cx_a2, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_b2 = server.create_client(cx_b2, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    let client_c2 = server.create_client(cx_c2, "user_c").await;

    assert_eq!(client_a.user_id().unwrap(), client_a2.user_id().unwrap());
    assert_eq!(client_b.user_id().unwrap(), client_b2.user_id().unwrap());
    assert_eq!(client_c.user_id().unwrap(), client_c2.user_id().unwrap());

    // User A and User C request that user B become their contact.
    client_a
        .user_store()
        .update(cx_a, |store, cx| {
            store.request_contact(client_b.user_id().unwrap(), cx)
        })
        .await
        .unwrap();
    client_c
        .user_store()
        .update(cx_c, |store, cx| {
            store.request_contact(client_b.user_id().unwrap(), cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();

    // All users see the pending request appear in all their clients.
    assert_eq!(
        client_a.summarize_contacts(cx_a).outgoing_requests,
        &["user_b"]
    );
    assert_eq!(
        client_a2.summarize_contacts(cx_a2).outgoing_requests,
        &["user_b"]
    );
    assert_eq!(
        client_b.summarize_contacts(cx_b).incoming_requests,
        &["user_a", "user_c"]
    );
    assert_eq!(
        client_b2.summarize_contacts(cx_b2).incoming_requests,
        &["user_a", "user_c"]
    );
    assert_eq!(
        client_c.summarize_contacts(cx_c).outgoing_requests,
        &["user_b"]
    );
    assert_eq!(
        client_c2.summarize_contacts(cx_c2).outgoing_requests,
        &["user_b"]
    );

    // Contact requests are present upon connecting (tested here via disconnect/reconnect)
    disconnect_and_reconnect(&client_a, cx_a).await;
    disconnect_and_reconnect(&client_b, cx_b).await;
    disconnect_and_reconnect(&client_c, cx_c).await;
    executor.run_until_parked();
    assert_eq!(
        client_a.summarize_contacts(cx_a).outgoing_requests,
        &["user_b"]
    );
    assert_eq!(
        client_b.summarize_contacts(cx_b).incoming_requests,
        &["user_a", "user_c"]
    );
    assert_eq!(
        client_c.summarize_contacts(cx_c).outgoing_requests,
        &["user_b"]
    );

    // User B accepts the request from user A.
    client_b
        .user_store()
        .update(cx_b, |store, cx| {
            store.respond_to_contact_request(client_a.user_id().unwrap(), true, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // User B sees user A as their contact now in all client, and the incoming request from them is removed.
    let contacts_b = client_b.summarize_contacts(cx_b);
    assert_eq!(contacts_b.current, &["user_a"]);
    assert_eq!(contacts_b.incoming_requests, &["user_c"]);
    let contacts_b2 = client_b2.summarize_contacts(cx_b2);
    assert_eq!(contacts_b2.current, &["user_a"]);
    assert_eq!(contacts_b2.incoming_requests, &["user_c"]);

    // User A sees user B as their contact now in all clients, and the outgoing request to them is removed.
    let contacts_a = client_a.summarize_contacts(cx_a);
    assert_eq!(contacts_a.current, &["user_b"]);
    assert!(contacts_a.outgoing_requests.is_empty());
    let contacts_a2 = client_a2.summarize_contacts(cx_a2);
    assert_eq!(contacts_a2.current, &["user_b"]);
    assert!(contacts_a2.outgoing_requests.is_empty());

    // Contacts are present upon connecting (tested here via disconnect/reconnect)
    disconnect_and_reconnect(&client_a, cx_a).await;
    disconnect_and_reconnect(&client_b, cx_b).await;
    disconnect_and_reconnect(&client_c, cx_c).await;
    executor.run_until_parked();
    assert_eq!(client_a.summarize_contacts(cx_a).current, &["user_b"]);
    assert_eq!(client_b.summarize_contacts(cx_b).current, &["user_a"]);
    assert_eq!(
        client_b.summarize_contacts(cx_b).incoming_requests,
        &["user_c"]
    );
    assert!(client_c.summarize_contacts(cx_c).current.is_empty());
    assert_eq!(
        client_c.summarize_contacts(cx_c).outgoing_requests,
        &["user_b"]
    );

    // User B rejects the request from user C.
    client_b
        .user_store()
        .update(cx_b, |store, cx| {
            store.respond_to_contact_request(client_c.user_id().unwrap(), false, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    // User B doesn't see user C as their contact, and the incoming request from them is removed.
    let contacts_b = client_b.summarize_contacts(cx_b);
    assert_eq!(contacts_b.current, &["user_a"]);
    assert!(contacts_b.incoming_requests.is_empty());
    let contacts_b2 = client_b2.summarize_contacts(cx_b2);
    assert_eq!(contacts_b2.current, &["user_a"]);
    assert!(contacts_b2.incoming_requests.is_empty());

    // User C doesn't see user B as their contact, and the outgoing request to them is removed.
    let contacts_c = client_c.summarize_contacts(cx_c);
    assert!(contacts_c.current.is_empty());
    assert!(contacts_c.outgoing_requests.is_empty());
    let contacts_c2 = client_c2.summarize_contacts(cx_c2);
    assert!(contacts_c2.current.is_empty());
    assert!(contacts_c2.outgoing_requests.is_empty());

    // Incoming/outgoing requests are not present upon connecting (tested here via disconnect/reconnect)
    disconnect_and_reconnect(&client_a, cx_a).await;
    disconnect_and_reconnect(&client_b, cx_b).await;
    disconnect_and_reconnect(&client_c, cx_c).await;
    executor.run_until_parked();
    assert_eq!(client_a.summarize_contacts(cx_a).current, &["user_b"]);
    assert_eq!(client_b.summarize_contacts(cx_b).current, &["user_a"]);
    assert!(
        client_b
            .summarize_contacts(cx_b)
            .incoming_requests
            .is_empty()
    );
    assert!(client_c.summarize_contacts(cx_c).current.is_empty());
    assert!(
        client_c
            .summarize_contacts(cx_c)
            .outgoing_requests
            .is_empty()
    );

    async fn disconnect_and_reconnect(client: &TestClient, cx: &mut TestAppContext) {
        client.disconnect(&cx.to_async());
        client.clear_contacts(cx).await;
        client
            .connect(false, &cx.to_async())
            .await
            .into_response()
            .unwrap();
    }
}

#[gpui::test(iterations = 10)]
async fn test_join_call_after_screen_was_shared(
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

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    // Call users B and C from client A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();

    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: Default::default(),
            pending: vec!["user_b".to_string()]
        }
    );

    // User B receives the call.

    let mut incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
    let call_b = incoming_call_b.next().await.unwrap().unwrap();
    assert_eq!(call_b.calling_user.github_login, "user_a");

    // User A shares their screen
    let display = gpui::TestScreenCaptureSource::new();
    cx_a.set_screen_capture_sources(vec![display]);
    let screen_a = cx_a
        .update(|cx| cx.screen_capture_sources())
        .await
        .unwrap()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();

    active_call_a
        .update(cx_a, |call, cx| {
            call.room()
                .unwrap()
                .update(cx, |room, cx| room.share_screen(screen_a, cx))
        })
        .await
        .unwrap();

    client_b.user_store().update(cx_b, |user_store, _| {
        user_store.clear_cache();
    });

    // User B joins the room
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    assert!(incoming_call_b.next().await.unwrap().is_none());

    executor.run_until_parked();
    assert_eq!(
        room_participants(&room_a, cx_a),
        RoomParticipants {
            remote: vec!["user_b".to_string()],
            pending: vec![],
        }
    );
    assert_eq!(
        room_participants(&room_b, cx_b),
        RoomParticipants {
            remote: vec!["user_a".to_string()],
            pending: vec![],
        }
    );

    // Ensure User B sees User A's screenshare.

    room_b.read_with(cx_b, |room, _| {
        assert_eq!(
            room.remote_participants()
                .get(&client_a.user_id().unwrap())
                .unwrap()
                .video_tracks
                .len(),
            1
        );
    });
}

#[gpui::test]
async fn test_right_click_menu_behind_collab_panel(cx: &mut TestAppContext) {
    let mut server = TestServer::start(cx.executor().clone()).await;
    let client_a = server.create_client(cx, "user_a").await;
    let (_workspace_a, cx) = client_a.build_test_workspace(cx).await;

    cx.simulate_resize(size(px(300.), px(300.)));

    cx.simulate_keystrokes("cmd-n cmd-n cmd-n");
    cx.update(|window, _cx| window.refresh());

    let tab_bounds = cx.debug_bounds("TAB-2").unwrap();
    let new_tab_button_bounds = cx.debug_bounds("ICON-Plus").unwrap();

    assert!(
        tab_bounds.intersects(&new_tab_button_bounds),
        "Tab should overlap with the new tab button, if this is failing check if there's been a redesign!"
    );

    cx.simulate_event(MouseDownEvent {
        button: MouseButton::Right,
        position: new_tab_button_bounds.center(),
        modifiers: Modifiers::default(),
        click_count: 1,
        first_mouse: false,
    });

    // regression test that the right click menu for tabs does not open.
    assert!(cx.debug_bounds("MENU_ITEM-Close").is_none());

    let tab_bounds = cx.debug_bounds("TAB-1").unwrap();
    cx.simulate_event(MouseDownEvent {
        button: MouseButton::Right,
        position: tab_bounds.center(),
        modifiers: Modifiers::default(),
        click_count: 1,
        first_mouse: false,
    });
    assert!(cx.debug_bounds("MENU_ITEM-Close").is_some());
}

#[gpui::test]
async fn test_pane_split_left(cx: &mut TestAppContext) {
    let (_, client) = TestServer::start1(cx).await;
    let (workspace, cx) = client.build_test_workspace(cx).await;

    cx.simulate_keystrokes("cmd-n");
    workspace.update(cx, |workspace, cx| {
        assert!(workspace.items(cx).collect::<Vec<_>>().len() == 1);
    });
    cx.simulate_keystrokes("cmd-k left");
    workspace.update(cx, |workspace, cx| {
        assert!(workspace.items(cx).collect::<Vec<_>>().len() == 2);
    });
    cx.simulate_keystrokes("cmd-k");
    // sleep for longer than the timeout in keyboard shortcut handling
    // to verify that it doesn't fire in this case.
    cx.executor().advance_clock(Duration::from_secs(2));
    cx.simulate_keystrokes("left");
    workspace.update(cx, |workspace, cx| {
        assert!(workspace.items(cx).collect::<Vec<_>>().len() == 2);
    });
}

#[gpui::test]
async fn test_join_after_restart(cx1: &mut TestAppContext, cx2: &mut TestAppContext) {
    let (mut server, client) = TestServer::start1(cx1).await;
    let channel1 = server.make_public_channel("channel1", &client, cx1).await;
    let channel2 = server.make_public_channel("channel2", &client, cx1).await;

    join_channel(channel1, &client, cx1).await.unwrap();
    drop(client);

    let client2 = server.create_client(cx2, "user_a").await;
    join_channel(channel2, &client2, cx2).await.unwrap();
}

#[gpui::test]
async fn test_preview_tabs(cx: &mut TestAppContext) {
    let (_server, client) = TestServer::start1(cx).await;
    let (workspace, cx) = client.build_test_workspace(cx).await;
    let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());

    let worktree_id = project.update(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });

    let path_1 = ProjectPath {
        worktree_id,
        path: Path::new("1.txt").into(),
    };
    let path_2 = ProjectPath {
        worktree_id,
        path: Path::new("2.js").into(),
    };
    let path_3 = ProjectPath {
        worktree_id,
        path: Path::new("3.rs").into(),
    };

    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

    let get_path = |pane: &Pane, idx: usize, cx: &App| {
        pane.item_for_index(idx).unwrap().project_path(cx).unwrap()
    };

    // Opening item 3 as a "permanent" tab
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(path_3.clone(), None, false, window, cx)
        })
        .await
        .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 1);
        assert_eq!(get_path(pane, 0, cx), path_3.clone());
        assert_eq!(pane.preview_item_id(), None);

        assert!(!pane.can_navigate_backward());
        assert!(!pane.can_navigate_forward());
    });

    // Open item 1 as preview
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path_preview(path_1.clone(), None, true, true, true, window, cx)
        })
        .await
        .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 2);
        assert_eq!(get_path(pane, 0, cx), path_3.clone());
        assert_eq!(get_path(pane, 1, cx), path_1.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().nth(1).unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(!pane.can_navigate_forward());
    });

    // Open item 2 as preview
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path_preview(path_2.clone(), None, true, true, true, window, cx)
        })
        .await
        .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 2);
        assert_eq!(get_path(pane, 0, cx), path_3.clone());
        assert_eq!(get_path(pane, 1, cx), path_2.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().nth(1).unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(!pane.can_navigate_forward());
    });

    // Going back should show item 1 as preview
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.go_back(pane.downgrade(), window, cx)
        })
        .await
        .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 2);
        assert_eq!(get_path(pane, 0, cx), path_3.clone());
        assert_eq!(get_path(pane, 1, cx), path_1.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().nth(1).unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(pane.can_navigate_forward());
    });

    // Closing item 1
    pane.update_in(cx, |pane, window, cx| {
        pane.close_item_by_id(
            pane.active_item().unwrap().item_id(),
            workspace::SaveIntent::Skip,
            window,
            cx,
        )
    })
    .await
    .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 1);
        assert_eq!(get_path(pane, 0, cx), path_3.clone());
        assert_eq!(pane.preview_item_id(), None);

        assert!(pane.can_navigate_backward());
        assert!(!pane.can_navigate_forward());
    });

    // Going back should show item 1 as preview
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.go_back(pane.downgrade(), window, cx)
        })
        .await
        .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 2);
        assert_eq!(get_path(pane, 0, cx), path_3.clone());
        assert_eq!(get_path(pane, 1, cx), path_1.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().nth(1).unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(pane.can_navigate_forward());
    });

    // Close permanent tab
    pane.update_in(cx, |pane, window, cx| {
        let id = pane.items().next().unwrap().item_id();
        pane.close_item_by_id(id, workspace::SaveIntent::Skip, window, cx)
    })
    .await
    .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 1);
        assert_eq!(get_path(pane, 0, cx), path_1.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().next().unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(pane.can_navigate_forward());
    });

    // Split pane to the right
    pane.update(cx, |pane, cx| {
        pane.split(workspace::SplitDirection::Right, cx);
    });

    let right_pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 1);
        assert_eq!(get_path(pane, 0, cx), path_1.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().next().unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(pane.can_navigate_forward());
    });

    right_pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 1);
        assert_eq!(get_path(pane, 0, cx), path_1.clone());
        assert_eq!(pane.preview_item_id(), None);

        assert!(!pane.can_navigate_backward());
        assert!(!pane.can_navigate_forward());
    });

    // Open item 2 as preview in right pane
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path_preview(path_2.clone(), None, true, true, true, window, cx)
        })
        .await
        .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 1);
        assert_eq!(get_path(pane, 0, cx), path_1.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().next().unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(pane.can_navigate_forward());
    });

    right_pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 2);
        assert_eq!(get_path(pane, 0, cx), path_1.clone());
        assert_eq!(get_path(pane, 1, cx), path_2.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().nth(1).unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(!pane.can_navigate_forward());
    });

    // Focus left pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.activate_pane_in_direction(workspace::SplitDirection::Left, window, cx)
    });

    // Open item 2 as preview in left pane
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path_preview(path_2.clone(), None, true, true, true, window, cx)
        })
        .await
        .unwrap();

    pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 1);
        assert_eq!(get_path(pane, 0, cx), path_2.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().next().unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(!pane.can_navigate_forward());
    });

    right_pane.update(cx, |pane, cx| {
        assert_eq!(pane.items_len(), 2);
        assert_eq!(get_path(pane, 0, cx), path_1.clone());
        assert_eq!(get_path(pane, 1, cx), path_2.clone());
        assert_eq!(
            pane.preview_item_id(),
            Some(pane.items().nth(1).unwrap().item_id())
        );

        assert!(pane.can_navigate_backward());
        assert!(!pane.can_navigate_forward());
    });
}

#[gpui::test(iterations = 10)]
async fn test_context_collaboration_with_reconnect(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a.fs().insert_tree("/a", Default::default()).await;
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Client A sees that a guest has joined.
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });
    project_b.read_with(cx_b, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });

    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let context_store_a = cx_a
        .update(|cx| {
            ContextStore::new(
                project_a.clone(),
                prompt_builder.clone(),
                Arc::new(SlashCommandWorkingSet::default()),
                cx,
            )
        })
        .await
        .unwrap();
    let context_store_b = cx_b
        .update(|cx| {
            ContextStore::new(
                project_b.clone(),
                prompt_builder.clone(),
                Arc::new(SlashCommandWorkingSet::default()),
                cx,
            )
        })
        .await
        .unwrap();

    // Client A creates a new chats.
    let context_a = context_store_a.update(cx_a, |store, cx| store.create(cx));
    executor.run_until_parked();

    // Client B retrieves host's contexts and joins one.
    let context_b = context_store_b
        .update(cx_b, |store, cx| {
            let host_contexts = store.host_contexts().to_vec();
            assert_eq!(host_contexts.len(), 1);
            store.open_remote_context(host_contexts[0].id.clone(), cx)
        })
        .await
        .unwrap();

    // Host and guest make changes
    context_a.update(cx_a, |context, cx| {
        context.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "Host change\n")], None, cx)
        })
    });
    context_b.update(cx_b, |context, cx| {
        context.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "Guest change\n")], None, cx)
        })
    });
    executor.run_until_parked();
    assert_eq!(
        context_a.read_with(cx_a, |context, cx| context.buffer().read(cx).text()),
        "Guest change\nHost change\n"
    );
    assert_eq!(
        context_b.read_with(cx_b, |context, cx| context.buffer().read(cx).text()),
        "Guest change\nHost change\n"
    );

    // Disconnect client A and make some changes while disconnected.
    server.disconnect_client(client_a.peer_id().unwrap());
    server.forbid_connections();
    context_a.update(cx_a, |context, cx| {
        context.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "Host offline change\n")], None, cx)
        })
    });
    context_b.update(cx_b, |context, cx| {
        context.buffer().update(cx, |buffer, cx| {
            buffer.edit([(0..0, "Guest offline change\n")], None, cx)
        })
    });
    executor.run_until_parked();
    assert_eq!(
        context_a.read_with(cx_a, |context, cx| context.buffer().read(cx).text()),
        "Host offline change\nGuest change\nHost change\n"
    );
    assert_eq!(
        context_b.read_with(cx_b, |context, cx| context.buffer().read(cx).text()),
        "Guest offline change\nGuest change\nHost change\n"
    );

    // Allow client A to reconnect and verify that contexts converge.
    server.allow_connections();
    executor.advance_clock(RECEIVE_TIMEOUT);
    assert_eq!(
        context_a.read_with(cx_a, |context, cx| context.buffer().read(cx).text()),
        "Guest offline change\nHost offline change\nGuest change\nHost change\n"
    );
    assert_eq!(
        context_b.read_with(cx_b, |context, cx| context.buffer().read(cx).text()),
        "Guest offline change\nHost offline change\nGuest change\nHost change\n"
    );

    // Client A disconnects without being able to reconnect. Context B becomes readonly.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    context_b.read_with(cx_b, |context, cx| {
        assert!(context.buffer().read(cx).read_only());
    });
}

#[gpui::test]
async fn test_remote_git_branches(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree("/project", serde_json::json!({ ".git":{} }))
        .await;
    let branches = ["main", "dev", "feature-1"];
    client_a
        .fs()
        .insert_branches(Path::new("/project/.git"), &branches);
    let branches_set = branches
        .into_iter()
        .map(ToString::to_string)
        .collect::<HashSet<_>>();

    let (project_a, _) = client_a.build_local_project("/project", cx_a).await;

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Client A sees that a guest has joined and the repo has been populated
    executor.run_until_parked();

    let repo_b = cx_b.update(|cx| project_b.read(cx).active_repository(cx).unwrap());

    let branches_b = cx_b
        .update(|cx| repo_b.update(cx, |repository, _| repository.branches()))
        .await
        .unwrap()
        .unwrap();

    let new_branch = branches[2];

    let branches_b = branches_b
        .into_iter()
        .map(|branch| branch.name().to_string())
        .collect::<HashSet<_>>();

    assert_eq!(branches_b, branches_set);

    cx_b.update(|cx| {
        repo_b.update(cx, |repository, _cx| {
            repository.change_branch(new_branch.to_string())
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

    let host_branch = cx_a.update(|cx| {
        project_a.update(cx, |project, cx| {
            project
                .repositories(cx)
                .values()
                .next()
                .unwrap()
                .read(cx)
                .branch
                .as_ref()
                .unwrap()
                .clone()
        })
    });

    assert_eq!(host_branch.name(), branches[2]);

    // Also try creating a new branch
    cx_b.update(|cx| {
        repo_b.update(cx, |repository, _cx| {
            repository.create_branch("totally-new-branch".to_string())
        })
    })
    .await
    .unwrap()
    .unwrap();

    cx_b.update(|cx| {
        repo_b.update(cx, |repository, _cx| {
            repository.change_branch("totally-new-branch".to_string())
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

    let host_branch = cx_a.update(|cx| {
        project_a.update(cx, |project, cx| {
            project
                .repositories(cx)
                .values()
                .next()
                .unwrap()
                .read(cx)
                .branch
                .as_ref()
                .unwrap()
                .clone()
        })
    });

    assert_eq!(host_branch.name(), "totally-new-branch");
}
