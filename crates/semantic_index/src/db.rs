use crate::{parsing::Document, SEMANTIC_INDEX_VERSION};
use anyhow::{anyhow, Context, Result};
use project::{search::PathMatcher, Fs};
use rpc::proto::Timestamp;
use rusqlite::{
    params,
    types::{FromSql, FromSqlResult, ValueRef},
};
use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::SystemTime,
};

#[derive(Debug)]
pub struct FileRecord {
    pub id: usize,
    pub relative_path: String,
    pub mtime: Timestamp,
}

#[derive(Debug)]
struct Embedding(pub Vec<f32>);

#[derive(Debug)]
struct Sha1(pub Vec<u8>);

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

impl FromSql for Sha1 {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let bytes = value.as_blob()?;
        let sha1: Result<Vec<u8>, Box<bincode::ErrorKind>> = bincode::deserialize(bytes);
        if sha1.is_err() {
            return Err(rusqlite::types::FromSqlError::Other(sha1.unwrap_err()));
        }
        return Ok(Sha1(sha1.unwrap()));
    }
}

pub struct VectorDatabase {
    db: rusqlite::Connection,
}

impl VectorDatabase {
    pub async fn new(fs: Arc<dyn Fs>, path: Arc<PathBuf>) -> Result<Self> {
        if let Some(db_directory) = path.parent() {
            fs.create_dir(db_directory).await?;
        }

        let this = Self {
            db: rusqlite::Connection::open(path.as_path())?,
        };
        this.initialize_database()?;
        Ok(this)
    }

    fn get_existing_version(&self) -> Result<i64> {
        let mut version_query = self
            .db
            .prepare("SELECT version from semantic_index_config")?;
        version_query
            .query_row([], |row| Ok(row.get::<_, i64>(0)?))
            .map_err(|err| anyhow!("version query failed: {err}"))
    }

    fn initialize_database(&self) -> Result<()> {
        rusqlite::vtab::array::load_module(&self.db)?;

        // Delete existing tables, if SEMANTIC_INDEX_VERSION is bumped
        if self
            .get_existing_version()
            .map_or(false, |version| version == SEMANTIC_INDEX_VERSION as i64)
        {
            log::trace!("vector database schema up to date");
            return Ok(());
        }

        log::trace!("vector database schema out of date. updating...");
        self.db
            .execute("DROP TABLE IF EXISTS documents", [])
            .context("failed to drop 'documents' table")?;
        self.db
            .execute("DROP TABLE IF EXISTS files", [])
            .context("failed to drop 'files' table")?;
        self.db
            .execute("DROP TABLE IF EXISTS worktrees", [])
            .context("failed to drop 'worktrees' table")?;
        self.db
            .execute("DROP TABLE IF EXISTS semantic_index_config", [])
            .context("failed to drop 'semantic_index_config' table")?;

        // Initialize Vector Databasing Tables
        self.db.execute(
            "CREATE TABLE semantic_index_config (
                version INTEGER NOT NULL
            )",
            [],
        )?;

        self.db.execute(
            "INSERT INTO semantic_index_config (version) VALUES (?1)",
            params![SEMANTIC_INDEX_VERSION],
        )?;

