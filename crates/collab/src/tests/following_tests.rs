#![allow(clippy::reversed_empty_ranges)]
use crate::tests::TestServer;
use call::{ActiveCall, ParticipantLocation};
use client::ChannelId;
use collab_ui::{
    channel_view::ChannelView,
    notifications::project_shared_notification::ProjectSharedNotification,
};
use editor::{Editor, MultiBuffer, PathKey};
use gpui::{
    AppContext as _, BackgroundExecutor, BorrowAppContext, Entity, SharedString, TestAppContext,
    VisualContext, VisualTestContext, point,
};
use language::Capability;
use project::WorktreeSettings;
use rpc::proto::PeerId;
use serde_json::json;
use settings::SettingsStore;
use text::{Point, ToPoint};
use util::{path, test::sample_text};
use workspace::{SplitDirection, Workspace, item::ItemHandle as _};

use super::TestClient;

#[gpui::test(iterations = 10)]
async fn test_basic_following(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
    cx_d: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
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
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "1.txt": "one\none\none",
                "2.txt": "two\ntwo\ntwo",
                "3.txt": "three\nthree\nthree",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    cx_b.update(|window, _| {
        assert!(window.is_window_active());
    });

    // Client A opens some editors.
    let pane_a = workspace_a.update(cx_a, |workspace, _| workspace.active_pane().clone());
    let editor_a1 = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let editor_a2 = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B opens an editor.
    let editor_b1 = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, window, cx)
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
    editor_a1.update_in(cx_a, |editor, window, cx| {
        editor.handle_input("a", window, cx);
        editor.handle_input("b", window, cx);
        editor.handle_input("c", window, cx);
        editor.select_left(&Default::default(), window, cx);
        assert_eq!(editor.selections.ranges(cx), vec![3..2]);
    });
    editor_a2.update_in(cx_a, |editor, window, cx| {
        editor.handle_input("d", window, cx);
        editor.handle_input("e", window, cx);
        editor.select_left(&Default::default(), window, cx);
        assert_eq!(editor.selections.ranges(cx), vec![2..1]);
    });

    // When client B starts following client A, only the active view state is replicated to client B.
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(peer_id_a, window, cx)
    });

    cx_c.executor().run_until_parked();
    let editor_b2 = workspace_b.update(cx_b, |workspace, cx| {
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
        editor_b2.update(cx_b, |editor, cx| editor.selections.ranges(cx)),
        vec![2..1]
    );
    assert_eq!(
        editor_b1.update(cx_b, |editor, cx| editor.selections.ranges(cx)),
        vec![3..3]
    );

    executor.run_until_parked();
    let active_call_c = cx_c.read(ActiveCall::global);
    let project_c = client_c.join_remote_project(project_id, cx_c).await;
    let (workspace_c, cx_c) = client_c.build_workspace(&project_c, cx_c);
    active_call_c
        .update(cx_c, |call, cx| call.set_location(Some(&project_c), cx))
        .await
        .unwrap();
    drop(project_c);

    // Client C also follows client A.
    workspace_c.update_in(cx_c, |workspace, window, cx| {
        workspace.follow(peer_id_a, window, cx)
    });

    cx_d.executor().run_until_parked();
    let active_call_d = cx_d.read(ActiveCall::global);
    let project_d = client_d.join_remote_project(project_id, cx_d).await;
    let (workspace_d, cx_d) = client_d.build_workspace(&project_d, cx_d);
    active_call_d
        .update(cx_d, |call, cx| call.set_location(Some(&project_d), cx))
        .await
        .unwrap();
    drop(project_d);

    // All clients see that clients B and C are following client A.
    cx_c.executor().run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("C", &cx_c), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[(peer_id_a, vec![peer_id_b, peer_id_c])],
            "followers seen by {name}"
        );
    }

    // Client C unfollows client A.
    workspace_c.update_in(cx_c, |workspace, window, cx| {
        workspace.unfollow(peer_id_a, window, cx).unwrap();
    });

    // All clients see that clients B is following client A.
    cx_c.executor().run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("C", &cx_c), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[(peer_id_a, vec![peer_id_b])],
            "followers seen by {name}"
        );
    }

    // Client C re-follows client A.
    workspace_c.update_in(cx_c, |workspace, window, cx| {
        workspace.follow(peer_id_a, window, cx)
    });

    // All clients see that clients B and C are following client A.
    cx_c.executor().run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("C", &cx_c), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[(peer_id_a, vec![peer_id_b, peer_id_c])],
            "followers seen by {name}"
        );
    }

    // Client D follows client B, then switches to following client C.
    workspace_d.update_in(cx_d, |workspace, window, cx| {
        workspace.follow(peer_id_b, window, cx)
    });
    cx_a.executor().run_until_parked();
    workspace_d.update_in(cx_d, |workspace, window, cx| {
        workspace.follow(peer_id_c, window, cx)
    });

    // All clients see that D is following C
    cx_a.executor().run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("C", &cx_c), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[
                (peer_id_a, vec![peer_id_b, peer_id_c]),
                (peer_id_c, vec![peer_id_d])
            ],
            "followers seen by {name}"
        );
    }

    // Client C closes the project.
    let weak_workspace_c = workspace_c.downgrade();
    workspace_c.update_in(cx_c, |workspace, window, cx| {
        workspace.close_window(&Default::default(), window, cx);
    });
    executor.run_until_parked();
    // are you sure you want to leave the call?
    cx_c.simulate_prompt_answer("Close window and hang up");
    cx_c.cx.update(|_| {
        drop(workspace_c);
    });
    executor.run_until_parked();
    cx_c.cx.update(|_| {});

    weak_workspace_c.assert_released();

    // Clients A and B see that client B is following A, and client C is not present in the followers.
    executor.run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[(peer_id_a, vec![peer_id_b]),],
            "followers seen by {name}"
        );
    }

    // When client A activates a different editor, client B does so as well.
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.activate_item(&editor_a1, true, true, window, cx)
    });
    executor.run_until_parked();
    workspace_b.update(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().item_id(),
            editor_b1.item_id()
        );
    });

    // When client A opens a multibuffer, client B does so as well.
    let multibuffer_a = cx_a.new(|cx| {
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
        let mut result = MultiBuffer::new(Capability::ReadWrite);
        result.set_excerpts_for_path(
            PathKey::for_buffer(&buffer_a1, cx),
            buffer_a1,
            [Point::row_range(1..2)],
            1,
            cx,
        );
        result.set_excerpts_for_path(
            PathKey::for_buffer(&buffer_a2, cx),
            buffer_a2,
            [Point::row_range(5..6)],
            1,
            cx,
        );
        result
    });
    let multibuffer_editor_a = workspace_a.update_in(cx_a, |workspace, window, cx| {
        let editor = cx
            .new(|cx| Editor::for_multibuffer(multibuffer_a, Some(project_a.clone()), window, cx));
        workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
        editor
    });
    executor.run_until_parked();
    let multibuffer_editor_b = workspace_b.update(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });
    assert_eq!(
        multibuffer_editor_a.update(cx_a, |editor, cx| editor.text(cx)),
        multibuffer_editor_b.update(cx_b, |editor, cx| editor.text(cx)),
    );

    // When client A navigates back and forth, client B does so as well.
    workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.go_back(workspace.active_pane().downgrade(), window, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    workspace_b.update(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().item_id(),
            editor_b1.item_id()
        );
    });

    workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.go_back(workspace.active_pane().downgrade(), window, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    workspace_b.update(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().item_id(),
            editor_b2.item_id()
        );
    });

    workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.go_forward(workspace.active_pane().downgrade(), window, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();
    workspace_b.update(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().item_id(),
            editor_b1.item_id()
        );
    });

    // Changes to client A's editor are reflected on client B.
    editor_a1.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([1..1, 2..2]));
    });
    executor.advance_clock(workspace::item::LEADER_UPDATE_THROTTLE);
    executor.run_until_parked();
    cx_b.background_executor.run_until_parked();

    editor_b1.update(cx_b, |editor, cx| {
        assert_eq!(editor.selections.ranges(cx), &[1..1, 2..2]);
    });

    editor_a1.update_in(cx_a, |editor, window, cx| {
        editor.set_text("TWO", window, cx)
    });
    executor.run_until_parked();
    editor_b1.update(cx_b, |editor, cx| assert_eq!(editor.text(cx), "TWO"));

    editor_a1.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([3..3]));
        editor.set_scroll_position(point(0., 100.), window, cx);
    });
    executor.advance_clock(workspace::item::LEADER_UPDATE_THROTTLE);
    executor.run_until_parked();
    editor_b1.update(cx_b, |editor, cx| {
        assert_eq!(editor.selections.ranges(cx), &[3..3]);
    });

    // After unfollowing, client B stops receiving updates from client A.
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.unfollow(peer_id_a, window, cx).unwrap()
    });
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.activate_item(&editor_a2, true, true, window, cx)
    });
    executor.run_until_parked();
    assert_eq!(
        workspace_b.update(cx_b, |workspace, cx| workspace
            .active_item(cx)
            .unwrap()
            .item_id()),
        editor_b1.item_id()
    );

    // Client A starts following client B.
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.follow(peer_id_b, window, cx)
    });
    executor.run_until_parked();
    assert_eq!(
        workspace_a.update(cx_a, |workspace, _| workspace.leader_for_pane(&pane_a)),
        Some(peer_id_b)
    );
    assert_eq!(
        workspace_a.update_in(cx_a, |workspace, _, cx| workspace
            .active_item(cx)
            .unwrap()
            .item_id()),
        editor_a1.item_id()
    );

    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    {
        use crate::rpc::RECONNECT_TIMEOUT;
        use gpui::TestScreenCaptureSource;
        use workspace::{
            dock::{DockPosition, test::TestPanel},
            item::test::TestItem,
            shared_screen::SharedScreen,
        };

        // Client B activates an external window, which causes a new screen-sharing item to be added to the pane.
        let display = TestScreenCaptureSource::new();
        active_call_b
            .update(cx_b, |call, cx| call.set_location(None, cx))
            .await
            .unwrap();
        cx_b.set_screen_capture_sources(vec![display]);
        active_call_b
            .update(cx_b, |call, cx| {
                call.room()
                    .unwrap()
                    .update(cx, |room, cx| room.share_screen(cx))
            })
            .await
            .unwrap();
        executor.run_until_parked();

        let shared_screen = workspace_a.update(cx_a, |workspace, cx| {
            workspace
                .active_item(cx)
                .expect("no active item")
                .downcast::<SharedScreen>()
                .expect("active item isn't a shared screen")
        });

        // Client B activates Zed again, which causes the previous editor to become focused again.
        active_call_b
            .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
            .await
            .unwrap();
        executor.run_until_parked();
        workspace_a.update(cx_a, |workspace, cx| {
            assert_eq!(
                workspace.active_item(cx).unwrap().item_id(),
                editor_a1.item_id()
            )
        });

        // Client B activates a multibuffer that was created by following client A. Client A returns to that multibuffer.
        workspace_b.update_in(cx_b, |workspace, window, cx| {
            workspace.activate_item(&multibuffer_editor_b, true, true, window, cx)
        });
        executor.run_until_parked();
        workspace_a.update(cx_a, |workspace, cx| {
            assert_eq!(
                workspace.active_item(cx).unwrap().item_id(),
                multibuffer_editor_a.item_id()
            )
        });

        // Client B activates a panel, and the previously-opened screen-sharing item gets activated.
        let panel = cx_b.new(|cx| TestPanel::new(DockPosition::Left, cx));
        workspace_b.update_in(cx_b, |workspace, window, cx| {
            workspace.add_panel(panel, window, cx);
            workspace.toggle_panel_focus::<TestPanel>(window, cx);
        });
        executor.run_until_parked();
        assert_eq!(
            workspace_a.update(cx_a, |workspace, cx| workspace
                .active_item(cx)
                .unwrap()
                .item_id()),
            shared_screen.item_id()
        );

        // Toggling the focus back to the pane causes client A to return to the multibuffer.
        workspace_b.update_in(cx_b, |workspace, window, cx| {
            workspace.toggle_panel_focus::<TestPanel>(window, cx);
        });
        executor.run_until_parked();
        workspace_a.update(cx_a, |workspace, cx| {
            assert_eq!(
                workspace.active_item(cx).unwrap().item_id(),
                multibuffer_editor_a.item_id()
            )
        });

        // Client B activates an item that doesn't implement following,
        // so the previously-opened screen-sharing item gets activated.
        let unfollowable_item = cx_b.new(TestItem::new);
        workspace_b.update_in(cx_b, |workspace, window, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.add_item(Box::new(unfollowable_item), true, true, None, window, cx)
            })
        });
        executor.run_until_parked();
        assert_eq!(
            workspace_a.update(cx_a, |workspace, cx| workspace
                .active_item(cx)
                .unwrap()
                .item_id()),
            shared_screen.item_id()
        );

        // Following interrupts when client B disconnects.
        client_b.disconnect(&cx_b.to_async());
        executor.advance_clock(RECONNECT_TIMEOUT);
        assert_eq!(
            workspace_a.update(cx_a, |workspace, _| workspace.leader_for_pane(&pane_a)),
            None
        );
    }
}

