use crate::{
    rpc::{CLEANUP_TIMEOUT, RECONNECT_TIMEOUT},
    tests::{TestClient, TestServer},
};
use call::{room, ActiveCall, ParticipantLocation, Room};
use client::{User, RECEIVE_TIMEOUT};
use collections::HashSet;
use editor::{
    test::editor_test_context::EditorTestContext, ConfirmCodeAction, ConfirmCompletion,
    ConfirmRename, Editor, ExcerptRange, MultiBuffer, Redo, Rename, ToOffset, ToggleCodeActions,
    Undo,
};
use fs::{repository::GitFileStatus, FakeFs, Fs as _, LineEnding, RemoveOptions};
use futures::StreamExt as _;
use gpui::{
    executor::Deterministic, geometry::vector::vec2f, test::EmptyView, AppContext, ModelHandle,
    TestAppContext, ViewHandle,
};
use indoc::indoc;
use language::{
    language_settings::{AllLanguageSettings, Formatter},
    tree_sitter_rust, Anchor, Diagnostic, DiagnosticEntry, FakeLspAdapter, Language,
    LanguageConfig, OffsetRangeExt, Point, Rope,
};
use live_kit_client::MacOSDisplay;
use lsp::LanguageServerId;
use project::{search::SearchQuery, DiagnosticSummary, HoverBlockKind, Project, ProjectPath};
use rand::prelude::*;
use serde_json::json;
use settings::SettingsStore;
use std::{
    cell::{Cell, RefCell},
    env, future, mem,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use unindent::Unindent as _;
use workspace::{item::ItemHandle as _, shared_screen::SharedScreen, SplitDirection, Workspace};

#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test(iterations = 10)]
async fn test_basic_calls(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;

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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
    let call_b2 = incoming_call_b2.next().await.unwrap().unwrap();
    assert_eq!(call_b2.calling_user.github_login, "user_a");

    // User B joins the room using the first client.
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    assert!(incoming_call_b.next().await.unwrap().is_none());

    deterministic.run_until_parked();
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

    deterministic.run_until_parked();
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
    active_call_c.update(cx_c, |call, _| call.decline_incoming().unwrap());
    assert!(incoming_call_c.next().await.unwrap().is_none());

    deterministic.run_until_parked();
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

    deterministic.run_until_parked();
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

    deterministic.run_until_parked();
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
    let display = MacOSDisplay::new();
    let events_b = active_call_events(cx_b);
    let events_c = active_call_events(cx_c);
    active_call_a
        .update(cx_a, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_display_sources(vec![display.clone()]);
                room.share_screen(cx)
            })
        })
        .await
        .unwrap();

    deterministic.run_until_parked();

    // User B observes the remote screen sharing track.
    assert_eq!(events_b.borrow().len(), 1);
    let event_b = events_b.borrow().first().unwrap().clone();
    if let call::room::Event::RemoteVideoTracksChanged { participant_id } = event_b {
        assert_eq!(participant_id, client_a.peer_id().unwrap());
        room_b.read_with(cx_b, |room, _| {
            assert_eq!(
                room.remote_participants()[&client_a.user_id().unwrap()]
                    .tracks
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
                    .tracks
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
    deterministic.run_until_parked();
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
        .test_live_kit_server
        .disconnect_client(client_b.user_id().unwrap().to_string())
        .await;
    deterministic.run_until_parked();
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_d: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;

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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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

    deterministic.run_until_parked();

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
async fn test_room_uniqueness(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_a2: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
    deterministic.run_until_parked();
    active_call_c
        .update(cx_c, |call, cx| {
            call.invite(client_b.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    let call_b2 = incoming_call_b.next().await.unwrap().unwrap();
    assert_eq!(call_b2.calling_user.github_login, "user_c");
}

#[gpui::test(iterations = 10)]
async fn test_client_disconnecting_from_room(
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
    deterministic.run_until_parked();
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
    deterministic.advance_clock(RECEIVE_TIMEOUT);
    deterministic.run_until_parked();
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
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
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
    deterministic.advance_clock(RECEIVE_TIMEOUT);

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
    deterministic.run_until_parked();
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
        .test_live_kit_server
        .disconnect_client(client_b.user_id().unwrap().to_string())
        .await;
    deterministic.run_until_parked();
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_d: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    client_a
        .fs
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

    deterministic.run_until_parked();
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
    client_c.override_establish_connection(|_, cx| cx.spawn(|_| future::pending()));
    deterministic.advance_clock(RECONNECT_TIMEOUT);
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
    deterministic.run_until_parked();
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
    deterministic.advance_clock(CLEANUP_TIMEOUT);
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    client_a.override_establish_connection(|_, cx| cx.spawn(|_| future::pending()));
    client_b.override_establish_connection(|_, cx| cx.spawn(|_| future::pending()));
    client_c.override_establish_connection(|_, cx| cx.spawn(|_| future::pending()));
    deterministic.advance_clock(RECONNECT_TIMEOUT);
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
    deterministic.advance_clock(CLEANUP_TIMEOUT);
    assert!(incoming_call_d.next().await.unwrap().is_none());
}

#[gpui::test(iterations = 10)]
async fn test_calls_on_multiple_connections(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b1: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User B declines the call on one of the two connections, causing both connections
    // to stop ringing.
    active_call_b2.update(cx_b2, |call, _| call.decline_incoming().unwrap());
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // Call user B again from client A.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User B accepts the call on one of the two connections, causing both connections
    // to stop ringing.
    active_call_b2
        .update(cx_b2, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // User B disconnects the client that is not on the call. Everything should be fine.
    client_b1.disconnect(&cx_b1.to_async());
    deterministic.advance_clock(RECEIVE_TIMEOUT);
    client_b1
        .authenticate_and_connect(false, &cx_b1.to_async())
        .await
        .unwrap();

    // User B hangs up, and user A calls them again.
    active_call_b2
        .update(cx_b2, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User A cancels the call, causing both connections to stop ringing.
    active_call_a
        .update(cx_a, |call, cx| {
            call.cancel_invite(client_b1.user_id().unwrap(), cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // User A calls user B again.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User A hangs up, causing both connections to stop ringing.
    active_call_a
        .update(cx_a, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // User A calls user B again.
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User A disconnects, causing both connections to stop ringing.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());

    // User A reconnects automatically, then calls user B again.
    server.allow_connections();
    deterministic.advance_clock(RECEIVE_TIMEOUT);
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b1.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(incoming_call_b1.next().await.unwrap().is_some());
    assert!(incoming_call_b2.next().await.unwrap().is_some());

    // User B disconnects all clients, causing user A to no longer see a pending call for them.
    server.forbid_connections();
    server.disconnect_client(client_b1.peer_id().unwrap());
    server.disconnect_client(client_b2.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    active_call_a.read_with(cx_a, |call, _| assert!(call.room().is_none()));
}

#[gpui::test(iterations = 10)]
async fn test_share_project(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let (window_b, _) = cx_b.add_window(|_| EmptyView);
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                ".gitignore": "ignored-dir",
                "a.txt": "a-contents",
                "b.txt": "b-contents",
                "ignored-dir": {
                    "c.txt": "",
                    "d.txt": "",
                }
            }),
        )
        .await;

    // Invite client B to collaborate on a project
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), Some(project_a.clone()), cx)
        })
        .await
        .unwrap();

    // Join that project as client B
    let incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
    deterministic.run_until_parked();
    let call = incoming_call_b.borrow().clone().unwrap();
    assert_eq!(call.calling_user.github_login, "user_a");
    let initial_project = call.initial_project.unwrap();
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    let client_b_peer_id = client_b.peer_id().unwrap();
    let project_b = client_b
        .build_remote_project(initial_project.id, cx_b)
        .await;
    let replica_id_b = project_b.read_with(cx_b, |project, _| project.replica_id());

    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| {
        let client_b_collaborator = project.collaborators().get(&client_b_peer_id).unwrap();
        assert_eq!(client_b_collaborator.replica_id, replica_id_b);
    });
    project_b.read_with(cx_b, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        assert_eq!(
            worktree.paths().map(AsRef::as_ref).collect::<Vec<_>>(),
            [
                Path::new(".gitignore"),
                Path::new("a.txt"),
                Path::new("b.txt"),
                Path::new("ignored-dir"),
                Path::new("ignored-dir/c.txt"),
                Path::new("ignored-dir/d.txt"),
            ]
        );
    });

    // Open the same file as client B and client A.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "b.txt"), cx))
        .await
        .unwrap();
    buffer_b.read_with(cx_b, |buf, _| assert_eq!(buf.text(), "b-contents"));
    project_a.read_with(cx_a, |project, cx| {
        assert!(project.has_open_buffer((worktree_id, "b.txt"), cx))
    });
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "b.txt"), cx))
        .await
        .unwrap();

    let editor_b = cx_b.add_view(window_b, |cx| Editor::for_buffer(buffer_b, None, cx));

    // Client A sees client B's selection
    deterministic.run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| {
        buffer
            .snapshot()
            .remote_selections_in_range(Anchor::MIN..Anchor::MAX)
            .count()
            == 1
    });

    // Edit the buffer as client B and see that edit as client A.
    editor_b.update(cx_b, |editor, cx| editor.handle_input("ok, ", cx));
    deterministic.run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "ok, b-contents")
    });

    // Client B can invite client C on a project shared by client A.
    active_call_b
        .update(cx_b, |call, cx| {
            call.invite(client_c.user_id().unwrap(), Some(project_b.clone()), cx)
        })
        .await
        .unwrap();

    let incoming_call_c = active_call_c.read_with(cx_c, |call, _| call.incoming());
    deterministic.run_until_parked();
    let call = incoming_call_c.borrow().clone().unwrap();
    assert_eq!(call.calling_user.github_login, "user_b");
    let initial_project = call.initial_project.unwrap();
    active_call_c
        .update(cx_c, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    let _project_c = client_c
        .build_remote_project(initial_project.id, cx_c)
        .await;

    // Client B closes the editor, and client A sees client B's selections removed.
    cx_b.update(move |_| drop(editor_b));
    deterministic.run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| {
        buffer
            .snapshot()
            .remote_selections_in_range(Anchor::MIN..Anchor::MAX)
            .count()
            == 0
    });
}

#[gpui::test(iterations = 10)]
async fn test_unshare_project(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    client_a
        .fs
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
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    deterministic.run_until_parked();
    assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

    project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // When client B leaves the room, the project becomes read-only.
    active_call_b
        .update(cx_b, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert!(project_b.read_with(cx_b, |project, _| project.is_read_only()));

    // Client C opens the project.
    let project_c = client_c.build_remote_project(project_id, cx_c).await;

    // When client A unshares the project, client C's project becomes read-only.
    project_a
        .update(cx_a, |project, cx| project.unshare(cx))
        .unwrap();
    deterministic.run_until_parked();
    assert!(worktree_a.read_with(cx_a, |tree, _| !tree.as_local().unwrap().is_shared()));
    assert!(project_c.read_with(cx_c, |project, _| project.is_read_only()));

    // Client C can open the project again after client A re-shares.
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_c2 = client_c.build_remote_project(project_id, cx_c).await;
    deterministic.run_until_parked();
    assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));
    project_c2
        .update(cx_c, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // When client A (the host) leaves the room, the project gets unshared and guests are notified.
    active_call_a
        .update(cx_a, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));
    project_c2.read_with(cx_c, |project, _| {
        assert!(project.is_read_only());
        assert!(project.collaborators().is_empty());
    });
}

