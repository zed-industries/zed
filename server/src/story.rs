use crate::{AppState, Request, RequestExt};
use std::sync::Arc;
use tide::http::mime;

pub fn add_routes(story: &mut tide::Server<Arc<AppState>>) {
    story.at("/story").get(get_story);
}

async fn get_story(mut request: Request) -> tide::Result {
    let data = request.layout_data().await?;
    Ok(tide::Response::builder(200)
        .body(request.state().render_template("story.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}
