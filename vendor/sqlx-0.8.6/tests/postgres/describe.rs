use sqlx::{postgres::Postgres, Column, Executor, TypeInfo};
use sqlx_test::new;

#[sqlx_macros::test]
async fn it_describes_simple() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let d = conn.describe("SELECT * FROM tweet").await?;

    assert_eq!(d.columns()[0].name(), "id");
    assert_eq!(d.columns()[1].name(), "created_at");
    assert_eq!(d.columns()[2].name(), "text");
    assert_eq!(d.columns()[3].name(), "owner_id");

    assert_eq!(d.nullable(0), Some(false));
    assert_eq!(d.nullable(1), Some(false));
    assert_eq!(d.nullable(2), Some(false));
    assert_eq!(d.nullable(3), Some(true));

    assert_eq!(d.columns()[0].type_info().name(), "INT8");
    assert_eq!(d.columns()[1].type_info().name(), "TIMESTAMPTZ");
    assert_eq!(d.columns()[2].type_info().name(), "TEXT");
    assert_eq!(d.columns()[3].type_info().name(), "INT8");

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_expression() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let d = conn.describe("SELECT 1::int8 + 10").await?;

    // ?column? will cause the macro to emit an error ad ask the user to explicitly name the type
    assert_eq!(d.columns()[0].name(), "?column?");

    // postgres cannot infer nullability from an expression
    // this will cause the macro to emit `Option<_>`
    assert_eq!(d.nullable(0), None);
    assert_eq!(d.columns()[0].type_info().name(), "INT8");

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_enum() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let d = conn.describe("SELECT 'open'::status as _1").await?;

    assert_eq!(d.columns()[0].name(), "_1");

    let ty = d.columns()[0].type_info();

    assert_eq!(ty.name(), "status");

    assert_eq!(
        format!("{:?}", ty.kind()),
        r#"Enum(["new", "open", "closed"])"#
    );

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_record() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let d = conn.describe("SELECT (true, 10::int2)").await?;

    let ty = d.columns()[0].type_info();
    assert_eq!(ty.name(), "RECORD");

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_composite() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let d = conn
        .describe("SELECT ROW('name',10,500)::inventory_item")
        .await?;

    let ty = d.columns()[0].type_info();

    assert_eq!(ty.name(), "inventory_item");

    assert_eq!(
        format!("{:?}", ty.kind()),
        r#"Composite([("name", PgTypeInfo(Text)), ("supplier_id", PgTypeInfo(Int4)), ("price", PgTypeInfo(Int8))])"#
    );

    Ok(())
}
