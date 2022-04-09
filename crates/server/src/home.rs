use crate::{AppState, Request, RequestExt as _};
use log::as_serde;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tide::{http::mime, Server};

pub fn add_routes(app: &mut Server<Arc<AppState>>) {
    app.at("/").get(get_home);
    app.at("/signups").post(post_signup);
    app.at("/releases/:tag_name/:name").get(get_release_asset);
}

async fn get_home(mut request: Request) -> tide::Result {
    let data = request.layout_data().await?;
    Ok(tide::Response::builder(200)
        .body(request.state().render_template("home.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}

async fn post_signup(mut request: Request) -> tide::Result {
    #[derive(Debug, Deserialize, Serialize)]
    struct Form {
        github_login: String,
        email_address: String,
        about: String,
        #[serde(default)]
        wants_releases: bool,
        #[serde(default)]
        wants_updates: bool,
        #[serde(default)]
        wants_community: bool,
    }

    let mut form: Form = request.body_form().await?;
    form.github_login = form
        .github_login
        .strip_prefix("@")
        .map(str::to_string)
        .unwrap_or(form.github_login);

    log::info!(form = as_serde!(form); "signup submitted");

    // Save signup in the database
    request
        .db()
        .create_signup(
            &form.github_login,
            &form.email_address,
            &form.about,
            form.wants_releases,
            form.wants_updates,
            form.wants_community,
        )
        .await?;

    let layout_data = request.layout_data().await?;
    Ok(tide::Response::builder(200)
        .body(
            request
                .state()
                .render_template("signup.hbs", &layout_data)?,
        )
        .content_type(mime::HTML)
        .build())
}

async fn get_release_asset(request: Request) -> tide::Result {
    let body = request
        .state()
        .repo_client
        .release_asset(request.param("tag_name")?, request.param("name")?)
        .await?;

    Ok(tide::Response::builder(200)
        .header("Cache-Control", "no-transform")
        .content_type(mime::BYTE_STREAM)
        .body(body)
        .build())
}
