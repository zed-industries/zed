#![allow(clippy::cognitive_complexity)]

use std::{
    fs::{self, File},
    io::{self, Write},
    mem,
    path::Path,
    thread, time,
};

use tempfile::Builder;

use async_tar::{GnuHeader, Header, HeaderMode};

#[test]
fn default_gnu() {
    let mut h = Header::new_gnu();
    assert!(h.as_gnu().is_some());
    assert!(h.as_gnu_mut().is_some());
    assert!(h.as_ustar().is_none());
    assert!(h.as_ustar_mut().is_none());
}

#[test]
fn goto_old() {
    let mut h = Header::new_old();
    assert!(h.as_gnu().is_none());
    assert!(h.as_gnu_mut().is_none());
    assert!(h.as_ustar().is_none());
    assert!(h.as_ustar_mut().is_none());
}

#[test]
fn goto_ustar() {
    let mut h = Header::new_ustar();
    assert!(h.as_gnu().is_none());
    assert!(h.as_gnu_mut().is_none());
    assert!(h.as_ustar().is_some());
    assert!(h.as_ustar_mut().is_some());
}

#[test]
fn link_name() {
    let mut h = Header::new_gnu();
    t!(h.set_link_name("foo"));
    assert_eq!(t!(h.link_name()).unwrap().to_str(), Some("foo"));
    t!(h.set_link_name("../foo"));
    assert_eq!(t!(h.link_name()).unwrap().to_str(), Some("../foo"));
    t!(h.set_link_name("foo/bar"));
    assert_eq!(t!(h.link_name()).unwrap().to_str(), Some("foo/bar"));
    t!(h.set_link_name("foo\\ba"));
    if cfg!(windows) {
        assert_eq!(t!(h.link_name()).unwrap().to_str(), Some("foo/ba"));
    } else {
        assert_eq!(t!(h.link_name()).unwrap().to_str(), Some("foo\\ba"));
    }

    let name = "foo\\bar\0";
    for (slot, val) in h.as_old_mut().linkname.iter_mut().zip(name.as_bytes()) {
        *slot = *val;
    }
    assert_eq!(t!(h.link_name()).unwrap().to_str(), Some("foo\\bar"));

    assert!(h.set_link_name("\0").is_err());
}

#[test]
fn mtime() {
    let h = Header::new_gnu();
    assert_eq!(t!(h.mtime()), 0);

    let h = Header::new_ustar();
    assert_eq!(t!(h.mtime()), 0);

    let h = Header::new_old();
    assert_eq!(t!(h.mtime()), 0);
}

#[test]
fn user_and_group_name() {
    let mut h = Header::new_gnu();
    t!(h.set_username("foo"));
    t!(h.set_groupname("bar"));
    assert_eq!(t!(h.username()), Some("foo"));
    assert_eq!(t!(h.groupname()), Some("bar"));

    h = Header::new_ustar();
    t!(h.set_username("foo"));
    t!(h.set_groupname("bar"));
    assert_eq!(t!(h.username()), Some("foo"));
    assert_eq!(t!(h.groupname()), Some("bar"));

    h = Header::new_old();
    assert_eq!(t!(h.username()), None);
    assert_eq!(t!(h.groupname()), None);
    assert!(h.set_username("foo").is_err());
    assert!(h.set_groupname("foo").is_err());
}

#[test]
fn dev_major_minor() {
    let mut h = Header::new_gnu();
    t!(h.set_device_major(1));
    t!(h.set_device_minor(2));
    assert_eq!(t!(h.device_major()), Some(1));
    assert_eq!(t!(h.device_minor()), Some(2));

    h = Header::new_ustar();
    t!(h.set_device_major(1));
    t!(h.set_device_minor(2));
    assert_eq!(t!(h.device_major()), Some(1));
    assert_eq!(t!(h.device_minor()), Some(2));

    h.as_ustar_mut().unwrap().dev_minor[0] = 0x7f;
    h.as_ustar_mut().unwrap().dev_major[0] = 0x7f;
    assert!(h.device_major().is_err());
    assert!(h.device_minor().is_err());

    h.as_ustar_mut().unwrap().dev_minor[0] = b'g';
    h.as_ustar_mut().unwrap().dev_major[0] = b'h';
    assert!(h.device_major().is_err());
    assert!(h.device_minor().is_err());

    h = Header::new_old();
    assert_eq!(t!(h.device_major()), None);
    assert_eq!(t!(h.device_minor()), None);
    assert!(h.set_device_major(1).is_err());
    assert!(h.set_device_minor(1).is_err());
}

