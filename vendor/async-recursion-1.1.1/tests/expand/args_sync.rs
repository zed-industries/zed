use async_recursion::async_recursion;

#[async_recursion(Sync)]
async fn sync() {}