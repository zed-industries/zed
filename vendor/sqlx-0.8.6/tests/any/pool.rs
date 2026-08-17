use sqlx::any::{AnyConnectOptions, AnyPoolOptions};
use sqlx::Executor;
use std::sync::{
    atomic::{AtomicI32, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

#[sqlx_macros::test]
async fn pool_should_invoke_after_connect() -> anyhow::Result<()> {
    sqlx::any::install_default_drivers();

    let counter = Arc::new(AtomicUsize::new(0));

    let pool = AnyPoolOptions::new()
        .after_connect({
            let counter = counter.clone();
            move |_conn, _meta| {
                let counter = counter.clone();
                Box::pin(async move {
                    counter.fetch_add(1, Ordering::SeqCst);

                    Ok(())
                })
            }
        })
        .connect(&dotenvy::var("DATABASE_URL")?)
        .await?;

    let _ = pool.acquire().await?;
    let _ = pool.acquire().await?;
    let _ = pool.acquire().await?;
    let _ = pool.acquire().await?;

    // since connections are released asynchronously,
    // `.after_connect()` may be called more than once
    assert!(counter.load(Ordering::SeqCst) >= 1);

    Ok(())
}

// https://github.com/launchbadge/sqlx/issues/527
#[sqlx_macros::test]
async fn pool_should_be_returned_failed_transactions() -> anyhow::Result<()> {
    sqlx::any::install_default_drivers();

    let pool = AnyPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&dotenvy::var("DATABASE_URL")?)
        .await?;

    let query = "blah blah";

    let mut tx = pool.begin().await?;
    let res = sqlx::query(query).execute(&mut *tx).await;
    assert!(res.is_err());
    drop(tx);

    let mut tx = pool.begin().await?;
    let res = sqlx::query(query).execute(&mut *tx).await;
    assert!(res.is_err());
    drop(tx);

    let mut tx = pool.begin().await?;
    let res = sqlx::query(query).execute(&mut *tx).await;
    assert!(res.is_err());
    drop(tx);

    Ok(())
}

#[sqlx_macros::test]
async fn test_pool_callbacks() -> anyhow::Result<()> {
    sqlx::any::install_default_drivers();

    #[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
    struct ConnStats {
        id: i32,
        before_acquire_calls: i32,
        after_release_calls: i32,
    }

    sqlx_test::setup_if_needed();

    let conn_options: AnyConnectOptions = std::env::var("DATABASE_URL")?.parse()?;

    let current_id = AtomicI32::new(0);

    let pool = AnyPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .after_connect(move |conn, meta| {
            assert_eq!(meta.age, Duration::ZERO);
            assert_eq!(meta.idle_for, Duration::ZERO);

            let id = current_id.fetch_add(1, Ordering::AcqRel);

            Box::pin(async move {
                let statement = format!(
                    // language=SQL
                    r#"
                    CREATE TEMPORARY TABLE conn_stats(
                        id int primary key,
                        before_acquire_calls int default 0,
                        after_release_calls int default 0
                    );
                    INSERT INTO conn_stats(id) VALUES ({});
                    "#,
                    // Until we have generalized bind parameters
                    id
                );

                conn.execute(&statement[..]).await?;
                Ok(())
            })
        })
        .before_acquire(|conn, meta| {
            // `age` and `idle_for` should both be nonzero
            assert_ne!(meta.age, Duration::ZERO);
            assert_ne!(meta.idle_for, Duration::ZERO);

            Box::pin(async move {
                // MySQL and MariaDB don't support UPDATE ... RETURNING
                sqlx::query(
                    r#"
                        UPDATE conn_stats
                        SET before_acquire_calls = before_acquire_calls + 1
                    "#,
                )
                .execute(&mut *conn)
                .await?;

                let stats: ConnStats = sqlx::query_as("SELECT * FROM conn_stats")
                    .fetch_one(conn)
                    .await?;

                // For even IDs, cap by the number of before_acquire calls.
                // Ignore the check for odd IDs.
                Ok((stats.id & 1) == 1 || stats.before_acquire_calls < 3)
            })
        })
        .after_release(|conn, meta| {
            // `age` should be nonzero but `idle_for` should be zero.
            assert_ne!(meta.age, Duration::ZERO);
            assert_eq!(meta.idle_for, Duration::ZERO);

            Box::pin(async move {
                sqlx::query(
                    r#"
                        UPDATE conn_stats
                        SET after_release_calls = after_release_calls + 1
                    "#,
                )
                .execute(&mut *conn)
                .await?;

                let stats: ConnStats = sqlx::query_as("SELECT * FROM conn_stats")
                    .fetch_one(conn)
                    .await?;

                // For odd IDs, cap by the number of before_release calls.
                // Ignore the check for even IDs.
                Ok((stats.id & 1) == 0 || stats.after_release_calls < 4)
            })
        })
        // Don't establish a connection yet.
        .connect_lazy_with(conn_options);

    // Expected pattern of (id, before_acquire_calls, after_release_calls)
    let pattern = [
        // The connection pool starts empty.
        (0, 0, 0),
        (0, 1, 1),
        (0, 2, 2),
        (1, 0, 0),
        (1, 1, 1),
        (1, 2, 2),
        // We should expect one more `acquire` because the ID is odd
        (1, 3, 3),
        (2, 0, 0),
        (2, 1, 1),
        (2, 2, 2),
        (3, 0, 0),
    ];

    for (id, before_acquire_calls, after_release_calls) in pattern {
        let conn_stats: ConnStats = sqlx::query_as("SELECT * FROM conn_stats")
            .fetch_one(&pool)
            .await?;

        assert_eq!(
            conn_stats,
            ConnStats {
                id,
                before_acquire_calls,
                after_release_calls
            }
        );
    }

    pool.close().await;

    Ok(())
}

#[ignore]
#[sqlx_macros::test]
async fn test_connection_maintenance() -> anyhow::Result<()> {
    sqlx::any::install_default_drivers();
    sqlx_test::setup_if_needed();
    let conn_options: AnyConnectOptions = std::env::var("DATABASE_URL")?.parse()?;

    let last_meta = Arc::new(Mutex::new(None));
    let last_meta_ = last_meta.clone();
    let pool = AnyPoolOptions::new()
        .max_lifetime(Duration::from_millis(400))
        .min_connections(3)
        .before_acquire(move |_conn, _meta| {
            *last_meta_.lock().unwrap() = Some(_meta);
            Box::pin(async { Ok(true) })
        })
        .connect_lazy_with(conn_options);

    // Open and release 5 connections
    let conns = vec![
        pool.acquire().await?,
        pool.acquire().await?,
        pool.acquire().await?,
        pool.acquire().await?,
        pool.acquire().await?,
    ];
    assert_eq!(pool.size(), 5);
    assert_eq!(pool.num_idle(), 0);
    for mut conn in conns {
        conn.return_to_pool().await;
    }

    assert_eq!(pool.size(), 5);
    assert_eq!(pool.num_idle(), 5);

    // Wait for at least two iterations of maintenance task
    sqlx_core::rt::sleep(Duration::from_secs(1)).await;

    // Existing connections should have been closed due to max lifetime
    // and the pool should have reopened min_connections new ones.
    // One connection might be in the process of being replaced so we assert 2-3.
    assert!(
        pool.size() >= 2 && pool.size() <= 3,
        "pool.size() = {}",
        pool.size()
    );
    for _ in 0..2 {
        // Check that the connections was both acquired from the pool AND it's new
        let _ = pool.acquire().await.expect("failed to acquire connection");
        let meta = last_meta
            .lock()
            .unwrap()
            .take()
            .expect("expected a connection from the pool");
        assert!(
            meta.age < Duration::from_secs(2),
            "expected a fresh connection (age {:?})",
            meta.age
        );
    }

    Ok(())
}
