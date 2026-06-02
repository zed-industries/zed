use std::{path::PathBuf, sync::mpsc, thread::Builder};

use anyhow::{Context as _, Result, anyhow};
use futures::channel::mpsc::UnboundedSender;
use gpui::{Keystroke, Modifiers, MouseButton, ScrollWheelEvent};
use log::debug;
use util::paths::PathStyle;

use crate::{
    Content, CursorShape, Point, PtyEvent, Range, TerminalBackendEvent, TerminalBounds,
    ghostty_backend::{FullContentBuilder, GhosttyBackend, GhosttyOsc52},
};

#[derive(Clone)]
pub(super) struct GhosttyBackendWorker {
    sender: mpsc::Sender<GhosttyBackendCommand>,
}

enum GhosttyBackendCommand {
    WriteOutput {
        bytes: Vec<u8>,
        reply: Option<mpsc::Sender<Vec<TerminalBackendEvent>>>,
    },
    Resize {
        bounds: TerminalBounds,
        reply: mpsc::Sender<Result<Vec<TerminalBackendEvent>>>,
    },
    Clear {
        reply: mpsc::Sender<Result<Vec<TerminalBackendEvent>>>,
    },
    SetAlternateScroll {
        enabled: bool,
        reply: mpsc::Sender<Result<Vec<TerminalBackendEvent>>>,
    },
    SetDefaultCursorShape(CursorShape),
    SetOsc52(GhosttyOsc52),
    SetDarkColorScheme(bool),
    ScrollLineUp,
    ScrollLineDown,
    ScrollToTop,
    ScrollToBottom,
    ScrollToPoint {
        point: Point,
        display_offset: usize,
        viewport_lines: usize,
    },
    EncodeKey {
        keystroke: Keystroke,
        option_as_meta: bool,
        reply: mpsc::Sender<Result<Option<Vec<u8>>>>,
    },
    EncodeFocus {
        gained: bool,
        reply: mpsc::Sender<Result<Option<Vec<u8>>>>,
    },
    EncodeMouseButton {
        point: Point,
        bounds: TerminalBounds,
        button: MouseButton,
        modifiers: Modifiers,
        pressed: bool,
        reply: mpsc::Sender<Result<Option<Vec<u8>>>>,
    },
    EncodeMouseMotion {
        point: Point,
        bounds: TerminalBounds,
        button: Option<MouseButton>,
        modifiers: Modifiers,
        reply: mpsc::Sender<Result<Option<Vec<u8>>>>,
    },
    EncodeMouseScroll {
        point: Point,
        bounds: TerminalBounds,
        scroll_lines: i32,
        event: ScrollWheelEvent,
        reply: mpsc::Sender<Result<Vec<Vec<u8>>>>,
    },
    TotalLines {
        reply: mpsc::Sender<Result<usize>>,
    },
    ViewportLines {
        reply: mpsc::Sender<Result<usize>>,
    },
    Content {
        last_content: Content,
        reply: mpsc::Sender<Result<(Content, bool)>>,
    },
    FormattedContent {
        reply: mpsc::Sender<Result<String>>,
    },
    FullContentRange {
        reply: mpsc::Sender<Result<Option<Range>>>,
    },
    FullContent {
        last_content: Content,
        reply: mpsc::Sender<Result<Content>>,
    },
    StartFullContent {
        last_content: Content,
        reply: mpsc::Sender<Result<FullContentBuilder>>,
    },
    AppendFullContentRows {
        builder: FullContentBuilder,
        row_count: usize,
        reply: mpsc::Sender<Result<(FullContentBuilder, bool)>>,
    },
    FirstOccupiedColumn {
        line: i32,
        reply: mpsc::Sender<Result<Option<usize>>>,
    },
    WorkingDirectory {
        path_style: PathStyle,
        reply: mpsc::Sender<Result<Option<PathBuf>>>,
    },
    DrainEvents {
        reply: mpsc::Sender<Vec<TerminalBackendEvent>>,
    },
}

impl GhosttyBackendWorker {
    pub(super) fn new(
        bounds: TerminalBounds,
        scrollback_lines: Option<usize>,
        events_tx: Option<UnboundedSender<PtyEvent>>,
    ) -> Result<Self> {
        let (sender, receiver) = mpsc::channel();
        let (init_tx, init_rx) = mpsc::channel();

        Builder::new()
            .name("ghostty-backend".to_string())
            .spawn(move || {
                let mut backend = match GhosttyBackend::new(bounds, scrollback_lines) {
                    Ok(backend) => {
                        send_init_result(init_tx, Ok(()));
                        backend
                    }
                    Err(error) => {
                        send_init_result(init_tx, Err(error));
                        return;
                    }
                };

                while let Ok(command) = receiver.recv() {
                    Self::process_command(&mut backend, &events_tx, command);
                }
            })
            .context("failed to spawn ghostty backend thread")?;

        match init_rx
            .recv()
            .map_err(|_| anyhow!("ghostty backend thread stopped during startup"))?
        {
            Ok(()) => Ok(Self { sender }),
            Err(error) => Err(error),
        }
    }

