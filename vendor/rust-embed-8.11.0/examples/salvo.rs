use salvo::http::{header, StatusCode};
use salvo::prelude::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let router = Router::new()
    .push(Router::with_path("dist/<**>").get(static_embed))
    .push(Router::with_path("/<**>").get(static_embed));

  let listener = TcpListener::bind("127.0.0.1:3000");
  Server::new(listener).serve(router).await;
  Ok(())
}

#[derive(rust_embed::Embed)]
#[folder = "examples/public/"]
struct Asset;

#[fn_handler]
async fn static_embed(req: &mut Request, res: &mut Response) {
  let mut path: String = req.get_param("**").unwrap_or_default();
  if path.is_empty() {
    path = "index.html".into();
  }

  match Asset::get(&path) {
    Some(content) => {
      let hash = hex::encode(content.metadata.sha256_hash());
      // if etag is matched, return 304
      if req
        .headers()
        .get(header::IF_NONE_MATCH)
        .map(|etag| etag.to_str().unwrap_or("000000").eq(&hash))
        .unwrap_or(false)
      {
        res.set_status_code(StatusCode::NOT_MODIFIED);
        return;
      }

      // otherwise, return 200 with etag hash
      let body: Vec<u8> = content.data.into();
      let mime = mime_guess::from_path(path).first_or_octet_stream();
      res.headers_mut().insert(header::ETAG, hash.parse().unwrap());
      res.render_binary(mime.as_ref().parse().unwrap(), &body);
    }
    None => res.set_status_code(StatusCode::NOT_FOUND),
  }
}
