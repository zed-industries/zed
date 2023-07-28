use gpui::{executor::Deterministic, TestAppContext};
use std::sync::Arc;

use crate::db::Channel;

use super::TestServer;

#[gpui::test]
async fn test_basic_channels(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx, "user_a").await;
    let a_id = crate::db::UserId(client_a.user_id().unwrap() as i32);
    let db = server._test_db.db();

    let zed_id = db.create_root_channel("zed").await.unwrap();
    let crdb_id = db.create_channel("crdb", Some(zed_id)).await.unwrap();
    let livestreaming_id = db
        .create_channel("livestreaming", Some(zed_id))
        .await
        .unwrap();
    let replace_id = db.create_channel("replace", Some(zed_id)).await.unwrap();
    let rust_id = db.create_root_channel("rust").await.unwrap();
    let cargo_id = db.create_channel("cargo", Some(rust_id)).await.unwrap();

    db.add_channel_member(zed_id, a_id).await.unwrap();
    db.add_channel_member(rust_id, a_id).await.unwrap();

    let channels = db.get_channels(a_id).await.unwrap();
    assert_eq!(
        channels,
        vec![
            Channel {
                id: zed_id,
                name: "zed".to_string(),
                parent_id: None,
            },
            Channel {
                id: rust_id,
                name: "rust".to_string(),
                parent_id: None,
            },
            Channel {
                id: crdb_id,
                name: "crdb".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: livestreaming_id,
                name: "livestreaming".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: replace_id,
                name: "replace".to_string(),
                parent_id: Some(zed_id),
            },
            Channel {
                id: cargo_id,
                name: "cargo".to_string(),
                parent_id: Some(rust_id),
            }
        ]
    );
}

#[gpui::test]
async fn test_block_cycle_creation(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx, "user_a").await;
    let a_id = crate::db::UserId(client_a.user_id().unwrap() as i32);
    let db = server._test_db.db();

    let zed_id = db.create_root_channel("zed").await.unwrap();
    let first_id = db.create_channel("first", Some(zed_id)).await.unwrap();
    let second_id = db
        .create_channel("second_id", Some(first_id))
        .await
        .unwrap();
}

/*
Linear things:
- A way of expressing progress to the team
- A way for us to agree on a scope
- A way to figure out what we're supposed to be doing

*/
