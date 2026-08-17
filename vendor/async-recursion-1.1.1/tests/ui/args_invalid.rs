use async_recursion::async_recursion;

#[async_recursion(?Sync)]
async fn not_sync() {}

#[async_recursion(Sync Sync)]
async fn not_punctuated() {}

#[async_recursion(Sync?Send)]
async fn what_even_is_this() {}



fn main() {}