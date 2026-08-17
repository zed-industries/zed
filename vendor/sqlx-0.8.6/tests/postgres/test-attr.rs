// The no-arg variant is covered by other tests already.

use sqlx::PgPool;

const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("tests/postgres/migrations");

#[sqlx::test]
async fn it_gets_a_pool(pool: PgPool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    let db_name: String = sqlx::query_scalar("SELECT current_database()")
        .fetch_one(&mut *conn)
        .await?;

    assert!(db_name.starts_with("_sqlx_test"), "dbname: {db_name:?}");

    Ok(())
}

// This should apply migrations and then `fixtures/users.sql`
#[sqlx::test(migrations = "tests/postgres/migrations", fixtures("users"))]
async fn it_gets_users(pool: PgPool) -> sqlx::Result<()> {
    let usernames: Vec<String> =
        sqlx::query_scalar(r#"SELECT username FROM "user" ORDER BY username"#)
            .fetch_all(&pool)
            .await?;

    assert_eq!(usernames, ["alice", "bob"]);

    let post_exists: bool = sqlx::query_scalar("SELECT exists(SELECT 1 FROM post)")
        .fetch_one(&pool)
        .await?;

    assert!(!post_exists);

    let comment_exists: bool = sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// This should apply migrations and then fixtures `fixtures/users.sql` and `fixtures/posts.sql`
#[sqlx::test(migrations = "tests/postgres/migrations", fixtures("users", "posts"))]
async fn it_gets_posts(pool: PgPool) -> sqlx::Result<()> {
    let post_contents: Vec<String> =
        sqlx::query_scalar("SELECT content FROM post ORDER BY created_at")
            .fetch_all(&pool)
            .await?;

    assert_eq!(
        post_contents,
        [
            "This new computer is lightning-fast!",
            "@alice is a haxxor :("
        ]
    );

    let comment_exists: bool = sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// This should apply migrations and then `../fixtures/postgres/users.sql` and `../fixtures/postgres/posts.sql`
#[sqlx::test(
    migrations = "tests/postgres/migrations",
    fixtures("../fixtures/postgres/users.sql", "../fixtures/postgres/posts.sql")
)]
async fn it_gets_posts_explicit_fixtures_path(pool: PgPool) -> sqlx::Result<()> {
    let post_contents: Vec<String> =
        sqlx::query_scalar("SELECT content FROM post ORDER BY created_at")
            .fetch_all(&pool)
            .await?;

    assert_eq!(
        post_contents,
        [
            "This new computer is lightning-fast!",
            "@alice is a haxxor :("
        ]
    );

    let comment_exists: bool = sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// This should apply migrations and then `../fixtures/postgres/users.sql` and `fixtures/posts.sql`
#[sqlx::test(
    migrations = "tests/postgres/migrations",
    fixtures("../fixtures/postgres/users.sql"),
    fixtures("posts")
)]
async fn it_gets_posts_mixed_fixtures_path(pool: PgPool) -> sqlx::Result<()> {
    let post_contents: Vec<String> =
        sqlx::query_scalar("SELECT content FROM post ORDER BY created_at")
            .fetch_all(&pool)
            .await?;

    assert_eq!(
        post_contents,
        [
            "This new computer is lightning-fast!",
            "@alice is a haxxor :("
        ]
    );

    let comment_exists: bool = sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// This should apply migrations and then `../fixtures/postgres/users.sql` and `../fixtures/postgres/posts.sql`
#[sqlx::test(
    migrations = "tests/postgres/migrations",
    fixtures("../fixtures/postgres/users.sql", "../fixtures/postgres/posts.sql")
)]
async fn it_gets_posts_custom_relative_fixtures_path(pool: PgPool) -> sqlx::Result<()> {
    let post_contents: Vec<String> =
        sqlx::query_scalar("SELECT content FROM post ORDER BY created_at")
            .fetch_all(&pool)
            .await?;

    assert_eq!(
        post_contents,
        [
            "This new computer is lightning-fast!",
            "@alice is a haxxor :("
        ]
    );

    let comment_exists: bool = sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// Try `migrator`
#[sqlx::test(migrator = "MIGRATOR", fixtures("users", "posts", "comments"))]
async fn it_gets_comments(pool: PgPool) -> sqlx::Result<()> {
    let post_1_comments: Vec<String> = sqlx::query_scalar(
        "SELECT content FROM comment WHERE post_id = $1::uuid ORDER BY created_at",
    )
    .bind(&"252c1d98-a9b0-4f18-8298-e59058bdfe16")
    .fetch_all(&pool)
    .await?;

    assert_eq!(
        post_1_comments,
        ["lol bet ur still bad, 1v1 me", "you're on!"]
    );

    let post_2_comments: Vec<String> = sqlx::query_scalar(
        "SELECT content FROM comment WHERE post_id = $1::uuid ORDER BY created_at",
    )
    .bind(&"844265f7-2472-4689-9a2e-b21f40dbf401")
    .fetch_all(&pool)
    .await?;

    assert_eq!(post_2_comments, ["lol you're just mad you lost :P"]);

    Ok(())
}

#[sqlx::test(
    migrations = "tests/postgres/migrations",
    fixtures(path = "../fixtures/postgres", scripts("users", "posts"))
)]
async fn this_should_compile(_pool: PgPool) -> sqlx::Result<()> {
    Ok(())
}

macro_rules! macro_using_test {
    ($migrations: literal) => {
        #[sqlx::test(
                            migrations = $migrations,
                            fixtures(path = "../fixtures/postgres", scripts("users", "posts"))
                        )]
        async fn macro_using_macro(_pool: PgPool) -> sqlx::Result<()> {
            Ok(())
        }
    };
}
macro_using_test!("tests/postgres/migrations");
