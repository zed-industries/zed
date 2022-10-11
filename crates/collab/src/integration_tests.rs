use crate::{
    db::{NewUserParams, ProjectId, TestDb, UserId},
    rpc::{Executor, Server, Store},
    AppState,
};
use ::rpc::Peer;
use anyhow::anyhow;
use call::{room, ActiveCall, ParticipantLocation, Room};
use client::{
    self, test::FakeHttpClient, Channel, ChannelDetails, ChannelList, Client, Connection,
    Credentials, EstablishConnectionError, User, UserStore, RECEIVE_TIMEOUT,
};
use collections::{BTreeMap, HashMap, HashSet};
use editor::{
    self, ConfirmCodeAction, ConfirmCompletion, ConfirmRename, Editor, Redo, Rename, ToOffset,
    ToggleCodeActions, Undo,
};
use fs::{FakeFs, Fs as _, LineEnding};
use futures::{channel::mpsc, Future, StreamExt as _};
use gpui::{
    executor::{self, Deterministic},
    geometry::vector::vec2f,
    test::EmptyView,
    ModelHandle, Task, TestAppContext, ViewHandle,
};
use language::{
    range_to_lsp, tree_sitter_rust, Diagnostic, DiagnosticEntry, FakeLspAdapter, Language,
    LanguageConfig, LanguageRegistry, OffsetRangeExt, Rope,
};
use lsp::{self, FakeLanguageServer};
use parking_lot::Mutex;
use project::{
    search::SearchQuery, worktree::WorktreeHandle, DiagnosticSummary, Project, ProjectPath,
    ProjectStore, WorktreeId,
};
use rand::prelude::*;
use rope::point::Point;
use rpc::PeerId;
use serde_json::json;
use settings::{Formatter, Settings};
use sqlx::types::time::OffsetDateTime;
use std::{
    cell::{Cell, RefCell},
    env, mem,
    ops::Deref,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    time::Duration,
};
use theme::ThemeRegistry;
use unindent::Unindent as _;
use workspace::{Item, SplitDirection, ToggleFollow, Workspace};

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
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    assert_eq!(call_b.caller.github_login, "user_a");

    // User B connects via another client and also receives a ring on the newly-connected client.
    let _client_b2 = server.create_client(cx_b2, "user_b").await;
    let active_call_b2 = cx_b2.read(ActiveCall::global);
    let mut incoming_call_b2 = active_call_b2.read_with(cx_b2, |call, _| call.incoming());
    deterministic.run_until_parked();
    let call_b2 = incoming_call_b2.next().await.unwrap().unwrap();
    assert_eq!(call_b2.caller.github_login, "user_a");

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
    assert_eq!(call_c.caller.github_login, "user_b");
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

    // User A leaves the room.
    active_call_a.update(cx_a, |call, cx| {
        call.hang_up(cx).unwrap();
        assert!(call.room().is_none());
    });
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
            remote: Default::default(),
            pending: Default::default()
        }
    );

    // User B leaves the room.
    active_call_b.update(cx_b, |call, cx| {
        call.hang_up(cx).unwrap();
        assert!(call.room().is_none());
    });
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
            remote: Default::default(),
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
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    assert_eq!(call_b1.caller.github_login, "user_a");

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
    assert_eq!(call_b2.caller.github_login, "user_c");
}

#[gpui::test(iterations = 10)]
async fn test_leaving_room_on_disconnection(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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

    // When user A disconnects, both client A and B clear their room on the active call.
    server.disconnect_client(client_a.current_user_id(cx_a));
    cx_a.foreground().advance_clock(rpc::RECEIVE_TIMEOUT);
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
}

#[gpui::test(iterations = 10)]
async fn test_calls_on_multiple_connections(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b1: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b1 = server.create_client(cx_b1, "user_b").await;
    let _client_b2 = server.create_client(cx_b2, "user_b").await;
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

    // User B hangs up, and user A calls them again.
    active_call_b2.update(cx_b2, |call, cx| call.hang_up(cx).unwrap());
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
    active_call_a.update(cx_a, |call, cx| call.hang_up(cx).unwrap());
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

    // User A disconnects up, causing both connections to stop ringing.
    server.disconnect_client(client_a.current_user_id(cx_a));
    cx_a.foreground().advance_clock(rpc::RECEIVE_TIMEOUT);
    assert!(incoming_call_b1.next().await.unwrap().is_none());
    assert!(incoming_call_b2.next().await.unwrap().is_none());
}

#[gpui::test(iterations = 10)]
async fn test_share_project(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let (_, window_b) = cx_b.add_window(|_| EmptyView);
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

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
    assert_eq!(call.caller.github_login, "user_a");
    let initial_project = call.initial_project.unwrap();
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    let client_b_peer_id = client_b.peer_id;
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

    let editor_b = cx_b.add_view(&window_b, |cx| Editor::for_buffer(buffer_b, None, cx));

    // TODO
    // // Create a selection set as client B and see that selection set as client A.
    // buffer_a
    //     .condition(&cx_a, |buffer, _| buffer.selection_sets().count() == 1)
    //     .await;

    // Edit the buffer as client B and see that edit as client A.
    editor_b.update(cx_b, |editor, cx| editor.handle_input("ok, ", cx));
    buffer_a
        .condition(cx_a, |buffer, _| buffer.text() == "ok, b-contents")
        .await;

    // TODO
    // // Remove the selection set as client B, see those selections disappear as client A.
    cx_b.update(move |_| drop(editor_b));
    // buffer_a
    //     .condition(&cx_a, |buffer, _| buffer.selection_sets().count() == 0)
    //     .await;
}

#[gpui::test(iterations = 10)]
async fn test_unshare_project(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

    project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // When client B leaves the room, the project becomes read-only.
    active_call_b.update(cx_b, |call, cx| call.hang_up(cx).unwrap());
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
    assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));
    project_c2
        .update(cx_c, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // When client A (the host) leaves the room, the project gets unshared and guests are notified.
    active_call_a.update(cx_a, |call, cx| call.hang_up(cx).unwrap());
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
    cx_b.update(editor::init);
    deterministic.forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

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
    assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

    let (_, workspace_b) =
        cx_b.add_window(|cx| Workspace::new(project_b.clone(), |_, _| unimplemented!(), cx));
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "b.txt"), true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    cx_b.read(|cx| {
        assert_eq!(
            cx.focused_view_id(workspace_b.window_id()),
            Some(editor_b.id())
        );
    });
    editor_b.update(cx_b, |editor, cx| editor.insert("X", cx));
    assert!(cx_b.is_window_edited(workspace_b.window_id()));

    // Drop client A's connection. Collaborators should disappear and the project should not be shown as shared.
    server.disconnect_client(client_a.current_user_id(cx_a));
    cx_a.foreground().advance_clock(rpc::RECEIVE_TIMEOUT);
    project_a
        .condition(cx_a, |project, _| project.collaborators().is_empty())
        .await;
    project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));
    project_b
        .condition(cx_b, |project, _| project.is_read_only())
        .await;
    assert!(worktree_a.read_with(cx_a, |tree, _| !tree.as_local().unwrap().is_shared()));

    // Ensure client B's edited state is reset and that the whole window is blurred.
    cx_b.read(|cx| {
        assert_eq!(cx.focused_view_id(workspace_b.window_id()), None);
    });
    assert!(!cx_b.is_window_edited(workspace_b.window_id()));

    // Ensure client B is not prompted to save edits when closing window after disconnecting.
    let can_close = workspace_b
        .update(cx_b, |workspace, cx| workspace.prepare_to_close(true, cx))
        .await
        .unwrap();
    assert!(can_close);
}

#[gpui::test(iterations = 10)]
async fn test_active_call_events(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
}

