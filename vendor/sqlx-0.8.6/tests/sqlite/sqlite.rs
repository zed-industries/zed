use futures::TryStreamExt;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use sqlx::sqlite::{SqliteConnectOptions, SqliteOperation, SqlitePoolOptions};
use sqlx::{
    query, sqlite::Sqlite, sqlite::SqliteRow, Column, ConnectOptions, Connection, Executor, Row,
    SqliteConnection, SqlitePool, Statement, TypeInfo,
};
use sqlx_sqlite::LockedSqliteHandle;
use sqlx_test::new;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[sqlx_macros::test]
async fn it_connects() -> anyhow::Result<()> {
    Ok(new::<Sqlite>().await?.ping().await?)
}

#[sqlx_macros::test]
async fn it_fetches_and_inflates_row() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    // process rows, one-at-a-time
    // this reuses the memory of the row

    {
        let expected = [15, 39, 51];
        let mut i = 0;
        let mut s = conn.fetch("SELECT 15 UNION SELECT 51 UNION SELECT 39");

        while let Some(row) = s.try_next().await? {
            let v1 = row.get::<i32, _>(0);
            assert_eq!(expected[i], v1);
            i += 1;
        }
    }

    // same query, but fetch all rows at once
    // this triggers the internal inflation

    let rows = conn
        .fetch_all("SELECT 15 UNION SELECT 51 UNION SELECT 39")
        .await?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].get::<i32, _>(0), 15);
    assert_eq!(rows[1].get::<i32, _>(0), 39);
    assert_eq!(rows[2].get::<i32, _>(0), 51);

    // same query but fetch the first row a few times from a non-persistent query
    // these rows should be immediately inflated

    let row1 = conn
        .fetch_one("SELECT 15 UNION SELECT 51 UNION SELECT 39")
        .await?;

    assert_eq!(row1.get::<i32, _>(0), 15);

    let row2 = conn
        .fetch_one("SELECT 15 UNION SELECT 51 UNION SELECT 39")
        .await?;

    assert_eq!(row1.get::<i32, _>(0), 15);
    assert_eq!(row2.get::<i32, _>(0), 15);

    // same query (again) but make it persistent
    // and fetch the first row a few times

    let row1 = conn
        .fetch_one(query("SELECT 15 UNION SELECT 51 UNION SELECT 39"))
        .await?;

    assert_eq!(row1.get::<i32, _>(0), 15);

    let row2 = conn
        .fetch_one(query("SELECT 15 UNION SELECT 51 UNION SELECT 39"))
        .await?;

    assert_eq!(row1.get::<i32, _>(0), 15);
    assert_eq!(row2.get::<i32, _>(0), 15);

    Ok(())
}

#[sqlx_macros::test]
async fn it_maths() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let value = sqlx::query("select 1 + ?1")
        .bind(5_i32)
        .try_map(|row: SqliteRow| row.try_get::<i32, _>(0))
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(6i32, value);

    Ok(())
}

#[sqlx_macros::test]
async fn test_bind_multiple_statements_multiple_values() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let values: Vec<i32> = sqlx::query_scalar::<_, i32>("select ?; select ?")
        .bind(5_i32)
        .bind(15_i32)
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], 5);
    assert_eq!(values[1], 15);

    Ok(())
}

#[sqlx_macros::test]
async fn test_bind_multiple_statements_same_value() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let values: Vec<i32> = sqlx::query_scalar::<_, i32>("select ?1; select ?1")
        .bind(25_i32)
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], 25);
    assert_eq!(values[1], 25);

    Ok(())
}

#[sqlx_macros::test]
async fn it_can_describe_with_pragma() -> anyhow::Result<()> {
    use sqlx::{Decode, TypeInfo, ValueRef};

    let mut conn = new::<Sqlite>().await?;

    let defaults = sqlx::query("pragma table_info (tweet)")
        .try_map(|row: SqliteRow| {
            let val = row.try_get_raw("dflt_value")?;
            let ty = val.type_info().clone().into_owned();

            let val: Option<i32> = Decode::<Sqlite>::decode(val).map_err(sqlx::Error::Decode)?;

            if val.is_some() {
                assert_eq!(ty.name(), "TEXT");
            }

            Ok(val)
        })
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(defaults[0], None);
    assert_eq!(defaults[2], Some(0));

    Ok(())
}

#[sqlx_macros::test]
async fn it_binds_positional_parameters_issue_467() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let row: (i32, i32, i32, i32) = sqlx::query_as("select ?1, ?1, ?3, ?2")
        .bind(5_i32)
        .bind(500_i32)
        .bind(1020_i32)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(row.0, 5);
    assert_eq!(row.1, 5);
    assert_eq!(row.2, 1020);
    assert_eq!(row.3, 500);

    Ok(())
}

