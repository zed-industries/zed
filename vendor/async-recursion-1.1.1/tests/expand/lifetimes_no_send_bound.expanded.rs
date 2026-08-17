use async_recursion::async_recursion;
struct Node<'a, T> {
    ptr: &'a T,
}
#[must_use]
fn contains_value<'a, 'life0, 'life1, 'async_recursion, T: PartialEq>(
    value: &'life0 T,
    node: &'life1 Node<'a, T>,
) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = bool> + 'async_recursion>>
where
    T: 'async_recursion,
    'life0: 'async_recursion,
    'life1: 'async_recursion,
{
    Box::pin(async move { if &node.ptr == value { true } else { false } })
}
