use ::fs::Fs;
use anyhow::Result;
use futures::Future;
use paths;
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

/// Whenever we change the format of the file summaries on disk, we should increment this.
const FILE_SUMMARIES_VERSION: u32 = 1;

/// Having all the summaries organized into a folder with their version number will make it
/// easier to clear oboslete caches, compared to having to read each individual file
/// to see what version it's using.
fn cache_dir() -> PathBuf {
    paths::temp_dir()
        .join("file_summaries")
        .join(concat!("v{FILE_SUMMARIES_VERSION}"))
}

fn cache_path_for_contents(content: impl AsRef<[u8]>) -> PathBuf {
    let mut hasher = blake3::Hasher::new();

    hasher.update(content.as_ref());

    cache_dir().join(format!("{}.json", hasher.finalize().to_hex()))
}

/// Given some source code, attempt to read its summary out of the cache. If there is no summary in the
/// cache, or it cannot be read for any reason, return None.
pub async fn get_cached_summary<
    Summary: Serialize + DeserializeOwned,
    SummarizeFuture: Future<Output = Result<Summary, Error>>,
    Error,
>(
    fs: &impl Fs,
    source_code: impl AsRef<[u8]>,
    summarize: impl FnOnce() -> SummarizeFuture,
) -> Result<Summary, Error> {
    let get_cache_file_path = || cache_path_for_contents(source_code.as_ref());
    let cache_file_path = get_cache_file_path();
    let opt_summary = fs
        .open_sync(&cache_file_path)
        .await
        .ok()
        .and_then(|reader| serde_json::from_reader(reader).ok());

    match opt_summary {
        Some(cached_summary) => Ok(cached_summary),
        None => {
            // We couldn't open the cache file (either because it wasn't there or was corrupted or something;
            // regardless, we don't have access to the cached data), so fall back on summarizing.
            let summary = summarize().await?;

            let get_json = || {
                serde_json::to_string(&summary)
                    .map_err(|serde_err| anyhow::Error::msg(serde_err.to_string()))
            };

            match get_json() {
                Ok(json) => {
                    if let Err(_) = fs.atomic_write(cache_file_path, json).await {
                        // This might have errored for a harmless reason such as the cache dir having
                        // been deleted, so try to create that dir and retry. If it fails again,
                        // log an error but don't block the user.
                        if let Err(err) = retry_cache_write(fs, get_cache_file_path, get_json).await
                        {
                            log::error!(
                                "Error attempting to write summary to cache file {}: {:?}",
                                cache_path_for_contents(source_code).display(),
                                err
                            )
                        }
                    }
                }
                Err(serde_err) => {
                    log::error!(
                        "Error trying to serialize source file contents as JSON. This is a programmer error and should never happen! {:?}",
                        serde_err
                    )
                }
            }

            Ok(summary)
        }
    }
}

async fn retry_cache_write(
    fs: &impl Fs,
    get_cache_file_path: impl FnOnce() -> PathBuf,
    get_cache_file_contents: impl FnOnce() -> Result<String>,
) -> Result<()> {
    // Maybe the cache dir didn't exist; if not, create it.
    // (It may have failed for a different reason, but the Fs trait doesn't
    // currently include that info, so for now just always try this.)
    //
    // We don't want to do this defensively before every single file write
    // to the cache, because it will be a waste of time essentially always
    // except for the very first time we ever write to this cache dir.
    fs.create_dir(&cache_dir()).await?;

    // Retry the file open, now that we've created the parent directory.
    // If it still fails even after trying that, give up and return Err.
    fs.atomic_write(get_cache_file_path(), get_cache_file_contents()?)
        .await
}
