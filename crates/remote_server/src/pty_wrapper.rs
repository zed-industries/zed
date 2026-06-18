use anyhow::{Context as _, Result, anyhow};
use std::ffi::CString;
use std::io;

/// Size of the buffer used when copying data between terminals
const READ_BUF_SIZE: usize = 64 * 1024;

/// Polling interval for the window size, since we can't rely on SIGWINCH signals
const WINSIZE_POLL_INTERVAL_MS: libc::c_int = 150;

/// Exit code used when the wrapped program cannot be executed
const EXEC_FAILURE_EXIT_CODE: i32 = 127;

/// Run the given command inside a new subservient PTY
///
/// Shells and terminal programs want control over the terminal, but if the PTY is
/// already "claimed" by a process it can't be claimed again by the shell. This spawns
/// `command` with a new, unclaimed PTY.
///
/// Returns the exit status of the command.
pub fn main(command: Vec<String>) -> Result<i32> {
    let program = command
        .first()
        .context("pty-wrapper requires a command to run")?;

    // Prepare everything the child needs *before* forking, so that between
    // `fork()` and `execvp()` we only touch async-signal-safe libc functions
    // and never allocate. `argv` includes the program itself as `argv[0]`.
    let program = CString::new(program.as_bytes()).context("program path contains a NUL byte")?;
    let argv: Vec<CString> = command
        .iter()
        .map(|arg| CString::new(arg.as_bytes()))
        .collect::<std::result::Result<_, _>>()
        .context("command argument contains a NUL byte")?;
    let mut argv_ptrs: Vec<*const libc::c_char> = argv.iter().map(|arg| arg.as_ptr()).collect();
    argv_ptrs.push(std::ptr::null());

    // Open the PTY pair the command will be running under
    let Pty { master, slave } = Pty::open()?;

    // Synchronize the window size of the new PTY, so the very first draw is correct.
    let mut winsize = WinSizeManager::new(master);
    winsize.sync(libc::STDIN_FILENO);

    // SAFETY: `fork` is safe to call; we are single-threaded at this point
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        let err = io::Error::last_os_error();
        // SAFETY: both descriptors are open and owned by us.
        unsafe {
            libc::close(master);
            libc::close(slave);
        }
        return Err(anyhow!("failed to fork pty-wrapper child: {err}"));
    }

    if pid == 0 {
        // Child process. Only async-signal-safe libc calls from here until `execvp`.
        // SAFETY: all pointers were constructed above and remain valid; we do
        // not return from this branch.
        unsafe {
            // Ensure the child is killed if the wrapper dies for any reason
            libc::prctl(
                libc::PR_SET_PDEATHSIG,
                libc::SIGKILL as libc::c_ulong,
                0,
                0,
                0,
            );

            // Become a session leader and adopt the fresh pty as our
            // controlling terminal. Both succeed because the pty is brand new.
            libc::setsid();
            libc::ioctl(slave, libc::TIOCSCTTY, 0);

            // Overwrite our stdio descriptors with the pty
            libc::dup2(slave, libc::STDIN_FILENO);
            libc::dup2(slave, libc::STDOUT_FILENO);
            libc::dup2(slave, libc::STDERR_FILENO);
            if slave > libc::STDERR_FILENO {
                libc::close(slave);
            }
            libc::close(master);

            // exec into the target process. If that fails, exit the child immediately.
            libc::execvp(program.as_ptr(), argv_ptrs.as_ptr());
            libc::_exit(EXEC_FAILURE_EXIT_CODE)
        }
    }

    // Parent process. Close the slave and start ferrying data between terminals.
    // SAFETY: the slave is owned by us and no longer needed on this side.
    unsafe { libc::close(slave) };

    let result = {
        // Set stdin to raw mode so we can transfer the individual keystrokes through
        // to the child PTY.
        let _raw_mode = RawModeGuard::enable(libc::STDIN_FILENO);

        ferry(master, pid, &mut winsize)?;
        wait_for_child(pid)
    };

    // SAFETY: the master is owned by us.
    unsafe { libc::close(master) };

    result
}

/// An owned host pseudo-terminal (PTY) pair.
struct Pty {
    master: libc::c_int,
    slave: libc::c_int,
}

