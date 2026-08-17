use async_recursion::async_recursion;

#[async_recursion]
pub async fn generic_parameter<S: Marker + Send>(mut x: S) -> u64 {
    if x.descend() {
        generic_parameter(x).await
    } else {
        0
    }
}