use std::{cell::RefCell, collections::VecDeque, path::PathBuf, rc::Rc, sync::Arc};

use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD as Base64};
use gpui::{Keystroke, Modifiers, MouseButton, ScrollWheelEvent, px};
use libghostty_vt::{
    RenderState, Terminal as GhosttyTerminal, TerminalOptions,
    ffi::{
        GhosttyColorScheme, GhosttyColorScheme_GHOSTTY_COLOR_SCHEME_DARK,
        GhosttyColorScheme_GHOSTTY_COLOR_SCHEME_LIGHT,
    },
    fmt::{Format, Formatter, FormatterOptions},
    focus::Event as GhosttyFocusEvent,
    key::{
        Action as GhosttyKeyAction, Encoder as GhosttyKeyEncoder, Event as GhosttyKeyEvent,
        Key as GhosttyKey, Mods as GhosttyKeyMods, OptionAsAlt,
    },
    mouse::{
        Action as GhosttyMouseAction, Button as GhosttyMouseButton, Encoder as GhosttyMouseEncoder,
        EncoderSize as GhosttyMouseEncoderSize, Event as GhosttyMouseEvent,
        Position as GhosttyMousePosition,
    },
    render::{CellIterator, CursorVisualStyle, RowIterator},
    screen::CellWide,
    style::{RgbColor, Style, StyleColor, Underline},
    terminal::{
        ConformanceLevel, DeviceAttributes, DeviceType, Mode, PrimaryDeviceAttributes,
        ScrollViewport, SecondaryDeviceAttributes, SizeReportSize, TertiaryDeviceAttributes,
    },
};
use url::Url;
use util::paths::{PathStyle, UrlExt};

use crate::{
    IndexedCell, MAX_SCROLL_HISTORY_LINES, TerminalBackendEvent, TerminalBounds, TerminalCell,
    TerminalCellFlags, TerminalColor, TerminalContent, TerminalCursor, TerminalCursorShape,
    TerminalHyperlink, TerminalModes, TerminalNamedColor, TerminalPoint, TerminalRgb,
};

pub struct GhosttyBackend {
    terminal: Box<GhosttyTerminal<'static, 'static>>,
    render_state: RenderState<'static>,
    row_iterator: RowIterator<'static>,
    cell_iterator: CellIterator<'static>,
    key_encoder: GhosttyKeyEncoder<'static>,
    mouse_encoder: GhosttyMouseEncoder<'static>,
    events: Rc<RefCell<VecDeque<TerminalBackendEvent>>>,
    size_report: Rc<RefCell<SizeReportSize>>,
    color_scheme: Rc<RefCell<Option<GhosttyColorScheme>>>,
    osc_state: OscState,
    osc52: GhosttyOsc52,
    colors: [Option<TerminalRgb>; TERMINAL_COLOR_COUNT],
    working_directory_report: Option<String>,
    default_cursor_shape: TerminalCursorShape,
    cursor_blinking: bool,
}

#[cfg_attr(not(feature = "alacritty-backend"), allow(dead_code))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GhosttyOsc52 {
    Disabled,
    #[default]
    OnlyCopy,
    OnlyPaste,
    CopyPaste,
}

#[derive(Default)]
enum OscState {
    #[default]
    Ground,
    Escape,
    Command(Vec<u8>),
    CommandEscape(Vec<u8>),
    Unsupported,
    UnsupportedEscape,
    Osc7Payload(Vec<u8>),
    Osc7PayloadEscape(Vec<u8>),
    OscColorPayload {
        command: Vec<u8>,
        payload: Vec<u8>,
    },
    OscColorPayloadEscape {
        command: Vec<u8>,
        payload: Vec<u8>,
    },
    Osc52Clipboard(Vec<u8>),
    Osc52Payload {
        clipboard: Vec<u8>,
        payload: Vec<u8>,
    },
    Osc52PayloadEscape {
        clipboard: Vec<u8>,
        payload: Vec<u8>,
    },
}

const MAX_OSC_PAYLOAD_LEN: usize = 4096;
const MAX_OSC52_PAYLOAD_LEN: usize = 1024 * 1024;
const TERMINAL_COLOR_COUNT: usize = 269;
const TERMINAL_FOREGROUND_COLOR_INDEX: usize = 256;
const TERMINAL_BACKGROUND_COLOR_INDEX: usize = 257;
const TERMINAL_CURSOR_COLOR_INDEX: usize = 258;

fn size_report_from_bounds(bounds: TerminalBounds) -> SizeReportSize {
    SizeReportSize {
        rows: bounds.num_lines().max(1).min(u16::MAX as usize) as u16,
        columns: bounds.num_columns().max(1).min(u16::MAX as usize) as u16,
        cell_width: f32::from(bounds.cell_width()).max(1.0) as u32,
        cell_height: f32::from(bounds.line_height()).max(1.0) as u32,
    }
}

fn ghostty_device_attributes() -> DeviceAttributes {
    DeviceAttributes {
        primary: PrimaryDeviceAttributes::new(ConformanceLevel::VT102, []),
        secondary: SecondaryDeviceAttributes {
            device_type: DeviceType(0),
            firmware_version: terminal_version_number(env!("CARGO_PKG_VERSION")),
            rom_cartridge: 1,
        },
        tertiary: TertiaryDeviceAttributes::default(),
    }
}

fn terminal_version_number(mut version: &str) -> u16 {
    if let Some(separator) = version.find('-') {
        version = &version[..separator];
    }

    let mut version_number = 0u32;
    for segment in version.split('.').take(3) {
        let segment = segment.parse::<u32>().unwrap_or(0).min(99);
        version_number = version_number.saturating_mul(100).saturating_add(segment);
    }

    u16::try_from(version_number).unwrap_or(u16::MAX)
}

impl GhosttyBackend {
    pub fn new(bounds: TerminalBounds, scrollback_lines: Option<usize>) -> Result<Self> {
        let rows = bounds.num_lines().max(1).min(u16::MAX as usize) as u16;
        let cols = bounds.num_columns().max(1).min(u16::MAX as usize) as u16;
        let mut terminal = Box::new(GhosttyTerminal::new(TerminalOptions {
            cols,
            rows,
            max_scrollback: scrollback_lines
                .unwrap_or(super::DEFAULT_SCROLL_HISTORY_LINES)
                .min(MAX_SCROLL_HISTORY_LINES),
        })?);

        let events = Rc::new(RefCell::new(VecDeque::new()));
        let size_report = Rc::new(RefCell::new(size_report_from_bounds(bounds)));
        let color_scheme = Rc::new(RefCell::new(None));

        terminal
            .on_pty_write({
                let events = events.clone();
                move |_terminal, data| {
                    let data = String::from_utf8_lossy(data).into_owned();
                    events
                        .borrow_mut()
                        .push_back(TerminalBackendEvent::PtyWrite(data));
                }
            })?
            .on_bell({
                let events = events.clone();
                move |_terminal| {
                    events.borrow_mut().push_back(TerminalBackendEvent::Bell);
                }
            })?
            .on_title_changed({
                let events = events.clone();
                move |terminal| match terminal.title() {
                    Ok(title) => events
                        .borrow_mut()
                        .push_back(TerminalBackendEvent::Title(title.to_string())),
                    Err(error) => log::error!("failed to read ghostty terminal title: {error}"),
                }
            })?
            .on_size({
                let size_report = size_report.clone();
                move |_terminal| Some(*size_report.borrow())
            })?
            .on_color_scheme({
                let color_scheme = color_scheme.clone();
                move |_terminal| *color_scheme.borrow()
            })?
            .on_device_attributes(|_terminal| Some(ghostty_device_attributes()))?;

        Ok(Self {
            terminal,
            render_state: RenderState::new()?,
            row_iterator: RowIterator::new()?,
            cell_iterator: CellIterator::new()?,
            key_encoder: GhosttyKeyEncoder::new()?,
            mouse_encoder: GhosttyMouseEncoder::new()?,
            events,
            size_report,
            color_scheme,
            osc_state: OscState::Ground,
            osc52: GhosttyOsc52::default(),
            colors: [None; TERMINAL_COLOR_COUNT],
            working_directory_report: None,
            default_cursor_shape: TerminalCursorShape::Block,
            cursor_blinking: false,
        })
    }

    pub fn resize(&mut self, bounds: TerminalBounds) -> Result<()> {
        self.terminal.resize(
            bounds.num_columns().max(1).min(u16::MAX as usize) as u16,
            bounds.num_lines().max(1).min(u16::MAX as usize) as u16,
            f32::from(bounds.cell_width()).max(1.0) as u32,
            f32::from(bounds.line_height()).max(1.0) as u32,
        )?;
        *self.size_report.borrow_mut() = size_report_from_bounds(bounds);
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.osc_state = OscState::Ground;
        let cursor_y = self.terminal.cursor_y()?;
        let rows = self.terminal.rows()?;

        let mut sequence = b"\x1b\\".to_vec();
        if cursor_y > 0 {
            sequence.extend_from_slice(b"\x1b[");
            sequence.extend_from_slice(cursor_y.to_string().as_bytes());
            sequence.extend_from_slice(b"S\x1b[");
            sequence.extend_from_slice(cursor_y.to_string().as_bytes());
            sequence.extend_from_slice(b"A");
        }
        if rows > 1 {
            sequence.extend_from_slice(b"\x1b7\x1b[1B\x1b[1G\x1b[0J\x1b8");
        }
        sequence.extend_from_slice(b"\x1b[3J");
        self.terminal.vt_write(&sequence);
        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode, value: bool) -> Result<()> {
        Ok(self.terminal.set_mode(mode, value)?)
    }