impl Pty {
    /// Allocates and open a new PTY pair
    fn open() -> Result<Self> {
        // SAFETY: each call is checked, and descriptors are closed on any error
        // path before returning.
        unsafe {
            // Open the PTY pair
            let master = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
            if master < 0 {
                return Err(anyhow!(
                    "posix_openpt failed: {}",
                    io::Error::last_os_error()
                ));
            }

            // Grant and unlock our process/user access to the slave PTY
            if libc::grantpt(master) != 0 {
                let err = io::Error::last_os_error();
                libc::close(master);
                return Err(anyhow!("grantpt failed: {err}"));
            }
            if libc::unlockpt(master) != 0 {
                let err = io::Error::last_os_error();
                libc::close(master);
                return Err(anyhow!("unlockpt failed: {err}"));
            }

            // Get the path of the slave PTY device. Note that this is thread-unsafe,
            // but we are single-threaded through this point so it's fine.
            let slave_path = libc::ptsname(master);
            if slave_path.is_null() {
                let err = io::Error::last_os_error();
                libc::close(master);
                return Err(anyhow!("ptsname failed: {err}"));
            }

            // Open the slave PTY
            let slave = libc::open(slave_path, libc::O_RDWR | libc::O_NOCTTY);
            if slave < 0 {
                let err = io::Error::last_os_error();
                libc::close(master);
                return Err(anyhow!("failed to open pts slave: {err}"));
            }

            Ok(Self { master, slave })
        }
    }
}

/// RAII guard for keeping a file descriptor in "raw mode." When this is dropped (falls
/// out of scope), the file descriptor will be restored to it's original state.
///
/// Raw mode absorbs all keystrokes, instead of echoing them back automatically.
#[must_use]
struct RawModeGuard {
    fd: libc::c_int,
    original: libc::termios,
}

impl RawModeGuard {
    /// Switches file descriptor `fd` into raw mode, returning a guard that restores its
    /// status when dropped.
    ///
    /// Returns [None] if the file descriptor isn't a terminal.
    fn enable(fd: libc::c_int) -> Option<Self> {
        // SAFETY: `termios` is a plain C struct; every call is checked.
        unsafe {
            let mut termios: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(fd, &mut termios) != 0 {
                return None;
            }
            let original = termios;
            libc::cfmakeraw(&mut termios);
            if libc::tcsetattr(fd, libc::TCSANOW, &termios) != 0 {
                return None;
            }
            Some(Self { fd, original })
        }
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // SAFETY: `self.original` was captured from this same descriptor.
        unsafe {
            libc::tcsetattr(self.fd, libc::TCSANOW, &self.original);
        }
    }
}

/// Copy bytes between our (parent process) stdio and the child's PTY master `fd`, until
/// the child process exits.
fn ferry(
    child_fd: libc::c_int,
    child_pid: libc::pid_t,
    winsize: &mut WinSizeManager,
) -> Result<()> {
    let mut buffer = vec![0u8; READ_BUF_SIZE];
    let mut child_hungup: bool = false;

    loop {
        let mut fds = [
            libc::pollfd {
                fd: if child_hungup { -1 } else { libc::STDIN_FILENO },
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                fd: child_fd,
                events: libc::POLLIN,
                revents: 0,
            },
        ];

        // Wait for some data to appear on either side.
        // SAFETY: `fds` is a valid, correctly-sized array for the duration of the call.
        let ready = unsafe {
            libc::poll(
                fds.as_mut_ptr(),
                fds.len() as libc::nfds_t,
                WINSIZE_POLL_INTERVAL_MS,
            )
        };
        if ready < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(anyhow!("poll failed: {err}"));
        }

        // Synchronize the window size now when it might have just changed.
        winsize.sync(libc::STDIN_FILENO);

        // No bytes are available on either side, try it again
        if ready == 0 {
            continue;
        }

        // Forward input from stdin to the target fd.
        if fds[0].revents & (libc::POLLIN | libc::POLLHUP) != 0 {
            match read_fd(libc::STDIN_FILENO, &mut buffer)? {
                0 => {
                    // The terminal on stdin has been closed. Hang up the child and
                    // continue ferrying data while it winds down.
                    // SAFETY: signals the child's process group.
                    if !child_hungup {
                        unsafe { libc::kill(-child_pid, libc::SIGHUP) };
                        child_hungup = true;
                    }
                }
                n => write_all_fd(child_fd, &buffer[..n])?,
            }
        }

        // Forward output from the target fd back to stdout.
        if fds[1].revents & (libc::POLLIN | libc::POLLHUP) != 0 {
            match read_fd(child_fd, &mut buffer) {
                // EOF or EIO indicate the child has exited and its PTY is unclaimed.
                // In both cases we have no need to continue ferrying data.
                Ok(0) => break,
                Err(err) if err.raw_os_error() == Some(libc::EIO) => break,
                // Otherwise we have data, or a real error
                Ok(n) => write_all_fd(libc::STDOUT_FILENO, &buffer[..n])?,
                Err(err) => return Err(anyhow!("reading from child PTY failed: {err}")),
            }
        }
    }

    Ok(())
}