    pub(super) fn write_output_from_pty(&self, bytes: Vec<u8>) {
        self.send(GhosttyBackendCommand::WriteOutput { bytes, reply: None });
    }

    pub(super) fn write_output(&self, bytes: &[u8]) -> Result<Vec<TerminalBackendEvent>> {
        let bytes = bytes.to_vec();
        let (reply, receiver) = mpsc::channel();
        self.send_request(GhosttyBackendCommand::WriteOutput {
            bytes,
            reply: Some(reply),
        })?;
        receive_response(receiver)
    }

    pub(super) fn resize(&self, bounds: TerminalBounds) -> Result<Vec<TerminalBackendEvent>> {
        self.request(|reply| GhosttyBackendCommand::Resize { bounds, reply })
    }

    pub(super) fn clear(&self) -> Result<Vec<TerminalBackendEvent>> {
        self.request(|reply| GhosttyBackendCommand::Clear { reply })
    }

    pub(super) fn set_alternate_scroll(&self, enabled: bool) -> Result<Vec<TerminalBackendEvent>> {
        self.request(|reply| GhosttyBackendCommand::SetAlternateScroll { enabled, reply })
    }

    pub(super) fn set_default_cursor_shape(&self, cursor_shape: CursorShape) {
        self.send(GhosttyBackendCommand::SetDefaultCursorShape(cursor_shape));
    }

    pub(super) fn set_osc52(&self, osc52: GhosttyOsc52) {
        self.send(GhosttyBackendCommand::SetOsc52(osc52));
    }

    pub(super) fn set_dark_color_scheme(&self, is_dark: bool) {
        self.send(GhosttyBackendCommand::SetDarkColorScheme(is_dark));
    }

    pub(super) fn scroll_line_up(&self) {
        self.send(GhosttyBackendCommand::ScrollLineUp);
    }

    pub(super) fn scroll_line_down(&self) {
        self.send(GhosttyBackendCommand::ScrollLineDown);
    }

    pub(super) fn scroll_to_top(&self) {
        self.send(GhosttyBackendCommand::ScrollToTop);
    }

    pub(super) fn scroll_to_bottom(&self) {
        self.send(GhosttyBackendCommand::ScrollToBottom);
    }

    pub(super) fn scroll_to_point(
        &self,
        point: Point,
        display_offset: usize,
        viewport_lines: usize,
    ) {
        self.send(GhosttyBackendCommand::ScrollToPoint {
            point,
            display_offset,
            viewport_lines,
        });
    }

    pub(super) fn encode_key(
        &self,
        keystroke: &Keystroke,
        option_as_meta: bool,
    ) -> Result<Option<Vec<u8>>> {
        self.request(|reply| GhosttyBackendCommand::EncodeKey {
            keystroke: keystroke.clone(),
            option_as_meta,
            reply,
        })
    }

    pub(super) fn encode_focus(&self, gained: bool) -> Result<Option<Vec<u8>>> {
        self.request(|reply| GhosttyBackendCommand::EncodeFocus { gained, reply })
    }

    pub(super) fn encode_mouse_button(
        &self,
        point: Point,
        bounds: TerminalBounds,
        button: MouseButton,
        modifiers: Modifiers,
        pressed: bool,
    ) -> Result<Option<Vec<u8>>> {
        self.request(|reply| GhosttyBackendCommand::EncodeMouseButton {
            point,
            bounds,
            button,
            modifiers,
            pressed,
            reply,
        })
    }

    pub(super) fn encode_mouse_motion(
        &self,
        point: Point,
        bounds: TerminalBounds,
        button: Option<MouseButton>,
        modifiers: Modifiers,
    ) -> Result<Option<Vec<u8>>> {
        self.request(|reply| GhosttyBackendCommand::EncodeMouseMotion {
            point,
            bounds,
            button,
            modifiers,
            reply,
        })
    }

