use std::fmt::Debug;
use std::io::{self, Error};
use std::os::fd::AsRawFd;
use std::os::unix::io::RawFd;
use std::ptr;
use std::time::Duration;

use kqueue_sys::constants::{EventFilter, EventFlag, FilterFlag};
use kqueue_sys::kevent;

use crate::os::vnode;
use crate::time::duration_to_timespec;
use crate::types::{Ident, Proc, Vnode};
use crate::watcher::Watcher;

/// Event-specific data returned with the event.
///
/// Like much of this library, this is OS-specific. Check `kqueue(2)` for more
/// details on your target OS.
#[derive(Debug)]
pub enum EventData {
    /// Data relating to `Vnode` events
    Vnode(Vnode),

    /// Data relating to process events
    Proc(Proc),

    /// The returned number of bytes are ready for reading from the watched
    /// descriptor
    ReadReady(usize),

    /// The file is ready for writing. On some files (like sockets, pipes, etc),
    /// the number of bytes in the write buffer will be returned.
    WriteReady(usize),

    /// One of the watched signals fired. The number of times this signal was received
    /// is returned.
    Signal(usize),

    /// One of the watched timers fired. The number of times this timer fired
    /// is returned.
    Timer(usize),

    /// Some error was received
    Error(Error),
}

/// An event from a `Watcher` object.
///
/// An event contains both the a signifier of the watched object that triggered
/// the event, as well as any event-specific. See the `EventData` enum for info
/// on what event-specific data is returned for each event.
#[derive(Debug)]
pub struct Event {
    /// The watched resource that triggered the event
    pub ident: Ident,

    /// Any event-specific data returned with the event.
    pub data: EventData,
}

pub struct EventIter<'a> {
    pub(crate) watcher: &'a Watcher,
}

fn find_file_ident(watcher: &Watcher, fd: RawFd) -> Option<Ident> {
    for (ident, _) in watcher.watched.keys() {
        match &ident {
            Ident::Fd(ident_fd) => {
                if fd == *ident_fd {
                    return Some(Ident::Fd(fd));
                }
            }
            Ident::Filename(ident_fd, ident_str) if fd == *ident_fd => {
                return Some(Ident::Filename(*ident_fd, ident_str.clone()));
            }
            _ => (),
        }
    }

    None
}

pub(crate) fn get_event(watcher: &Watcher, timeout: Option<Duration>) -> Option<Event> {
    // If we're not started, then we can immediately return `None`.
    // It might be better to return an error, however that's API-breaking.
    // The behavioral difference here is that we will not wait the full
    // timeout when we don't haven't called `Watcher.watch`.
    if !watcher.started {
        return None;
    }

    let mut kev = kevent::new(
        0,
        EventFilter::EVFILT_SYSCOUNT,
        EventFlag::empty(),
        FilterFlag::empty(),
    );
    let ret = if let Some(ts) = timeout {
        unsafe {
            kevent(
                watcher.queue.as_raw_fd(),
                ptr::null(),
                0,
                &raw mut kev,
                1,
                &duration_to_timespec(ts),
            )
        }
    } else {
        unsafe {
            kevent(
                watcher.queue.as_raw_fd(),
                ptr::null(),
                0,
                &raw mut kev,
                1,
                ptr::null(),
            )
        }
    };

    match ret {
        -1 => Some(Event::from_error(kev, watcher)),
        0 => None, // timeout expired
        _ => Some(Event::new(kev, watcher)),
    }
}

fn new_error_event(ident: i32, kind: io::ErrorKind, msg: String) -> Event {
    Event {
        // I don't want to use a sentinel value here, but the current api constraints force it
        // In the future this should be replaced with an Option<Ident>
        ident: Ident::Fd(ident),
        data: EventData::Error(Error::new(kind, msg)),
    }
}

