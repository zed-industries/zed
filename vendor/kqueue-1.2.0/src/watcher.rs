use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Error, Result};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use std::path::Path;
use std::ptr;
use std::time::Duration;

use kqueue_sys::constants::{EventFilter, EventFlag, FilterFlag};
use kqueue_sys::{kevent, kqueue};
use libc::close;

use crate::event::{get_event, Event, EventIter};
use crate::types::{Ident, KqueueOpts};

#[doc(hidden)]
#[derive(Debug)]
pub struct Watched {
    flags: FilterFlag,
}

/// Watches one or more resources
///
/// These can be created with `Watcher::new()`. You can create as many
/// `Watcher`s as you want, and they can watch as many objects as you wish.
/// The objects do not need to be the same type.
///
/// Each `Watcher` is backed by a `kqueue(2)` queue. These resources are freed
/// on the `Watcher`s destruction. If the destructor cannot run for whatever
/// reason, the underlying kernel object will be leaked.
///
/// Files and file descriptors given to the `Watcher` are presumed to be owned
/// by the `Watcher`. In a future version, the API will make this explicit via
/// `OwnedFd`s
#[derive(Debug)]
pub struct Watcher {
    pub(crate) watched: HashMap<(Ident, EventFilter), Watched>,
    pub(crate) queue: OwnedFd,
    pub(crate) started: bool,
    pub(crate) opts: KqueueOpts,
}

