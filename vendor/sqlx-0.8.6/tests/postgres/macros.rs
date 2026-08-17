use sqlx::{Connection, PgConnection, Postgres, Transaction};
use sqlx_postgres::types::PgHstore;
use sqlx_test::new;

use futures::TryStreamExt;

#[sqlx_macros::test]
async fn test_query() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let account = sqlx::query!(
        "SELECT * from (VALUES (1, 'Herp Derpinson')) accounts(id, name) where id = $1",
        1i32
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(account.id, Some(1));
    assert_eq!(account.name.as_deref(), Some("Herp Derpinson"));

    Ok(())
}

#[sqlx_macros::test]
async fn test_non_null() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut tx = conn.begin().await?;

    let _ = sqlx::query!("INSERT INTO tweet (text) VALUES ('Hello')")
        .execute(&mut *tx)
        .await?;

    let row = sqlx::query!("SELECT id, text, owner_id FROM tweet LIMIT 1")
        .fetch_one(&mut *tx)
        .await?;

    assert!(row.id > 0);
    assert_eq!(row.text, "Hello");
    assert_eq!(row.owner_id, None);

    // let the transaction rollback so we don't actually insert the tweet

    Ok(())
}

#[sqlx_macros::test]
async fn test_no_result() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut tx = conn.begin().await?;

    let _ = sqlx::query!("DELETE FROM tweet").execute(&mut *tx).await?;

    // let the transaction rollback so we don't actually delete the tweets

    Ok(())
}

#[sqlx_macros::test]
async fn test_text_var_char_char_n() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    // TEXT
    // we cannot infer nullability from an expression
    let rec = sqlx::query!("SELECT 'Hello'::text as greeting")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(rec.greeting.as_deref(), Some("Hello"));

    // VARCHAR(N)

    let rec = sqlx::query!("SELECT 'Hello'::varchar(5) as greeting")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(rec.greeting.as_deref(), Some("Hello"));

    // CHAR(N)

    let rec = sqlx::query!("SELECT 'Hello'::char(5) as greeting")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(rec.greeting.as_deref(), Some("Hello"));

    Ok(())
}

#[sqlx_macros::test]
async fn test_void() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let _ = sqlx::query!(r#"select pg_notify('chan', 'message')"#)
        .execute(&mut conn)
        .await?;

    Ok(())
}

#[sqlx_macros::test]
async fn test_call_procedure() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let row = sqlx::query!(r#"CALL forty_two(null)"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(row.forty_two, Some(42));

    Ok(())
}

#[sqlx_macros::test]
async fn test_query_file() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    // keep trailing comma as a test
    let account = sqlx::query_file!("tests/postgres/test-query.sql")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(account.id, 1);
    assert_eq!(account.name, Option::<String>::None);

    Ok(())
}

#[derive(Debug)]
struct Account {
    id: i32,
    name: Option<String>,
}

#[sqlx_macros::test]
async fn test_query_as() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let name: Option<&str> = None;
    let account = sqlx::query_as!(
        Account,
        r#"SELECT id "id!", name from (VALUES (1, $1)) accounts(id, name)"#,
        name
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(1, account.id);
    assert_eq!(None, account.name);

    println!("{account:?}");

    Ok(())
}

#[derive(Debug)]
struct RawAccount {
    r#type: i32,
    name: Option<String>,
}

#[sqlx_macros::test]
async fn test_query_as_raw() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let account = sqlx::query_as!(
        RawAccount,
        r#"SELECT type "type!", name from (VALUES (1, null)) accounts(type, name)"#
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(None, account.name);
    assert_eq!(1, account.r#type);

    println!("{account:?}");

    Ok(())
}

#[sqlx_macros::test]
async fn test_query_file_as() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let account = sqlx::query_file_as!(Account, "tests/postgres/test-query.sql")
        .fetch_one(&mut conn)
        .await?;

    println!("{account:?}");

    Ok(())
}