/// Window size manager for a target PTY
struct WinSizeManager {
    target_pty: libc::c_int,
    last_applied: Option<libc::winsize>,
}

impl WinSizeManager {
    /// Manage the window size of the given PTY.
    fn new(target_pty: libc::c_int) -> Self {
        Self {
            target_pty,
            last_applied: None,
        }
    }

    /// Synchronize the window size from a `source` PTY to the target PTY.
    fn sync(&mut self, source: libc::c_int) {
        // SAFETY: `winsize` is a plain C struct and the ioctl is checked.
        let current = unsafe {
            let mut winsize: libc::winsize = std::mem::zeroed();
            if libc::ioctl(source, libc::TIOCGWINSZ, &mut winsize) == 0 {
                winsize
            } else {
                return;
            }
        };
        let changed = match self.last_applied {
            Some(previous) => {
                previous.ws_row != current.ws_row
                    || previous.ws_col != current.ws_col
                    || previous.ws_xpixel != current.ws_xpixel
                    || previous.ws_ypixel != current.ws_ypixel
            }
            None => true,
        };
        if changed {
            // SAFETY: `current` is a valid winsize for the lifetime of the call.
            unsafe { libc::ioctl(self.target_pty, libc::TIOCSWINSZ, &current) };
            self.last_applied = Some(current);
        }
    }
}

/// Reads once from `fd`, retrying on `EINTR`. Returns the number of bytes read
/// (0 on EOF).
fn read_fd(fd: libc::c_int, buffer: &mut [u8]) -> io::Result<usize> {
    loop {
        // SAFETY: writing into `buffer` for at most `buffer.len()` bytes.
        let n = unsafe { libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()) };
        if n < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(err);
        }
        return Ok(n as usize);
    }
}

/// Writes the whole buffer to `fd`, retrying on `EINTR` and short writes.
fn write_all_fd(fd: libc::c_int, mut buffer: &[u8]) -> Result<()> {
    while !buffer.is_empty() {
        // SAFETY: reading `buffer.len()` bytes from a valid slice.
        let n = unsafe { libc::write(fd, buffer.as_ptr() as *const libc::c_void, buffer.len()) };
        if n < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(anyhow!("write failed: {err}"));
        }
        buffer = &buffer[n as usize..];
    }
    Ok(())
}

