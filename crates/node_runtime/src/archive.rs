use std::path::Path;

use anyhow::Result;
use async_compression::futures::bufread::{GzipDecoder, GzipEncoder};
use async_tar::Archive;
use async_zip::{
    base::{read::stream::ZipFileReader, write::ZipFileWriter},
    ZipEntryBuilder,
};
use futures::{io::BufReader, AsyncRead};
use smol::io::AsyncWriteExt;

#[allow(unused)]
pub async fn extract_gz<R: AsyncRead + Unpin>(dst: &Path, reader: R) -> Result<()> {
    let decompressed_bytes = GzipDecoder::new(BufReader::new(reader));
    let mut file = smol::fs::File::create(dst).await?;
    futures::io::copy(decompressed_bytes, &mut file).await?;

    Ok(())
}

#[allow(unused)]
async fn compress_gz(src: &Path, dst: &Path) -> Result<()> {
    let file = smol::fs::File::open(src).await?;
    let compressed_bytes = GzipEncoder::new(BufReader::new(file));
    let mut out = smol::fs::File::create(dst).await?;
    futures::io::copy(compressed_bytes, &mut out).await?;

    Ok(())
}

pub async fn extract_tar_gz<R: AsyncRead + Unpin>(dst: &Path, reader: R) -> Result<()> {
    let decompressed_bytes = GzipDecoder::new(BufReader::new(reader));
    let archive = Archive::new(decompressed_bytes);
    archive.unpack(dst).await?;

    Ok(())
}

#[allow(dead_code)]
async fn compress_tar_gz(src_dir: &Path, dst: &Path) -> Result<()> {
    let mut builder = async_tar::Builder::new(Vec::new());
    for entry in walkdir::WalkDir::new(src_dir) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src_dir)?;

        if path.is_dir() {
            builder.append_dir_all(&relative_path, &path).await?;
        } else {
            builder.append_path_with_name(&path, &relative_path).await?;
        }
    }

    let tar = builder.into_inner().await?;
    let compressed_bytes = GzipEncoder::new(tar.as_slice());
    let mut out = smol::fs::File::create(dst).await?;
    futures::io::copy(compressed_bytes, &mut out).await?;

    Ok(())
}

pub async fn extract_zip<R: AsyncRead + Unpin>(dst: &Path, reader: R) -> Result<()> {
    let mut reader = ZipFileReader::new(BufReader::new(reader));

    let dst = &dst.canonicalize().unwrap_or_else(|_| dst.to_path_buf());

    while let Some(mut item) = reader.next_with_entry().await? {
        let entry_reader = item.reader_mut();
        let entry = entry_reader.entry();
        let path = dst.join(entry.filename().as_str().unwrap());

        if entry.dir().unwrap() {
            std::fs::create_dir_all(&path)?;
        } else {
            let parent_dir = path.parent().expect("failed to get parent directory");
            std::fs::create_dir_all(&parent_dir)?;
            let mut file = smol::fs::File::create(&path).await?;
            futures::io::copy(entry_reader, &mut file).await?;
        }

        reader = item.skip().await?;
    }

    Ok(())
}

#[allow(dead_code)]
async fn compress_zip(src_dir: &Path, dst: &Path) -> Result<()> {
    let mut out = smol::fs::File::create(dst).await?;
    let mut writer = ZipFileWriter::new(&mut out);

    for entry in walkdir::WalkDir::new(src_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let relative_path = path.strip_prefix(src_dir)?;
        let data = smol::fs::read(&path).await?;

        let filename = relative_path.display().to_string();
        let builder = ZipEntryBuilder::new(filename.into(), async_zip::Compression::Deflate);

        writer.write_entry_whole(builder, &data).await?;
    }

    writer.close().await?;
    out.flush().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use smol::io::Cursor;
    use tempfile::{NamedTempFile, TempDir};

    use super::*;

    #[track_caller]
    fn assert_file_content(path: &Path, content: &str) {
        assert!(path.exists(), "file not found: {:?}", path);
        let actual = std::fs::read_to_string(path).unwrap();
        assert_eq!(actual, content);
    }

    #[track_caller]
    fn make_test_data() -> TempDir {
        let dir = tempfile::tempdir().unwrap();
        let dst = dir.path();

        std::fs::write(&dst.join("test"), "Hello world.").unwrap();
        std::fs::create_dir_all(&dst.join("foo/bar")).unwrap();
        std::fs::write(&dst.join("foo/bar.txt"), "Foo bar.").unwrap();
        std::fs::write(&dst.join("foo/dar.md"), "Bar dar.").unwrap();
        std::fs::write(&dst.join("foo/bar/dar你好.txt"), "你好世界").unwrap();

        dir
    }

    async fn read_archive(path: &PathBuf) -> impl AsyncRead + Unpin {
        let data = smol::fs::read(&path).await.unwrap();
        Cursor::new(data)
    }

    #[test]
    fn test_extract_gz() {
        let test_dir = make_test_data();
        let src_file = test_dir.path().join("test");
        let gz_file = test_dir.path().join("test.gz");

        smol::block_on(async {
            compress_gz(&src_file, &gz_file).await.unwrap();

            let reader = read_archive(&gz_file).await;
            let out_file = NamedTempFile::new().unwrap();
            extract_gz(&out_file.path(), reader).await.unwrap();

            assert_file_content(&out_file.path(), "Hello world.");
        });
    }

    #[test]
    fn test_extract_tar_gz() {
        let test_dir = make_test_data();
        let tgz_file = test_dir.path().join("test.tar.gz");

        smol::block_on(async {
            compress_tar_gz(&test_dir.path(), &tgz_file).await.unwrap();
            let reader = read_archive(&tgz_file).await;

            let dir = tempfile::tempdir().unwrap();
            let dst = dir.path();
            extract_tar_gz(dst, reader).await.unwrap();

            assert_file_content(&dst.join("test"), "Hello world.");
            assert_file_content(&dst.join("foo/bar.txt"), "Foo bar.");
            assert_file_content(&dst.join("foo/dar.md"), "Bar dar.");
            assert_file_content(&dst.join("foo/bar/dar你好.txt"), "你好世界");
        });
    }

    #[test]
    fn test_extract_zip() {
        let test_dir = make_test_data();
        let zip_file = test_dir.path().join("test.zip");

        smol::block_on(async {
            compress_zip(&test_dir.path(), &zip_file).await.unwrap();
            let reader = read_archive(&zip_file).await;

            let dir = tempfile::tempdir().unwrap();
            let dst = dir.path();
            extract_zip(dst, reader).await.unwrap();

            assert_file_content(&dst.join("test"), "Hello world.");
            assert_file_content(&dst.join("foo/bar.txt"), "Foo bar.");
            assert_file_content(&dst.join("foo/dar.md"), "Bar dar.");
            assert_file_content(&dst.join("foo/bar/dar你好.txt"), "你好世界");
        });
    }
}
