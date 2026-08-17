//! Pseudoterminal operations.
//!
//! # References
//!
//!  - [Linux]
//!  - [FreeBSD]
//!
//! [Linux]: https://man7.org/linux/man-pages/man7/pty.7.html
//! [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=pty&sektion=4

#![no_std]

extern crate alloc;

#[cfg(any(target_os = "android", target_os = "linux"))]
use alloc::vec::Vec;
use rustix::fd::{AsFd, OwnedFd};
#[cfg(any(target_os = "android", target_os = "linux"))] // for `RawDir`
use rustix::fd::{AsRawFd, RawFd};
use rustix::io;
use rustix::termios::{Termios, Winsize};

// Re-export our public dependency on rustix.
pub use rustix;

/// A pair of file descriptors representing a pseudoterminal.
pub struct Pty {
    /// The controller of the pseudoterminal.
    pub controller: OwnedFd,

    /// The user side of the pseudoterminal that applications can connect
    /// to and be controlled by.
    pub user: OwnedFd,
}

/// Open a pseudoterminal.
///
/// The `termios` and `winsize` arguments specify `Termios` and `Winsize`
/// settings to configure the user file descriptor with.
///
/// The returned file descriptors have the `CLOEXEC` flag set, though not all
/// platforms supporting setting it atomically.
///
/// On many platforms, this includes a call to [`libc::grantpt`], which has
/// unspecified behavior if the calling process has a `SIGCHLD` signal handler
/// installed.
///
/// # References
///  - [Linux]
///  - [Apple]
///  - [FreeBSD]
///  - [glibc]
///
/// [Linux]: https://man7.org/linux/man-pages/man3/openpty.3.html
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/openpty.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=openpty&sektion=3
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Pseudo_002dTerminal-Pairs.html#index-openpty
/// [`libc::grantpt`]: https://docs.rs/libc/latest/libc/fn.grantpt.html
pub fn openpty(termios: Option<&Termios>, winsize: Option<&Winsize>) -> io::Result<Pty> {
    // On non-Linux platforms, use `libc::openpty`. This doesn't have any way
    // to set `CLOEXEC` so we do it non-atomically.
    #[cfg(not(any(target_os = "android", target_os = "linux")))]
    {
        use core::mem::{align_of, size_of, MaybeUninit};
        use core::ptr::{null, null_mut};
        use rustix::fd::FromRawFd;

        assert_eq!(size_of::<Termios>(), size_of::<libc::termios>());
        assert_eq!(align_of::<Termios>(), align_of::<libc::termios>());

        let termios: *const libc::termios = match termios {
            Some(termios) => {
                let termios: *const Termios = termios;
                termios.cast()
            }
            None => null(),
        };
        let mut libc_winsize: libc::winsize;
        let winsize: *mut libc::winsize = match winsize {
            Some(winsize) => {
                libc_winsize = libc::winsize {
                    ws_row: winsize.ws_row,
                    ws_col: winsize.ws_col,
                    ws_xpixel: winsize.ws_xpixel,
                    ws_ypixel: winsize.ws_ypixel,
                };
                &mut libc_winsize
            }
            None => null_mut(),
        };

        let mut controller = MaybeUninit::<libc::c_int>::uninit();
        let mut user = MaybeUninit::<libc::c_int>::uninit();
        unsafe {
            if libc::openpty(
                controller.as_mut_ptr(),
                user.as_mut_ptr(),
                null_mut(),
                termios as _,
                winsize,
            ) == 0
            {
                let controller = OwnedFd::from_raw_fd(controller.assume_init());
                let user = OwnedFd::from_raw_fd(user.assume_init());

                set_cloexec(&controller)?;
                set_cloexec(&user)?;

                Ok(Pty { controller, user })
            } else {
                Err(io::Errno::from_raw_os_error(errno::errno().0))
            }
        }
    }

    // On Linux platforms, use `rustix::pty`. Linux has an `openpty` function,
    // but we use `rustix::pty` instead so that we can set the `CLOEXEC` flag
    // atomically.
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        use rustix::pty::{grantpt, openpt, unlockpt, OpenptFlags};
        use rustix::termios::{tcsetattr, tcsetwinsize, OptionalActions};

        let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC;
        let controller = openpt(flags)?;

        grantpt(&controller)?;
        unlockpt(&controller)?;

        let user = open_user(&controller, flags | OpenptFlags::CLOEXEC)?;

        if let Some(termios) = termios {
            tcsetattr(&user, OptionalActions::Now, termios)?;
        }
        if let Some(winsize) = winsize {
            tcsetwinsize(&user, *winsize)?;
        }

        Ok(Pty { controller, user })
    }
}

