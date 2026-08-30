//! A terminal backend whose PTY lives on a remote host.
//!
//! Platforms that cannot spawn processes (iOS) cannot run the usual
//! `ssh -t` command inside a local PTY. Instead, the remote transport hands us
//! one half of a socket pair carrying the raw terminal byte stream, and we
//! adapt it to alacritty's PTY traits so the ordinary terminal event loop can
//! drive it.

use std::io;
use std::os::unix::net::UnixStream;
use std::os::unix::process::ExitStatusExt as _;
use std::process::ExitStatus;
use std::sync::{Arc, Mutex};

use alacritty_terminal::event::{OnResize, WindowSize};
use alacritty_terminal::tty::{ChildEvent, EventedPty, EventedReadWrite};
use futures::channel::mpsc::UnboundedSender;
use polling::{Event, PollMode, Poller};

/// IO endpoints connecting a terminal to a PTY on a remote host.
pub struct RemotePtyChannels {
    /// Carries terminal bytes in both directions.
    pub data: UnixStream,
    /// Becomes readable (or reaches EOF) once the remote command exits.
    pub exit_notice: UnixStream,
    /// The remote exit code, set before `exit_notice` is signalled.
    pub exit_status: Arc<Mutex<Option<i32>>>,
    /// Receives `(cols, rows)` when the terminal is resized.
    pub resize_tx: UnboundedSender<(u16, u16)>,
}

// These match the token values alacritty's event loop dispatches on; its
// `PTY_READ_WRITE_TOKEN`/`PTY_CHILD_EVENT_TOKEN` constants are not public.
const PTY_READ_WRITE_TOKEN: usize = 0;
const PTY_CHILD_EVENT_TOKEN: usize = 1;

pub(crate) struct RemotePtyAdapter {
    data: UnixStream,
    exit_notice: UnixStream,
    exit_status: Arc<Mutex<Option<i32>>>,
    resize_tx: UnboundedSender<(u16, u16)>,
    exit_reported: bool,
}

impl RemotePtyAdapter {
    pub(crate) fn new(channels: RemotePtyChannels) -> io::Result<Self> {
        channels.data.set_nonblocking(true)?;
        channels.exit_notice.set_nonblocking(true)?;
        Ok(Self {
            data: channels.data,
            exit_notice: channels.exit_notice,
            exit_status: channels.exit_status,
            resize_tx: channels.resize_tx,
            exit_reported: false,
        })
    }
}

impl EventedReadWrite for RemotePtyAdapter {
    type Reader = UnixStream;
    type Writer = UnixStream;

    unsafe fn register(
        &mut self,
        poll: &Arc<Poller>,
        mut interest: Event,
        poll_opts: PollMode,
    ) -> io::Result<()> {
        interest.key = PTY_READ_WRITE_TOKEN;
        unsafe {
            poll.add_with_mode(&self.data, interest, poll_opts)?;
        }
        unsafe {
            poll.add_with_mode(
                &self.exit_notice,
                Event::readable(PTY_CHILD_EVENT_TOKEN),
                PollMode::Level,
            )
        }
    }

    fn reregister(
        &mut self,
        poll: &Arc<Poller>,
        mut interest: Event,
        poll_opts: PollMode,
    ) -> io::Result<()> {
        interest.key = PTY_READ_WRITE_TOKEN;
        poll.modify_with_mode(&self.data, interest, poll_opts)?;
        poll.modify_with_mode(
            &self.exit_notice,
            Event::readable(PTY_CHILD_EVENT_TOKEN),
            PollMode::Level,
        )
    }

    fn deregister(&mut self, poll: &Arc<Poller>) -> io::Result<()> {
        poll.delete(&self.data)?;
        poll.delete(&self.exit_notice)
    }

    fn reader(&mut self) -> &mut UnixStream {
        &mut self.data
    }

    fn writer(&mut self) -> &mut UnixStream {
        &mut self.data
    }
}

impl EventedPty for RemotePtyAdapter {
    fn next_child_event(&mut self) -> Option<ChildEvent> {
        use std::io::Read as _;

        if self.exit_reported {
            return None;
        }
        let mut buffer = [0u8; 8];
        match self.exit_notice.read(&mut buffer) {
            // A written byte or EOF both mean the remote command is gone.
            Ok(_) => {
                self.exit_reported = true;
                let code = self.exit_status.lock().ok().and_then(|status| *status);
                Some(ChildEvent::Exited(
                    code.map(|code| ExitStatus::from_raw(code << 8)),
                ))
            }
            Err(_) => None,
        }
    }
}

impl OnResize for RemotePtyAdapter {
    fn on_resize(&mut self, window_size: WindowSize) {
        self.resize_tx
            .unbounded_send((window_size.num_cols, window_size.num_lines))
            .ok();
    }
}
