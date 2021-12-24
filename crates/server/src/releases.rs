use crate::{
    auth::RequestExt as _, github::Release, AppState, LayoutData, Request, RequestExt as _,
};
use comrak::ComrakOptions;
use serde::Serialize;
use std::sync::Arc;
use tide::http::mime;

pub fn add_routes(releases: &mut tide::Server<Arc<AppState>>) {
    releases.at("/releases").get(get_releases);
}

async fn get_releases(mut request: Request) -> tide::Result {
    #[derive(Serialize)]
    struct ReleasesData {
        #[serde(flatten)]
        layout: Arc<LayoutData>,
        releases: Option<Vec<Release>>,
    }

    let mut data = ReleasesData {
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
        .body(request.state().render_template("releases.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}
