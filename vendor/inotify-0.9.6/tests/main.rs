#![deny(warnings)]


// This test suite is incomplete and doesn't cover all available functionality.
// Contributions to improve test coverage would be highly appreciated!

use inotify::{
    Inotify,
    WatchMask,
};
use std::fs::File;
use std::io::{
    Write,
    ErrorKind,
};
use std::os::unix::io::{
    AsRawFd,
    FromRawFd,
    IntoRawFd,
};
use std::path::PathBuf;
use tempfile::TempDir;


#[test]
fn it_should_watch_a_file() {
    let mut testdir = TestDir::new();
    let (path, mut file) = testdir.new_file();

    let mut inotify = Inotify::init().unwrap();
    let watch = inotify.add_watch(&path, WatchMask::MODIFY).unwrap();

    write_to(&mut file);

    let mut buffer = [0; 1024];
    let events = inotify.read_events_blocking(&mut buffer).unwrap();

    let mut num_events = 0;
    for event in events {
        assert_eq!(watch, event.wd);
        num_events += 1;
    }
    assert!(num_events > 0);
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn it_should_watch_a_file_async() {
    let mut testdir = TestDir::new();
    let (path, mut file) = testdir.new_file();

    let mut inotify = Inotify::init().unwrap();
    let watch = inotify.add_watch(&path, WatchMask::MODIFY).unwrap();

    write_to(&mut file);

    let mut buffer = [0; 1024];

    use futures_util::StreamExt;
    let events = inotify
        .event_stream(&mut buffer[..])
        .unwrap()
        .take(1)
        .collect::<Vec<_>>()
        .await;

    let mut num_events = 0;
    for event in events {
        if let Ok(event) = event {
            assert_eq!(watch, event.wd);
            num_events += 1;
        }
    }
    assert!(num_events > 0);
}

#[test]
fn it_should_return_immediately_if_no_events_are_available() {
    let mut inotify = Inotify::init().unwrap();

    let mut buffer = [0; 1024];
    assert_eq!(0, inotify.read_events(&mut buffer).unwrap().count());
}

#[test]
fn it_should_convert_the_name_into_an_os_str() {
    let mut testdir = TestDir::new();
    let (path, mut file) = testdir.new_file();

    let mut inotify = Inotify::init().unwrap();
    inotify.add_watch(&path.parent().unwrap(), WatchMask::MODIFY).unwrap();

    write_to(&mut file);

    let mut buffer = [0; 1024];
    let mut events = inotify.read_events_blocking(&mut buffer).unwrap();

    if let Some(event) = events.next() {
        assert_eq!(path.file_name(), event.name);
    }
    else {
        panic!("Expected inotify event");
    }
}

#[test]
fn it_should_set_name_to_none_if_it_is_empty() {
    let mut testdir = TestDir::new();
    let (path, mut file) = testdir.new_file();

    let mut inotify = Inotify::init().unwrap();
    inotify.add_watch(&path, WatchMask::MODIFY).unwrap();

    write_to(&mut file);

    let mut buffer = [0; 1024];
    let mut events = inotify.read_events_blocking(&mut buffer).unwrap();

    if let Some(event) = events.next() {
        assert_eq!(event.name, None);
    }
    else {
        panic!("Expected inotify event");
    }
}

#[test]
fn it_should_not_accept_watchdescriptors_from_other_instances() {
    let mut testdir = TestDir::new();
    let (path, _) = testdir.new_file();

    let mut inotify = Inotify::init().unwrap();
    let _ = inotify.add_watch(&path, WatchMask::ACCESS).unwrap();

    let mut second_inotify = Inotify::init().unwrap();
    let wd2 = second_inotify.add_watch(&path, WatchMask::ACCESS).unwrap();

    assert_eq!(inotify.rm_watch(wd2).unwrap_err().kind(), ErrorKind::InvalidInput);
}

#[test]
fn watch_descriptors_from_different_inotify_instances_should_not_be_equal() {
    let mut testdir = TestDir::new();
    let (path, _) = testdir.new_file();

    let mut inotify_1 = Inotify::init()
        .unwrap();
    let mut inotify_2 = Inotify::init()
        .unwrap();

    let wd_1 = inotify_1
        .add_watch(&path, WatchMask::ACCESS)
        .unwrap();
    let wd_2 = inotify_2
        .add_watch(&path, WatchMask::ACCESS)
        .unwrap();

    // As far as inotify is concerned, watch descriptors are just integers that
    // are scoped per inotify instance. This means that multiple instances will
    // produce the same watch descriptor number, a case we want inotify-rs to
    // detect.
    assert!(wd_1 != wd_2);
}

#[test]
fn watch_descriptor_equality_should_not_be_confused_by_reused_fds() {
    let mut testdir = TestDir::new();
    let (path, _) = testdir.new_file();

    // When a new inotify instance is created directly after closing another
    // one, it is possible that the file descriptor is reused immediately, and
    // we end up with a new instance that has the same file descriptor as the
    // old one.
    // This is quite likely, but it doesn't happen every time. Therefore we may
    // need a few tries until we find two instances where that is the case.
    let (wd_1, mut inotify_2) = loop {
        let mut inotify_1 = Inotify::init()
            .unwrap();

        let wd_1 = inotify_1
            .add_watch(&path, WatchMask::ACCESS)
            .unwrap();
        let fd_1 = inotify_1.as_raw_fd();

        inotify_1
            .close()
            .unwrap();
        let inotify_2 = Inotify::init()
            .unwrap();

        if fd_1 == inotify_2.as_raw_fd() {
            break (wd_1, inotify_2);
        }
    };

    let wd_2 = inotify_2
        .add_watch(&path, WatchMask::ACCESS)
        .unwrap();

    // The way we engineered this situation, both `WatchDescriptor` instances
    // have the same fields. They still come from different inotify instances
    // though, so they shouldn't be equal.
    assert!(wd_1 != wd_2);

    inotify_2
        .close()
        .unwrap();

    // A little extra gotcha: If both inotify instances are closed, and the `Eq`
    // implementation naively compares the weak pointers, both will be `None`,
    // making them equal. Let's make sure this isn't the case.
    assert!(wd_1 != wd_2);
}

#[test]
fn it_should_implement_raw_fd_traits_correctly() {
    let fd = Inotify::init()
        .expect("Failed to initialize inotify instance")
        .into_raw_fd();

    // If `IntoRawFd` has been implemented naively, `Inotify`'s `Drop`
    // implementation will have closed the inotify instance at this point. Let's
    // make sure this didn't happen.
    let mut inotify = unsafe { <Inotify as FromRawFd>::from_raw_fd(fd) };

    let mut buffer = [0; 1024];
    if let Err(error) = inotify.read_events(&mut buffer) {
        panic!("Failed to add watch: {}", error);
    }
}


struct TestDir {
    dir: TempDir,
    counter: u32,
}

impl TestDir {
    fn new() -> TestDir {
        TestDir {
            dir: TempDir::new().unwrap(),
            counter: 0,
        }
    }

    fn new_file(&mut self) -> (PathBuf, File) {
        let id = self.counter;
        self.counter += 1;

        let path = self.dir.path().join("file-".to_string() + &id.to_string());
        let file = File::create(&path)
            .unwrap_or_else(|error| panic!("Failed to create temporary file: {}", error));

        (path, file)
    }
}

fn write_to(file: &mut File) {
    file
        .write(b"This should trigger an inotify event.")
        .unwrap_or_else(|error|
            panic!("Failed to write to file: {}", error)
        );
}
