use anyhow::anyhow;
use rust_embed::RustEmbed;
use tide::{http::mime, Server};

#[derive(RustEmbed)]
#[folder = "static"]
struct Static;

pub fn add_routes(app: &mut Server<()>) {
    app.at("/*path").get(get_static_asset);
}

async fn get_static_asset(request: tide::Request<()>) -> tide::Result {
    let path = request.param("path").unwrap();
    let content = Static::get(path).ok_or_else(|| anyhow!("asset not found at {}", path))?;

    let content_type = if path.starts_with("svg") {
        mime::SVG
    } else if path.starts_with("styles") {
        mime::CSS
    } else {
        mime::BYTE_STREAM
    };

    Ok(tide::Response::builder(200)
        .content_type(content_type)
        .body(content.data.as_ref())
        .build())
}
