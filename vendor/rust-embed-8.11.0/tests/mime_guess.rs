use rust_embed::{Embed, EmbeddedFile};

#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn html_mime_is_correct() {
  let html_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");
  assert_eq!(html_file.metadata.mimetype(), "text/html");
}

#[test]
fn css_mime_is_correct() {
  let css_file: EmbeddedFile = Asset::get("main.css").expect("main.css exists");
  assert_eq!(css_file.metadata.mimetype(), "text/css");
}

#[test]
fn js_mime_is_correct() {
  let js_file: EmbeddedFile = Asset::get("main.js").expect("main.js exists");
  assert_eq!(js_file.metadata.mimetype(), "text/javascript");
}

#[test]
fn jpg_mime_is_correct() {
  let jpg_file: EmbeddedFile = Asset::get("images/flower.jpg").expect("flower.jpg exists");
  assert_eq!(jpg_file.metadata.mimetype(), "image/jpeg");
}

#[test]
fn png_mime_is_correct() {
  let png_file: EmbeddedFile = Asset::get("images/llama.png").expect("llama.png exists");
  assert_eq!(png_file.metadata.mimetype(), "image/png");
}