// OS specific
// TODO: Events can have more than one filter flag
impl Event {
    #[doc(hidden)]
    #[must_use]
    pub fn new(ev: kevent, watcher: &Watcher) -> Event {
        // unwrap is safe because i32::MAX is a constant
        if ev.ident > i32::MAX.try_into().unwrap() {
            return new_error_event(
                -1,
                io::ErrorKind::InvalidData,
                format!("received invalid ident: {:?}", ev.ident),
            );
        }

        // we've already checked this above
        #[allow(clippy::cast_possible_wrap)]
        #[allow(clippy::cast_possible_truncation)]
        let rawident: i32 = ev.ident as i32;
        let ident = match ev.filter {
            EventFilter::EVFILT_READ | EventFilter::EVFILT_WRITE | EventFilter::EVFILT_VNODE => {
                let Some(ident) = find_file_ident(watcher, rawident) else {
                    return new_error_event(
                        rawident,
                        io::ErrorKind::NotFound,
                        format!("received event for unwatched fd: {}", ev.ident),
                    );
                };
                ident
            }
            EventFilter::EVFILT_SIGNAL => Ident::Signal(rawident),
            EventFilter::EVFILT_TIMER => Ident::Timer(rawident),
            EventFilter::EVFILT_PROC => Ident::Pid(rawident),
            _ => {
                return new_error_event(
                    rawident,
                    io::ErrorKind::InvalidData,
                    format!("eventfilter not supported: {:?}", ev.filter),
                )
            }
        };

        // this is a much bigger problem that needs to be handled in v2. ev.data
        // can be negative in some cases, so having it be a usize was a bad
        // implementation. for now, we allow the clippy lints and we'll deal
        // with this v2.
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let data = match ev.filter {
            EventFilter::EVFILT_READ => EventData::ReadReady(ev.data as usize),
            EventFilter::EVFILT_WRITE => EventData::WriteReady(ev.data as usize),
            EventFilter::EVFILT_SIGNAL => EventData::Signal(ev.data as usize),
            EventFilter::EVFILT_TIMER => EventData::Timer(ev.data as usize),
            EventFilter::EVFILT_PROC => {
                let inner = if ev.fflags.contains(FilterFlag::NOTE_EXIT) {
                    Proc::Exit(ev.data as usize)
                } else if ev.fflags.contains(FilterFlag::NOTE_FORK) {
                    Proc::Fork
                } else if ev.fflags.contains(FilterFlag::NOTE_EXEC) {
                    Proc::Exec
                } else if ev.fflags.contains(FilterFlag::NOTE_TRACK) {
                    Proc::Track(ev.data as libc::pid_t)
                } else if ev.fflags.contains(FilterFlag::NOTE_CHILD) {
                    Proc::Child(ev.data as libc::pid_t)
                } else {
                    return Event {
                        ident,
                        data: EventData::Error(Error::new(
                            io::ErrorKind::InvalidData,
                            format!("proc filterflag not supported: {:?}", ev.fflags),
                        )),
                    };
                };

                EventData::Proc(inner)
            }
            EventFilter::EVFILT_VNODE => {
                let inner = if ev.fflags.contains(FilterFlag::NOTE_DELETE) {
                    Vnode::Delete
                } else if ev.fflags.contains(FilterFlag::NOTE_WRITE) {
                    Vnode::Write
                } else if ev.fflags.contains(FilterFlag::NOTE_EXTEND) {
                    Vnode::Extend
                } else if ev.fflags.contains(FilterFlag::NOTE_ATTRIB) {
                    Vnode::Attrib
                } else if ev.fflags.contains(FilterFlag::NOTE_LINK) {
                    Vnode::Link
                } else if ev.fflags.contains(FilterFlag::NOTE_RENAME) {
                    Vnode::Rename
                } else if ev.fflags.contains(FilterFlag::NOTE_REVOKE) {
                    Vnode::Revoke
                } else {
                    // This handles any filter flags that are OS-specific
                    let Some(vnode) = vnode::handle_vnode_extras(ev.fflags) else {
                        return Event {
                            ident,
                            data: EventData::Error(Error::new(
                                io::ErrorKind::InvalidData,
                                format!("vnode filterflag not supported: {:?}", ev.fflags),
                            )),
                        };
                    };

                    vnode
                };

                EventData::Vnode(inner)
            }
            _ => EventData::Error(Error::new(
                io::ErrorKind::InvalidData,
                format!("eventfilter not supported: {:?}", ev.filter),
            )),
        };

        Event { ident, data }
    }