impl Watcher {
    /// Creates a new `Watcher`
    ///
    /// Creates a brand new `Watcher` with `KqueueOpts::default()`.
    ///
    /// # Errors
    ///
    /// Will return `err` if the underlying `kqueue(2)` call fails.
    pub fn new() -> Result<Watcher> {
        let queue = unsafe { kqueue() };

        if queue == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(Watcher {
                watched: HashMap::new(),
                queue: unsafe { OwnedFd::from_raw_fd(queue) },
                started: false,
                opts: KqueueOpts::default(),
            })
        }
    }

    /// Disables the `clear` flag on a `Watcher`. New events will no longer
    /// be added with the `EV_CLEAR` flag on `watch`.
    pub fn disable_clears(&mut self) -> &mut Self {
        self.opts.clear = false;
        self
    }

    /// Adds a `pid` to the `Watcher` to be watched
    #[allow(clippy::missing_errors_doc)]
    pub fn add_pid(
        &mut self,
        pid: libc::pid_t,
        filter: EventFilter,
        flags: FilterFlag,
    ) -> Result<()> {
        let ident = Ident::Pid(pid);
        let watch = Watched { flags };

        self.watched.insert((ident, filter), watch);

        Ok(())
    }

    /// Adds a file by filename to be watched
    ///
    /// **NB**: `kqueue(2)` is an `fd`-based API. If you add a filename with
    /// `add_filename`, internally we open it and pass the file descriptor to
    /// `kqueue(2)`. If the file is moved or deleted, and a new file is created
    /// with the same name, you will not receive new events for it without
    /// calling `add_filename` again.
    ///
    /// TODO: Adding new files requires calling `Watcher.watch` again
    ///
    /// # Errors
    ///
    /// Will return an error if the file cannot be opened successfully.
    pub fn add_filename<P: AsRef<Path>>(
        &mut self,
        filename: P,
        filter: EventFilter,
        flags: FilterFlag,
    ) -> Result<()> {
        let file = File::open(filename.as_ref())?;
        let filename = filename.as_ref().to_string_lossy().into_owned();
        let ident = Ident::Filename(file.into_raw_fd(), filename);
        let watch = Watched { flags };

        // If the old filename was watched, we need to drop the old and start the new one.
        // We swap out the old item in the hashmap, and if it's not `None`, then we make sure
        // to close the fd
        let key = (ident, filter);
        if let Some((key, _)) = self.watched.remove_entry(&key) {
            // We're round-tripping the fd, so it shouldn't be a problem. Also, kqueue will drop
            // any events when we call close on the fd, so we do not need to call delete_kevents
            unsafe { close(key.0.into()) };
        }
        self.watched.insert(key, watch);

        Ok(())
    }

    /// Adds a descriptor to a `Watcher`. This or `add_file` is the preferred
    /// way to watch a file
    ///
    /// TODO: Adding new files requires calling `Watcher.watch` again
    #[allow(clippy::missing_errors_doc)]
    pub fn add_fd(&mut self, fd: RawFd, filter: EventFilter, flags: FilterFlag) -> Result<()> {
        let ident = Ident::Fd(fd);
        let watch = Watched { flags };

        self.watched.insert((ident, filter), watch);

        Ok(())
    }

    /// Adds a `File` to a `Watcher`. This, or `add_fd` is the preferred way
    /// to watch a file
    ///
    /// TODO: Adding new files requires calling `Watcher.watch` again
    #[allow(clippy::missing_errors_doc)]
    pub fn add_file(&mut self, file: &File, filter: EventFilter, flags: FilterFlag) -> Result<()> {
        self.add_fd(file.as_raw_fd(), filter, flags)
    }

    fn delete_kevents(&self, ident: &Ident, filter: EventFilter) -> Result<()> {
        let kev = &[kevent::new(
            ident.try_into()?,
            filter,
            EventFlag::EV_DELETE,
            FilterFlag::empty(),
        )];

        let ret = unsafe {
            kevent(
                self.queue.as_raw_fd(),
                kev.as_ptr(),
                1,
                ptr::null_mut(),
                0,
                ptr::null(),
            )
        };

        match ret {
            -1 => Err(Error::last_os_error()),
            _ => Ok(()),
        }
    }

    /// Removes a pid from a `Watcher`
    #[allow(clippy::missing_errors_doc)]
    pub fn remove_pid(&mut self, pid: libc::pid_t, filter: EventFilter) -> Result<()> {
        self.watched.remove(&(Ident::Pid(pid), filter));
        self.delete_kevents(&Ident::Pid(pid), filter)
    }

    /// Removes a filename from a `Watcher`.
    ///
    /// *NB*: This matches the `filename` that this item was initially added under.
    /// If a file has been moved, it will not be removable by the new name.
    ///
    /// # Errors
    ///
    /// Returns an error if asked to remove a filename that hasn't been watched.
    pub fn remove_filename<P: AsRef<Path> + Debug>(
        &mut self,
        filename: P,
        filter: EventFilter,
    ) -> Result<()> {
        // To ensure that we match the behavior of add_filename, and to avoid
        // breaking compatish, we're going to accept strings that aren't utf-8
        // and lossily convert them. In a v2, we should just use Paths here, idk
        // why we didn't do that initially.
        let filename = filename.as_ref().to_string_lossy().into_owned();
        let ident = Ident::Filename(0, filename);
        let key = (ident, filter);
        let Some((key, _)) = self.watched.remove_entry(&key) else {
            // Get name after it was moved into the ident
            let Ident::Filename(_, ref name) = key.0 else {
                // we literally just turned this into an ident
                unreachable!()
            };
            return Err(Error::new(
                io::ErrorKind::NotFound,
                format!("{name:?} was not being watched"),
            ));
        };

        let ret = self.delete_kevents(&key.0, filter);
        let fd = key.0.into();
        unsafe { close(fd) };
        ret
    }

    /// Removes an fd from a `Watcher`.
    #[allow(clippy::missing_errors_doc)]
    pub fn remove_fd(&mut self, fd: RawFd, filter: EventFilter) -> Result<()> {
        self.watched.remove(&(Ident::Fd(fd), filter));
        self.delete_kevents(&Ident::Fd(fd), filter)
    }

    /// Removes a `File` from a `Watcher`
    #[allow(clippy::missing_errors_doc)]
    pub fn remove_file(&mut self, file: &File, filter: EventFilter) -> Result<()> {
        self.remove_fd(file.as_raw_fd(), filter)
    }

    /// Starts watching for events from `kqueue(2)`. This function needs to
    /// be called before `Watcher.iter()` or `Watcher.poll()` to actually
    /// start listening for events.
    ///
    /// # Errors
    ///
    /// Returns an error in any of the following conditions:
    /// * one of the `Ident` objects is invalid
    /// * there are too many items (more than 2B) to watch
    /// * the underlying `kevent(2)` call fails
    pub fn watch(&mut self) -> Result<()> {
        let kevs: Vec<kevent> = self.watched.iter().try_fold(
            Vec::with_capacity(self.watched.len()),
            |mut v, ((ident, filter), watched)| {
                v.push(kevent::new(
                    ident.try_into()?,
                    *filter,
                    if self.opts.clear {
                        EventFlag::EV_ADD | EventFlag::EV_CLEAR
                    } else {
                        EventFlag::EV_ADD
                    },
                    watched.flags,
                ));
                Ok::<Vec<kevent>, Error>(v)
            },
        )?;

        // On NetBSD, this is passed as a usize, not i32
        #[cfg(target_os = "netbsd")]
        let Ok(len) = i32::try_from(kevs.len()).and_then(|i| i.try_into()) else {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid number of events: {:?}", kevs.len()),
            ));
        };

        #[cfg(not(target_os = "netbsd"))]
        let Ok(len) = i32::try_from(kevs.len()) else {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid number of events: {:?}", kevs.len()),
            ));
        };

        let ret = unsafe {
            kevent(
                self.queue.as_raw_fd(),
                kevs.as_ptr(),
                len,
                ptr::null_mut(),
                0,
                ptr::null(),
            )
        };

        if ret == -1 {
            Err(Error::last_os_error())
        } else {
            self.started = true;
            Ok(())
        }
    }

    /// Polls for a new event, with an optional timeout. If no `timeout`
    /// is passed or the `Watcher.watch` hasn't been called, then it will
    /// return immediately.
    #[must_use]
    pub fn poll(&self, timeout: Option<Duration>) -> Option<Event> {
        // poll will not block indefinitely
        // None -> return immediately
        match timeout {
            Some(timeout) => get_event(self, Some(timeout)),
            None => get_event(self, Some(Duration::new(0, 0))),
        }
    }

    /// Polls for a new event, with an optional timeout. If no `timeout`
    /// is passed, then it will block until an event is received. It will
    /// return immediately if `Watcher.watch` hasn't been called.
    #[must_use]
    pub fn poll_forever(&self, timeout: Option<Duration>) -> Option<Event> {
        if timeout.is_some() {
            self.poll(timeout)
        } else {
            get_event(self, None)
        }
    }

    /// Creates an iterator that iterates over the queue. This iterator will block
    /// until a new event is received. It will return `None` if `Watcher.watch`
    /// has not been called yet.
    #[must_use]
    pub fn iter(&self) -> EventIter<'_> {
        EventIter { watcher: self }
    }
}

