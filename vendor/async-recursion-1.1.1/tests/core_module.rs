use async_recursion::async_recursion;

mod core {
    // already defined core module shouldn't fail with message like 'could not find `pin` in `core`'
}

#[async_recursion]
pub async fn n(x: i32) -> i32 {
    x
}
