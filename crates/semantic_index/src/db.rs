use crate::{
    parsing::{Span, SpanDigest},
    SEMANTIC_INDEX_VERSION,
};
use ai::embedding::Embedding;
use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use futures::channel::oneshot;
use gpui::BackgroundExecutor;
use ndarray::{Array1, Array2};
use ordered_float::OrderedFloat;
use project::Fs;
use rpc::proto::Timestamp;
use rusqlite::params;
use rusqlite::types::Value;
use std::{
    future::Future,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::SystemTime,
};
use util::{paths::PathMatcher, TryFutureExt};

pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices.reverse();
    indices
}

#[derive(Debug)]
pub struct FileRecord {
    pub id: usize,
    pub relative_path: String,
    pub mtime: Timestamp,
}

#[derive(Clone)]
pub struct VectorDatabase {
    path: Arc<Path>,
    transactions:
        smol::channel::Sender<Box<dyn 'static + Send + FnOnce(&mut rusqlite::Connection)>>,
}

impl VectorDatabase {
    pub async fn new(
        fs: Arc<dyn Fs>,
        path: Arc<Path>,
        executor: BackgroundExecutor,
    ) -> Result<Self> {
        if let Some(db_directory) = path.parent() {
            fs.create_dir(db_directory).await?;
        }

        let (transactions_tx, transactions_rx) = smol::channel::unbounded::<
            Box<dyn 'static + Send + FnOnce(&mut rusqlite::Connection)>,
        >();
        executor
            .spawn({
                let path = path.clone();
                async move {
                    let mut connection = rusqlite::Connection::open(&path)?;

                    connection.pragma_update(None, "journal_mode", "wal")?;
                    connection.pragma_update(None, "synchronous", "normal")?;
                    connection.pragma_update(None, "cache_size", 1000000)?;
                    connection.pragma_update(None, "temp_store", "MEMORY")?;

                    while let Ok(transaction) = transactions_rx.recv().await {
                        transaction(&mut connection);
                    }

                    anyhow::Ok(())
                }
                .log_err()
            })
            .detach();
        let this = Self {
            transactions: transactions_tx,
            path,
        };
        this.initialize_database().await?;
        Ok(this)
    }

    pub fn path(&self) -> &Arc<Path> {
        &self.path
    }

    fn transact<F, T>(&self, f: F) -> impl Future<Output = Result<T>>
    where
        F: 'static + Send + FnOnce(&rusqlite::Transaction) -> Result<T>,
        T: 'static + Send,
    {
        let (tx, rx) = oneshot::channel();
        let transactions = self.transactions.clone();
        async move {
            if transactions
                .send(Box::new(|connection| {
                    let result = connection
                        .transaction()
                        .map_err(|err| anyhow!(err))
                        .and_then(|transaction| {
                            let result = f(&transaction)?;
                            transaction.commit()?;
                            Ok(result)
                        });
                    let _ = tx.send(result);
                }))
                .await
                .is_err()
            {
                return Err(anyhow!("connection was dropped"))?;
            }
            rx.await?
        }
    }

