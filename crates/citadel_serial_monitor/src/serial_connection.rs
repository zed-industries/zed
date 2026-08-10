/// A device that never sends `\n` (a `Serial.print()`-only sketch) would
/// otherwise grow `carry` without bound and never display anything. Once
/// this many bytes have accumulated with no newline in sight, flush them as
/// a line instead of waiting indefinitely.
const MAX_CARRY_BYTES: usize = 4096;

/// Splits `chunk` into complete lines, carrying any trailing partial line
/// over in `carry` for the next call. Recognizes `\n` as the line
/// terminator and strips a trailing `\r` (so both `\n`- and
/// `\r\n`-terminated sketches work). Invalid UTF-8 is replaced per
/// `String::from_utf8_lossy` rather than erroring -- a serial device can
/// send anything, and one garbled line shouldn't stop the monitor.
pub fn split_lines(carry: &mut Vec<u8>, chunk: &[u8]) -> Vec<String> {
    carry.extend_from_slice(chunk);
    let mut lines = Vec::new();
    while let Some(newline_index) = carry.iter().position(|&byte| byte == b'\n') {
        let mut line_bytes: Vec<u8> = carry.drain(..=newline_index).collect();
        line_bytes.pop(); // remove the '\n' itself
        if line_bytes.last() == Some(&b'\r') {
            line_bytes.pop();
        }
        lines.push(String::from_utf8_lossy(&line_bytes).into_owned());
    }
    if carry.len() > MAX_CARRY_BYTES {
        lines.push(String::from_utf8_lossy(carry).into_owned());
        carry.clear();
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lines_single_complete_line() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"hello\n");
        assert_eq!(lines, vec!["hello".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_multiple_lines_one_chunk() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"x\ny\nz\n");
        assert_eq!(lines, vec!["x".to_string(), "y".to_string(), "z".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_partial_line_carried_over() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"hello\nworl");
        assert_eq!(lines, vec!["hello".to_string()]);
        assert_eq!(carry, b"worl");

        let lines = split_lines(&mut carry, b"d\n");
        assert_eq!(lines, vec!["world".to_string()]);
        assert!(carry.is_empty());
    }

    #[test]
    fn test_split_lines_strips_carriage_return() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"a\r\nb\r\n");
        assert_eq!(lines, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_split_lines_no_newline_yet() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"abc");
        assert!(lines.is_empty());
        assert_eq!(carry, b"abc");
    }

    #[test]
    fn test_split_lines_invalid_utf8_does_not_panic() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, &[0xFF, b'\n']);
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_split_lines_empty_line() {
        let mut carry = Vec::new();
        let lines = split_lines(&mut carry, b"\n");
        assert_eq!(lines, vec!["".to_string()]);
    }

    #[test]
    fn test_split_lines_flushes_carry_past_cap_without_newline() {
        let mut carry = Vec::new();
        let chunk = vec![b'x'; MAX_CARRY_BYTES + 1];
        let lines = split_lines(&mut carry, &chunk);
        assert_eq!(lines, vec![String::from_utf8_lossy(&chunk).into_owned()]);
        assert!(carry.is_empty());
    }
}

use citadel_build::board_detect::{BoardMonitor, FlashFinished, FlashStarted, GlobalBoardMonitor};
use futures::StreamExt;
use futures::channel::mpsc::{UnboundedSender, unbounded};
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Global, Subscription};
use serialport::SerialPort;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const MAX_BUFFERED_LINES: usize = 1000;
pub const DEFAULT_BAUD_RATE: u32 = 9600;

/// Emitted for every complete line received on the open connection.
pub struct SerialLineReceived(pub String);
impl EventEmitter<SerialLineReceived> for SerialConnection {}

/// Emitted when the connection fails to open or a read/write fails while
/// connected. Also flips `is_open` to `false`.
pub struct SerialConnectionError(pub String);
impl EventEmitter<SerialConnectionError> for SerialConnection {}

pub struct GlobalSerialConnection(pub Entity<SerialConnection>);
impl Global for GlobalSerialConnection {}

pub fn init(cx: &mut App) {
    let connection = cx.new(SerialConnection::new);
    cx.set_global(GlobalSerialConnection(connection));
}

/// Reads `citadel_build::board_detect::GlobalBoardMonitor`'s currently
/// detected board, if any, to use as the default port for a freshly opened
/// Monitor panel or Plotter window. Never triggers a connection by itself.
pub fn default_port_name(cx: &App) -> Option<String> {
    cx.try_global::<GlobalBoardMonitor>()?
        .0
        .read(cx)
        .detected
        .as_ref()
        .map(|detected| detected.port_name.clone())
}

enum SerialConnectionMessage {
    Opened,
    Line(String),
    Error(String),
}

/// Tracks the background reader's relationship to the OS port handle across
/// the async gap between "we told the reader to open a port" and "the
/// reader's blocking `open()` call actually returns". Plain
/// `Option<Box<dyn SerialPort>>` can't distinguish "still opening" from
/// "closed" -- see `close_port`'s doc comment for why that distinction
/// matters.
enum PortSlot {
    Connecting,
    Open(Box<dyn SerialPort>),
    Closed,
}

