use async_recursion::async_recursion;
#[must_use]
pub fn n(
    x: i32,
) -> ::core::pin::Pin<
    Box<dyn ::core::future::Future<Output = i32> + ::core::marker::Send>,
> {
    Box::pin(async move { x })
}
