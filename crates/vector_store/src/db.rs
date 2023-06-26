use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

use rusqlite::{
    params,
    types::{FromSql, FromSqlResult, ValueRef},
    Connection,
};

use crate::IndexedFile;

// This is saving to a local database store within the users dev zed path
// Where do we want this to sit?
// Assuming near where the workspace DB sits.
pub const VECTOR_DB_URL: &str = "embeddings_db";

// Note this is not an appropriate document
#[derive(Debug)]
pub struct DocumentRecord {
    pub id: usize,
    pub file_id: usize,
    pub offset: usize,
    pub name: String,
    pub embedding: Embedding,
}

#[derive(Debug)]
pub struct FileRecord {
    pub id: usize,
    pub relative_path: String,
    pub sha1: String,
}

#[derive(Debug)]
pub struct Embedding(pub Vec<f32>);

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

pub struct VectorDatabase {
    db: rusqlite::Connection,
}

impl VectorDatabase {
    pub fn new(path: &str) -> Result<Self> {
        let this = Self {
            db: rusqlite::Connection::open(path)?,
        };
        this.initialize_database()?;
        Ok(this)
    }

    fn initialize_database(&self) -> Result<()> {
        // This will create the database if it doesnt exist

        // Initialize Vector Databasing Tables
        self.db.execute(
            "CREATE TABLE IF NOT EXISTS worktrees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                absolute_path VARCHAR NOT NULL
            );
            CREATE UNIQUE INDEX IF NOT EXISTS worktrees_absolute_path ON worktrees (absolute_path);
            ",
            [],
        )?;

        self.db.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                worktree_id INTEGER NOT NULL,
                relative_path VARCHAR NOT NULL,
                sha1 NVARCHAR(40) NOT NULL,
                FOREIGN KEY(worktree_id) REFERENCES worktrees(id) ON DELETE CASCADE
            )",
            [],
        )?;

        self.db.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_id INTEGER NOT NULL,
                offset INTEGER NOT NULL,
                name VARCHAR NOT NULL,
                embedding BLOB NOT NULL,
                FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(())
    }

    // pub async fn get_or_create_project(project_path: PathBuf) -> Result<usize> {
    //     // Check if we have the project, if we do, return the ID
    //     // If we do not have the project, insert the project and return the ID

    //     let db = rusqlite::Connection::open(VECTOR_DB_URL)?;

    //     let projects_query = db.prepare(&format!(
    //         "SELECT id FROM projects WHERE path = {}",
    //         project_path.to_str().unwrap() // This is unsafe
    //     ))?;

    //     let project_id = db.last_insert_rowid();

    //     return Ok(project_id as usize);
    // }

    pub fn insert_file(&self, indexed_file: IndexedFile) -> Result<()> {
        // Write to files table, and return generated id.
        let files_insert = self.db.execute(
            "INSERT INTO files (relative_path, sha1) VALUES (?1, ?2)",
            params![indexed_file.path.to_str(), indexed_file.sha1],
        )?;

        let inserted_id = self.db.last_insert_rowid();

        // Currently inserting at approximately 3400 documents a second
        // I imagine we can speed this up with a bulk insert of some kind.
        for document in indexed_file.documents {
            let embedding_blob = bincode::serialize(&document.embedding)?;

            self.db.execute(
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

    pub fn find_or_create_worktree(&self, worktree_root_path: &Path) -> Result<i64> {
        self.db.execute(
            "
            INSERT into worktrees (absolute_path) VALUES (?1)
            ON CONFLICT DO NOTHING
            ",
            params![worktree_root_path.to_string_lossy()],
        )?;
        Ok(self.db.last_insert_rowid())
    }

    pub fn get_file_hashes(&self, worktree_id: i64) -> Result<Vec<(PathBuf, String)>> {
        let mut statement = self
            .db
            .prepare("SELECT relative_path, sha1 FROM files ORDER BY relative_path")?;
        let mut result = Vec::new();
        for row in
            statement.query_map([], |row| Ok((row.get::<_, String>(0)?.into(), row.get(1)?)))?
        {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn get_files(&self) -> Result<HashMap<usize, FileRecord>> {
        let mut query_statement = self
            .db
            .prepare("SELECT id, relative_path, sha1 FROM files")?;
        let result_iter = query_statement.query_map([], |row| {
            Ok(FileRecord {
                id: row.get(0)?,
                relative_path: row.get(1)?,
                sha1: row.get(2)?,
            })
        })?;

        let mut pages: HashMap<usize, FileRecord> = HashMap::new();
        for result in result_iter {
            let result = result?;
            pages.insert(result.id, result);
        }

        Ok(pages)
    }

    pub fn for_each_document(
        &self,
        worktree_id: i64,
        mut f: impl FnMut(i64, Embedding),
    ) -> Result<()> {
        let mut query_statement = self.db.prepare("SELECT id, embedding FROM documents")?;
        query_statement
            .query_map(params![], |row| Ok((row.get(0)?, row.get(1)?)))?
            .filter_map(|row| row.ok())
            .for_each(|row| f(row.0, row.1));
        Ok(())
    }

    pub fn get_documents(&self) -> Result<HashMap<usize, DocumentRecord>> {
        let mut query_statement = self
            .db
            .prepare("SELECT id, file_id, offset, name, embedding FROM documents")?;
        let result_iter = query_statement.query_map([], |row| {
            Ok(DocumentRecord {
                id: row.get(0)?,
                file_id: row.get(1)?,
                offset: row.get(2)?,
                name: row.get(3)?,
                embedding: row.get(4)?,
            })
        })?;

        let mut documents: HashMap<usize, DocumentRecord> = HashMap::new();
        for result in result_iter {
            let result = result?;
            documents.insert(result.id, result);
        }

        return Ok(documents);
    }
}