#[sqlx_macros::test]
async fn it_fetches_in_loop() -> anyhow::Result<()> {
    // this is trying to check for any data races
    // there were a few that triggered *sometimes* while building out StatementWorker
    for _ in 0..1000_usize {
        let mut conn = new::<Sqlite>().await?;
        let v: Vec<(i32,)> = sqlx::query_as("SELECT 1").fetch_all(&mut conn).await?;

        assert_eq!(v[0].0, 1);
    }

    Ok(())
}

#[sqlx_macros::test]
async fn it_executes_with_pool() -> anyhow::Result<()> {
    let pool: SqlitePool = SqlitePoolOptions::new()
        .min_connections(2)
        .max_connections(2)
        .test_before_acquire(false)
        .connect(&dotenvy::var("DATABASE_URL")?)
        .await?;

    let rows = pool.fetch_all("SELECT 1; SElECT 2").await?;

    assert_eq!(rows.len(), 2);

    Ok(())
}

#[cfg(sqlite_ipaddr)]
#[sqlx_macros::test]
async fn it_opens_with_extension() -> anyhow::Result<()> {
    use std::str::FromStr;

    let opts = SqliteConnectOptions::from_str(&dotenvy::var("DATABASE_URL")?)?.extension("ipaddr");

    let mut conn = SqliteConnection::connect_with(&opts).await?;
    conn.execute("SELECT ipmasklen('192.168.16.12/24');")
        .await?;
    conn.close().await?;

    Ok(())
}

#[sqlx_macros::test]
async fn it_opens_in_memory() -> anyhow::Result<()> {
    // If the filename is ":memory:", then a private, temporary in-memory database
    // is created for the connection.
    let conn = SqliteConnection::connect(":memory:").await?;
    conn.close().await?;

    Ok(())
}

#[sqlx_macros::test]
async fn it_opens_temp_on_disk() -> anyhow::Result<()> {
    // If the filename is an empty string, then a private, temporary on-disk database will
    // be created.
    let conn = SqliteConnection::connect("").await?;
    conn.close().await?;

    Ok(())
}

#[sqlx_macros::test]
async fn it_fails_to_parse() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;
    let res = sqlx::raw_sql("SEELCT 1").execute(&mut conn).await;

    assert!(res.is_err());

    let err = res.unwrap_err().to_string();

    assert_eq!(
        "error returned from database: (code: 1) near \"SEELCT\": syntax error",
        err
    );

    Ok(())
}

#[sqlx_macros::test]
async fn it_handles_empty_queries() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;
    let done = conn.execute("").await?;

    assert_eq!(done.rows_affected(), 0);

    Ok(())
}

#[sqlx_macros::test]
async fn it_binds_parameters() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let v: i32 = sqlx::query_scalar("SELECT ?")
        .bind(10_i32)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(v, 10);

    let v: (i32, i32) = sqlx::query_as("SELECT ?1, ?")
        .bind(10_i32)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(v.0, 10);
    assert_eq!(v.1, 10);

    Ok(())
}

#[sqlx_macros::test]
async fn it_binds_dollar_parameters() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let v: (i32, i32) = sqlx::query_as("SELECT $1, $2")
        .bind(10_i32)
        .bind(11_i32)
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(v.0, 10);
    assert_eq!(v.1, 11);

    Ok(())
}

#[sqlx_macros::test]
async fn it_executes_queries() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let _ = conn
        .execute(
            r#"
CREATE TEMPORARY TABLE users (id INTEGER PRIMARY KEY)
            "#,
        )
        .await?;

    for index in 1..=10_i32 {
        let done = sqlx::query("INSERT INTO users (id) VALUES (?)")
            .bind(index * 2)
            .execute(&mut conn)
            .await?;

        assert_eq!(done.rows_affected(), 1);
    }

    let sum: i32 = sqlx::query_as("SELECT id FROM users")
        .fetch(&mut conn)
        .try_fold(0_i32, |acc, (x,): (i32,)| async move { Ok(acc + x) })
        .await?;

    assert_eq!(sum, 110);

    Ok(())
}

#[sqlx_macros::test]
async fn it_can_execute_multiple_statements() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let done = conn
        .execute(
            r#"
CREATE TEMPORARY TABLE users (id INTEGER PRIMARY KEY, other INTEGER);
INSERT INTO users DEFAULT VALUES;
            "#,
        )
        .await?;

    assert_eq!(done.rows_affected(), 1);

    for index in 2..5_i32 {
        let (id, other): (i32, i32) = sqlx::query_as(
            r#"
INSERT INTO users (other) VALUES (?);
SELECT id, other FROM users WHERE id = last_insert_rowid();
            "#,
        )
        .bind(index)
        .fetch_one(&mut conn)
        .await?;

        assert_eq!(id, index);
        assert_eq!(other, index);
    }

    Ok(())
}

