use std::iter::repeat;

#[cfg(feature = "runtime-async-std")]
use async_std::{
    fs::{self, File},
    io::{self, Read, ReadExt, Write, WriteExt},
    path::{Path, PathBuf},
    stream::StreamExt,
};
use async_tar::{Archive, ArchiveBuilder, Builder, EntryType, Header};
use filetime::FileTime;
#[cfg(feature = "runtime-tokio")]
use std::path::{Path, PathBuf};
use tempfile::{Builder as TempBuilder, TempDir};
#[cfg(feature = "runtime-tokio")]
use tokio::{
    fs::{self, File},
    io::{self, AsyncRead as Read, AsyncReadExt, AsyncWrite as Write, AsyncWriteExt},
};
#[cfg(feature = "runtime-tokio")]
use tokio_stream::StreamExt;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => panic!("{} returned {}", stringify!($e), e),
        }
    };
}

macro_rules! tar {
    ($e:expr) => {
        &include_bytes!(concat!("archives/", $e))[..]
    };
}

/// Helper to create a file and write contents, ensuring data is synced to disk.
/// This is necessary for tokio where metadata isn't updated until sync.
async fn create_file_with_contents(
    path: impl AsRef<std::path::Path>,
    contents: &[u8],
) -> std::io::Result<()> {
    let mut file = File::create(path.as_ref()).await?;
    file.write_all(contents).await?;
    file.sync_all().await?;
    Ok(())
}

mod header;