#[gpui::test]
async fn test_following_tab_order(
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
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let pane_a = workspace_a.update(cx_a, |workspace, _| workspace.active_pane().clone());

    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let pane_b = workspace_b.update(cx_b, |workspace, _| workspace.active_pane().clone());

    let client_b_id = project_a.update(cx_a, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });

    //Open 1, 3 in that order on client A
    workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, window, cx)
        })
        .await
        .unwrap();
    workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "3.txt"), None, true, window, cx)
        })
        .await
        .unwrap();

    let pane_paths = |pane: &Entity<workspace::Pane>, cx: &mut VisualTestContext| {
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
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.follow(client_b_id, window, cx)
    });
    executor.run_until_parked();

    //Open just 2 on client B
    workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, window, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();

    // Verify that newly opened followed file is at the end
    assert_eq!(&pane_paths(&pane_a, cx_a), &["1.txt", "3.txt", "2.txt"]);

    //Open just 1 on client B
    workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, window, cx)
        })
        .await
        .unwrap();
    assert_eq!(&pane_paths(&pane_b, cx_b), &["2.txt", "1.txt"]);
    executor.run_until_parked();

    // Verify that following into 1 did not reorder
    assert_eq!(&pane_paths(&pane_a, cx_a), &["1.txt", "3.txt", "2.txt"]);
}

