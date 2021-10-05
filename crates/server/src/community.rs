use crate::{AppState, Request, RequestExt};
use std::sync::Arc;
use tide::http::mime;

pub fn add_routes(community: &mut tide::Server<Arc<AppState>>) {
    community.at("/community").get(get_community);
}

async fn get_community(mut request: Request) -> tide::Result {
    let data = request.layout_data().await?;
    Ok(tide::Response::builder(200)
        .body(request.state().render_template("community.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}