#[sqlx_macros::test]
async fn it_interleaves_reads_and_writes() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let mut cursor = conn.fetch(
        "
CREATE TABLE IF NOT EXISTS _sqlx_test (
    id INT PRIMARY KEY,
    text TEXT NOT NULL
);

SELECT 'Hello World' as _1;

INSERT INTO _sqlx_test (text) VALUES ('this is a test');

SELECT id, text FROM _sqlx_test;
    ",
    );

    let row = cursor.try_next().await?.unwrap();

    assert!("Hello World" == row.try_get::<&str, _>("_1")?);

    let row = cursor.try_next().await?.unwrap();

    let id: i64 = row.try_get("id")?;
    let text: &str = row.try_get("text")?;

    assert_eq!(0, id);
    assert_eq!("this is a test", text);

    Ok(())
}

#[sqlx_macros::test]
async fn it_supports_collations() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    // also tests `.lock_handle()`
    conn.lock_handle()
        .await?
        .create_collation("test_collation", |l, r| l.cmp(r).reverse())?;

    let _ = conn
        .execute(
            r#"
CREATE TEMPORARY TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL COLLATE test_collation)
            "#,
        )
        .await?;

    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("a")
        .execute(&mut conn)
        .await?;
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("b")
        .execute(&mut conn)
        .await?;

    let row: SqliteRow = conn
        .fetch_one("SELECT name FROM users ORDER BY name ASC")
        .await?;
    let name: &str = row.try_get(0)?;

    assert_eq!(name, "b");

    Ok(())
}

#[sqlx_macros::test]
async fn it_caches_statements() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    // Initial PRAGMAs are not cached as we are not going to execute
    // them more than once.
    assert_eq!(0, conn.cached_statements_size());

    // `&str` queries are not persistent.
    let row = conn.fetch_one("SELECT 100 AS val").await?;
    let val: i32 = row.get("val");
    assert_eq!(val, 100);
    assert_eq!(0, conn.cached_statements_size());

    // `Query` is persistent by default.
    let mut conn = new::<Sqlite>().await?;
    for i in 0..2 {
        let row = sqlx::query("SELECT ? AS val")
            .bind(i)
            .fetch_one(&mut conn)
            .await?;

        let val: i32 = row.get("val");

        assert_eq!(i, val);
    }
    assert_eq!(1, conn.cached_statements_size());

    // Cache can be cleared.
    conn.clear_cached_statements().await?;
    assert_eq!(0, conn.cached_statements_size());

    // `Query` is not persistent if `.persistent(false)` is used
    // explicitly.
    let mut conn = new::<Sqlite>().await?;
    for i in 0..2 {
        let row = sqlx::query("SELECT ? AS val")
            .bind(i)
            .persistent(false)
            .fetch_one(&mut conn)
            .await?;

        let val: i32 = row.get("val");

        assert_eq!(i, val);
    }
    assert_eq!(0, conn.cached_statements_size());

    Ok(())
}

#[sqlx_macros::test]
async fn it_can_prepare_then_execute() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;
    let mut tx = conn.begin().await?;

    let _ = sqlx::query("INSERT INTO tweet ( id, text ) VALUES ( 2, 'Hello, World' )")
        .execute(&mut *tx)
        .await?;

    let tweet_id: i32 = 2;

    let statement = tx.prepare("SELECT * FROM tweet WHERE id = ?1").await?;

    assert_eq!(statement.column(0).name(), "id");
    assert_eq!(statement.column(1).name(), "text");
    assert_eq!(statement.column(2).name(), "is_sent");
    assert_eq!(statement.column(3).name(), "owner_id");

    assert_eq!(statement.column(0).type_info().name(), "INTEGER");
    assert_eq!(statement.column(1).type_info().name(), "TEXT");
    assert_eq!(statement.column(2).type_info().name(), "BOOLEAN");
    assert_eq!(statement.column(3).type_info().name(), "INTEGER");

    let row = statement.query().bind(tweet_id).fetch_one(&mut *tx).await?;
    let tweet_text: &str = row.try_get("text")?;

    assert_eq!(tweet_text, "Hello, World");

    Ok(())
}

#[sqlx_macros::test]
async fn it_resets_prepared_statement_after_fetch_one() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    conn.execute("CREATE TEMPORARY TABLE foobar (id INTEGER)")
        .await?;
    conn.execute("INSERT INTO foobar VALUES (42)").await?;

    let r = sqlx::query("SELECT id FROM foobar")
        .fetch_one(&mut conn)
        .await?;
    let x: i32 = r.try_get("id")?;
    assert_eq!(x, 42);

    conn.execute("DROP TABLE foobar").await?;

    Ok(())
}