#[gpui::test(iterations = 10)]
async fn test_peers_following_each_other(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
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
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
                "4.txt": "four",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Client B joins the project.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    // Client A opens a file.
    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B opens a different file.
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Clients A and B follow each other in split panes
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.split_and_clone(
            workspace.active_pane().clone(),
            SplitDirection::Right,
            window,
            cx,
        );
    });
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.follow(client_b.peer_id().unwrap(), window, cx)
    });
    executor.run_until_parked();
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.split_and_clone(
            workspace.active_pane().clone(),
            SplitDirection::Right,
            window,
            cx,
        );
    });
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(client_a.peer_id().unwrap(), window, cx)
    });
    executor.run_until_parked();

    // Clients A and B return focus to the original files they had open
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.activate_next_pane(window, cx)
    });
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.activate_next_pane(window, cx)
    });
    executor.run_until_parked();

    // Both clients see the other client's focused file in their right pane.
    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: true,
                leader: None,
                items: vec![(true, "1.txt".into())]
            },
            PaneSummary {
                active: false,
                leader: client_b.peer_id(),
                items: vec![(false, "1.txt".into()), (true, "2.txt".into())]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: true,
                leader: None,
                items: vec![(true, "2.txt".into())]
            },
            PaneSummary {
                active: false,
                leader: client_a.peer_id(),
                items: vec![(false, "2.txt".into()), (true, "1.txt".into())]
            },
        ]
    );

    // Clients A and B each open a new file.
    workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "3.txt"), None, true, window, cx)
        })
        .await
        .unwrap();

    workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "4.txt"), None, true, window, cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();

    // Both client's see the other client open the new file, but keep their
    // focus on their own active pane.
    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: true,
                leader: None,
                items: vec![(false, "1.txt".into()), (true, "3.txt".into())]
            },
            PaneSummary {
                active: false,
                leader: client_b.peer_id(),
                items: vec![
                    (false, "1.txt".into()),
                    (false, "2.txt".into()),
                    (true, "4.txt".into())
                ]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: true,
                leader: None,
                items: vec![(false, "2.txt".into()), (true, "4.txt".into())]
            },
            PaneSummary {
                active: false,
                leader: client_a.peer_id(),
                items: vec![
                    (false, "2.txt".into()),
                    (false, "1.txt".into()),
                    (true, "3.txt".into())
                ]
            },
        ]
    );

    // Client A focuses their right pane, in which they're following client B.
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.activate_next_pane(window, cx)
    });
    executor.run_until_parked();

    // Client B sees that client A is now looking at the same file as them.
    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "1.txt".into()), (true, "3.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: client_b.peer_id(),
                items: vec![
                    (false, "1.txt".into()),
                    (false, "2.txt".into()),
                    (true, "4.txt".into())
                ]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: true,
                leader: None,
                items: vec![(false, "2.txt".into()), (true, "4.txt".into())]
            },
            PaneSummary {
                active: false,
                leader: client_a.peer_id(),
                items: vec![
                    (false, "2.txt".into()),
                    (false, "1.txt".into()),
                    (false, "3.txt".into()),
                    (true, "4.txt".into())
                ]
            },
        ]
    );

    // Client B focuses their right pane, in which they're following client A,
    // who is following them.
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.activate_next_pane(window, cx)
    });
    executor.run_until_parked();

    // Client A sees that client B is now looking at the same file as them.
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "2.txt".into()), (true, "4.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: client_a.peer_id(),
                items: vec![
                    (false, "2.txt".into()),
                    (false, "1.txt".into()),
                    (false, "3.txt".into()),
                    (true, "4.txt".into())
                ]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "1.txt".into()), (true, "3.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: client_b.peer_id(),
                items: vec![
                    (false, "1.txt".into()),
                    (false, "2.txt".into()),
                    (true, "4.txt".into())
                ]
            },
        ]
    );

    // Client B focuses a file that they previously followed A to, breaking
    // the follow.
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.activate_prev_item(true, window, cx);
        });
    });
    executor.run_until_parked();

    // Both clients see that client B is looking at that previous file.
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "2.txt".into()), (true, "4.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: None,
                items: vec![
                    (false, "2.txt".into()),
                    (false, "1.txt".into()),
                    (true, "3.txt".into()),
                    (false, "4.txt".into())
                ]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "1.txt".into()), (true, "3.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: client_b.peer_id(),
                items: vec![
                    (false, "1.txt".into()),
                    (false, "2.txt".into()),
                    (false, "4.txt".into()),
                    (true, "3.txt".into()),
                ]
            },
        ]
    );

    // Client B closes tabs, some of which were originally opened by client A,
    // and some of which were originally opened by client B.
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.close_inactive_items(&Default::default(), window, cx)
                .unwrap()
                .detach();
        });
    });

    executor.run_until_parked();

    // Both clients see that Client B is looking at the previous tab.
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "2.txt".into()), (true, "4.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: None,
                items: vec![(true, "3.txt".into()),]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "1.txt".into()), (true, "3.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: client_b.peer_id(),
                items: vec![
                    (false, "1.txt".into()),
                    (false, "2.txt".into()),
                    (false, "4.txt".into()),
                    (true, "3.txt".into()),
                ]
            },
        ]
    );

    // Client B follows client A again.
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(client_a.peer_id().unwrap(), window, cx)
    });
    executor.run_until_parked();
    // Client A cycles through some tabs.
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.activate_prev_item(true, window, cx);
        });
    });
    executor.run_until_parked();

    // Client B follows client A into those tabs.
    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "1.txt".into()), (true, "3.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: None,
                items: vec![
                    (false, "1.txt".into()),
                    (false, "2.txt".into()),
                    (true, "4.txt".into()),
                    (false, "3.txt".into()),
                ]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "2.txt".into()), (true, "4.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: client_a.peer_id(),
                items: vec![(false, "3.txt".into()), (true, "4.txt".into())]
            },
        ]
    );

    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.activate_prev_item(true, window, cx);
        });
    });
    executor.run_until_parked();

    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "1.txt".into()), (true, "3.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: None,
                items: vec![
                    (false, "1.txt".into()),
                    (true, "2.txt".into()),
                    (false, "4.txt".into()),
                    (false, "3.txt".into()),
                ]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "2.txt".into()), (true, "4.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: client_a.peer_id(),
                items: vec![
                    (false, "3.txt".into()),
                    (false, "4.txt".into()),
                    (true, "2.txt".into())
                ]
            },
        ]
    );

    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.activate_prev_item(true, window, cx);
        });
    });
    executor.run_until_parked();

    assert_eq!(
        pane_summaries(&workspace_a, cx_a),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "1.txt".into()), (true, "3.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: None,
                items: vec![
                    (true, "1.txt".into()),
                    (false, "2.txt".into()),
                    (false, "4.txt".into()),
                    (false, "3.txt".into()),
                ]
            },
        ]
    );
    assert_eq!(
        pane_summaries(&workspace_b, cx_b),
        &[
            PaneSummary {
                active: false,
                leader: None,
                items: vec![(false, "2.txt".into()), (true, "4.txt".into())]
            },
            PaneSummary {
                active: true,
                leader: client_a.peer_id(),
                items: vec![
                    (false, "3.txt".into()),
                    (false, "4.txt".into()),
                    (false, "2.txt".into()),
                    (true, "1.txt".into()),
                ]
            },
        ]
    );
}