/// test that we can concatenate the simple.tar archive and extract the same entries twice when we
/// use the ignore_zeros option.
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn simple_concat() {
    let bytes = tar!("simple.tar");
    let mut archive_bytes = Vec::new();
    archive_bytes.extend(bytes);

    let original_names: Vec<String> = decode_names(Archive::new(&archive_bytes[..])).await;
    let expected: Vec<&str> = original_names.iter().map(|n| n.as_str()).collect();

    // concat two archives (with null in-between);
    archive_bytes.extend(bytes);

    // test now that when we read the archive, it stops processing at the first zero header.
    let actual = decode_names(Archive::new(&archive_bytes[..])).await;
    assert_eq!(expected, actual);

    // extend expected by itself.
    let expected: Vec<&str> = {
        let mut o = Vec::new();
        o.extend(&expected);
        o.extend(&expected);
        o
    };

    let builder = ArchiveBuilder::new(&archive_bytes[..]).set_ignore_zeros(true);
    let ar = builder.build();

    let actual = decode_names(ar).await;
    assert_eq!(expected, actual);

    async fn decode_names<R>(ar: Archive<R>) -> Vec<String>
    where
        R: Read + Unpin + Sync + Send,
    {
        let mut names = Vec::new();
        let mut entries = t!(ar.entries());

        while let Some(entry) = entries.next().await {
            let e = t!(entry);
            names.push(t!(::std::str::from_utf8(&e.path_bytes())).to_string());
        }

        names
    }
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn header_impls() {
    let ar = Archive::new(tar!("simple.tar"));
    let hn = Header::new_old();
    let hnb = hn.as_bytes();
    let mut entries = t!(ar.entries());
    while let Some(file) = entries.next().await {
        let file = t!(file);
        let h1 = file.header();
        let h1b = h1.as_bytes();
        let h2 = h1.clone();
        let h2b = h2.as_bytes();
        assert!(h1b[..] == h2b[..] && h2b[..] != hnb[..])
    }
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn header_impls_missing_last_header() {
    let ar = Archive::new(tar!("simple_missing_last_header.tar"));
    let hn = Header::new_old();
    let hnb = hn.as_bytes();
    let mut entries = t!(ar.entries());

    while let Some(file) = entries.next().await {
        let file = t!(file);
        let h1 = file.header();
        let h1b = h1.as_bytes();
        let h2 = h1.clone();
        let h2b = h2.as_bytes();
        assert!(h1b[..] == h2b[..] && h2b[..] != hnb[..])
    }
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn reading_files() {
    let rdr = tar!("reading_files.tar");
    let ar = Archive::new(rdr);
    let mut entries = t!(ar.entries());

    let mut a = t!(entries.next().await.unwrap());
    assert_eq!(&*a.header().path_bytes(), b"a");
    let mut s = String::new();
    t!(a.read_to_string(&mut s).await);
    assert_eq!(s, "a\na\na\na\na\na\na\na\na\na\na\n");

    let mut b = t!(entries.next().await.unwrap());
    assert_eq!(&*b.header().path_bytes(), b"b");
    s.truncate(0);
    t!(b.read_to_string(&mut s).await);
    assert_eq!(s, "b\nb\nb\nb\nb\nb\nb\nb\nb\nb\nb\n");

    assert!(entries.next().await.is_none());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn writing_files() {
    let mut ar = Builder::new(Vec::new());
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let path = td.path().join("test");
    t!(create_file_with_contents(&path, b"test").await);

    t!(ar
        .append_file("test2", &mut t!(File::open(&path).await))
        .await);

    let data = t!(ar.into_inner().await);
    let ar = Archive::new(&data[..]);
    let mut entries = t!(ar.entries());
    let mut f = t!(entries.next().await.unwrap());

    assert_eq!(&*f.header().path_bytes(), b"test2");
    assert_eq!(f.header().size().unwrap(), 4);
    let mut s = String::new();
    t!(f.read_to_string(&mut s).await);
    assert_eq!(s, "test");

    assert!(entries.next().await.is_none());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn large_filename() {
    let mut ar = Builder::new(Vec::new());
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let path = td.path().join("test");
    t!(create_file_with_contents(&path, b"test").await);

    let filename = "abcd/".repeat(50);
    let mut header = Header::new_ustar();
    header.set_path(&filename).unwrap();
    header.set_metadata(&t!(fs::metadata(&path).await));
    header.set_cksum();
    t!(ar.append(&header, &b"test"[..]).await);
    let too_long = "abcd".repeat(200);
    t!(ar
        .append_file(&too_long, &mut t!(File::open(&path).await))
        .await);
    t!(ar.append_data(&mut header, &too_long, &b"test"[..]).await);

    let rd = t!(ar.into_inner().await);
    let ar = Archive::new(&rd[..]);
    let mut entries = t!(ar.entries());

    // The short entry added with `append`
    let mut f = entries.next().await.unwrap().unwrap();
    assert_eq!(&*f.header().path_bytes(), filename.as_bytes());
    assert_eq!(f.header().size().unwrap(), 4);
    let mut s = String::new();
    t!(f.read_to_string(&mut s).await);
    assert_eq!(s, "test");

    // The long entry added with `append_file`
    let mut f = entries.next().await.unwrap().unwrap();
    assert_eq!(&*f.path_bytes(), too_long.as_bytes());
    assert_eq!(f.header().size().unwrap(), 4);
    let mut s = String::new();
    t!(f.read_to_string(&mut s).await);
    assert_eq!(s, "test");

    // The long entry added with `append_data`
    let mut f = entries.next().await.unwrap().unwrap();
    assert!(f.header().path_bytes().len() < too_long.len());
    assert_eq!(&*f.path_bytes(), too_long.as_bytes());
    assert_eq!(f.header().size().unwrap(), 4);
    let mut s = String::new();
    t!(f.read_to_string(&mut s).await);
    assert_eq!(s, "test");

    assert!(entries.next().await.is_none());
}

// This test checks very particular scenario where path component
// starting with ".." of a long path gets split at 100-byte mark
// so that ".." goes into header and gets interpreted as parent dir
// (and rejected) .
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn large_filename_with_dot_dot_at_100_byte_mark() {
    let mut ar = Builder::new(Vec::new());

    let mut header = Header::new_gnu();
    header.set_cksum();
    header.set_mode(0o644);
    header.set_size(4);

    let mut long_name_with_dot_dot = "tdir/".repeat(19);
    long_name_with_dot_dot.push_str("tt/..file");

    t!(ar
        .append_data(&mut header, &long_name_with_dot_dot, &b"test"[..])
        .await);

    let rd = t!(ar.into_inner().await);
    let ar = Archive::new(&rd[..]);
    let mut entries = t!(ar.entries());

    let mut f = entries.next().await.unwrap().unwrap();
    assert_eq!(&*f.path_bytes(), long_name_with_dot_dot.as_bytes());
    assert_eq!(f.header().size().unwrap(), 4);
    let mut s = String::new();
    t!(f.read_to_string(&mut s).await);
    assert_eq!(s, "test");
    assert!(entries.next().await.is_none());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn reading_entries() {
    let rdr = tar!("reading_files.tar");
    let ar = Archive::new(rdr);
    let mut entries = t!(ar.entries());
    let mut a = t!(entries.next().await.unwrap());
    assert_eq!(&*a.header().path_bytes(), b"a");
    let mut s = String::new();
    t!(a.read_to_string(&mut s).await);
    assert_eq!(s, "a\na\na\na\na\na\na\na\na\na\na\n");
    s.truncate(0);
    t!(a.read_to_string(&mut s).await);
    assert_eq!(s, "");
    let mut b = t!(entries.next().await.unwrap());

    assert_eq!(&*b.header().path_bytes(), b"b");
    s.truncate(0);
    t!(b.read_to_string(&mut s).await);
    assert_eq!(s, "b\nb\nb\nb\nb\nb\nb\nb\nb\nb\nb\n");
    assert!(entries.next().await.is_none());
}

async fn check_dirtree(td: &TempDir) {
    let dir_a = td.path().join("a");
    let dir_b = td.path().join("a/b");
    let file_c = td.path().join("a/c");
    assert!(
        fs::metadata(&dir_a)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    );
    assert!(
        fs::metadata(&dir_b)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    );
    assert!(
        fs::metadata(&file_c)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn extracting_directories() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let rdr = tar!("directory.tar");
    let ar = Archive::new(rdr);
    t!(ar.unpack(td.path()).await);
    check_dirtree(&td).await;
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(all(unix, feature = "xattr"))]
async fn xattrs() {
    // If /tmp is a tmpfs, xattr will fail
    // The xattr crate's unit tests also use /var/tmp for this reason
    let td = t!(TempBuilder::new()
        .prefix("async-tar")
        .tempdir_in("/var/tmp"));
    let rdr = tar!("xattrs.tar");
    let builder = ArchiveBuilder::new(rdr).set_unpack_xattrs(true);
    let ar = builder.build();
    t!(ar.unpack(td.path()).await);

    let val = xattr::get(td.path().join("a/b"), "user.pax.flags").unwrap();
    assert_eq!(val.unwrap(), b"epm");
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(all(unix, feature = "xattr"))]
async fn no_xattrs() {
    // If /tmp is a tmpfs, xattr will fail
    // The xattr crate's unit tests also use /var/tmp for this reason
    let td = t!(TempBuilder::new()
        .prefix("async-tar")
        .tempdir_in("/var/tmp"));
    let rdr = tar!("xattrs.tar");
    let builder = ArchiveBuilder::new(rdr).set_unpack_xattrs(false);
    let ar = builder.build();
    t!(ar.unpack(td.path()).await);

    assert_eq!(
        xattr::get(td.path().join("a/b"), "user.pax.flags").unwrap(),
        None
    );
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn writing_and_extracting_directories() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let mut ar = Builder::new(Vec::new());
    let tmppath = td.path().join("tmpfile");
    t!(create_file_with_contents(&tmppath, b"c").await);
    t!(ar.append_dir("a", ".").await);
    t!(ar.append_dir("a/b", ".").await);
    t!(ar
        .append_file("a/c", &mut t!(File::open(&tmppath).await))
        .await);
    t!(ar.finish().await);

    let rdr = t!(ar.into_inner().await);
    let ar = Archive::new(&rdr[..]);
    t!(ar.unpack(td.path()).await);
    check_dirtree(&td).await;
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn writing_directories_recursively() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let base_dir = td.path().join("base");
    t!(fs::create_dir(&base_dir).await);
    t!(t!(File::create(base_dir.join("file1")).await)
        .write_all(b"file1")
        .await);
    let sub_dir = base_dir.join("sub");
    t!(fs::create_dir(&sub_dir).await);
    t!(t!(File::create(sub_dir.join("file2")).await)
        .write_all(b"file2")
        .await);

    let mut ar = Builder::new(Vec::new());
    t!(ar.append_dir_all("foobar", base_dir).await);
    let data = t!(ar.into_inner().await);

    let ar = Archive::new(&data[..]);
    t!(ar.unpack(td.path()).await);
    let base_dir = td.path().join("foobar");
    assert!(
        fs::metadata(&base_dir)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    );
    let file1_path = base_dir.join("file1");
    assert!(
        fs::metadata(&file1_path)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
    let sub_dir = base_dir.join("sub");
    assert!(
        fs::metadata(&sub_dir)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    );
    let file2_path = sub_dir.join("file2");
    assert!(
        fs::metadata(&file2_path)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn append_dir_all_blank_dest() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let base_dir = td.path().join("base");
    t!(fs::create_dir(&base_dir).await);
    t!(t!(File::create(base_dir.join("file1")).await)
        .write_all(b"file1")
        .await);
    let sub_dir = base_dir.join("sub");
    t!(fs::create_dir(&sub_dir).await);
    t!(t!(File::create(sub_dir.join("file2")).await)
        .write_all(b"file2")
        .await);

    let mut ar = Builder::new(Vec::new());
    t!(ar.append_dir_all("", base_dir).await);
    let data = t!(ar.into_inner().await);

    let ar = Archive::new(&data[..]);
    t!(ar.unpack(td.path()).await);
    let base_dir = td.path();
    assert!(
        fs::metadata(&base_dir)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    );
    let file1_path = base_dir.join("file1");
    assert!(
        fs::metadata(&file1_path)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
    let sub_dir = base_dir.join("sub");
    assert!(
        fs::metadata(&sub_dir)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    );
    let file2_path = sub_dir.join("file2");
    assert!(
        fs::metadata(&file2_path)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn append_dir_all_does_not_work_on_non_directory() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let path = td.path().join("test");
    t!(t!(File::create(&path).await).write_all(b"test").await);

    let mut ar = Builder::new(Vec::new());
    let result = ar.append_dir_all("test", path).await;
    assert!(result.is_err());
    // Must finalize even after error (required for tokio runtime)
    t!(ar.finish().await);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn extracting_duplicate_dirs() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let rdr = tar!("duplicate_dirs.tar");
    let ar = Archive::new(rdr);
    t!(ar.unpack(td.path()).await);

    let some_dir = td.path().join("some_dir");
    assert!(
        fs::metadata(&some_dir)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    );
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn unpack_old_style_bsd_dir() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let mut ar = Builder::new(Vec::new());

    let mut header = Header::new_old();
    header.set_entry_type(EntryType::Regular);
    t!(header.set_path("testdir/"));
    header.set_size(0);
    header.set_cksum();
    t!(ar.append(&header, &mut io::empty()).await);

    // Extracting
    let rdr = t!(ar.into_inner().await);
    let ar = Archive::new(&rdr[..]);
    t!(ar.clone().unpack(td.path()).await);

    // Iterating
    let rdr = ar.into_inner().map_err(|_| ()).unwrap();
    let ar = Archive::new(rdr);
    assert!(t!(ar.entries()).all(|fr| fr.is_ok()).await);

    assert!(td.path().join("testdir").is_dir());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn handling_incorrect_file_size() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let mut ar = Builder::new(Vec::new());

    let path = td.path().join("tmpfile");
    t!(File::create(&path).await);
    let mut file = t!(File::open(&path).await);
    let mut header = Header::new_old();
    t!(header.set_path("somepath"));
    header.set_metadata(&t!(file.metadata().await));
    header.set_size(2048); // past the end of file null blocks
    header.set_cksum();
    t!(ar.append(&header, &mut file).await);

    // Extracting
    let rdr: Vec<u8> = t!(ar.into_inner().await);
    println!("extracting");
    let ar = Archive::new(&rdr[..]);
    assert!(ar.clone().unpack(td.path()).await.is_err());

    // Iterating
    let _ = ar.into_inner().map_err(|_| ()).unwrap();
    println!("iterating");
    let ar = Archive::new(&rdr[..]);
    assert!(t!(ar.entries()).any(|fr| fr.is_err()).await);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn extracting_malicious_tarball() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let mut evil_tar = Vec::new();

    {
        let mut a = Builder::new(&mut evil_tar);
        async fn append<R: Write + Unpin + Send + Sync>(a: &mut Builder<R>, path: &'static str) {
            let mut header = Header::new_gnu();
            assert!(header.set_path(path).is_err(), "was ok: {:?}", path);
            {
                let h = header.as_gnu_mut().unwrap();
                for (a, b) in h.name.iter_mut().zip(path.as_bytes()) {
                    *a = *b;
                }
            }
            header.set_size(1);
            header.set_cksum();
            t!(a.append(&header, io::repeat(1).take(1)).await);
        }

        append(&mut a, "/tmp/abs_evil.txt").await;
        append(&mut a, "//tmp/abs_evil2.txt").await;
        append(&mut a, "///tmp/abs_evil3.txt").await;
        append(&mut a, "/./tmp/abs_evil4.txt").await;
        append(&mut a, "//./tmp/abs_evil5.txt").await;
        append(&mut a, "///./tmp/abs_evil6.txt").await;
        append(&mut a, "/../tmp/rel_evil.txt").await;
        append(&mut a, "../rel_evil2.txt").await;
        append(&mut a, "./../rel_evil3.txt").await;
        append(&mut a, "some/../../rel_evil4.txt").await;
        append(&mut a, "").await;
        append(&mut a, "././//./..").await;
        append(&mut a, "..").await;
        append(&mut a, "/////////..").await;
        append(&mut a, "/////////").await;
        t!(a.finish().await);
    }

    let ar = Archive::new(&evil_tar[..]);
    t!(ar.unpack(td.path()).await);

    assert!(fs::metadata("/tmp/abs_evil.txt").await.is_err());
    assert!(fs::metadata("/tmp/abs_evil.txt2").await.is_err());
    assert!(fs::metadata("/tmp/abs_evil.txt3").await.is_err());
    assert!(fs::metadata("/tmp/abs_evil.txt4").await.is_err());
    assert!(fs::metadata("/tmp/abs_evil.txt5").await.is_err());
    assert!(fs::metadata("/tmp/abs_evil.txt6").await.is_err());
    assert!(fs::metadata("/tmp/rel_evil.txt").await.is_err());
    assert!(fs::metadata("/tmp/rel_evil.txt").await.is_err());
    assert!(
        fs::metadata(td.path().join("../tmp/rel_evil.txt"))
            .await
            .is_err()
    );
    assert!(
        fs::metadata(td.path().join("../rel_evil2.txt"))
            .await
            .is_err()
    );
    assert!(
        fs::metadata(td.path().join("../rel_evil3.txt"))
            .await
            .is_err()
    );
    assert!(
        fs::metadata(td.path().join("../rel_evil4.txt"))
            .await
            .is_err()
    );

    // The `some` subdirectory should not be created because the only
    // filename that references this has '..'.
    assert!(fs::metadata(td.path().join("some")).await.is_err());

    // The `tmp` subdirectory should be created and within this
    // subdirectory, there should be files named `abs_evil.txt` through
    // `abs_evil6.txt`.
    let tmp_root = td.path().join("tmp");

    assert!(
        fs::metadata(&tmp_root)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    );

    let mut entries = fs::read_dir(&tmp_root).await.unwrap();
    #[cfg(feature = "runtime-async-std")]
    {
        while let Some(entry) = entries.next().await {
            let entry = entry.unwrap();
            println!("- {:?}", entry.file_name());
        }
    }
    #[cfg(feature = "runtime-tokio")]
    {
        while let Some(entry) = entries.next_entry().await.unwrap() {
            println!("- {:?}", entry.file_name());
        }
    }

    assert!(
        fs::metadata(tmp_root.join("abs_evil.txt"))
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );

    // not present due to // being interpreted differently on windows
    #[cfg(not(target_os = "windows"))]
    assert!(
        fs::metadata(tmp_root.join("abs_evil2.txt"))
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
    assert!(
        fs::metadata(tmp_root.join("abs_evil3.txt"))
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
    assert!(
        fs::metadata(tmp_root.join("abs_evil4.txt"))
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );

    // not present due to // being interpreted differently on windows
    #[cfg(not(target_os = "windows"))]
    assert!(
        fs::metadata(tmp_root.join("abs_evil5.txt"))
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
    assert!(
        fs::metadata(tmp_root.join("abs_evil6.txt"))
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
    );
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn octal_spaces() {
    let rdr = tar!("spaces.tar");
    let ar = Archive::new(rdr);

    let entry = ar.entries().unwrap().next().await.unwrap().unwrap();
    assert_eq!(entry.header().mode().unwrap() & 0o777, 0o777);
    assert_eq!(entry.header().uid().unwrap(), 0);
    assert_eq!(entry.header().gid().unwrap(), 0);
    assert_eq!(entry.header().size().unwrap(), 2);
    assert_eq!(entry.header().mtime().unwrap(), 0o12_440_016_664);
    assert_eq!(entry.header().cksum().unwrap(), 0o4253);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn extracting_malformed_tar_null_blocks() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let mut ar = Builder::new(Vec::new());

    let path1 = td.path().join("tmpfile1");
    let path2 = td.path().join("tmpfile2");
    t!(File::create(&path1).await);
    t!(File::create(&path2).await);
    t!(ar
        .append_file("tmpfile1", &mut t!(File::open(&path1).await))
        .await);
    let mut data = t!(ar.into_inner().await);
    let amt = data.len();
    data.truncate(amt - 512);
    let mut ar = Builder::new(data);
    t!(ar
        .append_file("tmpfile2", &mut t!(File::open(&path2).await))
        .await);
    t!(ar.finish().await);

    let data = t!(ar.into_inner().await);
    let ar = Archive::new(&data[..]);
    assert!(ar.unpack(td.path()).await.is_ok());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn empty_filename() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let rdr = tar!("empty_filename.tar");
    let ar = Archive::new(rdr);
    assert!(ar.unpack(td.path()).await.is_ok());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn file_times() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let rdr = tar!("file_times.tar");
    let ar = Archive::new(rdr);
    t!(ar.unpack(td.path()).await);

    let meta = fs::metadata(td.path().join("a")).await.unwrap();
    let mtime = FileTime::from_last_modification_time(&meta);
    let atime = FileTime::from_last_access_time(&meta);
    assert_eq!(mtime.unix_seconds(), 1_000_000_000);
    assert_eq!(mtime.nanoseconds(), 0);
    assert_eq!(atime.unix_seconds(), 1_000_000_000);
    assert_eq!(atime.nanoseconds(), 0);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn backslash_treated_well() {
    // Insert a file into an archive with a backslash
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let mut ar = Builder::new(Vec::<u8>::new());
    t!(ar.append_dir("foo\\bar", td.path()).await);
    let data = t!(ar.into_inner().await);
    let ar = Archive::new(&data[..]);
    let mut entries = t!(ar.entries());
    let f = t!(entries.next().await.unwrap());
    if cfg!(unix) {
        assert_eq!(t!(f.header().path()).to_str(), Some("foo\\bar"));
    } else {
        assert_eq!(t!(f.header().path()).to_str(), Some("foo/bar"));
    }

    // Unpack an archive with a backslash in the name
    let mut ar = Builder::new(Vec::<u8>::new());
    let mut header = Header::new_gnu();
    header.set_metadata(&t!(fs::metadata(td.path()).await));
    header.set_size(0);
    for (a, b) in header.as_old_mut().name.iter_mut().zip(b"foo\\bar\x00") {
        *a = *b;
    }
    header.set_cksum();
    t!(ar.append(&header, &mut io::empty()).await);
    let data = t!(ar.into_inner().await);
    let ar = Archive::new(&data[..]);
    let f = t!(t!(ar.entries()).next().await.unwrap());
    assert_eq!(t!(f.header().path()).to_str(), Some("foo\\bar"));

    let ar = Archive::new(&data[..]);
    t!(ar.unpack(td.path()).await);
    assert!(fs::metadata(td.path().join("foo\\bar")).await.is_ok());
}

#[cfg(unix)]
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn nul_bytes_in_path() {
    use std::{ffi::OsStr, os::unix::prelude::*};

    let nul_path = OsStr::from_bytes(b"foo\0");
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let mut ar = Builder::new(Vec::<u8>::new());
    let err = ar.append_dir(nul_path, td.path()).await.unwrap_err();
    assert!(err.to_string().contains("contains a nul byte"));
    // Must finalize even after error (required for tokio runtime)
    t!(ar.finish().await);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn links() {
    let ar = Archive::new(tar!("link.tar"));
    let mut entries = t!(ar.entries());
    let link = t!(entries.next().await.unwrap());
    assert_eq!(
        t!(link.header().link_name()).as_ref().map(|p| &**p),
        Some(Path::new("file"))
    );
    let other = t!(entries.next().await.unwrap());
    assert!(t!(other.header().link_name()).is_none());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(unix)] // making symlinks on windows is hard
async fn unpack_links() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let ar = Archive::new(tar!("link.tar"));
    t!(ar.unpack(td.path()).await);

    let md = t!(fs::symlink_metadata(td.path().join("lnk")).await);
    assert!(md.file_type().is_symlink());
    assert_eq!(
        &*t!(fs::read_link(td.path().join("lnk")).await),
        Path::new("file")
    );
    t!(File::open(td.path().join("lnk")).await);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn pax_simple() {
    let ar = Archive::new(tar!("pax.tar"));
    let mut entries = t!(ar.entries());

    let mut first = t!(entries.next().await.unwrap());
    let mut attributes = t!(first.pax_extensions().await).unwrap();
    let first = t!(attributes.next().unwrap());
    let second = t!(attributes.next().unwrap());
    let third = t!(attributes.next().unwrap());
    assert!(attributes.next().is_none());

    assert_eq!(first.key(), Ok("mtime"));
    assert_eq!(first.value(), Ok("1453146164.953123768"));
    assert_eq!(second.key(), Ok("atime"));
    assert_eq!(second.value(), Ok("1453251915.24892486"));
    assert_eq!(third.key(), Ok("ctime"));
    assert_eq!(third.value(), Ok("1453146164.953123768"));
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn pax_path() {
    let ar = Archive::new(tar!("pax2.tar"));
    let mut entries = t!(ar.entries());

    let first = t!(entries.next().await.unwrap());
    assert!(first.path().unwrap().ends_with("aaaaaaaaaaaaaaa"));
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn pax_precedence() {
    let ar = Archive::new(tar!("pax-header-precedence.tar"));
    let mut entries = t!(ar.entries());

    let first = t!(entries.next().await.unwrap());
    assert!(first.path().unwrap().ends_with("normal.txt"));

    let second = t!(entries.next().await.unwrap());
    assert!(second.path().unwrap().ends_with("blob.bin"));

    let third = t!(entries.next().await.unwrap());
    assert!(third.path().unwrap().ends_with("marker.txt"));

    assert!(entries.next().await.is_none());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn long_name_trailing_nul() {
    let mut b = Builder::new(Vec::<u8>::new());

    let mut h = Header::new_gnu();
    t!(h.set_path("././@LongLink"));
    h.set_size(4);
    h.set_entry_type(EntryType::new(b'L'));
    h.set_cksum();
    t!(b.append(&h, "foo\0".as_bytes()).await);
    let mut h = Header::new_gnu();

    t!(h.set_path("bar"));
    h.set_size(6);
    h.set_entry_type(EntryType::file());
    h.set_cksum();
    t!(b.append(&h, b"foobar" as &[u8]).await);

    let contents = t!(b.into_inner().await);
    let a = Archive::new(&contents[..]);

    let e = t!(t!(a.entries()).next().await.unwrap());
    assert_eq!(&*e.path_bytes(), b"foo");
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn long_linkname_trailing_nul() {
    let mut b = Builder::new(Vec::<u8>::new());

    let mut h = Header::new_gnu();
    t!(h.set_path("././@LongLink"));
    h.set_size(4);
    h.set_entry_type(EntryType::new(b'K'));
    h.set_cksum();
    t!(b.append(&h, "foo\0".as_bytes()).await);
    let mut h = Header::new_gnu();

    t!(h.set_path("bar"));
    h.set_size(6);
    h.set_entry_type(EntryType::file());
    h.set_cksum();
    t!(b.append(&h, b"foobar" as &[u8]).await);

    let contents = t!(b.into_inner().await);
    let a = Archive::new(&contents[..]);

    let e = t!(t!(a.entries()).next().await.unwrap());
    assert_eq!(&*e.link_name_bytes().unwrap(), b"foo");
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn encoded_long_name_has_trailing_nul() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let path = td.path().join("foo");
    t!(create_file_with_contents(&path, b"test").await);

    let mut b = Builder::new(Vec::<u8>::new());
    let long = "abcd".repeat(200);

    t!(b.append_file(&long, &mut t!(File::open(&path).await)).await);

    let contents = t!(b.into_inner().await);
    let a = Archive::new(&contents[..]);

    let mut e = t!(t!(a.entries_raw()).next().await.unwrap());
    let mut name = Vec::new();
    t!(e.read_to_end(&mut name).await);
    assert_eq!(name[name.len() - 1], 0);

    let header_name = &e.header().as_gnu().unwrap().name;
    assert!(header_name.starts_with(b"././@LongLink\x00"));
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn reading_sparse() {
    let rdr = tar!("sparse.tar");
    let ar = Archive::new(rdr);
    let mut entries = t!(ar.entries());

    let mut a = t!(entries.next().await.unwrap());
    let mut s = String::new();
    assert_eq!(&*a.header().path_bytes(), b"sparse_begin.txt");
    t!(a.read_to_string(&mut s).await);
    assert_eq!(&s[..5], "test\n");
    assert!(s[5..].chars().all(|x| x == '\u{0}'));

    let mut a = t!(entries.next().await.unwrap());
    let mut s = String::new();
    assert_eq!(&*a.header().path_bytes(), b"sparse_end.txt");
    t!(a.read_to_string(&mut s).await);
    assert!(s[..s.len() - 9].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[s.len() - 9..], "test_end\n");

    let mut a = t!(entries.next().await.unwrap());
    let mut s = String::new();
    assert_eq!(&*a.header().path_bytes(), b"sparse_ext.txt");
    t!(a.read_to_string(&mut s).await);
    assert!(s[..0x1000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x1000..0x1000 + 5], "text\n");
    assert!(s[0x1000 + 5..0x3000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x3000..0x3000 + 5], "text\n");
    assert!(s[0x3000 + 5..0x5000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x5000..0x5000 + 5], "text\n");
    assert!(s[0x5000 + 5..0x7000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x7000..0x7000 + 5], "text\n");
    assert!(s[0x7000 + 5..0x9000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x9000..0x9000 + 5], "text\n");
    assert!(s[0x9000 + 5..0xb000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0xb000..0xb000 + 5], "text\n");

    let mut a = t!(entries.next().await.unwrap());
    let mut s = String::new();
    assert_eq!(&*a.header().path_bytes(), b"sparse.txt");
    t!(a.read_to_string(&mut s).await);
    assert!(s[..0x1000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x1000..0x1000 + 6], "hello\n");
    assert!(s[0x1000 + 6..0x2fa0].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x2fa0..0x2fa0 + 6], "world\n");
    assert!(s[0x2fa0 + 6..0x4000].chars().all(|x| x == '\u{0}'));

    assert!(entries.next().await.is_none());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn extract_sparse() {
    let rdr = tar!("sparse.tar");
    let ar = Archive::new(rdr);
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    t!(ar.unpack(td.path()).await);

    let mut s = String::new();
    t!(t!(File::open(td.path().join("sparse_begin.txt")).await)
        .read_to_string(&mut s)
        .await);
    assert_eq!(&s[..5], "test\n");
    assert!(s[5..].chars().all(|x| x == '\u{0}'));

    s.truncate(0);
    t!(t!(File::open(td.path().join("sparse_end.txt")).await)
        .read_to_string(&mut s)
        .await);
    assert!(s[..s.len() - 9].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[s.len() - 9..], "test_end\n");

    s.truncate(0);
    t!(t!(File::open(td.path().join("sparse_ext.txt")).await)
        .read_to_string(&mut s)
        .await);
    assert!(s[..0x1000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x1000..0x1000 + 5], "text\n");
    assert!(s[0x1000 + 5..0x3000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x3000..0x3000 + 5], "text\n");
    assert!(s[0x3000 + 5..0x5000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x5000..0x5000 + 5], "text\n");
    assert!(s[0x5000 + 5..0x7000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x7000..0x7000 + 5], "text\n");
    assert!(s[0x7000 + 5..0x9000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x9000..0x9000 + 5], "text\n");
    assert!(s[0x9000 + 5..0xb000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0xb000..0xb000 + 5], "text\n");

    s.truncate(0);
    t!(t!(File::open(td.path().join("sparse.txt")).await)
        .read_to_string(&mut s)
        .await);
    assert!(s[..0x1000].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x1000..0x1000 + 6], "hello\n");
    assert!(s[0x1000 + 6..0x2fa0].chars().all(|x| x == '\u{0}'));
    assert_eq!(&s[0x2fa0..0x2fa0 + 6], "world\n");
    assert!(s[0x2fa0 + 6..0x4000].chars().all(|x| x == '\u{0}'));
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn path_separators() {
    let mut ar = Builder::new(Vec::new());
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let path = td.path().join("test");
    t!(create_file_with_contents(&path, b"test").await);

    let short_path: PathBuf = repeat("abcd").take(2).collect();
    let long_path: PathBuf = repeat("abcd").take(50).collect();

    // Make sure UStar headers normalize to Unix path separators
    let mut header = Header::new_ustar();

    t!(header.set_path(&short_path));
    assert_eq!(t!(header.path()), short_path);
    assert!(!header.path_bytes().contains(&b'\\'));

    t!(header.set_path(&long_path));
    assert_eq!(t!(header.path()), long_path);
    assert!(!header.path_bytes().contains(&b'\\'));

    // Make sure GNU headers normalize to Unix path separators,
    // including the `@LongLink` fallback used by `append_file`.
    t!(ar
        .append_file(&short_path, &mut t!(File::open(&path).await))
        .await);
    t!(ar
        .append_file(&long_path, &mut t!(File::open(&path).await))
        .await);

    let rd = t!(ar.into_inner().await);
    let ar = Archive::new(&rd[..]);
    let mut entries = t!(ar.entries());

    let entry = t!(entries.next().await.unwrap());
    assert_eq!(t!(entry.path()), short_path);
    assert!(!entry.path_bytes().contains(&b'\\'));

    let entry = t!(entries.next().await.unwrap());
    assert_eq!(t!(entry.path()), long_path);
    assert!(!entry.path_bytes().contains(&b'\\'));

    let entry = entries.next().await;
    assert!(entry.is_none());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(unix)]
async fn append_path_symlink() {
    use std::{borrow::Cow, env, os::unix::fs::symlink};

    let mut ar = Builder::new(Vec::new());
    ar.follow_symlinks(false);
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let long_linkname = "abcd".repeat(30);
    let long_pathname = "dcba".repeat(30);
    t!(env::set_current_dir(td.path()));
    // "short" path name / short link name
    t!(symlink("testdest", "test"));
    t!(ar.append_path("test").await);
    // short path name / long link name
    t!(symlink(&long_linkname, "test2"));
    t!(ar.append_path("test2").await);
    // long path name / long link name
    t!(symlink(&long_linkname, &long_pathname));
    t!(ar.append_path(&long_pathname).await);

    let rd = t!(ar.into_inner().await);
    let ar = Archive::new(&rd[..]);
    let mut entries = t!(ar.entries());

    let entry = t!(entries.next().await.unwrap());
    assert_eq!(t!(entry.path()), Path::new("test"));
    assert_eq!(
        t!(entry.link_name()),
        Some(Cow::from(Path::new("testdest")))
    );
    assert_eq!(t!(entry.header().size()), 0);

    let entry = t!(entries.next().await.unwrap());
    assert_eq!(t!(entry.path()), Path::new("test2"));
    assert_eq!(
        t!(entry.link_name()),
        Some(Cow::from(Path::new(&long_linkname)))
    );
    assert_eq!(t!(entry.header().size()), 0);

    let entry = t!(entries.next().await.unwrap());
    assert_eq!(t!(entry.path()), Path::new(&long_pathname));
    assert_eq!(
        t!(entry.link_name()),
        Some(Cow::from(Path::new(&long_linkname)))
    );
    assert_eq!(t!(entry.header().size()), 0);

    assert!(entries.next().await.is_none());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn name_with_slash_doesnt_fool_long_link_and_bsd_compat() {
    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());

    let mut ar = Builder::new(Vec::new());

    let mut h = Header::new_gnu();
    t!(h.set_path("././@LongLink"));
    h.set_size(4);
    h.set_entry_type(EntryType::new(b'L'));
    h.set_cksum();
    t!(ar.append(&h, "foo\0".as_bytes()).await);

    let mut header = Header::new_gnu();
    header.set_entry_type(EntryType::Regular);
    t!(header.set_path("testdir/"));
    header.set_size(0);
    header.set_cksum();
    t!(ar.append(&header, &mut io::empty()).await);

    // Extracting
    let rdr = t!(ar.into_inner().await);
    let ar = Archive::new(&rdr[..]);
    t!(ar.clone().unpack(td.path()).await);

    // Iterating
    let rdr = ar.into_inner().map_err(|_| ()).unwrap();
    let ar = Archive::new(rdr);
    assert!(t!(ar.entries()).all(|fr| fr.is_ok()).await);

    assert!(td.path().join("foo").is_file());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn insert_local_file_different_name() {
    for i in 0..100 {
        println!("----- {i} ---");
        let mut ar = Builder::new(Vec::new());
        let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
        let path = td.path().join("directory");
        t!(fs::create_dir(&path).await);
        ar.append_path_with_name(&path, "archive/dir")
            .await
            .unwrap();
        let path = td.path().join("file");
        t!(create_file_with_contents(&path, b"test").await);
        ar.append_path_with_name(&path, "archive/dir/f")
            .await
            .unwrap();

        let rd = t!(ar.into_inner().await);
        let ar = Archive::new(&rd[..]);
        let mut entries = t!(ar.entries());
        let entry = t!(entries.next().await.unwrap());
        assert_eq!(t!(entry.path()), Path::new("archive/dir"));
        let entry = t!(entries.next().await.unwrap());
        assert_eq!(t!(entry.path()), Path::new("archive/dir/f"));
        let entry = entries.next().await;
        assert!(entry.is_none());
    }
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(unix)]
async fn tar_directory_containing_symlink_to_directory() {
    use std::os::unix::fs::symlink;

    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    let dummy_src = t!(TempBuilder::new().prefix("dummy_src").tempdir());
    let dummy_dst = td.path().join("dummy_dst");
    let mut ar = Builder::new(Vec::new());
    t!(symlink(dummy_src.path().display().to_string(), &dummy_dst));

    assert!(dummy_dst.read_link().is_ok());
    assert!(dummy_dst.read_link().unwrap().is_dir());
    ar.append_dir_all("symlinks", td.path()).await.unwrap();
    ar.finish().await.unwrap();
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn long_path() {
    let td = t!(TempBuilder::new().prefix("tar-rs").tempdir());
    let rdr = tar!("7z_long_path.tar");
    let ar = Archive::new(rdr);
    ar.unpack(td.path()).await.unwrap();
}

const BLOCK: usize = 512;

/// Append `data` as 512-byte tar blocks, zero-padding the last one.
fn push_block(out: &mut Vec<u8>, data: &[u8]) {
    out.extend_from_slice(data);
    let rem = data.len() % BLOCK;
    if rem != 0 {
        out.extend(repeat(0u8).take(BLOCK - rem));
    }
}

/// Encode a pax record `"<len> key=value\n"`, where `<len>` includes itself.
fn pax_record(key: &str, value: &str) -> Vec<u8> {
    let mut len = key.len() + value.len() + 3; // one ' ', one '=', one '\n'
    loop {
        let candidate = format!("{len} {key}={value}\n");
        if candidate.len() == len {
            return candidate.into_bytes();
        }
        len = candidate.len();
    }
}

/// Regression test for GHSA-35rm-7j9c-2f7m (PAX extension-header desync).
///
/// A buffered PAX `size` applies to the next file entry, not an intervening
/// extension header. Mis-applying it to a GNU longname (`L`) reads the longname
/// body with the wrong length and desyncs the parse, so an `x -> L -> file`
/// stream can smuggle content past a POSIX-correct parser like GNU tar.
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn pax_size_not_applied_to_intermediary_longname() {
    // B0: PAX local-extension header whose records declare size = 2 blocks.
    let recs = pax_record("size", &(BLOCK * 2).to_string());
    let mut x = Header::new_ustar();
    t!(x.set_path("PaxHeaders/0"));
    x.set_entry_type(EntryType::new(b'x'));
    x.set_size(recs.len() as u64);
    x.set_cksum();

    // B3: longname for the next file; the GNU body is exactly `name + NUL`.
    let longname = b"GNU_SEES_THIS.txt\0";
    // B2: longname header, sized to its own body (one block on the wire).
    let mut long = Header::new_gnu();
    t!(long.set_path("././@LongLink"));
    long.set_entry_type(EntryType::new(b'L'));
    long.set_size(longname.len() as u64);
    long.set_cksum();

    // B4: the file the PAX `size` legitimately describes (own size 1 block).
    let mut placeholder = Header::new_ustar();
    t!(placeholder.set_path("placeholder_A"));
    placeholder.set_entry_type(EntryType::Regular);
    placeholder.set_size(BLOCK as u64);
    placeholder.set_cksum();

    // B5: a second file header, the smuggled payload. To a correct parser this
    // is opaque data inside `placeholder_A`; a desynced parser parses it.
    let smuggled_body = b"#!/bin/sh\n# SMUGGLED ENTRY: invisible to a GNU-tar-based scanner\n";
    let mut smuggled = Header::new_ustar();
    t!(smuggled.set_path("hidden_payload.sh"));
    smuggled.set_entry_type(EntryType::Regular);
    smuggled.set_size(smuggled_body.len() as u64);
    smuggled.set_cksum();

    let mut tar = Vec::new();
    push_block(&mut tar, x.as_bytes()); // B0
    push_block(&mut tar, &recs); // B1
    push_block(&mut tar, long.as_bytes()); // B2
    push_block(&mut tar, longname); // B3
    push_block(&mut tar, placeholder.as_bytes()); // B4
    push_block(&mut tar, smuggled.as_bytes()); // B5
    push_block(&mut tar, smuggled_body); // B6
    tar.extend(repeat(0u8).take(BLOCK * 2)); // EOF

    // stream view: the surfaced entry must converge with GNU tar.
    let ar = Archive::new(&tar[..]);
    let mut entries = t!(ar.entries());

    let mut first = t!(entries.next().await.unwrap());
    let mut body = Vec::new();
    t!(first.read_to_end(&mut body).await);

    // a correct parser reads B5 (opaque) as this entry's data; a desynced one
    // reads B6, the smuggled script.
    assert!(
        body.starts_with(b"hidden_payload.sh"),
        "PAX size mis-applied to intermediary `L` header: entry data desynced \
         (prefix {:?})",
        String::from_utf8_lossy(&body[..body.len().min(16)]),
    );
    assert!(
        !body.starts_with(b"#!/bin/sh"),
        "smuggled executable payload was materialized as entry data",
    );
    assert_eq!(&*first.path_bytes(), b"GNU_SEES_THIS.txt");
    assert!(
        entries.next().await.is_none(),
        "unexpected extra entry surfaced"
    );

    // on-disk view: GNU tar writes one file, GNU_SEES_THIS.txt, holding the
    // opaque bytes B5+B6, and never writes hidden_payload.sh. async-tar must match.
    let mut expected = Vec::new();
    push_block(&mut expected, smuggled.as_bytes()); // B5
    push_block(&mut expected, smuggled_body); // B6

    let td = t!(TempBuilder::new().prefix("async-tar").tempdir());
    t!(Archive::new(&tar[..]).unpack(td.path()).await);

    let extracted = td.path().join("GNU_SEES_THIS.txt");
    assert!(
        extracted.exists(),
        "expected `GNU_SEES_THIS.txt` to be extracted"
    );
    let on_disk = t!(fs::read(&extracted).await);
    assert_eq!(
        on_disk,
        expected,
        "extracted file diverges from a POSIX-correct parser: for identical \
         archive bytes async-tar wrote different contents than GNU tar (got \
         prefix {:?})",
        String::from_utf8_lossy(&on_disk[..on_disk.len().min(16)]),
    );
    assert!(
        !td.path().join("hidden_payload.sh").exists(),
        "smuggled file `hidden_payload.sh` was written to disk",
    );
}

/// Regression test for the buffered-PAX state leak, adjacent to
/// GHSA-35rm-7j9c-2f7m and not covered by the extension-header guard.
///
/// The records buffered for an `x` header are cleared from one copy but not the
/// copy used for `size`/`uid`/`gid`, so the size bleeds onto the next entry. A
/// `size` for one file followed by an unrelated file must not truncate the
/// second file.
#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn pax_size_does_not_leak_to_subsequent_entry() {
    let first_body = b"first-file-7"; // 12 bytes
    let second_body = b"second-file-should-be-read-in-full-42-bytes"; // 43 bytes

    // B0: PAX local-extension header carrying `size` for the FIRST file only.
    let recs = pax_record("size", &first_body.len().to_string());
    let mut x = Header::new_ustar();
    t!(x.set_path("PaxHeaders/0"));
    x.set_entry_type(EntryType::new(b'x'));
    x.set_size(recs.len() as u64);
    x.set_cksum();

    // B2/B3: first file. Header size 0, so its length comes only from the pax
    // `size` record (proving the override is live).
    let mut first = Header::new_ustar();
    t!(first.set_path("first.txt"));
    first.set_entry_type(EntryType::Regular);
    first.set_size(0);
    first.set_cksum();

    // B4/B5: second file, with its own size and no pax header.
    let mut second = Header::new_ustar();
    t!(second.set_path("second.txt"));
    second.set_entry_type(EntryType::Regular);
    second.set_size(second_body.len() as u64);
    second.set_cksum();

    let mut tar = Vec::new();
    push_block(&mut tar, x.as_bytes()); // B0
    push_block(&mut tar, &recs); // B1
    push_block(&mut tar, first.as_bytes()); // B2
    push_block(&mut tar, first_body); // B3
    push_block(&mut tar, second.as_bytes()); // B4
    push_block(&mut tar, second_body); // B5
    tar.extend(repeat(0u8).take(BLOCK * 2)); // EOF

    let ar = Archive::new(&tar[..]);
    let mut entries = t!(ar.entries());

    let mut e1 = t!(entries.next().await.unwrap());
    assert_eq!(&*e1.path_bytes(), b"first.txt");
    let mut b1 = Vec::new();
    t!(e1.read_to_end(&mut b1).await);
    assert_eq!(b1, first_body, "first file read incorrectly");

    let mut e2 = t!(entries.next().await.unwrap());
    assert_eq!(&*e2.path_bytes(), b"second.txt");
    let mut b2 = Vec::new();
    t!(e2.read_to_end(&mut b2).await);
    // If the buffered PAX size leaked, this is truncated to first_body.len().
    assert_eq!(
        b2,
        second_body,
        "PAX `size` from the first entry leaked onto the second entry: \
         second file read as {} bytes instead of {}",
        b2.len(),
        second_body.len(),
    );
}
