//! Spawns a named task that prints its name.

use async_std::task;

async fn print_name() {
    println!("My name is {:?}", task::current().name());
}

fn main() {
    task::block_on(async {
        task::Builder::new()
            .name("my-task".to_string())
            .spawn(print_name())
            .unwrap()
            .await;
    })
}
