use crate::{auth::RequestExt as _, AppState, DbPool, LayoutData, Request, RequestExt as _};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow};
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
    app.at("/users").post(post_user);
    app.at("/users/:id").put(put_user);
    app.at("/users/:id/delete").post(delete_user);
    app.at("/signups/:id/delete").post(delete_signup);
}

#[derive(Serialize)]
struct AdminData {
    #[serde(flatten)]
    layout: Arc<LayoutData>,
    users: Vec<User>,
    signups: Vec<Signup>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub github_login: String,
    pub admin: bool,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Signup {
    pub id: i32,
    pub github_login: String,
    pub email_address: String,
    pub about: String,
}

async fn get_admin_page(mut request: Request) -> tide::Result {
    request.require_admin().await?;

    let data = AdminData {
        layout: request.layout_data().await?,
        users: sqlx::query_as("SELECT * FROM users ORDER BY github_login ASC")
            .fetch_all(request.db())
            .await?,
        signups: sqlx::query_as("SELECT * FROM signups ORDER BY id DESC")
            .fetch_all(request.db())
            .await?,
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
        create_user(request.db(), github_login, form.admin).await?;
    }

    Ok(tide::Redirect::new("/admin").into())
}

async fn put_user(mut request: Request) -> tide::Result {
    request.require_admin().await?;

    let user_id = request.param("id")?.parse::<i32>()?;

    #[derive(Deserialize)]
    struct Body {
        admin: bool,
    }

    let body: Body = request.body_json().await?;

    request
        .db()
        .execute(
            sqlx::query("UPDATE users SET admin = $1 WHERE id = $2;")
                .bind(body.admin)
                .bind(user_id),
        )
        .await?;

    Ok(tide::Response::builder(200).build())
}

async fn delete_user(request: Request) -> tide::Result {
    request.require_admin().await?;

    let user_id = request.param("id")?.parse::<i32>()?;
    request
        .db()
        .execute(sqlx::query("DELETE FROM users WHERE id = $1;").bind(user_id))
        .await?;

    Ok(tide::Redirect::new("/admin").into())
}

pub async fn create_user(db: &DbPool, github_login: &str, admin: bool) -> tide::Result<i32> {
    let id: i32 =
        sqlx::query_scalar("INSERT INTO users (github_login, admin) VALUES ($1, $2) RETURNING id;")
            .bind(github_login)
            .bind(admin)
            .fetch_one(db)
            .await?;
    Ok(id)
}

async fn delete_signup(request: Request) -> tide::Result {
    request.require_admin().await?;
    let signup_id = request.param("id")?.parse::<i32>()?;
    request
        .db()
        .execute(sqlx::query("DELETE FROM signups WHERE id = $1;").bind(signup_id))
        .await?;

    Ok(tide::Redirect::new("/admin").into())
}