#[gpui::test(iterations = 10)]
async fn test_host_disconnect(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    cx_b.update(editor::init);

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let worktree_a = project_a.read_with(cx_a, |project, cx| project.worktrees(cx).next().unwrap());
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    deterministic.run_until_parked();
    assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

    let (window_id_b, workspace_b) =
        cx_b.add_window(|cx| Workspace::test_new(project_b.clone(), cx));
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "b.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    assert!(cx_b
        .read_window(window_id_b, |cx| editor_b.is_focused(cx))
        .unwrap());
    editor_b.update(cx_b, |editor, cx| editor.insert("X", cx));
    assert!(cx_b.is_window_edited(workspace_b.window_id()));

    // Drop client A's connection. Collaborators should disappear and the project should not be shown as shared.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    project_a.read_with(cx_a, |project, _| project.collaborators().is_empty());
    project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));
    project_b.read_with(cx_b, |project, _| project.is_read_only());
    assert!(worktree_a.read_with(cx_a, |tree, _| !tree.as_local().unwrap().is_shared()));

    // Ensure client B's edited state is reset and that the whole window is blurred.
    cx_b.read_window(window_id_b, |cx| {
        assert_eq!(cx.focused_view_id(), None);
    });
    assert!(!cx_b.is_window_edited(workspace_b.window_id()));

    // Ensure client B is not prompted to save edits when closing window after disconnecting.
    let can_close = workspace_b
        .update(cx_b, |workspace, cx| workspace.prepare_to_close(true, cx))
        .await
        .unwrap();
    assert!(can_close);

    // Allow client A to reconnect to the server.
    server.allow_connections();
    deterministic.advance_clock(RECEIVE_TIMEOUT);

    // Client B calls client A again after they reconnected.
    let active_call_b = cx_b.read(ActiveCall::global);
    active_call_b
        .update(cx_b, |call, cx| {
            call.invite(client_a.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    active_call_a
        .update(cx_a, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Drop client A's connection again. We should still unshare it successfully.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));
}

#[gpui::test(iterations = 10)]
async fn test_project_reconnect(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    cx_b.update(editor::init);

    client_a
        .fs
        .insert_tree(
            "/root-1",
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
        .fs
        .insert_tree(
            "/root-2",
            json!({
                "2.txt": "2",
            }),
        )
        .await;
    client_a
        .fs
        .insert_tree(
            "/root-3",
            json!({
                "3.txt": "3",
            }),
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let (project_a1, _) = client_a.build_local_project("/root-1/dir1", cx_a).await;
    let (project_a2, _) = client_a.build_local_project("/root-2", cx_a).await;
    let (project_a3, _) = client_a.build_local_project("/root-3", cx_a).await;
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

    let project_b1 = client_b.build_remote_project(project1_id, cx_b).await;
    let project_b2 = client_b.build_remote_project(project2_id, cx_b).await;
    let project_b3 = client_b.build_remote_project(project3_id, cx_b).await;
    deterministic.run_until_parked();

    let worktree1_id = worktree_a1.read_with(cx_a, |worktree, _| {
        assert!(worktree.as_local().unwrap().is_shared());
        worktree.id()
    });
    let (worktree_a2, _) = project_a1
        .update(cx_a, |p, cx| {
            p.find_or_create_local_worktree("/root-1/dir2", true, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    let worktree2_id = worktree_a2.read_with(cx_a, |tree, _| {
        assert!(tree.as_local().unwrap().is_shared());
        tree.id()
    });
    deterministic.run_until_parked();
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
    deterministic.advance_clock(RECEIVE_TIMEOUT);
    project_a1.read_with(cx_a, |project, _| {
        assert!(project.is_shared());
        assert_eq!(project.collaborators().len(), 1);
    });
    project_b1.read_with(cx_b, |project, _| {
        assert!(!project.is_read_only());
        assert_eq!(project.collaborators().len(), 1);
    });
    worktree_a1.read_with(cx_a, |tree, _| {
        assert!(tree.as_local().unwrap().is_shared())
    });

    // While client A is disconnected, add and remove files from client A's project.
    client_a
        .fs
        .insert_tree(
            "/root-1/dir1/subdir2",
            json!({
                "f.txt": "f-contents",
                "g.txt": "g-contents",
                "h.txt": "h-contents",
                "i.txt": "i-contents",
            }),
        )
        .await;
    client_a
        .fs
        .remove_dir(
            "/root-1/dir1/subdir1".as_ref(),
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
            p.find_or_create_local_worktree("/root-1/dir3", true, cx)
        })
        .await
        .unwrap();
    worktree_a3
        .read_with(cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;
    let worktree3_id = worktree_a3.read_with(cx_a, |tree, _| {
        assert!(!tree.as_local().unwrap().is_shared());
        tree.id()
    });
    deterministic.run_until_parked();

    // While client A is disconnected, close project 2
    cx_a.update(|_| drop(project_a2));

    // While client A is disconnected, mutate a buffer on both the host and the guest.
    buffer_a1.update(cx_a, |buf, cx| buf.edit([(0..0, "W")], None, cx));
    buffer_b1.update(cx_b, |buf, cx| buf.edit([(1..1, "Z")], None, cx));
    deterministic.run_until_parked();

    // Client A reconnects. Their project is re-shared, and client B re-joins it.
    server.allow_connections();
    client_a
        .authenticate_and_connect(false, &cx_a.to_async())
        .await
        .unwrap();
    deterministic.run_until_parked();
    project_a1.read_with(cx_a, |project, cx| {
        assert!(project.is_shared());
        assert!(worktree_a1.read(cx).as_local().unwrap().is_shared());
        assert_eq!(
            worktree_a1
                .read(cx)
                .snapshot()
                .paths()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            vec![
                "a.txt",
                "b.txt",
                "subdir2",
                "subdir2/f.txt",
                "subdir2/g.txt",
                "subdir2/h.txt",
                "subdir2/i.txt"
            ]
        );
        assert!(worktree_a3.read(cx).as_local().unwrap().is_shared());
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
        assert!(!project.is_read_only());
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
                "a.txt",
                "b.txt",
                "subdir2",
                "subdir2/f.txt",
                "subdir2/g.txt",
                "subdir2/h.txt",
                "subdir2/i.txt"
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
    project_b2.read_with(cx_b, |project, _| assert!(project.is_read_only()));
    project_b3.read_with(cx_b, |project, _| assert!(!project.is_read_only()));
    buffer_a1.read_with(cx_a, |buffer, _| assert_eq!(buffer.text(), "WaZ"));
    buffer_b1.read_with(cx_b, |buffer, _| assert_eq!(buffer.text(), "WaZ"));

    // Drop client B's connection.
    server.forbid_connections();
    server.disconnect_client(client_b.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT);

    // While client B is disconnected, add and remove files from client A's project
    client_a
        .fs
        .insert_file("/root-1/dir1/subdir2/j.txt", "j-contents".into())
        .await;
    client_a
        .fs
        .remove_file("/root-1/dir1/subdir2/i.txt".as_ref(), Default::default())
        .await
        .unwrap();

    // While client B is disconnected, add and remove worktrees from client A's project.
    let (worktree_a4, _) = project_a1
        .update(cx_a, |p, cx| {
            p.find_or_create_local_worktree("/root-1/dir4", true, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    let worktree4_id = worktree_a4.read_with(cx_a, |tree, _| {
        assert!(tree.as_local().unwrap().is_shared());
        tree.id()
    });
    project_a1.update(cx_a, |project, cx| {
        project.remove_worktree(worktree3_id, cx)
    });
    deterministic.run_until_parked();

    // While client B is disconnected, mutate a buffer on both the host and the guest.
    buffer_a1.update(cx_a, |buf, cx| buf.edit([(1..1, "X")], None, cx));
    buffer_b1.update(cx_b, |buf, cx| buf.edit([(2..2, "Y")], None, cx));
    deterministic.run_until_parked();

    // While disconnected, close project 3
    cx_a.update(|_| drop(project_a3));

    // Client B reconnects. They re-join the room and the remaining shared project.
    server.allow_connections();
    client_b
        .authenticate_and_connect(false, &cx_b.to_async())
        .await
        .unwrap();
    deterministic.run_until_parked();
    project_b1.read_with(cx_b, |project, cx| {
        assert!(!project.is_read_only());
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
                "a.txt",
                "b.txt",
                "subdir2",
                "subdir2/f.txt",
                "subdir2/g.txt",
                "subdir2/h.txt",
                "subdir2/j.txt"
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
    project_b3.read_with(cx_b, |project, _| assert!(project.is_read_only()));
    buffer_a1.read_with(cx_a, |buffer, _| assert_eq!(buffer.text(), "WXaYZ"));
    buffer_b1.read_with(cx_b, |buffer, _| assert_eq!(buffer.text(), "WXaYZ"));
}

#[gpui::test(iterations = 10)]
async fn test_active_call_events(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    client_a.fs.insert_tree("/a", json!({})).await;
    client_b.fs.insert_tree("/b", json!({})).await;

    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    let (project_b, _) = client_b.build_local_project("/b", cx_b).await;

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let events_a = active_call_events(cx_a);
    let events_b = active_call_events(cx_b);

    let project_a_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert_eq!(mem::take(&mut *events_a.borrow_mut()), vec![]);
    assert_eq!(
        mem::take(&mut *events_b.borrow_mut()),
        vec![room::Event::RemoteProjectShared {
            owner: Arc::new(User {
                id: client_a.user_id().unwrap(),
                github_login: "user_a".to_string(),
                avatar: None,
            }),
            project_id: project_a_id,
            worktree_root_names: vec!["a".to_string()],
        }]
    );

    let project_b_id = active_call_b
        .update(cx_b, |call, cx| call.share_project(project_b.clone(), cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert_eq!(
        mem::take(&mut *events_a.borrow_mut()),
        vec![room::Event::RemoteProjectShared {
            owner: Arc::new(User {
                id: client_b.user_id().unwrap(),
                github_login: "user_b".to_string(),
                avatar: None,
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
    deterministic.run_until_parked();
    assert_eq!(mem::take(&mut *events_a.borrow_mut()), vec![]);
    assert_eq!(mem::take(&mut *events_b.borrow_mut()), vec![]);
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

#[gpui::test(iterations = 10)]
async fn test_room_location(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    client_a.fs.insert_tree("/a", json!({})).await;
    client_b.fs.insert_tree("/b", json!({})).await;

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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
        room: &ModelHandle<Room>,
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let rust = Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));
    let javascript = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            path_suffixes: vec!["js".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));
    for client in [&client_a, &client_b, &client_c] {
        client.language_registry.add(rust.clone());
        client.language_registry.add(javascript.clone());
    }

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "file1.rs": "",
                "file2": ""
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let worktree_a = project_a.read_with(cx_a, |p, cx| p.worktrees(cx).next().unwrap());
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Join that worktree as clients B and C.
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    let project_c = client_c.build_remote_project(project_id, cx_c).await;
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
        assert_eq!(&*buffer.language().unwrap().name(), "Rust");
    });
    buffer_c.read_with(cx_c, |buffer, _| {
        assert_eq!(&*buffer.language().unwrap().name(), "Rust");
    });
    buffer_b.update(cx_b, |buf, cx| buf.edit([(0..0, "i-am-b, ")], None, cx));
    buffer_c.update(cx_c, |buf, cx| buf.edit([(0..0, "i-am-c, ")], None, cx));

    // Open and edit that buffer as the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "file1.rs"), cx))
        .await
        .unwrap();

    deterministic.run_until_parked();
    buffer_a.read_with(cx_a, |buf, _| assert_eq!(buf.text(), "i-am-c, i-am-b, "));
    buffer_a.update(cx_a, |buf, cx| {
        buf.edit([(buf.len()..buf.len(), "i-am-a")], None, cx)
    });

    deterministic.run_until_parked();
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
        client_a.fs.load("/a/file1.rs".as_ref()).await.unwrap(),
        "hi-a, i-am-c, i-am-b, i-am-a"
    );

    deterministic.run_until_parked();
    buffer_a.read_with(cx_a, |buf, _| assert!(!buf.is_dirty()));
    buffer_b.read_with(cx_b, |buf, _| assert!(!buf.is_dirty()));
    buffer_c.read_with(cx_c, |buf, _| assert!(!buf.is_dirty()));

    // Make changes on host's file system, see those changes on guest worktrees.
    client_a
        .fs
        .rename(
            "/a/file1.rs".as_ref(),
            "/a/file1.js".as_ref(),
            Default::default(),
        )
        .await
        .unwrap();
    client_a
        .fs
        .rename("/a/file2".as_ref(), "/a/file3".as_ref(), Default::default())
        .await
        .unwrap();
    client_a.fs.insert_file("/a/file4", "4".into()).await;
    deterministic.run_until_parked();

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
        assert_eq!(&*buffer.language().unwrap().name(), "JavaScript");
    });
    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.file().unwrap().path().to_str(), Some("file1.js"));
        assert_eq!(&*buffer.language().unwrap().name(), "JavaScript");
    });
    buffer_c.read_with(cx_c, |buffer, _| {
        assert_eq!(buffer.file().unwrap().path().to_str(), Some("file1.js"));
        assert_eq!(&*buffer.language().unwrap().name(), "JavaScript");
    });

    let new_buffer_a = project_a
        .update(cx_a, |p, cx| p.create_buffer("", None, cx))
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
            project.save_buffer_as(new_buffer_a.clone(), "/a/file3.rs".into(), cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
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

    let project_remote = client_b.build_remote_project(project_id, cx_b).await;

    let diff_base = "
        one
        three
    "
    .unindent();

    let new_diff_base = "
        one
        two
    "
    .unindent();

    client_a.fs.as_fake().set_index_for_repo(
        Path::new("/dir/.git"),
        &[(Path::new("a.txt"), diff_base.clone())],
    );

    // Create the buffer
    let buffer_local_a = project_local
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // Wait for it to catch up to the new diff
    deterministic.run_until_parked();

    // Smoke test diffing
    buffer_local_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_row_range(0..4),
            &buffer,
            &diff_base,
            &[(1..2, "", "two\n")],
        );
    });

    // Create remote buffer
    let buffer_remote_a = project_remote
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // Wait remote buffer to catch up to the new diff
    deterministic.run_until_parked();

    // Smoke test diffing
    buffer_remote_a.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_row_range(0..4),
            &buffer,
            &diff_base,
            &[(1..2, "", "two\n")],
        );
    });

    client_a.fs.as_fake().set_index_for_repo(
        Path::new("/dir/.git"),
        &[(Path::new("a.txt"), new_diff_base.clone())],
    );

    // Wait for buffer_local_a to receive it
    deterministic.run_until_parked();

    // Smoke test new diffing
    buffer_local_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(new_diff_base.as_ref()));

        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_row_range(0..4),
            &buffer,
            &diff_base,
            &[(2..3, "", "three\n")],
        );
    });

    // Smoke test B
    buffer_remote_a.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(new_diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_row_range(0..4),
            &buffer,
            &diff_base,
            &[(2..3, "", "three\n")],
        );
    });

    //Nested git dir

    let diff_base = "
        one
        three
    "
    .unindent();

    let new_diff_base = "
        one
        two
    "
    .unindent();

    client_a.fs.as_fake().set_index_for_repo(
        Path::new("/dir/sub/.git"),
        &[(Path::new("b.txt"), diff_base.clone())],
    );

    // Create the buffer
    let buffer_local_b = project_local
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "sub/b.txt"), cx))
        .await
        .unwrap();

    // Wait for it to catch up to the new diff
    deterministic.run_until_parked();

    // Smoke test diffing
    buffer_local_b.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_row_range(0..4),
            &buffer,
            &diff_base,
            &[(1..2, "", "two\n")],
        );
    });

    // Create remote buffer
    let buffer_remote_b = project_remote
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "sub/b.txt"), cx))
        .await
        .unwrap();

    // Wait remote buffer to catch up to the new diff
    deterministic.run_until_parked();

    // Smoke test diffing
    buffer_remote_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_row_range(0..4),
            &buffer,
            &diff_base,
            &[(1..2, "", "two\n")],
        );
    });

    client_a.fs.as_fake().set_index_for_repo(
        Path::new("/dir/sub/.git"),
        &[(Path::new("b.txt"), new_diff_base.clone())],
    );

    // Wait for buffer_local_b to receive it
    deterministic.run_until_parked();

    // Smoke test new diffing
    buffer_local_b.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(new_diff_base.as_ref()));
        println!("{:?}", buffer.as_rope().to_string());
        println!("{:?}", buffer.diff_base());
        println!(
            "{:?}",
            buffer
                .snapshot()
                .git_diff_hunks_in_row_range(0..4)
                .collect::<Vec<_>>()
        );

        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_row_range(0..4),
            &buffer,
            &diff_base,
            &[(2..3, "", "three\n")],
        );
    });

    // Smoke test B
    buffer_remote_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(new_diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_row_range(0..4),
            &buffer,
            &diff_base,
            &[(2..3, "", "three\n")],
        );
    });
}

