use gpui::{executor::Deterministic, TestAppContext};
use std::sync::Arc;

use super::TestServer;

#[gpui::test]
async fn test_basic_channels(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx, "user_a").await;
    let a_id = crate::db::UserId(client_a.user_id().unwrap() as i32);
    let db = server._test_db.db();

    let zed_id = db.create_channel("zed").await.unwrap();

    db.add_channel_member(zed_id, a_id).await.unwrap();

    let channels = db.get_channels(a_id).await;

    assert_eq!(channels, vec![zed_id]);
}

/*
Linear things:
- A way of expressing progress to the team
- A way for us to agree on a scope
- A way to figure out what we're supposed to be doing

*/
