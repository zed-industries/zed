use anyhow::{anyhow, Result};
use futures::Future;
use gpui::{BackgroundExecutor, Task};
use heed::{types::SerdeBincode, Database};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{path::PathBuf, time::Instant};
use ui::SharedString;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub id: PromptId,
    pub title: Option<SharedString>,
    #[serde(
        serialize_with = "serialize_instant",
        deserialize_with = "deserialize_instant"
    )]
    pub mtime: Instant,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PromptId(Uuid);

impl PromptId {
    pub fn new() -> PromptId {
        PromptId(Uuid::new_v4())
    }
}

struct PromptStore {
    executor: BackgroundExecutor,
    db_env: heed::Env,
    contents: Database<SerdeBincode<PromptId>, SerdeBincode<String>>,
    metadata: Database<SerdeBincode<PromptId>, SerdeBincode<PromptMetadata>>,
}

impl PromptStore {
    pub fn new(db_path: PathBuf, executor: BackgroundExecutor) -> Task<Result<Self>> {
        executor.spawn({
            let executor = executor.clone();
            async move {
                let db_env = unsafe {
                    heed::EnvOpenOptions::new()
                        .map_size(1024 * 1024 * 1024) // 1GB
                        .max_dbs(1)
                        .open(db_path)?
                };

                let mut txn = db_env.write_txn()?;
                let contents = db_env.create_database(&mut txn, Some("contents"))?;
                let metadata = db_env.create_database(&mut txn, Some("metadata"))?;
                txn.commit()?;
                Ok(PromptStore {
                    executor,
                    db_env,
                    contents,
                    metadata,
                })
            }
        })
    }
}

impl PromptStore {
    fn all_metadata(&self) -> Task<Result<Vec<PromptMetadata>>> {
        let env = self.db_env.clone();
        let metadata = self.metadata.clone();
        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            let iter = metadata.iter(&txn)?;
            Ok(iter
                .map(|result| Ok(result?.1))
                .collect::<Result<Vec<_>>>()?)
        })
    }

    fn load(&self, id: PromptId) -> Task<Result<String>> {
        let env = self.db_env.clone();
        let contents = self.contents;
        self.executor.spawn(async move {
            let txn = env.read_txn()?;
            Ok(contents
                .get(&txn, &id)?
                .ok_or_else(|| anyhow!("prompt not found"))?)
        })
    }

    fn save(&self, id: PromptId, markdown: String) -> impl Future<Output = Result<()>> {
        let db_connection = self.db_env.clone();
        let contents = self.contents;
        let metadata = self.metadata;

        async move {
            let mut txn = db_connection.write_txn()?;
            let title = title_from_content(&markdown);

            metadata.put(
                &mut txn,
                &id,
                &PromptMetadata {
                    id,
                    title,
                    mtime: Instant::now(),
                },
            )?;
            contents.put(&mut txn, &id, &markdown)?;

            txn.commit()?;
            Ok(())
        }
    }
}

fn title_from_content<'a>(content: &'a str) -> Option<SharedString> {
    let mut chars = content.chars().take_while(|c| *c != '\n').peekable();

    let mut level = 0;
    let mut start = 0;
    while let Some('#') = chars.peek() {
        level += 1;
        start += '#'.len_utf8();
        chars.next();
    }

    if level > 0 {
        let end = chars.map(|c| c.len_utf8()).sum();
        Some(content[start..end].trim().to_string().into())
    } else {
        None
    }
}

fn serialize_instant<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u64(instant.elapsed().as_nanos() as u64)
}

fn deserialize_instant<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: Deserializer<'de>,
{
    let nanos = u64::deserialize(deserializer)?;
    Ok(Instant::now() - std::time::Duration::from_nanos(nanos))
}
