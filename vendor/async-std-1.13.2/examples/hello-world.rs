//! Spawns a task that says hello.

use async_std::task;

async fn say_hi() {
    println!("Hello, world!");
}

fn main() {
    task::block_on(say_hi())
}