/// Open a pseudoterminal, without setting `OFlags::CLOEXEC`.
///
/// This is identical to [`openpty`], except that it does not set
/// `OFlags::CLOEXEC` on the controller file descriptor. This more closely
/// matches how `libc::openpty` works.
pub fn openpty_nocloexec(termios: Option<&Termios>, winsize: Option<&Winsize>) -> io::Result<Pty> {
    // On non-Linux platforms, use `libc::openpty`.
    #[cfg(not(any(target_os = "android", target_os = "linux")))]
    {
        use core::mem::{align_of, size_of, MaybeUninit};
        use core::ptr::{null, null_mut};
        use rustix::fd::FromRawFd;

        assert_eq!(size_of::<Termios>(), size_of::<libc::termios>());
        assert_eq!(align_of::<Termios>(), align_of::<libc::termios>());

        let termios: *const libc::termios = match termios {
            Some(termios) => {
                let termios: *const Termios = termios;
                termios.cast()
            }
            None => null(),
        };
        let mut libc_winsize: libc::winsize;
        let winsize: *mut libc::winsize = match winsize {
            Some(winsize) => {
                libc_winsize = libc::winsize {
                    ws_row: winsize.ws_row,
                    ws_col: winsize.ws_col,
                    ws_xpixel: winsize.ws_xpixel,
                    ws_ypixel: winsize.ws_ypixel,
                };
                &mut libc_winsize
            }
            None => null_mut(),
        };

        let mut controller = MaybeUninit::<libc::c_int>::uninit();
        let mut user = MaybeUninit::<libc::c_int>::uninit();
        unsafe {
            if libc::openpty(
                controller.as_mut_ptr(),
                user.as_mut_ptr(),
                null_mut(),
                termios as _,
                winsize,
            ) == 0
            {
                let controller = OwnedFd::from_raw_fd(controller.assume_init());
                let user = OwnedFd::from_raw_fd(user.assume_init());

                Ok(Pty { controller, user })
            } else {
                Err(io::Errno::from_raw_os_error(errno::errno().0))
            }
        }
    }

    // On Linux platforms, use `rustix::pty`.
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        use rustix::pty::{grantpt, openpt, unlockpt, OpenptFlags};
        use rustix::termios::{tcsetattr, tcsetwinsize, OptionalActions};

        let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY;
        let controller = openpt(flags)?;

        grantpt(&controller)?;
        unlockpt(&controller)?;

        let user = open_user(&controller, flags)?;

        if let Some(termios) = termios {
            tcsetattr(&user, OptionalActions::Now, termios)?;
        }
        if let Some(winsize) = winsize {
            tcsetwinsize(&user, *winsize)?;
        }

        Ok(Pty { controller, user })
    }
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn set_cloexec<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    use rustix::io::{fcntl_getfd, fcntl_setfd, FdFlags};

    let fd = fd.as_fd();
    fcntl_setfd(fd, fcntl_getfd(fd)? | FdFlags::CLOEXEC)
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn open_user(controller: &OwnedFd, flags: rustix::pty::OpenptFlags) -> io::Result<OwnedFd> {
    use rustix::fs::{openat, Mode, CWD};

    // On Linux 4.13, we can use `ioctl_tiocgptpeer` as an optimization. But
    // don't try this on Android because Android's seccomp kills processes that
    // try to optimize.
    #[cfg(not(target_os = "android"))]
    {
        match rustix::pty::ioctl_tiocgptpeer(controller, flags) {
            Ok(fd) => return Ok(fd),
            Err(io::Errno::NOSYS) | Err(io::Errno::PERM) => {}
            Err(e) => return Err(e),
        }
    }

    // Get the user device file name and open it.
    let name = rustix::pty::ptsname(controller, Vec::new())?;

    openat(CWD, name, flags.into(), Mode::empty())
}

/// Prepare for a login on the given terminal.
///
/// # References
///  - [Linux]
///  - [Apple]
///  - [FreeBSD]
///
/// [Linux]: https://man7.org/linux/man-pages/man3/login_tty.3.html
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/login_tty.3.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=login_tty&sektion=3
#[cfg(not(any(target_os = "fuchsia", target_os = "illumos", target_os = "solaris")))]
pub fn login_tty<Fd: Into<OwnedFd>>(fd: Fd) -> io::Result<()> {
    _login_tty(fd.into())
}

#[cfg(not(any(target_os = "fuchsia", target_os = "illumos", target_os = "solaris")))]
fn _login_tty(fd: OwnedFd) -> io::Result<()> {
    #[cfg(not(any(target_os = "android", target_os = "linux")))]
    unsafe {
        if libc::login_tty(rustix::fd::IntoRawFd::into_raw_fd(fd)) != 0 {
            return Err(io::Errno::from_raw_os_error(errno::errno().0));
        }
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        // Create a new session.
        rustix::process::setsid().ok();

        // Set up `fd` as the controlling terminal.
        rustix::process::ioctl_tiocsctty(&fd)?;

        // Install `fd` as our stdio.
        rustix::stdio::dup2_stdin(&fd).ok();
        rustix::stdio::dup2_stdout(&fd).ok();
        rustix::stdio::dup2_stderr(&fd).ok();

        // If we overwrote the `fd` with our `dup2`s, don't close it now.
        if rustix::fd::AsRawFd::as_raw_fd(&fd) <= 2 {
            core::mem::forget(fd);
        }
    }

    Ok(())
}

/// Close all open file descriptors that are at least as great as `from`.
///
/// # Safety
///
/// This can close files out from underneath libraries, leaving them holding
/// dangling file descriptors. It's meant for use in spawning new processes
/// where the existing process state is about to be overwritten anyway.
#[cfg(any(target_os = "android", target_os = "linux"))] // for `RawDir`
pub unsafe fn closefrom(from: RawFd) {
    use core::mem::MaybeUninit;
    use core::str;
    use rustix::fs::{openat, Mode, OFlags, RawDir, CWD};

    let dir = openat(
        CWD,
        rustix::cstr!("/dev/fd"),
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();
    let dir_raw_fd = dir.as_fd().as_raw_fd();

    // We can use a fixed-sized buffer because `/dev/fd` names are only so big.
    let mut buf = [MaybeUninit::uninit(); 1024];
    let mut iter = RawDir::new(dir, &mut buf);
    while let Some(entry) = iter.next() {
        let entry = entry.unwrap();
        let name_bytes = entry.file_name().to_bytes();
        if name_bytes == b"." || name_bytes == b".." {
            continue;
        }
        let name = str::from_utf8(name_bytes).unwrap();
        let num = name.parse::<RawFd>().unwrap();

        if num >= from && num != dir_raw_fd {
            io::close(num);
        }
    }
}