/// Bundles the OS handle (shared with the background reader loop via a
/// mutex so `send()` can write without a second open handle) and the
/// channel used to report read/write errors back to the entity.
struct OpenPort {
    handle: Arc<Mutex<PortSlot>>,
    message_tx: UnboundedSender<SerialConnectionMessage>,
}

/// The single owner of the process's one persistent serial port connection.
/// Shared by the Monitor panel and the Plotter window via
/// `GlobalSerialConnection` so they never try to open the same port twice.
pub struct SerialConnection {
    pub port_name: Option<String>,
    pub baud_rate: u32,
    pub is_open: bool,
    pub lines: VecDeque<String>,
    open: Option<OpenPort>,
    /// Remembers (port_name, baud_rate) while paused for a flash, so
    /// `resume_after_flash` can reconnect at the same settings.
    paused: Option<(String, u32)>,
    _subscriptions: Vec<Subscription>,
}

impl SerialConnection {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut subscriptions = Vec::new();
        if let Some(monitor) = cx.try_global::<GlobalBoardMonitor>() {
            let monitor = monitor.0.clone();
            subscriptions.push(cx.subscribe(&monitor, Self::handle_flash_started));
            subscriptions.push(cx.subscribe(&monitor, Self::handle_flash_finished));
        }

        Self {
            port_name: None,
            baud_rate: DEFAULT_BAUD_RATE,
            is_open: false,
            lines: VecDeque::new(),
            open: None,
            paused: None,
            _subscriptions: subscriptions,
        }
    }

    fn handle_flash_started(
        &mut self,
        _monitor: Entity<BoardMonitor>,
        event: &FlashStarted,
        cx: &mut Context<Self>,
    ) {
        if self.open.is_some() && self.port_name.as_deref() == Some(event.port_name.as_str()) {
            self.pause_for_flash(cx);
        }
    }

    fn handle_flash_finished(
        &mut self,
        _monitor: Entity<BoardMonitor>,
        event: &FlashFinished,
        cx: &mut Context<Self>,
    ) {
        let should_resume = self
            .paused
            .as_ref()
            .is_some_and(|(port_name, _)| port_name == &event.port_name);
        if should_resume {
            self.resume_after_flash(cx);
        }
    }

    pub fn connect(&mut self, port_name: String, baud_rate: u32, cx: &mut Context<Self>) {
        self.close_port(cx);
        self.paused = None;
        self.port_name = Some(port_name.clone());
        self.baud_rate = baud_rate;

        let (message_tx, mut message_rx) = unbounded::<SerialConnectionMessage>();
        let handle: Arc<Mutex<PortSlot>> = Arc::new(Mutex::new(PortSlot::Connecting));
        self.open = Some(OpenPort {
            handle: handle.clone(),
            message_tx: message_tx.clone(),
        });

        cx.background_spawn(async move {
            run_serial_reader(port_name, baud_rate, handle, message_tx).await;
        })
        .detach();

        cx.spawn(async move |this, cx| -> anyhow::Result<()> {
            while let Some(message) = message_rx.next().await {
                this.update(cx, |this, cx| match message {
                    SerialConnectionMessage::Opened => {
                        this.is_open = true;
                        cx.notify();
                    }
                    SerialConnectionMessage::Line(line) => {
                        this.push_line(line, cx);
                    }
                    SerialConnectionMessage::Error(error) => {
                        this.close_port(cx);
                        cx.emit(SerialConnectionError(error));
                    }
                })?;
            }
            anyhow::Ok(())
        })
        .detach();
    }

    pub fn disconnect(&mut self, cx: &mut Context<Self>) {
        self.close_port(cx);
        self.port_name = None;
        self.paused = None;
    }

    pub fn send(&self, bytes: Vec<u8>, cx: &mut Context<Self>) {
        let Some(open) = &self.open else { return };
        let handle = open.handle.clone();
        let message_tx = open.message_tx.clone();
        cx.background_spawn(async move {
            let write_result = {
                let Ok(mut guard) = handle.lock() else { return };
                match &mut *guard {
                    PortSlot::Open(port) => port.write_all(&bytes).map_err(|error| error.to_string()),
                    PortSlot::Connecting | PortSlot::Closed => return,
                }
            };
            if let Err(error) = write_result {
                // ponytail: best-effort -- if the foreground entity (and its
                // message_rx loop) is already gone, there's no one left to
                // tell and nothing to recover.
                message_tx
                    .unbounded_send(SerialConnectionMessage::Error(error))
                    .ok();
            }
        })
        .detach();
    }

    fn pause_for_flash(&mut self, cx: &mut Context<Self>) {
        let Some(port_name) = self.port_name.clone() else {
            return;
        };
        self.paused = Some((port_name, self.baud_rate));
        self.close_port(cx);
    }

    fn resume_after_flash(&mut self, cx: &mut Context<Self>) {
        if let Some((port_name, baud_rate)) = self.paused.take() {
            self.connect(port_name, baud_rate, cx);
        }
    }

    /// Tears down the open handle (if any) without touching `port_name` or
    /// `paused` -- shared by `disconnect` (which clears both after) and
    /// `pause_for_flash` (which needs `paused` to survive).
    ///
    /// Clears the mutex inline (blocking this thread briefly) rather than in
    /// a detached background task, so an already-open OS port handle is
    /// guaranteed closed by the time this returns, and a still-opening one
    /// (state `PortSlot::Connecting`) is guaranteed to be dropped the moment
    /// its `serialport::open()` call returns rather than published into the
    /// read loop -- see `run_serial_reader`. `pause_for_flash` depends on
    /// this: avrdude must never race the reader thread for the same port.
    /// (One unavoidable exception: if a port is still mid-`open()` when this
    /// runs, the OS call itself can't be cancelled, so the handle may stay
    /// briefly held until that blocking call returns and notices `Closed`.)
    /// The reader thread only ever holds this mutex for the duration of one
    /// `port.read()` call, bounded by the port's 100ms read timeout, and
    /// never awaits anything while holding it, so this lock acquires
    /// quickly (worst case ~100ms) with no deadlock risk.
    fn close_port(&mut self, cx: &mut Context<Self>) {
        if let Some(open) = self.open.take() {
            if let Ok(mut guard) = open.handle.lock() {
                *guard = PortSlot::Closed;
            }
        }
        self.is_open = false;
        cx.notify();
    }

    fn push_line(&mut self, line: String, cx: &mut Context<Self>) {
        self.lines.push_back(line.clone());
        if self.lines.len() > MAX_BUFFERED_LINES {
            self.lines.pop_front();
        }
        cx.emit(SerialLineReceived(line));
        cx.notify();
    }
}