#[gpui::test(iterations = 10)]
async fn test_auto_unfollowing(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    // 2 clients connect to a server.
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
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
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "1.txt": "one",
                "2.txt": "two",
                "3.txt": "three",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    let _editor_a1 = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B starts following client A.
    let pane_b = workspace_b.update(cx_b, |workspace, _| workspace.active_pane().clone());
    let leader_id = project_b.update(cx_b, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(leader_id, window, cx)
    });
    executor.run_until_parked();
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );
    let editor_b2 = workspace_b.update(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });

    // When client B moves, it automatically stops following client A.
    editor_b2.update_in(cx_b, |editor, window, cx| {
        editor.move_right(&editor::actions::MoveRight, window, cx)
    });
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        None
    );

    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(leader_id, window, cx)
    });
    executor.run_until_parked();
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B edits, it automatically stops following client A.
    editor_b2.update_in(cx_b, |editor, window, cx| editor.insert("X", window, cx));
    assert_eq!(
        workspace_b.update_in(cx_b, |workspace, _, _| workspace.leader_for_pane(&pane_b)),
        None
    );

    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(leader_id, window, cx)
    });
    executor.run_until_parked();
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B scrolls, it automatically stops following client A.
    editor_b2.update_in(cx_b, |editor, window, cx| {
        editor.set_scroll_position(point(0., 3.), window, cx)
    });
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        None
    );

    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(leader_id, window, cx)
    });
    executor.run_until_parked();
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B activates a different pane, it continues following client A in the original pane.
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.split_and_clone(pane_b.clone(), SplitDirection::Right, window, cx)
    });
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.activate_next_pane(window, cx)
    });
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B activates a different item in the original pane, it automatically stops following client A.
    workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, window, cx)
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_b.update(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        None
    );
}

