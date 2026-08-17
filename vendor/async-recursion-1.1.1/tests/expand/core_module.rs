use async_recursion::async_recursion;

#[async_recursion]
pub async fn n(x: i32) -> i32 {
    x
}
