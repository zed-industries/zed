use anyhow::{Context as _, Result};
use arrayvec::ArrayString;
use completion::CompletionProvider;
use futures::stream::StreamExt;
use futures_batch::ChunksTimeoutStreamExt;
use gpui::{AppContext, Task};
use heed::types::{SerdeBincode, Str};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, Role};
use log;
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{
    future::Future,
    path::Path,
    sync::Arc,
    time::{Duration, SystemTime},
};

/// This model should be good for summarizing code - fast, low price, and good at outputting English.
///
/// It's called "preferred" because if the model isn't available (e.g. due to lacking the necessary API key),
/// we fall back on the global CompletionProvider's selected model.
const PREFERRED_SUMMARIZATION_MODEL: LanguageModel =
    LanguageModel::OpenAi(open_ai::Model::FourOmniMini);

#[derive(Debug, Serialize, Deserialize)]
struct UnsummarizedFile {
    // BLAKE3 hash of the source file's contents
    content_hash: String,
    // The source file's contents
    contents: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SummarizedFile {
    // BLAKE3 hash of the source file's contents
    content_hash: String,
    // The LLM's summary of the file's contents
    summary: String,
}

/// This is what blake3's to_hex() method returns - see https://docs.rs/blake3/1.5.3/src/blake3/lib.rs.html#246
type Blake3Digest = ArrayString<{ blake3::OUT_LEN * 2 }>;

#[derive(Debug, Serialize, Deserialize)]
struct FileDigest {
    path: Arc<Path>,
    mtime: Option<SystemTime>,
    digest: Blake3Digest,
}

struct SummarizeFiles {
    files: channel::Receiver<SummarizedFile>,
    task: Task<Result<()>>,
}

struct NeedsSummary {
    files: channel::Receiver<UnsummarizedFile>,
    task: Task<Result<()>>,
}

pub struct SummaryIndex {
    db_connection: heed::Env,
    file_digest_db: heed::Database<Str, SerdeBincode<FileDigest>>, // Key: file path. Val: BLAKE3 digest of its contents.
    summary_db: heed::Database<Str, Str>, // Key: BLAKE3 digest of a file's contents. Val: LLM summary of those contents.
}

impl SummaryIndex {
    pub fn new(
        db_connection: heed::Env,
        file_digest_db: heed::Database<Str, SerdeBincode<FileDigest>>,
        summary_db: heed::Database<Str, Str>,
    ) -> Self {
        Self {
            db_connection,
            file_digest_db,
            summary_db,
        }
    }

    fn check_summary_cache(
        &self,
        mut might_need_summary: channel::Receiver<UnsummarizedFile>,
        cx: &AppContext,
    ) -> NeedsSummary {
        let db_connection = self.db_connection.clone();
        let db = self.summary_db;
        let (needs_summary_tx, needs_summary_rx) = channel::bounded(512);
        let task = cx.background_executor().spawn(async move {
            while let Some(file) = might_need_summary.next().await {
                let tx = db_connection
                    .read_txn()
                    .context("Failed to create read transaction for checking which hashes are in summary cache")?;

                match db.get(&tx, &file.content_hash) {
                    Ok(opt_answer) => {
                        if opt_answer.is_none() {
                            // It's not in the summary cache db, so we need to summarize it.
                            log::debug!("{:?} was NOT in the db cache and needs to be resummarized.", &file.content_hash);
                            needs_summary_tx.send(file).await?;
                        } else {
                            log::debug!("{:?} was in the db cache and does not need to be resummarized.", &file.content_hash);
                        }
                    }
                    Err(err) => {
                        log::error!("Reading from the summaries database failed: {:?}", err);
                    }
                }
            }

            Ok(())
        });

        NeedsSummary {
            files: needs_summary_rx,
            task,
        }
    }

    fn summarize_files(
        &self,
        mut unsummarized_files: channel::Receiver<UnsummarizedFile>,
        cx: &AppContext,
    ) -> SummarizeFiles {
        let (summarized_tx, summarized_rx) = channel::bounded(512);
        let task = cx.spawn(|cx| async move {
            while let Some(file) = unsummarized_files.next().await {
                log::debug!("Summarizing {:?}", file);
                let summary = cx.update(|cx| Self::summarize_code(&file.contents, cx))?;

                summarized_tx
                    .send(SummarizedFile {
                        content_hash: file.content_hash,
                        summary: summary.await?,
                    })
                    .await?
            }

            Ok(())
        });

        SummarizeFiles {
            files: summarized_rx,
            task,
        }
    }

    fn summarize_code(code: &str, cx: &AppContext) -> impl Future<Output = Result<String>> {
        let start = std::time::Instant::now();
        let provider = CompletionProvider::global(cx);
        let model = PREFERRED_SUMMARIZATION_MODEL;
        const PROMPT_BEFORE_CODE: &str = "Summarize this code in 3 sentences, using no newlines or bullet points in the summary:";
        let prompt = format!("{PROMPT_BEFORE_CODE}\n{code}");

        log::debug!(
            "Summarizing code by sending this prompt to {:?}: {:?}",
            &model,
            &prompt
        );

        let request = LanguageModelRequest {
            model,
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: prompt,
            }],
            stop: Vec::new(),
            temperature: 1.0,
        };

        let response = provider.stream_completion_bg(request, cx);

        cx.background_executor().spawn(async move {
            let mut chunks = response.await?;
            let mut answer = String::new();

            while let Some(chunk) = chunks.next().await {
                answer.push_str(chunk?.as_str());
            }

            log::info!("Code summarization took {:?}", start.elapsed());
            Ok(answer)
        })
    }

    fn persist_summaries(
        &self,
        summaries: channel::Receiver<SummarizedFile>,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let db_connection = self.db_connection.clone();
        let db = self.summary_db;
        cx.background_executor().spawn(async move {
            let mut summaries = summaries.chunks_timeout(4096, Duration::from_secs(2));
            while let Some(summaries) = summaries.next().await {
                let mut txn = db_connection.write_txn()?;
                for file in &summaries {
                    log::debug!(
                        "Saving {} bytes of summary for content hash {:?}",
                        file.summary.len(),
                        file.content_hash
                    );
                    db.put(&mut txn, &file.content_hash, &file.summary)?;
                }
                txn.commit()?;

                drop(summaries);
                log::debug!("committed summaries");
            }

            Ok(())
        })
    }
}
