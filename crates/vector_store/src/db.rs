use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::{anyhow, Result};

use rusqlite::{
    params,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};
use sha1::{Digest, Sha1};

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
    pub sha1: FileSha1,
}

#[derive(Debug)]
pub struct FileSha1(pub Vec<u8>);

impl FileSha1 {
    pub fn from_str(content: String) -> Self {
        let mut hasher = Sha1::new();
        hasher.update(content);
        let sha1 = hasher.finalize()[..]
            .into_iter()
            .map(|val| val.to_owned())
            .collect::<Vec<u8>>();
        return FileSha1(sha1);
    }

    pub fn equals(&self, content: &String) -> bool {
        let mut hasher = Sha1::new();
        hasher.update(content);
        let sha1 = hasher.finalize()[..]
            .into_iter()
            .map(|val| val.to_owned())
            .collect::<Vec<u8>>();

        let equal = self
            .0
            .clone()
            .into_iter()
            .zip(sha1)
            .filter(|&(a, b)| a == b)
            .count()
            == self.0.len();

        equal
    }
}

impl ToSql for FileSha1 {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        return self.0.to_sql();
    }
}

impl FromSql for FileSha1 {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let bytes = value.as_blob()?;
        Ok(FileSha1(
            bytes
                .into_iter()
                .map(|val| val.to_owned())
                .collect::<Vec<u8>>(),
        ))
    }
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
        rusqlite::vtab::array::load_module(&self.db)?;

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
                sha1 BLOB NOT NULL,
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

    pub fn insert_file(&self, worktree_id: i64, indexed_file: IndexedFile) -> Result<()> {
        // Write to files table, and return generated id.
        log::info!("Inserting File!");
        self.db.execute(
            "
            DELETE FROM files WHERE worktree_id = ?1 AND relative_path = ?2;
            ",
            params![worktree_id, indexed_file.path.to_str()],
        )?;
        self.db.execute(
            "
            INSERT INTO files (worktree_id, relative_path, sha1) VALUES (?1, ?2, $3);
            ",
            params![worktree_id, indexed_file.path.to_str(), indexed_file.sha1],
        )?;

        let file_id = self.db.last_insert_rowid();

        // Currently inserting at approximately 3400 documents a second
        // I imagine we can speed this up with a bulk insert of some kind.
        for document in indexed_file.documents {
            let embedding_blob = bincode::serialize(&document.embedding)?;

            self.db.execute(
                "INSERT INTO documents (file_id, offset, name, embedding) VALUES (?1, ?2, ?3, ?4)",
                params![
                    file_id,
                    document.offset.to_string(),
                    document.name,
                    embedding_blob
                ],
            )?;
        }

        Ok(())
    }

    pub fn find_or_create_worktree(&self, worktree_root_path: &Path) -> Result<i64> {
        // Check that the absolute path doesnt exist
        let mut worktree_query = self
            .db
            .prepare("SELECT id FROM worktrees WHERE absolute_path = ?1")?;

        let worktree_id = worktree_query
            .query_row(params![worktree_root_path.to_string_lossy()], |row| {
                Ok(row.get::<_, i64>(0)?)
            })
            .map_err(|err| anyhow!(err));

        if worktree_id.is_ok() {
            return worktree_id;
        }

        // If worktree_id is Err, insert new worktree
        self.db.execute(
            "
            INSERT into worktrees (absolute_path) VALUES (?1)
            ",
            params![worktree_root_path.to_string_lossy()],
        )?;
        Ok(self.db.last_insert_rowid())
    }

    pub fn get_file_hashes(&self, worktree_id: i64) -> Result<HashMap<PathBuf, FileSha1>> {
        let mut statement = self.db.prepare(
            "SELECT relative_path, sha1 FROM files WHERE worktree_id = ?1 ORDER BY relative_path",
        )?;
        let mut result: HashMap<PathBuf, FileSha1> = HashMap::new();
        for row in statement.query_map(params![worktree_id], |row| {
            Ok((row.get::<_, String>(0)?.into(), row.get(1)?))
        })? {
            let row = row?;
            result.insert(row.0, row.1);
        }
        Ok(result)
    }

    pub fn for_each_document(
        &self,
        worktree_ids: &[i64],
        mut f: impl FnMut(i64, Embedding),
    ) -> Result<()> {
        let mut query_statement = self.db.prepare(
            "
            SELECT
                documents.id, documents.embedding
            FROM
                documents, files
            WHERE
                documents.file_id = files.id AND
                files.worktree_id IN rarray(?)
            ",
        )?;
        query_statement
            .query_map(params![ids_to_sql(worktree_ids)], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?
            .filter_map(|row| row.ok())
            .for_each(|row| f(row.0, row.1));
        Ok(())
    }

    pub fn get_documents_by_ids(&self, ids: &[i64]) -> Result<Vec<(i64, PathBuf, usize, String)>> {
        let mut statement = self.db.prepare(
            "
                SELECT
                    documents.id, files.worktree_id, files.relative_path, documents.offset, documents.name
                FROM
                    documents, files
                WHERE
                    documents.file_id = files.id AND
                    documents.id in rarray(?)
            ",
        )?;

        let result_iter = statement.query_map(params![ids_to_sql(ids)], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?.into(),
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut values_by_id = HashMap::<i64, (i64, PathBuf, usize, String)>::default();
        for row in result_iter {
            let (id, worktree_id, path, offset, name) = row?;
            values_by_id.insert(id, (worktree_id, path, offset, name));
        }

        let mut results = Vec::with_capacity(ids.len());
        for id in ids {
            let value = values_by_id
                .remove(id)
                .ok_or(anyhow!("missing document id {}", id))?;
            results.push(value);
        }

        Ok(results)
    }
}

fn ids_to_sql(ids: &[i64]) -> Rc<Vec<rusqlite::types::Value>> {
    Rc::new(
        ids.iter()
            .copied()
            .map(|v| rusqlite::types::Value::from(v))
            .collect::<Vec<_>>(),
    )
}
