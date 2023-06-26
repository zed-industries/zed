use std::collections::HashMap;

use anyhow::{anyhow, Result};

use rusqlite::{
    params,
    types::{FromSql, FromSqlResult, ValueRef},
    Connection,
};
use util::ResultExt;

use crate::{Document, IndexedFile};

// This is saving to a local database store within the users dev zed path
// Where do we want this to sit?
// Assuming near where the workspace DB sits.
const VECTOR_DB_URL: &str = "embeddings_db";

// Note this is not an appropriate document
#[derive(Debug)]
pub struct DocumentRecord {
    id: usize,
    offset: usize,
    name: String,
    embedding: Embedding,
}

#[derive(Debug)]
struct Embedding(Vec<f32>);

impl FromSql for Embedding {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let bytes = value.as_blob()?;
        let embedding: Result<Vec<f32>, Box<bincode::ErrorKind>> = bincode::deserialize(bytes);
        if embedding.is_err() {
            return Err(rusqlite::types::FromSqlError::Other(embedding.unwrap_err()));
        }
        return Ok(Embedding(embedding.unwrap()));
    }
}

pub struct VectorDatabase {}

impl VectorDatabase {
    pub async fn initialize_database() -> Result<()> {
        // This will create the database if it doesnt exist
        let db = rusqlite::Connection::open(VECTOR_DB_URL)?;

        // Initialize Vector Databasing Tables
        db.execute(
            "CREATE TABLE IF NOT EXISTS files (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        path NVARCHAR(100) NOT NULL,
        sha1 NVARCHAR(40) NOT NULL
        )",
            [],
        )?;

        db.execute(
            "CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_id INTEGER NOT NULL,
            offset INTEGER NOT NULL,
            name NVARCHAR(100) NOT NULL,
            embedding BLOB NOT NULL,
            FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(())
    }

    pub async fn insert_file(indexed_file: IndexedFile) -> Result<()> {
        // Write to files table, and return generated id.
        let db = rusqlite::Connection::open(VECTOR_DB_URL)?;

        let files_insert = db.execute(
            "INSERT INTO files (path, sha1) VALUES (?1, ?2)",
            params![indexed_file.path.to_str(), indexed_file.sha1],
        )?;

        let inserted_id = db.last_insert_rowid();

        // Currently inserting at approximately 3400 documents a second
        // I imagine we can speed this up with a bulk insert of some kind.
        for document in indexed_file.documents {
            let embedding_blob = bincode::serialize(&document.embedding)?;

            db.execute(
                "INSERT INTO documents (file_id, offset, name, embedding) VALUES (?1, ?2, ?3, ?4)",
                params![
                    inserted_id,
                    document.offset.to_string(),
                    document.name,
                    embedding_blob
                ],
            )?;
        }

        Ok(())
    }

    pub fn get_documents(&self) -> Result<HashMap<usize, Document>> {
        // Should return a HashMap in which the key is the id, and the value is the finished document

        // Get Data from Database
        let db = rusqlite::Connection::open(VECTOR_DB_URL)?;

        fn query(db: Connection) -> rusqlite::Result<Vec<DocumentRecord>> {
            let mut query_statement =
                db.prepare("SELECT id, offset, name, embedding FROM documents LIMIT 10")?;
            let result_iter = query_statement.query_map([], |row| {
                Ok(DocumentRecord {
                    id: row.get(0)?,
                    offset: row.get(1)?,
                    name: row.get(2)?,
                    embedding: row.get(3)?,
                })
            })?;

            let mut results = vec![];
            for result in result_iter {
                results.push(result?);
            }

            return Ok(results);
        }

        let mut documents: HashMap<usize, Document> = HashMap::new();
        let result_iter = query(db);
        if result_iter.is_ok() {
            for result in result_iter.unwrap() {
                documents.insert(
                    result.id,
                    Document {
                        offset: result.offset,
                        name: result.name,
                        embedding: result.embedding.0,
                    },
                );
            }
        }

        return Ok(documents);
    }
}
