use async_recursion::async_recursion;

#[async_recursion(?Send)]
pub async fn generic_parameter_no_send<T>(x: T, y: u64) -> u64 {
    if y > 0 {
        generic_parameter_no_send(x, y - 1).await
    } else {
        111
    }
}