    pub fn set_default_cursor_shape(&mut self, cursor_shape: TerminalCursorShape) {
        self.default_cursor_shape = cursor_shape;
    }

    pub fn set_osc52(&mut self, osc52: GhosttyOsc52) {
        self.osc52 = osc52;
    }

    pub fn set_dark_color_scheme(&mut self, is_dark: bool) {
        *self.color_scheme.borrow_mut() = Some(if is_dark {
            GhosttyColorScheme_GHOSTTY_COLOR_SCHEME_DARK
        } else {
            GhosttyColorScheme_GHOSTTY_COLOR_SCHEME_LIGHT
        });
    }

    pub fn scroll_line_up(&mut self) {
        self.terminal.scroll_viewport(ScrollViewport::Delta(-1));
    }

    pub fn scroll_line_down(&mut self) {
        self.terminal.scroll_viewport(ScrollViewport::Delta(1));
    }

    pub fn scroll_to_top(&mut self) {
        self.terminal.scroll_viewport(ScrollViewport::Top);
    }

    pub fn scroll_to_bottom(&mut self) {
        self.terminal.scroll_viewport(ScrollViewport::Bottom);
    }

    pub fn scroll_to_point(
        &mut self,
        point: TerminalPoint,
        display_offset: usize,
        viewport_lines: usize,
    ) {
        let display_offset = i32::try_from(display_offset).unwrap_or(i32::MAX);
        let viewport_end = i32::try_from(viewport_lines.saturating_sub(1)).unwrap_or(i32::MAX);
        let top_line = -display_offset;
        let bottom_line = top_line.saturating_add(viewport_end);

        let delta = if point.line < top_line {
            point.line.saturating_sub(top_line)
        } else if point.line > bottom_line {
            point.line.saturating_sub(bottom_line)
        } else {
            0
        };

        if delta != 0 {
            self.terminal
                .scroll_viewport(ScrollViewport::Delta(delta as isize));
        }
    }

    pub fn write_output(&mut self, bytes: &[u8]) {
        self.observe_output(bytes);
        self.terminal.vt_write(bytes);
    }

    pub fn encode_key(
        &mut self,
        keystroke: &Keystroke,
        option_as_meta: bool,
    ) -> Result<Option<Vec<u8>>> {
        if is_zed_shift_enter(keystroke) {
            return Ok(Some(b"\n".to_vec()));
        }
        if is_plain_text_keystroke(keystroke) {
            return Ok(None);
        }

        let Some(key) = ghostty_key(&keystroke.key) else {
            return Ok(None);
        };

        let mut event = GhosttyKeyEvent::new()?;
        event
            .set_action(GhosttyKeyAction::Press)
            .set_key(key)
            .set_mods(ghostty_key_mods(keystroke))
            .set_consumed_mods(GhosttyKeyMods::empty());

        if let Some(codepoint) = ghostty_unshifted_codepoint(&keystroke.key) {
            event.set_unshifted_codepoint(codepoint);
        }
        if let Some(text) = ghostty_key_text(keystroke) {
            event.set_utf8(Some(text));
        }

        self.key_encoder
            .set_options_from_terminal(self.terminal.as_ref())
            .set_macos_option_as_alt(if option_as_meta {
                OptionAsAlt::True
            } else {
                OptionAsAlt::False
            });

        let mut bytes = Vec::new();
        self.key_encoder.encode_to_vec(&event, &mut bytes)?;
        Ok((!bytes.is_empty()).then_some(bytes))
    }

    pub fn encode_focus(&self, gained: bool) -> Result<Option<Vec<u8>>> {
        if !self.mode_enabled(Mode::FOCUS_EVENT) {
            return Ok(None);
        }

        let event = if gained {
            GhosttyFocusEvent::Gained
        } else {
            GhosttyFocusEvent::Lost
        };
        let mut bytes = [0u8; 16];
        let len = event.encode(&mut bytes)?;
        Ok(Some(bytes[..len].to_vec()))
    }

    pub fn encode_mouse_button(
        &mut self,
        point: TerminalPoint,
        bounds: TerminalBounds,
        button: MouseButton,
        modifiers: Modifiers,
        pressed: bool,
    ) -> Result<Option<Vec<u8>>> {
        let Some(button) = ghostty_mouse_button(button) else {
            return Ok(None);
        };
        self.encode_mouse_event(
            point,
            bounds,
            if pressed {
                GhosttyMouseAction::Press
            } else {
                GhosttyMouseAction::Release
            },
            Some(button),
            modifiers,
            pressed,
        )
    }

    pub fn encode_mouse_motion(
        &mut self,
        point: TerminalPoint,
        bounds: TerminalBounds,
        button: Option<MouseButton>,
        modifiers: Modifiers,
    ) -> Result<Option<Vec<u8>>> {
        let button = match button {
            Some(button) => {
                let Some(button) = ghostty_mouse_button(button) else {
                    return Ok(None);
                };
                Some(button)
            }
            None => None,
        };
        self.encode_mouse_event(
            point,
            bounds,
            GhosttyMouseAction::Motion,
            button,
            modifiers,
            button.is_some(),
        )
    }

    pub fn encode_mouse_scroll(
        &mut self,
        point: TerminalPoint,
        bounds: TerminalBounds,
        scroll_lines: i32,
        event: &ScrollWheelEvent,
    ) -> Result<Vec<Vec<u8>>> {
        let Some(button) = ghostty_mouse_scroll_button(event) else {
            return Ok(Vec::new());
        };
        let Some(bytes) = self.encode_mouse_event(
            point,
            bounds,
            GhosttyMouseAction::Press,
            Some(button),
            event.modifiers,
            false,
        )?
        else {
            return Ok(Vec::new());
        };

        let count = scroll_lines.unsigned_abs() as usize;
        Ok((0..count).map(|_| bytes.clone()).collect())
    }

    fn encode_mouse_event(
        &mut self,
        point: TerminalPoint,
        bounds: TerminalBounds,
        action: GhosttyMouseAction,
        button: Option<GhosttyMouseButton>,
        modifiers: Modifiers,
        any_button_pressed: bool,
    ) -> Result<Option<Vec<u8>>> {
        let Some(position) = ghostty_mouse_position(point, bounds) else {
            return Ok(None);
        };

        let mut event = GhosttyMouseEvent::new()?;
        event
            .set_action(action)
            .set_button(button)
            .set_mods(ghostty_modifiers(modifiers))
            .set_position(position);

        self.mouse_encoder
            .set_options_from_terminal(self.terminal.as_ref())
            .set_size(ghostty_mouse_encoder_size(bounds))
            .set_any_button_pressed(any_button_pressed)
            .set_track_last_cell(false);

        let mut bytes = Vec::new();
        self.mouse_encoder.encode_to_vec(&event, &mut bytes)?;
        Ok((!bytes.is_empty()).then_some(bytes))
    }

    pub fn total_lines(&self) -> Result<usize> {
        Ok(self.terminal.total_rows()?)
    }

    pub fn viewport_lines(&self) -> Result<usize> {
        Ok(self.terminal.rows()? as usize)
    }

    pub(crate) fn drain_events(&self) -> Vec<TerminalBackendEvent> {
        self.events.borrow_mut().drain(..).collect()
    }

    pub fn cursor_blinking(&self) -> bool {
        self.cursor_blinking
    }