    pub(super) fn encode_mouse_scroll(
        &self,
        point: Point,
        bounds: TerminalBounds,
        scroll_lines: i32,
        event: &ScrollWheelEvent,
    ) -> Result<Vec<Vec<u8>>> {
        self.request(|reply| GhosttyBackendCommand::EncodeMouseScroll {
            point,
            bounds,
            scroll_lines,
            event: event.clone(),
            reply,
        })
    }

    pub(super) fn total_lines(&self) -> Result<usize> {
        self.request(|reply| GhosttyBackendCommand::TotalLines { reply })
    }

    pub(super) fn viewport_lines(&self) -> Result<usize> {
        self.request(|reply| GhosttyBackendCommand::ViewportLines { reply })
    }

    pub(super) fn content(&self, last_content: &Content) -> Result<(Content, bool)> {
        self.request(|reply| GhosttyBackendCommand::Content {
            last_content: last_content.clone(),
            reply,
        })
    }

    pub(super) fn formatted_content(&self) -> Result<String> {
        self.request(|reply| GhosttyBackendCommand::FormattedContent { reply })
    }

    pub(super) fn full_content_range(&self) -> Result<Option<Range>> {
        self.request(|reply| GhosttyBackendCommand::FullContentRange { reply })
    }

    pub(super) fn full_content(&self, last_content: &Content) -> Result<Content> {
        self.request(|reply| GhosttyBackendCommand::FullContent {
            last_content: last_content.clone(),
            reply,
        })
    }

    pub(super) fn start_full_content(&self, last_content: &Content) -> Result<FullContentBuilder> {
        self.request(|reply| GhosttyBackendCommand::StartFullContent {
            last_content: last_content.clone(),
            reply,
        })
    }

    pub(super) fn append_full_content_rows(
        &self,
        builder: FullContentBuilder,
        row_count: usize,
    ) -> Result<(FullContentBuilder, bool)> {
        self.request(|reply| GhosttyBackendCommand::AppendFullContentRows {
            builder,
            row_count,
            reply,
        })
    }

    pub(super) fn first_occupied_column(&self, line: i32) -> Result<Option<usize>> {
        self.request(|reply| GhosttyBackendCommand::FirstOccupiedColumn { line, reply })
    }

    pub(super) fn working_directory(&self, path_style: PathStyle) -> Result<Option<PathBuf>> {
        self.request(|reply| GhosttyBackendCommand::WorkingDirectory { path_style, reply })
    }

    pub(super) fn drain_events(&self) -> Vec<TerminalBackendEvent> {
        let (reply, receiver) = mpsc::channel();
        if self
            .send_request(GhosttyBackendCommand::DrainEvents { reply })
            .is_err()
        {
            return Vec::new();
        }
        receiver.recv().unwrap_or_default()
    }

    fn request<T>(
        &self,
        build_command: impl FnOnce(mpsc::Sender<Result<T>>) -> GhosttyBackendCommand,
    ) -> Result<T> {
        let (reply, receiver) = mpsc::channel();
        self.send_request(build_command(reply))?;
        receive_result(receiver)
    }

    fn send_request(&self, command: GhosttyBackendCommand) -> Result<()> {
        self.sender
            .send(command)
            .map_err(|_| anyhow!("ghostty backend worker stopped"))
    }

    fn send(&self, command: GhosttyBackendCommand) {
        if let Err(error) = self.sender.send(command) {
            debug!("failed to send ghostty backend command: {error}");
        }
    }

