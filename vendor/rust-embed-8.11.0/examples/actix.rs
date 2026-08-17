use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => HttpResponse::Ok()
      .content_type(from_path(path).first_or_octet_stream().as_ref())
      .body(content.data.into_owned()),
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
  handle_embedded_file("index.html")
}

#[actix_web::get("/dist/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
  handle_embedded_file(path.as_str())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(index).service(dist)).bind("127.0.0.1:8000")?.run().await
}
