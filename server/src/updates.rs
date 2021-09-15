use crate::{AppState, Request, RequestExt};
use std::sync::Arc;
use tide::http::mime;

pub fn add_routes(updates: &mut tide::Server<Arc<AppState>>) {
    updates.at("/updates").get(get_updates);
}

async fn get_updates(mut request: Request) -> tide::Result {
    let data = request.layout_data().await?;
    Ok(tide::Response::builder(200)
        .body(request.state().render_template("updates.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}
