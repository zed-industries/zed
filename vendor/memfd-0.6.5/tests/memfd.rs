extern crate memfd;
use std::fs;
use std::os::unix::io::AsRawFd;

#[test]
fn test_memfd_default() {
    let opts = memfd::MemfdOptions::default();
    let m0 = opts.create("default").unwrap();
    let meta0 = m0.as_file().metadata().unwrap();
    assert_eq!(meta0.len(), 0);
    assert_eq!(meta0.is_file(), true);
    assert_eq!(get_close_on_exec(&m0).unwrap(), true);
    drop(m0)
}

#[test]
fn test_memfd_no_cloexec() {
    let memfd = memfd::MemfdOptions::default()
        .close_on_exec(false)
        .create("no-cloexec")
        .unwrap();
    assert_eq!(get_close_on_exec(&memfd).unwrap(), false);
}

#[test]
fn test_memfd_multi() {
    let opts = memfd::MemfdOptions::default();
    let m0 = opts.create("default").unwrap();
    let f0 = m0.as_file().as_raw_fd();

    let m1 = opts.create("default").unwrap();
    let f1 = m1.as_file().as_raw_fd();
    assert!(f0 != f1);

    let m0_file = m0.into_file();
    assert_eq!(f0, m0_file.as_raw_fd());
}

#[test]
fn test_memfd_from_into() {
    let opts = memfd::MemfdOptions::default();
    let m0 = opts.create("default").unwrap();
    let f0 = m0.into_file();
    let _ = memfd::Memfd::try_from_file(f0).expect("failed to convert a legit memfd file");

    let rootdir = fs::File::open("/").unwrap();
    let _ = memfd::Memfd::try_from_file(rootdir)
        .expect_err("unexpected conversion from a non-memfd file");
}

/// Check if the close-on-exec flag is set for the memfd.
pub fn get_close_on_exec(memfd: &memfd::Memfd) -> std::io::Result<bool> {
    let flags = rustix::io::fcntl_getfd(memfd.as_file())?;
    Ok(flags.contains(rustix::io::FdFlags::CLOEXEC))
}
