use axum::{
  extract::Path,
  http::{header, StatusCode},
  response::{Html, IntoResponse, Response},
  routing::{get, Router},
};
use rust_embed::Embed;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  // Define our app routes, including a fallback option for anything not matched.
  let app = Router::new()
    .route("/", get(index_handler))
    .route("/index.html", get(index_handler))
    .route("/dist/{*file}", get(static_handler))
    .fallback_service(get(not_found));

  // Start listening on the given address.
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("listening on {}", addr);
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app.into_make_service()).await.unwrap();
}

// We use static route matchers ("/" and "/index.html") to serve our home
// page.
async fn index_handler() -> impl IntoResponse {
  static_handler(Path("index.html".to_string())).await
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
  StaticFile(path)
}

// Finally, we use a fallback route for anything that didn't match.
async fn not_found() -> Html<&'static str> {
  Html("<h1>404</h1><p>Not Found</p>")
}

#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
  T: Into<String>,
{
  fn into_response(self) -> Response {
    let path = self.0.into();

    match Asset::get(path.as_str()) {
      Some(content) => {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
      }
      None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
  }
}
