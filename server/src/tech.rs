use crate::{AppState, Request, RequestExt};
use std::sync::Arc;
use tide::http::mime;

pub fn add_routes(tech: &mut tide::Server<Arc<AppState>>) {
    tech.at("/tech").get(get_tech);
}

async fn get_tech(mut request: Request) -> tide::Result {
    let data = request.layout_data().await?;
    Ok(tide::Response::builder(200)
        .body(request.state().render_template("tech.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}
