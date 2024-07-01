use anyhow::{anyhow, Result};
use heed::{
    types::{SerdeJson, Str},
    Database as HeedDatabase, EnvOpenOptions, RwTxn,
};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::SystemTime};
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedSummary {
    pub summary: String,
    pub mtime: SystemTime,
}

#[derive(Clone)]
pub struct Database {
    tx: mpsc::Sender<Box<dyn FnOnce(&HeedDatabase<Str, SerdeJson<CachedSummary>>, RwTxn) + Send>>,
}

impl Database {
    pub async fn new(db_path: &Path, root: &Path) -> Result<Self> {
        std::fs::create_dir_all(&db_path)?;
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(1024 * 1024 * 1024)
                .max_dbs(3000)
                .open(db_path)?
        };
        let mut wtxn = env.write_txn()?;
        let db_name = format!("summaries_{}", root.to_string_lossy());
        let db: HeedDatabase<Str, SerdeJson<CachedSummary>> =
            env.create_database(&mut wtxn, Some(&db_name))?;
        wtxn.commit()?;

        let (tx, mut rx) = mpsc::channel::<
            Box<dyn FnOnce(&HeedDatabase<Str, SerdeJson<CachedSummary>>, RwTxn) + Send>,
        >(100);

        tokio::spawn(async move {
            while let Some(f) = rx.recv().await {
                let wtxn = env.write_txn().unwrap();
                f(&db, wtxn);
            }
        });

        Ok(Self { tx })
    }

    pub async fn transact<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&HeedDatabase<Str, SerdeJson<CachedSummary>>, &mut RwTxn) -> Result<T>
            + Send
            + 'static,
        T: 'static + Send,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(Box::new(move |db, mut txn| {
                let result = f(db, &mut txn);
                if result.is_ok() {
                    if let Err(e) = txn.commit() {
                        let _ = tx.send(Err(anyhow::Error::from(e)));
                        return;
                    }
                }
                let _ = tx.send(result);
            }))
            .await
            .map_err(|_| anyhow!("database closed"))?;
        Ok(rx.await.map_err(|_| anyhow!("transaction failed"))??)
    }
}
