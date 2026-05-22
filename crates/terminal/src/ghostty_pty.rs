#[cfg(unix)]
use std::ffi::CStr;
use std::{
    borrow::Cow,
    io::{ErrorKind, Read, Write},
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

use anyhow::{Context as _, Result};
use futures::channel::mpsc::UnboundedSender;
use log::{debug, error};
use portable_pty::{Child, MasterPty, PtySize};

use crate::{PtyEvent, TerminalBackendEvent, TerminalBounds};

const READ_BUFFER_SIZE: usize = 0x10_0000;
const DRAIN_ON_EXIT_TIMEOUT: Duration = Duration::from_secs(1);

enum GhosttyPtyMsg {
    Input(Cow<'static, [u8]>),
    Resize(TerminalBounds),
    Shutdown,
}

pub(super) struct GhosttyPtyEventLoop {
    master: Box<dyn MasterPty + Send>,
    reader: Box<dyn Read + Send>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn Child + Send + Sync>,
    receiver: mpsc::Receiver<GhosttyPtyMsg>,
    sender: mpsc::Sender<GhosttyPtyMsg>,
    events_tx: UnboundedSender<PtyEvent>,
    drain_on_exit: bool,
}

impl GhosttyPtyEventLoop {
    pub(super) fn new(
        events_tx: UnboundedSender<PtyEvent>,
        master: Box<dyn MasterPty + Send>,
        child: Box<dyn Child + Send + Sync>,
        drain_on_exit: bool,
    ) -> Result<Self> {
        let reader = master
            .try_clone_reader()
            .context("failed to clone ghostty pty reader")?;
        let writer = master
            .take_writer()
            .context("failed to take ghostty pty writer")?;
        let (sender, receiver) = mpsc::channel();

        Ok(Self {
            master,
            reader,
            writer,
            child,
            receiver,
            sender,
            events_tx,
            drain_on_exit,
        })
    }

    pub(super) fn channel(&self) -> GhosttyPtySender {
        GhosttyPtySender {
            sender: self.sender.clone(),
        }
    }

    pub(super) fn spawn(self) -> JoinHandle<()> {
        let Self {
            master,
            reader,
            writer,
            child,
            receiver,
            sender,
            events_tx,
            drain_on_exit,
        } = self;
        let (reader_done_sender, reader_done_receiver) = mpsc::channel();

        thread::spawn({
            let events_tx = events_tx.clone();
            move || Self::read_pty(reader, events_tx, reader_done_sender)
        });

        thread::spawn({
            move || {
                Self::wait_for_child(
                    child,
                    events_tx,
                    drain_on_exit,
                    reader_done_receiver,
                    sender,
                )
            }
        });

        thread::spawn(move || Self::control_pty(master, writer, receiver))
    }

    fn read_pty(
        mut reader: Box<dyn Read + Send>,
        events_tx: UnboundedSender<PtyEvent>,
        done_sender: mpsc::Sender<()>,
    ) {
        let mut buffer = [0u8; READ_BUFFER_SIZE];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => {
                    send_pty_event(
                        &events_tx,
                        PtyEvent::Output(buffer[..read].to_vec()),
                        "output",
                    );
                }
                Err(error) => match error.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => {
                        error!("error reading from ghostty pty: {error}");
                        break;
                    }
                },
            }
        }

        if done_sender.send(()).is_err() {
            debug!("failed to send ghostty pty reader completion");
        }
    }

    fn wait_for_child(
        mut child: Box<dyn Child + Send + Sync>,
        events_tx: UnboundedSender<PtyEvent>,
        drain_on_exit: bool,
        reader_done_receiver: mpsc::Receiver<()>,
        sender: mpsc::Sender<GhosttyPtyMsg>,
    ) {
        match child.wait() {
            Ok(status) => {
                if drain_on_exit
                    && let Err(error) = reader_done_receiver.recv_timeout(DRAIN_ON_EXIT_TIMEOUT)
                {
                    debug!("timed out draining ghostty pty after child exit: {error}");
                }

                send_pty_event(
                    &events_tx,
                    PtyEvent::Event(TerminalBackendEvent::ChildExit(
                        portable_exit_status_to_raw_status(status),
                    )),
                    "child-exit",
                );
            }
            Err(error) => {
                error!("error waiting for ghostty pty child process: {error}");
            }
        }

        send_pty_event(
            &events_tx,
            PtyEvent::Event(TerminalBackendEvent::Wakeup),
            "wakeup",
        );

        if let Err(error) = sender.send(GhosttyPtyMsg::Shutdown) {
            debug!("failed to stop ghostty pty control loop after child exit: {error}");
        }
    }

    fn control_pty(
        master: Box<dyn MasterPty + Send>,
        mut writer: Box<dyn Write + Send>,
        receiver: mpsc::Receiver<GhosttyPtyMsg>,
    ) {
        while let Ok(message) = receiver.recv() {
            match message {
                GhosttyPtyMsg::Input(input) => {
                    if let Err(error) = writer.write_all(&input) {
                        debug!("failed to write to ghostty pty: {error}");
                        if matches!(
                            error.kind(),
                            ErrorKind::BrokenPipe | ErrorKind::NotConnected
                        ) {
                            break;
                        }
                    } else if let Err(error) = writer.flush() {
                        debug!("failed to flush ghostty pty writer: {error}");
                    }
                }
                GhosttyPtyMsg::Resize(bounds) => {
                    if let Err(error) = master.resize(portable_pty_size(bounds)) {
                        error!("failed to resize ghostty pty: {error}");
                    }
                }
                GhosttyPtyMsg::Shutdown => break,
            }
        }
    }
}

