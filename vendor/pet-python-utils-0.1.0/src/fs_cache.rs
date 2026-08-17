// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use log::{error, trace};
use pet_fs::path::norm_case;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::env::ResolvedPythonEnv;

/// Represents a file path with its modification time and optional creation time.
/// Creation time (ctime) is optional because many Linux filesystems (ext4, etc.)
/// don't support file creation time, causing metadata.created() to return Err.
/// See: https://github.com/microsoft/python-environment-tools/issues/223
type FilePathWithMTimeCTime = (PathBuf, SystemTime, Option<SystemTime>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CacheEntry {
    pub environment: ResolvedPythonEnv,
    pub symlinks: Vec<FilePathWithMTimeCTime>,
}

pub fn generate_cache_file(cache_directory: &Path, executable: &Path) -> PathBuf {
    // Version 5: Relative and absolute aliases share an absolute cache identity.
    cache_directory.join(format!("{}.5.json", generate_hash(executable)))
}

pub fn delete_cache_file(cache_directory: &Path, executable: &Path) {
    let cache_file = generate_cache_file(cache_directory, executable);
    let _ = fs::remove_file(cache_file);
}

pub fn get_cache_from_file(
    cache_directory: &Path,
    executable: &Path,
) -> Option<(ResolvedPythonEnv, Vec<FilePathWithMTimeCTime>)> {
    let cache_file = generate_cache_file(cache_directory, executable);
    let file = File::open(cache_file.clone()).ok()?;
    let reader = BufReader::new(file);
    let cache: CacheEntry = serde_json::from_reader(reader).ok()?;
    let cache_key = executable_cache_key(executable);
    // Account for conflicts in the cache file. The tracked paths are stored as
    // absolute identities, so this remains valid when the caller uses a relative alias.
    if !cache.symlinks.iter().any(|symlink| symlink.0 == cache_key) {
        trace!(
            "Cache file {:?} {:?}, does not match executable {:?} (possible hash collision)",
            cache_file,
            cache.environment,
            executable
        );
        return None;
    }

    // Check if any of the exes have changed since we last cached them.
    let cache_is_valid = cache.symlinks.iter().all(|symlink| {
        if let Ok(metadata) = symlink.0.metadata() {
            let mtime_valid = metadata.modified().ok() == Some(symlink.1);
            // Only check ctime if we have it stored (may be None on Linux)
            let ctime_valid = match symlink.2 {
                Some(stored_ctime) => metadata.created().ok() == Some(stored_ctime),
                None => true, // Can't check ctime if we don't have it
            };
            mtime_valid && ctime_valid
        } else {
            // File may have been deleted.
            false
        }
    });

    if cache_is_valid {
        trace!("Using cache from {:?} for {:?}", cache_file, executable);
        Some((cache.environment, cache.symlinks))
    } else {
        let _ = fs::remove_file(cache_file);
        None
    }
}

pub fn store_cache_in_file(
    cache_directory: &Path,
    executable: &Path,
    environment: &ResolvedPythonEnv,
    symlinks_with_times: Vec<FilePathWithMTimeCTime>,
) {
    let cache_file = generate_cache_file(cache_directory, executable);
    match std::fs::create_dir_all(cache_directory) {
        Ok(_) => {
            let cache = CacheEntry {
                environment: environment.clone(),
                symlinks: symlinks_with_times,
            };
            match std::fs::File::create(cache_file.clone()) {
                Ok(file) => {
                    trace!("Caching {:?} in {:?}", executable, cache_file);
                    match serde_json::to_writer_pretty(file, &cache) {
                        Ok(_) => (),
                        Err(err) => error!("Error writing cache file {:?} {:?}", cache_file, err),
                    }
                }
                Err(err) => error!("Error creating cache file {:?} {:?}", cache_file, err),
            }
        }
        Err(err) => error!(
            "Error creating cache directory {:?} {:?}",
            cache_directory, err
        ),
    }
}

fn generate_hash(executable: &Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        executable_cache_key(executable)
            .to_string_lossy()
            .as_bytes(),
    );
    let h_bytes = hasher.finalize();
    // Convert 256 bits => Hext and then take 16 of the hex chars (that should be unique enough)
    // We will handle collisions if they happen.
    format!("{h_bytes:x}")[..16].to_string()
}

pub(crate) fn executable_cache_key(executable: &Path) -> PathBuf {
    executable_cache_key_with(executable, env::current_dir)
}

pub(crate) fn executable_cache_key_from(executable: &Path, current_dir: Option<&Path>) -> PathBuf {
    if executable.is_absolute() {
        norm_case(executable)
    } else if let Some(current_dir) = current_dir {
        norm_case(current_dir.join(executable))
    } else {
        norm_case(executable)
    }
}

fn executable_cache_key_with(
    executable: &Path,
    current_dir: impl FnOnce() -> io::Result<PathBuf>,
) -> PathBuf {
    if executable.is_absolute() {
        executable_cache_key_from(executable, None)
    } else {
        executable_cache_key_from(executable, current_dir().ok().as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    #[cfg(unix)]
    fn test_hash_generation() {
        assert_eq!(
            generate_hash(&PathBuf::from(
                "/Users/donjayamanne/demo/.venvTestInstall1/bin/python3.12"
            )),
            "e72c82125e7281e2"
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_hash_generation_upper_case() {
        assert_eq!(
            generate_hash(&PathBuf::from(
                "/Users/donjayamanne/DEMO/.venvTestInstall1/bin/python3.12"
            )),
            "ecb0ee73d6ddfe97"
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_hash_generation() {
        assert_eq!(
            generate_hash(&PathBuf::from(
                "C:\\temp\\poetry-folders\\demo-project1".to_string(),
            )),
            "c3694bfb39d7065b"
        );
    }

    #[test]
    fn absolute_cache_key_does_not_query_current_directory() {
        let current_dir_calls = AtomicUsize::new(0);
        let absolute = std::env::current_dir().unwrap().join("python");

        let key = executable_cache_key_with(&absolute, || {
            current_dir_calls.fetch_add(1, Ordering::Relaxed);
            std::env::current_dir()
        });

        assert_eq!(key, norm_case(&absolute));
        assert_eq!(current_dir_calls.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn relative_and_absolute_aliases_share_cache_file() {
        let current_dir = std::env::current_dir().unwrap();
        let relative = PathBuf::from("workspace")
            .join(".venv")
            .join("bin")
            .join("python");
        let absolute = current_dir.join(&relative);
        let cache_directory = current_dir.join("cache");

        assert_eq!(
            generate_cache_file(&cache_directory, &relative),
            generate_cache_file(&cache_directory, &absolute)
        );
    }
}