    pub fn content(&mut self, last_content: &TerminalContent) -> Result<TerminalContent> {
        let mode = self.mode();
        let scrollbar = self.terminal.scrollbar()?;
        let display_offset = scrollbar
            .total
            .saturating_sub(scrollbar.len)
            .saturating_sub(scrollbar.offset) as usize;

        let default_cursor_shape = self.default_cursor_shape;
        let snapshot = self.render_state.update(&self.terminal)?;
        self.cursor_blinking = snapshot.cursor_blinking()?;
        let cursor = if snapshot.cursor_visible()? {
            if let Some(cursor) = snapshot.cursor_viewport()? {
                TerminalCursor {
                    shape: Self::cursor_shape_from_visual_style(
                        snapshot.cursor_visual_style()?,
                        default_cursor_shape,
                    ),
                    point: TerminalPoint::new(
                        cursor.y as i32 - display_offset as i32,
                        cursor.x as usize,
                    ),
                }
            } else {
                TerminalCursor {
                    shape: TerminalCursorShape::Hidden,
                    point: TerminalPoint::new(0, 0),
                }
            }
        } else {
            TerminalCursor {
                shape: TerminalCursorShape::Hidden,
                point: TerminalPoint::new(0, 0),
            }
        };

        let mut cells = Vec::new();
        let mut raw_hyperlinks = Vec::new();
        let mut cursor_char = ' ';
        let mut row_index = 0i32;

        let row_iterator = &mut self.row_iterator;
        let cell_iterator = &mut self.cell_iterator;
        let mut rows = row_iterator.update(&snapshot)?;
        while let Some(row) = rows.next() {
            let mut cols = cell_iterator.update(row)?;
            let mut column_index = 0usize;
            while let Some(col) = cols.next() {
                let raw_cell = col.raw_cell()?;
                let wide = raw_cell.wide()?;
                let has_hyperlink = raw_cell.has_hyperlink()?;
                let cell = terminal_cell_from_ghostty_cell(
                    wide,
                    col.bg_color()?,
                    col.style()?,
                    col.graphemes()?,
                );
                let point = TerminalPoint::new(row_index - display_offset as i32, column_index);
                if point == cursor.point {
                    cursor_char = cell.c;
                }
                cells.push(IndexedCell { point, cell });
                raw_hyperlinks.push(has_hyperlink);
                column_index += 1;
            }
            row_index += 1;
        }

        if let Err(error) = self.apply_html_hyperlinks(&mut cells, &raw_hyperlinks) {
            log::error!("failed to map ghostty terminal hyperlinks: {error}");
        }

        Ok(TerminalContent {
            cells,
            mode,
            display_offset,
            selection_text: None,
            selection: None,
            cursor,
            cursor_char,
            terminal_bounds: last_content.terminal_bounds,
            last_hovered_word: last_content.last_hovered_word.clone(),
            scrolled_to_top: scrollbar.offset == 0,
            scrolled_to_bottom: display_offset == 0,
        })
    }

    fn cursor_shape_from_visual_style(
        cursor_visual_style: CursorVisualStyle,
        default_cursor_shape: TerminalCursorShape,
    ) -> TerminalCursorShape {
        match cursor_visual_style {
            CursorVisualStyle::Bar => TerminalCursorShape::Bar,
            CursorVisualStyle::Block => default_cursor_shape,
            CursorVisualStyle::Underline => TerminalCursorShape::Underline,
            CursorVisualStyle::BlockHollow => TerminalCursorShape::HollowBlock,
            _ => default_cursor_shape,
        }
    }

