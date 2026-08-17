use sqlx::Sqlite;
use sqlx_test::new;

#[sqlx_macros::test]
async fn macro_select() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let account = sqlx::query!("select id, name, is_active from accounts where id = 1")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(1, account.id);
    assert_eq!("Herp Derpinson", account.name);
    assert_eq!(account.is_active, Some(true));

    Ok(())
}

macro_rules! gen_macro_select_concats {
    ($param:literal) => {
        #[sqlx_macros::test]
        async fn macro_select_concat_single() -> anyhow::Result<()> {
            let mut conn = new::<Sqlite>().await?;

            let account = sqlx::query!("select " + $param + " from accounts where id = 1")
                .fetch_one(&mut conn)
                .await?;

            assert_eq!(1, account.id);
            assert_eq!("Herp Derpinson", account.name);
            assert_eq!(account.is_active, Some(true));

            Ok(())
        }
    };
}

gen_macro_select_concats!("id, name, is_active");

#[sqlx_macros::test]
async fn macro_select_expression() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let row = sqlx::query!("select 10 as _1, 'Hello' as _2")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(10, row._1);
    assert_eq!("Hello", &*row._2);

    Ok(())
}

#[sqlx_macros::test]
async fn macro_select_partial_expression() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let row = sqlx::query!(
        "select 10 as _1, 'Hello' as _2, is_active, name, id + 5 as id_p from accounts where id = 1"
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(10, row._1);
    assert_eq!("Hello", &*row._2);
    assert_eq!(6, row.id_p);
    assert_eq!("Herp Derpinson", row.name);
    assert_eq!(row.is_active, Some(true));

    Ok(())
}

#[sqlx_macros::test]
async fn macro_select_bind() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let account = sqlx::query!(
        "select id, name, is_active from accounts where id = ?",
        1i32
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(1, account.id);
    assert_eq!("Herp Derpinson", account.name);
    assert_eq!(account.is_active, Some(true));

    Ok(())
}

#[derive(Debug)]
struct RawAccount {
    id: i64,
    name: String,
    is_active: Option<bool>,
}

#[sqlx_macros::test]
async fn test_query_as_raw() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let account = sqlx::query_as!(RawAccount, "SELECT id, name, is_active from accounts")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(account.id, 1);
    assert_eq!(account.name, "Herp Derpinson");
    assert_eq!(account.is_active, Some(true));

    Ok(())
}

#[sqlx_macros::test]
async fn test_query_scalar() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let id = sqlx::query_scalar!("select 1").fetch_one(&mut conn).await?;
    assert_eq!(id, 1i64);

    // invalid column names are ignored
    let id = sqlx::query_scalar!(r#"select 1 as "&foo""#)
        .fetch_one(&mut conn)
        .await?;
    assert_eq!(id, 1i64);

    let id = sqlx::query_scalar!(r#"select 1 as "foo!""#)
        .fetch_one(&mut conn)
        .await?;
    assert_eq!(id, 1i64);

    let id = sqlx::query_scalar!(r#"select 1 as "foo?""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, Some(1i64));

    let id = sqlx::query_scalar!(r#"select 1 as "foo: MyInt""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt(1i64));

    let id = sqlx::query_scalar!(r#"select 1 as "foo?: MyInt""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, Some(MyInt(1i64)));

    let id = sqlx::query_scalar!(r#"select 1 as "foo!: MyInt""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt(1i64));

    let id: MyInt = sqlx::query_scalar!(r#"select 1 as "foo: _""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt(1i64));

    let id: MyInt = sqlx::query_scalar!(r#"select 1 as "foo?: _""#)
        .fetch_one(&mut conn)
        .await?
        // don't hint that it should be `Option<MyInt>`
        .unwrap();

    assert_eq!(id, MyInt(1i64));

    let id: MyInt = sqlx::query_scalar!(r#"select 1 as "foo!: _""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt(1i64));

    Ok(())
}

#[sqlx_macros::test]
async fn macro_select_from_view() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let account = sqlx::query!("SELECT id, name, is_active from accounts_view")
        .fetch_one(&mut conn)
        .await?;

    // SQLite tells us the true origin of these columns even through the view
    assert_eq!(account.id, 1);
    assert_eq!(account.name, "Herp Derpinson");
    assert_eq!(account.is_active, Some(true));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_not_null() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let record = sqlx::query!(r#"select owner_id as `owner_id!` from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.owner_id, 1);

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_nullable() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let record = sqlx::query!(r#"select text as `text?` from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.text.as_deref(), Some("#sqlx is pretty cool!"));

    Ok(())
}

#[derive(PartialEq, Eq, Debug, sqlx::Type)]
#[sqlx(transparent)]
struct MyInt(i64);

struct Record {
    id: MyInt,
}

struct OptionalRecord {
    id: Option<MyInt>,
}

#[sqlx_macros::test]
async fn test_column_override_wildcard() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let record = sqlx::query_as!(Record, r#"select id as "id: _" from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    // this syntax is also useful for expressions
    let record = sqlx::query_as!(Record, r#"select 1 as "id: _""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    let record = sqlx::query_as!(OptionalRecord, r#"select owner_id as "id: _" from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_wildcard_not_null() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let record = sqlx::query_as!(Record, r#"select owner_id as "id!: _" from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_wildcard_nullable() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let record = sqlx::query_as!(OptionalRecord, r#"select id as "id?: _" from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let record = sqlx::query!(r#"select id as "id: MyInt" from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    // we can also support this syntax for expressions
    let record = sqlx::query!(r#"select 1 as "id: MyInt""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    let record = sqlx::query!(r#"select owner_id as "id: MyInt" from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact_not_null() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let record = sqlx::query!(r#"select owner_id as "id!: MyInt" from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact_nullable() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let record = sqlx::query!(r#"select id as "id?: MyInt" from tweet"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

// we don't emit bind parameter typechecks for SQLite so testing the overrides is redundant