#[gpui::test(iterations = 10)]
async fn test_peers_simultaneously_following_each_other(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    client_a.fs().insert_tree("/a", json!({})).await;
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    executor.run_until_parked();
    let client_a_id = project_b.update(cx_b, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });
    let client_b_id = project_a.update(cx_a, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });

    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.follow(client_b_id, window, cx)
    });
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(client_a_id, window, cx)
    });
    executor.run_until_parked();

    workspace_a.update(cx_a, |workspace, _| {
        assert_eq!(
            workspace.leader_for_pane(workspace.active_pane()),
            Some(client_b_id)
        );
    });
    workspace_b.update(cx_b, |workspace, _| {
        assert_eq!(
            workspace.leader_for_pane(workspace.active_pane()),
            Some(client_a_id)
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_following_across_workspaces(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    // a and b join a channel/call
    // a shares project 1
    // b shares project 2
    //
    // b follows a: causes project 2 to be joined, and b to follow a.
    // b opens a different file in project 2, a follows b
    // b opens a different file in project 1, a cannot follow b
    // b shares the project, a joins the project and follows b
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "w.rs": "",
                "x.rs": "",
            }),
        )
        .await;

    client_b
        .fs()
        .insert_tree(
            path!("/b"),
            json!({
                "y.rs": "",
                "z.rs": "",
            }),
        )
        .await;

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let (project_a, worktree_id_a) = client_a.build_local_project(path!("/a"), cx_a).await;
    let (project_b, worktree_id_b) = client_b.build_local_project(path!("/b"), cx_b).await;

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id_a, "w.rs"), None, true, window, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();
    assert_eq!(visible_push_notifications(cx_b).len(), 1);

    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(client_a.peer_id().unwrap(), window, cx)
    });

    executor.run_until_parked();
    let window_b_project_a = *cx_b
        .windows()
        .iter()
        .max_by_key(|window| window.window_id())
        .unwrap();

    let mut cx_b2 = VisualTestContext::from_window(window_b_project_a, cx_b);

    let workspace_b_project_a = window_b_project_a
        .downcast::<Workspace>()
        .unwrap()
        .root(cx_b)
        .unwrap();

    // assert that b is following a in project a in w.rs
    workspace_b_project_a.update(&mut cx_b2, |workspace, cx| {
        assert!(workspace.is_being_followed(client_a.peer_id().unwrap()));
        assert_eq!(
            client_a.peer_id(),
            workspace.leader_for_pane(workspace.active_pane())
        );
        let item = workspace.active_item(cx).unwrap();
        assert_eq!(
            item.tab_content_text(0, cx),
            SharedString::from("w.rs")
        );
    });

    // TODO: in app code, this would be done by the collab_ui.
    active_call_b
        .update(&mut cx_b2, |call, cx| {
            let project = workspace_b_project_a.read(cx).project().clone();
            call.set_location(Some(&project), cx)
        })
        .await
        .unwrap();

    // assert that there are no share notifications open
    assert_eq!(visible_push_notifications(cx_b).len(), 0);

    // b moves to x.rs in a's project, and a follows
    workspace_b_project_a
        .update_in(&mut cx_b2, |workspace, window, cx| {
            workspace.open_path((worktree_id_a, "x.rs"), None, true, window, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();
    workspace_b_project_a.update(&mut cx_b2, |workspace, cx| {
        let item = workspace.active_item(cx).unwrap();
        assert_eq!(
            item.tab_content_text(0, cx),
            SharedString::from("x.rs")
        );
    });

    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.follow(client_b.peer_id().unwrap(), window, cx)
    });

    executor.run_until_parked();
    workspace_a.update(cx_a, |workspace, cx| {
        assert!(workspace.is_being_followed(client_b.peer_id().unwrap()));
        assert_eq!(
            client_b.peer_id(),
            workspace.leader_for_pane(workspace.active_pane())
        );
        let item = workspace.active_pane().read(cx).active_item().unwrap();
        assert_eq!(item.tab_content_text(0, cx), "x.rs");
    });

    // b moves to y.rs in b's project, a is still following but can't yet see
    workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id_b, "y.rs"), None, true, window, cx)
        })
        .await
        .unwrap();

    // TODO: in app code, this would be done by the collab_ui.
    active_call_b
        .update(cx_b, |call, cx| {
            let project = workspace_b.read(cx).project().clone();
            call.set_location(Some(&project), cx)
        })
        .await
        .unwrap();

    let project_b_id = active_call_b
        .update(cx_b, |call, cx| call.share_project(project_b.clone(), cx))
        .await
        .unwrap();

    executor.run_until_parked();
    assert_eq!(visible_push_notifications(cx_a).len(), 1);
    cx_a.update(|_, cx| {
        workspace::join_in_room_project(
            project_b_id,
            client_b.user_id().unwrap(),
            client_a.app_state.clone(),
            cx,
        )
    })
    .await
    .unwrap();

    executor.run_until_parked();

    assert_eq!(visible_push_notifications(cx_a).len(), 0);
    let window_a_project_b = *cx_a
        .windows()
        .iter()
        .max_by_key(|window| window.window_id())
        .unwrap();
    let cx_a2 = &mut VisualTestContext::from_window(window_a_project_b, cx_a);
    let workspace_a_project_b = window_a_project_b
        .downcast::<Workspace>()
        .unwrap()
        .root(cx_a)
        .unwrap();

    workspace_a_project_b.update(cx_a2, |workspace,  cx| {
        assert_eq!(workspace.project().read(cx).remote_id(), Some(project_b_id));
        assert!(workspace.is_being_followed(client_b.peer_id().unwrap()));
        assert_eq!(
            client_b.peer_id(),
            workspace.leader_for_pane(workspace.active_pane())
        );
        let item = workspace.active_item(cx).unwrap();
        assert_eq!(
            item.tab_content_text(0, cx),
            SharedString::from("y.rs")
        );
    });
}