#[sqlx_macros::test]
async fn it_resets_prepared_statement_after_fetch_many() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    conn.execute("CREATE TEMPORARY TABLE foobar (id INTEGER)")
        .await?;
    conn.execute("INSERT INTO foobar VALUES (42)").await?;
    conn.execute("INSERT INTO foobar VALUES (43)").await?;

    let mut rows = sqlx::query("SELECT id FROM foobar").fetch(&mut conn);
    let row = rows.try_next().await?.unwrap();
    let x: i32 = row.try_get("id")?;
    assert_eq!(x, 42);
    drop(rows);

    conn.execute("DROP TABLE foobar").await?;

    Ok(())
}

// https://github.com/launchbadge/sqlx/issues/1300
#[sqlx_macros::test]
async fn concurrent_resets_dont_segfault() {
    use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions};
    use std::{str::FromStr, time::Duration};

    let mut conn = SqliteConnectOptions::from_str(":memory:")
        .unwrap()
        .connect()
        .await
        .unwrap();

    sqlx::query("CREATE TABLE stuff (name INTEGER, value INTEGER)")
        .execute(&mut conn)
        .await
        .unwrap();

    sqlx_core::rt::spawn(async move {
        for i in 0..1000 {
            sqlx::query("INSERT INTO stuff (name, value) VALUES (?, ?)")
                .bind(i)
                .bind(0)
                .execute(&mut conn)
                .await
                .unwrap();
        }
    });

    sqlx_core::rt::sleep(Duration::from_millis(1)).await;
}

// https://github.com/launchbadge/sqlx/issues/1419
// note: this passes before and after the fix; you need to run it with `--nocapture`
// to see the panic from the worker thread, which doesn't happen after the fix
#[sqlx_macros::test]
async fn row_dropped_after_connection_doesnt_panic() {
    let mut conn = SqliteConnection::connect(":memory:").await.unwrap();

    let books = sqlx::query("SELECT 'hello' AS title")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    for book in &books {
        // force the row to be inflated
        let _title: String = book.get("title");
    }

    // hold `books` past the lifetime of `conn`
    drop(conn);
    sqlx_core::rt::sleep(std::time::Duration::from_secs(1)).await;
    drop(books);
}

// note: to repro issue #1467 this should be run in release mode
// May spuriously fail with UNIQUE constraint failures (which aren't relevant to the original issue)
// which I have tried to reproduce using the same seed as printed from CI but to no avail.
// It may be due to some nondeterminism in SQLite itself for all I know.
#[sqlx_macros::test]
#[ignore]
async fn issue_1467() -> anyhow::Result<()> {
    let mut conn = SqliteConnectOptions::new()
        .filename(":memory:")
        .connect()
        .await?;

    sqlx::query(
        r#"
    CREATE TABLE kv (k PRIMARY KEY, v);
    CREATE INDEX idx_kv ON kv (v);
    "#,
    )
    .execute(&mut conn)
    .await?;

    // Random seed:
    let seed: [u8; 32] = rand::random();
    println!("RNG seed: {}", hex::encode(&seed));

    // Pre-determined seed:
    // let mut seed: [u8; 32] = [0u8; 32];
    // hex::decode_to_slice(
    //     "135234871d03fc0479e22f2f06395b6074761bac5fe7dcf205dbe01eef9f7794",
    //     &mut seed,
    // )?;

    // reproducible RNG for testing
    let mut rng = Xoshiro256PlusPlus::from_seed(seed);

    for i in 0..1_000_000 {
        if i % 1_000 == 0 {
            println!("{i}");
        }
        let key = rng.gen_range(0..1_000);
        let value = rng.gen_range(0..1_000);
        let mut tx = conn.begin().await?;

        let exists = sqlx::query("SELECT 1 FROM kv WHERE k = ?")
            .bind(key)
            .fetch_optional(&mut *tx)
            .await?;
        if exists.is_some() {
            sqlx::query("UPDATE kv SET v = ? WHERE k = ?")
                .bind(value)
                .bind(key)
                .execute(&mut *tx)
                .await?;
        } else {
            sqlx::query("INSERT INTO kv(k, v) VALUES (?, ?)")
                .bind(key)
                .bind(value)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
    }
    Ok(())
}

#[sqlx_macros::test]
async fn concurrent_read_and_write() {
    let pool: SqlitePool = SqlitePoolOptions::new()
        .min_connections(2)
        .connect(":memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE kv (k PRIMARY KEY, v)")
        .execute(&pool)
        .await
        .unwrap();

    let n = 100;

    let read = sqlx_core::rt::spawn({
        let mut conn = pool.acquire().await.unwrap();

        async move {
            for i in 0u32..n {
                sqlx::query("SELECT v FROM kv")
                    .bind(i)
                    .fetch_all(&mut *conn)
                    .await
                    .unwrap();
            }
        }
    });

    let write = sqlx_core::rt::spawn({
        let mut conn = pool.acquire().await.unwrap();

        async move {
            for i in 0u32..n {
                sqlx::query("INSERT INTO kv (k, v) VALUES (?, ?)")
                    .bind(i)
                    .bind(i * i)
                    .execute(&mut *conn)
                    .await
                    .unwrap();
            }
        }
    });

    read.await;
    write.await;
}

#[sqlx_macros::test]
async fn test_query_with_progress_handler() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    // Using this string as a canary to ensure the callback doesn't get called with the wrong data pointer.
    let state = format!("test");
    conn.lock_handle().await?.set_progress_handler(1, move || {
        assert_eq!(state, "test");
        false
    });

    match sqlx::query("SELECT 'hello' AS title")
        .fetch_all(&mut conn)
        .await
    {
        Err(sqlx::Error::Database(err)) => assert_eq!(err.message(), String::from("interrupted")),
        _ => panic!("expected an interrupt"),
    }

    Ok(())
}

