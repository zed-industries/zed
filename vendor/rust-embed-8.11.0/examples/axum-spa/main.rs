use axum::{
  http::{header, StatusCode, Uri},
  response::{Html, IntoResponse, Response},
  routing::Router,
};
use rust_embed::Embed;
use std::net::SocketAddr;

static INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "examples/axum-spa/assets/"]
struct Assets;

#[tokio::main]
async fn main() {
  let app = Router::new().fallback(static_handler);

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  println!("listening on {}", addr);
  axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
  let path = uri.path().trim_start_matches('/');

  if path.is_empty() || path == INDEX_HTML {
    return index_html().await;
  }

  match Assets::get(path) {
    Some(content) => {
      let mime = mime_guess::from_path(path).first_or_octet_stream();

      ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
    }
    None => {
      if path.contains('.') {
        return not_found().await;
      }

      index_html().await
    }
  }
}

async fn index_html() -> Response {
  match Assets::get(INDEX_HTML) {
    Some(content) => Html(content.data).into_response(),
    None => not_found().await,
  }
}

async fn not_found() -> Response {
  (StatusCode::NOT_FOUND, "404").into_response()
}
