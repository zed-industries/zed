#![cfg(target_os = "macos")]

use bitflags::bitflags;
use fsevent_sys::{self as fs, core_foundation as cf};
use parking_lot::Mutex;
use std::{
    convert::AsRef,
    ffi::{c_void, CStr, OsStr},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    slice,
    sync::Arc,
    time::Duration,
};

#[derive(Clone, Debug)]
pub struct Event {
    pub event_id: u64,
    pub flags: StreamFlags,
    pub path: PathBuf,
}

pub struct EventStream {
    stream: fs::FSEventStreamRef,
    lifecycle: Arc<Mutex<Lifecycle>>,
    state: Box<State>,
}

struct State {
    latency: Duration,
    paths: cf::CFMutableArrayRef,
    callback: Option<Box<dyn FnMut(Vec<Event>) -> bool>>,
    last_valid_event_id: Option<fs::FSEventStreamEventId>,
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            cf::CFRelease(self.paths);
        }
    }
}

enum Lifecycle {
    New,
    Running(cf::CFRunLoopRef),
    Stopped,
}

pub struct Handle(Arc<Mutex<Lifecycle>>);

unsafe impl Send for EventStream {}
unsafe impl Send for Lifecycle {}

impl EventStream {
    pub fn new(paths: &[&Path], latency: Duration) -> (Self, Handle) {
        unsafe {
            let cf_paths =
                cf::CFArrayCreateMutable(cf::kCFAllocatorDefault, 0, &cf::kCFTypeArrayCallBacks);
            assert!(!cf_paths.is_null());

            for path in paths {
                let path_bytes = path.as_os_str().as_bytes();
                let cf_url = cf::CFURLCreateFromFileSystemRepresentation(
                    cf::kCFAllocatorDefault,
                    path_bytes.as_ptr() as *const i8,
                    path_bytes.len() as cf::CFIndex,
                    false,
                );
                let cf_path = cf::CFURLCopyFileSystemPath(cf_url, cf::kCFURLPOSIXPathStyle);
                cf::CFArrayAppendValue(cf_paths, cf_path);
                cf::CFRelease(cf_path);
                cf::CFRelease(cf_url);
            }

            let state = Box::new(State {
                latency,
                paths: cf_paths,
                callback: None,
                last_valid_event_id: None,
            });
            let stream_context = fs::FSEventStreamContext {
                version: 0,
                info: state.as_ref() as *const _ as *mut c_void,
                retain: None,
                release: None,
                copy_description: None,
            };
            let stream = fs::FSEventStreamCreate(
                cf::kCFAllocatorDefault,
                Self::trampoline,
                &stream_context,
                cf_paths,
                FSEventsGetCurrentEventId(),
                latency.as_secs_f64(),
                fs::kFSEventStreamCreateFlagFileEvents
                    | fs::kFSEventStreamCreateFlagNoDefer
                    | fs::kFSEventStreamCreateFlagWatchRoot,
            );

            let lifecycle = Arc::new(Mutex::new(Lifecycle::New));
            (
                EventStream {
                    stream,
                    lifecycle: lifecycle.clone(),
                    state,
                },
                Handle(lifecycle),
            )
        }
    }

    pub fn run<F>(mut self, f: F)
    where
        F: FnMut(Vec<Event>) -> bool + 'static,
    {
        self.state.callback = Some(Box::new(f));
        unsafe {
            let run_loop = cf::CFRunLoopGetCurrent();
            {
                let mut state = self.lifecycle.lock();
                match *state {
                    Lifecycle::New => *state = Lifecycle::Running(run_loop),
                    Lifecycle::Running(_) => unreachable!(),
                    Lifecycle::Stopped => return,
                }
            }
            fs::FSEventStreamScheduleWithRunLoop(self.stream, run_loop, cf::kCFRunLoopDefaultMode);
            fs::FSEventStreamStart(self.stream);
            cf::CFRunLoopRun();
            fs::FSEventStreamRelease(self.stream);
        }
    }

