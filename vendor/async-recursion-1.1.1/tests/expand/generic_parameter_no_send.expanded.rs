use async_recursion::async_recursion;
#[must_use]
pub fn generic_parameter_no_send<'async_recursion, T>(
    x: T,
    y: u64,
) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = u64> + 'async_recursion>>
where
    T: 'async_recursion,
{
    Box::pin(async move {
        if y > 0 { generic_parameter_no_send(x, y - 1).await } else { 111 }
    })
}
