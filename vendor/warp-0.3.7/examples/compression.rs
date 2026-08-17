#![deny(warnings)]

use warp::Filter;

#[tokio::main]
async fn main() {
    let file = warp::path("todos").and(warp::fs::file("./examples/todos.rs"));
    // NOTE: You could double compress something by adding a compression
    // filter here, a la
    // ```
    // let file = warp::path("todos")
    //     .and(warp::fs::file("./examples/todos.rs"))
    //     .with(warp::compression::brotli());
    // ```
    // This would result in a browser error, or downloading a file whose contents
    // are compressed

    let dir = warp::path("ws_chat").and(warp::fs::file("./examples/websockets_chat.rs"));

    let file_and_dir = warp::get()
        .and(file.or(dir))
        .with(warp::compression::gzip());

    let examples = warp::path("ex")
        .and(warp::fs::dir("./examples/"))
        .with(warp::compression::deflate());

    // GET /todos => gzip -> toods.rs
    // GET /ws_chat => gzip -> ws_chat.rs
    // GET /ex/... => deflate -> ./examples/...
    let routes = file_and_dir.or(examples);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