#[gpui::test]
async fn test_git_branch_name(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
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

    let project_remote = client_b.build_remote_project(project_id, cx_b).await;
    client_a
        .fs
        .as_fake()
        .set_branch_name(Path::new("/dir/.git"), Some("branch-1"));

    // Wait for it to catch up to the new branch
    deterministic.run_until_parked();

    #[track_caller]
    fn assert_branch(branch_name: Option<impl Into<String>>, project: &Project, cx: &AppContext) {
        let branch_name = branch_name.map(Into::into);
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1);
        let worktree = worktrees[0].clone();
        let root_entry = worktree.read(cx).snapshot().root_git_entry().unwrap();
        assert_eq!(root_entry.branch(), branch_name.map(Into::into));
    }

    // Smoke test branch reading
    project_local.read_with(cx_a, |project, cx| {
        assert_branch(Some("branch-1"), project, cx)
    });
    project_remote.read_with(cx_b, |project, cx| {
        assert_branch(Some("branch-1"), project, cx)
    });

    client_a
        .fs
        .as_fake()
        .set_branch_name(Path::new("/dir/.git"), Some("branch-2"));

    // Wait for buffer_local_a to receive it
    deterministic.run_until_parked();

    // Smoke test branch reading
    project_local.read_with(cx_a, |project, cx| {
        assert_branch(Some("branch-2"), project, cx)
    });
    project_remote.read_with(cx_b, |project, cx| {
        assert_branch(Some("branch-2"), project, cx)
    });

    let project_remote_c = client_c.build_remote_project(project_id, cx_c).await;
    deterministic.run_until_parked();
    project_remote_c.read_with(cx_c, |project, cx| {
        assert_branch(Some("branch-2"), project, cx)
    });
}

