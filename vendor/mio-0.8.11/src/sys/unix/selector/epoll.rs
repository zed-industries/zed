use crate::{Interest, Token};

use libc::{EPOLLET, EPOLLIN, EPOLLOUT, EPOLLPRI, EPOLLRDHUP};
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(debug_assertions)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::{cmp, i32, io, ptr};

/// Unique id for use as `SelectorId`.
#[cfg(debug_assertions)]
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub struct Selector {
    #[cfg(debug_assertions)]
    id: usize,
    ep: RawFd,
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        #[cfg(not(target_os = "android"))]
        let res = syscall!(epoll_create1(libc::EPOLL_CLOEXEC));

        // On Android < API level 16 `epoll_create1` is not defined, so use a
        // raw system call.
        // According to libuv, `EPOLL_CLOEXEC` is not defined on Android API <
        // 21. But `EPOLL_CLOEXEC` is an alias for `O_CLOEXEC` on that platform,
        // so we use it instead.
        #[cfg(target_os = "android")]
        let res = syscall!(syscall(libc::SYS_epoll_create1, libc::O_CLOEXEC));

        let ep = match res {
            Ok(ep) => ep as RawFd,
            Err(err) => {
                // When `epoll_create1` is not available fall back to use
                // `epoll_create` followed by `fcntl`.
                if let Some(libc::ENOSYS) = err.raw_os_error() {
                    match syscall!(epoll_create(1024)) {
                        Ok(ep) => match syscall!(fcntl(ep, libc::F_SETFD, libc::FD_CLOEXEC)) {
                            Ok(ep) => ep as RawFd,
                            Err(err) => {
                                // `fcntl` failed, cleanup `ep`.
                                let _ = unsafe { libc::close(ep) };
                                return Err(err);
                            }
                        },
                        Err(err) => return Err(err),
                    }
                } else {
                    return Err(err);
                }
            }
        };

        Ok(Selector {
            #[cfg(debug_assertions)]
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            ep,
        })
    }

    pub fn try_clone(&self) -> io::Result<Selector> {
        syscall!(fcntl(self.ep, libc::F_DUPFD_CLOEXEC, super::LOWEST_FD)).map(|ep| Selector {
            // It's the same selector, so we use the same id.
            #[cfg(debug_assertions)]
            id: self.id,
            ep,
        })
    }

    pub fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        // A bug in kernels < 2.6.37 makes timeouts larger than LONG_MAX / CONFIG_HZ
        // (approx. 30 minutes with CONFIG_HZ=1200) effectively infinite on 32 bits
        // architectures. The magic number is the same constant used by libuv.
        #[cfg(target_pointer_width = "32")]
        const MAX_SAFE_TIMEOUT: u128 = 1789569;
        #[cfg(not(target_pointer_width = "32"))]
        const MAX_SAFE_TIMEOUT: u128 = libc::c_int::max_value() as u128;

        let timeout = timeout
            .map(|to| {
                // `Duration::as_millis` truncates, so round up. This avoids
                // turning sub-millisecond timeouts into a zero timeout, unless
                // the caller explicitly requests that by specifying a zero
                // timeout.
                let to_ms = to
                    .checked_add(Duration::from_nanos(999_999))
                    .unwrap_or(to)
                    .as_millis();
                cmp::min(MAX_SAFE_TIMEOUT, to_ms) as libc::c_int
            })
            .unwrap_or(-1);

        events.clear();
        syscall!(epoll_wait(
            self.ep,
            events.as_mut_ptr(),
            events.capacity() as i32,
            timeout,
        ))
        .map(|n_events| {
            // This is safe because `epoll_wait` ensures that `n_events` are
            // assigned.
            unsafe { events.set_len(n_events as usize) };
        })
    }

    pub fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events: interests_to_epoll(interests),
            u64: usize::from(token) as u64,
            #[cfg(target_os = "redox")]
            _pad: 0,
        };

        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_ADD, fd, &mut event)).map(|_| ())
    }

    pub fn reregister(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events: interests_to_epoll(interests),
            u64: usize::from(token) as u64,
            #[cfg(target_os = "redox")]
            _pad: 0,
        };

        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_MOD, fd, &mut event)).map(|_| ())
    }

    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_DEL, fd, ptr::null_mut())).map(|_| ())
    }
}

