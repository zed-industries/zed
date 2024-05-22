use std::{path::Path, sync::Arc};

use call::ActiveCall;
use editor::Editor;
use fs::Fs;
use gpui::{TestAppContext, VisualTestContext, WindowHandle};
use rpc::{proto::DevServerStatus, ErrorCode, ErrorExt};
use serde_json::json;
use workspace::{AppState, Workspace};

use crate::tests::{following_tests::join_channel, TestServer};

use super::TestClient;

#[gpui::test]
async fn test_dev_server(cx: &mut gpui::TestAppContext, cx2: &mut gpui::TestAppContext) {
    let (server, client) = TestServer::start1(cx).await;

    let store = cx.update(|cx| dev_server_projects::Store::global(cx).clone());

    let resp = store
        .update(cx, |store, cx| {
            store.create_dev_server("server-1".to_string(), None, cx)
        })
        .await
        .unwrap();

    store.update(cx, |store, _| {
        assert_eq!(store.dev_servers().len(), 1);
        assert_eq!(store.dev_servers()[0].name, "server-1");
        assert_eq!(store.dev_servers()[0].status, DevServerStatus::Offline);
    });

    let dev_server = server.create_dev_server(resp.access_token, cx2).await;
    cx.executor().run_until_parked();
    store.update(cx, |store, _| {
        assert_eq!(store.dev_servers()[0].status, DevServerStatus::Online);
    });

    dev_server
        .fs()
        .insert_tree(
            "/remote",
            json!({
                "1.txt": "remote\nremote\nremote",
                "2.js": "function two() { return 2; }",
                "3.rs": "mod test",
            }),
        )
        .await;

    store
        .update(cx, |store, cx| {
            store.create_dev_server_project(
                client::DevServerId(resp.dev_server_id),
                "/remote".to_string(),
                cx,
            )
        })
        .await
        .unwrap();

    cx.executor().run_until_parked();

    let remote_workspace = store
        .update(cx, |store, cx| {
            let projects = store.dev_server_projects();
            assert_eq!(projects.len(), 1);
            assert_eq!(projects[0].path, "/remote");
            workspace::join_dev_server_project(
                projects[0].project_id.unwrap(),
                client.app_state.clone(),
                None,
                cx,
            )
        })
        .await
        .unwrap();

    cx.executor().run_until_parked();

    let cx = VisualTestContext::from_window(remote_workspace.into(), cx).as_mut();
    cx.simulate_keystrokes("cmd-p 1 enter");

    let editor = remote_workspace
        .update(cx, |ws, cx| {
            ws.active_item_as::<Editor>(cx).unwrap().clone()
        })
        .unwrap();
    editor.update(cx, |ed, cx| {
        assert_eq!(ed.text(cx).to_string(), "remote\nremote\nremote");
    });
    cx.simulate_input("wow!");
    cx.simulate_keystrokes("cmd-s");

    let content = dev_server
        .fs()
        .load(&Path::new("/remote/1.txt"))
        .await
        .unwrap();
    assert_eq!(content, "wow!remote\nremote\nremote\n");
}

#[gpui::test]
async fn test_dev_server_env_files(
    cx1: &mut gpui::TestAppContext,
    cx2: &mut gpui::TestAppContext,
    cx3: &mut gpui::TestAppContext,
) {
    let (server, client1, client2, channel_id) = TestServer::start2(cx1, cx2).await;

    let (_dev_server, remote_workspace) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx3).await;

    cx1.executor().run_until_parked();

    let cx1 = VisualTestContext::from_window(remote_workspace.into(), cx1).as_mut();
    cx1.simulate_keystrokes("cmd-p . e enter");

    let editor = remote_workspace
        .update(cx1, |ws, cx| {
            ws.active_item_as::<Editor>(cx).unwrap().clone()
        })
        .unwrap();
    editor.update(cx1, |ed, cx| {
        assert_eq!(ed.text(cx).to_string(), "SECRET");
    });

    cx1.update(|cx| {
        workspace::join_channel(
            channel_id,
            client1.app_state.clone(),
            Some(remote_workspace),
            cx,
        )
    })
    .await
    .unwrap();
    cx1.executor().run_until_parked();

    remote_workspace
        .update(cx1, |ws, cx| {
            assert!(ws.project().read(cx).is_shared());
        })
        .unwrap();

    join_channel(channel_id, &client2, cx2).await.unwrap();
    cx2.executor().run_until_parked();

    let (workspace2, cx2) = client2.active_workspace(cx2);
    let editor = workspace2.update(cx2, |ws, cx| {
        ws.active_item_as::<Editor>(cx).unwrap().clone()
    });
    // TODO: it'd be nice to hide .env files from other people
    editor.update(cx2, |ed, cx| {
        assert_eq!(ed.text(cx).to_string(), "SECRET");
    });
}

