#![deny(warnings)]
use warp::{http::StatusCode, Filter};

async fn dyn_reply(word: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if &word == "hello" {
        Ok(Box::new("world"))
    } else {
        Ok(Box::new(StatusCode::BAD_REQUEST))
    }
}

#[tokio::main]
async fn main() {
    let routes = warp::path::param().and_then(dyn_reply);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