#[gpui::test]
async fn test_git_status_sync(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree(
            "/dir",
            json!({
            ".git": {},
            "a.txt": "a",
            "b.txt": "b",
            }),
        )
        .await;

    const A_TXT: &'static str = "a.txt";
    const B_TXT: &'static str = "b.txt";

    client_a.fs.as_fake().set_status_for_repo_via_git_operation(
        Path::new("/dir/.git"),
        &[
            (&Path::new(A_TXT), GitFileStatus::Added),
            (&Path::new(B_TXT), GitFileStatus::Added),
        ],
    );

    let (project_local, _worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| {
            call.share_project(project_local.clone(), cx)
        })
        .await
        .unwrap();

    let project_remote = client_b.build_remote_project(project_id, cx_b).await;

    // Wait for it to catch up to the new status
    deterministic.run_until_parked();

    #[track_caller]
    fn assert_status(
        file: &impl AsRef<Path>,
        status: Option<GitFileStatus>,
        project: &Project,
        cx: &AppContext,
    ) {
        let file = file.as_ref();
        let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1);
        let worktree = worktrees[0].clone();
        let snapshot = worktree.read(cx).snapshot();
        assert_eq!(snapshot.status_for_file(file), status);
    }

    // Smoke test status reading
    project_local.read_with(cx_a, |project, cx| {
        assert_status(&Path::new(A_TXT), Some(GitFileStatus::Added), project, cx);
        assert_status(&Path::new(B_TXT), Some(GitFileStatus::Added), project, cx);
    });
    project_remote.read_with(cx_b, |project, cx| {
        assert_status(&Path::new(A_TXT), Some(GitFileStatus::Added), project, cx);
        assert_status(&Path::new(B_TXT), Some(GitFileStatus::Added), project, cx);
    });

    client_a
        .fs
        .as_fake()
        .set_status_for_repo_via_working_copy_change(
            Path::new("/dir/.git"),
            &[
                (&Path::new(A_TXT), GitFileStatus::Modified),
                (&Path::new(B_TXT), GitFileStatus::Modified),
            ],
        );

    // Wait for buffer_local_a to receive it
    deterministic.run_until_parked();

    // Smoke test status reading
    project_local.read_with(cx_a, |project, cx| {
        assert_status(
            &Path::new(A_TXT),
            Some(GitFileStatus::Modified),
            project,
            cx,
        );
        assert_status(
            &Path::new(B_TXT),
            Some(GitFileStatus::Modified),
            project,
            cx,
        );
    });
    project_remote.read_with(cx_b, |project, cx| {
        assert_status(
            &Path::new(A_TXT),
            Some(GitFileStatus::Modified),
            project,
            cx,
        );
        assert_status(
            &Path::new(B_TXT),
            Some(GitFileStatus::Modified),
            project,
            cx,
        );
    });

    // And synchronization while joining
    let project_remote_c = client_c.build_remote_project(project_id, cx_c).await;
    deterministic.run_until_parked();

    project_remote_c.read_with(cx_c, |project, cx| {
        assert_status(
            &Path::new(A_TXT),
            Some(GitFileStatus::Modified),
            project,
            cx,
        );
        assert_status(
            &Path::new(B_TXT),
            Some(GitFileStatus::Modified),
            project,
            cx,
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_fs_operations(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree(
            "/dir",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    let worktree_a = project_a.read_with(cx_a, |project, cx| project.worktrees(cx).next().unwrap());
    let worktree_b = project_b.read_with(cx_b, |project, cx| project.worktrees(cx).next().unwrap());

    let entry = project_b
        .update(cx_b, |project, cx| {
            project
                .create_entry((worktree_id, "c.txt"), false, cx)
                .unwrap()
        })
        .await
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
        .unwrap()
        .await
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
            project
                .create_entry((worktree_id, "DIR"), true, cx)
                .unwrap()
        })
        .await
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
            project
                .create_entry((worktree_id, "DIR/e.txt"), false, cx)
                .unwrap()
        })
        .await
        .unwrap();
    project_b
        .update(cx_b, |project, cx| {
            project
                .create_entry((worktree_id, "DIR/SUBDIR"), true, cx)
                .unwrap()
        })
        .await
        .unwrap();
    project_b
        .update(cx_b, |project, cx| {
            project
                .create_entry((worktree_id, "DIR/SUBDIR/f.txt"), false, cx)
                .unwrap()
        })
        .await
        .unwrap();
    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            [
                "DIR",
                "DIR/SUBDIR",
                "DIR/SUBDIR/f.txt",
                "DIR/e.txt",
                "a.txt",
                "b.txt",
                "d.txt"
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
                "DIR",
                "DIR/SUBDIR",
                "DIR/SUBDIR/f.txt",
                "DIR/e.txt",
                "a.txt",
                "b.txt",
                "d.txt"
            ]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            project
                .copy_entry(entry.id, Path::new("f.txt"), cx)
                .unwrap()
        })
        .await
        .unwrap();
    worktree_a.read_with(cx_a, |worktree, _| {
        assert_eq!(
            worktree
                .paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>(),
            [
                "DIR",
                "DIR/SUBDIR",
                "DIR/SUBDIR/f.txt",
                "DIR/e.txt",
                "a.txt",
                "b.txt",
                "d.txt",
                "f.txt"
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
                "DIR",
                "DIR/SUBDIR",
                "DIR/SUBDIR/f.txt",
                "DIR/e.txt",
                "a.txt",
                "b.txt",
                "d.txt",
                "f.txt"
            ]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            project.delete_entry(dir_entry.id, cx).unwrap()
        })
        .await
        .unwrap();
    deterministic.run_until_parked();

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
            project.delete_entry(entry.id, cx).unwrap()
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // As client A, open a project that contains some local settings files
    client_a
        .fs
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
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // As client B, join that project and observe the local settings.
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    let worktree_b = project_b.read_with(cx_b, |project, cx| project.worktrees(cx).next().unwrap());
    deterministic.run_until_parked();
    cx_b.read(|cx| {
        let store = cx.global::<SettingsStore>();
        assert_eq!(
            store.local_settings(worktree_b.id()).collect::<Vec<_>>(),
            &[
                (Path::new("").into(), r#"{"tab_size":2}"#.to_string()),
                (Path::new("a").into(), r#"{"tab_size":8}"#.to_string()),
            ]
        )
    });

    // As client A, update a settings file. As Client B, see the changed settings.
    client_a
        .fs
        .insert_file("/dir/.zed/settings.json", r#"{}"#.into())
        .await;
    deterministic.run_until_parked();
    cx_b.read(|cx| {
        let store = cx.global::<SettingsStore>();
        assert_eq!(
            store.local_settings(worktree_b.id()).collect::<Vec<_>>(),
            &[
                (Path::new("").into(), r#"{}"#.to_string()),
                (Path::new("a").into(), r#"{"tab_size":8}"#.to_string()),
            ]
        )
    });

    // As client A, create and remove some settings files. As client B, see the changed settings.
    client_a
        .fs
        .remove_file("/dir/.zed/settings.json".as_ref(), Default::default())
        .await
        .unwrap();
    client_a
        .fs
        .create_dir("/dir/b/.zed".as_ref())
        .await
        .unwrap();
    client_a
        .fs
        .insert_file("/dir/b/.zed/settings.json", r#"{"tab_size": 4}"#.into())
        .await;
    deterministic.run_until_parked();
    cx_b.read(|cx| {
        let store = cx.global::<SettingsStore>();
        assert_eq!(
            store.local_settings(worktree_b.id()).collect::<Vec<_>>(),
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
        .fs
        .insert_file("/dir/a/.zed/settings.json", r#"{"hard_tabs":true}"#.into())
        .await;
    client_a
        .fs
        .remove_file("/dir/b/.zed/settings.json".as_ref(), Default::default())
        .await
        .unwrap();
    deterministic.run_until_parked();

    // As client B, reconnect and see the changed settings.
    server.allow_connections();
    deterministic.advance_clock(RECEIVE_TIMEOUT);
    cx_b.read(|cx| {
        let store = cx.global::<SettingsStore>();
        assert_eq!(
            store.local_settings(worktree_b.id()).collect::<Vec<_>>(),
            &[(Path::new("a").into(), r#"{"hard_tabs":true}"#.to_string()),]
        )
    });
}

#[gpui::test(iterations = 10)]
async fn test_buffer_conflict_after_save(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree(
            "/dir",
            json!({
                "a.txt": "a-contents",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

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
    cx_a.foreground().forbid_parking();
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree(
            "/dir",
            json!({
                "a.txt": "a\nb\nc",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

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
        .fs
        .save("/dir/a.txt".as_ref(), &new_contents, LineEnding::Windows)
        .await
        .unwrap();
    cx_a.foreground().run_until_parked();
    buffer_b.read_with(cx_b, |buf, _| {
        assert_eq!(buf.text(), new_contents.to_string());
        assert!(!buf.is_dirty());
        assert!(!buf.has_conflict());
        assert_eq!(buf.line_ending(), LineEnding::Windows);
    });
}

#[gpui::test(iterations = 10)]
async fn test_editing_while_guest_opens_buffer(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree("/dir", json!({ "a.txt": "a-contents" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open a buffer as client A
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // Start opening the same buffer as client B
    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx)));

    // Edit the buffer as client A while client B is still opening it.
    cx_b.background().simulate_random_delay().await;
    buffer_a.update(cx_a, |buf, cx| buf.edit([(0..0, "X")], None, cx));
    cx_b.background().simulate_random_delay().await;
    buffer_a.update(cx_a, |buf, cx| buf.edit([(1..1, "Y")], None, cx));

    let text = buffer_a.read_with(cx_a, |buf, _| buf.text());
    let buffer_b = buffer_b.await.unwrap();
    cx_a.foreground().run_until_parked();
    buffer_b.read_with(cx_b, |buf, _| assert_eq!(buf.text(), text));
}

#[gpui::test]
async fn test_newline_above_or_below_does_not_move_guest_cursor(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree("/dir", json!({ "a.txt": "Some text\n" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open a buffer as client A
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();
    let (window_a, _) = cx_a.add_window(|_| EmptyView);
    let editor_a = cx_a.add_view(window_a, |cx| {
        Editor::for_buffer(buffer_a, Some(project_a), cx)
    });
    let mut editor_cx_a = EditorTestContext {
        cx: cx_a,
        window_id: window_a,
        editor: editor_a,
    };

    // Open a buffer as client B
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();
    let (window_b, _) = cx_b.add_window(|_| EmptyView);
    let editor_b = cx_b.add_view(window_b, |cx| {
        Editor::for_buffer(buffer_b, Some(project_b), cx)
    });
    let mut editor_cx_b = EditorTestContext {
        cx: cx_b,
        window_id: window_b,
        editor: editor_b,
    };

    // Test newline above
    editor_cx_a.set_selections_state(indoc! {"
        Some textˇ
    "});
    editor_cx_b.set_selections_state(indoc! {"
        Some textˇ
    "});
    editor_cx_a.update_editor(|editor, cx| editor.newline_above(&editor::NewlineAbove, cx));
    deterministic.run_until_parked();
    editor_cx_a.assert_editor_state(indoc! {"
        ˇ
        Some text
    "});
    editor_cx_b.assert_editor_state(indoc! {"

        Some textˇ
    "});

    // Test newline below
    editor_cx_a.set_selections_state(indoc! {"

        Some textˇ
    "});
    editor_cx_b.set_selections_state(indoc! {"

        Some textˇ
    "});
    editor_cx_a.update_editor(|editor, cx| editor.newline_below(&editor::NewlineBelow, cx));
    deterministic.run_until_parked();
    editor_cx_a.assert_editor_state(indoc! {"

        Some text
        ˇ
    "});
    editor_cx_b.assert_editor_state(indoc! {"

        Some textˇ

    "});
}

#[gpui::test(iterations = 10)]
async fn test_leaving_worktree_while_opening_buffer(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree("/dir", json!({ "a.txt": "a-contents" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // See that a guest has joined as client A.
    cx_a.foreground().run_until_parked();
    project_a.read_with(cx_a, |p, _| assert_eq!(p.collaborators().len(), 1));

    // Begin opening a buffer as client B, but leave the project before the open completes.
    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx)));
    cx_b.update(|_| drop(project_b));
    drop(buffer_b);

    // See that the guest has left.
    cx_a.foreground().run_until_parked();
    project_a.read_with(cx_a, |p, _| assert!(p.collaborators().is_empty()));
}

#[gpui::test(iterations = 10)]
async fn test_canceling_buffer_opening(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();

    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
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
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // Open a buffer as client B but cancel after a random amount of time.
    let buffer_b = project_b.update(cx_b, |p, cx| {
        p.open_buffer_by_id(buffer_a.read_with(cx_a, |a, _| a.remote_id()), cx)
    });
    deterministic.simulate_random_delay().await;
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
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
    let project_b1 = client_b.build_remote_project(project_id, cx_b).await;
    let project_c = client_c.build_remote_project(project_id, cx_c).await;

    // Client A sees that a guest has joined.
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });
    project_c.read_with(cx_c, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });

    // Client B re-joins the project and can open buffers as before.
    let project_b2 = client_b.build_remote_project(project_id, cx_b).await;
    deterministic.run_until_parked();
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

    // Drop client B's connection and ensure client A and client C observe client B leaving.
    client_b.disconnect(&cx_b.to_async());
    deterministic.advance_clock(RECONNECT_TIMEOUT);
    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });
    project_b2.read_with(cx_b, |project, _| {
        assert!(project.is_read_only());
    });
    project_c.read_with(cx_c, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });

    // Client B can't join the project, unless they re-join the room.
    cx_b.spawn(|cx| {
        Project::remote(
            project_id,
            client_b.client.clone(),
            client_b.user_store.clone(),
            client_b.language_registry.clone(),
            FakeFs::new(cx.background()),
            cx,
        )
    })
    .await
    .unwrap_err();

    // Simulate connection loss for client C and ensure client A observes client C leaving the project.
    client_c.wait_for_current_user(cx_c).await;
    server.forbid_connections();
    server.disconnect_client(client_c.peer_id().unwrap());
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 0);
    });
    project_b2.read_with(cx_b, |project, _| {
        assert!(project.is_read_only());
    });
    project_c.read_with(cx_c, |project, _| {
        assert!(project.is_read_only());
    });
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_diagnostics(
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
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    // Share a project as client A
    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "a.rs": "let one = two",
                "other.rs": "",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;

    // Cause the language server to start.
    let _buffer = project_a
        .update(cx_a, |project, cx| {
            project.open_buffer(
                ProjectPath {
                    worktree_id,
                    path: Path::new("other.rs").into(),
                },
                cx,
            )
        })
        .await
        .unwrap();

    // Simulate a language server reporting errors for a file.
    let mut fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;
    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        lsp::PublishDiagnosticsParams {
            uri: lsp::Url::from_file_path("/a/a.rs").unwrap(),
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
        lsp::PublishDiagnosticsParams {
            uri: lsp::Url::from_file_path("/a/a.rs").unwrap(),
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
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Wait for server to see the diagnostics update.
    deterministic.run_until_parked();

    // Ensure client B observes the new diagnostics.
    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(cx).collect::<Vec<_>>(),
            &[(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("a.rs")),
                },
                LanguageServerId(0),
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 0,
                    ..Default::default()
                },
            )]
        )
    });

    // Join project as client C and observe the diagnostics.
    let project_c = client_c.build_remote_project(project_id, cx_c).await;
    let project_c_diagnostic_summaries =
        Rc::new(RefCell::new(project_c.read_with(cx_c, |project, cx| {
            project.diagnostic_summaries(cx).collect::<Vec<_>>()
        })));
    project_c.update(cx_c, |_, cx| {
        let summaries = project_c_diagnostic_summaries.clone();
        cx.subscribe(&project_c, {
            move |p, _, event, cx| {
                if let project::Event::DiskBasedDiagnosticsFinished { .. } = event {
                    *summaries.borrow_mut() = p.diagnostic_summaries(cx).collect();
                }
            }
        })
        .detach();
    });

    deterministic.run_until_parked();
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
                ..Default::default()
            },
        )]
    );

    // Simulate a language server reporting more errors for a file.
    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        lsp::PublishDiagnosticsParams {
            uri: lsp::Url::from_file_path("/a/a.rs").unwrap(),
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
    deterministic.run_until_parked();
    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(cx).collect::<Vec<_>>(),
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
            project.diagnostic_summaries(cx).collect::<Vec<_>>(),
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
    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
        .await
        .unwrap();

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
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(0, 10)..Point::new(0, 13),
                    diagnostic: Diagnostic {
                        group_id: 3,
                        severity: lsp::DiagnosticSeverity::WARNING,
                        message: "message 2".to_string(),
                        is_primary: true,
                        ..Default::default()
                    }
                }
            ]
        );
    });

    // Simulate a language server reporting no errors for a file.
    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        lsp::PublishDiagnosticsParams {
            uri: lsp::Url::from_file_path("/a/a.rs").unwrap(),
            version: None,
            diagnostics: vec![],
        },
    );
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, cx| {
        assert_eq!(project.diagnostic_summaries(cx).collect::<Vec<_>>(), [])
    });
    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(project.diagnostic_summaries(cx).collect::<Vec<_>>(), [])
    });
    project_c.read_with(cx_c, |project, cx| {
        assert_eq!(project.diagnostic_summaries(cx).collect::<Vec<_>>(), [])
    });
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_lsp_progress_updates_and_diagnostics_ordering(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            disk_based_diagnostics_progress_token: Some("the-disk-based-token".into()),
            disk_based_diagnostics_sources: vec!["the-disk-based-diagnostics-source".into()],
            ..Default::default()
        }))
        .await;
    client_a.language_registry.add(Arc::new(language));

    let file_names = &["one.rs", "two.rs", "three.rs", "four.rs", "five.rs"];
    client_a
        .fs
        .insert_tree(
            "/test",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = 2;",
                "three.rs": "const THREE: usize = 3;",
                "four.rs": "const FOUR: usize = 3;",
                "five.rs": "const FIVE: usize = 3;",
            }),
        )
        .await;

    let (project_a, worktree_id) = client_a.build_local_project("/test", cx_a).await;

    // Share a project as client A
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Join the project as client B and open all three files.
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    let guest_buffers = futures::future::try_join_all(file_names.iter().map(|file_name| {
        project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, file_name), cx))
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
        .unwrap();
    fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
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
            lsp::PublishDiagnosticsParams {
                uri: lsp::Url::from_file_path(Path::new("/test").join(file_name)).unwrap(),
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
    fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
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
                    for buffer in &guest_buffers {
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

    deterministic.run_until_parked();
    assert!(disk_based_diagnostics_finished.load(SeqCst));
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_completion(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }))
        .await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open a file in an editor as the guest.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    let (window_b, _) = cx_b.add_window(|_| EmptyView);
    let editor_b = cx_b.add_view(window_b, |cx| {
        Editor::for_buffer(buffer_b.clone(), Some(project_b.clone()), cx)
    });

    let fake_language_server = fake_language_servers.next().await.unwrap();
    cx_a.foreground().run_until_parked();
    buffer_b.read_with(cx_b, |buffer, _| {
        assert!(!buffer.completion_triggers().is_empty())
    });

    // Type a completion trigger character as the guest.
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
        editor.handle_input(".", cx);
        cx.focus(&editor_b);
    });

    // Receive a completion request as the host's language server.
    // Return some completions from the host's language server.
    cx_a.foreground().start_waiting();
    fake_language_server
        .handle_request::<lsp::request::Completion, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 14),
            );

            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "first_method(…)".into(),
                    detail: Some("fn(&mut self, B) -> C".into()),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        new_text: "first_method($1)".to_string(),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 14),
                            lsp::Position::new(0, 14),
                        ),
                    })),
                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
                lsp::CompletionItem {
                    label: "second_method(…)".into(),
                    detail: Some("fn(&mut self, C) -> D<E>".into()),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        new_text: "second_method()".to_string(),
                        range: lsp::Range::new(
                            lsp::Position::new(0, 14),
                            lsp::Position::new(0, 14),
                        ),
                    })),
                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                    ..Default::default()
                },
            ])))
        })
        .next()
        .await
        .unwrap();
    cx_a.foreground().finish_waiting();

    // Open the buffer on the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    cx_a.foreground().run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a. }")
    });

    // Confirm a completion on the guest.
    editor_b.read_with(cx_b, |editor, _| assert!(editor.context_menu_visible()));
    editor_b.update(cx_b, |editor, cx| {
        editor.confirm_completion(&ConfirmCompletion { item_ix: Some(0) }, cx);
        assert_eq!(editor.text(cx), "fn main() { a.first_method() }");
    });

    // Return a resolved completion from the host's language server.
    // The resolved completion has an additional text edit.
    fake_language_server.handle_request::<lsp::request::ResolveCompletionItem, _, _>(
        |params, _| async move {
            assert_eq!(params.label, "first_method(…)");
            Ok(lsp::CompletionItem {
                label: "first_method(…)".into(),
                detail: Some("fn(&mut self, B) -> C".into()),
                text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                    new_text: "first_method($1)".to_string(),
                    range: lsp::Range::new(lsp::Position::new(0, 14), lsp::Position::new(0, 14)),
                })),
                additional_text_edits: Some(vec![lsp::TextEdit {
                    new_text: "use d::SomeTrait;\n".to_string(),
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                }]),
                insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                ..Default::default()
            })
        },
    );

    // The additional edit is applied.
    cx_a.foreground().run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "use d::SomeTrait;\nfn main() { a.first_method() }"
        );
    });
    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "use d::SomeTrait;\nfn main() { a.first_method() }"
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_reloading_buffer_manually(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree("/a", json!({ "a.rs": "let one = 1;" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
        .await
        .unwrap();
    buffer_b.update(cx_b, |buffer, cx| {
        buffer.edit([(4..7, "six")], None, cx);
        buffer.edit([(10..11, "6")], None, cx);
        assert_eq!(buffer.text(), "let six = 6;");
        assert!(buffer.is_dirty());
        assert!(!buffer.has_conflict());
    });
    cx_a.foreground().run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| assert_eq!(buffer.text(), "let six = 6;"));

    client_a
        .fs
        .save(
            "/a/a.rs".as_ref(),
            &Rope::from("let seven = 7;"),
            LineEnding::Unix,
        )
        .await
        .unwrap();
    cx_a.foreground().run_until_parked();
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    use project::FormatTrigger;

    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    // Here we insert a fake tree with a directory that exists on disk. This is needed
    // because later we'll invoke a command, which requires passing a working directory
    // that points to a valid location on disk.
    let directory = env::current_dir().unwrap();
    client_a
        .fs
        .insert_tree(&directory, json!({ "a.rs": "let one = \"two\"" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(&directory, cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
        .await
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.handle_request::<lsp::request::Formatting, _, _>(|_, _| async move {
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

    // Ensure buffer can be formatted using an external command. Notice how the
    // host's configuration is honored as opposed to using the guest's settings.
    cx_a.update(|cx| {
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |file| {
                file.defaults.formatter = Some(Formatter::External {
                    command: "awk".into(),
                    arguments: vec!["{sub(/two/,\"{buffer_path}\")}1".to_string()].into(),
                });
            });
        });
    });
    project_b
        .update(cx_b, |project, cx| {
            project.format(
                HashSet::from_iter([buffer_b.clone()]),
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

#[gpui::test(iterations = 10)]
async fn test_definition(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/root",
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
    let (project_a, worktree_id) = client_a.build_local_project("/root/dir-1", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open the file on client B.
    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
        .await
        .unwrap();

    // Request the definition of a symbol as the guest.
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.handle_request::<lsp::request::GotoDefinition, _, _>(|_, _| async move {
        Ok(Some(lsp::GotoDefinitionResponse::Scalar(
            lsp::Location::new(
                lsp::Url::from_file_path("/root/dir-2/b.rs").unwrap(),
                lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
            ),
        )))
    });

    let definitions_1 = project_b
        .update(cx_b, |p, cx| p.definition(&buffer_b, 23, cx))
        .await
        .unwrap();
    cx_b.read(|cx| {
        assert_eq!(definitions_1.len(), 1);
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
    fake_language_server.handle_request::<lsp::request::GotoDefinition, _, _>(|_, _| async move {
        Ok(Some(lsp::GotoDefinitionResponse::Scalar(
            lsp::Location::new(
                lsp::Url::from_file_path("/root/dir-2/b.rs").unwrap(),
                lsp::Range::new(lsp::Position::new(1, 6), lsp::Position::new(1, 11)),
            ),
        )))
    });

    let definitions_2 = project_b
        .update(cx_b, |p, cx| p.definition(&buffer_b, 33, cx))
        .await
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

    fake_language_server.handle_request::<lsp::request::GotoTypeDefinition, _, _>(
        |req, _| async move {
            assert_eq!(
                req.text_document_position_params.position,
                lsp::Position::new(0, 7)
            );
            Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                lsp::Location::new(
                    lsp::Url::from_file_path("/root/dir-2/c.rs").unwrap(),
                    lsp::Range::new(lsp::Position::new(0, 5), lsp::Position::new(0, 7)),
                ),
            )))
        },
    );

    let type_definitions = project_b
        .update(cx_b, |p, cx| p.type_definition(&buffer_b, 7, cx))
        .await
        .unwrap();
    cx_b.read(|cx| {
        assert_eq!(type_definitions.len(), 1);
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/root",
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
    let (project_a, worktree_id) = client_a.build_local_project("/root/dir-1", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open the file on client B.
    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "one.rs"), cx)))
        .await
        .unwrap();

    // Request references to a symbol as the guest.
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.handle_request::<lsp::request::References, _, _>(|params, _| async move {
        assert_eq!(
            params.text_document_position.text_document.uri.as_str(),
            "file:///root/dir-1/one.rs"
        );
        Ok(Some(vec![
            lsp::Location {
                uri: lsp::Url::from_file_path("/root/dir-1/two.rs").unwrap(),
                range: lsp::Range::new(lsp::Position::new(0, 24), lsp::Position::new(0, 27)),
            },
            lsp::Location {
                uri: lsp::Url::from_file_path("/root/dir-1/two.rs").unwrap(),
                range: lsp::Range::new(lsp::Position::new(0, 35), lsp::Position::new(0, 38)),
            },
            lsp::Location {
                uri: lsp::Url::from_file_path("/root/dir-2/three.rs").unwrap(),
                range: lsp::Range::new(lsp::Position::new(0, 37), lsp::Position::new(0, 40)),
            },
        ]))
    });

    let references = project_b
        .update(cx_b, |p, cx| p.references(&buffer_b, 7, cx))
        .await
        .unwrap();
    cx_b.read(|cx| {
        assert_eq!(references.len(), 3);
        assert_eq!(project_b.read(cx).worktrees(cx).count(), 2);

        let two_buffer = references[0].buffer.read(cx);
        let three_buffer = references[2].buffer.read(cx);
        assert_eq!(
            two_buffer.file().unwrap().path().as_ref(),
            Path::new("two.rs")
        );
        assert_eq!(references[1].buffer, references[0].buffer);
        assert_eq!(
            three_buffer.file().unwrap().full_path(cx),
            Path::new("/root/dir-2/three.rs")
        );

        assert_eq!(references[0].range.to_offset(two_buffer), 24..27);
        assert_eq!(references[1].range.to_offset(two_buffer), 35..38);
        assert_eq!(references[2].range.to_offset(three_buffer), 37..40);
    });
}

#[gpui::test(iterations = 10)]
async fn test_project_search(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
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
            p.find_or_create_local_worktree("/root/dir-2", true, cx)
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

    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Perform a search as the guest.
    let results = project_b
        .update(cx_b, |project, cx| {
            project.search(
                SearchQuery::text("world", false, false, Vec::new(), Vec::new()),
                cx,
            )
        })
        .await
        .unwrap();

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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree(
            "/root-1",
            json!({
                "main.rs": "fn double(number: i32) -> i32 { number + number }",
            }),
        )
        .await;

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    let (project_a, worktree_id) = client_a.build_local_project("/root-1", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open the file on client B.
    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx)))
        .await
        .unwrap();

    // Request document highlights as the guest.
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.handle_request::<lsp::request::DocumentHighlightRequest, _, _>(
        |params, _| async move {
            assert_eq!(
                params
                    .text_document_position_params
                    .text_document
                    .uri
                    .as_str(),
                "file:///root-1/main.rs"
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs
        .insert_tree(
            "/root-1",
            json!({
                "main.rs": "use std::collections::HashMap;",
            }),
        )
        .await;

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    let (project_a, worktree_id) = client_a.build_local_project("/root-1", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open the file as the guest
    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx)))
        .await
        .unwrap();

    // Request hover information as the guest.
    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.handle_request::<lsp::request::HoverRequest, _, _>(
        |params, _| async move {
            assert_eq!(
                params
                    .text_document_position_params
                    .text_document
                    .uri
                    .as_str(),
                "file:///root-1/main.rs"
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
    );

    let hover_info = project_b
        .update(cx_b, |p, cx| p.hover(&buffer_b, 22, cx))
        .await
        .unwrap()
        .unwrap();
    buffer_b.read_with(cx_b, |buffer, _| {
        let snapshot = buffer.snapshot();
        assert_eq!(hover_info.range.unwrap().to_offset(&snapshot), 22..29);
        assert_eq!(
            hover_info.contents,
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
    });
}

#[gpui::test(iterations = 10)]
async fn test_project_symbols(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/code",
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
    let (project_a, worktree_id) = client_a.build_local_project("/code/crate-1", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Cause the language server to start.
    let _buffer = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "one.rs"), cx)))
        .await
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.handle_request::<lsp::WorkspaceSymbolRequest, _, _>(|_, _| async move {
        Ok(Some(lsp::WorkspaceSymbolResponse::Flat(vec![
            #[allow(deprecated)]
            lsp::SymbolInformation {
                name: "TWO".into(),
                location: lsp::Location {
                    uri: lsp::Url::from_file_path("/code/crate-2/two.rs").unwrap(),
                    range: lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                },
                kind: lsp::SymbolKind::CONSTANT,
                tags: None,
                container_name: None,
                deprecated: None,
            },
        ])))
    });

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
    buffer_b_2.read_with(cx_b, |buffer, _| {
        assert_eq!(
            buffer.file().unwrap().path().as_ref(),
            Path::new("../crate-2/two.rs")
        );
    });

    // Attempt to craft a symbol and violate host's privacy by opening an arbitrary file.
    let mut fake_symbol = symbols[0].clone();
    fake_symbol.path.path = Path::new("/code/secrets").into();
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    mut rng: StdRng,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/root",
            json!({
                "a.rs": "const ONE: usize = b::TWO;",
                "b.rs": "const TWO: usize = 2",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/root", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    let buffer_b1 = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx)))
        .await
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.handle_request::<lsp::request::GotoDefinition, _, _>(|_, _| async move {
        Ok(Some(lsp::GotoDefinitionResponse::Scalar(
            lsp::Location::new(
                lsp::Url::from_file_path("/root/b.rs").unwrap(),
                lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
            ),
        )))
    });

    let definitions;
    let buffer_b2;
    if rng.gen() {
        definitions = project_b.update(cx_b, |p, cx| p.definition(&buffer_b1, 23, cx));
        buffer_b2 = project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "b.rs"), cx));
    } else {
        buffer_b2 = project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "b.rs"), cx));
        definitions = project_b.update(cx_b, |p, cx| p.definition(&buffer_b1, 23, cx));
    }

    let buffer_b2 = buffer_b2.await.unwrap();
    let definitions = definitions.await.unwrap();
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].target.buffer, buffer_b2);
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_code_actions(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_b.update(editor::init);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "main.rs": "mod other;\nfn main() { let foo = other::foo(); }",
                "other.rs": "pub fn foo() -> usize { 4 }",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Join the project as client B.
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    let (_window_b, workspace_b) = cx_b.add_window(|cx| Workspace::test_new(project_b.clone(), cx));
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let mut fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server
        .handle_request::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
            );
            assert_eq!(params.range.start, lsp::Position::new(0, 0));
            assert_eq!(params.range.end, lsp::Position::new(0, 0));
            Ok(None)
        })
        .next()
        .await;

    // Move cursor to a location that contains code actions.
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(1, 31)..Point::new(1, 31)])
        });
        cx.focus(&editor_b);
    });

    fake_language_server
        .handle_request::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
            );
            assert_eq!(params.range.start, lsp::Position::new(1, 31));
            assert_eq!(params.range.end, lsp::Position::new(1, 31));

            Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                lsp::CodeAction {
                    title: "Inline into all callers".to_string(),
                    edit: Some(lsp::WorkspaceEdit {
                        changes: Some(
                            [
                                (
                                    lsp::Url::from_file_path("/a/main.rs").unwrap(),
                                    vec![lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(1, 22),
                                            lsp::Position::new(1, 34),
                                        ),
                                        "4".to_string(),
                                    )],
                                ),
                                (
                                    lsp::Url::from_file_path("/a/other.rs").unwrap(),
                                    vec![lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(0, 0),
                                            lsp::Position::new(0, 27),
                                        ),
                                        "".to_string(),
                                    )],
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                        ..Default::default()
                    }),
                    data: Some(json!({
                        "codeActionParams": {
                            "range": {
                                "start": {"line": 1, "column": 31},
                                "end": {"line": 1, "column": 31},
                            }
                        }
                    })),
                    ..Default::default()
                },
            )]))
        })
        .next()
        .await;

    // Toggle code actions and wait for them to display.
    editor_b.update(cx_b, |editor, cx| {
        editor.toggle_code_actions(
            &ToggleCodeActions {
                deployed_from_indicator: false,
            },
            cx,
        );
    });
    cx_a.foreground().run_until_parked();
    editor_b.read_with(cx_b, |editor, _| assert!(editor.context_menu_visible()));

    fake_language_server.remove_request_handler::<lsp::request::CodeActionRequest>();

    // Confirming the code action will trigger a resolve request.
    let confirm_action = workspace_b
        .update(cx_b, |workspace, cx| {
            Editor::confirm_code_action(workspace, &ConfirmCodeAction { item_ix: Some(0) }, cx)
        })
        .unwrap();
    fake_language_server.handle_request::<lsp::request::CodeActionResolveRequest, _, _>(
        |_, _| async move {
            Ok(lsp::CodeAction {
                title: "Inline into all callers".to_string(),
                edit: Some(lsp::WorkspaceEdit {
                    changes: Some(
                        [
                            (
                                lsp::Url::from_file_path("/a/main.rs").unwrap(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(1, 22),
                                        lsp::Position::new(1, 34),
                                    ),
                                    "4".to_string(),
                                )],
                            ),
                            (
                                lsp::Url::from_file_path("/a/other.rs").unwrap(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 0),
                                        lsp::Position::new(0, 27),
                                    ),
                                    "".to_string(),
                                )],
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            })
        },
    );

    // After the action is confirmed, an editor containing both modified files is opened.
    confirm_action.await.unwrap();
    let code_action_editor = workspace_b.read_with(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });
    code_action_editor.update(cx_b, |editor, cx| {
        assert_eq!(editor.text(cx), "mod other;\nfn main() { let foo = 4; }\n");
        editor.undo(&Undo, cx);
        assert_eq!(
            editor.text(cx),
            "mod other;\nfn main() { let foo = other::foo(); }\npub fn foo() -> usize { 4 }"
        );
        editor.redo(&Redo, cx);
        assert_eq!(editor.text(cx), "mod other;\nfn main() { let foo = 4; }\n");
    });
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_renames(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_b.update(editor::init);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                ..Default::default()
            },
            ..Default::default()
        }))
        .await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;"
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    let (_window_b, workspace_b) = cx_b.add_window(|cx| Workspace::test_new(project_b.clone(), cx));
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "one.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let fake_language_server = fake_language_servers.next().await.unwrap();

    // Move cursor to a location that can be renamed.
    let prepare_rename = editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([7..7]));
        editor.rename(&Rename, cx).unwrap()
    });

    fake_language_server
        .handle_request::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
            assert_eq!(params.text_document.uri.as_str(), "file:///dir/one.rs");
            assert_eq!(params.position, lsp::Position::new(0, 7));
            Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                lsp::Position::new(0, 6),
                lsp::Position::new(0, 9),
            ))))
        })
        .next()
        .await
        .unwrap();
    prepare_rename.await.unwrap();
    editor_b.update(cx_b, |editor, cx| {
        let rename = editor.pending_rename().unwrap();
        let buffer = editor.buffer().read(cx).snapshot(cx);
        assert_eq!(
            rename.range.start.to_offset(&buffer)..rename.range.end.to_offset(&buffer),
            6..9
        );
        rename.editor.update(cx, |rename_editor, cx| {
            rename_editor.buffer().update(cx, |rename_buffer, cx| {
                rename_buffer.edit([(0..3, "THREE")], None, cx);
            });
        });
    });

    let confirm_rename = workspace_b.update(cx_b, |workspace, cx| {
        Editor::confirm_rename(workspace, &ConfirmRename, cx).unwrap()
    });
    fake_language_server
        .handle_request::<lsp::request::Rename, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri.as_str(),
                "file:///dir/one.rs"
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 6)
            );
            assert_eq!(params.new_name, "THREE");
            Ok(Some(lsp::WorkspaceEdit {
                changes: Some(
                    [
                        (
                            lsp::Url::from_file_path("/dir/one.rs").unwrap(),
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                                "THREE".to_string(),
                            )],
                        ),
                        (
                            lsp::Url::from_file_path("/dir/two.rs").unwrap(),
                            vec![
                                lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 24),
                                        lsp::Position::new(0, 27),
                                    ),
                                    "THREE".to_string(),
                                ),
                                lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 35),
                                        lsp::Position::new(0, 38),
                                    ),
                                    "THREE".to_string(),
                                ),
                            ],
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            }))
        })
        .next()
        .await
        .unwrap();
    confirm_rename.await.unwrap();

    let rename_editor = workspace_b.read_with(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });
    rename_editor.update(cx_b, |editor, cx| {
        assert_eq!(
            editor.text(cx),
            "const THREE: usize = 1;\nconst TWO: usize = one::THREE + one::THREE;"
        );
        editor.undo(&Undo, cx);
        assert_eq!(
            editor.text(cx),
            "const ONE: usize = 1;\nconst TWO: usize = one::ONE + one::ONE;"
        );
        editor.redo(&Redo, cx);
        assert_eq!(
            editor.text(cx),
            "const THREE: usize = 1;\nconst TWO: usize = one::THREE + one::THREE;"
        );
    });

    // Ensure temporary rename edits cannot be undone/redone.
    editor_b.update(cx_b, |editor, cx| {
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "const ONE: usize = 1;");
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "const ONE: usize = 1;");
        editor.redo(&Redo, cx);
        assert_eq!(editor.text(cx), "const THREE: usize = 1;");
    })
}