async fn create_dev_server_project(
    server: &TestServer,
    client_app_state: Arc<AppState>,
    cx: &mut TestAppContext,
    cx_devserver: &mut TestAppContext,
) -> (TestClient, WindowHandle<Workspace>) {
    let store = cx.update(|cx| dev_server_projects::Store::global(cx).clone());

    let resp = store
        .update(cx, |store, cx| {
            store.create_dev_server("server-1".to_string(), None, cx)
        })
        .await
        .unwrap();
    let dev_server = server
        .create_dev_server(resp.access_token, cx_devserver)
        .await;

    cx.executor().run_until_parked();

    dev_server
        .fs()
        .insert_tree(
            "/remote",
            json!({
                "1.txt": "remote\nremote\nremote",
                ".env": "SECRET",
            }),
        )
        .await;

    store
        .update(cx, |store, cx| {
            store.create_dev_server_project(
                client::DevServerId(resp.dev_server_id),
                "/remote".to_string(),
                cx,
            )
        })
        .await
        .unwrap();

    cx.executor().run_until_parked();

    let workspace = store
        .update(cx, |store, cx| {
            let projects = store.dev_server_projects();
            assert_eq!(projects.len(), 1);
            assert_eq!(projects[0].path, "/remote");
            workspace::join_dev_server_project(
                projects[0].project_id.unwrap(),
                client_app_state,
                None,
                cx,
            )
        })
        .await
        .unwrap();

    cx.executor().run_until_parked();

    (dev_server, workspace)
}

#[gpui::test]
async fn test_dev_server_leave_room(
    cx1: &mut gpui::TestAppContext,
    cx2: &mut gpui::TestAppContext,
    cx3: &mut gpui::TestAppContext,
) {
    let (server, client1, client2, channel_id) = TestServer::start2(cx1, cx2).await;

    let (_dev_server, remote_workspace) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx3).await;

    cx1.update(|cx| {
        workspace::join_channel(
            channel_id,
            client1.app_state.clone(),
            Some(remote_workspace),
            cx,
        )
    })
    .await
    .unwrap();
    cx1.executor().run_until_parked();

    remote_workspace
        .update(cx1, |ws, cx| {
            assert!(ws.project().read(cx).is_shared());
        })
        .unwrap();

    join_channel(channel_id, &client2, cx2).await.unwrap();
    cx2.executor().run_until_parked();

    cx1.update(|cx| ActiveCall::global(cx).update(cx, |active_call, cx| active_call.hang_up(cx)))
        .await
        .unwrap();

    cx1.executor().run_until_parked();

    let (workspace, cx2) = client2.active_workspace(cx2);
    cx2.update(|cx| assert!(workspace.read(cx).project().read(cx).is_disconnected()));
}

#[gpui::test]
async fn test_dev_server_delete(
    cx1: &mut gpui::TestAppContext,
    cx2: &mut gpui::TestAppContext,
    cx3: &mut gpui::TestAppContext,
) {
    let (server, client1, client2, channel_id) = TestServer::start2(cx1, cx2).await;

    let (_dev_server, remote_workspace) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx3).await;

    cx1.update(|cx| {
        workspace::join_channel(
            channel_id,
            client1.app_state.clone(),
            Some(remote_workspace),
            cx,
        )
    })
    .await
    .unwrap();
    cx1.executor().run_until_parked();

    remote_workspace
        .update(cx1, |ws, cx| {
            assert!(ws.project().read(cx).is_shared());
        })
        .unwrap();

    join_channel(channel_id, &client2, cx2).await.unwrap();
    cx2.executor().run_until_parked();

    cx1.update(|cx| {
        dev_server_projects::Store::global(cx).update(cx, |store, cx| {
            store.delete_dev_server_project(store.dev_server_projects().first().unwrap().id, cx)
        })
    })
    .await
    .unwrap();

    cx1.executor().run_until_parked();

    let (workspace, cx2) = client2.active_workspace(cx2);
    cx2.update(|cx| assert!(workspace.read(cx).project().read(cx).is_disconnected()));

    cx1.update(|cx| {
        dev_server_projects::Store::global(cx).update(cx, |store, _| {
            assert_eq!(store.dev_server_projects().len(), 0);
        })
    })
}

