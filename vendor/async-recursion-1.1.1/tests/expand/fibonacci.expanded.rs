use async_recursion::async_recursion;
#[must_use]
fn fib(
    n: u32,
) -> ::core::pin::Pin<
    Box<dyn ::core::future::Future<Output = u64> + ::core::marker::Send>,
> {
    Box::pin(async move {
        match n {
            0 => {
                ::std::rt::begin_panic("zero is not a valid argument to fib()!");
            }
            1 | 2 => 1,
            3 => 2,
            _ => fib(n - 1).await + fib(n - 2).await,
        }
    })
}