#[gpui::test(iterations = 10)]
async fn test_language_server_statuses(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_b.update(editor::init);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            name: "the-language-server",
            ..Default::default()
        }))
        .await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/dir",
            json!({
                "main.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;

    let _buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.start_progress("the-token").await;
    fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
            lsp::WorkDoneProgressReport {
                message: Some("the-message".to_string()),
                ..Default::default()
            },
        )),
    });
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| {
        let status = project.language_server_statuses().next().unwrap();
        assert_eq!(status.name, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work["the-token"].message.as_ref().unwrap(),
            "the-message"
        );
    });

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    project_b.read_with(cx_b, |project, _| {
        let status = project.language_server_statuses().next().unwrap();
        assert_eq!(status.name, "the-language-server");
    });

    fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
            lsp::WorkDoneProgressReport {
                message: Some("the-message-2".to_string()),
                ..Default::default()
            },
        )),
    });
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| {
        let status = project.language_server_statuses().next().unwrap();
        assert_eq!(status.name, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work["the-token"].message.as_ref().unwrap(),
            "the-message-2"
        );
    });
    project_b.read_with(cx_b, |project, _| {
        let status = project.language_server_statuses().next().unwrap();
        assert_eq!(status.name, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work["the-token"].message.as_ref().unwrap(),
            "the-message-2"
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_contacts(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_d: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
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

    deterministic.run_until_parked();
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
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
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
        .authenticate_and_connect(false, &cx_c.to_async())
        .await
        .unwrap();

    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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

    active_call_b.update(cx_b, |call, _| call.decline_incoming().unwrap());
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.run_until_parked();
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
    deterministic.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);
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
        .user_store
        .update(cx_b, |store, cx| {
            store.remove_contact(client_c.user_id().unwrap(), cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
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
        client.user_store.read_with(cx, |store, _| {
            store
                .contacts()
                .iter()
                .map(|contact| {
                    (
                        contact.user.github_login.clone(),
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
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_a2: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_c2: &mut TestAppContext,
) {
    deterministic.forbid_parking();

    // Connect to a server as 3 clients.
    let mut server = TestServer::start(&deterministic).await;
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
        .user_store
        .update(cx_a, |store, cx| {
            store.request_contact(client_b.user_id().unwrap(), cx)
        })
        .await
        .unwrap();
    client_c
        .user_store
        .update(cx_c, |store, cx| {
            store.request_contact(client_b.user_id().unwrap(), cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();

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
    deterministic.run_until_parked();
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
        .user_store
        .update(cx_b, |store, cx| {
            store.respond_to_contact_request(client_a.user_id().unwrap(), true, cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();

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
    deterministic.run_until_parked();
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
        .user_store
        .update(cx_b, |store, cx| {
            store.respond_to_contact_request(client_c.user_id().unwrap(), false, cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();

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
    deterministic.run_until_parked();
    assert_eq!(client_a.summarize_contacts(cx_a).current, &["user_b"]);
    assert_eq!(client_b.summarize_contacts(cx_b).current, &["user_a"]);
    assert!(client_b
        .summarize_contacts(cx_b)
        .incoming_requests
        .is_empty());
    assert!(client_c.summarize_contacts(cx_c).current.is_empty());
    assert!(client_c
        .summarize_contacts(cx_c)
        .outgoing_requests
        .is_empty());

    async fn disconnect_and_reconnect(client: &TestClient, cx: &mut TestAppContext) {
        client.disconnect(&cx.to_async());
        client.clear_contacts(cx).await;
        client
            .authenticate_and_connect(false, &cx.to_async())
            .await
            .unwrap();
    }
}

#[gpui::test(iterations = 10)]
async fn test_basic_following(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_d: &mut TestAppContext,
) {
    deterministic.forbid_parking();

    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    let client_d = server.create_client(cx_d, "user_d").await;
    server
        .create_room(&mut [
            (&client_a, cx_a),
            (&client_b, cx_b),
            (&client_c, cx_c),
            (&client_d, cx_d),
        ])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "1.txt": "one\none\none",
                "2.txt": "two\ntwo\ntwo",
                "3.txt": "three\nthree\nthree",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let workspace_a = client_a.build_workspace(&project_a, cx_a);
    let workspace_b = client_b.build_workspace(&project_b, cx_b);

    // Client A opens some editors.
    let pane_a = workspace_a.read_with(cx_a, |workspace, _| workspace.active_pane().clone());
    let editor_a1 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let editor_a2 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B opens an editor.
    let editor_b1 = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let peer_id_a = client_a.peer_id().unwrap();
    let peer_id_b = client_b.peer_id().unwrap();
    let peer_id_c = client_c.peer_id().unwrap();
    let peer_id_d = client_d.peer_id().unwrap();

    // Client A updates their selections in those editors
    editor_a1.update(cx_a, |editor, cx| {
        editor.handle_input("a", cx);
        editor.handle_input("b", cx);
        editor.handle_input("c", cx);
        editor.select_left(&Default::default(), cx);
        assert_eq!(editor.selections.ranges(cx), vec![3..2]);
    });
    editor_a2.update(cx_a, |editor, cx| {
        editor.handle_input("d", cx);
        editor.handle_input("e", cx);
        editor.select_left(&Default::default(), cx);
        assert_eq!(editor.selections.ranges(cx), vec![2..1]);
    });

    // When client B starts following client A, all visible view states are replicated to client B.
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.toggle_follow(peer_id_a, cx).unwrap()
        })
        .await
        .unwrap();

    cx_c.foreground().run_until_parked();
    let editor_b2 = workspace_b.read_with(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });
    assert_eq!(
        cx_b.read(|cx| editor_b2.project_path(cx)),
        Some((worktree_id, "2.txt").into())
    );
    assert_eq!(
        editor_b2.read_with(cx_b, |editor, cx| editor.selections.ranges(cx)),
        vec![2..1]
    );
    assert_eq!(
        editor_b1.read_with(cx_b, |editor, cx| editor.selections.ranges(cx)),
        vec![3..2]
    );

    cx_c.foreground().run_until_parked();
    let active_call_c = cx_c.read(ActiveCall::global);
    let project_c = client_c.build_remote_project(project_id, cx_c).await;
    let workspace_c = client_c.build_workspace(&project_c, cx_c);
    active_call_c
        .update(cx_c, |call, cx| call.set_location(Some(&project_c), cx))
        .await
        .unwrap();
    drop(project_c);

    // Client C also follows client A.
    workspace_c
        .update(cx_c, |workspace, cx| {
            workspace.toggle_follow(peer_id_a, cx).unwrap()
        })
        .await
        .unwrap();

    cx_d.foreground().run_until_parked();
    let active_call_d = cx_d.read(ActiveCall::global);
    let project_d = client_d.build_remote_project(project_id, cx_d).await;
    let workspace_d = client_d.build_workspace(&project_d, cx_d);
    active_call_d
        .update(cx_d, |call, cx| call.set_location(Some(&project_d), cx))
        .await
        .unwrap();
    drop(project_d);

    // All clients see that clients B and C are following client A.
    cx_c.foreground().run_until_parked();
    for (name, active_call, cx) in [
        ("A", &active_call_a, &cx_a),
        ("B", &active_call_b, &cx_b),
        ("C", &active_call_c, &cx_c),
        ("D", &active_call_d, &cx_d),
    ] {
        active_call.read_with(*cx, |call, cx| {
            let room = call.room().unwrap().read(cx);
            assert_eq!(
                room.followers_for(peer_id_a, project_id),
                &[peer_id_b, peer_id_c],
                "checking followers for A as {name}"
            );
        });
    }

    // Client C unfollows client A.
    workspace_c.update(cx_c, |workspace, cx| {
        workspace.toggle_follow(peer_id_a, cx);
    });

    // All clients see that clients B is following client A.
    cx_c.foreground().run_until_parked();
    for (name, active_call, cx) in [
        ("A", &active_call_a, &cx_a),
        ("B", &active_call_b, &cx_b),
        ("C", &active_call_c, &cx_c),
        ("D", &active_call_d, &cx_d),
    ] {
        active_call.read_with(*cx, |call, cx| {
            let room = call.room().unwrap().read(cx);
            assert_eq!(
                room.followers_for(peer_id_a, project_id),
                &[peer_id_b],
                "checking followers for A as {name}"
            );
        });
    }

    // Client C re-follows client A.
    workspace_c.update(cx_c, |workspace, cx| {
        workspace.toggle_follow(peer_id_a, cx);
    });

    // All clients see that clients B and C are following client A.
    cx_c.foreground().run_until_parked();
    for (name, active_call, cx) in [
        ("A", &active_call_a, &cx_a),
        ("B", &active_call_b, &cx_b),
        ("C", &active_call_c, &cx_c),
        ("D", &active_call_d, &cx_d),
    ] {
        active_call.read_with(*cx, |call, cx| {
            let room = call.room().unwrap().read(cx);
            assert_eq!(
                room.followers_for(peer_id_a, project_id),
                &[peer_id_b, peer_id_c],
                "checking followers for A as {name}"
            );
        });
    }

    // Client D follows client C.
    workspace_d
        .update(cx_d, |workspace, cx| {
            workspace.toggle_follow(peer_id_c, cx).unwrap()
        })
        .await
        .unwrap();

    // All clients see that D is following C
    cx_d.foreground().run_until_parked();
    for (name, active_call, cx) in [
        ("A", &active_call_a, &cx_a),
        ("B", &active_call_b, &cx_b),
        ("C", &active_call_c, &cx_c),
        ("D", &active_call_d, &cx_d),
    ] {
        active_call.read_with(*cx, |call, cx| {
            let room = call.room().unwrap().read(cx);
            assert_eq!(
                room.followers_for(peer_id_c, project_id),
                &[peer_id_d],
                "checking followers for C as {name}"
            );
        });
    }

    // Client C closes the project.
    cx_c.drop_last(workspace_c);

    // Clients A and B see that client B is following A, and client C is not present in the followers.
    cx_c.foreground().run_until_parked();
    for (name, active_call, cx) in [("A", &active_call_a, &cx_a), ("B", &active_call_b, &cx_b)] {
        active_call.read_with(*cx, |call, cx| {
            let room = call.room().unwrap().read(cx);
            assert_eq!(
                room.followers_for(peer_id_a, project_id),
                &[peer_id_b],
                "checking followers for A as {name}"
            );
        });
    }

    // All clients see that no-one is following C
    for (name, active_call, cx) in [
        ("A", &active_call_a, &cx_a),
        ("B", &active_call_b, &cx_b),
        ("C", &active_call_c, &cx_c),
        ("D", &active_call_d, &cx_d),
    ] {
        active_call.read_with(*cx, |call, cx| {
            let room = call.room().unwrap().read(cx);
            assert_eq!(
                room.followers_for(peer_id_c, project_id),
                &[],
                "checking followers for C as {name}"
            );
        });
    }

    // When client A activates a different editor, client B does so as well.
    workspace_a.update(cx_a, |workspace, cx| {
        workspace.activate_item(&editor_a1, cx)
    });
    deterministic.run_until_parked();
    workspace_b.read_with(cx_b, |workspace, cx| {
        assert_eq!(workspace.active_item(cx).unwrap().id(), editor_b1.id());
    });

    // When client A opens a multibuffer, client B does so as well.
    let multibuffer_a = cx_a.add_model(|cx| {
        let buffer_a1 = project_a.update(cx, |project, cx| {
            project
                .get_open_buffer(&(worktree_id, "1.txt").into(), cx)
                .unwrap()
        });
        let buffer_a2 = project_a.update(cx, |project, cx| {
            project
                .get_open_buffer(&(worktree_id, "2.txt").into(), cx)
                .unwrap()
        });
        let mut result = MultiBuffer::new(0);
        result.push_excerpts(
            buffer_a1,
            [ExcerptRange {
                context: 0..3,
                primary: None,
            }],
            cx,
        );
        result.push_excerpts(
            buffer_a2,
            [ExcerptRange {
                context: 4..7,
                primary: None,
            }],
            cx,
        );
        result
    });
    let multibuffer_editor_a = workspace_a.update(cx_a, |workspace, cx| {
        let editor =
            cx.add_view(|cx| Editor::for_multibuffer(multibuffer_a, Some(project_a.clone()), cx));
        workspace.add_item(Box::new(editor.clone()), cx);
        editor
    });
    deterministic.run_until_parked();
    let multibuffer_editor_b = workspace_b.read_with(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });
    assert_eq!(
        multibuffer_editor_a.read_with(cx_a, |editor, cx| editor.text(cx)),
        multibuffer_editor_b.read_with(cx_b, |editor, cx| editor.text(cx)),
    );

    // When client A navigates back and forth, client B does so as well.
    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.go_back(workspace.active_pane().downgrade(), cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    workspace_b.read_with(cx_b, |workspace, cx| {
        assert_eq!(workspace.active_item(cx).unwrap().id(), editor_b1.id());
    });

    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.go_back(workspace.active_pane().downgrade(), cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    workspace_b.read_with(cx_b, |workspace, cx| {
        assert_eq!(workspace.active_item(cx).unwrap().id(), editor_b2.id());
    });

    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.go_forward(workspace.active_pane().downgrade(), cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    workspace_b.read_with(cx_b, |workspace, cx| {
        assert_eq!(workspace.active_item(cx).unwrap().id(), editor_b1.id());
    });

    // Changes to client A's editor are reflected on client B.
    editor_a1.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([1..1, 2..2]));
    });
    deterministic.run_until_parked();
    editor_b1.read_with(cx_b, |editor, cx| {
        assert_eq!(editor.selections.ranges(cx), &[1..1, 2..2]);
    });

    editor_a1.update(cx_a, |editor, cx| editor.set_text("TWO", cx));
    deterministic.run_until_parked();
    editor_b1.read_with(cx_b, |editor, cx| assert_eq!(editor.text(cx), "TWO"));

    editor_a1.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([3..3]));
        editor.set_scroll_position(vec2f(0., 100.), cx);
    });
    deterministic.run_until_parked();
    editor_b1.read_with(cx_b, |editor, cx| {
        assert_eq!(editor.selections.ranges(cx), &[3..3]);
    });

    // After unfollowing, client B stops receiving updates from client A.
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.unfollow(&workspace.active_pane().clone(), cx)
    });
    workspace_a.update(cx_a, |workspace, cx| {
        workspace.activate_item(&editor_a2, cx)
    });
    deterministic.run_until_parked();
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, cx| workspace
            .active_item(cx)
            .unwrap()
            .id()),
        editor_b1.id()
    );

    // Client A starts following client B.
    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.toggle_follow(peer_id_b, cx).unwrap()
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, _| workspace.leader_for_pane(&pane_a)),
        Some(peer_id_b)
    );
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, cx| workspace
            .active_item(cx)
            .unwrap()
            .id()),
        editor_a1.id()
    );

    // Client B activates an external window, which causes a new screen-sharing item to be added to the pane.
    let display = MacOSDisplay::new();
    active_call_b
        .update(cx_b, |call, cx| call.set_location(None, cx))
        .await
        .unwrap();
    active_call_b
        .update(cx_b, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_display_sources(vec![display.clone()]);
                room.share_screen(cx)
            })
        })
        .await
        .unwrap();
    deterministic.run_until_parked();
    let shared_screen = workspace_a.read_with(cx_a, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<SharedScreen>()
            .unwrap()
    });

    // Client B activates Zed again, which causes the previous editor to become focused again.
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    workspace_a.read_with(cx_a, |workspace, cx| {
        assert_eq!(workspace.active_item(cx).unwrap().id(), editor_a1.id())
    });

    // Client B activates a multibuffer that was created by following client A. Client A returns to that multibuffer.
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.activate_item(&multibuffer_editor_b, cx)
    });
    deterministic.run_until_parked();
    workspace_a.read_with(cx_a, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().id(),
            multibuffer_editor_a.id()
        )
    });

    // Client B activates an external window again, and the previously-opened screen-sharing item
    // gets activated.
    active_call_b
        .update(cx_b, |call, cx| call.set_location(None, cx))
        .await
        .unwrap();
    deterministic.run_until_parked();
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, cx| workspace
            .active_item(cx)
            .unwrap()
            .id()),
        shared_screen.id()
    );

    // Following interrupts when client B disconnects.
    client_b.disconnect(&cx_b.to_async());
    deterministic.advance_clock(RECONNECT_TIMEOUT);
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, _| workspace.leader_for_pane(&pane_a)),
        None
    );
}