    pub fn formatted_content(&self) -> Result<String> {
        let mut formatter = Formatter::new(
            &self.terminal,
            FormatterOptions {
                format: Format::Plain,
                trim: true,
                unwrap: true,
            },
        )?;
        let bytes = formatter
            .format_alloc(None::<&libghostty_vt::alloc::Allocator<'_, ()>>)?
            .to_vec();
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    pub fn working_directory(&self, path_style: PathStyle) -> Result<Option<PathBuf>> {
        if let Some(working_directory_report) = self.working_directory_report.as_deref()
            && let Some(path) = parse_working_directory_report(working_directory_report, path_style)
        {
            return Ok(Some(path));
        }

        let pwd = self.terminal.pwd()?;
        let Some(path) = parse_working_directory_report(pwd, path_style) else {
            return Ok(None);
        };

        Ok(Some(path))
    }

    fn observe_output(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.observe_output_byte(byte);
        }
    }

    fn observe_output_byte(&mut self, byte: u8) {
        let state = std::mem::take(&mut self.osc_state);
        self.osc_state = match state {
            OscState::Ground => {
                if byte == b'\x1b' {
                    OscState::Escape
                } else {
                    OscState::Ground
                }
            }
            OscState::Escape => {
                if byte == b']' {
                    OscState::Command(Vec::new())
                } else if byte == b'\x1b' {
                    OscState::Escape
                } else {
                    OscState::Ground
                }
            }
            OscState::Command(mut command) => {
                if byte == b';' {
                    if command == b"7" {
                        OscState::Osc7Payload(Vec::new())
                    } else if command == b"52" {
                        OscState::Osc52Clipboard(Vec::new())
                    } else if is_osc_color_command(&command) {
                        OscState::OscColorPayload {
                            command,
                            payload: Vec::new(),
                        }
                    } else {
                        OscState::Unsupported
                    }
                } else if byte == b'\x07' {
                    self.handle_osc_command_terminator(command, "\x07");
                    OscState::Ground
                } else if byte == b'\x1b' {
                    OscState::CommandEscape(command)
                } else if push_osc_byte(&mut command, byte) {
                    OscState::Command(command)
                } else {
                    OscState::Unsupported
                }
            }
            OscState::CommandEscape(command) => {
                if byte == b'\\' {
                    self.handle_osc_command_terminator(command, "\x1b\\");
                    OscState::Ground
                } else if byte == b'\x1b' {
                    OscState::CommandEscape(command)
                } else {
                    OscState::Unsupported
                }
            }
            OscState::Unsupported => {
                if byte == b'\x07' {
                    OscState::Ground
                } else if byte == b'\x1b' {
                    OscState::UnsupportedEscape
                } else {
                    OscState::Unsupported
                }
            }
            OscState::UnsupportedEscape => {
                if byte == b'\\' {
                    OscState::Ground
                } else if byte == b'\x1b' {
                    OscState::UnsupportedEscape
                } else {
                    OscState::Unsupported
                }
            }
            OscState::Osc7Payload(mut payload) => {
                if byte == b'\x07' {
                    self.set_working_directory_report(payload);
                    OscState::Ground
                } else if byte == b'\x1b' {
                    OscState::Osc7PayloadEscape(payload)
                } else if push_osc_byte(&mut payload, byte) {
                    OscState::Osc7Payload(payload)
                } else {
                    OscState::Unsupported
                }
            }
            OscState::Osc7PayloadEscape(mut payload) => {
                if byte == b'\\' {
                    self.set_working_directory_report(payload);
                    OscState::Ground
                } else {
                    let has_room_for_escape =
                        push_limited_osc_byte(&mut payload, b'\x1b', MAX_OSC_PAYLOAD_LEN);
                    let has_room_for_byte = has_room_for_escape
                        && push_limited_osc_byte(&mut payload, byte, MAX_OSC_PAYLOAD_LEN);
                    if has_room_for_byte {
                        OscState::Osc7Payload(payload)
                    } else {
                        OscState::Unsupported
                    }
                }
            }
            OscState::OscColorPayload {
                command,
                mut payload,
            } => {
                if byte == b'\x07' {
                    self.handle_osc_color(command, payload, "\x07");
                    OscState::Ground
                } else if byte == b'\x1b' {
                    OscState::OscColorPayloadEscape { command, payload }
                } else if push_osc_byte(&mut payload, byte) {
                    OscState::OscColorPayload { command, payload }
                } else {
                    OscState::Unsupported
                }
            }
            OscState::OscColorPayloadEscape {
                command,
                mut payload,
            } => {
                if byte == b'\\' {
                    self.handle_osc_color(command, payload, "\x1b\\");
                    OscState::Ground
                } else {
                    let has_room_for_escape =
                        push_limited_osc_byte(&mut payload, b'\x1b', MAX_OSC_PAYLOAD_LEN);
                    let has_room_for_byte = has_room_for_escape
                        && push_limited_osc_byte(&mut payload, byte, MAX_OSC_PAYLOAD_LEN);
                    if has_room_for_byte {
                        OscState::OscColorPayload { command, payload }
                    } else {
                        OscState::Unsupported
                    }
                }
            }
            OscState::Osc52Clipboard(mut clipboard) => {
                if byte == b';' {
                    OscState::Osc52Payload {
                        clipboard,
                        payload: Vec::new(),
                    }
                } else if byte == b'\x07' {
                    OscState::Ground
                } else if byte == b'\x1b' {
                    OscState::UnsupportedEscape
                } else if push_limited_osc_byte(&mut clipboard, byte, MAX_OSC_PAYLOAD_LEN) {
                    OscState::Osc52Clipboard(clipboard)
                } else {
                    OscState::Unsupported
                }
            }
            OscState::Osc52Payload {
                clipboard,
                mut payload,
            } => {
                if byte == b'\x07' {
                    self.handle_osc52(clipboard, payload, "\x07");
                    OscState::Ground
                } else if byte == b'\x1b' {
                    OscState::Osc52PayloadEscape { clipboard, payload }
                } else if push_limited_osc_byte(&mut payload, byte, MAX_OSC52_PAYLOAD_LEN) {
                    OscState::Osc52Payload { clipboard, payload }
                } else {
                    OscState::Unsupported
                }
            }
            OscState::Osc52PayloadEscape {
                clipboard,
                mut payload,
            } => {
                if byte == b'\\' {
                    self.handle_osc52(clipboard, payload, "\x1b\\");
                    OscState::Ground
                } else {
                    let has_room_for_escape =
                        push_limited_osc_byte(&mut payload, b'\x1b', MAX_OSC52_PAYLOAD_LEN);
                    let has_room_for_byte = has_room_for_escape
                        && push_limited_osc_byte(&mut payload, byte, MAX_OSC52_PAYLOAD_LEN);
                    if has_room_for_byte {
                        OscState::Osc52Payload { clipboard, payload }
                    } else {
                        OscState::Unsupported
                    }
                }
            }
        };
    }

    fn set_working_directory_report(&mut self, payload: Vec<u8>) {
        if payload.is_empty() {
            self.working_directory_report = None;
        } else {
            self.working_directory_report = Some(String::from_utf8_lossy(&payload).into_owned());
        }
    }

    fn handle_osc_command_terminator(&mut self, command: Vec<u8>, terminator: &'static str) {
        if is_osc_color_command(&command) {
            self.handle_osc_color(command, Vec::new(), terminator);
        }
    }

    fn handle_osc_color(&mut self, command: Vec<u8>, payload: Vec<u8>, terminator: &'static str) {
        match command.as_slice() {
            b"4" => self.handle_osc4_color(payload, terminator),
            b"10" | b"11" | b"12" => self.handle_dynamic_color(command, payload, terminator),
            b"104" => self.handle_osc104_color_reset(payload),
            b"110" => self.reset_color(terminal_named_color_index(TerminalNamedColor::Foreground)),
            b"111" => self.reset_color(terminal_named_color_index(TerminalNamedColor::Background)),
            b"112" => self.reset_color(terminal_named_color_index(TerminalNamedColor::Cursor)),
            _ => {}
        }
    }

    fn handle_osc4_color(&mut self, payload: Vec<u8>, terminator: &'static str) {
        let params = split_osc_params(&payload);
        if params.is_empty() || !params.len().is_multiple_of(2) {
            return;
        }

        for chunk in params.chunks(2) {
            let Some(index) = parse_osc_number(chunk[0]) else {
                continue;
            };
            let index = usize::from(index);
            if chunk[1] == b"?" {
                let prefix = format!("4;{index}");
                self.report_color(prefix, index, terminator);
            } else if let Some(color) = parse_osc_color(chunk[1]) {
                self.set_color(index, color);
            }
        }
    }

    fn handle_dynamic_color(
        &mut self,
        command: Vec<u8>,
        payload: Vec<u8>,
        terminator: &'static str,
    ) {
        let Some(mut dynamic_code) = parse_osc_number(&command) else {
            return;
        };

        for param in split_osc_params(&payload) {
            let offset = usize::from(dynamic_code).saturating_sub(10);
            let index = terminal_named_color_index(TerminalNamedColor::Foreground) + offset;
            if index > terminal_named_color_index(TerminalNamedColor::Cursor) {
                break;
            }

            if param == b"?" {
                self.report_color(dynamic_code.to_string(), index, terminator);
            } else if let Some(color) = parse_osc_color(param) {
                self.set_color(index, color);
            }
            dynamic_code = dynamic_code.saturating_add(1);
        }
    }

    fn handle_osc104_color_reset(&mut self, payload: Vec<u8>) {
        if payload.is_empty() {
            self.reset_color_range(0..256);
            return;
        }

        let params = split_osc_params(&payload);
        if params.len() == 1 && params[0].is_empty() {
            self.reset_color_range(0..256);
            return;
        }

        for param in params {
            let Some(index) = parse_osc_number(param) else {
                continue;
            };
            self.reset_color(usize::from(index));
        }
    }

    fn set_color(&mut self, index: usize, color: TerminalRgb) {
        if let Some(slot) = self.colors.get_mut(index) {
            *slot = Some(color);
        }
    }

    fn reset_color(&mut self, index: usize) {
        if let Some(slot) = self.colors.get_mut(index) {
            *slot = None;
        }
    }

    fn reset_color_range(&mut self, range: std::ops::Range<usize>) {
        for index in range {
            self.reset_color(index);
        }
    }

    fn report_color(&mut self, prefix: String, index: usize, terminator: &'static str) {
        let formatter = move |color: TerminalRgb| {
            format!(
                "\x1b]{};rgb:{1:02x}{1:02x}/{2:02x}{2:02x}/{3:02x}{3:02x}{4}",
                prefix, color.r, color.g, color.b, terminator
            )
        };

        if let Some(Some(color)) = self.colors.get(index) {
            self.events
                .borrow_mut()
                .push_back(TerminalBackendEvent::PtyWrite(formatter(*color)));
        } else {
            self.events
                .borrow_mut()
                .push_back(TerminalBackendEvent::ColorRequest(
                    index,
                    Arc::new(formatter),
                ));
        }
    }

    fn handle_osc52(&mut self, clipboard: Vec<u8>, payload: Vec<u8>, terminator: &'static str) {
        let Some(clipboard) = osc52_clipboard_type(&clipboard) else {
            return;
        };

        if payload == b"?" {
            if !matches!(
                self.osc52,
                GhosttyOsc52::OnlyPaste | GhosttyOsc52::CopyPaste
            ) {
                return;
            }

            self.events
                .borrow_mut()
                .push_back(TerminalBackendEvent::ClipboardLoad(Arc::new(move |text| {
                    let base64 = Base64.encode(text);
                    format!("\x1b]52;{};{}{}", clipboard as char, base64, terminator)
                })));
            return;
        }

        if !matches!(self.osc52, GhosttyOsc52::OnlyCopy | GhosttyOsc52::CopyPaste) {
            return;
        }

        match Base64.decode(payload) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(text) => self
                    .events
                    .borrow_mut()
                    .push_back(TerminalBackendEvent::ClipboardStore(text)),
                Err(error) => log::debug!("invalid UTF-8 in OSC52 clipboard payload: {error}"),
            },
            Err(error) => log::debug!("invalid base64 in OSC52 clipboard payload: {error}"),
        }
    }

    fn mode(&self) -> TerminalModes {
        let mut mode = TerminalModes::empty();

        self.add_mode(&mut mode, Mode::DECCKM, TerminalModes::APP_CURSOR);
        self.add_mode(&mut mode, Mode::KEYPAD_KEYS, TerminalModes::APP_KEYPAD);
        self.add_mode(&mut mode, Mode::CURSOR_VISIBLE, TerminalModes::SHOW_CURSOR);
        self.add_mode(&mut mode, Mode::WRAPAROUND, TerminalModes::LINE_WRAP);
        self.add_mode(&mut mode, Mode::ORIGIN, TerminalModes::ORIGIN);
        self.add_mode(&mut mode, Mode::INSERT, TerminalModes::INSERT);
        self.add_mode(&mut mode, Mode::LINEFEED, TerminalModes::LINE_FEED_NEW_LINE);
        self.add_mode(&mut mode, Mode::FOCUS_EVENT, TerminalModes::FOCUS_IN_OUT);
        self.add_mode(&mut mode, Mode::ALT_SCROLL, TerminalModes::ALTERNATE_SCROLL);
        self.add_mode(
            &mut mode,
            Mode::BRACKETED_PASTE,
            TerminalModes::BRACKETED_PASTE,
        );
        self.add_mode(&mut mode, Mode::SGR_MOUSE, TerminalModes::SGR_MOUSE);
        self.add_mode(&mut mode, Mode::UTF8_MOUSE, TerminalModes::UTF8_MOUSE);

        if self.mode_enabled(Mode::ALT_SCREEN)
            || self.mode_enabled(Mode::ALT_SCREEN_LEGACY)
            || self.mode_enabled(Mode::ALT_SCREEN_SAVE)
        {
            mode.insert(TerminalModes::ALT_SCREEN);
        }

        if self.mode_enabled(Mode::X10_MOUSE) || self.mode_enabled(Mode::NORMAL_MOUSE) {
            mode.insert(TerminalModes::MOUSE_REPORT_CLICK);
        }
        if self.mode_enabled(Mode::BUTTON_MOUSE) {
            mode.insert(TerminalModes::MOUSE_DRAG);
        }
        if self.mode_enabled(Mode::ANY_MOUSE) {
            mode.insert(TerminalModes::MOUSE_MOTION);
        }

        mode
    }

    fn add_mode(&self, mode: &mut TerminalModes, ghostty_mode: Mode, terminal_mode: TerminalModes) {
        if self.mode_enabled(ghostty_mode) {
            mode.insert(terminal_mode);
        }
    }

    fn mode_enabled(&self, ghostty_mode: Mode) -> bool {
        self.terminal.mode(ghostty_mode).unwrap_or(false)
    }

    fn apply_html_hyperlinks(
        &self,
        cells: &mut [IndexedCell],
        raw_hyperlinks: &[bool],
    ) -> Result<()> {
        if !raw_hyperlinks.iter().any(|has_hyperlink| *has_hyperlink) {
            return Ok(());
        }

        let html_links = html_text_hyperlinks(&self.formatted_html_content()?)
            .into_iter()
            .filter_map(|(character, uri)| uri.map(|uri| (character, uri)))
            .collect::<Vec<_>>();
        let rendered_links = rendered_cell_text(cells)
            .into_iter()
            .filter(|(_, cell_index)| raw_hyperlinks.get(*cell_index).copied().unwrap_or(false))
            .collect::<Vec<_>>();

        let Some(offset) = html_link_offset(&html_links, &rendered_links) else {
            return Ok(());
        };

        for (rendered_index, (_, cell_index)) in rendered_links.iter().enumerate() {
            let Some((_, uri)) = html_links.get(offset + rendered_index) else {
                continue;
            };

            cells[*cell_index]
                .cell
                .set_hyperlink(Some(TerminalHyperlink::new(None::<&str>, uri.clone())));
        }

        Ok(())
    }

    fn formatted_html_content(&self) -> Result<String> {
        let mut formatter = Formatter::new(
            &self.terminal,
            FormatterOptions {
                format: Format::Html,
                trim: false,
                unwrap: false,
            },
        )?;
        let bytes = formatter
            .format_alloc(None::<&libghostty_vt::alloc::Allocator<'_, ()>>)?
            .to_vec();
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
}

fn rendered_cell_text(cells: &[IndexedCell]) -> Vec<(char, usize)> {
    cells
        .iter()
        .enumerate()
        .filter_map(|(index, cell)| {
            (!cell.is_wide_char_spacer_or_leading()).then_some((cell.c, index))
        })
        .collect()
}

fn html_link_offset(
    html_links: &[(char, String)],
    rendered_links: &[(char, usize)],
) -> Option<usize> {
    if rendered_links.is_empty() || html_links.len() < rendered_links.len() {
        return None;
    }

    html_links.windows(rendered_links.len()).position(|window| {
        window
            .iter()
            .map(|(character, _)| character)
            .eq(rendered_links.iter().map(|(character, _)| character))
    })
}

fn html_text_hyperlinks(html: &str) -> Vec<(char, Option<String>)> {
    let mut result = Vec::new();
    let mut current_href = None;
    let mut index = 0;

    while index < html.len() {
        let remaining = &html[index..];
        if remaining.starts_with('<') {
            if let Some(tag_end) = remaining.find('>') {
                let tag = &remaining[1..tag_end];
                if tag.eq_ignore_ascii_case("/a") {
                    current_href = None;
                } else if tag.starts_with("a ")
                    && let Some(href) = html_tag_href(tag)
                {
                    current_href = Some(href);
                }
                index += tag_end + 1;
                continue;
            }
        }

        if remaining.starts_with('&')
            && let Some(entity_end) = remaining.find(';')
            && let Some(character) = decode_html_entity(&remaining[1..entity_end])
        {
            result.push((character, current_href.clone()));
            index += entity_end + 1;
            continue;
        }

        let Some(character) = remaining.chars().next() else {
            break;
        };
        result.push((character, current_href.clone()));
        index += character.len_utf8();
    }

    result
}

fn html_tag_href(tag: &str) -> Option<String> {
    let href_start = tag.find("href=\"")? + "href=\"".len();
    let href_end = tag[href_start..].find('"')? + href_start;
    Some(decode_html_text(&tag[href_start..href_end]))
}

fn decode_html_text(text: &str) -> String {
    let mut decoded = String::new();
    let mut index = 0;
    while index < text.len() {
        let remaining = &text[index..];
        if remaining.starts_with('&')
            && let Some(entity_end) = remaining.find(';')
            && let Some(character) = decode_html_entity(&remaining[1..entity_end])
        {
            decoded.push(character);
            index += entity_end + 1;
            continue;
        }

        let Some(character) = remaining.chars().next() else {
            break;
        };
        decoded.push(character);
        index += character.len_utf8();
    }
    decoded
}

fn decode_html_entity(entity: &str) -> Option<char> {
    match entity {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => entity
            .strip_prefix("#x")
            .and_then(|hex| u32::from_str_radix(hex, 16).ok())
            .or_else(|| {
                entity
                    .strip_prefix('#')
                    .and_then(|decimal| decimal.parse::<u32>().ok())
            })
            .and_then(char::from_u32),
    }
}

fn push_osc_byte(buffer: &mut Vec<u8>, byte: u8) -> bool {
    push_limited_osc_byte(buffer, byte, MAX_OSC_PAYLOAD_LEN)
}

fn push_limited_osc_byte(buffer: &mut Vec<u8>, byte: u8, max_len: usize) -> bool {
    if buffer.len() >= max_len {
        false
    } else {
        buffer.push(byte);
        true
    }
}

fn is_osc_color_command(command: &[u8]) -> bool {
    matches!(
        command,
        b"4" | b"10" | b"11" | b"12" | b"104" | b"110" | b"111" | b"112"
    )
}

fn split_osc_params(payload: &[u8]) -> Vec<&[u8]> {
    payload.split(|byte| *byte == b';').collect()
}

fn parse_osc_number(input: &[u8]) -> Option<u8> {
    if input.is_empty() {
        return None;
    }

    let mut number = 0u8;
    for byte in input {
        let digit = char::from(*byte).to_digit(10)?;
        number = number
            .checked_mul(10)
            .and_then(|number| number.checked_add(digit as u8))?;
    }
    Some(number)
}

fn parse_osc_color(color: &[u8]) -> Option<TerminalRgb> {
    if let Some(color) = color.strip_prefix(b"#") {
        parse_legacy_osc_color(color)
    } else if let Some(color) = color.strip_prefix(b"rgb:") {
        parse_rgb_osc_color(color)
    } else {
        None
    }
}

fn parse_rgb_osc_color(color: &[u8]) -> Option<TerminalRgb> {
    let colors = std::str::from_utf8(color)
        .ok()?
        .split('/')
        .collect::<Vec<_>>();
    if colors.len() != 3 {
        return None;
    }

    Some(TerminalRgb {
        r: scale_osc_color_channel(colors[0])?,
        g: scale_osc_color_channel(colors[1])?,
        b: scale_osc_color_channel(colors[2])?,
    })
}

fn scale_osc_color_channel(input: &str) -> Option<u8> {
    if input.is_empty() || input.len() > 4 {
        return None;
    }

    let max = u32::pow(16, input.len() as u32) - 1;
    let value = u32::from_str_radix(input, 16).ok()?;
    Some((255 * value / max) as u8)
}

fn parse_legacy_osc_color(color: &[u8]) -> Option<TerminalRgb> {
    let item_len = color.len() / 3;
    let red = legacy_osc_color_channel(color.get(0..item_len)?)?;
    let green = legacy_osc_color_channel(color.get(item_len..item_len * 2)?)?;
    let blue = legacy_osc_color_channel(color.get(item_len * 2..)?)?;

    Some(TerminalRgb {
        r: red,
        g: green,
        b: blue,
    })
}

fn legacy_osc_color_channel(channel: &[u8]) -> Option<u8> {
    let color = usize::from_str_radix(std::str::from_utf8(channel).ok()?, 16).ok()? << 4;
    Some((color >> (4 * channel.len().saturating_sub(1))) as u8)
}

fn is_plain_text_keystroke(keystroke: &Keystroke) -> bool {
    !keystroke.modifiers.alt
        && !keystroke.modifiers.control
        && !keystroke.modifiers.platform
        && is_text_key(&keystroke.key)
}

fn is_zed_shift_enter(keystroke: &Keystroke) -> bool {
    keystroke.key == "enter"
        && keystroke.modifiers.shift
        && !keystroke.modifiers.alt
        && !keystroke.modifiers.control
        && !keystroke.modifiers.platform
}

fn is_text_key(key: &str) -> bool {
    key.chars().count() == 1 || key == "space"
}

fn ghostty_key_mods(keystroke: &Keystroke) -> GhosttyKeyMods {
    ghostty_modifiers(keystroke.modifiers)
}

fn ghostty_modifiers(modifiers: Modifiers) -> GhosttyKeyMods {
    let mut mods = GhosttyKeyMods::empty();
    if modifiers.shift {
        mods.insert(GhosttyKeyMods::SHIFT);
    }
    if modifiers.alt {
        mods.insert(GhosttyKeyMods::ALT);
    }
    if modifiers.control {
        mods.insert(GhosttyKeyMods::CTRL);
    }
    if modifiers.platform {
        mods.insert(GhosttyKeyMods::SUPER);
    }
    mods
}

fn ghostty_mouse_button(button: MouseButton) -> Option<GhosttyMouseButton> {
    match button {
        MouseButton::Left => Some(GhosttyMouseButton::Left),
        MouseButton::Right => Some(GhosttyMouseButton::Right),
        MouseButton::Middle => Some(GhosttyMouseButton::Middle),
        MouseButton::Navigate(_) => None,
    }
}

fn ghostty_mouse_scroll_button(event: &ScrollWheelEvent) -> Option<GhosttyMouseButton> {
    let delta = match event.delta {
        gpui::ScrollDelta::Pixels(pixels) => pixels.y,
        gpui::ScrollDelta::Lines(lines) => px(lines.y),
    };

    if delta > px(0.) {
        Some(GhosttyMouseButton::Four)
    } else if delta < px(0.) {
        Some(GhosttyMouseButton::Five)
    } else {
        None
    }
}

fn ghostty_mouse_position(
    point: TerminalPoint,
    bounds: TerminalBounds,
) -> Option<GhosttyMousePosition> {
    if point.line < 0 {
        return None;
    }

    Some(GhosttyMousePosition {
        x: point.column as f32 * f32::from(bounds.cell_width()).max(1.0),
        y: point.line as f32 * f32::from(bounds.line_height()).max(1.0),
    })
}

fn ghostty_mouse_encoder_size(bounds: TerminalBounds) -> GhosttyMouseEncoderSize {
    GhosttyMouseEncoderSize {
        screen_width: pixel_u32(bounds.width()),
        screen_height: pixel_u32(bounds.height()),
        cell_width: pixel_u32(bounds.cell_width()),
        cell_height: pixel_u32(bounds.line_height()),
        padding_top: 0,
        padding_bottom: 0,
        padding_right: 0,
        padding_left: 0,
    }
}

fn pixel_u32(value: gpui::Pixels) -> u32 {
    f32::from(value).ceil().max(1.0).min(u32::MAX as f32) as u32
}

fn ghostty_key(key: &str) -> Option<GhosttyKey> {
    let lower_key = key.to_ascii_lowercase();
    match lower_key.as_str() {
        "`" => Some(GhosttyKey::Backquote),
        "\\" => Some(GhosttyKey::Backslash),
        "|" => Some(GhosttyKey::Backslash),
        "[" | "{" => Some(GhosttyKey::BracketLeft),
        "]" | "}" => Some(GhosttyKey::BracketRight),
        "," | "<" => Some(GhosttyKey::Comma),
        "." | ">" => Some(GhosttyKey::Period),
        "/" | "?" => Some(GhosttyKey::Slash),
        ";" | ":" => Some(GhosttyKey::Semicolon),
        "'" | "\"" => Some(GhosttyKey::Quote),
        "-" | "_" => Some(GhosttyKey::Minus),
        "=" | "+" => Some(GhosttyKey::Equal),
        "0" | ")" => Some(GhosttyKey::Digit0),
        "1" | "!" => Some(GhosttyKey::Digit1),
        "2" | "@" => Some(GhosttyKey::Digit2),
        "3" | "#" => Some(GhosttyKey::Digit3),
        "4" | "$" => Some(GhosttyKey::Digit4),
        "5" | "%" => Some(GhosttyKey::Digit5),
        "6" | "^" => Some(GhosttyKey::Digit6),
        "7" | "&" => Some(GhosttyKey::Digit7),
        "8" | "*" => Some(GhosttyKey::Digit8),
        "9" | "(" => Some(GhosttyKey::Digit9),
        "a" => Some(GhosttyKey::A),
        "b" => Some(GhosttyKey::B),
        "c" => Some(GhosttyKey::C),
        "d" => Some(GhosttyKey::D),
        "e" => Some(GhosttyKey::E),
        "f" => Some(GhosttyKey::F),
        "g" => Some(GhosttyKey::G),
        "h" => Some(GhosttyKey::H),
        "i" => Some(GhosttyKey::I),
        "j" => Some(GhosttyKey::J),
        "k" => Some(GhosttyKey::K),
        "l" => Some(GhosttyKey::L),
        "m" => Some(GhosttyKey::M),
        "n" => Some(GhosttyKey::N),
        "o" => Some(GhosttyKey::O),
        "p" => Some(GhosttyKey::P),
        "q" => Some(GhosttyKey::Q),
        "r" => Some(GhosttyKey::R),
        "s" => Some(GhosttyKey::S),
        "t" => Some(GhosttyKey::T),
        "u" => Some(GhosttyKey::U),
        "v" => Some(GhosttyKey::V),
        "w" => Some(GhosttyKey::W),
        "x" => Some(GhosttyKey::X),
        "y" => Some(GhosttyKey::Y),
        "z" => Some(GhosttyKey::Z),
        "alt" => Some(GhosttyKey::AltLeft),
        "back" | "backspace" => Some(GhosttyKey::Backspace),
        "capslock" => Some(GhosttyKey::CapsLock),
        "ctrl" | "control" => Some(GhosttyKey::ControlLeft),
        "delete" => Some(GhosttyKey::Delete),
        "down" => Some(GhosttyKey::ArrowDown),
        "end" => Some(GhosttyKey::End),
        "enter" => Some(GhosttyKey::Enter),
        "escape" => Some(GhosttyKey::Escape),
        "f1" => Some(GhosttyKey::F1),
        "f2" => Some(GhosttyKey::F2),
        "f3" => Some(GhosttyKey::F3),
        "f4" => Some(GhosttyKey::F4),
        "f5" => Some(GhosttyKey::F5),
        "f6" => Some(GhosttyKey::F6),
        "f7" => Some(GhosttyKey::F7),
        "f8" => Some(GhosttyKey::F8),
        "f9" => Some(GhosttyKey::F9),
        "f10" => Some(GhosttyKey::F10),
        "f11" => Some(GhosttyKey::F11),
        "f12" => Some(GhosttyKey::F12),
        "f13" => Some(GhosttyKey::F13),
        "f14" => Some(GhosttyKey::F14),
        "f15" => Some(GhosttyKey::F15),
        "f16" => Some(GhosttyKey::F16),
        "f17" => Some(GhosttyKey::F17),
        "f18" => Some(GhosttyKey::F18),
        "f19" => Some(GhosttyKey::F19),
        "f20" => Some(GhosttyKey::F20),
        "f21" => Some(GhosttyKey::F21),
        "f22" => Some(GhosttyKey::F22),
        "f23" => Some(GhosttyKey::F23),
        "f24" => Some(GhosttyKey::F24),
        "home" => Some(GhosttyKey::Home),
        "insert" => Some(GhosttyKey::Insert),
        "left" => Some(GhosttyKey::ArrowLeft),
        "meta" | "cmd" | "command" => Some(GhosttyKey::MetaLeft),
        "pagedown" => Some(GhosttyKey::PageDown),
        "pageup" => Some(GhosttyKey::PageUp),
        "right" => Some(GhosttyKey::ArrowRight),
        "shift" => Some(GhosttyKey::ShiftLeft),
        "space" => Some(GhosttyKey::Space),
        "tab" => Some(GhosttyKey::Tab),
        "up" => Some(GhosttyKey::ArrowUp),
        _ => None,
    }
}

fn ghostty_unshifted_codepoint(key: &str) -> Option<char> {
    let character = if key == "space" {
        ' '
    } else {
        let mut characters = key.chars();
        let character = characters.next()?;
        if characters.next().is_some() {
            return None;
        }
        character
    };

    Some(match character {
        'A'..='Z' => character.to_ascii_lowercase(),
        ')' => '0',
        '!' => '1',
        '@' => '2',
        '#' => '3',
        '$' => '4',
        '%' => '5',
        '^' => '6',
        '&' => '7',
        '*' => '8',
        '(' => '9',
        '_' => '-',
        '+' => '=',
        '{' => '[',
        '}' => ']',
        '|' => '\\',
        ':' => ';',
        '"' => '\'',
        '<' => ',',
        '>' => '.',
        '?' => '/',
        _ => character,
    })
}

fn ghostty_key_text(keystroke: &Keystroke) -> Option<String> {
    if keystroke.key == "space" {
        Some(" ".to_string())
    } else if keystroke.key.chars().count() == 1 {
        Some(keystroke.key.clone())
    } else {
        None
    }
}

fn osc52_clipboard_type(selector: &[u8]) -> Option<u8> {
    let clipboard = selector.first().copied().unwrap_or(b'c');
    match clipboard {
        b'c' | b'p' | b's' => Some(clipboard),
        _ => return None,
    }
}

fn terminal_named_color_index(color: TerminalNamedColor) -> usize {
    match color {
        TerminalNamedColor::Black => 0,
        TerminalNamedColor::Red => 1,
        TerminalNamedColor::Green => 2,
        TerminalNamedColor::Yellow => 3,
        TerminalNamedColor::Blue => 4,
        TerminalNamedColor::Magenta => 5,
        TerminalNamedColor::Cyan => 6,
        TerminalNamedColor::White => 7,
        TerminalNamedColor::BrightBlack => 8,
        TerminalNamedColor::BrightRed => 9,
        TerminalNamedColor::BrightGreen => 10,
        TerminalNamedColor::BrightYellow => 11,
        TerminalNamedColor::BrightBlue => 12,
        TerminalNamedColor::BrightMagenta => 13,
        TerminalNamedColor::BrightCyan => 14,
        TerminalNamedColor::BrightWhite => 15,
        TerminalNamedColor::Foreground => TERMINAL_FOREGROUND_COLOR_INDEX,
        TerminalNamedColor::Background => TERMINAL_BACKGROUND_COLOR_INDEX,
        TerminalNamedColor::Cursor => TERMINAL_CURSOR_COLOR_INDEX,
        TerminalNamedColor::DimBlack => 259,
        TerminalNamedColor::DimRed => 260,
        TerminalNamedColor::DimGreen => 261,
        TerminalNamedColor::DimYellow => 262,
        TerminalNamedColor::DimBlue => 263,
        TerminalNamedColor::DimMagenta => 264,
        TerminalNamedColor::DimCyan => 265,
        TerminalNamedColor::DimWhite => 266,
        TerminalNamedColor::BrightForeground => 267,
        TerminalNamedColor::DimForeground => 268,
    }
}

fn parse_working_directory_report(report: &str, path_style: PathStyle) -> Option<PathBuf> {
    if report.is_empty() {
        return None;
    }

    if let Ok(url) = Url::parse(report) {
        if url.scheme() == "file" {
            return url.to_file_path_ext(path_style).ok();
        } else if report.contains("://") {
            return None;
        }
    }

    Some(PathBuf::from(report))
}

fn terminal_cell_from_ghostty_cell(
    wide: CellWide,
    background_color: Option<RgbColor>,
    style: Style,
    graphemes: Vec<char>,
) -> TerminalCell {
    let mut cell = TerminalCell::new(
        graphemes.first().copied().unwrap_or(' '),
        ghostty_style_color_to_terminal(style.fg_color)
            .unwrap_or(TerminalColor::Named(TerminalNamedColor::Foreground)),
        ghostty_style_color_to_terminal(style.bg_color)
            .or_else(|| background_color.map(rgb_color_to_terminal_color))
            .unwrap_or(TerminalColor::Named(TerminalNamedColor::Background)),
        ghostty_style_flags(style) | ghostty_wide_flags(wide),
    );

    for character in graphemes.into_iter().skip(1) {
        cell.push_zerowidth(character);
    }

    if let Some(underline_color) = ghostty_style_color_to_terminal(style.underline_color) {
        cell.set_underline_color(Some(underline_color));
    }

    cell
}

fn ghostty_style_flags(style: Style) -> TerminalCellFlags {
    let mut flags = TerminalCellFlags::empty();

    if style.bold {
        flags.insert(TerminalCellFlags::BOLD);
    }
    if style.italic {
        flags.insert(TerminalCellFlags::ITALIC);
    }
    if style.faint {
        flags.insert(TerminalCellFlags::DIM);
    }
    if style.inverse {
        flags.insert(TerminalCellFlags::INVERSE);
    }
    if style.invisible {
        flags.insert(TerminalCellFlags::HIDDEN);
    }
    if style.strikethrough {
        flags.insert(TerminalCellFlags::STRIKEOUT);
    }

    match style.underline {
        Underline::None => {}
        Underline::Single => flags.insert(TerminalCellFlags::UNDERLINE),
        Underline::Double => flags.insert(TerminalCellFlags::DOUBLE_UNDERLINE),
        Underline::Curly => flags.insert(TerminalCellFlags::UNDERCURL),
        Underline::Dotted => flags.insert(TerminalCellFlags::DOTTED_UNDERLINE),
        Underline::Dashed => flags.insert(TerminalCellFlags::DASHED_UNDERLINE),
        _ => flags.insert(TerminalCellFlags::UNDERLINE),
    }

    flags
}

fn ghostty_wide_flags(wide: CellWide) -> TerminalCellFlags {
    match wide {
        CellWide::Narrow => TerminalCellFlags::empty(),
        CellWide::Wide => TerminalCellFlags::WIDE_CHAR,
        CellWide::SpacerTail => TerminalCellFlags::WIDE_CHAR_SPACER,
        CellWide::SpacerHead => TerminalCellFlags::LEADING_WIDE_CHAR_SPACER,
    }
}

fn ghostty_style_color_to_terminal(color: StyleColor) -> Option<TerminalColor> {
    match color {
        StyleColor::None => None,
        StyleColor::Palette(index) => Some(TerminalColor::Indexed(index.0)),
        StyleColor::Rgb(color) => Some(rgb_color_to_terminal_color(color)),
    }
}

fn rgb_color_to_terminal_color(color: RgbColor) -> TerminalColor {
    TerminalColor::Spec(TerminalRgb {
        r: color.r,
        g: color.g,
        b: color.b,
    })
}

#[cfg(test)]
mod tests {
    use anyhow::{Context as _, Result};
    use gpui::{
        Bounds, Modifiers, MouseButton, Point, ScrollDelta, ScrollWheelEvent, Size, TouchPhase,
        point, px,
    };

    use super::*;

    fn test_bounds() -> TerminalBounds {
        TerminalBounds::new(
            px(10.0),
            px(10.0),
            Bounds {
                origin: Point::default(),
                size: Size {
                    width: px(80.0),
                    height: px(30.0),
                },
            },
        )
    }

    fn row_text(content: &TerminalContent, row: i32) -> String {
        content
            .cells
            .iter()
            .filter(|cell| cell.point.line == row)
            .map(|cell| cell.c)
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    fn pty_writes(events: Vec<TerminalBackendEvent>) -> Vec<String> {
        events
            .into_iter()
            .filter_map(|event| match event {
                TerminalBackendEvent::PtyWrite(write) => Some(write),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn renders_text_and_sgr_styles() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;
        backend.write_output(b"hello\r\n\x1b[1;4;31mred\x1b[0m");

        let content = backend.content(&TerminalContent {
            terminal_bounds: bounds,
            ..Default::default()
        })?;

        assert_eq!(row_text(&content, 0), "hello");
        assert_eq!(row_text(&content, 1), "red");

        let red_cell = content
            .cells
            .iter()
            .find(|cell| cell.c == 'r')
            .context("missing red cell")?;
        assert!(red_cell.is_bold());
        assert!(red_cell.has_underline());
        assert_eq!(red_cell.fg, TerminalColor::Indexed(1));

        Ok(())
    }

    #[test]
    fn reports_vt_effects_as_terminal_events() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;
        backend.write_output(b"\x07\x1b]2;Ghostty title\x1b\\");

        let events = backend.drain_events();
        assert!(
            events
                .iter()
                .any(|event| matches!(event, TerminalBackendEvent::Bell))
        );
        let title = events.iter().find_map(|event| match event {
            TerminalBackendEvent::Title(title) => Some(title.as_str()),
            _ => None,
        });
        assert_eq!(title, Some("Ghostty title"));

        Ok(())
    }

    #[test]
    fn responds_to_device_attribute_queries() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        backend.write_output(b"\x1b[c\x1b[>c");

        let writes = pty_writes(backend.drain_events());
        assert!(
            writes.iter().any(|write| write == "\x1b[?6c"),
            "missing primary device attributes response in {writes:?}",
        );
        assert!(
            writes
                .iter()
                .any(|write| write.starts_with("\x1b[>0;") && write.ends_with(";1c")),
            "missing secondary device attributes response in {writes:?}",
        );

        Ok(())
    }

    #[test]
    fn responds_to_size_queries_from_current_bounds() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        backend.write_output(b"\x1b[18t");

        let writes = pty_writes(backend.drain_events());
        assert!(
            writes.iter().any(|write| write == "\x1b[8;3;8t"),
            "missing character-size response in {writes:?}",
        );

        let resized_bounds = TerminalBounds::new(
            px(12.0),
            px(8.0),
            Bounds {
                origin: Point::default(),
                size: Size {
                    width: px(160.0),
                    height: px(48.0),
                },
            },
        );
        backend.resize(resized_bounds)?;
        backend.write_output(b"\x1b[18t");

        let writes = pty_writes(backend.drain_events());
        assert!(
            writes.iter().any(|write| write == "\x1b[8;4;20t"),
            "missing resized character-size response in {writes:?}",
        );

        Ok(())
    }

    #[test]
    fn responds_to_color_scheme_queries() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        backend.set_dark_color_scheme(true);
        backend.write_output(b"\x1b[?996n");
        let dark_writes = pty_writes(backend.drain_events());

        backend.set_dark_color_scheme(false);
        backend.write_output(b"\x1b[?996n");
        let light_writes = pty_writes(backend.drain_events());

        assert_eq!(dark_writes, vec!["\x1b[?997;1n"]);
        assert_eq!(light_writes, vec!["\x1b[?997;2n"]);

        Ok(())
    }

    #[test]
    fn encodes_key_input_from_ghostty_terminal_state() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        assert_eq!(
            backend
                .encode_key(&Keystroke::parse("up").context("parse up")?, false)?
                .as_deref(),
            Some(b"\x1b[A".as_slice())
        );

        backend.set_mode(Mode::DECCKM, true)?;
        assert_eq!(
            backend
                .encode_key(
                    &Keystroke::parse("up").context("parse app cursor up")?,
                    false
                )?
                .as_deref(),
            Some(b"\x1bOA".as_slice())
        );

        assert_eq!(
            backend
                .encode_key(&Keystroke::parse("ctrl-c").context("parse ctrl-c")?, false)?
                .as_deref(),
            Some(b"\x03".as_slice())
        );
        assert_eq!(
            backend
                .encode_key(&Keystroke::parse("ctrl-d").context("parse ctrl-d")?, false)?
                .as_deref(),
            Some(b"\x04".as_slice())
        );

        assert_eq!(
            backend
                .encode_key(
                    &Keystroke::parse("shift-enter").context("parse shift-enter")?,
                    false
                )?
                .as_deref(),
            Some(b"\x0a".as_slice())
        );

        assert!(
            backend
                .encode_key(&Keystroke::parse("a").context("parse plain text")?, false)?
                .is_none()
        );

        Ok(())
    }

    #[test]
    fn encodes_focus_input_when_enabled() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        assert!(backend.encode_focus(true)?.is_none());

        backend.set_mode(Mode::FOCUS_EVENT, true)?;
        assert_eq!(
            backend.encode_focus(true)?.as_deref(),
            Some(b"\x1b[I".as_slice())
        );
        assert_eq!(
            backend.encode_focus(false)?.as_deref(),
            Some(b"\x1b[O".as_slice())
        );

        Ok(())
    }

    #[test]
    fn encodes_mouse_input_from_ghostty_terminal_state() -> Result<()> {
        let bounds = test_bounds();
        let mouse_point = TerminalPoint::new(2, 1);
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        assert!(
            backend
                .encode_mouse_button(
                    mouse_point,
                    bounds,
                    MouseButton::Left,
                    Modifiers::default(),
                    true
                )?
                .is_none()
        );

        backend.write_output(b"\x1b[?1000h\x1b[?1006h");
        assert_eq!(
            backend
                .encode_mouse_button(
                    mouse_point,
                    bounds,
                    MouseButton::Left,
                    Modifiers::default(),
                    true
                )?
                .as_deref(),
            Some(b"\x1b[<0;2;3M".as_slice())
        );
        assert_eq!(
            backend
                .encode_mouse_button(
                    mouse_point,
                    bounds,
                    MouseButton::Right,
                    Modifiers::default(),
                    true
                )?
                .as_deref(),
            Some(b"\x1b[<2;2;3M".as_slice())
        );
        assert_eq!(
            backend
                .encode_mouse_button(
                    mouse_point,
                    bounds,
                    MouseButton::Left,
                    Modifiers::default(),
                    false
                )?
                .as_deref(),
            Some(b"\x1b[<0;2;3m".as_slice())
        );

        backend.write_output(b"\x1b[?1000l\x1b[?1002h");
        assert_eq!(
            backend
                .encode_mouse_motion(
                    mouse_point,
                    bounds,
                    Some(MouseButton::Left),
                    Modifiers::default()
                )?
                .as_deref(),
            Some(b"\x1b[<32;2;3M".as_slice())
        );

        Ok(())
    }

    #[test]
    fn encodes_scroll_input_from_ghostty_terminal_state() -> Result<()> {
        let bounds = test_bounds();
        let mouse_point = TerminalPoint::new(2, 1);
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;
        let scroll_event = ScrollWheelEvent {
            delta: ScrollDelta::Lines(point(0., 1.)),
            touch_phase: TouchPhase::Moved,
            ..Default::default()
        };

        backend.write_output(b"\x1b[?1000h\x1b[?1006h");

        assert_eq!(
            backend.encode_mouse_scroll(mouse_point, bounds, 2, &scroll_event)?,
            vec![b"\x1b[<64;2;3M".to_vec(), b"\x1b[<64;2;3M".to_vec()]
        );

        Ok(())
    }

    #[test]
    fn reports_theme_relative_osc_color_queries() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        backend.write_output(b"\x1b]11;?\x1b\\");

        let events = backend.drain_events();
        let response = events.iter().find_map(|event| match event {
            TerminalBackendEvent::ColorRequest(index, format)
                if *index == TERMINAL_BACKGROUND_COLOR_INDEX =>
            {
                Some(format(TerminalRgb { r: 1, g: 2, b: 3 }))
            }
            _ => None,
        });
        assert_eq!(
            response,
            Some("\x1b]11;rgb:0101/0202/0303\x1b\\".to_string())
        );

        Ok(())
    }

    #[test]
    fn reports_explicit_osc_color_queries_in_order() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        backend.write_output(b"\x1b]11;#010203\x1b\\\x1b]11;?\x1b\\\x1b[c");

        let writes = pty_writes(backend.drain_events());
        assert_eq!(
            writes.first().map(String::as_str),
            Some("\x1b]11;rgb:0101/0202/0303\x1b\\")
        );
        assert!(
            writes.iter().any(|write| write == "\x1b[?6c"),
            "missing primary device attributes response in {writes:?}",
        );

        Ok(())
    }

    #[test]
    fn resets_explicit_osc_color_queries() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        backend.write_output(b"\x1b]11;#010203\x1b\\\x1b]111\x1b\\\x1b]11;?\x1b\\");

        let events = backend.drain_events();
        assert!(
            events
                .iter()
                .all(|event| !matches!(event, TerminalBackendEvent::PtyWrite(write) if write.starts_with("\x1b]11;rgb:")))
        );
        assert!(
            events.iter().any(|event| {
                matches!(event, TerminalBackendEvent::ColorRequest(index, _) if *index == TERMINAL_BACKGROUND_COLOR_INDEX)
            }),
            "missing fallback color request in {events:?}",
        );

        Ok(())
    }

    #[test]
    fn reports_split_osc52_clipboard_store() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        for chunk in b"\x1b]52;c;aGVsbG8=\x1b\\".chunks(2) {
            backend.write_output(chunk);
        }

        let events = backend.drain_events();
        let stored = events.iter().find_map(|event| match event {
            TerminalBackendEvent::ClipboardStore(text) => Some(text.as_str()),
            _ => None,
        });
        assert_eq!(stored, Some("hello"));

        Ok(())
    }

    #[test]
    fn treats_empty_osc52_clipboard_selector_as_clipboard() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        backend.write_output(b"\x1b]52;;aGVsbG8=\x1b\\");

        let events = backend.drain_events();
        let stored = events.iter().find_map(|event| match event {
            TerminalBackendEvent::ClipboardStore(text) => Some(text.as_str()),
            _ => None,
        });
        assert_eq!(stored, Some("hello"));

        Ok(())
    }

    #[test]
    fn ignores_osc52_clipboard_load_by_default() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;

        backend.write_output(b"\x1b]52;c;?\x1b\\");

        let events = backend.drain_events();
        assert!(
            events
                .iter()
                .all(|event| !matches!(event, TerminalBackendEvent::ClipboardLoad(..)))
        );

        Ok(())
    }

    #[test]
    fn reports_osc52_clipboard_load_when_enabled() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;
        backend.set_osc52(GhosttyOsc52::CopyPaste);

        backend.write_output(b"\x1b]52;c;?\x07");

        let events = backend.drain_events();
        let response = events.iter().find_map(|event| match event {
            TerminalBackendEvent::ClipboardLoad(format) => Some(format("hello")),
            _ => None,
        });
        assert_eq!(response, Some("\x1b]52;c;aGVsbG8=\x07".to_string()));

        Ok(())
    }

    #[test]
    fn tracks_split_osc7_working_directory_reports() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;
        let working_directory = PathBuf::from("/tmp/ghostty osc7 cwd");
        let url = Url::from_directory_path(&working_directory)
            .map_err(|()| anyhow::anyhow!("failed to build OSC7 directory URL"))?;
        let sequence = format!("\x1b]7;{url}\x1b\\");

        for chunk in sequence.as_bytes().chunks(3) {
            backend.write_output(chunk);
        }

        assert_eq!(
            backend.working_directory(PathStyle::Posix)?,
            Some(working_directory)
        );

        Ok(())
    }

    #[test]
    fn maps_osc8_hyperlinks_into_terminal_content() -> Result<()> {
        let bounds = test_bounds();
        let mut backend = GhosttyBackend::new(bounds, Some(100))?;
        backend.write_output(
            b"\x1b]8;;https://example.com/path?a=1&b=2\x1b\\link\x1b]8;;\x1b\\ plain",
        );

        let content = backend.content(&TerminalContent {
            terminal_bounds: bounds,
            ..Default::default()
        })?;

        let linked_cell = content
            .cells
            .iter()
            .find(|cell| cell.c == 'l')
            .context("missing linked cell")?;
        let hyperlink = linked_cell
            .hyperlink()
            .context("missing OSC8 hyperlink metadata")?;
        assert_eq!(hyperlink.uri(), "https://example.com/path?a=1&b=2");

        let plain_cell = content
            .cells
            .iter()
            .find(|cell| cell.c == 'p')
            .context("missing plain cell")?;
        assert!(plain_cell.hyperlink().is_none());

        Ok(())
    }
}
