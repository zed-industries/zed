use sqlx::{Connection, MySql, MySqlConnection, Transaction};
use sqlx_test::new;

#[sqlx_macros::test]
async fn macro_select_from_cte() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;
    let account =
        sqlx::query!("select * from (select (1) as id, 'Herp Derpinson' as name, cast(null as char) email) accounts")
            .fetch_one(&mut conn)
            .await?;

    assert_eq!(account.id, 1);
    assert_eq!(account.name, "Herp Derpinson");
    // MySQL can tell us the nullability of expressions, ain't that cool
    assert_eq!(account.email, None);

    Ok(())
}

#[sqlx_macros::test]
async fn macro_select_from_cte_bind() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;
    let account = sqlx::query!(
        "select * from (select (1) as id, 'Herp Derpinson' as name) accounts where id = ?",
        1i32
    )
    .fetch_one(&mut conn)
    .await?;

    println!("{account:?}");
    println!("{}: {}", account.id, account.name);

    Ok(())
}

#[derive(Debug)]
struct RawAccount {
    r#type: i32,
    name: Option<String>,
}

#[sqlx_macros::test]
async fn test_query_as_raw() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    let account = sqlx::query_as!(
        RawAccount,
        "SELECT * from (select 1 as type, cast(null as char) as name) accounts"
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(account.name, None);
    assert_eq!(account.r#type, 1);

    println!("{account:?}");

    Ok(())
}

#[sqlx_macros::test]
async fn test_query_scalar() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    let id = sqlx::query_scalar!("select 1").fetch_one(&mut conn).await?;
    // MySQL tells us `LONG LONG` while MariaDB just `LONG`
    assert_eq!(id, 1);

    // invalid column names are ignored
    let id = sqlx::query_scalar!(r#"select 1 as `&foo`"#)
        .fetch_one(&mut conn)
        .await?;
    assert_eq!(id, 1);

    let id = sqlx::query_scalar!(r#"select 1 as `foo!`"#)
        .fetch_one(&mut conn)
        .await?;
    assert_eq!(id, 1);

    let id = sqlx::query_scalar!(r#"select 1 as `foo?`"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, Some(1));

    let id = sqlx::query_scalar!(r#"select 1 as `foo: MyInt`"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt(1));

    let id = sqlx::query_scalar!(r#"select 1 as `foo?: MyInt`"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, Some(MyInt(1)));

    let id = sqlx::query_scalar!(r#"select 1 as `foo!: MyInt`"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt(1));

    let id: MyInt = sqlx::query_scalar!(r#"select 1 as `foo: _`"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt(1));

    let id: MyInt = sqlx::query_scalar!(r#"select 1 as `foo?: _`"#)
        .fetch_one(&mut conn)
        .await?
        // don't hint that it should be `Option<MyInt>`
        .unwrap();

    assert_eq!(id, MyInt(1));

    let id: MyInt = sqlx::query_scalar!(r#"select 1 as `foo!: _`"#)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(id, MyInt(1));

    Ok(())
}

#[sqlx_macros::test]
async fn test_query_as_bool() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    struct Article {
        id: i32,
        deleted: bool,
    }

    let article = sqlx::query_as_unchecked!(
        Article,
        "select * from (select 51 as id, true as deleted) articles"
    )
    .fetch_one(&mut conn)
    .await?;

    assert_eq!(51, article.id);
    assert_eq!(true, article.deleted);

    Ok(())
}

#[sqlx_macros::test]
async fn test_query_bytes() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    let rec = sqlx::query!("SELECT X'01AF' as _1")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(rec._1, &[0x01_u8, 0xAF_u8]);

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_not_null() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    let record = sqlx::query!("select * from (select 1 as `id!`) records")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, 1);

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_nullable() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    // MySQL by default tells us `id` is not-null
    let record = sqlx::query!("select * from (select 1 as `id?`) records")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.id, Some(1));

    Ok(())
}

async fn with_test_row<'a>(
    conn: &'a mut MySqlConnection,
) -> anyhow::Result<(Transaction<'a, MySql>, MyInt)> {
    let mut transaction = conn.begin().await?;
    let id = sqlx::query!("INSERT INTO tweet(text, owner_id) VALUES ('#sqlx is pretty cool!', 1)")
        .execute(&mut *transaction)
        .await?
        .last_insert_id();
    Ok((transaction, MyInt(id as i64)))
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
    let mut conn = new::<MySql>().await?;
    let (mut conn, id) = with_test_row(&mut conn).await?;

    let record = sqlx::query_as!(Record, "select id as `id: _` from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, id);

    // this syntax is also useful for expressions
    let record = sqlx::query_as!(Record, "select * from (select 1 as `id: _`) records")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    let record = sqlx::query_as!(OptionalRecord, "select owner_id as `id: _` from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_wildcard_not_null() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;
    let (mut conn, _) = with_test_row(&mut conn).await?;

    let record = sqlx::query_as!(Record, "select owner_id as `id!: _` from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_wildcard_nullable() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;
    let (mut conn, id) = with_test_row(&mut conn).await?;

    let record = sqlx::query_as!(OptionalRecord, "select id as `id?: _` from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Some(id));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;
    let (mut conn, id) = with_test_row(&mut conn).await?;

    let record = sqlx::query!("select id as `id: MyInt` from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, id);

    // we can also support this syntax for expressions
    let record = sqlx::query!("select * from (select 1 as `id: MyInt`) records")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    let record = sqlx::query!("select owner_id as `id: MyInt` from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Some(MyInt(1)));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact_not_null() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;
    let (mut conn, _) = with_test_row(&mut conn).await?;

    let record = sqlx::query!("select owner_id as `id!: MyInt` from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, MyInt(1));

    Ok(())
}

#[sqlx_macros::test]
async fn test_column_override_exact_nullable() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;
    let (mut conn, id) = with_test_row(&mut conn).await?;

    let record = sqlx::query!("select id as `id?: MyInt` from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Some(id));

    Ok(())
}

#[derive(PartialEq, Eq, Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
enum MyEnum {
    Red,
    Green,
    Blue,
}

#[derive(PartialEq, Eq, Debug, sqlx::Type)]
#[repr(i32)]
enum MyCEnum {
    Red = 0,
    Green,
    Blue,
}

#[sqlx_macros::test]
async fn test_column_override_exact_enum() -> anyhow::Result<()> {
    let mut conn = new::<MySql>().await?;

    let record = sqlx::query!("select * from (select 'red' as `color: MyEnum`) records")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.color, MyEnum::Red);

    let record = sqlx::query!("select * from (select 2 as `color: MyCEnum`) records")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.color, MyCEnum::Blue);

    Ok(())
}

#[sqlx_macros::test]
async fn test_try_from_attr_for_native_type() -> anyhow::Result<()> {
    #[derive(sqlx::FromRow)]
    struct Record {
        #[sqlx(try_from = "i64")]
        id: u64,
    }

    let mut conn = new::<MySql>().await?;
    let (mut conn, id) = with_test_row(&mut conn).await?;

    let record = sqlx::query_as::<_, Record>("select id from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, id.0 as u64);

    Ok(())
}

#[sqlx_macros::test]
async fn test_try_from_attr_for_custom_type() -> anyhow::Result<()> {
    #[derive(sqlx::FromRow)]
    struct Record {
        #[sqlx(try_from = "i64")]
        id: Id,
    }

    #[derive(Debug, PartialEq)]
    struct Id(i64);
    impl std::convert::TryFrom<i64> for Id {
        type Error = std::io::Error;
        fn try_from(value: i64) -> Result<Self, Self::Error> {
            Ok(Id(value))
        }
    }

    let mut conn = new::<MySql>().await?;
    let (mut conn, id) = with_test_row(&mut conn).await?;

    let record = sqlx::query_as::<_, Record>("select id from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, Id(id.0));

    Ok(())
}

#[sqlx_macros::test]
async fn test_try_from_attr_with_flatten() -> anyhow::Result<()> {
    #[derive(sqlx::FromRow)]
    struct Record {
        #[sqlx(try_from = "Id", flatten)]
        id: u64,
    }

    #[derive(Debug, PartialEq, sqlx::FromRow)]
    struct Id {
        id: i64,
    }

    impl std::convert::TryFrom<Id> for u64 {
        type Error = std::io::Error;
        fn try_from(value: Id) -> Result<Self, Self::Error> {
            Ok(value.id as u64)
        }
    }

    let mut conn = new::<MySql>().await?;
    let (mut conn, id) = with_test_row(&mut conn).await?;

    let record = sqlx::query_as::<_, Record>("select id from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, id.0 as u64);

    Ok(())
}

#[sqlx_macros::test]
async fn test_try_from_attr_with_complex_type() -> anyhow::Result<()> {
    mod m {
        #[derive(sqlx::Type)]
        #[sqlx(transparent)]
        pub struct ComplexType<T>(T);

        impl std::convert::TryFrom<ComplexType<i64>> for u64 {
            type Error = std::num::TryFromIntError;
            fn try_from(value: ComplexType<i64>) -> Result<Self, Self::Error> {
                u64::try_from(value.0)
            }
        }
    }

    #[derive(sqlx::FromRow)]
    struct Record {
        #[sqlx(try_from = "m::ComplexType<i64>")]
        id: u64,
    }

    let mut conn = new::<MySql>().await?;
    let (mut conn, id) = with_test_row(&mut conn).await?;

    let record = sqlx::query_as::<_, Record>("select id from tweet")
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(record.id, id.0 as u64);

    Ok(())
}

#[sqlx_macros::test]
async fn test_from_row_json_attr() -> anyhow::Result<()> {
    #[derive(serde::Deserialize)]
    struct J {
        a: u32,
        b: u32,
    }

    #[derive(sqlx::FromRow)]
    struct Record {
        #[sqlx(json)]
        j: J,
    }

    let mut conn = new::<MySql>().await?;

    let record = sqlx::query_as::<_, Record>("select json_object('a', 1, 'b', 2) as j")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.j.a, 1);
    assert_eq!(record.j.b, 2);

    Ok(())
}

#[sqlx_macros::test]
async fn test_from_row_json_attr_nullable() -> anyhow::Result<()> {
    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct J {
        a: u32,
        b: u32,
    }

    #[derive(sqlx::FromRow)]
    struct Record {
        #[sqlx(json(nullable))]
        j: Option<J>,
    }

    let mut conn = new::<MySql>().await?;

    let record = sqlx::query_as::<_, Record>("select NULL as j")
        .fetch_one(&mut conn)
        .await?;

    assert!(record.j.is_none());
    Ok(())
}

#[sqlx_macros::test]
async fn test_from_row_json_try_from_attr() -> anyhow::Result<()> {
    #[derive(serde::Deserialize)]
    struct J {
        a: u32,
        b: u32,
    }

    // Non-deserializable
    struct J2 {
        sum: u32,
    }

    impl std::convert::From<J> for J2 {
        fn from(j: J) -> Self {
            Self { sum: j.a + j.b }
        }
    }

    #[derive(sqlx::FromRow)]
    struct Record {
        #[sqlx(json, try_from = "J")]
        j: J2,
    }

    let mut conn = new::<MySql>().await?;

    let record = sqlx::query_as::<_, Record>("select json_object('a', 1, 'b', 2) as j")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(record.j.sum, 3);

    Ok(())
}

// we don't emit bind parameter type-checks for MySQL so testing the overrides is redundant