#[gpui::test(iterations = 10)]
async fn test_room_location(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    let room_a = active_call_a.read_with(cx_a, |call, _| call.room().unwrap().clone());
    let a_notified = Rc::new(Cell::new(false));
    cx_a.update({
        let notified = a_notified.clone();
        |cx| {
            cx.observe(&active_call_a, move |_, _| notified.set(true))
                .detach()
        }
    });

    let active_call_b = cx_b.read(ActiveCall::global);
    let room_b = active_call_b.read_with(cx_b, |call, _| call.room().unwrap().clone());
    let b_notified = Rc::new(Cell::new(false));
    cx_b.update({
        let b_notified = b_notified.clone();
        |cx| {
            cx.observe(&active_call_b, move |_, _| b_notified.set(true))
                .detach()
        }
    });

    room_a
        .update(cx_a, |room, cx| room.set_location(Some(&project_a), cx))
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

    room_b
        .update(cx_b, |room, cx| room.set_location(Some(&project_b), cx))
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

    room_b
        .update(cx_b, |room, cx| room.set_location(None, cx))
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
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
                "file1": "",
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
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "file1"), cx))
        .await
        .unwrap();
    let buffer_c = project_c
        .update(cx_c, |p, cx| p.open_buffer((worktree_id, "file1"), cx))
        .await
        .unwrap();
    buffer_b.update(cx_b, |buf, cx| buf.edit([(0..0, "i-am-b, ")], None, cx));
    buffer_c.update(cx_c, |buf, cx| buf.edit([(0..0, "i-am-c, ")], None, cx));

    // Open and edit that buffer as the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "file1"), cx))
        .await
        .unwrap();

    buffer_a
        .condition(cx_a, |buf, _| buf.text() == "i-am-c, i-am-b, ")
        .await;
    buffer_a.update(cx_a, |buf, cx| {
        buf.edit([(buf.len()..buf.len(), "i-am-a")], None, cx)
    });

    // Wait for edits to propagate
    buffer_a
        .condition(cx_a, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
        .await;
    buffer_b
        .condition(cx_b, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
        .await;
    buffer_c
        .condition(cx_c, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
        .await;

    // Edit the buffer as the host and concurrently save as guest B.
    let save_b = buffer_b.update(cx_b, |buf, cx| buf.save(cx));
    buffer_a.update(cx_a, |buf, cx| buf.edit([(0..0, "hi-a, ")], None, cx));
    save_b.await.unwrap();
    assert_eq!(
        client_a.fs.load("/a/file1".as_ref()).await.unwrap(),
        "hi-a, i-am-c, i-am-b, i-am-a"
    );
    buffer_a.read_with(cx_a, |buf, _| assert!(!buf.is_dirty()));
    buffer_b.read_with(cx_b, |buf, _| assert!(!buf.is_dirty()));
    buffer_c.condition(cx_c, |buf, _| !buf.is_dirty()).await;

    worktree_a.flush_fs_events(cx_a).await;

    // Make changes on host's file system, see those changes on guest worktrees.
    client_a
        .fs
        .rename(
            "/a/file1".as_ref(),
            "/a/file1-renamed".as_ref(),
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

    worktree_a
        .condition(cx_a, |tree, _| {
            tree.paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>()
                == ["file1-renamed", "file3", "file4"]
        })
        .await;
    worktree_b
        .condition(cx_b, |tree, _| {
            tree.paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>()
                == ["file1-renamed", "file3", "file4"]
        })
        .await;
    worktree_c
        .condition(cx_c, |tree, _| {
            tree.paths()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>()
                == ["file1-renamed", "file3", "file4"]
        })
        .await;

    // Ensure buffer files are updated as well.
    buffer_a
        .condition(cx_a, |buf, _| {
            buf.file().unwrap().path().to_str() == Some("file1-renamed")
        })
        .await;
    buffer_b
        .condition(cx_b, |buf, _| {
            buf.file().unwrap().path().to_str() == Some("file1-renamed")
        })
        .await;
    buffer_c
        .condition(cx_c, |buf, _| {
            buf.file().unwrap().path().to_str() == Some("file1-renamed")
        })
        .await;
}

#[gpui::test(iterations = 10)]
async fn test_git_diff_base_change(
    executor: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    executor.forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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

    client_a
        .fs
        .as_fake()
        .set_index_for_repo(
            Path::new("/dir/.git"),
            &[(Path::new("a.txt"), diff_base.clone())],
        )
        .await;

    // Create the buffer
    let buffer_local_a = project_local
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();

    // Wait for it to catch up to the new diff
    executor.run_until_parked();

    // Smoke test diffing
    buffer_local_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_range(0..4),
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
    executor.run_until_parked();

    // Smoke test diffing
    buffer_remote_a.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_range(0..4),
            &buffer,
            &diff_base,
            &[(1..2, "", "two\n")],
        );
    });

    client_a
        .fs
        .as_fake()
        .set_index_for_repo(
            Path::new("/dir/.git"),
            &[(Path::new("a.txt"), new_diff_base.clone())],
        )
        .await;

    // Wait for buffer_local_a to receive it
    executor.run_until_parked();

    // Smoke test new diffing
    buffer_local_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(new_diff_base.as_ref()));

        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_range(0..4),
            &buffer,
            &diff_base,
            &[(2..3, "", "three\n")],
        );
    });

    // Smoke test B
    buffer_remote_a.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(new_diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_range(0..4),
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

    client_a
        .fs
        .as_fake()
        .set_index_for_repo(
            Path::new("/dir/sub/.git"),
            &[(Path::new("b.txt"), diff_base.clone())],
        )
        .await;

    // Create the buffer
    let buffer_local_b = project_local
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "sub/b.txt"), cx))
        .await
        .unwrap();

    // Wait for it to catch up to the new diff
    executor.run_until_parked();

    // Smoke test diffing
    buffer_local_b.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_range(0..4),
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
    executor.run_until_parked();

    // Smoke test diffing
    buffer_remote_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_range(0..4),
            &buffer,
            &diff_base,
            &[(1..2, "", "two\n")],
        );
    });

    client_a
        .fs
        .as_fake()
        .set_index_for_repo(
            Path::new("/dir/sub/.git"),
            &[(Path::new("b.txt"), new_diff_base.clone())],
        )
        .await;

    // Wait for buffer_local_b to receive it
    executor.run_until_parked();

    // Smoke test new diffing
    buffer_local_b.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(new_diff_base.as_ref()));
        println!("{:?}", buffer.as_rope().to_string());
        println!("{:?}", buffer.diff_base());
        println!(
            "{:?}",
            buffer
                .snapshot()
                .git_diff_hunks_in_range(0..4)
                .collect::<Vec<_>>()
        );

        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_range(0..4),
            &buffer,
            &diff_base,
            &[(2..3, "", "three\n")],
        );
    });

    // Smoke test B
    buffer_remote_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.diff_base(), Some(new_diff_base.as_ref()));
        git::diff::assert_hunks(
            buffer.snapshot().git_diff_hunks_in_range(0..4),
            &buffer,
            &diff_base,
            &[(2..3, "", "three\n")],
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_fs_operations(
    executor: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    executor.forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
async fn test_buffer_conflict_after_save(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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

    buffer_b.update(cx_b, |buf, cx| buf.save(cx)).await.unwrap();
    buffer_b
        .condition(cx_b, |buffer_b, _| !buffer_b.is_dirty())
        .await;
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
async fn test_buffer_reloading(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    buffer_b
        .condition(cx_b, |buf, _| {
            buf.text() == new_contents.to_string() && !buf.is_dirty()
        })
        .await;
    buffer_b.read_with(cx_b, |buf, _| {
        assert!(!buf.is_dirty());
        assert!(!buf.has_conflict());
        assert_eq!(buf.line_ending(), LineEnding::Windows);
    });
}

#[gpui::test(iterations = 10)]
async fn test_editing_while_guest_opens_buffer(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    buffer_b.condition(cx_b, |buf, _| buf.text() == text).await;
}

#[gpui::test(iterations = 10)]
async fn test_leaving_worktree_while_opening_buffer(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    project_a
        .condition(cx_a, |p, _| p.collaborators().len() == 1)
        .await;

    // Begin opening a buffer as client B, but leave the project before the open completes.
    let buffer_b = cx_b
        .background()
        .spawn(project_b.update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx)));
    cx_b.update(|_| drop(project_b));
    drop(buffer_b);

    // See that the guest has left.
    project_a
        .condition(cx_a, |p, _| p.collaborators().is_empty())
        .await;
}

#[gpui::test(iterations = 10)]
async fn test_canceling_buffer_opening(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();

    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    let buffer_b = project_b.update(cx_b, |p, cx| p.open_buffer_by_id(buffer_a.id() as u64, cx));
    deterministic.simulate_random_delay().await;
    drop(buffer_b);

    // Try opening the same buffer again as client B, and ensure we can
    // still do it despite the cancellation above.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer_by_id(buffer_a.id() as u64, cx))
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
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    let project_c = client_c.build_remote_project(project_id, cx_c).await;

    // Client A sees that a guest has joined.
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });
    project_b.read_with(cx_b, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });
    project_c.read_with(cx_c, |project, _| {
        assert_eq!(project.collaborators().len(), 2);
    });

    // Drop client B's connection and ensure client A and client C observe client B leaving the project.
    client_b.disconnect(&cx_b.to_async()).unwrap();
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 1);
    });
    project_b.read_with(cx_b, |project, _| {
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
            client_b.project_store.clone(),
            client_b.language_registry.clone(),
            FakeFs::new(cx.background()),
            cx,
        )
    })
    .await
    .unwrap_err();

    // Simulate connection loss for client C and ensure client A observes client C leaving the project.
    client_c.wait_for_current_user(cx_c).await;
    server.disconnect_client(client_c.current_user_id(cx_c));
    cx_a.foreground().advance_clock(rpc::RECEIVE_TIMEOUT);
    deterministic.run_until_parked();
    project_a.read_with(cx_a, |project, _| {
        assert_eq!(project.collaborators().len(), 0);
    });
    project_b.read_with(cx_b, |project, _| {
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
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Cause the language server to start.
    let _buffer = cx_a
        .background()
        .spawn(project_a.update(cx_a, |project, cx| {
            project.open_buffer(
                ProjectPath {
                    worktree_id,
                    path: Path::new("other.rs").into(),
                },
                cx,
            )
        }))
        .await
        .unwrap();

    // Join the worktree as client B.
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

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
                severity: Some(lsp::DiagnosticSeverity::ERROR),
                range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 7)),
                message: "message 1".to_string(),
                ..Default::default()
            }],
        },
    );

    // Wait for server to see the diagnostics update.
    deterministic.run_until_parked();
    {
        let store = server.store.lock().await;
        let project = store.project(ProjectId::from_proto(project_id)).unwrap();
        let worktree = project.worktrees.get(&worktree_id.to_proto()).unwrap();
        assert!(!worktree.diagnostic_summaries.is_empty());
    }

    // Ensure client B observes the new diagnostics.
    project_b.read_with(cx_b, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(cx).collect::<Vec<_>>(),
            &[(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("a.rs")),
                },
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
    deterministic.run_until_parked();
    project_c.read_with(cx_c, |project, cx| {
        assert_eq!(
            project.diagnostic_summaries(cx).collect::<Vec<_>>(),
            &[(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("a.rs")),
                },
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 0,
                    ..Default::default()
                },
            )]
        )
    });

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
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 1,
                    ..Default::default()
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
                DiagnosticSummary {
                    error_count: 1,
                    warning_count: 1,
                    ..Default::default()
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
                        group_id: 1,
                        message: "message 1".to_string(),
                        severity: lsp::DiagnosticSeverity::ERROR,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(0, 10)..Point::new(0, 13),
                    diagnostic: Diagnostic {
                        group_id: 2,
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
async fn test_collaborating_with_completion(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    let (_, window_b) = cx_b.add_window(|_| EmptyView);
    let editor_b = cx_b.add_view(&window_b, |cx| {
        Editor::for_buffer(buffer_b.clone(), Some(project_b.clone()), cx)
    });

    let fake_language_server = fake_language_servers.next().await.unwrap();
    buffer_b
        .condition(cx_b, |buffer, _| !buffer.completion_triggers().is_empty())
        .await;

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
    buffer_a
        .condition(cx_a, |buffer, _| buffer.text() == "fn main() { a. }")
        .await;

    // Confirm a completion on the guest.
    editor_b
        .condition(cx_b, |editor, _| editor.context_menu_visible())
        .await;
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
    buffer_a
        .condition(cx_a, |buffer, _| {
            buffer.text() == "use d::SomeTrait;\nfn main() { a.first_method() }"
        })
        .await;
    buffer_b
        .condition(cx_b, |buffer, _| {
            buffer.text() == "use d::SomeTrait;\nfn main() { a.first_method() }"
        })
        .await;
}

#[gpui::test(iterations = 10)]
async fn test_reloading_buffer_manually(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    buffer_a
        .condition(cx_a, |buffer, _| buffer.text() == "let six = 6;")
        .await;

    client_a
        .fs
        .save(
            "/a/a.rs".as_ref(),
            &Rope::from("let seven = 7;"),
            LineEnding::Unix,
        )
        .await
        .unwrap();
    buffer_a
        .condition(cx_a, |buffer, _| buffer.has_conflict())
        .await;
    buffer_b
        .condition(cx_b, |buffer, _| buffer.has_conflict())
        .await;

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
async fn test_formatting_buffer(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    use project::FormatTrigger;

    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    assert_eq!(
        buffer_b.read_with(cx_b, |buffer, _| buffer.text()),
        "let honey = \"two\""
    );

    // Ensure buffer can be formatted using an external command. Notice how the
    // host's configuration is honored as opposed to using the guest's settings.
    cx_a.update(|cx| {
        cx.update_global(|settings: &mut Settings, _| {
            settings.editor_defaults.formatter = Some(Formatter::External {
                command: "awk".to_string(),
                arguments: vec!["{sub(/two/,\"{buffer_path}\")}1".to_string()],
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
async fn test_definition(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
async fn test_references(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
            Path::new("three.rs")
        );

        assert_eq!(references[0].range.to_offset(two_buffer), 24..27);
        assert_eq!(references[1].range.to_offset(two_buffer), 35..38);
        assert_eq!(references[2].range.to_offset(three_buffer), 37..40);
    });
}

#[gpui::test(iterations = 10)]
async fn test_project_search(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
            project.search(SearchQuery::text("world", false, false), cx)
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
async fn test_document_highlights(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
async fn test_lsp_hover(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
                    language: None,
                },
                project::HoverBlock {
                    text: "let foo = 42;".to_string(),
                    language: Some("Rust".to_string()),
                }
            ]
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_project_symbols(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    fake_language_server.handle_request::<lsp::request::WorkspaceSymbol, _, _>(|_, _| async move {
        #[allow(deprecated)]
        Ok(Some(vec![lsp::SymbolInformation {
            name: "TWO".into(),
            location: lsp::Location {
                uri: lsp::Url::from_file_path("/code/crate-2/two.rs").unwrap(),
                range: lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
            },
            kind: lsp::SymbolKind::CONSTANT,
            tags: None,
            container_name: None,
            deprecated: None,
        }]))
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
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    mut rng: StdRng,
) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    cx_a.foreground().forbid_parking();
    cx_b.update(editor::init);
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
    let (_window_b, workspace_b) =
        cx_b.add_window(|cx| Workspace::new(project_b.clone(), |_, _| unimplemented!(), cx));
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), true, cx)
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
    editor_b
        .condition(cx_b, |editor, _| editor.context_menu_visible())
        .await;

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
async fn test_collaborating_with_renames(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    cx_b.update(editor::init);
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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

    let (_window_b, workspace_b) =
        cx_b.add_window(|cx| Workspace::new(project_b.clone(), |_, _| unimplemented!(), cx));
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "one.rs"), true, cx)
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

    cx_b.update(editor::init);
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
async fn test_basic_chat(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    // Create an org that includes these 2 users.
    let db = &server.app_state.db;
    let org_id = db.create_org("Test Org", "test-org").await.unwrap();
    db.add_org_member(org_id, client_a.current_user_id(cx_a), false)
        .await
        .unwrap();
    db.add_org_member(org_id, client_b.current_user_id(cx_b), false)
        .await
        .unwrap();

    // Create a channel that includes all the users.
    let channel_id = db.create_org_channel(org_id, "test-channel").await.unwrap();
    db.add_channel_member(channel_id, client_a.current_user_id(cx_a), false)
        .await
        .unwrap();
    db.add_channel_member(channel_id, client_b.current_user_id(cx_b), false)
        .await
        .unwrap();
    db.create_channel_message(
        channel_id,
        client_b.current_user_id(cx_b),
        "hello A, it's B.",
        OffsetDateTime::now_utc(),
        1,
    )
    .await
    .unwrap();

    let channels_a =
        cx_a.add_model(|cx| ChannelList::new(client_a.user_store.clone(), client_a.clone(), cx));
    channels_a
        .condition(cx_a, |list, _| list.available_channels().is_some())
        .await;
    channels_a.read_with(cx_a, |list, _| {
        assert_eq!(
            list.available_channels().unwrap(),
            &[ChannelDetails {
                id: channel_id.to_proto(),
                name: "test-channel".to_string()
            }]
        )
    });
    let channel_a = channels_a.update(cx_a, |this, cx| {
        this.get_channel(channel_id.to_proto(), cx).unwrap()
    });
    channel_a.read_with(cx_a, |channel, _| assert!(channel.messages().is_empty()));
    channel_a
        .condition(cx_a, |channel, _| {
            channel_messages(channel)
                == [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
        })
        .await;

    let channels_b =
        cx_b.add_model(|cx| ChannelList::new(client_b.user_store.clone(), client_b.clone(), cx));
    channels_b
        .condition(cx_b, |list, _| list.available_channels().is_some())
        .await;
    channels_b.read_with(cx_b, |list, _| {
        assert_eq!(
            list.available_channels().unwrap(),
            &[ChannelDetails {
                id: channel_id.to_proto(),
                name: "test-channel".to_string()
            }]
        )
    });

    let channel_b = channels_b.update(cx_b, |this, cx| {
        this.get_channel(channel_id.to_proto(), cx).unwrap()
    });
    channel_b.read_with(cx_b, |channel, _| assert!(channel.messages().is_empty()));
    channel_b
        .condition(cx_b, |channel, _| {
            channel_messages(channel)
                == [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
        })
        .await;

    channel_a
        .update(cx_a, |channel, cx| {
            channel
                .send_message("oh, hi B.".to_string(), cx)
                .unwrap()
                .detach();
            let task = channel.send_message("sup".to_string(), cx).unwrap();
            assert_eq!(
                channel_messages(channel),
                &[
                    ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                    ("user_a".to_string(), "oh, hi B.".to_string(), true),
                    ("user_a".to_string(), "sup".to_string(), true)
                ]
            );
            task
        })
        .await
        .unwrap();

    channel_b
        .condition(cx_b, |channel, _| {
            channel_messages(channel)
                == [
                    ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                    ("user_a".to_string(), "oh, hi B.".to_string(), false),
                    ("user_a".to_string(), "sup".to_string(), false),
                ]
        })
        .await;

    assert_eq!(
        server
            .store()
            .await
            .channel(channel_id)
            .unwrap()
            .connection_ids
            .len(),
        2
    );
    cx_b.update(|_| drop(channel_b));
    server
        .condition(|state| state.channel(channel_id).unwrap().connection_ids.len() == 1)
        .await;

    cx_a.update(|_| drop(channel_a));
    server
        .condition(|state| state.channel(channel_id).is_none())
        .await;
}

#[gpui::test(iterations = 10)]
async fn test_chat_message_validation(cx_a: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;

    let db = &server.app_state.db;
    let org_id = db.create_org("Test Org", "test-org").await.unwrap();
    let channel_id = db.create_org_channel(org_id, "test-channel").await.unwrap();
    db.add_org_member(org_id, client_a.current_user_id(cx_a), false)
        .await
        .unwrap();
    db.add_channel_member(channel_id, client_a.current_user_id(cx_a), false)
        .await
        .unwrap();

    let channels_a =
        cx_a.add_model(|cx| ChannelList::new(client_a.user_store.clone(), client_a.clone(), cx));
    channels_a
        .condition(cx_a, |list, _| list.available_channels().is_some())
        .await;
    let channel_a = channels_a.update(cx_a, |this, cx| {
        this.get_channel(channel_id.to_proto(), cx).unwrap()
    });

    // Messages aren't allowed to be too long.
    channel_a
        .update(cx_a, |channel, cx| {
            let long_body = "this is long.\n".repeat(1024);
            channel.send_message(long_body, cx).unwrap()
        })
        .await
        .unwrap_err();

    // Messages aren't allowed to be blank.
    channel_a.update(cx_a, |channel, cx| {
        channel.send_message(String::new(), cx).unwrap_err()
    });

    // Leading and trailing whitespace are trimmed.
    channel_a
        .update(cx_a, |channel, cx| {
            channel
                .send_message("\n surrounded by whitespace  \n".to_string(), cx)
                .unwrap()
        })
        .await
        .unwrap();
    assert_eq!(
        db.get_channel_messages(channel_id, 10, None)
            .await
            .unwrap()
            .iter()
            .map(|m| &m.body)
            .collect::<Vec<_>>(),
        &["surrounded by whitespace"]
    );
}

#[gpui::test(iterations = 10)]
async fn test_chat_reconnection(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let mut status_b = client_b.status();

    // Create an org that includes these 2 users.
    let db = &server.app_state.db;
    let org_id = db.create_org("Test Org", "test-org").await.unwrap();
    db.add_org_member(org_id, client_a.current_user_id(cx_a), false)
        .await
        .unwrap();
    db.add_org_member(org_id, client_b.current_user_id(cx_b), false)
        .await
        .unwrap();

    // Create a channel that includes all the users.
    let channel_id = db.create_org_channel(org_id, "test-channel").await.unwrap();
    db.add_channel_member(channel_id, client_a.current_user_id(cx_a), false)
        .await
        .unwrap();
    db.add_channel_member(channel_id, client_b.current_user_id(cx_b), false)
        .await
        .unwrap();
    db.create_channel_message(
        channel_id,
        client_b.current_user_id(cx_b),
        "hello A, it's B.",
        OffsetDateTime::now_utc(),
        2,
    )
    .await
    .unwrap();

    let channels_a =
        cx_a.add_model(|cx| ChannelList::new(client_a.user_store.clone(), client_a.clone(), cx));
    channels_a
        .condition(cx_a, |list, _| list.available_channels().is_some())
        .await;

    channels_a.read_with(cx_a, |list, _| {
        assert_eq!(
            list.available_channels().unwrap(),
            &[ChannelDetails {
                id: channel_id.to_proto(),
                name: "test-channel".to_string()
            }]
        )
    });
    let channel_a = channels_a.update(cx_a, |this, cx| {
        this.get_channel(channel_id.to_proto(), cx).unwrap()
    });
    channel_a.read_with(cx_a, |channel, _| assert!(channel.messages().is_empty()));
    channel_a
        .condition(cx_a, |channel, _| {
            channel_messages(channel)
                == [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
        })
        .await;

    let channels_b =
        cx_b.add_model(|cx| ChannelList::new(client_b.user_store.clone(), client_b.clone(), cx));
    channels_b
        .condition(cx_b, |list, _| list.available_channels().is_some())
        .await;
    channels_b.read_with(cx_b, |list, _| {
        assert_eq!(
            list.available_channels().unwrap(),
            &[ChannelDetails {
                id: channel_id.to_proto(),
                name: "test-channel".to_string()
            }]
        )
    });

    let channel_b = channels_b.update(cx_b, |this, cx| {
        this.get_channel(channel_id.to_proto(), cx).unwrap()
    });
    channel_b.read_with(cx_b, |channel, _| assert!(channel.messages().is_empty()));
    channel_b
        .condition(cx_b, |channel, _| {
            channel_messages(channel)
                == [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
        })
        .await;

    // Disconnect client B, ensuring we can still access its cached channel data.
    server.forbid_connections();
    server.disconnect_client(client_b.current_user_id(cx_b));
    cx_b.foreground().advance_clock(rpc::RECEIVE_TIMEOUT);
    while !matches!(
        status_b.next().await,
        Some(client::Status::ReconnectionError { .. })
    ) {}

    channels_b.read_with(cx_b, |channels, _| {
        assert_eq!(
            channels.available_channels().unwrap(),
            [ChannelDetails {
                id: channel_id.to_proto(),
                name: "test-channel".to_string()
            }]
        )
    });
    channel_b.read_with(cx_b, |channel, _| {
        assert_eq!(
            channel_messages(channel),
            [("user_b".to_string(), "hello A, it's B.".to_string(), false)]
        )
    });

    // Send a message from client B while it is disconnected.
    channel_b
        .update(cx_b, |channel, cx| {
            let task = channel
                .send_message("can you see this?".to_string(), cx)
                .unwrap();
            assert_eq!(
                channel_messages(channel),
                &[
                    ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                    ("user_b".to_string(), "can you see this?".to_string(), true)
                ]
            );
            task
        })
        .await
        .unwrap_err();

    // Send a message from client A while B is disconnected.
    channel_a
        .update(cx_a, |channel, cx| {
            channel
                .send_message("oh, hi B.".to_string(), cx)
                .unwrap()
                .detach();
            let task = channel.send_message("sup".to_string(), cx).unwrap();
            assert_eq!(
                channel_messages(channel),
                &[
                    ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                    ("user_a".to_string(), "oh, hi B.".to_string(), true),
                    ("user_a".to_string(), "sup".to_string(), true)
                ]
            );
            task
        })
        .await
        .unwrap();

    // Give client B a chance to reconnect.
    server.allow_connections();
    cx_b.foreground().advance_clock(Duration::from_secs(10));

    // Verify that B sees the new messages upon reconnection, as well as the message client B
    // sent while offline.
    channel_b
        .condition(cx_b, |channel, _| {
            channel_messages(channel)
                == [
                    ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                    ("user_a".to_string(), "oh, hi B.".to_string(), false),
                    ("user_a".to_string(), "sup".to_string(), false),
                    ("user_b".to_string(), "can you see this?".to_string(), false),
                ]
        })
        .await;

    // Ensure client A and B can communicate normally after reconnection.
    channel_a
        .update(cx_a, |channel, cx| {
            channel.send_message("you online?".to_string(), cx).unwrap()
        })
        .await
        .unwrap();
    channel_b
        .condition(cx_b, |channel, _| {
            channel_messages(channel)
                == [
                    ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                    ("user_a".to_string(), "oh, hi B.".to_string(), false),
                    ("user_a".to_string(), "sup".to_string(), false),
                    ("user_b".to_string(), "can you see this?".to_string(), false),
                    ("user_a".to_string(), "you online?".to_string(), false),
                ]
        })
        .await;

    channel_b
        .update(cx_b, |channel, cx| {
            channel.send_message("yep".to_string(), cx).unwrap()
        })
        .await
        .unwrap();
    channel_a
        .condition(cx_a, |channel, _| {
            channel_messages(channel)
                == [
                    ("user_b".to_string(), "hello A, it's B.".to_string(), false),
                    ("user_a".to_string(), "oh, hi B.".to_string(), false),
                    ("user_a".to_string(), "sup".to_string(), false),
                    ("user_b".to_string(), "can you see this?".to_string(), false),
                    ("user_a".to_string(), "you online?".to_string(), false),
                    ("user_b".to_string(), "yep".to_string(), false),
                ]
        })
        .await;
}

#[gpui::test(iterations = 10)]
async fn test_contacts(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    cx_a.foreground().forbid_parking();
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);

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

    server.disconnect_client(client_c.current_user_id(cx_c));
    server.forbid_connections();
    deterministic.advance_clock(rpc::RECEIVE_TIMEOUT);
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
            ("user_c".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "free")
        ]
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
            ("user_c".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "free")
        ]
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
            ("user_c".to_string(), "online", "busy")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "online", "busy"),
            ("user_b".to_string(), "online", "busy")
        ]
    );

    active_call_a.update(cx_a, |call, cx| call.hang_up(cx).unwrap());
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

    server.forbid_connections();
    server.disconnect_client(client_a.current_user_id(cx_a));
    deterministic.advance_clock(rpc::RECEIVE_TIMEOUT);
    assert_eq!(contacts(&client_a, cx_a), []);
    assert_eq!(
        contacts(&client_b, cx_b),
        [
            ("user_a".to_string(), "offline", "free"),
            ("user_c".to_string(), "online", "free")
        ]
    );
    assert_eq!(
        contacts(&client_c, cx_c),
        [
            ("user_a".to_string(), "offline", "free"),
            ("user_b".to_string(), "online", "free")
        ]
    );

    #[allow(clippy::type_complexity)]
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
    executor: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_a2: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_b2: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_c2: &mut TestAppContext,
) {
    cx_a.foreground().forbid_parking();

    // Connect to a server as 3 clients.
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
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
        .user_store
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
        .user_store
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
        client.disconnect(&cx.to_async()).unwrap();
        client.clear_contacts(cx).await;
        client
            .authenticate_and_connect(false, &cx.to_async())
            .await
            .unwrap();
    }
}

#[gpui::test(iterations = 10)]
async fn test_following(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    cx_a.update(editor::init);
    cx_b.update(editor::init);

    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

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
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Client A opens some editors.
    let workspace_a = client_a.build_workspace(&project_a, cx_a);
    let pane_a = workspace_a.read_with(cx_a, |workspace, _| workspace.active_pane().clone());
    let editor_a1 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let editor_a2 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "2.txt"), true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B opens an editor.
    let workspace_b = client_b.build_workspace(&project_b, cx_b);
    let editor_b1 = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let client_a_id = project_b.read_with(cx_b, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });
    let client_b_id = project_a.read_with(cx_a, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });

    // When client B starts following client A, all visible view states are replicated to client B.
    editor_a1.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([0..1]))
    });
    editor_a2.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([2..3]))
    });
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace
                .toggle_follow(&ToggleFollow(client_a_id), cx)
                .unwrap()
        })
        .await
        .unwrap();

    let editor_b2 = workspace_b.read_with(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });
    assert!(cx_b.read(|cx| editor_b2.is_focused(cx)));
    assert_eq!(
        editor_b2.read_with(cx_b, |editor, cx| editor.project_path(cx)),
        Some((worktree_id, "2.txt").into())
    );
    assert_eq!(
        editor_b2.read_with(cx_b, |editor, cx| editor.selections.ranges(cx)),
        vec![2..3]
    );
    assert_eq!(
        editor_b1.read_with(cx_b, |editor, cx| editor.selections.ranges(cx)),
        vec![0..1]
    );

    // When client A activates a different editor, client B does so as well.
    workspace_a.update(cx_a, |workspace, cx| {
        workspace.activate_item(&editor_a1, cx)
    });
    workspace_b
        .condition(cx_b, |workspace, cx| {
            workspace.active_item(cx).unwrap().id() == editor_b1.id()
        })
        .await;

    // When client A navigates back and forth, client B does so as well.
    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace::Pane::go_back(workspace, None, cx)
        })
        .await;
    workspace_b
        .condition(cx_b, |workspace, cx| {
            workspace.active_item(cx).unwrap().id() == editor_b2.id()
        })
        .await;

    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace::Pane::go_forward(workspace, None, cx)
        })
        .await;
    workspace_b
        .condition(cx_b, |workspace, cx| {
            workspace.active_item(cx).unwrap().id() == editor_b1.id()
        })
        .await;

    // Changes to client A's editor are reflected on client B.
    editor_a1.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([1..1, 2..2]));
    });
    editor_b1
        .condition(cx_b, |editor, cx| {
            editor.selections.ranges(cx) == vec![1..1, 2..2]
        })
        .await;

    editor_a1.update(cx_a, |editor, cx| editor.set_text("TWO", cx));
    editor_b1
        .condition(cx_b, |editor, cx| editor.text(cx) == "TWO")
        .await;

    editor_a1.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([3..3]));
        editor.set_scroll_position(vec2f(0., 100.), cx);
    });
    editor_b1
        .condition(cx_b, |editor, cx| {
            editor.selections.ranges(cx) == vec![3..3]
        })
        .await;

    // After unfollowing, client B stops receiving updates from client A.
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.unfollow(&workspace.active_pane().clone(), cx)
    });
    workspace_a.update(cx_a, |workspace, cx| {
        workspace.activate_item(&editor_a2, cx)
    });
    cx_a.foreground().run_until_parked();
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
            workspace
                .toggle_follow(&ToggleFollow(client_b_id), cx)
                .unwrap()
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, _| workspace.leader_for_pane(&pane_a)),
        Some(client_b_id)
    );
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, cx| workspace
            .active_item(cx)
            .unwrap()
            .id()),
        editor_a1.id()
    );

    // Following interrupts when client B disconnects.
    client_b.disconnect(&cx_b.to_async()).unwrap();
    cx_a.foreground().run_until_parked();
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, _| workspace.leader_for_pane(&pane_a)),
        None
    );
}

