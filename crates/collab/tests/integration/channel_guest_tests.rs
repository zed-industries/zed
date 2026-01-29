use crate::TestServer;
use call::ActiveCall;
use chrono::Utc;
use collab::db::ChannelId;
use editor::Editor;
use gpui::{BackgroundExecutor, TestAppContext};
use rpc::proto;
use util::rel_path::rel_path;
#[gpui::test]
async fn test_channel_guests(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let channel_id = server
        .make_public_channel("the-channel", &client_a, cx_a)
        .await;

    // Client A shares a project in the channel
    let project_a = client_a.build_test_project(cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    cx_a.executor().run_until_parked();

    // Client B joins channel A as a guest
    cx_b.update(|cx| workspace::join_channel(channel_id, client_b.app_state.clone(), None, cx))
        .await
        .unwrap();

    // b should be following a in the shared project.
    // B is a guest,
    executor.run_until_parked();

    let active_call_b = cx_b.read(ActiveCall::global);
    let project_b =
        active_call_b.read_with(cx_b, |call, _| call.location().unwrap().upgrade().unwrap());
    let room_b = active_call_b.update(cx_b, |call, _| call.room().unwrap().clone());

    assert_eq!(
        project_b.read_with(cx_b, |project, _| project.remote_id()),
        Some(project_id),
    );
    assert!(project_b.read_with(cx_b, |project, cx| project.is_read_only(cx)));
    assert!(
        project_b
            .update(cx_b, |project, cx| {
                let worktree_id = project.worktrees(cx).next().unwrap().read(cx).id();
                project.create_entry((worktree_id, rel_path("b.txt")), false, cx)
            })
            .await
            .is_err()
    );
    assert!(room_b.read_with(cx_b, |room, _| room.is_muted()));
}

#[gpui::test]
async fn test_channel_guest_promotion(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let channel_id = server
        .make_public_channel("the-channel", &client_a, cx_a)
        .await;

    let project_a = client_a.build_test_project(cx_a).await;
    cx_a.update(|cx| workspace::join_channel(channel_id, client_a.app_state.clone(), None, cx))
        .await
        .unwrap();

    // Client A shares a project in the channel
    active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    cx_a.run_until_parked();

    // Client B joins channel A as a guest
    cx_b.update(|cx| workspace::join_channel(channel_id, client_b.app_state.clone(), None, cx))
        .await
        .unwrap();
    cx_a.run_until_parked();

    // client B opens 1.txt as a guest
    let (workspace_b, cx_b) = client_b.active_workspace(cx_b);
    let room_b = cx_b
        .read(ActiveCall::global)
        .update(cx_b, |call, _| call.room().unwrap().clone());
    cx_b.simulate_keystrokes("cmd-p");
    cx_a.run_until_parked();
    cx_b.simulate_keystrokes("1 enter");

    let (project_b, editor_b) = workspace_b.update(cx_b, |workspace, cx| {
        (
            workspace.project().clone(),
            workspace.active_item_as::<Editor>(cx).unwrap(),
        )
    });
    assert!(project_b.read_with(cx_b, |project, cx| project.is_read_only(cx)));
    assert!(editor_b.update(cx_b, |e, cx| e.read_only(cx)));
    cx_b.update(|_window, cx_b| {
        assert!(room_b.read_with(cx_b, |room, _| !room.can_use_microphone()));
    });
    assert!(
        room_b
            .update(cx_b, |room, cx| room.share_microphone(cx))
            .await
            .is_err()
    );

    // B is promoted
    active_call_a
        .update(cx_a, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_participant_role(
                    client_b.user_id().unwrap(),
                    proto::ChannelRole::Member,
                    cx,
                )
            })
        })
        .await
        .unwrap();
    cx_a.run_until_parked();

    // project and buffers are now editable
    assert!(project_b.read_with(cx_b, |project, cx| !project.is_read_only(cx)));
    assert!(editor_b.update(cx_b, |editor, cx| !editor.read_only(cx)));

    // B sees themselves as muted, and can unmute.
    cx_b.update(|_window, cx_b| {
        assert!(room_b.read_with(cx_b, |room, _| room.can_use_microphone()));
    });
    room_b.read_with(cx_b, |room, _| assert!(room.is_muted()));
    room_b.update(cx_b, |room, cx| room.toggle_mute(cx));
    cx_a.run_until_parked();
    room_b.read_with(cx_b, |room, _| assert!(!room.is_muted()));

    // B is demoted
    active_call_a
        .update(cx_a, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_participant_role(
                    client_b.user_id().unwrap(),
                    proto::ChannelRole::Guest,
                    cx,
                )
            })
        })
        .await
        .unwrap();
    cx_a.run_until_parked();

    // project and buffers are no longer editable
    assert!(project_b.read_with(cx_b, |project, cx| project.is_read_only(cx)));
    assert!(editor_b.update(cx_b, |editor, cx| editor.read_only(cx)));
    assert!(
        room_b
            .update(cx_b, |room, cx| room.share_microphone(cx))
            .await
            .is_err()
    );
}

