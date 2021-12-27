use crate::{auth::RequestExt as _, db, AppState, LayoutData, Request, RequestExt as _};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surf::http::mime;

#[async_trait]
pub trait RequestExt {
    async fn require_admin(&self) -> tide::Result<()>;
}

#[async_trait]
impl RequestExt for Request {
    async fn require_admin(&self) -> tide::Result<()> {
        let current_user = self
            .current_user()
            .await?
            .ok_or_else(|| tide::Error::from_str(401, "not logged in"))?;

        if current_user.is_admin {
            Ok(())
        } else {
            Err(tide::Error::from_str(
                403,
                "authenticated user is not an admin",
            ))
        }
    }
}

pub fn add_routes(app: &mut tide::Server<Arc<AppState>>) {
    app.at("/admin").get(get_admin_page);
    app.at("/admin/users").post(post_user);
    app.at("/admin/users/:id").put(put_user);
    app.at("/admin/users/:id/delete").post(delete_user);
    app.at("/admin/signups/:id/delete").post(delete_signup);
}

#[derive(Serialize)]
struct AdminData {
    #[serde(flatten)]
    layout: Arc<LayoutData>,
    users: Vec<db::User>,
    signups: Vec<db::Signup>,
}

async fn get_admin_page(mut request: Request) -> tide::Result {
    request.require_admin().await?;

    let data = AdminData {
        layout: request.layout_data().await?,
        users: request.db().get_all_users().await?,
        signups: request.db().get_all_signups().await?,
    };

    Ok(tide::Response::builder(200)
        .body(request.state().render_template("admin.hbs", &data)?)
        .content_type(mime::HTML)
        .build())
}

async fn post_user(mut request: Request) -> tide::Result {
    request.require_admin().await?;

    #[derive(Deserialize)]
    struct Form {
        github_login: String,
        #[serde(default)]
        admin: bool,
    }

    let form = request.body_form::<Form>().await?;
    let github_login = form
        .github_login
        .strip_prefix("@")
        .unwrap_or(&form.github_login);

    if !github_login.is_empty() {
        request.db().create_user(github_login, form.admin).await?;
    }

    Ok(tide::Redirect::new("/admin").into())
}

async fn put_user(mut request: Request) -> tide::Result {
    request.require_admin().await?;

    let user_id = request.param("id")?.parse()?;

    #[derive(Deserialize)]
    struct Body {
        admin: bool,
    }

    let body: Body = request.body_json().await?;

    request
        .db()
        .set_user_is_admin(db::UserId(user_id), body.admin)
        .await?;

    Ok(tide::Response::builder(200).build())
}

async fn delete_user(request: Request) -> tide::Result {
    request.require_admin().await?;
    let user_id = db::UserId(request.param("id")?.parse()?);
    request.db().destroy_user(user_id).await?;
    Ok(tide::Redirect::new("/admin").into())
}

async fn delete_signup(request: Request) -> tide::Result {
    request.require_admin().await?;
    let signup_id = db::SignupId(request.param("id")?.parse()?);
    request.db().destroy_signup(signup_id).await?;
    Ok(tide::Redirect::new("/admin").into())
}