/// Waits for the wrapped child to terminate and maps its status to an exit code.
fn wait_for_child(pid: libc::pid_t) -> Result<i32> {
    loop {
        let mut status: libc::c_int = 0;
        // SAFETY: `status` outlives the call.
        let result = unsafe { libc::waitpid(pid, &mut status, 0) };
        if result < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(anyhow!("waitpid failed: {err}"));
        }

        if libc::WIFEXITED(status) {
            return Ok(libc::WEXITSTATUS(status));
        }
        if libc::WIFSIGNALED(status) {
            // Match the shell convention of reporting a killed process as
            // 128 + signal number.
            return Ok(128 + libc::WTERMSIG(status));
        }
        // Stopped/continued: keep waiting for a process status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Closes a raw file descriptor, ignoring errors (test cleanup only).
    fn close(fd: libc::c_int) {
        // SAFETY: `fd` is a descriptor owned by the test.
        unsafe { libc::close(fd) };
    }

    /// Creates an anonymous pipe, returning `(read, write)` descriptors.
    fn pipe() -> (libc::c_int, libc::c_int) {
        let mut fds = [0 as libc::c_int; 2];
        // SAFETY: `fds` is a valid two-element array.
        let result = unsafe { libc::pipe(fds.as_mut_ptr()) };
        assert_eq!(result, 0, "pipe() failed: {}", io::Error::last_os_error());
        (fds[0], fds[1])
    }

    fn winsize(rows: u16, cols: u16) -> libc::winsize {
        libc::winsize {
            ws_row: rows,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        }
    }

    fn get_winsize(pty: libc::c_int) -> Option<libc::winsize> {
        // SAFETY: `winsize` is a plain C struct and the ioctl is checked.
        unsafe {
            let mut winsize: libc::winsize = std::mem::zeroed();
            if libc::ioctl(pty, libc::TIOCGWINSZ, &mut winsize) == 0 {
                Some(winsize)
            } else {
                None
            }
        }
    }

    /// Reads a terminal's line-discipline flags.
    fn local_flags(fd: libc::c_int) -> libc::tcflag_t {
        // SAFETY: `termios` is a plain C struct and the call is checked.
        unsafe {
            let mut termios: libc::termios = std::mem::zeroed();
            assert_eq!(libc::tcgetattr(fd, &mut termios), 0);
            termios.c_lflag
        }
    }

    #[test]
    fn pty_open_yields_a_terminal() {
        let pty = Pty::open().expect("failed to open pty");

        // SAFETY: `slave` is a valid descriptor.
        assert_eq!(
            unsafe { libc::isatty(pty.slave) },
            1,
            "slave should be a tty"
        );

        close(pty.master);
        close(pty.slave);
    }

    #[test]
    fn sync_winsize_mirrors_source_onto_master_only_on_change() {
        let source = Pty::open().expect("failed to open source pty");
        let target = Pty::open().expect("failed to open target pty");

        // Give the source terminal a concrete size.
        let initial = winsize(24, 80);
        // SAFETY: `initial` is valid for the duration of the ioctl.
        unsafe { libc::ioctl(source.master, libc::TIOCSWINSZ, &initial) };

        let mut winsizeman = WinSizeManager::new(target.master);
        winsizeman.sync(source.slave);
        assert_eq!(
            winsizeman.last_applied.map(|ws| (ws.ws_row, ws.ws_col)),
            Some((24, 80))
        );
        let mirrored = get_winsize(target.slave).expect("target should have a size");
        assert_eq!((mirrored.ws_row, mirrored.ws_col), (24, 80));

        // A resize on the source is mirrored on the next sync.
        let resized = winsize(40, 120);
        // SAFETY: `resized` is valid for the duration of the ioctl.
        unsafe { libc::ioctl(source.slave, libc::TIOCSWINSZ, &resized) };
        winsizeman.sync(source.slave);
        let mirrored = get_winsize(target.slave).expect("target should have a size");
        assert_eq!((mirrored.ws_row, mirrored.ws_col), (40, 120));

        close(source.master);
        close(source.slave);
        close(target.master);
        close(target.slave);
    }

    #[test]
    fn read_and_write_round_trip_through_a_pipe() {
        let (read, write) = pipe();

        write_all_fd(write, b"hello, host").expect("write should succeed");
        let mut buffer = [0u8; 64];
        let n = read_fd(read, &mut buffer).expect("read should succeed");
        assert_eq!(&buffer[..n], b"hello, host");

        // Once the writer is closed, the reader observes EOF.
        close(write);
        assert_eq!(read_fd(read, &mut buffer).expect("read should succeed"), 0);

        close(read);
    }

    #[test]
    fn raw_mode_disables_echo_and_restores_on_drop() {
        let pty = Pty::open().expect("failed to open pty");

        let before = local_flags(pty.slave);
        assert_ne!(before & libc::ECHO, 0, "a fresh pts should echo by default");

        {
            let _guard = RawModeGuard::enable(pty.slave).expect("slave is a tty");
            assert_eq!(
                local_flags(pty.slave) & libc::ECHO,
                0,
                "raw mode should disable echo"
            );
        }

        assert_eq!(
            local_flags(pty.slave),
            before,
            "dropping the guard should restore the original attributes"
        );

        close(pty.master);
        close(pty.slave);
    }

    #[test]
    fn raw_mode_is_none_for_non_terminal() {
        let (read, write) = pipe();
        assert!(RawModeGuard::enable(read).is_none());
        close(read);
        close(write);
    }

    #[test]
    fn wait_for_child_reports_exit_code() {
        // SAFETY: the child only calls async-signal-safe functions before
        // exiting, so forking from the test harness is safe here.
        let pid = unsafe { libc::fork() };
        assert!(pid >= 0, "fork failed: {}", io::Error::last_os_error());
        if pid == 0 {
            // SAFETY: child immediately exits without touching shared state.
            unsafe { libc::_exit(7) };
        }
        assert_eq!(wait_for_child(pid).expect("wait should succeed"), 7);
    }

    #[test]
    fn wait_for_child_reports_terminating_signal() {
        // SAFETY: the child only calls async-signal-safe functions.
        let pid = unsafe { libc::fork() };
        assert!(pid >= 0, "fork failed: {}", io::Error::last_os_error());
        if pid == 0 {
            // SAFETY: terminate ourselves with a signal.
            unsafe {
                libc::raise(libc::SIGKILL);
                libc::_exit(0);
            }
        }
        assert_eq!(
            wait_for_child(pid).expect("wait should succeed"),
            128 + libc::SIGKILL
        );
    }
}