/// Runs on a background thread for the lifetime of one connection. Opens
/// the port, publishes the handle into `handle` for `send()` to use, then
/// loops reading with a short timeout (so it notices `handle` being cleared
/// to `None` by `close_port` within about 100ms) until the port closes or
/// errors.
async fn run_serial_reader(
    port_name: String,
    baud_rate: u32,
    handle: Arc<Mutex<PortSlot>>,
    message_tx: UnboundedSender<SerialConnectionMessage>,
) {
    let port = match serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()
    {
        Ok(port) => port,
        Err(error) => {
            // ponytail: best-effort -- if the foreground entity (and its
            // message_rx loop) is already gone, there's no one left to
            // tell and nothing to recover.
            message_tx
                .unbounded_send(SerialConnectionMessage::Error(error.to_string()))
                .ok();
            return;
        }
    };

    {
        let Ok(mut guard) = handle.lock() else { return };
        // close_port may have run while `open()` above was still blocking --
        // it can only have set the slot to `Closed` (nothing else touches
        // this mutex before we do). Publishing `Open(port)` over that would
        // leak the handle forever: nothing would ever tell this new port to
        // close. Drop it and bail instead of entering the read loop.
        if matches!(&*guard, PortSlot::Closed) {
            return;
        }
        *guard = PortSlot::Open(port);
    }
    if message_tx
        .unbounded_send(SerialConnectionMessage::Opened)
        .is_err()
    {
        return;
    }

    let mut carry = Vec::new();
    let mut read_buf = [0u8; 1024];
    loop {
        let read_result = {
            let Ok(mut guard) = handle.lock() else { return };
            match &mut *guard {
                PortSlot::Open(port) => port.read(&mut read_buf),
                PortSlot::Closed => return, // closed by close_port
                PortSlot::Connecting => return, // unreachable: we already transitioned out above
            }
        };

        match read_result {
            Ok(0) => continue,
            Ok(byte_count) => {
                for line in split_lines(&mut carry, &read_buf[..byte_count]) {
                    if message_tx
                        .unbounded_send(SerialConnectionMessage::Line(line))
                        .is_err()
                    {
                        return;
                    }
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(error) => {
                // ponytail: best-effort -- if the foreground entity (and its
                // message_rx loop) is already gone, there's no one left to
                // tell and nothing to recover.
                message_tx
                    .unbounded_send(SerialConnectionMessage::Error(error.to_string()))
                    .ok();
                return;
            }
        }
    }
}

#[cfg(test)]
mod connection_tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_pause_for_flash_when_never_connected_is_a_no_op(cx: &mut TestAppContext) {
        let connection = cx.new(SerialConnection::new);

        connection.update(cx, |connection, cx| connection.pause_for_flash(cx));

        connection.read_with(cx, |connection, _cx| {
            assert_eq!(connection.port_name, None);
            assert_eq!(connection.baud_rate, DEFAULT_BAUD_RATE);
            assert!(!connection.is_open);
            assert!(connection.paused.is_none());
        });
    }

    #[gpui::test]
    async fn test_resume_after_flash_when_nothing_paused_is_a_no_op(cx: &mut TestAppContext) {
        let connection = cx.new(SerialConnection::new);

        connection.update(cx, |connection, cx| connection.resume_after_flash(cx));

        connection.read_with(cx, |connection, _cx| {
            assert_eq!(connection.port_name, None);
            assert!(!connection.is_open);
            assert!(connection.open.is_none());
            assert!(connection.paused.is_none());
        });
    }
}
