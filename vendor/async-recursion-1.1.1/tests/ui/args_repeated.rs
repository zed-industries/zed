use async_recursion::async_recursion;

#[async_recursion(?Send, ?Send)]
async fn repeated_args_1() {}

#[async_recursion(?Send, Sync, ?Send)]
async fn repeated_args_2() {}

#[async_recursion(Sync, ?Send, Sync, ?Send)]
async fn repeated_args_3() {}

fn main() {}