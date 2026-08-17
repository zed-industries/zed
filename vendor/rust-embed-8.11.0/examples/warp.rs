use rust_embed::Embed;
use warp::{http::header::HeaderValue, path::Tail, reply::Response, Filter, Rejection, Reply};

#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

#[tokio::main]
async fn main() {
  let index_html = warp::path::end().and_then(serve_index);
  let dist = warp::path("dist").and(warp::path::tail()).and_then(serve);

  let routes = index_html.or(dist);
  warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn serve_index() -> Result<impl Reply, Rejection> {
  serve_impl("index.html")
}

async fn serve(path: Tail) -> Result<impl Reply, Rejection> {
  serve_impl(path.as_str())
}

fn serve_impl(path: &str) -> Result<impl Reply, Rejection> {
  let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;
  let mime = mime_guess::from_path(path).first_or_octet_stream();

  let mut res = Response::new(asset.data.into());
  res.headers_mut().insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
  Ok(res)
}