#[gpui::test]
async fn test_following_stops_on_unshare(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let (_server, client_a, client_b, channel_id) = TestServer::start2(cx_a, cx_b).await;

    let (workspace_a, cx_a) = client_a.build_test_workspace(cx_a).await;
    client_a
        .host_workspace(&workspace_a, channel_id, cx_a)
        .await;
    let (workspace_b, cx_b) = client_b.join_workspace(channel_id, cx_b).await;

    cx_a.simulate_keystrokes("cmd-p");
    cx_a.run_until_parked();
    cx_a.simulate_keystrokes("2 enter");

    let editor_a = workspace_a.update(cx_a, |workspace, cx| {
        workspace.active_item_as::<Editor>(cx).unwrap()
    });
    let editor_b = workspace_b.update(cx_b, |workspace, cx| {
        workspace.active_item_as::<Editor>(cx).unwrap()
    });

    // b should follow a to position 1
    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([1..1]))
    });
    cx_a.executor()
        .advance_clock(workspace::item::LEADER_UPDATE_THROTTLE);
    cx_a.run_until_parked();
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(editor.selections.ranges(cx), vec![1..1])
    });

    // a unshares the project
    cx_a.update(|_, cx| {
        let project = workspace_a.read(cx).project().clone();
        ActiveCall::global(cx).update(cx, |call, cx| {
            call.unshare_project(project, cx).unwrap();
        })
    });
    cx_a.run_until_parked();

    // b should not follow a to position 2
    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([2..2]))
    });
    cx_a.executor()
        .advance_clock(workspace::item::LEADER_UPDATE_THROTTLE);
    cx_a.run_until_parked();
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(editor.selections.ranges(cx), vec![1..1])
    });
    cx_b.update(|_, cx| {
        let room = ActiveCall::global(cx).read(cx).room().unwrap().read(cx);
        let participant = room.remote_participants().get(&client_a.id()).unwrap();
        assert_eq!(participant.location, ParticipantLocation::UnsharedProject)
    })
}

#[gpui::test]
async fn test_following_into_excluded_file(
    mut cx_a: &mut TestAppContext,
    mut cx_b: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    for cx in [&mut cx_a, &mut cx_b] {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<WorktreeSettings>(cx, |settings| {
                    settings.file_scan_exclusions = Some(vec!["**/.git".to_string()]);
                });
            });
        });
    }
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let peer_id_a = client_a.peer_id().unwrap();

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                ".git": {
                    "COMMIT_EDITMSG": "write your commit message here",
                },
                "1.txt": "one\none\none",
                "2.txt": "two\ntwo\ntwo",
                "3.txt": "three\nthree\nthree",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    // Client A opens editors for a regular file and an excluded file.
    let editor_for_regular = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let editor_for_excluded_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, ".git/COMMIT_EDITMSG"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client A updates their selections in those editors
    editor_for_regular.update_in(cx_a, |editor, window, cx| {
        editor.handle_input("a", window, cx);
        editor.handle_input("b", window, cx);
        editor.handle_input("c", window, cx);
        editor.select_left(&Default::default(), window, cx);
        assert_eq!(editor.selections.ranges(cx), vec![3..2]);
    });
    editor_for_excluded_a.update_in(cx_a, |editor, window, cx| {
        editor.select_all(&Default::default(), window, cx);
        editor.handle_input("new commit message", window, cx);
        editor.select_left(&Default::default(), window, cx);
        assert_eq!(editor.selections.ranges(cx), vec![18..17]);
    });

    // When client B starts following client A, currently visible file is replicated
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.follow(peer_id_a, window, cx)
    });
    executor.advance_clock(workspace::item::LEADER_UPDATE_THROTTLE);
    executor.run_until_parked();

    let editor_for_excluded_b = workspace_b.update(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });
    assert_eq!(
        cx_b.read(|cx| editor_for_excluded_b.project_path(cx)),
        Some((worktree_id, ".git/COMMIT_EDITMSG").into())
    );
    assert_eq!(
        editor_for_excluded_b.update(cx_b, |editor, cx| editor.selections.ranges(cx)),
        vec![18..17]
    );

    editor_for_excluded_a.update_in(cx_a, |editor, window, cx| {
        editor.select_right(&Default::default(), window, cx);
    });
    executor.advance_clock(workspace::item::LEADER_UPDATE_THROTTLE);
    executor.run_until_parked();

    // Changes from B to the excluded file are replicated in A's editor
    editor_for_excluded_b.update_in(cx_b, |editor, window, cx| {
        editor.handle_input("\nCo-Authored-By: B <b@b.b>", window, cx);
    });
    executor.run_until_parked();
    editor_for_excluded_a.update(cx_a, |editor, cx| {
        assert_eq!(
            editor.text(cx),
            "new commit message\nCo-Authored-By: B <b@b.b>"
        );
    });
}

fn visible_push_notifications(cx: &mut TestAppContext) -> Vec<Entity<ProjectSharedNotification>> {
    let mut ret = Vec::new();
    for window in cx.windows() {
        window
            .update(cx, |window, _, _| {
                if let Ok(handle) = window.downcast::<ProjectSharedNotification>() {
                    ret.push(handle)
                }
            })
            .unwrap();
    }
    ret
}

#[derive(Debug, PartialEq, Eq)]
struct PaneSummary {
    active: bool,
    leader: Option<PeerId>,
    items: Vec<(bool, String)>,
}

fn followers_by_leader(project_id: u64, cx: &TestAppContext) -> Vec<(PeerId, Vec<PeerId>)> {
    cx.read(|cx| {
        let active_call = ActiveCall::global(cx).read(cx);
        let peer_id = active_call.client().peer_id();
        let room = active_call.room().unwrap().read(cx);
        let mut result = room
            .remote_participants()
            .values()
            .map(|participant| participant.peer_id)
            .chain(peer_id)
            .filter_map(|peer_id| {
                let followers = room.followers_for(peer_id, project_id);
                if followers.is_empty() {
                    None
                } else {
                    Some((peer_id, followers.to_vec()))
                }
            })
            .collect::<Vec<_>>();
        result.sort_by_key(|e| e.0);
        result
    })
}

