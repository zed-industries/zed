use async_recursion::async_recursion;

#[async_recursion(?Send)]
async fn contains_value_2<'a, 'b, T: PartialEq>(value: &'b T, node: &'b Node<'a, T>) -> bool {
    contains_value(value, node).await
}