#[gpui::test(iterations = 10)]
async fn test_join_call_after_screen_was_shared(
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
    deterministic.run_until_parked();
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
    let display = MacOSDisplay::new();
    active_call_a
        .update(cx_a, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_display_sources(vec![display.clone()]);
                room.share_screen(cx)
            })
        })
        .await
        .unwrap();

    client_b.user_store.update(cx_b, |user_store, _| {
        user_store.clear_cache();
    });

    // User B joins the room
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    assert!(incoming_call_b.next().await.unwrap().is_none());

    deterministic.run_until_parked();
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
                .tracks
                .len(),
            1
        );
    });
}

#[gpui::test]
async fn test_following_tab_order(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let workspace_a = client_a.build_workspace(&project_a, cx_a);
    let pane_a = workspace_a.read_with(cx_a, |workspace, _| workspace.active_pane().clone());

    let workspace_b = client_b.build_workspace(&project_b, cx_b);
    let pane_b = workspace_b.read_with(cx_b, |workspace, _| workspace.active_pane().clone());

    let client_b_id = project_a.read_with(cx_a, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });

    //Open 1, 3 in that order on client A
    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, cx)
        })
        .await
        .unwrap();
    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "3.txt"), None, true, cx)
        })
        .await
        .unwrap();

    let pane_paths = |pane: &ViewHandle<workspace::Pane>, cx: &mut TestAppContext| {
        pane.update(cx, |pane, cx| {
            pane.items()
                .map(|item| {
                    item.project_path(cx)
                        .unwrap()
                        .path
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
                .collect::<Vec<_>>()
        })
    };

    //Verify that the tabs opened in the order we expect
    assert_eq!(&pane_paths(&pane_a, cx_a), &["1.txt", "3.txt"]);

    //Follow client B as client A
    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.toggle_follow(client_b_id, cx).unwrap()
        })
        .await
        .unwrap();

    //Open just 2 on client B
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();

    // Verify that newly opened followed file is at the end
    assert_eq!(&pane_paths(&pane_a, cx_a), &["1.txt", "3.txt", "2.txt"]);

    //Open just 1 on client B
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, cx)
        })
        .await
        .unwrap();
    assert_eq!(&pane_paths(&pane_b, cx_b), &["2.txt", "1.txt"]);
    deterministic.run_until_parked();

    // Verify that following into 1 did not reorder
    assert_eq!(&pane_paths(&pane_a, cx_a), &["1.txt", "3.txt", "2.txt"]);
}

