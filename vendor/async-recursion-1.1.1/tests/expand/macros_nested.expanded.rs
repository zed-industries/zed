#[must_use]
fn owned<'life0, 'async_recursion, F>(
    param: usize,
    f: &'life0 F,
) -> ::core::pin::Pin<
    Box<
        dyn ::core::future::Future<Output = ()> + 'async_recursion + ::core::marker::Send,
    >,
>
where
    F: Fn(usize) + Sync + Send,
    F: 'async_recursion,
    'life0: 'async_recursion,
{
    Box::pin(async move {
        f(param);
    })
}
#[must_use]
fn by_ref<'life0, 'life1, 'async_recursion, F>(
    param: &'life0 usize,
    f: &'life1 F,
) -> ::core::pin::Pin<
    Box<
        dyn ::core::future::Future<Output = ()> + 'async_recursion + ::core::marker::Send,
    >,
>
where
    F: Fn(&usize) + Sync + Send,
    F: 'async_recursion,
    'life0: 'async_recursion,
    'life1: 'async_recursion,
{
    Box::pin(async move {
        f(param);
    })
}
#[must_use]
fn by_ref_mut<'life0, 'life1, 'async_recursion, F>(
    param: &'life0 mut usize,
    f: &'life1 F,
) -> ::core::pin::Pin<
    Box<
        dyn ::core::future::Future<Output = ()> + 'async_recursion + ::core::marker::Send,
    >,
>
where
    F: Fn(&mut usize) + Sync + Send,
    F: 'async_recursion,
    'life0: 'async_recursion,
    'life1: 'async_recursion,
{
    Box::pin(async move {
        f(param);
    })
}
