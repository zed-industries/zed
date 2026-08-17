use async_recursion::async_recursion;

#[async_recursion]
async fn count_down(foo: Option<&str>) -> i32 {
    let _ = foo;
    0
}