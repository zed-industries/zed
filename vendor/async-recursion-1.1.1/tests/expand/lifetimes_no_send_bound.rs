use async_recursion::async_recursion;

struct Node<'a, T> {
    ptr: &'a T,
}

#[async_recursion(?Send)]
async fn contains_value<'a, T: PartialEq>(value: &T, node: &Node<'a, T>) -> bool {
    if &node.ptr == value {
        true
    } else {
        false
    }
}