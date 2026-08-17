use async_recursion::async_recursion;
#[must_use]
fn sync() -> ::core::pin::Pin<
    Box<
        dyn ::core::future::Future<
            Output = (),
        > + ::core::marker::Send + ::core::marker::Sync,
    >,
> {
    Box::pin(async move {})
}
