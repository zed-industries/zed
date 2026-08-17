use std::ffi::OsString;
use std::fs::Metadata;
use std::io;
use std::path::{Path, PathBuf};

use crate::rt;

pub struct ReadDir {
    inner: Option<std::fs::ReadDir>,
}

pub struct DirEntry {
    pub path: PathBuf,
    pub file_name: OsString,
    pub metadata: Metadata,
}

// Filesystem operations are generally not capable of being non-blocking
// so Tokio and async-std don't bother; they just send the work to a blocking thread pool.
//
// We save on code duplication here by just implementing the same strategy ourselves
// using the runtime's `spawn_blocking()` primitive.

pub async fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let path = PathBuf::from(path.as_ref());
    rt::spawn_blocking(move || std::fs::read(path)).await
}

pub async fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let path = PathBuf::from(path.as_ref());
    rt::spawn_blocking(move || std::fs::read_to_string(path)).await
}

pub async fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = PathBuf::from(path.as_ref());
    rt::spawn_blocking(move || std::fs::create_dir_all(path)).await
}

pub async fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = PathBuf::from(path.as_ref());
    rt::spawn_blocking(move || std::fs::remove_file(path)).await
}

pub async fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = PathBuf::from(path.as_ref());
    rt::spawn_blocking(move || std::fs::remove_dir(path)).await
}

pub async fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = PathBuf::from(path.as_ref());
    rt::spawn_blocking(move || std::fs::remove_dir_all(path)).await
}

pub async fn read_dir(path: PathBuf) -> io::Result<ReadDir> {
    let read_dir = rt::spawn_blocking(move || std::fs::read_dir(path)).await?;

    Ok(ReadDir {
        inner: Some(read_dir),
    })
}

impl ReadDir {
    pub async fn next(&mut self) -> io::Result<Option<DirEntry>> {
        if let Some(mut read_dir) = self.inner.take() {
            let maybe = rt::spawn_blocking(move || {
                let entry = read_dir.next().transpose()?;

                entry
                    .map(|entry| -> io::Result<_> {
                        Ok((
                            read_dir,
                            DirEntry {
                                path: entry.path(),
                                file_name: entry.file_name(),
                                // We always want the metadata as well so might as well fetch
                                // it in the same blocking call.
                                metadata: entry.metadata()?,
                            },
                        ))
                    })
                    .transpose()
            })
            .await?;

            match maybe {
                Some((read_dir, entry)) => {
                    self.inner = Some(read_dir);
                    Ok(Some(entry))
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
