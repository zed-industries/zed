use fsevent_sys::{self as fs, core_foundation as cf};
use parking_lot::Mutex;
use std::{
    convert::AsRef,
    ffi::{c_void, CStr, OsStr},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    ptr, slice,
    sync::Arc,
    time::Duration,
};

use crate::{Event, StreamFlags};

pub struct EventStream {
    lifecycle: Arc<Mutex<Lifecycle>>,
    state: Box<State>,
}

struct State {
    latency: Duration,
    paths: cf::CFMutableArrayRef,
    callback: Option<Box<dyn FnMut(Vec<Event>) -> bool>>,
    last_valid_event_id: Option<fs::FSEventStreamEventId>,
    stream: fs::FSEventStreamRef,
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            cf::CFRelease(self.paths);
            fs::FSEventStreamStop(self.stream);
            fs::FSEventStreamInvalidate(self.stream);
            fs::FSEventStreamRelease(self.stream);
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

            let mut state = Box::new(State {
                latency,
                paths: cf_paths,
                callback: None,
                last_valid_event_id: None,
                stream: ptr::null_mut(),
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
            state.stream = stream;

            let lifecycle = Arc::new(Mutex::new(Lifecycle::New));
            (
                EventStream {
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
            fs::FSEventStreamScheduleWithRunLoop(
                self.state.stream,
                run_loop,
                cf::kCFRunLoopDefaultMode,
            );
            fs::FSEventStreamStart(self.state.stream);
            cf::CFRunLoopRun();
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
            let mut stream_restarted = false;

            // Sometimes FSEvents reports a "dropped" event, an indication that either the kernel
            // or our code couldn't keep up with the sheer volume of file-system events that were
            // generated. If we observed a valid event before this happens, we'll try to read the
            // file-system journal by stopping the current stream and creating a new one starting at
            // such event. Otherwise, we'll let invoke the callback with the dropped event, which
            // will likely perform a re-scan of one of the root directories.
            if flags
                .iter()
                .copied()
                .filter_map(StreamFlags::from_bits)
                .any(|flags| {
                    flags.contains(StreamFlags::USER_DROPPED)
                        || flags.contains(StreamFlags::KERNEL_DROPPED)
                })
            {
                if let Some(last_valid_event_id) = state.last_valid_event_id.take() {
                    fs::FSEventStreamStop(state.stream);
                    fs::FSEventStreamInvalidate(state.stream);
                    fs::FSEventStreamRelease(state.stream);

                    let stream_context = fs::FSEventStreamContext {
                        version: 0,
                        info,
                        retain: None,
                        release: None,
                        copy_description: None,
                    };
                    let stream = fs::FSEventStreamCreate(
                        cf::kCFAllocatorDefault,
                        Self::trampoline,
                        &stream_context,
                        state.paths,
                        last_valid_event_id,
                        state.latency.as_secs_f64(),
                        fs::kFSEventStreamCreateFlagFileEvents
                            | fs::kFSEventStreamCreateFlagNoDefer
                            | fs::kFSEventStreamCreateFlagWatchRoot,
                    );

                    state.stream = stream;
                    fs::FSEventStreamScheduleWithRunLoop(
                        state.stream,
                        cf::CFRunLoopGetCurrent(),
                        cf::kCFRunLoopDefaultMode,
                    );
                    fs::FSEventStreamStart(state.stream);
                    stream_restarted = true;
                }
            }

            if !stream_restarted {
                let mut events = Vec::with_capacity(num);
                for p in 0..num {
                    if let Some(flag) = StreamFlags::from_bits(flags[p]) {
                        if !flag.contains(StreamFlags::HISTORY_DONE) {
                            let path_c_str = CStr::from_ptr(paths[p]);
                            let path = PathBuf::from(OsStr::from_bytes(path_c_str.to_bytes()));
                            let event = Event {
                                event_id: ids[p],
                                flags: flag,
                                path,
                            };
                            state.last_valid_event_id = Some(event.event_id);
                            events.push(event);
                        }
                    } else {
                        debug_assert!(false, "unknown flag set for fs event: {}", flags[p]);
                    }
                }

                if !events.is_empty() && !callback(events) {
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

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub fn FSEventsGetCurrentEventId() -> u64;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, sync::mpsc, thread, time::Duration};

    #[test]
    fn test_event_stream_simple() {
        for _ in 0..3 {
            let dir = tempfile::Builder::new()
                .prefix("test-event-stream")
                .tempdir()
                .unwrap();
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
            let dir = tempfile::Builder::new()
                .prefix("test-event-stream")
                .tempdir()
                .unwrap();
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
        let dir = tempfile::Builder::new()
            .prefix("test-event-stream")
            .tempdir()
            .unwrap();
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
        let dir = tempfile::Builder::new()
            .prefix("test-event-stream")
            .tempdir()
            .unwrap();
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
