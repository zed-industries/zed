use async_recursion::async_recursion;

#[async_recursion(?Send, Sync)]
async fn not_send_sync_1() {}

#[async_recursion(?Send,Sync)]
async fn not_send_sync_2() {}

#[async_recursion(Sync, ?Send)]
async fn sync_not_send_1() {}

#[async_recursion(Sync,?Send)]
async fn sync_not_send_2() {}