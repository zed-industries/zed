use std::path::Path;

use anyhow::Result;
use async_zip::base::read::stream::ZipFileReader;
use futures::{AsyncRead, io::BufReader};

pub async fn extract_zip<R: AsyncRead + Unpin>(destination: &Path, reader: R) -> Result<()> {
    let mut reader = ZipFileReader::new(BufReader::new(reader));

    let destination = &destination
        .canonicalize()
        .unwrap_or_else(|_| destination.to_path_buf());

    while let Some(mut item) = reader.next_with_entry().await? {
        let entry_reader = item.reader_mut();
        let entry = entry_reader.entry();
        let path = destination.join(entry.filename().as_str().unwrap());

        if entry.dir().unwrap() {
            std::fs::create_dir_all(&path)?;
        } else {
            let parent_dir = path.parent().expect("failed to get parent directory");
            std::fs::create_dir_all(parent_dir)?;
            let mut file = smol::fs::File::create(&path).await?;
            futures::io::copy(entry_reader, &mut file).await?;
        }

        reader = item.skip().await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use async_zip::ZipEntryBuilder;
    use async_zip::base::write::ZipFileWriter;
    use futures::AsyncWriteExt;
    use smol::io::Cursor;
    use tempfile::TempDir;

    use super::*;

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

        std::fs::write(dst.join("test"), "Hello world.").unwrap();
        std::fs::create_dir_all(dst.join("foo/bar")).unwrap();
        std::fs::write(dst.join("foo/bar.txt"), "Foo bar.").unwrap();
        std::fs::write(dst.join("foo/dar.md"), "Bar dar.").unwrap();
        std::fs::write(dst.join("foo/bar/dar你好.txt"), "你好世界").unwrap();

        dir
    }

    async fn read_archive(path: &PathBuf) -> impl AsyncRead + Unpin {
        let data = smol::fs::read(&path).await.unwrap();
        Cursor::new(data)
    }

    #[test]
    fn test_extract_zip() {
        let test_dir = make_test_data();
        let zip_file = test_dir.path().join("test.zip");

        smol::block_on(async {
            compress_zip(test_dir.path(), &zip_file).await.unwrap();
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
