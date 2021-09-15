use crate::{AppState, Request, RequestExt};
use std::sync::Arc;
use tide::http::mime;

pub fn add_routes(releases: &mut tide::Server<Arc<AppState>>) {
    releases.at("/releases").get(get_releases);
}

async fn get_releases(mut request: Request) -> tide::Result {
    let data = request.layout_data().await?;
    Ok(tide::Response::builder(200)
        .body(request.state().render_template("releases.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}
