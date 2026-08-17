use async_recursion::async_recursion;

#[async_recursion(?Send)]
async fn no_send_bound() {}