#[gpui::test(iterations = 10)]
async fn test_peers_following_each_other(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    cx_a.update(editor::init);
    cx_b.update(editor::init);

    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

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
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Client B joins the project.
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Client A opens some editors.
    let workspace_a = client_a.build_workspace(&project_a, cx_a);
    let pane_a1 = workspace_a.read_with(cx_a, |workspace, _| workspace.active_pane().clone());
    let _editor_a1 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), true, cx)
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
            workspace.open_path((worktree_id, "2.txt"), true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Clients A and B follow each other in split panes
    workspace_a.update(cx_a, |workspace, cx| {
        workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
        let pane_a1 = pane_a1.clone();
        cx.defer(move |workspace, _| {
            assert_ne!(*workspace.active_pane(), pane_a1);
        });
    });
    workspace_a
        .update(cx_a, |workspace, cx| {
            let leader_id = *project_a.read(cx).collaborators().keys().next().unwrap();
            workspace
                .toggle_follow(&workspace::ToggleFollow(leader_id), cx)
                .unwrap()
        })
        .await
        .unwrap();
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
        let pane_b1 = pane_b1.clone();
        cx.defer(move |workspace, _| {
            assert_ne!(*workspace.active_pane(), pane_b1);
        });
    });
    workspace_b
        .update(cx_b, |workspace, cx| {
            let leader_id = *project_b.read(cx).collaborators().keys().next().unwrap();
            workspace
                .toggle_follow(&workspace::ToggleFollow(leader_id), cx)
                .unwrap()
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
            workspace.open_path((worktree_id, "3.txt"), true, cx)
        })
        .await
        .unwrap();
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.activate_next_pane(cx);
    });

    workspace_b
        .update(cx_b, |workspace, cx| {
            assert_eq!(*workspace.active_pane(), pane_b1);
            workspace.open_path((worktree_id, "4.txt"), true, cx)
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
async fn test_auto_unfollowing(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    cx_a.foreground().forbid_parking();
    cx_a.update(editor::init);
    cx_b.update(editor::init);

    // 2 clients connect to a server.
    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

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
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    // Client A opens some editors.
    let workspace_a = client_a.build_workspace(&project_a, cx_a);
    let _editor_a1 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), true, cx)
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
            workspace
                .toggle_follow(&ToggleFollow(leader_id), cx)
                .unwrap()
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
            workspace
                .toggle_follow(&ToggleFollow(leader_id), cx)
                .unwrap()
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
            workspace
                .toggle_follow(&ToggleFollow(leader_id), cx)
                .unwrap()
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
            workspace
                .toggle_follow(&ToggleFollow(leader_id), cx)
                .unwrap()
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
            workspace.open_path((worktree_id, "2.txt"), true, cx)
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
    cx_a.update(editor::init);
    cx_b.update(editor::init);

    let mut server = TestServer::start(cx_a.foreground(), cx_a.background()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

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
        workspace
            .toggle_follow(&ToggleFollow(client_b_id), cx)
            .unwrap()
    });
    let b_follow_a = workspace_b.update(cx_b, |workspace, cx| {
        workspace
            .toggle_follow(&ToggleFollow(client_a_id), cx)
            .unwrap()
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

#[gpui::test(iterations = 100)]
async fn test_random_collaboration(
    cx: &mut TestAppContext,
    deterministic: Arc<Deterministic>,
    rng: StdRng,
) {
    deterministic.forbid_parking();
    let max_peers = env::var("MAX_PEERS")
        .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
        .unwrap_or(5);
    assert!(max_peers <= 5);

    let max_operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let rng = Arc::new(Mutex::new(rng));

    let guest_lang_registry = Arc::new(LanguageRegistry::test());
    let host_language_registry = Arc::new(LanguageRegistry::test());

    let fs = FakeFs::new(cx.background());
    fs.insert_tree("/_collab", json!({"init": ""})).await;

    let mut server = TestServer::start(cx.foreground(), cx.background()).await;
    let db = server.app_state.db.clone();

    let room_creator_user_id = db
        .create_user(
            "room-creator@example.com",
            false,
            NewUserParams {
                github_login: "room-creator".into(),
                github_user_id: 0,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;
    let mut available_guests = vec![
        "guest-1".to_string(),
        "guest-2".to_string(),
        "guest-3".to_string(),
        "guest-4".to_string(),
    ];

    for (ix, username) in Some(&"host".to_string())
        .into_iter()
        .chain(&available_guests)
        .enumerate()
    {
        let user_id = db
            .create_user(
                &format!("{username}@example.com"),
                false,
                NewUserParams {
                    github_login: username.into(),
                    github_user_id: (ix + 1) as i32,
                    invite_count: 0,
                },
            )
            .await
            .unwrap()
            .user_id;
        server
            .app_state
            .db
            .send_contact_request(user_id, room_creator_user_id)
            .await
            .unwrap();
        server
            .app_state
            .db
            .respond_to_contact_request(room_creator_user_id, user_id, true)
            .await
            .unwrap();
    }

    let _room_creator = server.create_client(cx, "room-creator").await;
    let active_call = cx.read(ActiveCall::global);

    let mut clients = Vec::new();
    let mut user_ids = Vec::new();
    let mut op_start_signals = Vec::new();

    let mut next_entity_id = 100000;
    let mut host_cx = TestAppContext::new(
        cx.foreground_platform(),
        cx.platform(),
        deterministic.build_foreground(next_entity_id),
        deterministic.build_background(),
        cx.font_cache(),
        cx.leak_detector(),
        next_entity_id,
        cx.function_name.clone(),
    );
    let host = server.create_client(&mut host_cx, "host").await;
    let host_project = host_cx.update(|cx| {
        Project::local(
            host.client.clone(),
            host.user_store.clone(),
            host.project_store.clone(),
            host_language_registry.clone(),
            fs.clone(),
            cx,
        )
    });

    let (collab_worktree, _) = host_project
        .update(&mut host_cx, |project, cx| {
            project.find_or_create_local_worktree("/_collab", true, cx)
        })
        .await
        .unwrap();
    collab_worktree
        .read_with(&host_cx, |tree, _| tree.as_local().unwrap().scan_complete())
        .await;

    // Set up fake language servers.
    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        None,
    );
    let _fake_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            name: "the-fake-language-server",
            capabilities: lsp::LanguageServer::full_capabilities(),
            initializer: Some(Box::new({
                let rng = rng.clone();
                let fs = fs.clone();
                let project = host_project.downgrade();
                move |fake_server: &mut FakeLanguageServer| {
                    fake_server.handle_request::<lsp::request::Completion, _, _>(
                        |_, _| async move {
                            Ok(Some(lsp::CompletionResponse::Array(vec![
                                lsp::CompletionItem {
                                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                        range: lsp::Range::new(
                                            lsp::Position::new(0, 0),
                                            lsp::Position::new(0, 0),
                                        ),
                                        new_text: "the-new-text".to_string(),
                                    })),
                                    ..Default::default()
                                },
                            ])))
                        },
                    );

                    fake_server.handle_request::<lsp::request::CodeActionRequest, _, _>(
                        |_, _| async move {
                            Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                                lsp::CodeAction {
                                    title: "the-code-action".to_string(),
                                    ..Default::default()
                                },
                            )]))
                        },
                    );

                    fake_server.handle_request::<lsp::request::PrepareRenameRequest, _, _>(
                        |params, _| async move {
                            Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                                params.position,
                                params.position,
                            ))))
                        },
                    );

                    fake_server.handle_request::<lsp::request::GotoDefinition, _, _>({
                        let fs = fs.clone();
                        let rng = rng.clone();
                        move |_, _| {
                            let fs = fs.clone();
                            let rng = rng.clone();
                            async move {
                                let files = fs.files().await;
                                let mut rng = rng.lock();
                                let count = rng.gen_range::<usize, _>(1..3);
                                let files = (0..count)
                                    .map(|_| files.choose(&mut *rng).unwrap())
                                    .collect::<Vec<_>>();
                                log::info!("LSP: Returning definitions in files {:?}", &files);
                                Ok(Some(lsp::GotoDefinitionResponse::Array(
                                    files
                                        .into_iter()
                                        .map(|file| lsp::Location {
                                            uri: lsp::Url::from_file_path(file).unwrap(),
                                            range: Default::default(),
                                        })
                                        .collect(),
                                )))
                            }
                        }
                    });

                    fake_server.handle_request::<lsp::request::DocumentHighlightRequest, _, _>({
                        let rng = rng.clone();
                        let project = project;
                        move |params, mut cx| {
                            let highlights = if let Some(project) = project.upgrade(&cx) {
                                project.update(&mut cx, |project, cx| {
                                    let path = params
                                        .text_document_position_params
                                        .text_document
                                        .uri
                                        .to_file_path()
                                        .unwrap();
                                    let (worktree, relative_path) =
                                        project.find_local_worktree(&path, cx)?;
                                    let project_path =
                                        ProjectPath::from((worktree.read(cx).id(), relative_path));
                                    let buffer =
                                        project.get_open_buffer(&project_path, cx)?.read(cx);

                                    let mut highlights = Vec::new();
                                    let highlight_count = rng.lock().gen_range(1..=5);
                                    let mut prev_end = 0;
                                    for _ in 0..highlight_count {
                                        let range =
                                            buffer.random_byte_range(prev_end, &mut *rng.lock());

                                        highlights.push(lsp::DocumentHighlight {
                                            range: range_to_lsp(range.to_point_utf16(buffer)),
                                            kind: Some(lsp::DocumentHighlightKind::READ),
                                        });
                                        prev_end = range.end;
                                    }
                                    Some(highlights)
                                })
                            } else {
                                None
                            };
                            async move { Ok(highlights) }
                        }
                    });
                }
            })),
            ..Default::default()
        }))
        .await;
    host_language_registry.add(Arc::new(language));

    let host_user_id = host.current_user_id(&host_cx);
    active_call
        .update(cx, |call, cx| {
            call.invite(host_user_id.to_proto(), None, cx)
        })
        .await
        .unwrap();
    active_call.read_with(cx, |call, cx| call.room().unwrap().read(cx).id());
    deterministic.run_until_parked();
    let host_active_call = host_cx.read(ActiveCall::global);
    host_active_call
        .update(&mut host_cx, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    let host_project_id = host_active_call
        .update(&mut host_cx, |call, cx| {
            call.share_project(host_project.clone(), cx)
        })
        .await
        .unwrap();

    let op_start_signal = futures::channel::mpsc::unbounded();
    user_ids.push(host_user_id);
    op_start_signals.push(op_start_signal.0);
    clients.push(host_cx.foreground().spawn(host.simulate_host(
        host_project,
        op_start_signal.1,
        rng.clone(),
        host_cx,
    )));

    let disconnect_host_at = if rng.lock().gen_bool(0.2) {
        rng.lock().gen_range(0..max_operations)
    } else {
        max_operations
    };

    let mut operations = 0;
    while operations < max_operations {
        if operations == disconnect_host_at {
            server.disconnect_client(user_ids[0]);
            deterministic.advance_clock(RECEIVE_TIMEOUT);
            drop(op_start_signals);

            deterministic.start_waiting();
            let mut clients = futures::future::join_all(clients).await;
            deterministic.finish_waiting();
            deterministic.run_until_parked();

            let (host, host_project, mut host_cx, host_err) = clients.remove(0);
            if let Some(host_err) = host_err {
                log::error!("host error - {:?}", host_err);
            }
            host_project.read_with(&host_cx, |project, _| assert!(!project.is_shared()));
            for (guest, guest_project, mut guest_cx, guest_err) in clients {
                if let Some(guest_err) = guest_err {
                    log::error!("{} error - {:?}", guest.username, guest_err);
                }

                guest_project.read_with(&guest_cx, |project, _| assert!(project.is_read_only()));
                guest_cx.update(|cx| {
                    cx.clear_globals();
                    drop((guest, guest_project));
                });
            }
            host_cx.update(|cx| {
                cx.clear_globals();
                drop((host, host_project));
            });

            return;
        }

        let distribution = rng.lock().gen_range(0..100);
        match distribution {
            0..=19 if !available_guests.is_empty() => {
                let guest_ix = rng.lock().gen_range(0..available_guests.len());
                let guest_username = available_guests.remove(guest_ix);
                log::info!("Adding new connection for {}", guest_username);
                next_entity_id += 100000;
                let mut guest_cx = TestAppContext::new(
                    cx.foreground_platform(),
                    cx.platform(),
                    deterministic.build_foreground(next_entity_id),
                    deterministic.build_background(),
                    cx.font_cache(),
                    cx.leak_detector(),
                    next_entity_id,
                    cx.function_name.clone(),
                );

                deterministic.start_waiting();
                let guest = server.create_client(&mut guest_cx, &guest_username).await;
                let guest_user_id = guest.current_user_id(&guest_cx);

                active_call
                    .update(cx, |call, cx| {
                        call.invite(guest_user_id.to_proto(), None, cx)
                    })
                    .await
                    .unwrap();
                deterministic.run_until_parked();
                guest_cx
                    .read(ActiveCall::global)
                    .update(&mut guest_cx, |call, cx| call.accept_incoming(cx))
                    .await
                    .unwrap();

                let guest_project = Project::remote(
                    host_project_id,
                    guest.client.clone(),
                    guest.user_store.clone(),
                    guest.project_store.clone(),
                    guest_lang_registry.clone(),
                    FakeFs::new(cx.background()),
                    guest_cx.to_async(),
                )
                .await
                .unwrap();
                deterministic.finish_waiting();

                let op_start_signal = futures::channel::mpsc::unbounded();
                user_ids.push(guest_user_id);
                op_start_signals.push(op_start_signal.0);
                clients.push(guest_cx.foreground().spawn(guest.simulate_guest(
                    guest_username.clone(),
                    guest_project,
                    op_start_signal.1,
                    rng.clone(),
                    guest_cx,
                )));

                log::info!("Added connection for {}", guest_username);
                operations += 1;
            }
            20..=29 if clients.len() > 1 => {
                let guest_ix = rng.lock().gen_range(1..clients.len());
                log::info!("Removing guest {}", user_ids[guest_ix]);
                let removed_guest_id = user_ids.remove(guest_ix);
                let guest = clients.remove(guest_ix);
                op_start_signals.remove(guest_ix);
                server.forbid_connections();
                server.disconnect_client(removed_guest_id);
                deterministic.advance_clock(RECEIVE_TIMEOUT);
                deterministic.start_waiting();
                log::info!("Waiting for guest {} to exit...", removed_guest_id);
                let (guest, guest_project, mut guest_cx, guest_err) = guest.await;
                deterministic.finish_waiting();
                server.allow_connections();

                if let Some(guest_err) = guest_err {
                    log::error!("{} error - {:?}", guest.username, guest_err);
                }
                guest_project.read_with(&guest_cx, |project, _| assert!(project.is_read_only()));
                for user_id in &user_ids {
                    let contacts = server.app_state.db.get_contacts(*user_id).await.unwrap();
                    let contacts = server
                        .store
                        .lock()
                        .await
                        .build_initial_contacts_update(contacts)
                        .contacts;
                    for contact in contacts {
                        if contact.online {
                            assert_ne!(
                                contact.user_id, removed_guest_id.0 as u64,
                                "removed guest is still a contact of another peer"
                            );
                        }
                    }
                }

                log::info!("{} removed", guest.username);
                available_guests.push(guest.username.clone());
                guest_cx.update(|cx| {
                    cx.clear_globals();
                    drop((guest, guest_project));
                });

                operations += 1;
            }
            _ => {
                while operations < max_operations && rng.lock().gen_bool(0.7) {
                    op_start_signals
                        .choose(&mut *rng.lock())
                        .unwrap()
                        .unbounded_send(())
                        .unwrap();
                    operations += 1;
                }

                if rng.lock().gen_bool(0.8) {
                    deterministic.run_until_parked();
                }
            }
        }
    }

    drop(op_start_signals);
    deterministic.start_waiting();
    let mut clients = futures::future::join_all(clients).await;
    deterministic.finish_waiting();
    deterministic.run_until_parked();

    let (host_client, host_project, mut host_cx, host_err) = clients.remove(0);
    if let Some(host_err) = host_err {
        panic!("host error - {:?}", host_err);
    }
    let host_worktree_snapshots = host_project.read_with(&host_cx, |project, cx| {
        project
            .worktrees(cx)
            .map(|worktree| {
                let snapshot = worktree.read(cx).snapshot();
                (snapshot.id(), snapshot)
            })
            .collect::<BTreeMap<_, _>>()
    });

    host_project.read_with(&host_cx, |project, cx| project.check_invariants(cx));

    for (guest_client, guest_project, mut guest_cx, guest_err) in clients.into_iter() {
        if let Some(guest_err) = guest_err {
            panic!("{} error - {:?}", guest_client.username, guest_err);
        }
        let worktree_snapshots = guest_project.read_with(&guest_cx, |project, cx| {
            project
                .worktrees(cx)
                .map(|worktree| {
                    let worktree = worktree.read(cx);
                    (worktree.id(), worktree.snapshot())
                })
                .collect::<BTreeMap<_, _>>()
        });

        assert_eq!(
            worktree_snapshots.keys().collect::<Vec<_>>(),
            host_worktree_snapshots.keys().collect::<Vec<_>>(),
            "{} has different worktrees than the host",
            guest_client.username
        );
        for (id, host_snapshot) in &host_worktree_snapshots {
            let guest_snapshot = &worktree_snapshots[id];
            assert_eq!(
                guest_snapshot.root_name(),
                host_snapshot.root_name(),
                "{} has different root name than the host for worktree {}",
                guest_client.username,
                id
            );
            assert_eq!(
                guest_snapshot.entries(false).collect::<Vec<_>>(),
                host_snapshot.entries(false).collect::<Vec<_>>(),
                "{} has different snapshot than the host for worktree {}",
                guest_client.username,
                id
            );
            assert_eq!(guest_snapshot.scan_id(), host_snapshot.scan_id());
        }

        guest_project.read_with(&guest_cx, |project, cx| project.check_invariants(cx));

        for guest_buffer in &guest_client.buffers {
            let buffer_id = guest_buffer.read_with(&guest_cx, |buffer, _| buffer.remote_id());
            let host_buffer = host_project.read_with(&host_cx, |project, cx| {
                project.buffer_for_id(buffer_id, cx).unwrap_or_else(|| {
                    panic!(
                        "host does not have buffer for guest:{}, peer:{}, id:{}",
                        guest_client.username, guest_client.peer_id, buffer_id
                    )
                })
            });
            let path =
                host_buffer.read_with(&host_cx, |buffer, cx| buffer.file().unwrap().full_path(cx));

            assert_eq!(
                guest_buffer.read_with(&guest_cx, |buffer, _| buffer.deferred_ops_len()),
                0,
                "{}, buffer {}, path {:?} has deferred operations",
                guest_client.username,
                buffer_id,
                path,
            );
            assert_eq!(
                guest_buffer.read_with(&guest_cx, |buffer, _| buffer.text()),
                host_buffer.read_with(&host_cx, |buffer, _| buffer.text()),
                "{}, buffer {}, path {:?}, differs from the host's buffer",
                guest_client.username,
                buffer_id,
                path
            );
        }

        guest_cx.update(|cx| {
            cx.clear_globals();
            drop((guest_project, guest_client));
        });
    }

    host_cx.update(|cx| {
        cx.clear_globals();
        drop((host_client, host_project))
    });
}