impl<'a> IntoIterator for &'a Watcher {
    type Item = crate::event::Event;
    type IntoIter = crate::event::EventIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl AsRawFd for Watcher {
    fn as_raw_fd(&self) -> RawFd {
        self.queue.as_raw_fd()
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        for (ident, _) in self.watched.keys() {
            match *ident {
                Ident::Filename(fd, _) => unsafe { libc::close(fd) },
                _ => continue,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::{ErrorKind, Seek, SeekFrom, Write};
    use std::os::unix::io::AsRawFd;
    use std::path::Path;
    use std::thread;
    use std::time;

    use kqueue_sys::constants::*;

    use crate::{EventData, Ident, Vnode, Watcher};

    #[cfg(target_os = "freebsd")]
    use std::io::Read;
    #[cfg(target_os = "freebsd")]
    use std::process;

    #[test]
    fn test_add_filename_duplicate_no_fd_leak() {
        let file = tempfile::NamedTempFile::new().expect("couldn't create tempfile");
        let mut watcher = Watcher::new().expect("couldn't create watcher");

        watcher
            .add_filename(
                file.path(),
                EventFilter::EVFILT_VNODE,
                FilterFlag::NOTE_WRITE,
            )
            .expect("first add failed");

        // We open the next possible fd so that we can figure out the next fd number that
        // will be allocated, and immediately close it. Then, since we're pretty sure that
        // we own all the fd's in our test code, we can assume that the previous fd is the
        // one that is in use by the watcher.
        let probe_fd = unsafe { libc::dup(0) };
        let test_fd = probe_fd - 1;
        assert!(probe_fd >= 0, "dup failed");
        unsafe { libc::close(probe_fd) };

        watcher
            .add_filename(
                file.path(),
                EventFilter::EVFILT_VNODE,
                FilterFlag::NOTE_WRITE,
            )
            .expect("second add failed");

        drop(watcher);

        // Check to see if the previous fd is closed or not.
        let leaked = unsafe { libc::fcntl(test_fd, libc::F_GETFD) } != -1;
        assert!(
            !leaked,
            "fd {test_fd} opened by the first add_filename was not closed on drop"
        );
        let leaked = unsafe { libc::fcntl(probe_fd, libc::F_GETFD) } != -1;
        assert!(
            !leaked,
            "fd {probe_fd} opened by the second add_filename was not closed on drop"
        );
    }

    #[test]
    fn test_new_watcher() {
        let mut watcher = Watcher::new().expect("new failed");
        let file = tempfile::tempfile().expect("Couldn't create tempfile");

        watcher
            .add_file(&file, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("add failed");
        watcher.watch().expect("watch failed");
    }

    #[test]
    fn test_filename() {
        let mut watcher = Watcher::new().expect("new failed");
        let file = tempfile::NamedTempFile::new().expect("Couldn't create tempfile");

        watcher
            .add_filename(
                file.path(),
                EventFilter::EVFILT_VNODE,
                FilterFlag::NOTE_WRITE,
            )
            .expect("add failed");
        watcher.watch().expect("watch failed");

        let mut new_file = fs::OpenOptions::new()
            .write(true)
            .open(file.path())
            .expect("open failed");

        new_file.write_all(b"foo").expect("write failed");

        thread::sleep(time::Duration::from_secs(1));

        let ev = watcher.iter().next().expect("Could not get a watch");
        assert!(matches!(ev.data, EventData::Vnode(Vnode::Write)));

        match ev.ident {
            Ident::Filename(_, name) => assert!(Path::new(&name) == file.path()),
            _ => panic!(),
        };
    }

    #[test]
    fn test_file() {
        let mut watcher = Watcher::new().expect("new failed");
        let mut file = tempfile::tempfile().expect("Could not create tempfile");

        watcher
            .add_file(&file, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("add failed");
        watcher.watch().expect("watch failed");
        file.write_all(b"foo").expect("write failed");

        thread::sleep(time::Duration::from_secs(1));

        let ev = watcher.iter().next().expect("Didn't get an event");

        assert!(matches!(ev.data, EventData::Vnode(Vnode::Write)));
        assert!(matches!(ev.ident, Ident::Fd(_)));
    }

    #[test]
    fn test_delete_filename() {
        let mut watcher = Watcher::new().expect("new failed");

        let file = tempfile::NamedTempFile::new().expect("Could not create tempfile");
        let filename = file.path();

        watcher
            .add_filename(filename, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("add failed");
        watcher.watch().expect("watch failed");
        watcher
            .remove_filename(filename, EventFilter::EVFILT_VNODE)
            .expect("delete failed");
    }

    #[test]
    fn test_dupe() {
        let mut watcher = Watcher::new().expect("new failed");
        let file = tempfile::NamedTempFile::new().expect("Couldn't create tempfile");
        let filename = file.path();

        watcher
            .add_filename(filename, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("add failed");
        watcher
            .add_filename(filename, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("second add failed");

        assert_eq!(
            watcher.watched.len(),
            1,
            "Did not get an expected number of events"
        );
    }

    #[test]
    fn test_two_files() {
        let mut watcher = Watcher::new().expect("new failed");

        let mut first_file = tempfile::tempfile().expect("Unable to create first temporary file");
        let mut second_file = tempfile::tempfile().expect("Unable to create second temporary file");

        watcher
            .add_file(
                &first_file,
                EventFilter::EVFILT_VNODE,
                FilterFlag::NOTE_WRITE,
            )
            .expect("add failed");

        watcher
            .add_file(
                &second_file,
                EventFilter::EVFILT_VNODE,
                FilterFlag::NOTE_WRITE,
            )
            .expect("add failed");

        watcher.watch().expect("watch failed");
        first_file.write_all(b"foo").expect("first write failed");
        second_file.write_all(b"foo").expect("second write failed");

        thread::sleep(time::Duration::from_secs(1));

        watcher.iter().next().expect("didn't get any events");
        watcher.iter().next().expect("didn't get any events");
    }

    #[test]
    fn test_nested_kqueue() {
        let mut watcher = Watcher::new().expect("Failed to create main watcher");
        let mut nested_watcher = Watcher::new().expect("Failed to create nested watcher");

        let kqueue_fd = nested_watcher.as_raw_fd();
        watcher
            .add_fd(kqueue_fd, EventFilter::EVFILT_READ, FilterFlag::empty())
            .expect("add_file failed for main watcher");

        let mut file = tempfile::tempfile().expect("Couldn't create tempfile");
        nested_watcher
            .add_file(&file, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("add_file failed for nested watcher");

        watcher.watch().expect("watch failed on main watcher");
        nested_watcher
            .watch()
            .expect("watch failed on nested watcher");

        file.write_all(b"foo").expect("write failed");
        file.flush().expect("flush failed");

        watcher
            .poll(Some(time::Duration::from_secs(2)))
            .expect("didn't get any events");
        nested_watcher
            .poll(Some(time::Duration::from_secs(2)))
            .expect("didn't get any events");
    }

    #[test]
    #[cfg(target_os = "freebsd")]
    fn test_close_read() {
        let mut watcher = Watcher::new().expect("new failed");

        {
            let file = tempfile::NamedTempFile::new().expect("temporary file failed to create");
            watcher
                .add_filename(
                    file.path(),
                    EventFilter::EVFILT_VNODE,
                    FilterFlag::NOTE_CLOSE,
                )
                .expect("add failed");
            watcher.watch().expect("watch failed");

            // we launch this in a separate process since it appears that FreeBSD does not fire
            // off a NOTE_CLOSE(_WRITE)? event for the same process closing a file descriptor.
            process::Command::new("cat")
                .arg(file.path())
                .spawn()
                .expect("should spawn a file")
                .wait()
                .expect("should exit successfully");
            thread::sleep(time::Duration::from_secs(1));
        }
        let ev = watcher.iter().next().expect("did not receive event");
        assert!(matches!(ev.data, EventData::Vnode(Vnode::Close)));
    }

    #[test]
    #[cfg(target_os = "freebsd")]
    fn test_close_write() {
        let mut watcher = match Watcher::new() {
            Ok(wat) => wat,
            Err(_) => panic!("new failed"),
        };

        {
            let file = tempfile::NamedTempFile::new().expect("couldn't create tempfile");
            watcher
                .add_filename(
                    file.path(),
                    EventFilter::EVFILT_VNODE,
                    FilterFlag::NOTE_CLOSE_WRITE,
                )
                .expect("add failed");
            watcher.watch().expect("watch failed");

            // See above for rationale as to why we use a separate process here
            process::Command::new("cat")
                .arg(file.path())
                .spawn()
                .expect("should spawn a file")
                .wait()
                .expect("should exit successfully");
            thread::sleep(time::Duration::from_secs(1));
        }
        let ev = watcher.iter().next().expect("didn't get an event");
        assert!(matches!(ev.data, EventData::Vnode(Vnode::CloseWrite)));
    }

    #[test]
    fn test_not_found_remove_watch() {
        let mut watcher = Watcher::new().unwrap();

        let ret = watcher.remove_filename("foo", EventFilter::EVFILT_VNODE);
        assert!(ret.is_err());

        let err = ret.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert_eq!(err.to_string(), "\"foo\" was not being watched");
    }

    #[test]
    fn test_double_close_file() {
        let mut watcher = Watcher::new().expect("couldn't create watcher");

        {
            let fil = File::open("/").expect("failed to open /");
            watcher
                .add_file(&fil, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_RENAME)
                .expect("couldn't add test file");
            watcher.watch().expect("failed to watch");
            watcher
                .remove_file(&fil, EventFilter::EVFILT_VNODE)
                .expect("couldn't remove file");
            watcher.watch().expect("failed to watch");
        }
    }

    #[test]
    fn test_double_close_file_drop() {
        let fil = File::open("/").expect("failed to open /");
        {
            let mut watcher = Watcher::new().expect("couldn't create watcher");
            watcher
                .add_file(&fil, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_RENAME)
                .expect("couldn't add test file");
            watcher.watch().expect("failed to watch");
        }
    }

    #[test]
    #[cfg(target_os = "freebsd")]
    fn test_read() {
        let mut contents = Vec::<u8>::new();
        let mut fil = tempfile::tempfile().expect("Unable to first temporary file");
        fil.write_all(b"foo bar baz").expect("Couldn't write");

        let mut watcher = Watcher::new().expect("couldn't create watcher");
        watcher
            .add_file(&fil, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_READ)
            .expect("couldn't add test file");
        watcher.watch().expect("failed to watch");

        let _ = fil.read_to_end(&mut contents);
        let _expected_text: Vec<u8> = b"foo bar baz".into();
        assert!(matches!(contents, _expected_text));
        let ev = watcher.iter().next().expect("didn't get an event");
        assert!(matches!(ev.data, EventData::Vnode(Vnode::Read)));
    }

    #[test]
    fn test_same_fd_different_filters() {
        let mut fil = tempfile::tempfile().expect("Unable to write temporary file");

        let mut watcher = Watcher::new().expect("couldn't create watcher");
        watcher
            .add_file(&fil, EventFilter::EVFILT_READ, FilterFlag::empty())
            .expect("couldn't add test file");
        watcher
            .add_file(&fil, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("couldn't add test file another time");
        watcher.watch().expect("failed to watch");

        let expected_written = 3;
        let actual_written = fil.write("foo".as_bytes()).expect("couldn't write");
        assert_eq!(
            actual_written, expected_written,
            "expected to write {expected_written} bytes, wrote: {actual_written}"
        );

        // Trigger the other filter on the same fd, this makes the
        // fd appear ready for more reading
        fil.seek(SeekFrom::Start(0)).expect("couldn't seek");

        let (mut rr_happened, mut write_happened) = (false, false);
        for _ in 0..2 {
            let ev = watcher.iter().next().expect("didn't get event");
            match ev.data {
                EventData::ReadReady(bytes) => {
                    assert_eq!(bytes, expected_written, "more than 3 bytes ready");
                    if rr_happened {
                        panic!("received {:?} more than once", ev);
                    }
                    rr_happened = true;
                }
                EventData::Vnode(Vnode::Write) => {
                    if write_happened {
                        panic!("received {:?} more than once", ev);
                    }
                    write_happened = true;
                }
                _ => panic!("got unexpected event: {:?}", ev),
            };
        }

        assert!(rr_happened, "did not get EventData::ReadReady");
        assert!(write_happened, "did not get EventData::Vnode(Vnode::Write)");
    }

    #[test]
    fn test_same_filename_different_filters() {
        let mut fil = tempfile::NamedTempFile::new().expect("Unable to write temporary file");

        let mut watcher = Watcher::new().expect("couldn't create watcher");
        watcher
            .add_filename(fil.path(), EventFilter::EVFILT_READ, FilterFlag::empty())
            .expect("couldn't add test file");
        watcher
            .add_filename(
                fil.path(),
                EventFilter::EVFILT_VNODE,
                FilterFlag::NOTE_WRITE,
            )
            .expect("couldn't add test file another time");
        watcher.watch().expect("failed to watch");

        let expected_written = 3;
        let actual_written = fil.write("foo".as_bytes()).expect("couldn't write");
        assert_eq!(
            actual_written, expected_written,
            "expected to write {expected_written} bytes, wrote: {actual_written}"
        );

        // We don't need a seek here since it's two different fd's
        let (mut rr_happened, mut write_happened) = (false, false);
        for _ in 0..2 {
            let ev = watcher.iter().next().expect("didn't get event");
            match ev.data {
                EventData::ReadReady(bytes) => {
                    assert_eq!(bytes, expected_written, "more than 3 bytes ready");
                    if rr_happened {
                        panic!("received {:?} more than once", ev);
                    }
                    rr_happened = true;
                }
                EventData::Vnode(Vnode::Write) => {
                    if write_happened {
                        panic!("received {:?} more than once", ev);
                    }
                    write_happened = true;
                }
                _ => panic!("got unexpected event: {:?}", ev),
            };
        }

        assert!(rr_happened, "did not get EventData::ReadReady");
        assert!(write_happened, "did not get EventData::Vnode(Vnode::Write)");
    }

    #[test]
    // disabled on macOS targets since HFS+ and APFS mandate utf-8 filenames
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    fn test_remove_non_utf8_filename() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let dir = tempfile::tempdir().expect("couldn't create tempdir");
        let path = dir.path().join(OsStr::from_bytes(b"bad\xff.txt"));

        File::create(&path).expect("could not file");

        let mut watcher = Watcher::new().expect("couldn't create watcher");
        watcher
            .add_filename(&path, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("add_filename should accept non-UTF-8 paths");
        watcher.watch().expect("watch failed");

        let result = watcher.remove_filename(&path, EventFilter::EVFILT_VNODE);
        assert!(
            result.is_ok(),
            "unexpected result from remove_filename: {result:?}",
        );
    }

    #[test]
    fn test_into_iter() {
        let mut watcher = Watcher::new().expect("new failed");
        let mut file = tempfile::tempfile().expect("Could not create tempfile");

        watcher
            .add_file(&file, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("add failed");
        watcher.watch().expect("watch failed");
        file.write_all(b"foo").expect("write failed");

        thread::sleep(time::Duration::from_millis(200));

        let ev = watcher.into_iter().next().expect("Didn't get an event");

        assert!(matches!(ev.data, EventData::Vnode(Vnode::Write)));
        assert!(matches!(ev.ident, Ident::Fd(_)));
    }
}