        self.db.execute(
            "CREATE TABLE worktrees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                absolute_path VARCHAR NOT NULL
            );
            CREATE UNIQUE INDEX worktrees_absolute_path ON worktrees (absolute_path);
            ",
            [],
        )?;

        self.db.execute(
            "CREATE TABLE files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                worktree_id INTEGER NOT NULL,
                relative_path VARCHAR NOT NULL,
                mtime_seconds INTEGER NOT NULL,
                mtime_nanos INTEGER NOT NULL,
                FOREIGN KEY(worktree_id) REFERENCES worktrees(id) ON DELETE CASCADE
            )",
            [],
        )?;

        self.db.execute(
            "CREATE TABLE documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_id INTEGER NOT NULL,
                start_byte INTEGER NOT NULL,
                end_byte INTEGER NOT NULL,
                name VARCHAR NOT NULL,
                embedding BLOB NOT NULL,
                sha1 BLOB NOT NULL,
                FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
            )",
            [],
        )?;

        log::trace!("vector database initialized with updated schema.");
        Ok(())
    }

    pub fn delete_file(&self, worktree_id: i64, delete_path: PathBuf) -> Result<()> {
        self.db.execute(
            "DELETE FROM files WHERE worktree_id = ?1 AND relative_path = ?2",
            params![worktree_id, delete_path.to_str()],
        )?;
        Ok(())
    }

    pub fn insert_file(
        &self,
        worktree_id: i64,
        path: PathBuf,
        mtime: SystemTime,
        documents: Vec<Document>,
    ) -> Result<()> {
        // Return the existing ID, if both the file and mtime match
        let mtime = Timestamp::from(mtime);
        let mut existing_id_query = self.db.prepare("SELECT id FROM files WHERE worktree_id = ?1 AND relative_path = ?2 AND mtime_seconds = ?3 AND mtime_nanos = ?4")?;
        let existing_id = existing_id_query
            .query_row(
                params![worktree_id, path.to_str(), mtime.seconds, mtime.nanos],
                |row| Ok(row.get::<_, i64>(0)?),
            )
            .map_err(|err| anyhow!(err));
        let file_id = if existing_id.is_ok() {
            // If already exists, just return the existing id
            existing_id.unwrap()
        } else {
            // Delete Existing Row
            self.db.execute(
                "DELETE FROM files WHERE worktree_id = ?1 AND relative_path = ?2;",
                params![worktree_id, path.to_str()],
            )?;
            self.db.execute("INSERT INTO files (worktree_id, relative_path, mtime_seconds, mtime_nanos) VALUES (?1, ?2, ?3, ?4);", params![worktree_id, path.to_str(), mtime.seconds, mtime.nanos])?;
            self.db.last_insert_rowid()
        };

        // Currently inserting at approximately 3400 documents a second
        // I imagine we can speed this up with a bulk insert of some kind.
        for document in documents {
            let embedding_blob = bincode::serialize(&document.embedding)?;
            let sha_blob = bincode::serialize(&document.sha1)?;

            self.db.execute(
                "INSERT INTO documents (file_id, start_byte, end_byte, name, embedding, sha1) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    file_id,
                    document.range.start.to_string(),
                    document.range.end.to_string(),
                    document.name,
                    embedding_blob,
                    sha_blob
                ],
            )?;
        }

        Ok(())
    }

    pub fn worktree_previously_indexed(&self, worktree_root_path: &Path) -> Result<bool> {
        let mut worktree_query = self
            .db
            .prepare("SELECT id FROM worktrees WHERE absolute_path = ?1")?;
        let worktree_id = worktree_query
            .query_row(params![worktree_root_path.to_string_lossy()], |row| {
                Ok(row.get::<_, i64>(0)?)
            })
            .map_err(|err| anyhow!(err));

        if worktree_id.is_ok() {
            return Ok(true);
        } else {
            return Ok(false);
        }
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

    pub fn get_file_mtimes(&self, worktree_id: i64) -> Result<HashMap<PathBuf, SystemTime>> {
        let mut statement = self.db.prepare(
            "
            SELECT relative_path, mtime_seconds, mtime_nanos
            FROM files
            WHERE worktree_id = ?1
            ORDER BY relative_path",
        )?;
        let mut result: HashMap<PathBuf, SystemTime> = HashMap::new();
        for row in statement.query_map(params![worktree_id], |row| {
            Ok((
                row.get::<_, String>(0)?.into(),
                Timestamp {
                    seconds: row.get(1)?,
                    nanos: row.get(2)?,
                }
                .into(),
            ))
        })? {
            let row = row?;
            result.insert(row.0, row.1);
        }
        Ok(result)
    }

    pub fn top_k_search(
        &self,
        query_embedding: &Vec<f32>,
        limit: usize,
        file_ids: &[i64],
    ) -> Result<Vec<(i64, f32)>> {
        let mut results = Vec::<(i64, f32)>::with_capacity(limit + 1);
        self.for_each_document(file_ids, |id, embedding| {
            let similarity = dot(&embedding, &query_embedding);
            let ix = match results
                .binary_search_by(|(_, s)| similarity.partial_cmp(&s).unwrap_or(Ordering::Equal))
            {
                Ok(ix) => ix,
                Err(ix) => ix,
            };
            results.insert(ix, (id, similarity));
            results.truncate(limit);
        })?;

        Ok(results)
    }

    pub fn retrieve_included_file_ids(
        &self,
        worktree_ids: &[i64],
        includes: &[PathMatcher],
        excludes: &[PathMatcher],
    ) -> Result<Vec<i64>> {
        let mut file_query = self.db.prepare(
            "
            SELECT
                id, relative_path
            FROM
                files
            WHERE
                worktree_id IN rarray(?)
            ",
        )?;

        let mut file_ids = Vec::<i64>::new();
        let mut rows = file_query.query([ids_to_sql(worktree_ids)])?;

        while let Some(row) = rows.next()? {
            let file_id = row.get(0)?;
            let relative_path = row.get_ref(1)?.as_str()?;
            let included =
                includes.is_empty() || includes.iter().any(|glob| glob.is_match(relative_path));
            let excluded = excludes.iter().any(|glob| glob.is_match(relative_path));
            if included && !excluded {
                file_ids.push(file_id);
            }
        }

        Ok(file_ids)
    }

    fn for_each_document(&self, file_ids: &[i64], mut f: impl FnMut(i64, Vec<f32>)) -> Result<()> {
        let mut query_statement = self.db.prepare(
            "
            SELECT
                id, embedding
            FROM
                documents
            WHERE
                file_id IN rarray(?)
            ",
        )?;

        query_statement
            .query_map(params![ids_to_sql(&file_ids)], |row| {
                Ok((row.get(0)?, row.get::<_, Embedding>(1)?))
            })?
            .filter_map(|row| row.ok())
            .for_each(|(id, embedding)| f(id, embedding.0));
        Ok(())
    }

    pub fn get_documents_by_ids(&self, ids: &[i64]) -> Result<Vec<(i64, PathBuf, Range<usize>)>> {
        let mut statement = self.db.prepare(
            "
                SELECT
                    documents.id,
                    files.worktree_id,
                    files.relative_path,
                    documents.start_byte,
                    documents.end_byte
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
                row.get(3)?..row.get(4)?,
            ))
        })?;

        let mut values_by_id = HashMap::<i64, (i64, PathBuf, Range<usize>)>::default();
        for row in result_iter {
            let (id, worktree_id, path, range) = row?;
            values_by_id.insert(id, (worktree_id, path, range));
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

pub(crate) fn dot(vec_a: &[f32], vec_b: &[f32]) -> f32 {
    let len = vec_a.len();
    assert_eq!(len, vec_b.len());

    let mut result = 0.0;
    unsafe {
        matrixmultiply::sgemm(
            1,
            len,
            1,
            1.0,
            vec_a.as_ptr(),
            len as isize,
            1,
            vec_b.as_ptr(),
            1,
            len as isize,
            0.0,
            &mut result as *mut f32,
            1,
            1,
        );
    }
    result
}