#[gpui::test(iterations = 10)]
async fn test_peers_following_each_other(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    // Client A shares a project.
    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
                "4.txt": "four",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Client B joins the project.
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    // Client A opens some editors.
    let workspace_a = client_a.build_workspace(&project_a, cx_a);
    let pane_a1 = workspace_a.read_with(cx_a, |workspace, _| workspace.active_pane().clone());
    let _editor_a1 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B opens an editor.
    let workspace_b = client_b.build_workspace(&project_b, cx_b);
    let pane_b1 = workspace_b.read_with(cx_b, |workspace, _| workspace.active_pane().clone());
    let _editor_b1 = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Clients A and B follow each other in split panes
    workspace_a.update(cx_a, |workspace, cx| {
        workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
    });
    workspace_a
        .update(cx_a, |workspace, cx| {
            assert_ne!(*workspace.active_pane(), pane_a1);
            let leader_id = *project_a.read(cx).collaborators().keys().next().unwrap();
            workspace.toggle_follow(leader_id, cx).unwrap()
        })
        .await
        .unwrap();
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
    });
    workspace_b
        .update(cx_b, |workspace, cx| {
            assert_ne!(*workspace.active_pane(), pane_b1);
            let leader_id = *project_b.read(cx).collaborators().keys().next().unwrap();
            workspace.toggle_follow(leader_id, cx).unwrap()
        })
        .await
        .unwrap();

    workspace_a.update(cx_a, |workspace, cx| {
        workspace.activate_next_pane(cx);
    });
    // Wait for focus effects to be fully flushed
    workspace_a.update(cx_a, |workspace, _| {
        assert_eq!(*workspace.active_pane(), pane_a1);
    });

    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "3.txt"), None, true, cx)
        })
        .await
        .unwrap();
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.activate_next_pane(cx);
    });

    workspace_b
        .update(cx_b, |workspace, cx| {
            assert_eq!(*workspace.active_pane(), pane_b1);
            workspace.open_path((worktree_id, "4.txt"), None, true, cx)
        })
        .await
        .unwrap();
    cx_a.foreground().run_until_parked();

    // Ensure leader updates don't change the active pane of followers
    workspace_a.read_with(cx_a, |workspace, _| {
        assert_eq!(*workspace.active_pane(), pane_a1);
    });
    workspace_b.read_with(cx_b, |workspace, _| {
        assert_eq!(*workspace.active_pane(), pane_b1);
    });

    // Ensure peers following each other doesn't cause an infinite loop.
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, cx| workspace
            .active_item(cx)
            .unwrap()
            .project_path(cx)),
        Some((worktree_id, "3.txt").into())
    );
    workspace_a.update(cx_a, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().project_path(cx),
            Some((worktree_id, "3.txt").into())
        );
        workspace.activate_next_pane(cx);
    });

    workspace_a.update(cx_a, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().project_path(cx),
            Some((worktree_id, "4.txt").into())
        );
    });

    workspace_b.update(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().project_path(cx),
            Some((worktree_id, "4.txt").into())
        );
        workspace.activate_next_pane(cx);
    });

    workspace_b.update(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().project_path(cx),
            Some((worktree_id, "3.txt").into())
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_auto_unfollowing(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();

    // 2 clients connect to a server.
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    // Client A shares a project.
    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    // Client A opens some editors.
    let workspace_a = client_a.build_workspace(&project_a, cx_a);
    let _editor_a1 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B starts following client A.
    let workspace_b = client_b.build_workspace(&project_b, cx_b);
    let pane_b = workspace_b.read_with(cx_b, |workspace, _| workspace.active_pane().clone());
    let leader_id = project_b.read_with(cx_b, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.toggle_follow(leader_id, cx).unwrap()
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );
    let editor_b2 = workspace_b.read_with(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });

    // When client B moves, it automatically stops following client A.
    editor_b2.update(cx_b, |editor, cx| editor.move_right(&editor::MoveRight, cx));
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        None
    );

    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.toggle_follow(leader_id, cx).unwrap()
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B edits, it automatically stops following client A.
    editor_b2.update(cx_b, |editor, cx| editor.insert("X", cx));
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        None
    );

    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.toggle_follow(leader_id, cx).unwrap()
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B scrolls, it automatically stops following client A.
    editor_b2.update(cx_b, |editor, cx| {
        editor.set_scroll_position(vec2f(0., 3.), cx)
    });
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        None
    );

    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.toggle_follow(leader_id, cx).unwrap()
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B activates a different pane, it continues following client A in the original pane.
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.split_pane(pane_b.clone(), SplitDirection::Right, cx)
    });
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    workspace_b.update(cx_b, |workspace, cx| workspace.activate_next_pane(cx));
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B activates a different item in the original pane, it automatically stops following client A.
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, cx)
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        None
    );
}

