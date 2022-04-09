use crate::{AppState, Request, RequestExt};
use std::sync::Arc;
use tide::http::mime;

pub fn add_routes(app: &mut tide::Server<Arc<AppState>>) {
    app.at("/careers").get(get_careers);
}

async fn get_careers(mut request: Request) -> tide::Result {
    let data = request.layout_data().await?;
    Ok(tide::Response::builder(200)
        .body(request.state().render_template("careers.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}
