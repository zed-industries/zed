use bytes::Buf;
use futures_util::{Stream, StreamExt};
use warp::{reply::Response, Filter, Reply};

#[tokio::main]
async fn main() {
    // Running curl -T /path/to/a/file 'localhost:3030/' should echo back the content of the file,
    // or an HTTP 413 error if the configured size limit is exceeded.
    let route = warp::body::content_length_limit(65536)
        .and(warp::body::stream())
        .then(handler);
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handler(
    mut body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin + Send + Sync,
) -> Response {
    let mut collected: Vec<u8> = vec![];
    while let Some(buf) = body.next().await {
        let mut buf = buf.unwrap();
        while buf.remaining() > 0 {
            let chunk = buf.chunk();
            let chunk_len = chunk.len();
            collected.extend_from_slice(chunk);
            buf.advance(chunk_len);
        }
    }
    println!("Sending {} bytes", collected.len());
    collected.into_response()
}