struct TestServer {
    peer: Arc<Peer>,
    app_state: Arc<AppState>,
    server: Arc<Server>,
    foreground: Rc<executor::Foreground>,
    notifications: mpsc::UnboundedReceiver<()>,
    connection_killers: Arc<Mutex<HashMap<UserId, Arc<AtomicBool>>>>,
    forbid_connections: Arc<AtomicBool>,
    _test_db: TestDb,
}

impl TestServer {
    async fn start(
        foreground: Rc<executor::Foreground>,
        background: Arc<executor::Background>,
    ) -> Self {
        let test_db = TestDb::fake(background.clone());
        let app_state = Self::build_app_state(&test_db).await;
        let peer = Peer::new();
        let notifications = mpsc::unbounded();
        let server = Server::new(app_state.clone(), Some(notifications.0));
        Self {
            peer,
            app_state,
            server,
            foreground,
            notifications: notifications.1,
            connection_killers: Default::default(),
            forbid_connections: Default::default(),
            _test_db: test_db,
        }
    }

    async fn create_client(&mut self, cx: &mut TestAppContext, name: &str) -> TestClient {
        cx.update(|cx| {
            let mut settings = Settings::test(cx);
            settings.projects_online_by_default = false;
            cx.set_global(settings);
        });

        let http = FakeHttpClient::with_404_response();
        let user_id = if let Ok(Some(user)) = self
            .app_state
            .db
            .get_user_by_github_account(name, None)
            .await
        {
            user.id
        } else {
            self.app_state
                .db
                .create_user(
                    &format!("{name}@example.com"),
                    false,
                    NewUserParams {
                        github_login: name.into(),
                        github_user_id: 0,
                        invite_count: 0,
                    },
                )
                .await
                .unwrap()
                .user_id
        };
        let client_name = name.to_string();
        let mut client = cx.read(|cx| Client::new(http.clone(), cx));
        let server = self.server.clone();
        let db = self.app_state.db.clone();
        let connection_killers = self.connection_killers.clone();
        let forbid_connections = self.forbid_connections.clone();
        let (connection_id_tx, mut connection_id_rx) = mpsc::channel(16);

        Arc::get_mut(&mut client)
            .unwrap()
            .set_id(user_id.0 as usize)
            .override_authenticate(move |cx| {
                cx.spawn(|_| async move {
                    let access_token = "the-token".to_string();
                    Ok(Credentials {
                        user_id: user_id.0 as u64,
                        access_token,
                    })
                })
            })
            .override_establish_connection(move |credentials, cx| {
                assert_eq!(credentials.user_id, user_id.0 as u64);
                assert_eq!(credentials.access_token, "the-token");

                let server = server.clone();
                let db = db.clone();
                let connection_killers = connection_killers.clone();
                let forbid_connections = forbid_connections.clone();
                let client_name = client_name.clone();
                let connection_id_tx = connection_id_tx.clone();
                cx.spawn(move |cx| async move {
                    if forbid_connections.load(SeqCst) {
                        Err(EstablishConnectionError::other(anyhow!(
                            "server is forbidding connections"
                        )))
                    } else {
                        let (client_conn, server_conn, killed) =
                            Connection::in_memory(cx.background());
                        connection_killers.lock().insert(user_id, killed);
                        let user = db.get_user_by_id(user_id).await.unwrap().unwrap();
                        cx.background()
                            .spawn(server.handle_connection(
                                server_conn,
                                client_name,
                                user,
                                Some(connection_id_tx),
                                cx.background(),
                            ))
                            .detach();
                        Ok(client_conn)
                    }
                })
            });

        let fs = FakeFs::new(cx.background());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));
        let project_store = cx.add_model(|_| ProjectStore::new());
        let app_state = Arc::new(workspace::AppState {
            client: client.clone(),
            user_store: user_store.clone(),
            project_store: project_store.clone(),
            languages: Arc::new(LanguageRegistry::new(Task::ready(()))),
            themes: ThemeRegistry::new((), cx.font_cache()),
            fs: fs.clone(),
            build_window_options: Default::default,
            initialize_workspace: |_, _, _| unimplemented!(),
            default_item_factory: |_, _| unimplemented!(),
        });

        Channel::init(&client);
        Project::init(&client);
        cx.update(|cx| {
            workspace::init(app_state.clone(), cx);
            call::init(client.clone(), user_store.clone(), cx);
        });

        client
            .authenticate_and_connect(false, &cx.to_async())
            .await
            .unwrap();
        let peer_id = PeerId(connection_id_rx.next().await.unwrap().0);

        let client = TestClient {
            client,
            peer_id,
            username: name.to_string(),
            user_store,
            project_store,
            fs,
            language_registry: Arc::new(LanguageRegistry::test()),
            buffers: Default::default(),
        };
        client.wait_for_current_user(cx).await;
        client
    }

    fn disconnect_client(&self, user_id: UserId) {
        self.connection_killers
            .lock()
            .remove(&user_id)
            .unwrap()
            .store(true, SeqCst);
    }

    fn forbid_connections(&self) {
        self.forbid_connections.store(true, SeqCst);
    }

    fn allow_connections(&self) {
        self.forbid_connections.store(false, SeqCst);
    }

    async fn make_contacts(&self, clients: &mut [(&TestClient, &mut TestAppContext)]) {
        for ix in 1..clients.len() {
            let (left, right) = clients.split_at_mut(ix);
            let (client_a, cx_a) = left.last_mut().unwrap();
            for (client_b, cx_b) in right {
                client_a
                    .user_store
                    .update(*cx_a, |store, cx| {
                        store.request_contact(client_b.user_id().unwrap(), cx)
                    })
                    .await
                    .unwrap();
                cx_a.foreground().run_until_parked();
                client_b
                    .user_store
                    .update(*cx_b, |store, cx| {
                        store.respond_to_contact_request(client_a.user_id().unwrap(), true, cx)
                    })
                    .await
                    .unwrap();
            }
        }
    }

    async fn create_room(&self, clients: &mut [(&TestClient, &mut TestAppContext)]) {
        self.make_contacts(clients).await;

        let (left, right) = clients.split_at_mut(1);
        let (_client_a, cx_a) = &mut left[0];
        let active_call_a = cx_a.read(ActiveCall::global);

        for (client_b, cx_b) in right {
            let user_id_b = client_b.current_user_id(*cx_b).to_proto();
            active_call_a
                .update(*cx_a, |call, cx| call.invite(user_id_b, None, cx))
                .await
                .unwrap();

            cx_b.foreground().run_until_parked();
            let active_call_b = cx_b.read(ActiveCall::global);
            active_call_b
                .update(*cx_b, |call, cx| call.accept_incoming(cx))
                .await
                .unwrap();
        }
    }

    async fn build_app_state(test_db: &TestDb) -> Arc<AppState> {
        Arc::new(AppState {
            db: test_db.db().clone(),
            api_token: Default::default(),
            invite_link_prefix: Default::default(),
        })
    }

    async fn condition<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&Store) -> bool,
    {
        assert!(
            self.foreground.parking_forbidden(),
            "you must call forbid_parking to use server conditions so we don't block indefinitely"
        );
        while !(predicate)(&*self.server.store.lock().await) {
            self.foreground.start_waiting();
            self.notifications.next().await;
            self.foreground.finish_waiting();
        }
    }
}