fn pane_summaries(workspace: &Entity<Workspace>, cx: &mut VisualTestContext) -> Vec<PaneSummary> {
    workspace.update(cx, |workspace, cx| {
        let active_pane = workspace.active_pane();
        workspace
            .panes()
            .iter()
            .map(|pane| {
                let leader = workspace.leader_for_pane(pane);
                let active = pane == active_pane;
                let pane = pane.read(cx);
                let active_ix = pane.active_item_index();
                PaneSummary {
                    active,
                    leader,
                    items: pane
                        .items()
                        .enumerate()
                        .map(|(ix, item)| {
                            (
                                ix == active_ix,
                                item.tab_content_text(0, cx)
                                    .map_or(String::new(), |s| s.to_string()),
                            )
                        })
                        .collect(),
                }
            })
            .collect()
    })
}

#[gpui::test(iterations = 10)]
async fn test_following_to_channel_notes_without_a_shared_project(
    deterministic: BackgroundExecutor,
    mut cx_a: &mut TestAppContext,
    mut cx_b: &mut TestAppContext,
    mut cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(deterministic.clone()).await;
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
    client_a
        .fs()
        .insert_tree("/a", json!({ "1.txt": "" }))
        .await;
    client_b.fs().insert_tree("/b", json!({})).await;
    client_c.fs().insert_tree("/c", json!({})).await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let (project_b, _) = client_b.build_local_project("/b", cx_b).await;
    let (project_c, _) = client_b.build_local_project("/c", cx_c).await;
    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let (_workspace_c, _cx_c) = client_c.build_workspace(&project_c, cx_c);

    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    // Client A opens the notes for channel 1.
    let channel_notes_1_a = cx_a
        .update(|window, cx| ChannelView::open(channel_1_id, None, workspace_a.clone(), window, cx))
        .await
        .unwrap();
    channel_notes_1_a.update_in(cx_a, |notes, window, cx| {
        assert_eq!(notes.channel(cx).unwrap().name, "channel-1");
        notes.editor.update(cx, |editor, cx| {
            editor.insert("Hello from A.", window, cx);
            editor.change_selections(None, window, cx, |selections| {
                selections.select_ranges(vec![3..4]);
            });
        });
    });

    // Client B follows client A.
    workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace
                .start_following(client_a.peer_id().unwrap(), window, cx)
                .unwrap()
        })
        .await
        .unwrap();

    // Client B is taken to the notes for channel 1, with the same
    // text selected as client A.
    deterministic.run_until_parked();
    let channel_notes_1_b = workspace_b.update(cx_b, |workspace, cx| {
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
    channel_notes_1_b.update(cx_b, |notes, cx| {
        assert_eq!(notes.channel(cx).unwrap().name, "channel-1");
        notes.editor.update(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Hello from A.");
            assert_eq!(editor.selections.ranges::<usize>(cx), &[3..4]);
        })
    });

    //  Client A opens the notes for channel 2.
    let channel_notes_2_a = cx_a
        .update(|window, cx| ChannelView::open(channel_2_id, None, workspace_a.clone(), window, cx))
        .await
        .unwrap();
    channel_notes_2_a.update(cx_a, |notes, cx| {
        assert_eq!(notes.channel(cx).unwrap().name, "channel-2");
    });

    // Client B is taken to the notes for channel 2.
    deterministic.run_until_parked();
    let channel_notes_2_b = workspace_b.update(cx_b, |workspace, cx| {
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
    channel_notes_2_b.update(cx_b, |notes, cx| {
        assert_eq!(notes.channel(cx).unwrap().name, "channel-2");
    });

    // Client A opens a local buffer in their unshared project.
    let _unshared_editor_a1 = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // This does not send any leader update message to client B.
    // If it did, an error would occur on client B, since this buffer
    // is not shared with them.
    deterministic.run_until_parked();
    workspace_b.update(cx_b, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).expect("no active item").item_id(),
            channel_notes_2_b.entity_id()
        );
    });
}

pub(crate) async fn join_channel(
    channel_id: ChannelId,
    client: &TestClient,
    cx: &mut TestAppContext,
) -> anyhow::Result<()> {
    cx.update(|cx| workspace::join_channel(channel_id, client.app_state.clone(), None, cx))
        .await
}

async fn share_workspace(
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> anyhow::Result<u64> {
    let project = workspace.update(cx, |workspace, _| workspace.project().clone());
    cx.read(ActiveCall::global)
        .update(cx, |call, cx| call.share_project(project, cx))
        .await
}

#[gpui::test]
async fn test_following_after_replacement(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let (_server, client_a, client_b, channel) = TestServer::start2(cx_a, cx_b).await;

    let (workspace, cx_a) = client_a.build_test_workspace(cx_a).await;
    join_channel(channel, &client_a, cx_a).await.unwrap();
    share_workspace(&workspace, cx_a).await.unwrap();
    let buffer = workspace.update(cx_a, |workspace, cx| {
        workspace.project().update(cx, |project, cx| {
            project.create_local_buffer(&sample_text(26, 5, 'a'), None, cx)
        })
    });
    let multibuffer = cx_a.new(|cx| {
        let mut mb = MultiBuffer::new(Capability::ReadWrite);
        mb.set_excerpts_for_path(
            PathKey::for_buffer(&buffer, cx),
            buffer.clone(),
            [Point::row_range(1..1), Point::row_range(5..5)],
            1,
            cx,
        );
        mb
    });
    let snapshot = buffer.update(cx_a, |buffer, _| buffer.snapshot());
    let editor: Entity<Editor> = cx_a.new_window_entity(|window, cx| {
        Editor::for_multibuffer(
            multibuffer.clone(),
            Some(workspace.read(cx).project().clone()),
            window,
            cx,
        )
    });
    workspace.update_in(cx_a, |workspace, window, cx| {
        workspace.add_item_to_center(Box::new(editor.clone()) as _, window, cx)
    });
    editor.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::row_range(4..4)]);
        })
    });
    let positions = editor.update(cx_a, |editor, _| {
        editor
            .selections
            .disjoint_anchor_ranges()
            .map(|range| range.start.text_anchor.to_point(&snapshot))
            .collect::<Vec<_>>()
    });
    multibuffer.update(cx_a, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            PathKey::for_buffer(&buffer, cx),
            buffer,
            [Point::row_range(1..5)],
            1,
            cx,
        );
    });

    let (workspace_b, cx_b) = client_b.join_workspace(channel, cx_b).await;
    cx_b.run_until_parked();
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace
                .active_item(cx)
                .and_then(|item| item.downcast::<Editor>())
        })
        .unwrap();

    let new_positions = editor_b.update(cx_b, |editor, _| {
        editor
            .selections
            .disjoint_anchor_ranges()
            .map(|range| range.start.text_anchor.to_point(&snapshot))
            .collect::<Vec<_>>()
    });
    assert_eq!(positions, new_positions);
}

