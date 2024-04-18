use std::path::Path;

use editor::Editor;
use fs::Fs;
use gpui::VisualTestContext;
use rpc::proto::DevServerStatus;
use serde_json::json;

use crate::tests::{following_tests::join_channel, TestServer};

#[gpui::test]
async fn test_dev_server(cx: &mut gpui::TestAppContext, cx2: &mut gpui::TestAppContext) {
    let (server, client) = TestServer::start1(cx).await;

    let store = cx.update(|cx| remote_projects::Store::global(cx).clone());

    let resp = store
        .update(cx, |store, cx| {
            store.create_dev_server("server-1".to_string(), cx)
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
            store.create_remote_project(
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
            let projects = store.remote_projects();
            assert_eq!(projects.len(), 1);
            assert_eq!(projects[0].path, "/remote");
            workspace::join_remote_project(
                projects[0].project_id.unwrap(),
                client.app_state.clone(),
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

    let store = cx1.update(|cx| remote_projects::Store::global(cx).clone());

    let resp = store
        .update(cx1, |store, cx| {
            store.create_dev_server("server-1".to_string(), cx)
        })
        .await
        .unwrap();
    let dev_server = server.create_dev_server(resp.access_token, cx3).await;
    cx1.executor().run_until_parked();

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
        .update(cx1, |store, cx| {
            store.create_remote_project(
                client::DevServerId(resp.dev_server_id),
                "/remote".to_string(),
                cx,
            )
        })
        .await
        .unwrap();

    cx1.executor().run_until_parked();

    let remote_workspace = store
        .update(cx1, |store, cx| {
            let projects = store.remote_projects();
            assert_eq!(projects.len(), 1);
            assert_eq!(projects[0].path, "/remote");
            workspace::join_remote_project(
                projects[0].project_id.unwrap(),
                client1.app_state.clone(),
                cx,
            )
        })
        .await
        .unwrap();

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
