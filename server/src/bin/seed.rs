use sqlx::postgres::PgPoolOptions;
use tide::log;

#[path = "../env.rs"]
mod env;

#[async_std::main]
async fn main() {
    if let Err(error) = env::load_dotenv() {
        log::error!(
            "error loading .env.toml (this is expected in production): {}",
            error
        );
    }

    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env var");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to postgres database");

    let zed_users = ["nathansobo", "maxbrunsfeld", "as-cii", "iamnbutler"];
    let mut zed_user_ids = Vec::<i32>::new();
    for zed_user in zed_users {
        zed_user_ids.push(
            sqlx::query_scalar(
                r#"
                INSERT INTO users
                    (github_login, admin)
                VALUES
                    ($1, true)
                ON CONFLICT (github_login) DO UPDATE SET
                    github_login=EXCLUDED.github_login
                RETURNING id
                "#,
            )
            .bind(zed_user)
            .fetch_one(&db)
            .await
            .expect("failed to insert user"),
        )
    }

    let zed_org_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO orgs
            (name, slug)
        VALUES
            ('Zed', 'zed')
        ON CONFLICT (slug) DO UPDATE SET
            slug=EXCLUDED.slug
        RETURNING id
        "#,
    )
    .fetch_one(&db)
    .await
    .expect("failed to insert org");

    let general_channel_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO channels
            (owner_is_user, owner_id, name)
        VALUES
            (false, $1, 'General')
        ON CONFLICT (owner_is_user, owner_id, name) DO UPDATE SET
            name=EXCLUDED.name
        RETURNING id
        "#,
    )
    .bind(zed_org_id)
    .fetch_one(&db)
    .await
    .expect("failed to insert channel");

    for user_id in zed_user_ids {
        sqlx::query(
            r#"
            INSERT INTO org_memberships
                (org_id, user_id, admin)
            VALUES
                ($1, $2, true)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(zed_org_id)
        .bind(user_id)
        .execute(&db)
        .await
        .expect("failed to insert org membership");

        sqlx::query(
            r#"
            INSERT INTO channel_memberships
                (channel_id, user_id, admin)
            VALUES
                ($1, $2, true)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(general_channel_id)
        .bind(user_id)
        .execute(&db)
        .await
        .expect("failed to insert channel membership");
    }
}