    fn process_command(
        backend: &mut GhosttyBackend,
        events_tx: &Option<UnboundedSender<PtyEvent>>,
        command: GhosttyBackendCommand,
    ) {
        match command {
            GhosttyBackendCommand::WriteOutput { bytes, reply } => {
                backend.write_output(&bytes);
                let events = backend.drain_events();
                if let Some(reply) = reply {
                    respond(reply, events);
                } else if let Some(events_tx) = events_tx {
                    send_pty_event(
                        events_tx,
                        PtyEvent::OutputProcessed(events),
                        "backend-output",
                    );
                } else {
                    debug!("ghostty backend output processed without event receiver");
                }
            }
            GhosttyBackendCommand::Resize { bounds, reply } => {
                let response = backend.resize(bounds).map(|()| backend.drain_events());
                respond(reply, response);
            }
            GhosttyBackendCommand::Clear { reply } => {
                let response = backend.clear().map(|()| backend.drain_events());
                respond(reply, response);
            }
            GhosttyBackendCommand::SetAlternateScroll { enabled, reply } => {
                let response = backend
                    .set_alternate_scroll(enabled)
                    .map(|()| backend.drain_events());
                respond(reply, response);
            }
            GhosttyBackendCommand::SetDefaultCursorShape(cursor_shape) => {
                backend.set_default_cursor_shape(cursor_shape);
            }
            GhosttyBackendCommand::SetOsc52(osc52) => backend.set_osc52(osc52),
            GhosttyBackendCommand::SetDarkColorScheme(is_dark) => {
                backend.set_dark_color_scheme(is_dark);
            }
            GhosttyBackendCommand::ScrollLineUp => backend.scroll_line_up(),
            GhosttyBackendCommand::ScrollLineDown => backend.scroll_line_down(),
            GhosttyBackendCommand::ScrollToTop => backend.scroll_to_top(),
            GhosttyBackendCommand::ScrollToBottom => backend.scroll_to_bottom(),
            GhosttyBackendCommand::ScrollToPoint {
                point,
                display_offset,
                viewport_lines,
            } => backend.scroll_to_point(point, display_offset, viewport_lines),
            GhosttyBackendCommand::EncodeKey {
                keystroke,
                option_as_meta,
                reply,
            } => respond(reply, backend.encode_key(&keystroke, option_as_meta)),
            GhosttyBackendCommand::EncodeFocus { gained, reply } => {
                respond(reply, backend.encode_focus(gained));
            }
            GhosttyBackendCommand::EncodeMouseButton {
                point,
                bounds,
                button,
                modifiers,
                pressed,
                reply,
            } => respond(
                reply,
                backend.encode_mouse_button(point, bounds, button, modifiers, pressed),
            ),
            GhosttyBackendCommand::EncodeMouseMotion {
                point,
                bounds,
                button,
                modifiers,
                reply,
            } => respond(
                reply,
                backend.encode_mouse_motion(point, bounds, button, modifiers),
            ),
            GhosttyBackendCommand::EncodeMouseScroll {
                point,
                bounds,
                scroll_lines,
                event,
                reply,
            } => respond(
                reply,
                backend.encode_mouse_scroll(point, bounds, scroll_lines, &event),
            ),
            GhosttyBackendCommand::TotalLines { reply } => {
                respond(reply, backend.total_lines());
            }
            GhosttyBackendCommand::ViewportLines { reply } => {
                respond(reply, backend.viewport_lines());
            }
            GhosttyBackendCommand::Content {
                last_content,
                reply,
            } => {
                let response = backend
                    .content(&last_content)
                    .map(|content| (content, backend.cursor_blinking()));
                respond(reply, response);
            }
            GhosttyBackendCommand::FormattedContent { reply } => {
                respond(reply, backend.formatted_content());
            }
            GhosttyBackendCommand::FullContentRange { reply } => {
                respond(reply, backend.full_content_range());
            }
            GhosttyBackendCommand::FullContent {
                last_content,
                reply,
            } => respond(reply, backend.full_content(&last_content)),
            GhosttyBackendCommand::StartFullContent {
                last_content,
                reply,
            } => respond(reply, backend.start_full_content(&last_content)),
            GhosttyBackendCommand::AppendFullContentRows {
                mut builder,
                row_count,
                reply,
            } => {
                let response = backend
                    .append_full_content_rows(&mut builder, row_count)
                    .map(|done| (builder, done));
                respond(reply, response);
            }
            GhosttyBackendCommand::FirstOccupiedColumn { line, reply } => {
                respond(reply, backend.first_occupied_column(line));
            }
            GhosttyBackendCommand::WorkingDirectory { path_style, reply } => {
                respond(reply, backend.working_directory(path_style));
            }
            GhosttyBackendCommand::DrainEvents { reply } => {
                respond(reply, backend.drain_events());
            }
        }
    }
}

fn send_init_result(sender: mpsc::Sender<Result<()>>, result: Result<()>) {
    if let Err(error) = sender.send(result) {
        debug!("failed to send ghostty backend startup result: {error}");
    }
}

fn send_pty_event(
    events_tx: &UnboundedSender<PtyEvent>,
    event: PtyEvent,
    description: &'static str,
) {
    if let Err(error) = events_tx.unbounded_send(event) {
        debug!("failed to send ghostty backend {description} event: {error}");
    }
}

fn receive_result<T>(receiver: mpsc::Receiver<Result<T>>) -> Result<T> {
    receiver
        .recv()
        .map_err(|_| anyhow!("ghostty backend worker stopped"))?
}

fn receive_response<T>(receiver: mpsc::Receiver<T>) -> Result<T> {
    receiver
        .recv()
        .map_err(|_| anyhow!("ghostty backend worker stopped"))
}

fn respond<T>(sender: mpsc::Sender<T>, response: T) {
    if sender.send(response).is_err() {
        debug!("failed to send ghostty backend response");
    }
}