    #[doc(hidden)]
    #[must_use]
    pub fn from_error(ev: kevent, watcher: &Watcher) -> Event {
        let last_os_err = io::Error::last_os_error();
        // unwrap is safe because i32::MAX is a constant
        let rawident = if ev.ident <= i32::MAX.try_into().unwrap() {
            // we've checked that this conversion is safe
            i32::try_from(ev.ident).unwrap()
        } else {
            // fill in with a sentinel because we don't
            // want to obliterate the original error
            -1
        };

        let ident = match ev.filter {
            EventFilter::EVFILT_READ | EventFilter::EVFILT_WRITE | EventFilter::EVFILT_VNODE => {
                let Some(ident) = find_file_ident(watcher, rawident) else {
                    return new_error_event(
                        rawident,
                        last_os_err.kind(),
                        format!(
                            "received event for unwatched fd: {} while reporting OS error: {}",
                            ev.ident, last_os_err,
                        ),
                    );
                };
                ident
            }
            EventFilter::EVFILT_SIGNAL => Ident::Signal(rawident),
            EventFilter::EVFILT_TIMER => Ident::Timer(rawident),
            EventFilter::EVFILT_PROC => Ident::Pid(rawident),
            _ => {
                return new_error_event(
                    rawident,
                    last_os_err.kind(),
                    format!(
                        "eventfilter not supported: {:?} while reporting OS error: {}",
                        ev.filter, last_os_err,
                    ),
                )
            }
        };

        Event {
            data: EventData::Error(last_os_err),
            ident,
        }
    }

    #[doc(hidden)]
    #[must_use]
    pub fn is_err(&self) -> bool {
        matches!(self.data, EventData::Error(_))
    }
}