#[gpui::test]
async fn test_channel_requires_zed_cla(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;

    server
        .app_state
        .db
        .update_or_create_user_by_github_account("user_b", 100, None, None, Utc::now(), None)
        .await
        .unwrap();

    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    // Create a parent channel that requires the Zed CLA
    let parent_channel_id = server
        .make_channel("the-channel", None, (&client_a, cx_a), &mut [])
        .await;
    server
        .app_state
        .db
        .set_channel_requires_zed_cla(ChannelId::from_proto(parent_channel_id.0), true)
        .await
        .unwrap();

    // Create a public channel that is a child of the parent channel.
    let channel_id = client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            store.create_channel("the-sub-channel", Some(parent_channel_id), cx)
        })
        .await
        .unwrap();
    client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            store.set_channel_visibility(parent_channel_id, proto::ChannelVisibility::Public, cx)
        })
        .await
        .unwrap();
    client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            store.set_channel_visibility(channel_id, proto::ChannelVisibility::Public, cx)
        })
        .await
        .unwrap();

    // Users A and B join the channel. B is a guest.
    active_call_a
        .update(cx_a, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();
    active_call_b
        .update(cx_b, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();
    cx_a.run_until_parked();
    let room_b = cx_b
        .read(ActiveCall::global)
        .update(cx_b, |call, _| call.room().unwrap().clone());
    cx_b.update(|cx_b| {
        assert!(room_b.read_with(cx_b, |room, _| !room.can_use_microphone()));
    });

    // A tries to grant write access to B, but cannot because B has not
    // yet signed the zed CLA.
    active_call_a
        .update(cx_a, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_participant_role(
                    client_b.user_id().unwrap(),
                    proto::ChannelRole::Member,
                    cx,
                )
            })
        })
        .await
        .unwrap_err();
    cx_a.run_until_parked();
    assert!(room_b.read_with(cx_b, |room, _| !room.can_share_projects()));
    cx_b.update(|cx_b| {
        assert!(room_b.read_with(cx_b, |room, _| !room.can_use_microphone()));
    });

    // A tries to grant write access to B, but cannot because B has not
    // yet signed the zed CLA.
    active_call_a
        .update(cx_a, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_participant_role(
                    client_b.user_id().unwrap(),
                    proto::ChannelRole::Talker,
                    cx,
                )
            })
        })
        .await
        .unwrap();
    cx_a.run_until_parked();
    assert!(room_b.read_with(cx_b, |room, _| !room.can_share_projects()));
    cx_b.update(|cx_b| {
        assert!(room_b.read_with(cx_b, |room, _| room.can_use_microphone()));
    });

    // User B signs the zed CLA.
    server
        .app_state
        .db
        .add_contributor("user_b", 100, None, None, Utc::now(), None)
        .await
        .unwrap();

    // A can now grant write access to B.
    active_call_a
        .update(cx_a, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_participant_role(
                    client_b.user_id().unwrap(),
                    proto::ChannelRole::Member,
                    cx,
                )
            })
        })
        .await
        .unwrap();
    cx_a.run_until_parked();
    assert!(room_b.read_with(cx_b, |room, _| room.can_share_projects()));
    cx_b.update(|cx_b| {
        assert!(room_b.read_with(cx_b, |room, _| room.can_use_microphone()));
    });
}
