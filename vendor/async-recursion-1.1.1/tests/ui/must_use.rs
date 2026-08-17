#![deny(unused_must_use)]

use async_recursion::async_recursion;

#[async_recursion]
async fn apples(_: u16) {}

fn main() {
    apples(3);
}