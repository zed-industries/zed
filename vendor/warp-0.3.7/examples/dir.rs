#![deny(warnings)]

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    warp::serve(warp::fs::dir("examples/dir"))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
