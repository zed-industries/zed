use async_std::task;

#[async_attributes::main]
async fn main() {
    task::spawn(async {
        println!("Hello, world!");
    })
    .await;
}
