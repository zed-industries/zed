use async_recursion::async_recursion;
use futures_executor::block_on;

struct Node<'a, T> {
    value: T,
    left: Option<&'a Node<'a, T>>,
    right: Option<&'a Node<'a, T>>,
}

impl<T> Node<'_, T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

// Note: Use the `?Send` notation here allows us not to require that our type parameter `T` is
// `T: PartialEq + Sync + Send`.
#[async_recursion(?Send)]
async fn contains_value<'a, T: PartialEq>(value: &T, node: &Node<'a, T>) -> bool {
    if &node.value == value {
        true
    } else {
        (node.left.is_some() && contains_value(value, node.left.unwrap()).await)
            || (node.right.is_some() && contains_value(value, node.right.unwrap()).await)
    }
}

#[async_recursion(?Send)]
async fn contains_value_2<'a, 'b, T: PartialEq>(value: &'b T, node: &'b Node<'a, T>) -> bool {
    contains_value(value, node).await
}

// The reference inside foo needs a `async_recursion bound
#[async_recursion]
async fn count_down(foo: Option<&str>) -> i32 {
    let _ = foo;
    0
}

#[async_recursion]
async fn explicit_async_recursion_bound(_: Option<&'async_recursion String>) {}

#[test]
fn lifetime_expansion_works() {
    block_on(async move {
        let mut node = Node::new(10);
        let mut left = Node::new(5);
        let left_left = Node::new(3);
        let left_right = Node::new(7);
        let mut right = Node::new(15);
        let right_left = Node::new(13);
        let right_right = Node::new(17);

        left.left = Some(&left_left);
        left.right = Some(&left_right);
        right.left = Some(&right_left);
        right.right = Some(&right_right);

        node.left = Some(&left);
        node.right = Some(&right);

        assert_eq!(contains_value(&3, &node).await, true);
        assert_eq!(contains_value(&4, &node).await, false);
        assert_eq!(contains_value(&17, &node).await, true);
        assert_eq!(contains_value(&13, &node).await, true);
        assert_eq!(contains_value(&12, &node).await, false);

        assert_eq!(contains_value_2(&3, &node).await, true);
        assert_eq!(contains_value_2(&4, &node).await, false);
        assert_eq!(contains_value_2(&17, &node).await, true);
        assert_eq!(contains_value_2(&13, &node).await, true);
        assert_eq!(contains_value_2(&12, &node).await, false);

        count_down(None).await;
        explicit_async_recursion_bound(None).await;
    });
}