#[sqlx_macros::test]
async fn test_multiple_set_progress_handler_calls_drop_old_handler() -> anyhow::Result<()> {
    let ref_counted_object = Arc::new(0);
    assert_eq!(1, Arc::strong_count(&ref_counted_object));

    {
        let mut conn = new::<Sqlite>().await?;

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_progress_handler(1, move || {
            println!("{o:?}");
            false
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_progress_handler(1, move || {
            println!("{o:?}");
            false
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_progress_handler(1, move || {
            println!("{o:?}");
            false
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        match sqlx::query("SELECT 'hello' AS title")
            .fetch_all(&mut conn)
            .await
        {
            Err(sqlx::Error::Database(err)) => {
                assert_eq!(err.message(), String::from("interrupted"))
            }
            _ => panic!("expected an interrupt"),
        }

        conn.lock_handle().await?.remove_progress_handler();
    }

    assert_eq!(1, Arc::strong_count(&ref_counted_object));
    Ok(())
}

#[sqlx_macros::test]
async fn test_query_with_update_hook() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;
    static CALLED: AtomicBool = AtomicBool::new(false);
    // Using this string as a canary to ensure the callback doesn't get called with the wrong data pointer.
    let state = format!("test");
    conn.lock_handle().await?.set_update_hook(move |result| {
        assert_eq!(state, "test");
        assert_eq!(result.operation, SqliteOperation::Insert);
        assert_eq!(result.database, "main");
        assert_eq!(result.table, "tweet");
        assert_eq!(result.rowid, 2);
        CALLED.store(true, Ordering::Relaxed);
    });

    let _ = sqlx::query("INSERT INTO tweet ( id, text ) VALUES ( 3, 'Hello, World' )")
        .execute(&mut conn)
        .await?;
    assert!(CALLED.load(Ordering::Relaxed));

    Ok(())
}

#[sqlx_macros::test]
async fn test_multiple_set_update_hook_calls_drop_old_handler() -> anyhow::Result<()> {
    let ref_counted_object = Arc::new(0);
    assert_eq!(1, Arc::strong_count(&ref_counted_object));

    {
        let mut conn = new::<Sqlite>().await?;

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_update_hook(move |_| {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_update_hook(move |_| {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_update_hook(move |_| {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        conn.lock_handle().await?.remove_update_hook();
    }

    assert_eq!(1, Arc::strong_count(&ref_counted_object));
    Ok(())
}

#[sqlx_macros::test]
async fn test_query_with_commit_hook() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;
    static CALLED: AtomicBool = AtomicBool::new(false);
    // Using this string as a canary to ensure the callback doesn't get called with the wrong data pointer.
    let state = format!("test");
    conn.lock_handle().await?.set_commit_hook(move || {
        CALLED.store(true, Ordering::Relaxed);
        assert_eq!(state, "test");
        false
    });

    let mut tx = conn.begin().await?;
    sqlx::query("INSERT INTO tweet ( id, text ) VALUES ( 4, 'Hello, World' )")
        .execute(&mut *tx)
        .await?;
    match tx.commit().await {
        Err(sqlx::Error::Database(err)) => {
            assert_eq!(err.message(), String::from("constraint failed"))
        }
        _ => panic!("expected an error"),
    }
    assert!(CALLED.load(Ordering::Relaxed));
    Ok(())
}

#[sqlx_macros::test]
async fn test_multiple_set_commit_hook_calls_drop_old_handler() -> anyhow::Result<()> {
    let ref_counted_object = Arc::new(0);
    assert_eq!(1, Arc::strong_count(&ref_counted_object));

    {
        let mut conn = new::<Sqlite>().await?;

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_commit_hook(move || {
            println!("{o:?}");
            true
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_commit_hook(move || {
            println!("{o:?}");
            true
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_commit_hook(move || {
            println!("{o:?}");
            true
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        conn.lock_handle().await?.remove_commit_hook();
    }

    assert_eq!(1, Arc::strong_count(&ref_counted_object));
    Ok(())
}

#[sqlx_macros::test]
async fn test_query_with_rollback_hook() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    // Using this string as a canary to ensure the callback doesn't get called with the wrong data pointer.
    let state = format!("test");
    static CALLED: AtomicBool = AtomicBool::new(false);
    conn.lock_handle().await?.set_rollback_hook(move || {
        assert_eq!(state, "test");
        CALLED.store(true, Ordering::Relaxed);
    });

    let mut tx = conn.begin().await?;
    sqlx::query("INSERT INTO tweet ( id, text ) VALUES (5, 'Hello, World' )")
        .execute(&mut *tx)
        .await?;
    tx.rollback().await?;
    assert!(CALLED.load(Ordering::Relaxed));
    Ok(())
}

#[sqlx_macros::test]
async fn test_multiple_set_rollback_hook_calls_drop_old_handler() -> anyhow::Result<()> {
    let ref_counted_object = Arc::new(0);
    assert_eq!(1, Arc::strong_count(&ref_counted_object));

    {
        let mut conn = new::<Sqlite>().await?;

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_rollback_hook(move || {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_rollback_hook(move || {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_rollback_hook(move || {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        conn.lock_handle().await?.remove_rollback_hook();
    }

    assert_eq!(1, Arc::strong_count(&ref_counted_object));
    Ok(())
}

#[cfg(feature = "sqlite-preupdate-hook")]
#[sqlx_macros::test]
async fn test_query_with_preupdate_hook_insert() -> anyhow::Result<()> {
    use sqlx::Decode;

    let mut conn = new::<Sqlite>().await?;
    static CALLED: AtomicBool = AtomicBool::new(false);
    // Using this string as a canary to ensure the callback doesn't get called with the wrong data pointer.
    let state = format!("test");
    conn.lock_handle().await?.set_preupdate_hook({
        move |result| {
            assert_eq!(state, "test");
            assert_eq!(result.operation, SqliteOperation::Insert);
            assert_eq!(result.database, "main");
            assert_eq!(result.table, "tweet");

            assert_eq!(4, result.get_column_count());
            assert_eq!(2, result.get_new_row_id().unwrap());
            assert_eq!(0, result.get_query_depth());
            assert_eq!(
                4,
                <i64 as Decode<Sqlite>>::decode(result.get_new_column_value(0).unwrap()).unwrap()
            );
            assert_eq!(
                "Hello, World",
                <String as Decode<Sqlite>>::decode(result.get_new_column_value(1).unwrap())
                    .unwrap()
            );
            // out of bounds access should return an error
            assert!(result.get_new_column_value(4).is_err());
            // old values aren't available for inserts
            assert!(result.get_old_column_value(0).is_err());
            assert!(result.get_old_row_id().is_err());

            CALLED.store(true, Ordering::Relaxed);
        }
    });

    let _ = sqlx::query("INSERT INTO tweet ( id, text ) VALUES ( 4, 'Hello, World' )")
        .execute(&mut conn)
        .await?;

    assert!(CALLED.load(Ordering::Relaxed));
    conn.lock_handle().await?.remove_preupdate_hook();
    let _ = sqlx::query("DELETE FROM tweet where id = 4")
        .execute(&mut conn)
        .await?;
    Ok(())
}

#[cfg(feature = "sqlite-preupdate-hook")]
#[sqlx_macros::test]
async fn test_query_with_preupdate_hook_delete() -> anyhow::Result<()> {
    use sqlx::Decode;

    let mut conn = new::<Sqlite>().await?;
    let _ = sqlx::query("INSERT INTO tweet ( id, text ) VALUES ( 5, 'Hello, World' )")
        .execute(&mut conn)
        .await?;
    static CALLED: AtomicBool = AtomicBool::new(false);
    // Using this string as a canary to ensure the callback doesn't get called with the wrong data pointer.
    let state = format!("test");
    conn.lock_handle().await?.set_preupdate_hook(move |result| {
        assert_eq!(state, "test");
        assert_eq!(result.operation, SqliteOperation::Delete);
        assert_eq!(result.database, "main");
        assert_eq!(result.table, "tweet");

        assert_eq!(4, result.get_column_count());
        assert_eq!(2, result.get_old_row_id().unwrap());
        assert_eq!(0, result.get_query_depth());
        assert_eq!(
            5,
            <i64 as Decode<Sqlite>>::decode(result.get_old_column_value(0).unwrap()).unwrap()
        );
        assert_eq!(
            "Hello, World",
            <String as Decode<Sqlite>>::decode(result.get_old_column_value(1).unwrap()).unwrap()
        );
        // out of bounds access should return an error
        assert!(result.get_old_column_value(4).is_err());
        // new values aren't available for deletes
        assert!(result.get_new_column_value(0).is_err());
        assert!(result.get_new_row_id().is_err());

        CALLED.store(true, Ordering::Relaxed);
    });

    let _ = sqlx::query("DELETE FROM tweet WHERE id = 5")
        .execute(&mut conn)
        .await?;
    assert!(CALLED.load(Ordering::Relaxed));
    Ok(())
}

#[cfg(feature = "sqlite-preupdate-hook")]
#[sqlx_macros::test]
async fn test_query_with_preupdate_hook_update() -> anyhow::Result<()> {
    use sqlx::Decode;
    use sqlx::{Value, ValueRef};

    let mut conn = new::<Sqlite>().await?;
    let _ = sqlx::query("INSERT INTO tweet ( id, text ) VALUES ( 6, 'Hello, World' )")
        .execute(&mut conn)
        .await?;
    static CALLED: AtomicBool = AtomicBool::new(false);
    let sqlite_value_stored: Arc<std::sync::Mutex<Option<_>>> = Default::default();
    // Using this string as a canary to ensure the callback doesn't get called with the wrong data pointer.
    let state = format!("test");
    conn.lock_handle().await?.set_preupdate_hook({
        let sqlite_value_stored = sqlite_value_stored.clone();
        move |result| {
            assert_eq!(state, "test");
            assert_eq!(result.operation, SqliteOperation::Update);
            assert_eq!(result.database, "main");
            assert_eq!(result.table, "tweet");

            assert_eq!(4, result.get_column_count());
            assert_eq!(4, result.get_column_count());

            assert_eq!(2, result.get_old_row_id().unwrap());
            assert_eq!(2, result.get_new_row_id().unwrap());

            assert_eq!(0, result.get_query_depth());
            assert_eq!(0, result.get_query_depth());

            assert_eq!(
                6,
                <i64 as Decode<Sqlite>>::decode(result.get_old_column_value(0).unwrap()).unwrap()
            );
            assert_eq!(
                6,
                <i64 as Decode<Sqlite>>::decode(result.get_new_column_value(0).unwrap()).unwrap()
            );

            assert_eq!(
                "Hello, World",
                <String as Decode<Sqlite>>::decode(result.get_old_column_value(1).unwrap())
                    .unwrap()
            );
            assert_eq!(
                "Hello, World2",
                <String as Decode<Sqlite>>::decode(result.get_new_column_value(1).unwrap())
                    .unwrap()
            );
            *sqlite_value_stored.lock().unwrap() =
                Some(result.get_old_column_value(0).unwrap().to_owned());

            // out of bounds access should return an error
            assert!(result.get_old_column_value(4).is_err());
            assert!(result.get_new_column_value(4).is_err());

            CALLED.store(true, Ordering::Relaxed);
        }
    });

    let _ = sqlx::query("UPDATE tweet SET text = 'Hello, World2' WHERE id = 6")
        .execute(&mut conn)
        .await?;

    assert!(CALLED.load(Ordering::Relaxed));
    conn.lock_handle().await?.remove_preupdate_hook();
    let _ = sqlx::query("DELETE FROM tweet where id = 6")
        .execute(&mut conn)
        .await?;
    // Ensure that taking an owned SqliteValue maintains a valid reference after the callback returns
    assert_eq!(
        6,
        <i64 as Decode<Sqlite>>::decode(
            sqlite_value_stored.lock().unwrap().take().unwrap().as_ref()
        )
        .unwrap()
    );
    Ok(())
}

#[cfg(feature = "sqlite-preupdate-hook")]
#[sqlx_macros::test]
async fn test_multiple_set_preupdate_hook_calls_drop_old_handler() -> anyhow::Result<()> {
    let ref_counted_object = Arc::new(0);
    assert_eq!(1, Arc::strong_count(&ref_counted_object));

    {
        let mut conn = new::<Sqlite>().await?;

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_preupdate_hook(move |_| {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_preupdate_hook(move |_| {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        let o = ref_counted_object.clone();
        conn.lock_handle().await?.set_preupdate_hook(move |_| {
            println!("{o:?}");
        });
        assert_eq!(2, Arc::strong_count(&ref_counted_object));

        conn.lock_handle().await?.remove_preupdate_hook();
    }

    assert_eq!(1, Arc::strong_count(&ref_counted_object));
    Ok(())
}

#[sqlx_macros::test]
async fn test_get_last_error() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let _ = sqlx::query("select 1").fetch_one(&mut conn).await?;

    {
        let mut handle = conn.lock_handle().await?;
        assert!(handle.last_error().is_none());
    }

    let _ = sqlx::query("invalid statement").fetch_one(&mut conn).await;

    {
        let mut handle = conn.lock_handle().await?;
        assert!(handle.last_error().is_some());
    }

    Ok(())
}

#[sqlx_macros::test]
async fn test_serialize_deserialize() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    sqlx::raw_sql("create table foo(bar integer not null, baz text not null)")
        .execute(&mut conn)
        .await?;

    sqlx::query("insert into foo(bar, baz) values (1234, 'Lorem ipsum'), (5678, 'dolor sit amet')")
        .execute(&mut conn)
        .await?;

    let serialized = conn.serialize(None).await?;

    // Close and open a new connection to ensure cleanliness.
    conn.close().await?;
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    conn.deserialize(None, serialized, false).await?;

    let rows = sqlx::query_as::<_, (i32, String)>("select bar, baz from foo")
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0].0, 1234);
    assert_eq!(rows[0].1, "Lorem ipsum");

    assert_eq!(rows[1].0, 5678);
    assert_eq!(rows[1].1, "dolor sit amet");

    Ok(())
}
#[sqlx_macros::test]
async fn test_serialize_deserialize_with_schema() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    sqlx::raw_sql(
        "attach ':memory:' as foo; create table foo.foo(bar integer not null, baz text not null)",
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        "insert into foo.foo(bar, baz) values (1234, 'Lorem ipsum'), (5678, 'dolor sit amet')",
    )
    .execute(&mut conn)
    .await?;

    let serialized = conn.serialize(Some("foo")).await?;

    // Close and open a new connection to ensure cleanliness.
    conn.close().await?;
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    // Unexpected quirk: the schema must exist before deserialization.
    sqlx::raw_sql("attach ':memory:' as foo")
        .execute(&mut conn)
        .await?;

    conn.deserialize(Some("foo"), serialized, false).await?;

    let rows = sqlx::query_as::<_, (i32, String)>("select bar, baz from foo.foo")
        .fetch_all(&mut conn)
        .await?;

    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0].0, 1234);
    assert_eq!(rows[0].1, "Lorem ipsum");

    assert_eq!(rows[1].0, 5678);
    assert_eq!(rows[1].1, "dolor sit amet");

    Ok(())
}

#[sqlx_macros::test]
async fn test_serialize_nonexistent_schema() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    let err = conn
        .serialize(Some("foobar"))
        .await
        .expect_err("an error should have been returned");

    let sqlx::Error::Database(dbe) = err else {
        panic!("expected DatabaseError: {err:?}")
    };

    assert_eq!(dbe.code().as_deref(), Some("1"));
    assert_eq!(dbe.message(), "database foobar does not exist");

    Ok(())
}

#[sqlx_macros::test]
async fn test_serialize_invalid_schema() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    let err = conn
        .serialize(Some("foo\0bar"))
        .await
        .expect_err("an error should have been returned");

    let sqlx::Error::InvalidArgument(msg) = err else {
        panic!("expected InvalidArgument: {err:?}")
    };

    assert_eq!(
        msg,
        "schema name \"foo\\0bar\" contains a zero byte at index 3"
    );

    Ok(())
}

#[sqlx_macros::test]
async fn it_can_use_transaction_options() -> anyhow::Result<()> {
    async fn check_txn_state(conn: &mut SqliteConnection, expected: SqliteTransactionState) {
        let state = transaction_state(&mut conn.lock_handle().await.unwrap());
        assert_eq!(state, expected);
    }

    let mut conn = SqliteConnectOptions::new()
        .in_memory(true)
        .connect()
        .await
        .unwrap();

    check_txn_state(&mut conn, SqliteTransactionState::None).await;

    let mut tx = conn.begin_with("BEGIN DEFERRED").await?;
    check_txn_state(&mut tx, SqliteTransactionState::None).await;
    drop(tx);

    let mut tx = conn.begin_with("BEGIN IMMEDIATE").await?;
    check_txn_state(&mut tx, SqliteTransactionState::Write).await;
    drop(tx);

    let mut tx = conn.begin_with("BEGIN EXCLUSIVE").await?;
    check_txn_state(&mut tx, SqliteTransactionState::Write).await;
    drop(tx);

    Ok(())
}

fn transaction_state(handle: &mut LockedSqliteHandle) -> SqliteTransactionState {
    use libsqlite3_sys::{sqlite3_txn_state, SQLITE_TXN_NONE, SQLITE_TXN_READ, SQLITE_TXN_WRITE};

    let unchecked_state =
        unsafe { sqlite3_txn_state(handle.as_raw_handle().as_ptr(), std::ptr::null()) };
    match unchecked_state {
        SQLITE_TXN_NONE => SqliteTransactionState::None,
        SQLITE_TXN_READ => SqliteTransactionState::Read,
        SQLITE_TXN_WRITE => SqliteTransactionState::Write,
        _ => panic!("unknown txn state: {unchecked_state}"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SqliteTransactionState {
    None,
    Read,
    Write,
}