    extern "C" fn trampoline(
        stream_ref: fs::FSEventStreamRef,
        info: *mut ::std::os::raw::c_void,
        num: usize,                                 // size_t numEvents
        event_paths: *mut ::std::os::raw::c_void,   // void *eventPaths
        event_flags: *const ::std::os::raw::c_void, // const FSEventStreamEventFlags eventFlags[]
        event_ids: *const ::std::os::raw::c_void,   // const FSEventStreamEventId eventIds[]
    ) {
        unsafe {
            let event_paths = event_paths as *const *const ::std::os::raw::c_char;
            let e_ptr = event_flags as *mut u32;
            let i_ptr = event_ids as *mut u64;
            let state = (info as *mut State).as_mut().unwrap();
            let callback = if let Some(callback) = state.callback.as_mut() {
                callback
            } else {
                return;
            };

            let paths = slice::from_raw_parts(event_paths, num);
            let flags = slice::from_raw_parts_mut(e_ptr, num);
            let ids = slice::from_raw_parts_mut(i_ptr, num);

            let mut events = Vec::with_capacity(num);
            for p in 0..num {
                let path_c_str = CStr::from_ptr(paths[p]);
                let path = PathBuf::from(OsStr::from_bytes(path_c_str.to_bytes()));
                if let Some(flag) = StreamFlags::from_bits(flags[p]) {
                    if flag.contains(StreamFlags::HISTORY_DONE) {
                        events.clear();
                    } else {
                        events.push(Event {
                            event_id: ids[p],
                            flags: flag,
                            path,
                        });
                    }
                } else {
                    debug_assert!(false, "unknown flag set for fs event: {}", flags[p]);
                }
            }

            if !events.is_empty() {
                if !callback(events) {
                    fs::FSEventStreamStop(stream_ref);
                    cf::CFRunLoopStop(cf::CFRunLoopGetCurrent());
                }
            }
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let mut state = self.0.lock();
        if let Lifecycle::Running(run_loop) = *state {
            unsafe {
                cf::CFRunLoopStop(run_loop);
            }
        }
        *state = Lifecycle::Stopped;
    }
}

// Synchronize with
// /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/FSEvents.framework/Versions/A/Headers/FSEvents.h
bitflags! {
  #[repr(C)]
  pub struct StreamFlags: u32 {
    const NONE = 0x00000000;
    const MUST_SCAN_SUBDIRS = 0x00000001;
    const USER_DROPPED = 0x00000002;
    const KERNEL_DROPPED = 0x00000004;
    const IDS_WRAPPED = 0x00000008;
    const HISTORY_DONE = 0x00000010;
    const ROOT_CHANGED = 0x00000020;
    const MOUNT = 0x00000040;
    const UNMOUNT = 0x00000080;
    const ITEM_CREATED = 0x00000100;
    const ITEM_REMOVED = 0x00000200;
    const INODE_META_MOD = 0x00000400;
    const ITEM_RENAMED = 0x00000800;
    const ITEM_MODIFIED = 0x00001000;
    const FINDER_INFO_MOD = 0x00002000;
    const ITEM_CHANGE_OWNER = 0x00004000;
    const ITEM_XATTR_MOD = 0x00008000;
    const IS_FILE = 0x00010000;
    const IS_DIR = 0x00020000;
    const IS_SYMLINK = 0x00040000;
    const OWN_EVENT = 0x00080000;
    const IS_HARDLINK = 0x00100000;
    const IS_LAST_HARDLINK = 0x00200000;
    const ITEM_CLONED = 0x400000;
  }
}

impl std::fmt::Display for StreamFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.contains(StreamFlags::MUST_SCAN_SUBDIRS) {
            let _d = write!(f, "MUST_SCAN_SUBDIRS ");
        }
        if self.contains(StreamFlags::USER_DROPPED) {
            let _d = write!(f, "USER_DROPPED ");
        }
        if self.contains(StreamFlags::KERNEL_DROPPED) {
            let _d = write!(f, "KERNEL_DROPPED ");
        }
        if self.contains(StreamFlags::IDS_WRAPPED) {
            let _d = write!(f, "IDS_WRAPPED ");
        }
        if self.contains(StreamFlags::HISTORY_DONE) {
            let _d = write!(f, "HISTORY_DONE ");
        }
        if self.contains(StreamFlags::ROOT_CHANGED) {
            let _d = write!(f, "ROOT_CHANGED ");
        }
        if self.contains(StreamFlags::MOUNT) {
            let _d = write!(f, "MOUNT ");
        }
        if self.contains(StreamFlags::UNMOUNT) {
            let _d = write!(f, "UNMOUNT ");
        }
        if self.contains(StreamFlags::ITEM_CREATED) {
            let _d = write!(f, "ITEM_CREATED ");
        }
        if self.contains(StreamFlags::ITEM_REMOVED) {
            let _d = write!(f, "ITEM_REMOVED ");
        }
        if self.contains(StreamFlags::INODE_META_MOD) {
            let _d = write!(f, "INODE_META_MOD ");
        }
        if self.contains(StreamFlags::ITEM_RENAMED) {
            let _d = write!(f, "ITEM_RENAMED ");
        }
        if self.contains(StreamFlags::ITEM_MODIFIED) {
            let _d = write!(f, "ITEM_MODIFIED ");
        }
        if self.contains(StreamFlags::FINDER_INFO_MOD) {
            let _d = write!(f, "FINDER_INFO_MOD ");
        }
        if self.contains(StreamFlags::ITEM_CHANGE_OWNER) {
            let _d = write!(f, "ITEM_CHANGE_OWNER ");
        }
        if self.contains(StreamFlags::ITEM_XATTR_MOD) {
            let _d = write!(f, "ITEM_XATTR_MOD ");
        }
        if self.contains(StreamFlags::IS_FILE) {
            let _d = write!(f, "IS_FILE ");
        }
        if self.contains(StreamFlags::IS_DIR) {
            let _d = write!(f, "IS_DIR ");
        }
        if self.contains(StreamFlags::IS_SYMLINK) {
            let _d = write!(f, "IS_SYMLINK ");
        }
        if self.contains(StreamFlags::OWN_EVENT) {
            let _d = write!(f, "OWN_EVENT ");
        }
        if self.contains(StreamFlags::IS_LAST_HARDLINK) {
            let _d = write!(f, "IS_LAST_HARDLINK ");
        }
        if self.contains(StreamFlags::IS_HARDLINK) {
            let _d = write!(f, "IS_HARDLINK ");
        }
        if self.contains(StreamFlags::ITEM_CLONED) {
            let _d = write!(f, "ITEM_CLONED ");
        }
        write!(f, "")
    }
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub fn FSEventsGetCurrentEventId() -> u64;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, sync::mpsc, thread, time::Duration};
    use tempdir::TempDir;

    #[test]
    fn test_event_stream_simple() {
        for _ in 0..3 {
            let dir = TempDir::new("test-event-stream").unwrap();
            let path = dir.path().canonicalize().unwrap();
            for i in 0..10 {
                fs::write(path.join(format!("existing-file-{}", i)), "").unwrap();
            }
            flush_historical_events();

            let (tx, rx) = mpsc::channel();
            let (stream, handle) = EventStream::new(&[&path], Duration::from_millis(50));
            thread::spawn(move || stream.run(move |events| tx.send(events.to_vec()).is_ok()));

            fs::write(path.join("new-file"), "").unwrap();
            let events = rx.recv_timeout(Duration::from_secs(2)).unwrap();
            let event = events.last().unwrap();
            assert_eq!(event.path, path.join("new-file"));
            assert!(event.flags.contains(StreamFlags::ITEM_CREATED));

            fs::remove_file(path.join("existing-file-5")).unwrap();
            let events = rx.recv_timeout(Duration::from_secs(2)).unwrap();
            let event = events.last().unwrap();
            assert_eq!(event.path, path.join("existing-file-5"));
            assert!(event.flags.contains(StreamFlags::ITEM_REMOVED));
            drop(handle);
        }
    }

    #[test]
    fn test_event_stream_delayed_start() {
        for _ in 0..3 {
            let dir = TempDir::new("test-event-stream").unwrap();
            let path = dir.path().canonicalize().unwrap();
            for i in 0..10 {
                fs::write(path.join(format!("existing-file-{}", i)), "").unwrap();
            }
            flush_historical_events();

            let (tx, rx) = mpsc::channel();
            let (stream, handle) = EventStream::new(&[&path], Duration::from_millis(50));

            // Delay the call to `run` in order to make sure we don't miss any events that occur
            // between creating the `EventStream` and calling `run`.
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));
                stream.run(move |events| tx.send(events.to_vec()).is_ok())
            });

            fs::write(path.join("new-file"), "").unwrap();
            let events = rx.recv_timeout(Duration::from_secs(2)).unwrap();
            let event = events.last().unwrap();
            assert_eq!(event.path, path.join("new-file"));
            assert!(event.flags.contains(StreamFlags::ITEM_CREATED));

            fs::remove_file(path.join("existing-file-5")).unwrap();
            let events = rx.recv_timeout(Duration::from_secs(2)).unwrap();
            let event = events.last().unwrap();
            assert_eq!(event.path, path.join("existing-file-5"));
            assert!(event.flags.contains(StreamFlags::ITEM_REMOVED));
            drop(handle);
        }
    }

    #[test]
    fn test_event_stream_shutdown_by_dropping_handle() {
        let dir = TempDir::new("test-event-stream").unwrap();
        let path = dir.path().canonicalize().unwrap();
        flush_historical_events();

        let (tx, rx) = mpsc::channel();
        let (stream, handle) = EventStream::new(&[&path], Duration::from_millis(50));
        thread::spawn(move || {
            stream.run({
                let tx = tx.clone();
                move |_| {
                    tx.send("running").unwrap();
                    true
                }
            });
            tx.send("stopped").unwrap();
        });

        fs::write(path.join("new-file"), "").unwrap();
        assert_eq!(rx.recv_timeout(Duration::from_secs(2)).unwrap(), "running");

        // Dropping the handle causes `EventStream::run` to return.
        drop(handle);
        assert_eq!(rx.recv_timeout(Duration::from_secs(2)).unwrap(), "stopped");
    }

    #[test]
    fn test_event_stream_shutdown_before_run() {
        let dir = TempDir::new("test-event-stream").unwrap();
        let path = dir.path().canonicalize().unwrap();

        let (stream, handle) = EventStream::new(&[&path], Duration::from_millis(50));
        drop(handle);

        // This returns immediately because the handle was already dropped.
        stream.run(|_| true);
    }

    fn flush_historical_events() {
        let duration = if std::env::var("CI").is_ok() {
            Duration::from_secs(2)
        } else {
            Duration::from_millis(500)
        };
        thread::sleep(duration);
    }
}
