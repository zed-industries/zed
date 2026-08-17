use async_recursion::async_recursion;

#[async_recursion(Sync)]
async fn send_and_sync() {}

fn assert_is_send_and_sync(_: impl Send + Sync) {}

#[test]
fn test_sync_argument() {
    assert_is_send_and_sync(send_and_sync());
}
