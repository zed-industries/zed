use crate::{AppState, Request};
use anyhow::anyhow;
use rust_embed::RustEmbed;
use std::sync::Arc;
use tide::{http::mime, Server};

#[derive(RustEmbed)]
#[folder = "static"]
struct Static;

pub fn add_routes(app: &mut Server<Arc<AppState>>) {
    app.at("/static/*path").get(get_static_asset);
}

async fn get_static_asset(request: Request) -> tide::Result {
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
        .body(content.as_ref())
        .build())
}
