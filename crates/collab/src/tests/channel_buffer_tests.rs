use crate::tests::TestServer;

use channel::channel_buffer::ChannelBuffer;
use gpui::{executor::Deterministic, ModelHandle, TestAppContext};
use std::{ops::Range, sync::Arc};

#[gpui::test]
async fn test_channel_buffers(
    deterministic: Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let zed_id = server
        .make_channel("zed", (&client_a, cx_a), &mut [(&client_b, cx_b)])
        .await;

    let channel_buffer_a = cx_a
        .update(|cx| ChannelBuffer::for_channel(zed_id, client_a.client().to_owned(), cx))
        .await
        .unwrap();

    let buffer_a = channel_buffer_a.read_with(cx_a, |buffer, _| buffer.buffer());

    edit_channel_buffer(&buffer_a, cx_a, [(0..0, "hello world")]);
    edit_channel_buffer(&buffer_a, cx_a, [(5..5, ", cruel")]);
    edit_channel_buffer(&buffer_a, cx_a, [(0..5, "goodbye")]);
    undo_channel_buffer(&buffer_a, cx_a);

    assert_eq!(channel_buffer_text(&buffer_a, cx_a), "hello, cruel world");

    let channel_buffer_b = cx_b
        .update(|cx| ChannelBuffer::for_channel(zed_id, client_b.client().to_owned(), cx))
        .await
        .unwrap();

    let buffer_b = channel_buffer_b.read_with(cx_b, |buffer, _| buffer.buffer());

    assert_eq!(channel_buffer_text(&buffer_b, cx_b), "hello, cruel world");

    edit_channel_buffer(&buffer_b, cx_b, [(7..12, "beautiful")]);

    deterministic.run_until_parked();

    assert_eq!(
        channel_buffer_text(&buffer_a, cx_a),
        "hello, beautiful world"
    );
    assert_eq!(
        channel_buffer_text(&buffer_b, cx_b),
        "hello, beautiful world"
    );
}

fn edit_channel_buffer<I>(
    channel_buffer: &ModelHandle<language::Buffer>,
    cx: &mut TestAppContext,
    edits: I,
) where
    I: IntoIterator<Item = (Range<usize>, &'static str)>,
{
    channel_buffer.update(cx, |buffer, cx| buffer.edit(edits, None, cx));
}

fn undo_channel_buffer(channel_buffer: &ModelHandle<language::Buffer>, cx: &mut TestAppContext) {
    channel_buffer.update(cx, |buffer, cx| buffer.undo(cx));
}

fn channel_buffer_text(
    channel_buffer: &ModelHandle<language::Buffer>,
    cx: &mut TestAppContext,
) -> String {
    channel_buffer.read_with(cx, |buffer, _| buffer.text())
}
