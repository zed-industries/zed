use sqlx::mssql::Mssql;
use sqlx::{Column, Executor, TypeInfo};
use sqlx_test::new;

#[sqlx_macros::test]
async fn it_describes_simple() -> anyhow::Result<()> {
    let mut conn = new::<Mssql>().await?;

    let d = conn.describe("SELECT * FROM tweet").await?;

    assert_eq!(d.columns()[0].name(), "id");
    assert_eq!(d.columns()[1].name(), "text");
    assert_eq!(d.columns()[2].name(), "is_sent");
    assert_eq!(d.columns()[3].name(), "owner_id");

    assert_eq!(d.nullable(0), Some(false));
    assert_eq!(d.nullable(1), Some(false));
    assert_eq!(d.nullable(2), Some(false));
    assert_eq!(d.nullable(3), Some(true));

    assert_eq!(d.columns()[0].type_info().name(), "BIGINT");
    assert_eq!(d.columns()[1].type_info().name(), "NVARCHAR");
    assert_eq!(d.columns()[2].type_info().name(), "TINYINT");
    assert_eq!(d.columns()[3].type_info().name(), "BIGINT");

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_with_params() -> anyhow::Result<()> {
    let mut conn = new::<Mssql>().await?;

    let d = conn
        .describe("SELECT text FROM tweet WHERE id = @p1")
        .await?;

    assert_eq!(d.columns()[0].name(), "text");
    assert_eq!(d.nullable(0), Some(false));

    Ok(())
}