#[derive(Clone)]
pub(super) struct GhosttyPtySender {
    sender: mpsc::Sender<GhosttyPtyMsg>,
}

impl GhosttyPtySender {
    fn send(&self, message: GhosttyPtyMsg) {
        if let Err(error) = self.sender.send(message) {
            debug!("failed to send ghostty pty message: {error}");
        }
    }
}

#[derive(Clone)]
pub(super) struct GhosttyPtyNotifier {
    sender: GhosttyPtySender,
}

impl GhosttyPtyNotifier {
    pub(super) fn new(sender: GhosttyPtySender) -> Self {
        Self { sender }
    }

    pub(super) fn notify<B>(&self, bytes: B)
    where
        B: Into<Cow<'static, [u8]>>,
    {
        let bytes = bytes.into();
        if !bytes.is_empty() {
            self.sender.send(GhosttyPtyMsg::Input(bytes));
        }
    }

    pub(super) fn resize(&self, bounds: TerminalBounds) {
        self.sender.send(GhosttyPtyMsg::Resize(bounds));
    }

    pub(super) fn shutdown(&self) {
        self.sender.send(GhosttyPtyMsg::Shutdown);
    }
}

pub(super) fn portable_pty_size(bounds: TerminalBounds) -> PtySize {
    let rows = bounds.num_lines().max(1).min(u16::MAX as usize) as u16;
    let cols = bounds.num_columns().max(1).min(u16::MAX as usize) as u16;
    let cell_width = f32::from(bounds.cell_width()).max(1.0);
    let cell_height = f32::from(bounds.line_height()).max(1.0);

    PtySize {
        rows,
        cols,
        pixel_width: pixels_to_u16(cell_width * f32::from(cols)),
        pixel_height: pixels_to_u16(cell_height * f32::from(rows)),
    }
}

fn pixels_to_u16(pixels: f32) -> u16 {
    pixels.max(0.0).min(u16::MAX as f32) as u16
}

fn portable_exit_status_to_raw_status(status: portable_pty::ExitStatus) -> i32 {
    #[cfg(unix)]
    {
        if let Some(signal) = status.signal()
            && let Some(signal) = portable_signal_to_raw_signal(signal)
        {
            return signal;
        }

        let code = status.exit_code().min((i32::MAX >> 8) as u32) as i32;
        code << 8
    }

    #[cfg(windows)]
    {
        status.exit_code().min(i32::MAX as u32) as i32
    }
}

#[cfg(unix)]
fn portable_signal_to_raw_signal(signal: &str) -> Option<i32> {
    for signal_number in 1..128 {
        let description = unsafe { libc::strsignal(signal_number) };
        if description.is_null() {
            continue;
        }

        let description = unsafe { CStr::from_ptr(description) }.to_string_lossy();
        if description == signal {
            return Some(signal_number);
        }
    }

    signal
        .rsplit(|character: char| !character.is_ascii_digit())
        .find_map(|segment| {
            let signal_number = segment.parse::<i32>().ok()?;
            (1..128).contains(&signal_number).then_some(signal_number)
        })
}

fn send_pty_event(
    events_tx: &UnboundedSender<PtyEvent>,
    event: PtyEvent,
    description: &'static str,
) {
    if let Err(error) = events_tx.unbounded_send(event) {
        debug!("failed to send ghostty pty {description} event: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn test_portable_exit_status_to_raw_status_preserves_unix_signal() {
        use std::os::unix::process::ExitStatusExt as _;

        let signal_name = unsafe { libc::strsignal(libc::SIGTERM) };
        assert!(!signal_name.is_null());
        let signal_name = unsafe { CStr::from_ptr(signal_name) }
            .to_string_lossy()
            .to_string();

        let raw_status =
            portable_exit_status_to_raw_status(portable_pty::ExitStatus::with_signal(&signal_name));
        let exit_status = std::process::ExitStatus::from_raw(raw_status);

        assert_eq!(exit_status.signal(), Some(libc::SIGTERM));
        assert_eq!(exit_status.code(), None);
    }

    #[cfg(unix)]
    #[test]
    fn test_portable_exit_status_to_raw_status_preserves_exit_code() {
        use std::os::unix::process::ExitStatusExt as _;

        let raw_status =
            portable_exit_status_to_raw_status(portable_pty::ExitStatus::with_exit_code(42));
        let exit_status = std::process::ExitStatus::from_raw(raw_status);

        assert_eq!(exit_status.code(), Some(42));
        assert_eq!(exit_status.signal(), None);
    }
}
