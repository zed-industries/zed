use crate::{rpc::RECONNECT_TIMEOUT, tests::TestServer};
use call::ActiveCall;
use collab_ui::project_shared_notification::ProjectSharedNotification;
use editor::{Editor, ExcerptRange, MultiBuffer};
use gpui::{executor::Deterministic, geometry::vector::vec2f, TestAppContext, ViewHandle};
use live_kit_client::MacOSDisplay;
use rpc::proto::PeerId;
use serde_json::json;
use std::{borrow::Cow, sync::Arc};
use workspace::{
    dock::{test::TestPanel, DockPosition},
    item::{test::TestItem, ItemHandle as _},
    shared_screen::SharedScreen,
    SplitDirection, Workspace,
};

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
        .fs()
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

    let window_a = client_a.build_workspace(&project_a, cx_a);
    let workspace_a = window_a.root(cx_a);
    let window_b = client_b.build_workspace(&project_b, cx_b);
    let workspace_b = window_b.root(cx_b);

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
            workspace.follow(peer_id_a, cx).unwrap()
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
    let window_c = client_c.build_workspace(&project_c, cx_c);
    let workspace_c = window_c.root(cx_c);
    active_call_c
        .update(cx_c, |call, cx| call.set_location(Some(&project_c), cx))
        .await
        .unwrap();
    drop(project_c);

    // Client C also follows client A.
    workspace_c
        .update(cx_c, |workspace, cx| {
            workspace.follow(peer_id_a, cx).unwrap()
        })
        .await
        .unwrap();

    cx_d.foreground().run_until_parked();
    let active_call_d = cx_d.read(ActiveCall::global);
    let project_d = client_d.build_remote_project(project_id, cx_d).await;
    let workspace_d = client_d.build_workspace(&project_d, cx_d).root(cx_d);
    active_call_d
        .update(cx_d, |call, cx| call.set_location(Some(&project_d), cx))
        .await
        .unwrap();
    drop(project_d);

    // All clients see that clients B and C are following client A.
    cx_c.foreground().run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("C", &cx_c), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[(peer_id_a, vec![peer_id_b, peer_id_c])],
            "followers seen by {name}"
        );
    }

    // Client C unfollows client A.
    workspace_c.update(cx_c, |workspace, cx| {
        workspace.unfollow(&workspace.active_pane().clone(), cx);
    });

    // All clients see that clients B is following client A.
    cx_c.foreground().run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("C", &cx_c), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[(peer_id_a, vec![peer_id_b])],
            "followers seen by {name}"
        );
    }

    // Client C re-follows client A.
    workspace_c
        .update(cx_c, |workspace, cx| {
            workspace.follow(peer_id_a, cx).unwrap()
        })
        .await
        .unwrap();

    // All clients see that clients B and C are following client A.
    cx_c.foreground().run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("C", &cx_c), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[(peer_id_a, vec![peer_id_b, peer_id_c])],
            "followers seen by {name}"
        );
    }

    // Client D follows client B, then switches to following client C.
    workspace_d
        .update(cx_d, |workspace, cx| {
            workspace.follow(peer_id_b, cx).unwrap()
        })
        .await
        .unwrap();
    workspace_d
        .update(cx_d, |workspace, cx| {
            workspace.follow(peer_id_c, cx).unwrap()
        })
        .await
        .unwrap();

    // All clients see that D is following C
    cx_d.foreground().run_until_parked();
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
    window_c.remove(cx_c);
    cx_c.drop_last(workspace_c);

    // Clients A and B see that client B is following A, and client C is not present in the followers.
    cx_c.foreground().run_until_parked();
    for (name, cx) in [("A", &cx_a), ("B", &cx_b), ("C", &cx_c), ("D", &cx_d)] {
        assert_eq!(
            followers_by_leader(project_id, cx),
            &[(peer_id_a, vec![peer_id_b]),],
            "followers seen by {name}"
        );
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
            workspace.follow(peer_id_b, cx).unwrap()
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
            .expect("no active item")
            .downcast::<SharedScreen>()
            .expect("active item isn't a shared screen")
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

    // Client B activates a panel, and the previously-opened screen-sharing item gets activated.
    let panel = window_b.add_view(cx_b, |_| TestPanel::new(DockPosition::Left));
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.add_panel(panel, cx);
        workspace.toggle_panel_focus::<TestPanel>(cx);
    });
    deterministic.run_until_parked();
    assert_eq!(
        workspace_a.read_with(cx_a, |workspace, cx| workspace
            .active_item(cx)
            .unwrap()
            .id()),
        shared_screen.id()
    );

    // Toggling the focus back to the pane causes client A to return to the multibuffer.
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.toggle_panel_focus::<TestPanel>(cx);
    });
    deterministic.run_until_parked();
    workspace_a.read_with(cx_a, |workspace, cx| {
        assert_eq!(
            workspace.active_item(cx).unwrap().id(),
            multibuffer_editor_a.id()
        )
    });

    // Client B activates an item that doesn't implement following,
    // so the previously-opened screen-sharing item gets activated.
    let unfollowable_item = window_b.add_view(cx_b, |_| TestItem::new());
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.add_item(Box::new(unfollowable_item), true, true, None, cx)
        })
    });
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
        .fs()
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

    let workspace_a = client_a.build_workspace(&project_a, cx_a).root(cx_a);
    let pane_a = workspace_a.read_with(cx_a, |workspace, _| workspace.active_pane().clone());

    let workspace_b = client_b.build_workspace(&project_b, cx_b).root(cx_b);
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
            workspace.follow(client_b_id, cx).unwrap()
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
        .fs()
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

    // Client A opens a file.
    let workspace_a = client_a.build_workspace(&project_a, cx_a).root(cx_a);
    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B opens a different file.
    let workspace_b = client_b.build_workspace(&project_b, cx_b).root(cx_b);
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "2.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Clients A and B follow each other in split panes
    workspace_a.update(cx_a, |workspace, cx| {
        workspace.split_and_clone(workspace.active_pane().clone(), SplitDirection::Right, cx);
    });
    workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.follow(client_b.peer_id().unwrap(), cx).unwrap()
        })
        .await
        .unwrap();
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.split_and_clone(workspace.active_pane().clone(), SplitDirection::Right, cx);
    });
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.follow(client_a.peer_id().unwrap(), cx).unwrap()
        })
        .await
        .unwrap();

    // Clients A and B return focus to the original files they had open
    workspace_a.update(cx_a, |workspace, cx| workspace.activate_next_pane(cx));
    workspace_b.update(cx_b, |workspace, cx| workspace.activate_next_pane(cx));
    deterministic.run_until_parked();

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
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "3.txt"), None, true, cx)
        })
        .await
        .unwrap();

    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "4.txt"), None, true, cx)
        })
        .await
        .unwrap();
    deterministic.run_until_parked();

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
    workspace_a.update(cx_a, |workspace, cx| workspace.activate_next_pane(cx));
    deterministic.run_until_parked();

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
    workspace_b.update(cx_b, |workspace, cx| workspace.activate_next_pane(cx));
    deterministic.run_until_parked();

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
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.activate_prev_item(true, cx);
        });
    });
    deterministic.run_until_parked();

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
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.close_inactive_items(&Default::default(), cx)
                .unwrap()
                .detach();
        });
    });

    deterministic.run_until_parked();

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
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.follow(client_a.peer_id().unwrap(), cx).unwrap()
        })
        .await
        .unwrap();

    // Client A cycles through some tabs.
    workspace_a.update(cx_a, |workspace, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.activate_prev_item(true, cx);
        });
    });
    deterministic.run_until_parked();

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

    workspace_a.update(cx_a, |workspace, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.activate_prev_item(true, cx);
        });
    });
    deterministic.run_until_parked();

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

    workspace_a.update(cx_a, |workspace, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            pane.activate_prev_item(true, cx);
        });
    });
    deterministic.run_until_parked();

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
        .fs()
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
    let workspace_a = client_a.build_workspace(&project_a, cx_a).root(cx_a);
    let _editor_a1 = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "1.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B starts following client A.
    let workspace_b = client_b.build_workspace(&project_b, cx_b).root(cx_b);
    let pane_b = workspace_b.read_with(cx_b, |workspace, _| workspace.active_pane().clone());
    let leader_id = project_b.read_with(cx_b, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.follow(leader_id, cx).unwrap()
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
            workspace.follow(leader_id, cx).unwrap()
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
            workspace.follow(leader_id, cx).unwrap()
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
            workspace.follow(leader_id, cx).unwrap()
        })
        .await
        .unwrap();
    assert_eq!(
        workspace_b.read_with(cx_b, |workspace, _| workspace.leader_for_pane(&pane_b)),
        Some(leader_id)
    );

    // When client B activates a different pane, it continues following client A in the original pane.
    workspace_b.update(cx_b, |workspace, cx| {
        workspace.split_and_clone(pane_b.clone(), SplitDirection::Right, cx)
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

    client_a.fs().insert_tree("/a", json!({})).await;
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
    let workspace_a = client_a.build_workspace(&project_a, cx_a).root(cx_a);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_remote_project(project_id, cx_b).await;
    let workspace_b = client_b.build_workspace(&project_b, cx_b).root(cx_b);

    deterministic.run_until_parked();
    let client_a_id = project_b.read_with(cx_b, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });
    let client_b_id = project_a.read_with(cx_a, |project, _| {
        project.collaborators().values().next().unwrap().peer_id
    });

    let a_follow_b = workspace_a.update(cx_a, |workspace, cx| {
        workspace.follow(client_b_id, cx).unwrap()
    });
    let b_follow_a = workspace_b.update(cx_b, |workspace, cx| {
        workspace.follow(client_a_id, cx).unwrap()
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
async fn test_following_across_workspaces(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    // a and b join a channel/call
    // a shares project 1
    // b shares project 2
    //
    // b follows a: causes project 2 to be joined, and b to follow a.
    // b opens a different file in project 2, a follows b
    // b opens a different file in project 1, a cannot follow b
    // b shares the project, a joins the project and follows b
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    cx_a.update(editor::init);
    cx_b.update(editor::init);

    client_a
        .fs()
        .insert_tree(
            "/a",
            json!({
                "w.rs": "",
                "x.rs": "",
            }),
        )
        .await;

    client_b
        .fs()
        .insert_tree(
            "/b",
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

    let (project_a, worktree_id_a) = client_a.build_local_project("/a", cx_a).await;
    let (project_b, worktree_id_b) = client_b.build_local_project("/b", cx_b).await;

    let workspace_a = client_a.build_workspace(&project_a, cx_a).root(cx_a);
    let workspace_b = client_b.build_workspace(&project_b, cx_b).root(cx_b);

    cx_a.update(|cx| collab_ui::init(&client_a.app_state, cx));
    cx_b.update(|cx| collab_ui::init(&client_b.app_state, cx));

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
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id_a, "w.rs"), None, true, cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();
    assert_eq!(visible_push_notifications(cx_b).len(), 1);

    workspace_b.update(cx_b, |workspace, cx| {
        workspace
            .follow(client_a.peer_id().unwrap(), cx)
            .unwrap()
            .detach()
    });

    deterministic.run_until_parked();
    let workspace_b_project_a = cx_b
        .windows()
        .iter()
        .max_by_key(|window| window.id())
        .unwrap()
        .downcast::<Workspace>()
        .unwrap()
        .root(cx_b);

    // assert that b is following a in project a in w.rs
    workspace_b_project_a.update(cx_b, |workspace, cx| {
        assert!(workspace.is_being_followed(client_a.peer_id().unwrap()));
        assert_eq!(
            client_a.peer_id(),
            workspace.leader_for_pane(workspace.active_pane())
        );
        let item = workspace.active_item(cx).unwrap();
        assert_eq!(item.tab_description(0, cx).unwrap(), Cow::Borrowed("w.rs"));
    });

    // TODO: in app code, this would be done by the collab_ui.
    active_call_b
        .update(cx_b, |call, cx| {
            let project = workspace_b_project_a.read(cx).project().clone();
            call.set_location(Some(&project), cx)
        })
        .await
        .unwrap();

    // assert that there are no share notifications open
    assert_eq!(visible_push_notifications(cx_b).len(), 0);

    // b moves to x.rs in a's project, and a follows
    workspace_b_project_a
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id_a, "x.rs"), None, true, cx)
        })
        .await
        .unwrap();

    deterministic.run_until_parked();
    workspace_b_project_a.update(cx_b, |workspace, cx| {
        let item = workspace.active_item(cx).unwrap();
        assert_eq!(item.tab_description(0, cx).unwrap(), Cow::Borrowed("x.rs"));
    });

    workspace_a.update(cx_a, |workspace, cx| {
        workspace
            .follow(client_b.peer_id().unwrap(), cx)
            .unwrap()
            .detach()
    });

    deterministic.run_until_parked();
    workspace_a.update(cx_a, |workspace, cx| {
        assert!(workspace.is_being_followed(client_b.peer_id().unwrap()));
        assert_eq!(
            client_b.peer_id(),
            workspace.leader_for_pane(workspace.active_pane())
        );
        let item = workspace.active_pane().read(cx).active_item().unwrap();
        assert_eq!(item.tab_description(0, cx).unwrap(), Cow::Borrowed("x.rs"));
    });

    // b moves to y.rs in b's project, a is still following but can't yet see
    workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id_b, "y.rs"), None, true, cx)
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

    deterministic.run_until_parked();
    assert_eq!(visible_push_notifications(cx_a).len(), 1);
    cx_a.update(|cx| {
        workspace::join_remote_project(
            project_b_id,
            client_b.user_id().unwrap(),
            client_a.app_state.clone(),
            cx,
        )
    })
    .await
    .unwrap();

    deterministic.run_until_parked();

    assert_eq!(visible_push_notifications(cx_a).len(), 0);
    let workspace_a_project_b = cx_a
        .windows()
        .iter()
        .max_by_key(|window| window.id())
        .unwrap()
        .downcast::<Workspace>()
        .unwrap()
        .root(cx_a);

    workspace_a_project_b.update(cx_a, |workspace, cx| {
        assert_eq!(workspace.project().read(cx).remote_id(), Some(project_b_id));
        assert!(workspace.is_being_followed(client_b.peer_id().unwrap()));
        assert_eq!(
            client_b.peer_id(),
            workspace.leader_for_pane(workspace.active_pane())
        );
        let item = workspace.active_item(cx).unwrap();
        assert_eq!(item.tab_description(0, cx).unwrap(), Cow::Borrowed("y.rs"));
    });
}

fn visible_push_notifications(
    cx: &mut TestAppContext,
) -> Vec<gpui::ViewHandle<ProjectSharedNotification>> {
    let mut ret = Vec::new();
    for window in cx.windows() {
        window.read_with(cx, |window| {
            if let Some(handle) = window
                .root_view()
                .clone()
                .downcast::<ProjectSharedNotification>()
            {
                ret.push(handle)
            }
        });
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

fn pane_summaries(workspace: &ViewHandle<Workspace>, cx: &mut TestAppContext) -> Vec<PaneSummary> {
    workspace.read_with(cx, |workspace, cx| {
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
                                item.tab_description(0, cx)
                                    .map_or(String::new(), |s| s.to_string()),
                            )
                        })
                        .collect(),
                }
            })
            .collect()
    })
}
