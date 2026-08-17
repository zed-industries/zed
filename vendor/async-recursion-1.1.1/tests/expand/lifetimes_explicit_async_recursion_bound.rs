// Test that an explicit `async_recursion bound is left alone.
// This is a workaround many
use async_recursion::async_recursion;


#[async_recursion]
async fn explicit_async_recursion_bound(
    t: &T,
    p: &[String],
    prefix: Option<&'async_recursion [u8]>,
    layer: Option<&'async_recursion [u8]>,
) {}