#[sqlx_macros::test]
async fn test_query_scalar() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let id = sqlx::query_scalar!("select 1").fetch_one(&mut conn).await?;
    // nullability inference can't handle expressions
    assert_eq!(id, Some(1i32));

    // invalid column names are ignored
    let id = sqlx::query_scalar!(r#"select 1 as "&foo""#)
        .fetch_one(&mut conn)
        .await?;
    assert_eq!(id, Some(1i32));

    let id = sqlx::query_scalar!(r#"select 1 as "foo!""#)
        .fetch_one(&mut conn)
        .await?;
    assert_eq!(id, 1i32);

    let id = sqlx::query_scalar!(r#"select 1 as "foo?""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, Some(1i32));

    let id = sqlx::query_scalar!(r#"select 1 as "foo: MyInt4""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, Some(MyInt4(1i32)));

    let id = sqlx::query_scalar!(r#"select 1 as "foo?: MyInt4""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, Some(MyInt4(1i32)));

    let id = sqlx::query_scalar!(r#"select 1 as "foo!: MyInt4""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt4(1i32));

    let id: MyInt4 = sqlx::query_scalar!(r#"select 1 as "foo: _""#)
        .fetch_one(&mut conn)
        .await?
        // don't hint that it should be `Option<MyInt4>`
        .unwrap();

    assert_eq!(id, MyInt4(1i32));

    let id: MyInt4 = sqlx::query_scalar!(r#"select 1 as "foo?: _""#)
        .fetch_one(&mut conn)
        .await?
        // don't hint that it should be `Option<MyInt4>`
        .unwrap();

    assert_eq!(id, MyInt4(1i32));

    let id: MyInt4 = sqlx::query_scalar!(r#"select 1 as "foo!: _""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt4(1i32));

    Ok(())
}

#[sqlx_macros::test]
async fn query_by_string() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let string = "Hello, world!".to_string();
    let ref tuple = ("Hello, world!".to_string(),);

    let result = sqlx::query!(
        "SELECT * from (VALUES('Hello, world!')) strings(string)\
         where string in ($1, $2, $3, $4, $5, $6, $7)",
        string, // make sure we don't actually take ownership here
        &string[..],
        Some(&string),
        Some(&string[..]),
        Option::<String>::None,
        string.clone(),
        tuple.0 // make sure we're not trying to move out of a field expression
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(result.string, Some(string));

    Ok(())
}

#[sqlx_macros::test]
#[cfg(feature = "bigdecimal")]
async fn query_by_bigdecimal() -> anyhow::Result<()> {
    use sqlx::types::BigDecimal;
    let mut conn = new::<Postgres>().await?;

    // this tests querying by a non-`Copy` type that doesn't have special reborrow semantics

    let decimal = "1234".parse::<BigDecimal>()?;
    let ref tuple = ("51245.121232".parse::<BigDecimal>()?,);

    let result = sqlx::query!(
        "SELECT * from (VALUES(1234.0)) decimals(decimal)\
         where decimal in ($1, $2, $3, $4, $5, $6, $7)",
        decimal,  // make sure we don't actually take ownership here
        &decimal, // allow query-by-reference
        Some(&decimal),
        Some(&decimal),
        Option::<BigDecimal>::None,
        decimal.clone(),
        tuple.0 // make sure we're not trying to move out of a field expression
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(result.decimal, Some(decimal));

    Ok(())
}

#[sqlx_macros::test]
async fn test_nullable_err() -> anyhow::Result<()> {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Account {
        id: i32,
        name: String,
    }

    let mut conn = new::<Postgres>().await?;

    let err = sqlx::query_as!(
        Account,
        r#"SELECT id "id!", name "name!" from (VALUES (1, null::text)) accounts(id, name)"#
    )
    .fetch_one(&mut conn)
    .await
    .unwrap_err();

    if let sqlx::Error::ColumnDecode { source, .. } = &err {
        if let Some(sqlx::error::UnexpectedNullError) = source.downcast_ref() {
            return Ok(());
        }
    }

    panic!("expected `UnexpectedNullError`, got {err}")
}

#[sqlx_macros::test]
async fn test_many_args() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    // previous implementation would only have supported 10 bind parameters
    // (this is really gross to test in MySQL)
    let rows = sqlx::query!(
        "SELECT * from unnest(array[$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12]::int[]) ids(id);",
        0i32, 1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32, 10i32, 11i32
    )
        .fetch_all(&mut conn)
        .await?;

    for (i, row) in rows.iter().enumerate() {
        assert_eq!(Some(i as i32), row.id);
    }

    Ok(())
}

#[sqlx_macros::test]
async fn test_array_from_slice() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let list: &[i32] = &[1, 2, 3, 4i32];

    let result = sqlx::query!("SELECT $1::int[] as my_array", list)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(result.my_array, Some(vec![1, 2, 3, 4]));

    println!("result ID: {:?}", result.my_array);

    let account = sqlx::query!("SELECT ARRAY[4,3,2,1] as my_array")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(account.my_array, Some(vec![4, 3, 2, 1]));

    println!("account ID: {:?}", account.my_array);

    Ok(())
}

#[sqlx_macros::test]
async fn fetch_is_usable_issue_224() -> anyhow::Result<()> {
    // ensures that the stream returned by `query::Map::fetch()` is usable with `TryStreamExt`
    let mut conn = new::<Postgres>().await?;

    let mut stream =
        sqlx::query!("select * from generate_series(1, 3) as series(num);").fetch(&mut conn);

    // `num` is generated by a function so we can't assume it's non-null
    assert_eq!(stream.try_next().await?.unwrap().num, Some(1));
    assert_eq!(stream.try_next().await?.unwrap().num, Some(2));
    assert_eq!(stream.try_next().await?.unwrap().num, Some(3));
    assert!(stream.try_next().await?.is_none());

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_not_null() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let record = sqlx::query!(r#"select 1 as "id!""#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, 1);

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_nullable() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    // workaround for https://github.com/launchbadge/sqlx/issues/367
    // declare a `NOT NULL` column from a left-joined table to be nullable
    let record = sqlx::query!(
        r#"select text as "text?" from (values (1)) foo(id) left join tweet on false"#
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(record.text, None);

    Ok(())
}

async fn with_test_row<'a>(
    conn: &'a mut PgConnection,
) -> anyhow::Result<Transaction<'a, Postgres>> {
    let mut transaction = conn.begin().await?;
    sqlx::query!("INSERT INTO tweet(id, text, owner_id) VALUES (1, '#sqlx is pretty cool!', 1)")
        .execute(&mut *transaction)
        .await?;
    Ok(transaction)
}

#[derive(PartialEq, Eq, Debug, sqlx::Type)]
#[sqlx(transparent)]
struct MyInt(i64);

#[derive(PartialEq, Eq, Debug, sqlx::Type)]
#[sqlx(transparent)]
struct MyInt4(i32);

struct Record {
    id: MyInt,
}

struct OptionalRecord {
    id: Option<MyInt>,
}

#[sqlx_macros::test]
async fn test_column_override_wildcard() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut conn = with_test_row(&mut conn).await?;

    let record = sqlx::query_as!(Record, r#"select id as "id: _" from tweet"#)
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    let record = sqlx::query_as!(OptionalRecord, r#"select owner_id as "id: _" from tweet"#)
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_wildcard_not_null() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut conn = with_test_row(&mut conn).await?;

    let record = sqlx::query_as!(Record, r#"select owner_id as "id!: _" from tweet"#)
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_wildcard_nullable() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut conn = with_test_row(&mut conn).await?;

    let record = sqlx::query_as!(OptionalRecord, r#"select id as "id?: _" from tweet"#)
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut conn = with_test_row(&mut conn).await?;

    let record = sqlx::query!(r#"select id as "id: MyInt" from tweet"#)
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    let record = sqlx::query!(r#"select owner_id as "id: MyInt" from tweet"#)
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact_not_null() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut conn = with_test_row(&mut conn).await?;

    let record = sqlx::query!(r#"select owner_id as "id!: MyInt" from tweet"#)
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact_nullable() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut conn = with_test_row(&mut conn).await?;

    let record = sqlx::query!(r#"select id as "id?: MyInt" from tweet"#)
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_bind_arg_override_exact() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let my_int = MyInt4(1);

    // this query should require a bind parameter override as we would otherwise expect the bind
    // to be the same type
    let record = sqlx::query!(
        "select * from (select 1::int4) records(id) where id = $1",
        my_int as MyInt4
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(record.id, Some(1i32));

    // test that we're actually emitting the typecast by requiring the bound type to be the same
    let record = sqlx::query!("select $1::int8 as id", 1i32 as i64)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, Some(1i64));

    // test the override with `Option`
    let my_opt_int = Some(MyInt4(1));

    let record = sqlx::query!(
        "select * from (select 1::int4) records(id) where id = $1",
        my_opt_int as Option<MyInt4>
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(record.id, Some(1i32));

    Ok(())
}

#[sqlx_macros::test]
async fn test_bind_arg_override_wildcard() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let my_int = MyInt4(1);

    let record = sqlx::query!(
        "select * from (select 1::int4) records(id) where id = $1",
        my_int as _
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(record.id, Some(1i32));

    Ok(())
}

#[sqlx_macros::test]
async fn test_to_from_citext() -> anyhow::Result<()> {
    // Ensure that the macros consider `CITEXT` to be compatible with `String` and friends

    let mut conn = new::<Postgres>().await?;

    let mut tx = conn.begin().await?;

    let foo_in = "Hello, world!";

    sqlx::query!("insert into test_citext(foo) values ($1)", foo_in)
        .execute(&mut *tx)
        .await?;

    let foo_out: String = sqlx::query_scalar!("select foo from test_citext")
        .fetch_one(&mut *tx)
        .await?;

    assert_eq!(foo_in, foo_out);

    tx.rollback().await?;

    Ok(())
}

#[sqlx_macros::test]
async fn pghstore_tests() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;

    let mut store = PgHstore::default();
    let stores = vec![store.clone(), store.clone()];

    store.insert("key".into(), Some("value".to_string()));
    sqlx::query!("         insert into mytable(f) values ($1)", store)
        .execute(&mut conn)
        .await?;

    sqlx::query!(
        "         insert into mytable(f) select * from unnest($1::hstore[])",
        &stores
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}