#[test]
fn set_path() {
    let mut h = Header::new_gnu();
    t!(h.set_path("foo"));
    assert_eq!(t!(h.path()).to_str(), Some("foo"));
    t!(h.set_path("foo/"));
    assert_eq!(t!(h.path()).to_str(), Some("foo/"));
    t!(h.set_path("foo/bar"));
    assert_eq!(t!(h.path()).to_str(), Some("foo/bar"));
    t!(h.set_path("foo\\bar"));
    if cfg!(windows) {
        assert_eq!(t!(h.path()).to_str(), Some("foo/bar"));
    } else {
        assert_eq!(t!(h.path()).to_str(), Some("foo\\bar"));
    }

    let long_name = "foo".repeat(100);
    let medium1 = "foo".repeat(52);
    let medium2 = "fo/".repeat(52);

    assert!(h.set_path(&long_name).is_err());
    assert!(h.set_path(&medium1).is_err());
    assert!(h.set_path(&medium2).is_err());
    assert!(h.set_path("\0").is_err());

    assert!(h.set_path("..").is_err());
    assert!(h.set_path("foo/..").is_err());
    assert!(h.set_path("foo/../bar").is_err());

    h = Header::new_ustar();
    t!(h.set_path("foo"));
    assert_eq!(t!(h.path()).to_str(), Some("foo"));

    assert!(h.set_path(&long_name).is_err());
    assert!(h.set_path(&medium1).is_err());
    t!(h.set_path(&medium2));
    assert_eq!(t!(h.path()).to_str(), Some(&medium2[..]));
}

#[test]
fn set_ustar_path_hard() {
    let mut h = Header::new_ustar();
    let p = Path::new("a").join(vec!["a"; 100].join(""));
    t!(h.set_path(&p));
    let path = t!(h.path());
    let actual: &Path = path.as_ref().into();
    assert_eq!(actual, p);
}

#[test]
fn set_metadata_deterministic() {
    let td = t!(Builder::new().prefix("async-tar").tempdir());
    let tmppath = td.path().join("tmpfile");

    fn mk_header(path: &Path, readonly: bool) -> Result<Header, io::Error> {
        let mut file = t!(File::create(path));
        t!(file.write_all(b"c"));
        let mut perms = t!(file.metadata()).permissions();
        perms.set_readonly(readonly);
        t!(fs::set_permissions(path, perms));
        let mut h = Header::new_ustar();
        h.set_metadata_in_mode(&t!(path.metadata()), HeaderMode::Deterministic);
        Ok(h)
    }

    // Create "the same" File twice in a row, one second apart, with differing readonly values.
    let one = t!(mk_header(tmppath.as_path(), false));
    thread::sleep(time::Duration::from_millis(1050));
    let two = t!(mk_header(tmppath.as_path(), true));

    // Always expected to match.
    assert_eq!(t!(one.size()), t!(two.size()));
    assert_eq!(t!(one.path()), t!(two.path()));
    assert_eq!(t!(one.mode()), t!(two.mode()));

    // Would not match without `Deterministic`.
    assert_eq!(t!(one.mtime()), t!(two.mtime()));
    // TODO: No great way to validate that these would not be filled, but
    // check them anyway.
    assert_eq!(t!(one.uid()), t!(two.uid()));
    assert_eq!(t!(one.gid()), t!(two.gid()));
}

#[test]
fn extended_numeric_format() {
    let mut h: GnuHeader = unsafe { mem::zeroed() };
    h.as_header_mut().set_size(42);
    assert_eq!(h.size, [48, 48, 48, 48, 48, 48, 48, 48, 48, 53, 50, 0]);
    h.as_header_mut().set_size(8_589_934_593);
    assert_eq!(h.size, [0x80, 0, 0, 0, 0, 0, 0, 0x02, 0, 0, 0, 1]);
    h.size = [0x80, 0, 0, 0, 0, 0, 0, 0x02, 0, 0, 0, 0];
    assert_eq!(h.as_header().entry_size().unwrap(), 0x0002_0000_0000);
    h.size = [48, 48, 48, 48, 48, 48, 48, 48, 48, 53, 51, 0];
    assert_eq!(h.as_header().entry_size().unwrap(), 43);

    h.as_header_mut().set_gid(42);
    assert_eq!(h.gid, [48, 48, 48, 48, 48, 53, 50, 0]);
    assert_eq!(h.as_header().gid().unwrap(), 42);
    h.as_header_mut().set_gid(0x7fff_ffff_ffff_ffff);
    assert_eq!(h.gid, [0xff; 8]);
    assert_eq!(h.as_header().gid().unwrap(), 0x7fff_ffff_ffff_ffff);
    h.uid = [0x80, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78];
    assert_eq!(h.as_header().uid().unwrap(), 0x1234_5678);

    h.mtime = [
        0x80, 0, 0, 0, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    ];
    assert_eq!(h.as_header().mtime().unwrap(), 0x0123_4567_89ab_cdef);
}

#[test]
fn byte_slice_conversion() {
    let h = Header::new_gnu();
    let b: &[u8] = h.as_bytes();
    let b_conv: &[u8] = Header::from_byte_slice(h.as_bytes()).as_bytes();
    assert_eq!(b, b_conv);
}