    fn initialize_database(&self) -> impl Future<Output = Result<()>> {
        self.transact(|db| {
            rusqlite::vtab::array::load_module(&db)?;

            // Delete existing tables, if SEMANTIC_INDEX_VERSION is bumped
            let version_query = db.prepare("SELECT version from semantic_index_config");
            let version = version_query
                .and_then(|mut query| query.query_row([], |row| row.get::<_, i64>(0)));
            if version.map_or(false, |version| version == SEMANTIC_INDEX_VERSION as i64) {
                log::trace!("vector database schema up to date");
                return Ok(());
            }

            log::trace!("vector database schema out of date. updating...");
            // We renamed the `documents` table to `spans`, so we want to drop
            // `documents` without recreating it if it exists.
            db.execute("DROP TABLE IF EXISTS documents", [])
                .context("failed to drop 'documents' table")?;
            db.execute("DROP TABLE IF EXISTS spans", [])
                .context("failed to drop 'spans' table")?;
            db.execute("DROP TABLE IF EXISTS files", [])
                .context("failed to drop 'files' table")?;
            db.execute("DROP TABLE IF EXISTS worktrees", [])
                .context("failed to drop 'worktrees' table")?;
            db.execute("DROP TABLE IF EXISTS semantic_index_config", [])
                .context("failed to drop 'semantic_index_config' table")?;

            // Initialize Vector Databasing Tables
            db.execute(
                "CREATE TABLE semantic_index_config (
                    version INTEGER NOT NULL
                )",
                [],
            )?;

            db.execute(
                "INSERT INTO semantic_index_config (version) VALUES (?1)",
                params![SEMANTIC_INDEX_VERSION],
            )?;

            db.execute(
                "CREATE TABLE worktrees (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    absolute_path VARCHAR NOT NULL
                );
                CREATE UNIQUE INDEX worktrees_absolute_path ON worktrees (absolute_path);
                ",
                [],
            )?;

            db.execute(
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

            db.execute(
                "CREATE UNIQUE INDEX files_worktree_id_and_relative_path ON files (worktree_id, relative_path)",
                [],
            )?;

            db.execute(
                "CREATE TABLE spans (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    file_id INTEGER NOT NULL,
                    start_byte INTEGER NOT NULL,
                    end_byte INTEGER NOT NULL,
                    name VARCHAR NOT NULL,
                    embedding BLOB NOT NULL,
                    digest BLOB NOT NULL,
                    FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
                )",
                [],
            )?;
            db.execute(
                "CREATE INDEX spans_digest ON spans (digest)",
                [],
            )?;

            log::trace!("vector database initialized with updated schema.");
            Ok(())
        })
    }

    pub fn delete_file(
        &self,
        worktree_id: i64,
        delete_path: Arc<Path>,
    ) -> impl Future<Output = Result<()>> {
        self.transact(move |db| {
            db.execute(
                "DELETE FROM files WHERE worktree_id = ?1 AND relative_path = ?2",
                params![worktree_id, delete_path.to_str()],
            )?;
            Ok(())
        })
    }

    pub fn insert_file(
        &self,
        worktree_id: i64,
        path: Arc<Path>,
        mtime: SystemTime,
        spans: Vec<Span>,
    ) -> impl Future<Output = Result<()>> {
        self.transact(move |db| {
            // Return the existing ID, if both the file and mtime match
            let mtime = Timestamp::from(mtime);

            db.execute(
                "
                REPLACE INTO files
                (worktree_id, relative_path, mtime_seconds, mtime_nanos)
                VALUES (?1, ?2, ?3, ?4)
                ",
                params![worktree_id, path.to_str(), mtime.seconds, mtime.nanos],
            )?;

            let file_id = db.last_insert_rowid();

            let mut query = db.prepare(
                "
                INSERT INTO spans
                (file_id, start_byte, end_byte, name, embedding, digest)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ",
            )?;

            for span in spans {
                query.execute(params![
                    file_id,
                    span.range.start.to_string(),
                    span.range.end.to_string(),
                    span.name,
                    span.embedding,
                    span.digest
                ])?;
            }

            Ok(())
        })
    }

    pub fn worktree_previously_indexed(
        &self,
        worktree_root_path: &Path,
    ) -> impl Future<Output = Result<bool>> {
        let worktree_root_path = worktree_root_path.to_string_lossy().into_owned();
        self.transact(move |db| {
            let mut worktree_query =
                db.prepare("SELECT id FROM worktrees WHERE absolute_path = ?1")?;
            let worktree_id =
                worktree_query.query_row(params![worktree_root_path], |row| row.get::<_, i64>(0));

            Ok(worktree_id.is_ok())
        })
    }

    pub fn embeddings_for_digests(
        &self,
        digests: Vec<SpanDigest>,
    ) -> impl Future<Output = Result<HashMap<SpanDigest, Embedding>>> {
        self.transact(move |db| {
            let mut query = db.prepare(
                "
                SELECT digest, embedding
                FROM spans
                WHERE digest IN rarray(?)
                ",
            )?;
            let mut embeddings_by_digest = HashMap::default();
            let digests = Rc::new(
                digests
                    .into_iter()
                    .map(|digest| Value::Blob(digest.0.to_vec()))
                    .collect::<Vec<_>>(),
            );
            let rows = query.query_map(params![digests], |row| {
                Ok((row.get::<_, SpanDigest>(0)?, row.get::<_, Embedding>(1)?))
            })?;

            for (digest, embedding) in rows.flatten() {
                embeddings_by_digest.insert(digest, embedding);
            }

            Ok(embeddings_by_digest)
        })
    }

    pub fn embeddings_for_files(
        &self,
        worktree_id_file_paths: HashMap<i64, Vec<Arc<Path>>>,
    ) -> impl Future<Output = Result<HashMap<SpanDigest, Embedding>>> {
        self.transact(move |db| {
            let mut query = db.prepare(
                "
                SELECT digest, embedding
                FROM spans
                LEFT JOIN files ON files.id = spans.file_id
                WHERE files.worktree_id = ? AND files.relative_path IN rarray(?)
            ",
            )?;
            let mut embeddings_by_digest = HashMap::default();
            for (worktree_id, file_paths) in worktree_id_file_paths {
                let file_paths = Rc::new(
                    file_paths
                        .into_iter()
                        .map(|p| Value::Text(p.to_string_lossy().into_owned()))
                        .collect::<Vec<_>>(),
                );
                let rows = query.query_map(params![worktree_id, file_paths], |row| {
                    Ok((row.get::<_, SpanDigest>(0)?, row.get::<_, Embedding>(1)?))
                })?;

                for (digest, embedding) in rows.flatten() {
                    embeddings_by_digest.insert(digest, embedding);
                }
            }

            Ok(embeddings_by_digest)
        })
    }

    pub fn find_or_create_worktree(
        &self,
        worktree_root_path: Arc<Path>,
    ) -> impl Future<Output = Result<i64>> {
        self.transact(move |db| {
            let mut worktree_query =
                db.prepare("SELECT id FROM worktrees WHERE absolute_path = ?1")?;
            let worktree_id = worktree_query
                .query_row(params![worktree_root_path.to_string_lossy()], |row| {
                    row.get::<_, i64>(0)
                });

            if worktree_id.is_ok() {
                return Ok(worktree_id?);
            }

            // If worktree_id is Err, insert new worktree
            db.execute(
                "INSERT into worktrees (absolute_path) VALUES (?1)",
                params![worktree_root_path.to_string_lossy()],
            )?;
            Ok(db.last_insert_rowid())
        })
    }

    pub fn get_file_mtimes(
        &self,
        worktree_id: i64,
    ) -> impl Future<Output = Result<HashMap<PathBuf, SystemTime>>> {
        self.transact(move |db| {
            let mut statement = db.prepare(
                "
                SELECT relative_path, mtime_seconds, mtime_nanos
                FROM files
                WHERE worktree_id = ?1
                ORDER BY relative_path",
            )?;
            let mut result: HashMap<PathBuf, SystemTime> = HashMap::default();
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
        })
    }

    pub fn top_k_search(
        &self,
        query_embedding: &Embedding,
        limit: usize,
        file_ids: &[i64],
    ) -> impl Future<Output = Result<Vec<(i64, OrderedFloat<f32>)>>> {
        let file_ids = file_ids.to_vec();
        let query = query_embedding.clone().0;
        let query = Array1::from_vec(query);
        self.transact(move |db| {
            let mut query_statement = db.prepare(
                "
                    SELECT
                        id, embedding
                    FROM
                        spans
                    WHERE
                        file_id IN rarray(?)
                    ",
            )?;

            let deserialized_rows = query_statement
                .query_map(params![ids_to_sql(&file_ids)], |row| {
                    Ok((row.get::<_, usize>(0)?, row.get::<_, Embedding>(1)?))
                })?
                .filter_map(|row| row.ok())
                .collect::<Vec<(usize, Embedding)>>();

            if deserialized_rows.len() == 0 {
                return Ok(Vec::new());
            }

            // Get Length of Embeddings Returned
            let embedding_len = deserialized_rows[0].1 .0.len();

            let batch_n = 1000;
            let mut batches = Vec::new();
            let mut batch_ids = Vec::new();
            let mut batch_embeddings: Vec<f32> = Vec::new();
            deserialized_rows.iter().for_each(|(id, embedding)| {
                batch_ids.push(id);
                batch_embeddings.extend(&embedding.0);

                if batch_ids.len() == batch_n {
                    let embeddings = std::mem::take(&mut batch_embeddings);
                    let ids = std::mem::take(&mut batch_ids);
                    let array = Array2::from_shape_vec((ids.len(), embedding_len), embeddings);
                    match array {
                        Ok(array) => {
                            batches.push((ids, array));
                        }
                        Err(err) => log::error!("Failed to deserialize to ndarray: {:?}", err),
                    }
                }
            });

            if batch_ids.len() > 0 {
                let array = Array2::from_shape_vec(
                    (batch_ids.len(), embedding_len),
                    batch_embeddings.clone(),
                );
                match array {
                    Ok(array) => {
                        batches.push((batch_ids.clone(), array));
                    }
                    Err(err) => log::error!("Failed to deserialize to ndarray: {:?}", err),
                }
            }

            let mut ids: Vec<usize> = Vec::new();
            let mut results = Vec::new();
            for (batch_ids, array) in batches {
                let scores = array
                    .dot(&query.t())
                    .to_vec()
                    .iter()
                    .map(|score| OrderedFloat(*score))
                    .collect::<Vec<OrderedFloat<f32>>>();
                results.extend(scores);
                ids.extend(batch_ids);
            }

            let sorted_idx = argsort(&results);
            let mut sorted_results = Vec::new();
            let last_idx = limit.min(sorted_idx.len());
            for idx in &sorted_idx[0..last_idx] {
                sorted_results.push((ids[*idx] as i64, results[*idx]))
            }

            Ok(sorted_results)
        })
    }

    pub fn retrieve_included_file_ids(
        &self,
        worktree_ids: &[i64],
        includes: &[PathMatcher],
        excludes: &[PathMatcher],
    ) -> impl Future<Output = Result<Vec<i64>>> {
        let worktree_ids = worktree_ids.to_vec();
        let includes = includes.to_vec();
        let excludes = excludes.to_vec();
        self.transact(move |db| {
            let mut file_query = db.prepare(
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
            let mut rows = file_query.query([ids_to_sql(&worktree_ids)])?;

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

            anyhow::Ok(file_ids)
        })
    }

    pub fn spans_for_ids(
        &self,
        ids: &[i64],
    ) -> impl Future<Output = Result<Vec<(i64, PathBuf, Range<usize>)>>> {
        let ids = ids.to_vec();
        self.transact(move |db| {
            let mut statement = db.prepare(
                "
                    SELECT
                        spans.id,
                        files.worktree_id,
                        files.relative_path,
                        spans.start_byte,
                        spans.end_byte
                    FROM
                        spans, files
                    WHERE
                        spans.file_id = files.id AND
                        spans.id in rarray(?)
                ",
            )?;

            let result_iter = statement.query_map(params![ids_to_sql(&ids)], |row| {
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
            for id in &ids {
                let value = values_by_id
                    .remove(id)
                    .ok_or(anyhow!("missing span id {}", id))?;
                results.push(value);
            }

            Ok(results)
        })
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
