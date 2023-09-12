use crate::{rpc::RECONNECT_TIMEOUT, tests::TestServer};
use channel::ChannelChat;
use gpui::{executor::Deterministic, ModelHandle, TestAppContext};
use std::sync::Arc;

#[gpui::test]
async fn test_basic_channel_messages(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel("the-channel", (&client_a, cx_a), &mut [(&client_b, cx_b)])
        .await;

    let channel_chat_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();
    let channel_chat_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("one".into(), cx).unwrap())
        .await
        .unwrap();
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("two".into(), cx).unwrap())
        .await
        .unwrap();

    deterministic.run_until_parked();
    channel_chat_b
        .update(cx_b, |c, cx| c.send_message("three".into(), cx).unwrap())
        .await
        .unwrap();

    deterministic.run_until_parked();
    channel_chat_a.update(cx_a, |c, _| {
        assert_eq!(
            c.messages()
                .iter()
                .map(|m| m.body.as_str())
                .collect::<Vec<_>>(),
            vec!["one", "two", "three"]
        );
    })
}

#[gpui::test]
async fn test_rejoin_channel_chat(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel("the-channel", (&client_a, cx_a), &mut [(&client_b, cx_b)])
        .await;

    let channel_chat_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();
    let channel_chat_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("one".into(), cx).unwrap())
        .await
        .unwrap();
    channel_chat_b
        .update(cx_b, |c, cx| c.send_message("two".into(), cx).unwrap())
        .await
        .unwrap();

    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());

    // While client A is disconnected, clients A and B both send new messages.
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("three".into(), cx).unwrap())
        .await
        .unwrap_err();
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("four".into(), cx).unwrap())
        .await
        .unwrap_err();
    channel_chat_b
        .update(cx_b, |c, cx| c.send_message("five".into(), cx).unwrap())
        .await
        .unwrap();
    channel_chat_b
        .update(cx_b, |c, cx| c.send_message("six".into(), cx).unwrap())
        .await
        .unwrap();

    // Client A reconnects.
    server.allow_connections();
    deterministic.advance_clock(RECONNECT_TIMEOUT);

    // Client A fetches the messages that were sent while they were disconnected
    // and resends their own messages which failed to send.
    let expected_messages = &["one", "two", "five", "six", "three", "four"];
    assert_messages(&channel_chat_a, expected_messages, cx_a);
    assert_messages(&channel_chat_b, expected_messages, cx_b);
}

#[track_caller]
fn assert_messages(chat: &ModelHandle<ChannelChat>, messages: &[&str], cx: &mut TestAppContext) {
    chat.update(cx, |chat, _| {
        assert_eq!(
            chat.messages()
                .iter()
                .map(|m| m.body.as_str())
                .collect::<Vec<_>>(),
            messages
        );
    })
}
