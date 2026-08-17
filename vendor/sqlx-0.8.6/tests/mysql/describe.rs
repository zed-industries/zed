use sqlx::mysql::MySql;
use sqlx::{Column, Executor, Type, TypeInfo};
use sqlx_test::new;

#[sqlx_macros::test]
async fn it_describes_simple() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    let d = conn.describe("SELECT * FROM tweet").await?;

    assert_eq!(d.columns()[0].name(), "id");
    assert_eq!(d.columns()[1].name(), "created_at");
    assert_eq!(d.columns()[2].name(), "text");
    assert_eq!(d.columns()[3].name(), "owner_id");

    assert_eq!(d.nullable(0), Some(false));
    assert_eq!(d.nullable(1), Some(false));
    assert_eq!(d.nullable(2), Some(false));
    assert_eq!(d.nullable(3), Some(true));

    assert_eq!(d.columns()[0].type_info().name(), "BIGINT");
    assert_eq!(d.columns()[1].type_info().name(), "TIMESTAMP");
    assert_eq!(d.columns()[2].type_info().name(), "TEXT");
    assert_eq!(d.columns()[3].type_info().name(), "BIGINT");

    Ok(())
}

#[sqlx_macros::test]
async fn test_boolean() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    conn.execute(
        r#"
CREATE TEMPORARY TABLE with_bit_and_tinyint (
    id INT PRIMARY KEY AUTO_INCREMENT,
    value_bit_1 BIT(1),
    value_bool BOOLEAN,
    bit_n BIT(64),
    value_int TINYINT
);
    "#,
    )
    .await?;

    let d = conn.describe("SELECT * FROM with_bit_and_tinyint").await?;

    assert_eq!(d.column(2).name(), "value_bool");
    assert_eq!(d.column(2).type_info().name(), "BOOLEAN");

    assert_eq!(d.column(1).name(), "value_bit_1");
    assert_eq!(d.column(1).type_info().name(), "BIT");

    assert!(<bool as Type<MySql>>::compatible(&d.column(1).type_info()));
    assert!(<bool as Type<MySql>>::compatible(&d.column(2).type_info()));

    Ok(())
}

#[sqlx_macros::test]
async fn uses_alias_name() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    let d = conn
        .describe("SELECT text AS tweet_text FROM tweet")
        .await?;

    assert_eq!(d.columns()[0].name(), "tweet_text");

    Ok(())
}