#[gpui::test]
async fn test_following_to_channel_notes_other_workspace(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let (_server, client_a, client_b, channel) = TestServer::start2(cx_a, cx_b).await;

    let mut cx_a2 = cx_a.clone();
    let (workspace_a, cx_a) = client_a.build_test_workspace(cx_a).await;
    join_channel(channel, &client_a, cx_a).await.unwrap();
    share_workspace(&workspace_a, cx_a).await.unwrap();

    // a opens 1.txt
    cx_a.simulate_keystrokes("cmd-p");
    cx_a.run_until_parked();
    cx_a.simulate_keystrokes("1 enter");
    cx_a.run_until_parked();
    workspace_a.update(cx_a, |workspace, cx| {
        let editor = workspace.active_item(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "1.txt");
    });

    // b joins channel and is following a
    join_channel(channel, &client_b, cx_b).await.unwrap();
    cx_b.run_until_parked();
    let (workspace_b, cx_b) = client_b.active_workspace(cx_b);
    workspace_b.update(cx_b, |workspace, cx| {
        let editor = workspace.active_item(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "1.txt");
    });

    // a opens a second workspace and the channel notes
    let (workspace_a2, cx_a2) = client_a.build_test_workspace(&mut cx_a2).await;
    cx_a2.update(|window, _| window.activate_window());
    cx_a2
        .update(|window, cx| ChannelView::open(channel, None, workspace_a2, window, cx))
        .await
        .unwrap();
    cx_a2.run_until_parked();

    // b should follow a to the channel notes
    workspace_b.update(cx_b, |workspace, cx| {
        let editor = workspace.active_item_as::<ChannelView>(cx).unwrap();
        assert_eq!(editor.read(cx).channel(cx).unwrap().id, channel);
    });

    // a returns to the shared project
    cx_a.update(|window, _| window.activate_window());
    cx_a.run_until_parked();

    workspace_a.update(cx_a, |workspace, cx| {
        let editor = workspace.active_item(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "1.txt");
    });

    // b should follow a back
    workspace_b.update(cx_b, |workspace, cx| {
        let editor = workspace.active_item_as::<Editor>(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "1.txt");
    });
}

#[gpui::test]
async fn test_following_while_deactivated(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let (_server, client_a, client_b, channel) = TestServer::start2(cx_a, cx_b).await;

    let mut cx_a2 = cx_a.clone();
    let (workspace_a, cx_a) = client_a.build_test_workspace(cx_a).await;
    join_channel(channel, &client_a, cx_a).await.unwrap();
    share_workspace(&workspace_a, cx_a).await.unwrap();

    // a opens 1.txt
    cx_a.simulate_keystrokes("cmd-p");
    cx_a.run_until_parked();
    cx_a.simulate_keystrokes("1 enter");
    cx_a.run_until_parked();
    workspace_a.update(cx_a, |workspace, cx| {
        let editor = workspace.active_item(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "1.txt");
    });

    // b joins channel and is following a
    join_channel(channel, &client_b, cx_b).await.unwrap();
    cx_b.run_until_parked();
    let (workspace_b, cx_b) = client_b.active_workspace(cx_b);
    workspace_b.update(cx_b, |workspace, cx| {
        let editor = workspace.active_item(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "1.txt");
    });

    // stop following
    cx_b.simulate_keystrokes("down");

    // a opens a different file while not followed
    cx_a.simulate_keystrokes("cmd-p");
    cx_a.run_until_parked();
    cx_a.simulate_keystrokes("2 enter");

    workspace_b.update(cx_b, |workspace, cx| {
        let editor = workspace.active_item_as::<Editor>(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "1.txt");
    });

    // a opens a file in a new window
    let (_, cx_a2) = client_a.build_test_workspace(&mut cx_a2).await;
    cx_a2.update(|window, _| window.activate_window());
    cx_a2.simulate_keystrokes("cmd-p");
    cx_a2.run_until_parked();
    cx_a2.simulate_keystrokes("3 enter");
    cx_a2.run_until_parked();

    // b starts following a again
    cx_b.simulate_keystrokes("cmd-ctrl-alt-f");
    cx_a.run_until_parked();

    // a returns to the shared project
    cx_a.update(|window, _| window.activate_window());
    cx_a.run_until_parked();

    workspace_a.update(cx_a, |workspace, cx| {
        let editor = workspace.active_item(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "2.js");
    });

    // b should follow a back
    workspace_b.update(cx_b, |workspace, cx| {
        let editor = workspace.active_item_as::<Editor>(cx).unwrap();
        assert_eq!(editor.tab_content_text(0, cx), "2.js");
    });
}