#[gpui::test(iterations = 10)]
async fn test_peers_simultaneously_following_each_other(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();

    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    client_a.fs.insert_tree("/a", json!({})).await;
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    let workspace_a = client_a.build_workspace(&project_a, cx_a);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    let workspace_b = client_b.build_workspace(&project_b, cx_b);

    deterministic.run_until_parked();
    let client_a_id = project_b.read_with(cx_b, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });
    let client_b_id = project_a.read_with(cx_a, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });

    let a_follow_b = workspace_a.update(cx_a, |workspace, cx| {
        workspace.toggle_follow(client_b_id, cx).unwrap()
    });
    let b_follow_a = workspace_b.update(cx_b, |workspace, cx| {
        workspace.toggle_follow(client_a_id, cx).unwrap()
    });

    futures::try_join!(a_follow_b, b_follow_a).unwrap();
    workspace_a.read_with(cx_a, |workspace, _| {
        assert_eq!(
            workspace.leader_for_pane(workspace.active_pane()),
            Some(client_b_id)
        );
    });
    workspace_b.read_with(cx_b, |workspace, _| {
        assert_eq!(
            workspace.leader_for_pane(workspace.active_pane()),
            Some(client_a_id)
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_on_input_format_from_host_to_guest(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_on_type_formatting_provider: Some(lsp::DocumentOnTypeFormattingOptions {
                    first_trigger_character: ":".to_string(),
                    more_trigger_character: Some(vec![">".to_string()]),
                }),
                ..Default::default()
            },
            ..Default::default()
        }))
        .await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open a file in an editor as the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    let (window_a, _) = cx_a.add_window(|_| EmptyView);
    let editor_a = cx_a.add_view(window_a, |cx| {
        Editor::for_buffer(buffer_a, Some(project_a.clone()), cx)
    });

    let fake_language_server = fake_language_servers.next().await.unwrap();
    cx_b.foreground().run_until_parked();

    // Receive an OnTypeFormatting request as the host's language server.
    // Return some formattings from the host's language server.
    fake_language_server.handle_request::<lsp::request::OnTypeFormatting, _, _>(
        |params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 14),
            );

            Ok(Some(vec![lsp::TextEdit {
                new_text: "~<".to_string(),
                range: lsp::Range::new(lsp::Position::new(0, 14), lsp::Position::new(0, 14)),
            }]))
        },
    );

    // Open the buffer on the guest and see that the formattings worked
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();

    // Type a on type formatting trigger character as the guest.
    editor_a.update(cx_a, |editor, cx| {
        cx.focus(&editor_a);
        editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
        editor.handle_input(">", cx);
    });

    cx_b.foreground().run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a>~< }")
    });

    // Undo should remove LSP edits first
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(editor.text(cx), "fn main() { a>~< }");
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "fn main() { a> }");
    });
    cx_b.foreground().run_until_parked();
    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a> }")
    });

    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(editor.text(cx), "fn main() { a> }");
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "fn main() { a }");
    });
    cx_b.foreground().run_until_parked();
    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a }")
    });
}

#[gpui::test(iterations = 10)]
async fn test_on_input_format_from_guest_to_host(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a fake language server.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_language_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_on_type_formatting_provider: Some(lsp::DocumentOnTypeFormattingOptions {
                    first_trigger_character: ":".to_string(),
                    more_trigger_character: Some(vec![">".to_string()]),
                }),
                ..Default::default()
            },
            ..Default::default()
        }))
        .await;
    client_a.language_registry.add(Arc::new(language));

    client_a
        .fs
        .insert_tree(
            "/a",
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Open a file in an editor as the guest.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    let (window_b, _) = cx_b.add_window(|_| EmptyView);
    let editor_b = cx_b.add_view(window_b, |cx| {
        Editor::for_buffer(buffer_b, Some(project_b.clone()), cx)
    });

    let fake_language_server = fake_language_servers.next().await.unwrap();
    cx_a.foreground().run_until_parked();
    // Type a on type formatting trigger character as the guest.
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
        editor.handle_input(":", cx);
        cx.focus(&editor_b);
    });

    // Receive an OnTypeFormatting request as the host's language server.
    // Return some formattings from the host's language server.
    cx_a.foreground().start_waiting();
    fake_language_server
        .handle_request::<lsp::request::OnTypeFormatting, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 14),
            );

            Ok(Some(vec![lsp::TextEdit {
                new_text: "~:".to_string(),
                range: lsp::Range::new(lsp::Position::new(0, 14), lsp::Position::new(0, 14)),
            }]))
        })
        .next()
        .await
        .unwrap();
    cx_a.foreground().finish_waiting();

    // Open the buffer on the host and see that the formattings worked
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    cx_a.foreground().run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a:~: }")
    });

    // Undo should remove LSP edits first
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(editor.text(cx), "fn main() { a:~: }");
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "fn main() { a: }");
    });
    cx_a.foreground().run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a: }")
    });

    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(editor.text(cx), "fn main() { a: }");
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "fn main() { a }");
    });
    cx_a.foreground().run_until_parked();
    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a }")
    });
}

#[derive(Debug, Eq, PartialEq)]
struct RoomParticipants {
    remote: Vec<String>,
    pending: Vec<String>,
}

fn room_participants(room: &ModelHandle<Room>, cx: &mut TestAppContext) -> RoomParticipants {
    room.read_with(cx, |room, _| {
        let mut remote = room
            .remote_participants()
            .iter()
            .map(|(_, participant)| participant.user.github_login.clone())
            .collect::<Vec<_>>();
        let mut pending = room
            .pending_participants()
            .iter()
            .map(|user| user.github_login.clone())
            .collect::<Vec<_>>();
        remote.sort();
        pending.sort();
        RoomParticipants { remote, pending }
    })
}
