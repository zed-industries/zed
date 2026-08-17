//! Demonstrates how to safely extract everything from a ZIP file.
//!
//! Extracting zip files from untrusted sources without proper sanitization
//! could be exploited by directory traversal attacks.
//! <https://en.wikipedia.org/wiki/Directory_traversal_attack#Archives>
//!
//! This example tries to minimize that risk by following the implementation from
//! Python's Standard Library.
//! <https://docs.python.org/3/library/zipfile.html#zipfile.ZipFile.extract>
//! <https://github.com/python/cpython/blob/ac0a19b62ae137c2c9f53fbba8ba3f769acf34dc/Lib/zipfile.py#L1662>
//!

use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use async_zip::base::read::seek::ZipFileReader;
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

#[tokio::main]
async fn main() {
    let archive = File::open("example.zip").await.expect("Failed to open zip file");
    let out_dir = current_dir().expect("Failed to get current working directory");
    unzip_file(archive, &out_dir).await;
}

/// Returns a relative path without reserved names, redundant separators, ".", or "..".
fn sanitize_file_path(path: &str) -> PathBuf {
    // Replaces backwards slashes
    path.replace('\\', "/")
        // Sanitizes each component
        .split('/')
        .map(sanitize_filename::sanitize)
        .collect()
}

/// Extracts everything from the ZIP archive to the output directory
async fn unzip_file(archive: File, out_dir: &Path) {
    let archive = BufReader::new(archive).compat();
    let mut reader = ZipFileReader::new(archive).await.expect("Failed to read zip file");
    for index in 0..reader.file().entries().len() {
        let entry = reader.file().entries().get(index).unwrap();
        let path = out_dir.join(sanitize_file_path(entry.filename().as_str().unwrap()));
        // If the filename of the entry ends with '/', it is treated as a directory.
        // This is implemented by previous versions of this crate and the Python Standard Library.
        // https://docs.rs/async_zip/0.0.8/src/async_zip/read/mod.rs.html#63-65
        // https://github.com/python/cpython/blob/820ef62833bd2d84a141adedd9a05998595d6b6d/Lib/zipfile.py#L528
        let entry_is_dir = entry.dir().unwrap();

        let mut entry_reader = reader.reader_without_entry(index).await.expect("Failed to read ZipEntry");

        if entry_is_dir {
            // The directory may have been created if iteration is out of order.
            if !path.exists() {
                create_dir_all(&path).await.expect("Failed to create extracted directory");
            }
        } else {
            // Creates parent directories. They may not exist if iteration is out of order
            // or the archive does not contain directory entries.
            let parent = path.parent().expect("A file entry should have parent directories");
            if !parent.is_dir() {
                create_dir_all(parent).await.expect("Failed to create parent directories");
            }
            let writer = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .await
                .expect("Failed to create extracted file");
            futures_lite::io::copy(&mut entry_reader, &mut writer.compat_write())
                .await
                .expect("Failed to copy to extracted file");

            // Closes the file and manipulates its metadata here if you wish to preserve its metadata from the archive.
        }
    }
}
