use async_recursion::async_recursion;
#[must_use]
fn count_down<'life0, 'async_recursion>(
    foo: Option<&'life0 str>,
) -> ::core::pin::Pin<
    Box<
        dyn ::core::future::Future<
            Output = i32,
        > + 'async_recursion + ::core::marker::Send,
    >,
>
where
    'life0: 'async_recursion,
{
    Box::pin(async move {
        let _ = foo;
        0
    })
}
