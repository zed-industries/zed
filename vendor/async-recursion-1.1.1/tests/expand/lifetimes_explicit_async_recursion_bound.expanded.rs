use async_recursion::async_recursion;
#[must_use]
fn explicit_async_recursion_bound<'life0, 'life1, 'async_recursion>(
    t: &'life0 T,
    p: &'life1 [String],
    prefix: Option<&'async_recursion [u8]>,
    layer: Option<&'async_recursion [u8]>,
) -> ::core::pin::Pin<
    Box<
        dyn ::core::future::Future<Output = ()> + 'async_recursion + ::core::marker::Send,
    >,
>
where
    'life0: 'async_recursion,
    'life1: 'async_recursion,
    'async_recursion: 'async_recursion,
{
    Box::pin(async move {})
}
