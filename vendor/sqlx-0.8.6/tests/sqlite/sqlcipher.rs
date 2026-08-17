#![cfg(sqlite_test_sqlcipher)]

use std::str::FromStr;

use sqlx::sqlite::SqliteQueryResult;
use sqlx::{query, Connection, SqliteConnection};
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions};
use tempfile::TempDir;

async fn new_db_url() -> anyhow::Result<(String, TempDir)> {
    let dir = TempDir::new()?;
    let filepath = dir.path().join("database.sqlite3");

    Ok((format!("sqlite://{}", filepath.display()), dir))
}

async fn fill_db(conn: &mut SqliteConnection) -> anyhow::Result<SqliteQueryResult> {
    conn.transaction(|tx| {
        Box::pin(async move {
            query(
                "
                CREATE TABLE Company(
                    Id INT PRIMARY KEY     NOT NULL,
                    Name           TEXT    NOT NULL,
                    Salary         REAL
                 );
                 ",
            )
            .execute(&mut **tx)
            .await?;

            query(
                r#"
                INSERT INTO Company(Id, Name, Salary)
                VALUES
                    (1, "aaa", 111),
                    (2, "bbb", 222)
                "#,
            )
            .execute(&mut **tx)
            .await
        })
    })
    .await
    .map_err(|e| e.into())
}

#[sqlx_macros::test]
async fn it_encrypts() -> anyhow::Result<()> {
    let (url, _dir) = new_db_url().await?;

    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("key", "the_password")
        .create_if_missing(true)
        .connect()
        .await?;

    fill_db(&mut conn).await?;

    // Create another connection without key, query should fail
    let mut conn = SqliteConnectOptions::from_str(&url)?.connect().await?;

    assert!(conn
        .transaction(|tx| {
            Box::pin(async move { query("SELECT * FROM Company;").fetch_all(&mut **tx).await })
        })
        .await
        .is_err());

    Ok(())
}

#[sqlx_macros::test]
async fn it_can_store_and_read_encrypted_data() -> anyhow::Result<()> {
    let (url, _dir) = new_db_url().await?;

    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("key", "the_password")
        .create_if_missing(true)
        .connect()
        .await?;

    fill_db(&mut conn).await?;

    // Create another connection with valid key
    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("key", "the_password")
        .connect()
        .await?;

    let result = conn
        .transaction(|tx| {
            Box::pin(async move { query("SELECT * FROM Company;").fetch_all(&mut **tx).await })
        })
        .await?;

    assert!(result.len() > 0);

    Ok(())
}

#[sqlx_macros::test]
async fn it_fails_if_password_is_incorrect() -> anyhow::Result<()> {
    let (url, _dir) = new_db_url().await?;

    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("key", "the_password")
        .create_if_missing(true)
        .connect()
        .await?;

    fill_db(&mut conn).await?;

    // Connection with invalid key should not allow to execute queries
    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("key", "BADBADBAD")
        .connect()
        .await?;

    assert!(conn
        .transaction(|tx| {
            Box::pin(async move { query("SELECT * FROM Company;").fetch_all(&mut **tx).await })
        })
        .await
        .is_err());

    Ok(())
}

#[sqlx_macros::test]
async fn it_honors_order_of_encryption_pragmas() -> anyhow::Result<()> {
    let (url, _dir) = new_db_url().await?;

    // Make call of cipher configuration mixed with other pragmas,
    // it should have no effect, encryption related pragmas should be
    // executed first and allow to establish valid connection
    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("cipher_kdf_algorithm", "PBKDF2_HMAC_SHA1")
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .pragma("cipher_page_size", "1024")
        .pragma("key", "the_password")
        .foreign_keys(true)
        .pragma("kdf_iter", "64000")
        .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
        .pragma("cipher_hmac_algorithm", "HMAC_SHA1")
        .create_if_missing(true)
        .connect()
        .await?;

    fill_db(&mut conn).await?;

    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("dummy", "pragma")
        // The cipher configuration set on first connection is
        // version 3 of SQLCipher, so for second it's enough to set
        // the compatibility mode.
        .pragma("cipher_compatibility", "3")
        .pragma("key", "the_password")
        .connect()
        .await?;

    let result = conn
        .transaction(|tx| {
            Box::pin(async move { query("SELECT * FROM COMPANY;").fetch_all(&mut **tx).await })
        })
        .await?;

    assert!(result.len() > 0);

    Ok(())
}

#[sqlx_macros::test]
async fn it_allows_to_rekey_the_db() -> anyhow::Result<()> {
    let (url, _dir) = new_db_url().await?;

    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("key", "the_password")
        .create_if_missing(true)
        .connect()
        .await?;

    fill_db(&mut conn).await?;

    // The 'pragma rekey' can be called at any time
    query("PRAGMA rekey = new_password;")
        .execute(&mut conn)
        .await?;

    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("dummy", "pragma")
        .pragma("key", "new_password")
        .connect()
        .await?;

    let result = conn
        .transaction(|tx| {
            Box::pin(async move { query("SELECT * FROM COMPANY;").fetch_all(&mut **tx).await })
        })
        .await?;

    assert!(result.len() > 0);

    Ok(())
}
