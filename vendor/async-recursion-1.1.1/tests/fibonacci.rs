use async_recursion::async_recursion;
use futures_executor::block_on;

#[async_recursion]
async fn fib(n: u32) -> u64 {
    match n {
        0 => panic!("zero is not a valid argument to fib()!"),
        1 | 2 => 1,
        3 => 2,
        _ => fib(n - 1).await + fib(n - 2).await,
    }
}

#[test]
fn fibonacci_example_works() {
    block_on(async move {
        assert_eq!(fib(3).await, 2);
        assert_eq!(fib(4).await, 3);
        assert_eq!(fib(5).await, 5);
        assert_eq!(fib(6).await, 8);
    });
}

#[test]
fn fibonacci_example_is_send() {
    fn assert_is_send(_: impl Send) {}
    assert_is_send(fib(6));
}
