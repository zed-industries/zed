use async_recursion::async_recursion;

#[async_recursion]
async fn fib(n: u32) -> u64 {
    match n {
        0 => panic!("zero is not a valid argument to fib()!"),
        1 | 2 => 1,
        3 => 2,
        _ => fib(n - 1).await + fib(n - 2).await,
    }
}