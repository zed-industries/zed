use crate::{
    auth::RequestExt as _, github::Release, AppState, LayoutData, Request, RequestExt as _,
};
use comrak::ComrakOptions;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tide::{http::mime, log, Server};

pub fn add_routes(app: &mut Server<Arc<AppState>>) {
    app.at("/").get(get_home);
    app.at("/signups").post(post_signup);
    app.at("/releases/:tag_name/:name").get(get_release_asset);
}

async fn get_home(mut request: Request) -> tide::Result {
    #[derive(Serialize)]
    struct HomeData {
        #[serde(flatten)]
        layout: Arc<LayoutData>,
        releases: Option<Vec<Release>>,
    }

    let mut data = HomeData {
        layout: request.layout_data().await?,
        releases: None,
    };

    if let Some(user) = request.current_user().await? {
        if user.is_insider {
            data.releases = Some(
                request
                    .state()
                    .repo_client
                    .releases()
                    .await?
                    .into_iter()
                    .filter_map(|mut release| {
                        if release.draft {
                            None
                        } else {
                            let mut options = ComrakOptions::default();
                            options.render.unsafe_ = true; // Allow raw HTML in the markup. We control these release notes anyway.
                            release.body = comrak::markdown_to_html(&release.body, &options);
                            Some(release)
                        }
                    })
                    .collect(),
            );
        }
    }

    Ok(tide::Response::builder(200)
        .body(request.state().render_template("home.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}

async fn post_signup(mut request: Request) -> tide::Result {
    #[derive(Debug, Deserialize)]
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

    log::info!("Signup submitted: {:?}", form);

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