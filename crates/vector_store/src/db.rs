use anyhow::Result;
use async_compat::{Compat, CompatExt};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::IndexedFile;

// This is saving to a local database store within the users dev zed path
// Where do we want this to sit?
// Assuming near where the workspace DB sits.
const VECTOR_DB_URL: &str = "embeddings_db";

pub struct VectorDatabase {}

impl VectorDatabase {
    pub async fn initialize_database() -> Result<()> {
        // If database doesnt exist create database
        if !Sqlite::database_exists(VECTOR_DB_URL)
            .compat()
            .await
            .unwrap_or(false)
        {
            Sqlite::create_database(VECTOR_DB_URL).compat().await?;
        }

        let db = SqlitePool::connect(VECTOR_DB_URL).compat().await?;

        // Initialize Vector Databasing Tables
        // We may be able to skip this assuming the database is never created
        // without creating the tables at the same time.
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path NVARCHAR(100) NOT NULL,
            sha1 NVARCHAR(40) NOT NULL
            )",
        )
        .execute(&db)
        .compat()
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_id INTEGER NOT NULL,
            offset INTEGER NOT NULL,
            name NVARCHAR(100) NOT NULL,
            embedding BLOB NOT NULL,
            FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
            )",
        )
        .execute(&db)
        .compat()
        .await?;

        Ok(())
    }

    pub async fn insert_file(indexed_file: IndexedFile) -> Result<()> {
        // Write to files table, and return generated id.
        let db = SqlitePool::connect(VECTOR_DB_URL).compat().await?;

        let files_insert = sqlx::query("INSERT INTO files (path, sha1) VALUES ($1, $2)")
            .bind(indexed_file.path.to_str())
            .bind(indexed_file.sha1)
            .execute(&db)
            .compat()
            .await?;

        let inserted_id = files_insert.last_insert_rowid();

        // I stole this from https://stackoverflow.com/questions/71829931/how-do-i-convert-a-negative-f32-value-to-binary-string-and-back-again
        // I imagine there is a better way to serialize to/from blob
        fn get_binary_from_values(values: Vec<f32>) -> String {
            let bits: Vec<_> = values.iter().map(|v| v.to_bits().to_string()).collect();
            bits.join(";")
        }

        fn get_values_from_binary(bin: &str) -> Vec<f32> {
            (0..bin.len() / 32)
                .map(|i| {
                    let start = i * 32;
                    let end = start + 32;
                    f32::from_bits(u32::from_str_radix(&bin[start..end], 2).unwrap())
                })
                .collect()
        }

        // Currently inserting at approximately 3400 documents a second
        // I imagine we can speed this up with a bulk insert of some kind.
        for document in indexed_file.documents {
            sqlx::query(
                "INSERT INTO documents (file_id, offset, name, embedding) VALUES ($1, $2, $3, $4)",
            )
            .bind(inserted_id)
            .bind(document.offset.to_string())
            .bind(document.name)
            .bind(get_binary_from_values(document.embedding))
            .execute(&db)
            .compat()
            .await?;
        }

        Ok(())
    }
}