cfg_io_source! {
    impl Selector {
        #[cfg(debug_assertions)]
        pub fn id(&self) -> usize {
            self.id
        }
    }
}

impl AsRawFd for Selector {
    fn as_raw_fd(&self) -> RawFd {
        self.ep
    }
}

impl Drop for Selector {
    fn drop(&mut self) {
        if let Err(err) = syscall!(close(self.ep)) {
            error!("error closing epoll: {}", err);
        }
    }
}

fn interests_to_epoll(interests: Interest) -> u32 {
    let mut kind = EPOLLET;

    if interests.is_readable() {
        kind = kind | EPOLLIN | EPOLLRDHUP;
    }

    if interests.is_writable() {
        kind |= EPOLLOUT;
    }

    if interests.is_priority() {
        kind |= EPOLLPRI;
    }

    kind as u32
}

pub type Event = libc::epoll_event;
pub type Events = Vec<Event>;

pub mod event {
    use std::fmt;

    use crate::sys::Event;
    use crate::Token;

    pub fn token(event: &Event) -> Token {
        Token(event.u64 as usize)
    }

    pub fn is_readable(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLIN) != 0
            || (event.events as libc::c_int & libc::EPOLLPRI) != 0
    }

    pub fn is_writable(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLOUT) != 0
    }

    pub fn is_error(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLERR) != 0
    }

    pub fn is_read_closed(event: &Event) -> bool {
        // Both halves of the socket have closed
        event.events as libc::c_int & libc::EPOLLHUP != 0
            // Socket has received FIN or called shutdown(SHUT_RD)
            || (event.events as libc::c_int & libc::EPOLLIN != 0
                && event.events as libc::c_int & libc::EPOLLRDHUP != 0)
    }

    pub fn is_write_closed(event: &Event) -> bool {
        // Both halves of the socket have closed
        event.events as libc::c_int & libc::EPOLLHUP != 0
            // Unix pipe write end has closed
            || (event.events as libc::c_int & libc::EPOLLOUT != 0
                && event.events as libc::c_int & libc::EPOLLERR != 0)
            // The other side (read end) of a Unix pipe has closed.
            || event.events as libc::c_int == libc::EPOLLERR
    }

    pub fn is_priority(event: &Event) -> bool {
        (event.events as libc::c_int & libc::EPOLLPRI) != 0
    }

    pub fn is_aio(_: &Event) -> bool {
        // Not supported in the kernel, only in libc.
        false
    }

    pub fn is_lio(_: &Event) -> bool {
        // Not supported.
        false
    }

    pub fn debug_details(f: &mut fmt::Formatter<'_>, event: &Event) -> fmt::Result {
        #[allow(clippy::trivially_copy_pass_by_ref)]
        fn check_events(got: &u32, want: &libc::c_int) -> bool {
            (*got as libc::c_int & want) != 0
        }
        debug_detail!(
            EventsDetails(u32),
            check_events,
            libc::EPOLLIN,
            libc::EPOLLPRI,
            libc::EPOLLOUT,
            libc::EPOLLRDNORM,
            libc::EPOLLRDBAND,
            libc::EPOLLWRNORM,
            libc::EPOLLWRBAND,
            libc::EPOLLMSG,
            libc::EPOLLERR,
            libc::EPOLLHUP,
            libc::EPOLLET,
            libc::EPOLLRDHUP,
            libc::EPOLLONESHOT,
            #[cfg(target_os = "linux")]
            libc::EPOLLEXCLUSIVE,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            libc::EPOLLWAKEUP,
            libc::EPOLL_CLOEXEC,
        );

        // Can't reference fields in packed structures.
        let e_u64 = event.u64;
        f.debug_struct("epoll_event")
            .field("events", &EventsDetails(event.events))
            .field("u64", &e_u64)
            .finish()
    }
}

#[cfg(target_os = "android")]
#[test]
fn assert_close_on_exec_flag() {
    // This assertion need to be true for Selector::new.
    assert_eq!(libc::O_CLOEXEC, libc::EPOLL_CLOEXEC);
}