#[gpui::test]
async fn test_dev_server_rename(
    cx1: &mut gpui::TestAppContext,
    cx2: &mut gpui::TestAppContext,
    cx3: &mut gpui::TestAppContext,
) {
    let (server, client1, client2, channel_id) = TestServer::start2(cx1, cx2).await;

    let (_dev_server, remote_workspace) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx3).await;

    cx1.update(|cx| {
        workspace::join_channel(
            channel_id,
            client1.app_state.clone(),
            Some(remote_workspace),
            cx,
        )
    })
    .await
    .unwrap();
    cx1.executor().run_until_parked();

    remote_workspace
        .update(cx1, |ws, cx| {
            assert!(ws.project().read(cx).is_shared());
        })
        .unwrap();

    join_channel(channel_id, &client2, cx2).await.unwrap();
    cx2.executor().run_until_parked();

    cx1.update(|cx| {
        dev_server_projects::Store::global(cx).update(cx, |store, cx| {
            store.rename_dev_server(
                store.dev_servers().first().unwrap().id,
                "name-edited".to_string(),
                None,
                cx,
            )
        })
    })
    .await
    .unwrap();

    cx1.executor().run_until_parked();

    cx1.update(|cx| {
        dev_server_projects::Store::global(cx).update(cx, |store, _| {
            assert_eq!(store.dev_servers().first().unwrap().name, "name-edited");
        })
    })
}

#[gpui::test]
async fn test_dev_server_refresh_access_token(
    cx1: &mut gpui::TestAppContext,
    cx2: &mut gpui::TestAppContext,
    cx3: &mut gpui::TestAppContext,
    cx4: &mut gpui::TestAppContext,
) {
    let (server, client1, client2, channel_id) = TestServer::start2(cx1, cx2).await;

    let (_dev_server, remote_workspace) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx3).await;

    cx1.update(|cx| {
        workspace::join_channel(
            channel_id,
            client1.app_state.clone(),
            Some(remote_workspace),
            cx,
        )
    })
    .await
    .unwrap();
    cx1.executor().run_until_parked();

    remote_workspace
        .update(cx1, |ws, cx| {
            assert!(ws.project().read(cx).is_shared());
        })
        .unwrap();

    join_channel(channel_id, &client2, cx2).await.unwrap();
    cx2.executor().run_until_parked();

    // Regenerate the access token
    let new_token_response = cx1
        .update(|cx| {
            dev_server_projects::Store::global(cx).update(cx, |store, cx| {
                store.regenerate_dev_server_token(store.dev_servers().first().unwrap().id, cx)
            })
        })
        .await
        .unwrap();

    cx1.executor().run_until_parked();

    // Assert that the other client was disconnected
    let (workspace, cx2) = client2.active_workspace(cx2);
    cx2.update(|cx| assert!(workspace.read(cx).project().read(cx).is_disconnected()));

    // Assert that the owner of the dev server does not see the dev server as online anymore
    let (workspace, cx1) = client1.active_workspace(cx1);
    cx1.update(|cx| {
        assert!(workspace.read(cx).project().read(cx).is_disconnected());
        dev_server_projects::Store::global(cx).update(cx, |store, _| {
            assert_eq!(
                store.dev_servers().first().unwrap().status,
                DevServerStatus::Offline
            );
        })
    });

    // Reconnect the dev server with the new token
    let _dev_server = server
        .create_dev_server(new_token_response.access_token, cx4)
        .await;

    cx1.executor().run_until_parked();

    // Assert that the dev server is online again
    cx1.update(|cx| {
        dev_server_projects::Store::global(cx).update(cx, |store, _| {
            assert_eq!(store.dev_servers().len(), 1);
            assert_eq!(
                store.dev_servers().first().unwrap().status,
                DevServerStatus::Online
            );
        })
    });
}

