use sqlx::Mssql;
use sqlx_test::new;

#[sqlx_macros::test]
async fn test_query_simple() -> anyhow::Result<()> {
    let mut conn = new::<Mssql>().await?;

    let account =
        sqlx::query!("select * from (select (1) as id, 'Herp Derpinson' as name, cast(null as char) as email, CAST(1 as bit) as deleted) accounts")
            .fetch_one(&mut conn)
            .await?;

    assert_eq!(account.id, 1);
    assert_eq!(account.name, "Herp Derpinson");
    assert_eq!(account.email, None);
    assert_eq!(account.deleted, Some(true));

    Ok(())
}
