#[cfg(feature = "runtime-async-std")]
use async_std::{
    fs::{self, File, create_dir},
    prelude::*,
};

#[cfg(feature = "runtime-tokio")]
use tokio::{
    fs::{self, File, create_dir},
    io::AsyncReadExt,
};
#[cfg(feature = "runtime-tokio")]
use tokio_stream::StreamExt;

use tempfile::Builder;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => panic!("{} returned {}", stringify!($e), e),
        }
    };
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn absolute_symlink() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Symlink);
    t!(header.set_path("foo"));
    t!(header.set_link_name("/bar"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    t!(ar.unpack(td.path()).await);

    t!(fs::symlink_metadata(td.path().join("foo")).await);

    let ar = async_tar::Archive::new(&bytes[..]);
    let mut entries = t!(ar.entries());
    let entry = t!(entries.next().await.unwrap());
    assert_eq!(&*entry.link_name_bytes().unwrap(), b"/bar");
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn absolute_hardlink() {
    let td = t!(Builder::new().prefix("tar").tempdir());
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("foo"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Link);
    t!(header.set_path("bar"));
    // This absolute path under tempdir will be created at unpack time
    t!(header.set_link_name(td.path().join("foo")));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    t!(ar.unpack(td.path()).await);
    t!(td.path().join("foo").metadata());
    t!(td.path().join("bar").metadata());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn relative_hardlink() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("foo"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Link);
    t!(header.set_path("bar"));
    t!(header.set_link_name("foo"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    t!(ar.unpack(td.path()).await);
    t!(td.path().join("foo").metadata());
    t!(td.path().join("bar").metadata());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn absolute_link_deref_error() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Symlink);
    t!(header.set_path("foo"));
    t!(header.set_link_name("/"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("foo/bar"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    assert!(ar.unpack(td.path()).await.is_err());
    t!(fs::symlink_metadata(td.path().join("foo")).await);
    assert!(File::open(td.path().join("foo").join("bar")).await.is_err());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn relative_link_deref_error() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Symlink);
    t!(header.set_path("foo"));
    t!(header.set_link_name("../../../../"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("foo/bar"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    assert!(ar.unpack(td.path()).await.is_err());
    t!(fs::symlink_metadata(td.path().join("foo")).await);
    assert!(File::open(td.path().join("foo").join("bar")).await.is_err());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(unix)]
async fn directory_maintains_permissions() {
    use ::std::os::unix::fs::PermissionsExt;

    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Directory);
    t!(header.set_path("foo"));
    header.set_mode(0o777);
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    t!(ar.unpack(td.path()).await);
    let f = t!(File::open(td.path().join("foo")).await);
    let md = t!(f.metadata().await);
    assert!(md.is_dir());
    assert_eq!(md.permissions().mode(), 0o40777);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(not(windows))] // dangling symlinks have weird permissions
async fn modify_link_just_created() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Symlink);
    t!(header.set_path("foo"));
    t!(header.set_link_name("bar"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("bar/foo"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("foo/bar"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    t!(ar.unpack(td.path()).await);

    t!(File::open(td.path().join("bar/foo")).await);
    t!(File::open(td.path().join("bar/bar")).await);
    t!(File::open(td.path().join("foo/foo")).await);
    t!(File::open(td.path().join("foo/bar")).await);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(not(windows))] // dangling symlinks have weird permissions
async fn modify_outside_with_relative_symlink() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Symlink);
    t!(header.set_path("symlink"));
    t!(header.set_link_name(".."));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("symlink/foo/bar"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    let tar_dir = td.path().join("tar");
    create_dir(&tar_dir).await.unwrap();
    assert!(ar.unpack(tar_dir).await.is_err());
    assert!(!td.path().join("foo").exists());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn parent_paths_error() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Symlink);
    t!(header.set_path("foo"));
    t!(header.set_link_name(".."));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("foo/bar"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    assert!(ar.unpack(td.path()).await.is_err());
    t!(td.path().join("foo").symlink_metadata());
    assert!(File::open(td.path().join("foo").join("bar")).await.is_err());
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
#[cfg(unix)]
async fn good_parent_paths_ok() {
    use std::path::PathBuf;
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Symlink);
    t!(header.set_path(PathBuf::from("foo").join("bar")));
    t!(header.set_link_name(PathBuf::from("..").join("bar")));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("bar"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());
    t!(ar.unpack(td.path()).await);
    t!(td.path().join("foo").join("bar").read_link());
    let dst = t!(td.path().join("foo").join("bar").canonicalize());
    t!(File::open(dst).await);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn modify_hard_link_just_created() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Link);
    t!(header.set_path("foo"));
    t!(header.set_link_name("../test"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(1);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("foo"));
    header.set_cksum();
    t!(ar.append(&header, &b"x"[..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());

    let test = td.path().join("test");
    t!(File::create(&test).await);

    let dir = td.path().join("dir");
    assert!(ar.unpack(&dir).await.is_err());

    let mut contents = Vec::new();
    t!(t!(File::open(&test).await).read_to_end(&mut contents).await);
    assert_eq!(contents.len(), 0);
}

#[cfg_attr(feature = "runtime-async-std", async_std::test)]
#[cfg_attr(feature = "runtime-tokio", tokio::test)]
async fn modify_symlink_just_created() {
    let mut ar = async_tar::Builder::new(Vec::new());

    let mut header = async_tar::Header::new_gnu();
    header.set_size(0);
    header.set_entry_type(async_tar::EntryType::Symlink);
    t!(header.set_path("foo"));
    t!(header.set_link_name("../test"));
    header.set_cksum();
    t!(ar.append(&header, &[][..]).await);

    let mut header = async_tar::Header::new_gnu();
    header.set_size(1);
    header.set_entry_type(async_tar::EntryType::Regular);
    t!(header.set_path("foo"));
    header.set_cksum();
    t!(ar.append(&header, &b"x"[..]).await);

    let bytes = t!(ar.into_inner().await);
    let ar = async_tar::Archive::new(&bytes[..]);

    let td = t!(Builder::new().prefix("tar").tempdir());

    let test = td.path().join("test");
    t!(File::create(&test).await);

    let dir = td.path().join("dir");
    t!(ar.unpack(&dir).await);

    let mut contents = Vec::new();
    t!(t!(File::open(&test).await).read_to_end(&mut contents).await);
    assert_eq!(contents.len(), 0);
}