impl Iterator for EventIter<'_> {
    type Item = Event;

    // rather than call kevent(2) each time, we can likely optimize and
    // call it once for like 100 items
    fn next(&mut self) -> Option<Self::Item> {
        if !self.watcher.started {
            return None;
        }

        get_event(self.watcher, None)
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use std::os::unix::io::AsRawFd;
    use std::time::{Duration, Instant};

    use kqueue_sys::constants::*;
    use kqueue_sys::kevent;

    use crate::{get_event, Event, EventData, Ident, Watcher};

    #[test]
    fn test_unknown_proc_filter_flag() {
        let ev = Event::new(
            kevent::new(
                0,
                EventFilter::EVFILT_PROC,
                EventFlag::all(),
                FilterFlag::NOTE_WRITE,
            ),
            &Watcher::new().expect("Could not create watcher"),
        );

        assert!(ev.is_err());
        let EventData::Error(err) = ev.data else {
            panic!("did not unpack an error");
        };

        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(
            err.to_string()
                .starts_with("proc filterflag not supported: "),
            "Got \"{}\"",
            err,
        );
    }

    #[test]
    fn test_unknown_vnode_filter_flag() {
        let tmp = tempfile::tempfile().expect("couldn't create tempfile");
        let mut watcher = Watcher::new().expect("could not create watcher");
        watcher
            .add_file(&tmp, EventFilter::EVFILT_VNODE, FilterFlag::NOTE_WRITE)
            .expect("could not watch fd 1");
        watcher.watch().expect("could not watch");
        let ev = Event::new(
            kevent::new(
                tmp.as_raw_fd() as usize,
                EventFilter::EVFILT_VNODE,
                EventFlag::all(),
                FilterFlag::NOTE_EXIT,
            ),
            &watcher,
        );

        assert!(ev.is_err());
        let EventData::Error(err) = ev.data else {
            panic!("did not unpack an error");
        };

        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(
            err.to_string()
                .starts_with("vnode filterflag not supported: "),
            "Got \"{}\"",
            err,
        );
    }

    #[test]
    fn test_unknown_event_filter() {
        let ev = Event::new(
            kevent::new(
                0,
                EventFilter::EVFILT_SYSCOUNT,
                EventFlag::all(),
                FilterFlag::all(),
            ),
            &Watcher::new().expect("Could not create watcher"),
        );

        assert!(ev.is_err());
        let EventData::Error(err) = ev.data else {
            panic!("did not unpack an error");
        };

        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(
            err.to_string().as_str(),
            "eventfilter not supported: EVFILT_SYSCOUNT",
        );
    }

    #[test]
    fn test_unknown_file_ident() {
        let ev = Event::new(
            kevent::new(
                0,
                EventFilter::EVFILT_READ,
                EventFlag::all(),
                FilterFlag::all(),
            ),
            &Watcher::new().expect("Could not create watcher"),
        );

        assert!(ev.is_err());
        let EventData::Error(err) = ev.data else {
            panic!("did not unpack error");
        };
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert_eq!(
            err.to_string().as_str(),
            "received event for unwatched fd: 0",
        )
    }

    #[test]
    fn test_unknown_event_filter_from_error() {
        let ev = Event::from_error(
            kevent::new(
                0,
                EventFilter::EVFILT_SYSCOUNT,
                EventFlag::all(),
                FilterFlag::all(),
            ),
            &Watcher::new().expect("could not create watcher"),
        );

        assert!(ev.is_err());
        let EventData::Error(err) = ev.data else {
            panic!("did not unpack an error");
        };

        assert_ne!(
            err.kind(),
            ErrorKind::InvalidData,
            "err.kind(): {:?}",
            err.kind()
        );
        assert!(err
            .to_string()
            .starts_with("eventfilter not supported: EVFILT_SYSCOUNT while reporting OS error:"),
                "did not start with \"eventfilter not supported: EVFILT_SYSCOUNT while reporting OS error:\" got \"{}\"",
                err,
        );
        assert!(
            err.to_string().ends_with("(os error 0)"),
            "did not start with \"(os error 0)\""
        );
    }

    #[test]
    fn test_unknown_file_ident_from_error() {
        let ev = Event::from_error(
            kevent::new(
                0,
                EventFilter::EVFILT_READ,
                EventFlag::all(),
                FilterFlag::all(),
            ),
            &Watcher::new().expect("could not create watcher"),
        );

        assert!(ev.is_err());
        let EventData::Error(err) = ev.data else {
            panic!("did not unpack an error");
        };

        assert_ne!(
            err.kind(),
            ErrorKind::NotFound,
            "err.kind(): {:?}",
            err.kind()
        );
        assert!(err
            .to_string()
            .starts_with("received event for unwatched fd: 0 while reporting OS error: "),
                "did not start with \"received event for unwatched fd: 0 while reporting OS error: \", got \"{}\"",
                err,
        );
        assert!(
            err.to_string().ends_with("(os error 0)"),
            "did not start with \"(os error 0)\""
        );
    }

    #[test]
    fn test_not_started() {
        let watcher = Watcher::new().expect("could not create watcher");
        let start = Instant::now();
        let ev = get_event(&watcher, Some(Duration::from_secs(5)));
        assert!(
            start.elapsed() < Duration::from_secs(4),
            "more than 4 seconds has elapsed, implying that we timed out"
        );
        assert!(
            ev.is_none(),
            "Got a `Some(event)` rather than a `None`: {:?}",
            ev,
        );
    }

    #[test]
    fn test_new_event_ident_too_big() {
        let watcher = Watcher::new().expect("could not create watcher");
        let ev = Event::new(
            kevent::new(
                usize::try_from(i32::MAX).unwrap() + 1,
                EventFilter::EVFILT_AIO,
                EventFlag::EV_ADD,
                FilterFlag::empty(),
            ),
            &watcher,
        );

        assert!(ev.is_err());
        let EventData::Error(err) = ev.data else {
            panic!("did not unpack an error");
        };
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "received invalid ident: 2147483648");
    }

    #[test]
    fn test_error_event_ident_too_big() {
        let watcher = Watcher::new().expect("could not create watcher");
        let ev = Event::from_error(
            kevent::new(
                usize::try_from(i32::MAX).unwrap() + 1,
                EventFilter::EVFILT_READ,
                EventFlag::EV_ADD,
                FilterFlag::empty(),
            ),
            &watcher,
        );

        assert!(ev.is_err());
        let Ident::Fd(inner) = ev.ident else {
            panic!("got a weird ev.ident: {:?}", ev.ident);
        };

        assert_eq!(inner, -1, "got an unexpected ident value: {inner}");
    }
}