#[gpui::test]
async fn test_dev_server_reconnect(
    cx1: &mut gpui::TestAppContext,
    cx2: &mut gpui::TestAppContext,
    cx3: &mut gpui::TestAppContext,
) {
    let (mut server, client1) = TestServer::start1(cx1).await;
    let channel_id = server
        .make_channel("test", None, (&client1, cx1), &mut [])
        .await;

    let (_dev_server, remote_workspace) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx3).await;

    cx1.update(|cx| {
        workspace::join_channel(
            channel_id,
            client1.app_state.clone(),
            Some(remote_workspace),
            cx,
        )
    })
    .await
    .unwrap();
    cx1.executor().run_until_parked();

    remote_workspace
        .update(cx1, |ws, cx| {
            assert!(ws.project().read(cx).is_shared());
        })
        .unwrap();

    drop(client1);

    let client2 = server.create_client(cx2, "user_a").await;

    let store = cx2.update(|cx| dev_server_projects::Store::global(cx).clone());

    store
        .update(cx2, |store, cx| {
            let projects = store.dev_server_projects();
            workspace::join_dev_server_project(
                projects[0].project_id.unwrap(),
                client2.app_state.clone(),
                None,
                cx,
            )
        })
        .await
        .unwrap();
}

#[gpui::test]
async fn test_create_dev_server_project_path_validation(
    cx1: &mut gpui::TestAppContext,
    cx2: &mut gpui::TestAppContext,
    cx3: &mut gpui::TestAppContext,
) {
    let (server, client1) = TestServer::start1(cx1).await;
    let _channel_id = server
        .make_channel("test", None, (&client1, cx1), &mut [])
        .await;

    // Creating a project with a path that does exist should not fail
    let (_dev_server, _) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx2).await;

    cx1.executor().run_until_parked();

    let store = cx1.update(|cx| dev_server_projects::Store::global(cx).clone());

    let resp = store
        .update(cx1, |store, cx| {
            store.create_dev_server("server-2".to_string(), None, cx)
        })
        .await
        .unwrap();

    cx1.executor().run_until_parked();

    let _dev_server = server.create_dev_server(resp.access_token, cx3).await;

    cx1.executor().run_until_parked();

    // Creating a remote project with a path that does not exist should fail
    let result = store
        .update(cx1, |store, cx| {
            store.create_dev_server_project(
                client::DevServerId(resp.dev_server_id),
                "/notfound".to_string(),
                cx,
            )
        })
        .await;

    cx1.executor().run_until_parked();

    let error = result.unwrap_err();
    assert!(matches!(
        error.error_code(),
        ErrorCode::DevServerProjectPathDoesNotExist
    ));
}

#[gpui::test]
async fn test_save_as_remote(cx1: &mut gpui::TestAppContext, cx2: &mut gpui::TestAppContext) {
    let (server, client1) = TestServer::start1(cx1).await;

    // Creating a project with a path that does exist should not fail
    let (dev_server, remote_workspace) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx2).await;

    let mut cx = VisualTestContext::from_window(remote_workspace.into(), cx1);

    cx.simulate_keystrokes("cmd-p 1 enter");
    cx.simulate_keystrokes("cmd-shift-s");
    cx.simulate_input("2.txt");
    cx.simulate_keystrokes("enter");

    cx.executor().run_until_parked();

    let title = remote_workspace
        .update(&mut cx, |ws, cx| {
            ws.active_item(cx).unwrap().tab_description(0, &cx).unwrap()
        })
        .unwrap();

    assert_eq!(title, "2.txt");

    let path = Path::new("/remote/2.txt");
    assert_eq!(
        dev_server.fs().load(&path).await.unwrap(),
        "remote\nremote\nremote"
    );
}

#[gpui::test]
async fn test_new_file_remote(cx1: &mut gpui::TestAppContext, cx2: &mut gpui::TestAppContext) {
    let (server, client1) = TestServer::start1(cx1).await;

    // Creating a project with a path that does exist should not fail
    let (dev_server, remote_workspace) =
        create_dev_server_project(&server, client1.app_state.clone(), cx1, cx2).await;

    let mut cx = VisualTestContext::from_window(remote_workspace.into(), cx1);

    cx.simulate_keystrokes("cmd-n");
    cx.simulate_input("new!");
    cx.simulate_keystrokes("cmd-shift-s");
    cx.simulate_input("2.txt");
    cx.simulate_keystrokes("enter");

    cx.executor().run_until_parked();

    let title = remote_workspace
        .update(&mut cx, |ws, cx| {
            ws.active_item(cx).unwrap().tab_description(0, &cx).unwrap()
        })
        .unwrap();

    assert_eq!(title, "2.txt");

    let path = Path::new("/remote/2.txt");
    assert_eq!(dev_server.fs().load(&path).await.unwrap(), "new!");
}
