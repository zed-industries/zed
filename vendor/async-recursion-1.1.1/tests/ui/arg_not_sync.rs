use async_recursion::async_recursion;

fn assert_is_sync(_: impl Sync) {}


#[async_recursion]
async fn send_not_sync() {}

#[async_recursion(?Send)]
async fn not_send_not_sync() {}

fn main() {
    assert_is_sync(send_not_sync());
    assert_is_sync(not_send_not_sync());
}