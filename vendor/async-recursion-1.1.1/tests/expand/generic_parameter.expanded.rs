use async_recursion::async_recursion;
#[must_use]
pub fn generic_parameter<'async_recursion, S: Marker + Send>(
    mut x: S,
) -> ::core::pin::Pin<
    Box<
        dyn ::core::future::Future<
            Output = u64,
        > + 'async_recursion + ::core::marker::Send,
    >,
>
where
    S: 'async_recursion,
{
    Box::pin(async move { if x.descend() { generic_parameter(x).await } else { 0 } })
}