impl Deref for TestServer {
    type Target = Server;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.peer.reset();
    }
}

struct TestClient {
    client: Arc<Client>,
    username: String,
    pub peer_id: PeerId,
    pub user_store: ModelHandle<UserStore>,
    pub project_store: ModelHandle<ProjectStore>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<FakeFs>,
    buffers: HashSet<ModelHandle<language::Buffer>>,
}

impl Deref for TestClient {
    type Target = Arc<Client>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

struct ContactsSummary {
    pub current: Vec<String>,
    pub outgoing_requests: Vec<String>,
    pub incoming_requests: Vec<String>,
}

impl TestClient {
    pub fn current_user_id(&self, cx: &TestAppContext) -> UserId {
        UserId::from_proto(
            self.user_store
                .read_with(cx, |user_store, _| user_store.current_user().unwrap().id),
        )
    }

    async fn wait_for_current_user(&self, cx: &TestAppContext) {
        let mut authed_user = self
            .user_store
            .read_with(cx, |user_store, _| user_store.watch_current_user());
        while authed_user.next().await.unwrap().is_none() {}
    }

    async fn clear_contacts(&self, cx: &mut TestAppContext) {
        self.user_store
            .update(cx, |store, _| store.clear_contacts())
            .await;
    }

    fn summarize_contacts(&self, cx: &TestAppContext) -> ContactsSummary {
        self.user_store.read_with(cx, |store, _| ContactsSummary {
            current: store
                .contacts()
                .iter()
                .map(|contact| contact.user.github_login.clone())
                .collect(),
            outgoing_requests: store
                .outgoing_contact_requests()
                .iter()
                .map(|user| user.github_login.clone())
                .collect(),
            incoming_requests: store
                .incoming_contact_requests()
                .iter()
                .map(|user| user.github_login.clone())
                .collect(),
        })
    }

