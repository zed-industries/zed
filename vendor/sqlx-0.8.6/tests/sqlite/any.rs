use sqlx::{Any, Sqlite};
use sqlx_test::new;

#[sqlx_macros::test]
async fn it_encodes_bool_with_any() -> anyhow::Result<()> {
    sqlx::any::install_default_drivers();
    let mut conn = new::<Any>().await?;

    let res = sqlx::query("INSERT INTO accounts (name, is_active) VALUES (?, ?)")
        .bind("Harrison Ford")
        .bind(true)
        .execute(&mut conn)
        .await
        .expect("failed to encode bool");
    assert_eq!(res.rows_affected(), 1);

    Ok(())
}

#[sqlx_macros::test]
async fn issue_3179() -> anyhow::Result<()> {
    sqlx::any::install_default_drivers();

    let mut conn = new::<Any>().await?;

    // 4294967297 = 2^32
    let number: i64 = sqlx::query_scalar("SELECT 4294967296")
        .fetch_one(&mut conn)
        .await?;

    // Previously, the decoding would use `i32` as an intermediate which would overflow to 0.
    assert_eq!(number, 4294967296);

    Ok(())
}