    async fn build_local_project(
        &self,
        root_path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> (ModelHandle<Project>, WorktreeId) {
        let project = cx.update(|cx| {
            Project::local(
                self.client.clone(),
                self.user_store.clone(),
                self.project_store.clone(),
                self.language_registry.clone(),
                self.fs.clone(),
                cx,
            )
        });
        let (worktree, _) = project
            .update(cx, |p, cx| {
                p.find_or_create_local_worktree(root_path, true, cx)
            })
            .await
            .unwrap();
        worktree
            .read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        (project, worktree.read_with(cx, |tree, _| tree.id()))
    }

    async fn build_remote_project(
        &self,
        host_project_id: u64,
        guest_cx: &mut TestAppContext,
    ) -> ModelHandle<Project> {
        let project_b = guest_cx.spawn(|cx| {
            Project::remote(
                host_project_id,
                self.client.clone(),
                self.user_store.clone(),
                self.project_store.clone(),
                self.language_registry.clone(),
                FakeFs::new(cx.background()),
                cx,
            )
        });
        project_b.await.unwrap()
    }

    fn build_workspace(
        &self,
        project: &ModelHandle<Project>,
        cx: &mut TestAppContext,
    ) -> ViewHandle<Workspace> {
        let (_, root_view) = cx.add_window(|_| EmptyView);
        cx.add_view(&root_view, |cx| {
            Workspace::new(project.clone(), |_, _| unimplemented!(), cx)
        })
    }

    async fn simulate_host(
        mut self,
        project: ModelHandle<Project>,
        op_start_signal: futures::channel::mpsc::UnboundedReceiver<()>,
        rng: Arc<Mutex<StdRng>>,
        mut cx: TestAppContext,
    ) -> (
        Self,
        ModelHandle<Project>,
        TestAppContext,
        Option<anyhow::Error>,
    ) {
        async fn simulate_host_internal(
            client: &mut TestClient,
            project: ModelHandle<Project>,
            mut op_start_signal: futures::channel::mpsc::UnboundedReceiver<()>,
            rng: Arc<Mutex<StdRng>>,
            cx: &mut TestAppContext,
        ) -> anyhow::Result<()> {
            let fs = project.read_with(cx, |project, _| project.fs().clone());

            while op_start_signal.next().await.is_some() {
                let distribution = rng.lock().gen_range::<usize, _>(0..100);
                let files = fs.as_fake().files().await;
                match distribution {
                    0..=19 if !files.is_empty() => {
                        let path = files.choose(&mut *rng.lock()).unwrap();
                        let mut path = path.as_path();
                        while let Some(parent_path) = path.parent() {
                            path = parent_path;
                            if rng.lock().gen() {
                                break;
                            }
                        }

                        log::info!("Host: find/create local worktree {:?}", path);
                        let find_or_create_worktree = project.update(cx, |project, cx| {
                            project.find_or_create_local_worktree(path, true, cx)
                        });
                        if rng.lock().gen() {
                            cx.background().spawn(find_or_create_worktree).detach();
                        } else {
                            find_or_create_worktree.await?;
                        }
                    }
                    20..=79 if !files.is_empty() => {
                        let buffer = if client.buffers.is_empty() || rng.lock().gen() {
                            let file = files.choose(&mut *rng.lock()).unwrap();
                            let (worktree, path) = project
                                .update(cx, |project, cx| {
                                    project.find_or_create_local_worktree(file.clone(), true, cx)
                                })
                                .await?;
                            let project_path =
                                worktree.read_with(cx, |worktree, _| (worktree.id(), path));
                            log::info!(
                                "Host: opening path {:?}, worktree {}, relative_path {:?}",
                                file,
                                project_path.0,
                                project_path.1
                            );
                            let buffer = project
                                .update(cx, |project, cx| project.open_buffer(project_path, cx))
                                .await
                                .unwrap();
                            client.buffers.insert(buffer.clone());
                            buffer
                        } else {
                            client
                                .buffers
                                .iter()
                                .choose(&mut *rng.lock())
                                .unwrap()
                                .clone()
                        };

                        if rng.lock().gen_bool(0.1) {
                            cx.update(|cx| {
                                log::info!(
                                    "Host: dropping buffer {:?}",
                                    buffer.read(cx).file().unwrap().full_path(cx)
                                );
                                client.buffers.remove(&buffer);
                                drop(buffer);
                            });
                        } else {
                            buffer.update(cx, |buffer, cx| {
                                log::info!(
                                    "Host: updating buffer {:?} ({})",
                                    buffer.file().unwrap().full_path(cx),
                                    buffer.remote_id()
                                );

                                if rng.lock().gen_bool(0.7) {
                                    buffer.randomly_edit(&mut *rng.lock(), 5, cx);
                                } else {
                                    buffer.randomly_undo_redo(&mut *rng.lock(), cx);
                                }
                            });
                        }
                    }
                    _ => loop {
                        let path_component_count = rng.lock().gen_range::<usize, _>(1..=5);
                        let mut path = PathBuf::new();
                        path.push("/");
                        for _ in 0..path_component_count {
                            let letter = rng.lock().gen_range(b'a'..=b'z');
                            path.push(std::str::from_utf8(&[letter]).unwrap());
                        }
                        path.set_extension("rs");
                        let parent_path = path.parent().unwrap();

                        log::info!("Host: creating file {:?}", path,);

                        if fs.create_dir(parent_path).await.is_ok()
                            && fs.create_file(&path, Default::default()).await.is_ok()
                        {
                            break;
                        } else {
                            log::info!("Host: cannot create file");
                        }
                    },
                }

                cx.background().simulate_random_delay().await;
            }

            Ok(())
        }

        let result =
            simulate_host_internal(&mut self, project.clone(), op_start_signal, rng, &mut cx).await;
        log::info!("Host done");
        (self, project, cx, result.err())
    }

    pub async fn simulate_guest(
        mut self,
        guest_username: String,
        project: ModelHandle<Project>,
        op_start_signal: futures::channel::mpsc::UnboundedReceiver<()>,
        rng: Arc<Mutex<StdRng>>,
        mut cx: TestAppContext,
    ) -> (
        Self,
        ModelHandle<Project>,
        TestAppContext,
        Option<anyhow::Error>,
    ) {
        async fn simulate_guest_internal(
            client: &mut TestClient,
            guest_username: &str,
            project: ModelHandle<Project>,
            mut op_start_signal: futures::channel::mpsc::UnboundedReceiver<()>,
            rng: Arc<Mutex<StdRng>>,
            cx: &mut TestAppContext,
        ) -> anyhow::Result<()> {
            while op_start_signal.next().await.is_some() {
                let buffer = if client.buffers.is_empty() || rng.lock().gen() {
                    let worktree = if let Some(worktree) = project.read_with(cx, |project, cx| {
                        project
                            .worktrees(cx)
                            .filter(|worktree| {
                                let worktree = worktree.read(cx);
                                worktree.is_visible()
                                    && worktree.entries(false).any(|e| e.is_file())
                            })
                            .choose(&mut *rng.lock())
                    }) {
                        worktree
                    } else {
                        cx.background().simulate_random_delay().await;
                        continue;
                    };

                    let (worktree_root_name, project_path) =
                        worktree.read_with(cx, |worktree, _| {
                            let entry = worktree
                                .entries(false)
                                .filter(|e| e.is_file())
                                .choose(&mut *rng.lock())
                                .unwrap();
                            (
                                worktree.root_name().to_string(),
                                (worktree.id(), entry.path.clone()),
                            )
                        });
                    log::info!(
                        "{}: opening path {:?} in worktree {} ({})",
                        guest_username,
                        project_path.1,
                        project_path.0,
                        worktree_root_name,
                    );
                    let buffer = project
                        .update(cx, |project, cx| {
                            project.open_buffer(project_path.clone(), cx)
                        })
                        .await?;
                    log::info!(
                        "{}: opened path {:?} in worktree {} ({}) with buffer id {}",
                        guest_username,
                        project_path.1,
                        project_path.0,
                        worktree_root_name,
                        buffer.read_with(cx, |buffer, _| buffer.remote_id())
                    );
                    client.buffers.insert(buffer.clone());
                    buffer
                } else {
                    client
                        .buffers
                        .iter()
                        .choose(&mut *rng.lock())
                        .unwrap()
                        .clone()
                };

                let choice = rng.lock().gen_range(0..100);
                match choice {
                    0..=9 => {
                        cx.update(|cx| {
                            log::info!(
                                "{}: dropping buffer {:?}",
                                guest_username,
                                buffer.read(cx).file().unwrap().full_path(cx)
                            );
                            client.buffers.remove(&buffer);
                            drop(buffer);
                        });
                    }
                    10..=19 => {
                        let completions = project.update(cx, |project, cx| {
                            log::info!(
                                "{}: requesting completions for buffer {} ({:?})",
                                guest_username,
                                buffer.read(cx).remote_id(),
                                buffer.read(cx).file().unwrap().full_path(cx)
                            );
                            let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                            project.completions(&buffer, offset, cx)
                        });
                        let completions = cx.background().spawn(async move {
                            completions
                                .await
                                .map_err(|err| anyhow!("completions request failed: {:?}", err))
                        });
                        if rng.lock().gen_bool(0.3) {
                            log::info!("{}: detaching completions request", guest_username);
                            cx.update(|cx| completions.detach_and_log_err(cx));
                        } else {
                            completions.await?;
                        }
                    }
                    20..=29 => {
                        let code_actions = project.update(cx, |project, cx| {
                            log::info!(
                                "{}: requesting code actions for buffer {} ({:?})",
                                guest_username,
                                buffer.read(cx).remote_id(),
                                buffer.read(cx).file().unwrap().full_path(cx)
                            );
                            let range = buffer.read(cx).random_byte_range(0, &mut *rng.lock());
                            project.code_actions(&buffer, range, cx)
                        });
                        let code_actions = cx.background().spawn(async move {
                            code_actions
                                .await
                                .map_err(|err| anyhow!("code actions request failed: {:?}", err))
                        });
                        if rng.lock().gen_bool(0.3) {
                            log::info!("{}: detaching code actions request", guest_username);
                            cx.update(|cx| code_actions.detach_and_log_err(cx));
                        } else {
                            code_actions.await?;
                        }
                    }
                    30..=39 if buffer.read_with(cx, |buffer, _| buffer.is_dirty()) => {
                        let (requested_version, save) = buffer.update(cx, |buffer, cx| {
                            log::info!(
                                "{}: saving buffer {} ({:?})",
                                guest_username,
                                buffer.remote_id(),
                                buffer.file().unwrap().full_path(cx)
                            );
                            (buffer.version(), buffer.save(cx))
                        });
                        let save = cx.background().spawn(async move {
                            let (saved_version, _, _) = save
                                .await
                                .map_err(|err| anyhow!("save request failed: {:?}", err))?;
                            assert!(saved_version.observed_all(&requested_version));
                            Ok::<_, anyhow::Error>(())
                        });
                        if rng.lock().gen_bool(0.3) {
                            log::info!("{}: detaching save request", guest_username);
                            cx.update(|cx| save.detach_and_log_err(cx));
                        } else {
                            save.await?;
                        }
                    }
                    40..=44 => {
                        let prepare_rename = project.update(cx, |project, cx| {
                            log::info!(
                                "{}: preparing rename for buffer {} ({:?})",
                                guest_username,
                                buffer.read(cx).remote_id(),
                                buffer.read(cx).file().unwrap().full_path(cx)
                            );
                            let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                            project.prepare_rename(buffer, offset, cx)
                        });
                        let prepare_rename = cx.background().spawn(async move {
                            prepare_rename
                                .await
                                .map_err(|err| anyhow!("prepare rename request failed: {:?}", err))
                        });
                        if rng.lock().gen_bool(0.3) {
                            log::info!("{}: detaching prepare rename request", guest_username);
                            cx.update(|cx| prepare_rename.detach_and_log_err(cx));
                        } else {
                            prepare_rename.await?;
                        }
                    }
                    45..=49 => {
                        let definitions = project.update(cx, |project, cx| {
                            log::info!(
                                "{}: requesting definitions for buffer {} ({:?})",
                                guest_username,
                                buffer.read(cx).remote_id(),
                                buffer.read(cx).file().unwrap().full_path(cx)
                            );
                            let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                            project.definition(&buffer, offset, cx)
                        });
                        let definitions = cx.background().spawn(async move {
                            definitions
                                .await
                                .map_err(|err| anyhow!("definitions request failed: {:?}", err))
                        });
                        if rng.lock().gen_bool(0.3) {
                            log::info!("{}: detaching definitions request", guest_username);
                            cx.update(|cx| definitions.detach_and_log_err(cx));
                        } else {
                            client.buffers.extend(
                                definitions.await?.into_iter().map(|loc| loc.target.buffer),
                            );
                        }
                    }
                    50..=54 => {
                        let highlights = project.update(cx, |project, cx| {
                            log::info!(
                                "{}: requesting highlights for buffer {} ({:?})",
                                guest_username,
                                buffer.read(cx).remote_id(),
                                buffer.read(cx).file().unwrap().full_path(cx)
                            );
                            let offset = rng.lock().gen_range(0..=buffer.read(cx).len());
                            project.document_highlights(&buffer, offset, cx)
                        });
                        let highlights = cx.background().spawn(async move {
                            highlights
                                .await
                                .map_err(|err| anyhow!("highlights request failed: {:?}", err))
                        });
                        if rng.lock().gen_bool(0.3) {
                            log::info!("{}: detaching highlights request", guest_username);
                            cx.update(|cx| highlights.detach_and_log_err(cx));
                        } else {
                            highlights.await?;
                        }
                    }
                    55..=59 => {
                        let search = project.update(cx, |project, cx| {
                            let query = rng.lock().gen_range('a'..='z');
                            log::info!("{}: project-wide search {:?}", guest_username, query);
                            project.search(SearchQuery::text(query, false, false), cx)
                        });
                        let search = cx.background().spawn(async move {
                            search
                                .await
                                .map_err(|err| anyhow!("search request failed: {:?}", err))
                        });
                        if rng.lock().gen_bool(0.3) {
                            log::info!("{}: detaching search request", guest_username);
                            cx.update(|cx| search.detach_and_log_err(cx));
                        } else {
                            client.buffers.extend(search.await?.into_keys());
                        }
                    }
                    60..=69 => {
                        let worktree = project
                            .read_with(cx, |project, cx| {
                                project
                                    .worktrees(cx)
                                    .filter(|worktree| {
                                        let worktree = worktree.read(cx);
                                        worktree.is_visible()
                                            && worktree.entries(false).any(|e| e.is_file())
                                            && worktree.root_entry().map_or(false, |e| e.is_dir())
                                    })
                                    .choose(&mut *rng.lock())
                            })
                            .unwrap();
                        let (worktree_id, worktree_root_name) = worktree
                            .read_with(cx, |worktree, _| {
                                (worktree.id(), worktree.root_name().to_string())
                            });

                        let mut new_name = String::new();
                        for _ in 0..10 {
                            let letter = rng.lock().gen_range('a'..='z');
                            new_name.push(letter);
                        }
                        let mut new_path = PathBuf::new();
                        new_path.push(new_name);
                        new_path.set_extension("rs");
                        log::info!(
                            "{}: creating {:?} in worktree {} ({})",
                            guest_username,
                            new_path,
                            worktree_id,
                            worktree_root_name,
                        );
                        project
                            .update(cx, |project, cx| {
                                project.create_entry((worktree_id, new_path), false, cx)
                            })
                            .unwrap()
                            .await?;
                    }
                    _ => {
                        buffer.update(cx, |buffer, cx| {
                            log::info!(
                                "{}: updating buffer {} ({:?})",
                                guest_username,
                                buffer.remote_id(),
                                buffer.file().unwrap().full_path(cx)
                            );
                            if rng.lock().gen_bool(0.7) {
                                buffer.randomly_edit(&mut *rng.lock(), 5, cx);
                            } else {
                                buffer.randomly_undo_redo(&mut *rng.lock(), cx);
                            }
                        });
                    }
                }
                cx.background().simulate_random_delay().await;
            }
            Ok(())
        }

        let result = simulate_guest_internal(
            &mut self,
            &guest_username,
            project.clone(),
            op_start_signal,
            rng,
            &mut cx,
        )
        .await;
        log::info!("{}: done", guest_username);

        (self, project, cx, result.err())
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
        self.client.tear_down();
    }
}

impl Executor for Arc<gpui::executor::Background> {
    type Sleep = gpui::executor::Timer;

    fn spawn_detached<F: 'static + Send + Future<Output = ()>>(&self, future: F) {
        self.spawn(future).detach();
    }

    fn sleep(&self, duration: Duration) -> Self::Sleep {
        self.as_ref().timer(duration)
    }
}

fn channel_messages(channel: &Channel) -> Vec<(String, String, bool)> {
    channel
        .messages()
        .cursor::<()>()
        .map(|m| {
            (
                m.sender.github_login.clone(),
                m.body.clone(),
                m.is_pending(),
            )
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct RoomParticipants {
    remote: Vec<String>,
    pending: Vec<String>,
}

fn room_participants(room: &ModelHandle<Room>, cx: &mut TestAppContext) -> RoomParticipants {
    room.read_with(cx, |room, _| RoomParticipants {
        remote: room
            .remote_participants()
            .iter()
            .map(|(_, participant)| participant.user.github_login.clone())
            .collect(),
        pending: room
            .pending_participants()
            .iter()
            .map(|user| user.github_login.clone())
            .collect(),
    })
}
