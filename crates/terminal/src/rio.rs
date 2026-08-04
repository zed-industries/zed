#[cfg(target_os = "windows")]
use std::num::NonZeroU32;
use std::{borrow::Cow, ops::RangeInclusive, path::PathBuf, sync::Arc};

mod hyperlinks;

use anyhow::{Result, anyhow};
use futures::channel::mpsc::UnboundedSender;
use rio_vt::{
    ansi::{ClearMode, CursorShape as RioCursorShape},
    ansi::mode::{NamedPrivateMode, PrivateMode},
    config::colors::{AnsiColor, ColorArray, NamedColor as RioNamedColor},
    crosswords::{
        Crosswords, CrosswordsSize, Mode as RioMode,
        grid::{Dimensions as _, Grid, GridIterator, Scroll as RioScroll, row::Row},
        pos::{Boundary, Column, Direction as RioDirection, Line, Pos, Side},
        search::{Match, RegexIter, RegexSearch},
        square::{Hyperlink as RioHyperlink, Square},
        style::StyleFlags,
        vi_mode::{ViModeCursor, ViMotion as RioViMotion},
    },
    corcovado,
    event::{EventListener, Msg, RioEvent, WindowId, WindowSize, sync::FairMutex},
    performer::{Machine, handler::Handler as _, handler::Processor},
    teletypewriter,
};
use util::paths::PathStyle;
use vte::ansi::{Color, NamedColor, Rgb};

use crate::{
    Cell, Content, Cursor, CursorShape, Hyperlink, IndexedCell, Modes, Point, PtyEvent, Range,
    RenderableCells, Scroll, Search, Selection, SelectionRange, SelectionSide, SelectionType,
    TerminalBackendEvent, TerminalBounds, ViMotion,
    pty_info::ProcessIdGetter,
    terminal_settings::{AlternateScroll, CursorShape as SettingsCursorShape},
};

pub(super) use hyperlinks::{HyperlinkMatch, RegexSearches};

pub(super) type RioProcessor = Processor;
pub(super) type RioPty = teletypewriter::Pty;
pub(super) type RioTerm = Crosswords<ZedListener>;
pub(super) type RioTermLock = FairMutex<RioTerm>;
pub(super) type RioGrid = Grid<Square>;
pub(super) type RioGridIterator<'a> = GridIterator<'a, Square>;

/// OSC 52 clipboard access policy, mirroring the alacritty options Zed used:
/// regular terminals allow copy only, display-only terminals allow nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Osc52 {
    Disabled,
    OnlyCopy,
}

#[derive(Clone)]
pub(super) struct RioTermConfig {
    scrolling_history: usize,
    default_cursor_shape: SettingsCursorShape,
    osc52: Osc52,
}

#[derive(Clone)]
pub(super) struct ZedListener {
    events_tx: UnboundedSender<PtyEvent>,
    osc52: Osc52,
}

#[derive(Clone, Debug)]
pub(super) struct RioSearch {
    search: RegexSearch,
}

/// The per-cell data Zed renders from, resolved out of rio's packed
/// `Square` + interned style/extras tables into an owned value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RioCell {
    c: char,
    fg: Color,
    bg: Color,
    flags: StyleFlags,
    is_wide_char_spacer: bool,
    zerowidth: Option<Arc<[char]>>,
    hyperlink: Option<Hyperlink>,
}

impl Default for RioCell {
    fn default() -> Self {
        Self {
            c: ' ',
            fg: Color::Named(NamedColor::Foreground),
            bg: Color::Named(NamedColor::Background),
            flags: StyleFlags::empty(),
            is_wide_char_spacer: false,
            zerowidth: None,
            hyperlink: None,
        }
    }
}

#[cfg(unix)]
impl From<&RioPty> for ProcessIdGetter {
    fn from(pty: &RioPty) -> Self {
        Self::new(*pty.child.id, *pty.child.pid as u32)
    }
}

#[cfg(windows)]
impl From<&RioPty> for ProcessIdGetter {
    fn from(pty: &RioPty) -> Self {
        use windows::Win32::{Foundation::HANDLE, System::Threading::GetProcessId};

        let child = pty.child_watcher();
        let handle = child.raw_handle();
        let fallback_pid = child.pid().unwrap_or_else(|| unsafe {
            NonZeroU32::new_unchecked(GetProcessId(HANDLE(handle as _)))
        });

        Self::new(handle as i32, u32::from(fallback_pid))
    }
}

pub(super) struct PtySender {
    sender: corcovado::channel::Sender<Msg>,
}

impl PtySender {
    pub(super) fn notify(&self, input: impl Into<Cow<'static, [u8]>>) {
        let input = input.into();
        if input.is_empty() {
            return;
        }
        if let Err(error) = self.sender.send(Msg::Input(input)) {
            log::debug!("failed to send input to rio pty loop: {error}");
        }
    }

    pub(super) fn resize(&self, bounds: TerminalBounds) {
        if let Err(error) = self
            .sender
            .send(Msg::Resize(window_size_from_terminal_bounds(bounds)))
        {
            log::error!("failed to resize rio pty: {error}");
        }
    }

    pub(super) fn shutdown(&self) {
        if let Err(error) = self.sender.send(Msg::Shutdown) {
            log::debug!("failed to shut down rio pty loop: {error}");
        }
    }
}

fn window_size_from_terminal_bounds(bounds: TerminalBounds) -> WindowSize {
    WindowSize {
        rows: bounds.num_lines() as u16,
        cols: bounds.num_columns() as u16,
        width: f32::from(bounds.width()) as u16,
        height: f32::from(bounds.height()) as u16,
    }
}

fn crosswords_size_from_terminal_bounds(bounds: TerminalBounds) -> CrosswordsSize {
    CrosswordsSize::new_with_dimensions(
        bounds.num_columns(),
        bounds.num_lines(),
        f32::from(bounds.width()) as u32,
        f32::from(bounds.height()) as u32,
        f32::from(bounds.cell_width()) as u32,
        f32::from(bounds.line_height()) as u32,
    )
}

pub(super) fn display_only_term_config(
    scrolling_history: usize,
    cursor_shape: SettingsCursorShape,
) -> RioTermConfig {
    RioTermConfig {
        scrolling_history,
        default_cursor_shape: cursor_shape,
        osc52: Osc52::Disabled,
    }
}

pub(super) fn pty_term_config(
    scrolling_history: usize,
    cursor_shape: SettingsCursorShape,
) -> RioTermConfig {
    RioTermConfig {
        scrolling_history,
        default_cursor_shape: cursor_shape,
        osc52: Osc52::OnlyCopy,
    }
}

pub(super) fn set_default_cursor_style(
    config: &mut RioTermConfig,
    cursor_shape: SettingsCursorShape,
) {
    config.default_cursor_shape = cursor_shape;
}

pub(super) fn apply_config(term: &RioTermLock, config: &RioTermConfig) {
    let mut term = term.lock();
    let new_default = rio_cursor_shape(config.default_cursor_shape);
    // Only override the live shape when the application hasn't changed it
    // away from the default via escape sequences.
    if term.cursor_shape == term.default_cursor_shape {
        term.cursor_shape = new_default;
    }
    term.default_cursor_shape = new_default;
    term.grid.update_history(config.scrolling_history);
}

pub(super) struct RioPtyOptions {
    shell: Option<(String, Vec<String>)>,
    working_directory: Option<PathBuf>,
    env: Vec<(String, String)>,
    #[cfg(windows)]
    escape_args: bool,
}

pub(super) fn pty_options(
    shell: Option<(String, Vec<String>)>,
    working_directory: Option<PathBuf>,
    env: impl IntoIterator<Item = (String, String)>,
    #[cfg(windows)] escape_args: bool,
) -> RioPtyOptions {
    RioPtyOptions {
        shell,
        working_directory,
        env: env.into_iter().collect(),
        #[cfg(windows)]
        escape_args,
    }
}

#[cfg(unix)]
pub(super) fn open_pty(
    options: &RioPtyOptions,
    bounds: TerminalBounds,
    window_id: u64,
) -> std::io::Result<RioPty> {
    // `None` leaves the shell up to teletypewriter, which resolves the user's
    // default and starts it as a login shell (as alacritty_terminal did).
    let (shell, args) = match options.shell.clone() {
        Some((program, args)) => (Some(program), args),
        None => (None, Vec::new()),
    };
    let mut env = options.env.clone();
    env.push(("WINDOWID".to_string(), window_id.to_string()));

    teletypewriter::create_pty_with_spawn(
        shell.as_deref(),
        args,
        &options
            .working_directory
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned()),
        Some(env),
        bounds.num_columns() as u16,
        bounds.num_lines() as u16,
        f32::from(bounds.width()) as u16,
        f32::from(bounds.height()) as u16,
    )
}

#[cfg(windows)]
pub(super) fn open_pty(
    options: &RioPtyOptions,
    bounds: TerminalBounds,
    window_id: u64,
) -> std::io::Result<RioPty> {
    let (shell, args) = match options.shell.clone() {
        Some((program, args)) => (Some(program), args),
        None => (None, Vec::new()),
    };
    let args = if options.escape_args {
        args.iter().map(|arg| escape_windows_arg(arg)).collect()
    } else {
        args
    };
    let mut env = options.env.clone();
    env.push(("WINDOWID".to_string(), window_id.to_string()));

    teletypewriter::create_pty(
        shell.as_deref(),
        args,
        &options
            .working_directory
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned()),
        Some(env),
        bounds.num_columns() as u16,
        bounds.num_lines() as u16,
    )
}

/// Quotes an argument for inclusion in a Windows command line, following the
/// `CommandLineToArgvW` rules.
#[cfg(windows)]
fn escape_windows_arg(arg: &str) -> String {
    if !arg.is_empty() && !arg.contains([' ', '\t', '"']) {
        return arg.to_string();
    }

    let mut escaped = String::with_capacity(arg.len() + 2);
    escaped.push('"');
    let mut backslashes = 0;
    for character in arg.chars() {
        match character {
            '\\' => backslashes += 1,
            '"' => {
                escaped.extend(std::iter::repeat_n('\\', backslashes * 2 + 1));
                escaped.push('"');
                backslashes = 0;
                continue;
            }
            _ => {
                escaped.extend(std::iter::repeat_n('\\', backslashes));
                backslashes = 0;
            }
        }
        if character != '\\' {
            escaped.push(character);
        }
    }
    escaped.extend(std::iter::repeat_n('\\', backslashes * 2));
    escaped.push('"');
    escaped
}

pub(super) fn new_term(
    config: &RioTermConfig,
    bounds: TerminalBounds,
    events_tx: UnboundedSender<PtyEvent>,
    alternate_scroll: AlternateScroll,
) -> Arc<RioTermLock> {
    let listener = ZedListener {
        events_tx,
        osc52: config.osc52,
    };
    let mut term = Crosswords::new(
        crosswords_size_from_terminal_bounds(bounds),
        rio_cursor_shape(config.default_cursor_shape),
        listener,
        WindowId::from(0),
        0,
        config.scrolling_history,
    );

    if let AlternateScroll::Off = alternate_scroll {
        term.unset_private_mode(PrivateMode::Named(NamedPrivateMode::AlternateScroll));
    }

    Arc::new(FairMutex::new(term))
}

pub(super) fn spawn_event_loop(
    term: Arc<RioTermLock>,
    events_tx: UnboundedSender<PtyEvent>,
    pty: RioPty,
) -> Result<PtySender> {
    let listener = ZedListener {
        events_tx,
        osc52: Osc52::Disabled,
    };
    let machine = Machine::new(term, pty, listener, WindowId::from(0), 0)
        .map_err(|error| anyhow!("failed to create pty event loop: {error}"))?;
    let sender = machine.channel();
    let _io_thread = machine.spawn();

    Ok(PtySender { sender })
}

pub(super) fn resize(term: &mut RioTerm, bounds: TerminalBounds) {
    term.resize(crosswords_size_from_terminal_bounds(bounds));
}

pub(super) fn display_offset(term: &RioTerm) -> usize {
    term.display_offset()
}

pub(super) fn scroll_display(term: &mut RioTerm, scroll: Scroll) {
    term.scroll_display(scroll.to_rio());
}

pub(super) fn set_selection(term: &mut RioTerm, selection: Option<&Selection>) {
    term.selection = selection.map(Selection::to_rio);
}

pub(super) fn update_selection(term: &mut RioTerm, point: Point, side: SelectionSide) -> bool {
    let Some(mut selection) = term.selection.take() else {
        return false;
    };
    selection.update(point.to_rio(), side.to_rio());
    term.selection = Some(selection);
    true
}

pub(super) fn selection_text(term: &RioTerm) -> Option<String> {
    term.selection_to_string()
}

pub(super) fn scroll_to_point(term: &mut RioTerm, point: Point) {
    term.scroll_to_pos(point.to_rio());
}

pub(super) fn vi_goto_point(term: &mut RioTerm, point: Point) {
    term.vi_goto_pos(point.to_rio());
}

pub(super) fn toggle_vi_mode(term: &mut RioTerm) {
    term.toggle_vi_mode();
}

pub(super) fn vi_motion(term: &mut RioTerm, motion: ViMotion) {
    term.vi_motion(motion.to_rio());
}

pub(super) fn cursor_blinking(term: &RioTerm) -> bool {
    term.blinking_cursor
}

/// Re-arms the terminal's damage notification. The PTY event loop suppresses
/// further `TerminalDamaged` events while one is in flight and expects the
/// consumer to clear the flag once it has observed the update; without this,
/// wakeups stop after the first damage event.
pub(super) fn rearm_damage_events(term: &mut RioTerm) {
    term.damage_event_in_flight = false;
    term.reset_damage();
}

pub(super) fn color_at_index(term: &RioTerm, index: usize) -> Option<Rgb> {
    term.colors()[index].map(rgb_from_color_array)
}

fn rgb_from_color_array(color: ColorArray) -> Rgb {
    Rgb {
        r: (color[0].clamp(0., 1.) * 255.) as u8,
        g: (color[1].clamp(0., 1.) * 255.) as u8,
        b: (color[2].clamp(0., 1.) * 255.) as u8,
    }
}

fn rio_cursor_shape(cursor_shape: SettingsCursorShape) -> RioCursorShape {
    match cursor_shape {
        SettingsCursorShape::Block => RioCursorShape::Block,
        SettingsCursorShape::Underline => RioCursorShape::Underline,
        SettingsCursorShape::Bar => RioCursorShape::Beam,
        // Rio's VT layer has no hollow-block shape; Zed re-applies the hollow
        // shape in `make_content` while the application hasn't overridden the
        // default cursor via escape sequences.
        SettingsCursorShape::Hollow => RioCursorShape::Block,
    }
}

fn backend_event_from_rio(event: RioEvent, osc52: Osc52) -> Option<TerminalBackendEvent> {
    match event {
        RioEvent::MouseCursorDirty => Some(TerminalBackendEvent::MouseCursorDirty),
        RioEvent::Title(title) => Some(TerminalBackendEvent::Title(title)),
        RioEvent::TitleWithSubtitle(title, _) => Some(TerminalBackendEvent::Title(title)),
        RioEvent::ResetTitle => Some(TerminalBackendEvent::ResetTitle),
        RioEvent::ClipboardStore(_, data) => {
            if osc52 == Osc52::OnlyCopy {
                Some(TerminalBackendEvent::ClipboardStore(data))
            } else {
                None
            }
        }
        // OSC 52 clipboard reads are never allowed (matching the alacritty
        // `Osc52::OnlyCopy` policy Zed used).
        RioEvent::ClipboardLoad(..) => None,
        RioEvent::ColorRequest(_, index, format) => Some(TerminalBackendEvent::ColorRequest(
            index,
            Arc::new(move |rgb: Rgb| {
                format(rio_vt::config::colors::ColorRgb {
                    r: rgb.r,
                    g: rgb.g,
                    b: rgb.b,
                })
            }),
        )),
        RioEvent::PtyWrite(_, output) => Some(TerminalBackendEvent::PtyWrite(output)),
        RioEvent::TextAreaSizeRequest(_, format) => Some(
            TerminalBackendEvent::TextAreaSizeRequest(Arc::new(move |bounds| {
                format(window_size_from_terminal_bounds(bounds))
            })),
        ),
        RioEvent::CursorBlinkingChange | RioEvent::CursorBlinkingChangeOnRoute(_) => {
            Some(TerminalBackendEvent::CursorBlinkingChange)
        }
        RioEvent::Render
        | RioEvent::RenderRoute(_)
        | RioEvent::TerminalDamaged(_)
        | RioEvent::PrepareRender(_)
        | RioEvent::PrepareRenderOnRoute(..) => Some(TerminalBackendEvent::Wakeup),
        RioEvent::Bell => Some(TerminalBackendEvent::Bell),
        RioEvent::ChildExited(_, status) => match status {
            Some(status) => Some(TerminalBackendEvent::ChildExit(exit_status_from_raw(status))),
            None => Some(TerminalBackendEvent::Exit),
        },
        RioEvent::Exit | RioEvent::Quit | RioEvent::CloseTerminal(_) => {
            Some(TerminalBackendEvent::Exit)
        }
        _ => None,
    }
}

#[cfg(unix)]
fn exit_status_from_raw(status: i32) -> std::process::ExitStatus {
    use std::os::unix::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(status)
}

#[cfg(windows)]
fn exit_status_from_raw(status: i32) -> std::process::ExitStatus {
    use std::os::windows::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(status as u32)
}

impl EventListener for ZedListener {
    fn event(&self) -> (Option<RioEvent>, bool) {
        (None, false)
    }

    fn send_event(&self, event: RioEvent, _window_id: WindowId) {
        if let Some(event) = backend_event_from_rio(event, self.osc52) {
            self.events_tx.unbounded_send(PtyEvent::Event(event)).ok();
        }
    }
}

impl Scroll {
    fn to_rio(self) -> RioScroll {
        match self {
            Self::Delta(delta) => RioScroll::Delta(delta),
            Self::PageUp => RioScroll::PageUp,
            Self::PageDown => RioScroll::PageDown,
            Self::Top => RioScroll::Top,
            Self::Bottom => RioScroll::Bottom,
        }
    }
}

impl ViMotion {
    fn to_rio(self) -> RioViMotion {
        match self {
            Self::Up => RioViMotion::Up,
            Self::Down => RioViMotion::Down,
            Self::Left => RioViMotion::Left,
            Self::Right => RioViMotion::Right,
            Self::First => RioViMotion::First,
            Self::Last => RioViMotion::Last,
            Self::FirstOccupied => RioViMotion::FirstOccupied,
            Self::High => RioViMotion::High,
            Self::Middle => RioViMotion::Middle,
            Self::Low => RioViMotion::Low,
            Self::WordLeft => RioViMotion::WordLeft,
            Self::WordRight => RioViMotion::WordRight,
            Self::WordRightEnd => RioViMotion::WordRightEnd,
            Self::Bracket => RioViMotion::Bracket,
            Self::ParagraphUp => RioViMotion::ParagraphUp,
            Self::ParagraphDown => RioViMotion::ParagraphDown,
        }
    }
}

impl Search {
    pub fn new(search: &str) -> Option<Self> {
        Some(Self {
            search: RioSearch {
                search: RegexSearch::new(search).ok()?,
            },
        })
    }

    fn into_rio(self) -> RegexSearch {
        self.search.search
    }
}

impl SelectionSide {
    fn to_rio(self) -> Side {
        match self {
            Self::Left => Side::Left,
            Self::Right => Side::Right,
        }
    }
}

impl SelectionType {
    fn to_rio(self) -> rio_vt::selection::SelectionType {
        match self {
            Self::Simple => rio_vt::selection::SelectionType::Simple,
            Self::Semantic => rio_vt::selection::SelectionType::Semantic,
            Self::Lines => rio_vt::selection::SelectionType::Lines,
        }
    }
}

impl Selection {
    fn to_rio(&self) -> rio_vt::selection::Selection {
        let mut selection = rio_vt::selection::Selection::new(
            self.ty.to_rio(),
            self.start.point.to_rio(),
            self.start.side.to_rio(),
        );
        if self.start.point != self.end.point || self.start.side != self.end.side {
            selection.update(self.end.point.to_rio(), self.end.side.to_rio());
        }
        selection
    }
}

impl Hyperlink {
    pub fn new<T: ToString>(id: Option<T>, uri: String) -> Self {
        Self {
            id: id.map(|id| Arc::from(id.to_string())),
            uri: Arc::from(uri),
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    fn from_rio(hyperlink: &RioHyperlink) -> Self {
        Self {
            id: Some(Arc::from(hyperlink.id())),
            uri: Arc::from(hyperlink.uri()),
        }
    }
}

fn resolve_cell(grid: &Grid<Square>, square: &Square) -> RioCell {
    let style = grid.style_set.get(square.style_id());
    let extras = square
        .extras_id()
        .and_then(|extras_id| grid.extras_table.get(extras_id));

    RioCell {
        // Never-written cells hold '\0' in rio's grid; Zed's renderer and
        // content checks expect blanks to read as spaces.
        c: match square.c() {
            '\0' => ' ',
            c => c,
        },
        fg: color_from_rio(style.fg),
        bg: color_from_rio(style.bg),
        flags: style.flags,
        is_wide_char_spacer: square.is_spacer(),
        zerowidth: extras
            .filter(|extras| !extras.zerowidth.is_empty())
            .map(|extras| Arc::from(extras.zerowidth.as_slice())),
        hyperlink: extras
            .and_then(|extras| extras.hyperlink.as_ref())
            .map(Hyperlink::from_rio),
    }
}

fn color_from_rio(color: AnsiColor) -> Color {
    match color {
        AnsiColor::Named(named) => Color::Named(named_color_from_rio(named)),
        AnsiColor::Spec(rgb) => Color::Spec(Rgb {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }),
        AnsiColor::Indexed(index) => Color::Indexed(index),
    }
}

fn named_color_from_rio(color: RioNamedColor) -> NamedColor {
    match color {
        RioNamedColor::Black => NamedColor::Black,
        RioNamedColor::Red => NamedColor::Red,
        RioNamedColor::Green => NamedColor::Green,
        RioNamedColor::Yellow => NamedColor::Yellow,
        RioNamedColor::Blue => NamedColor::Blue,
        RioNamedColor::Magenta => NamedColor::Magenta,
        RioNamedColor::Cyan => NamedColor::Cyan,
        RioNamedColor::White => NamedColor::White,
        RioNamedColor::LightBlack => NamedColor::BrightBlack,
        RioNamedColor::LightRed => NamedColor::BrightRed,
        RioNamedColor::LightGreen => NamedColor::BrightGreen,
        RioNamedColor::LightYellow => NamedColor::BrightYellow,
        RioNamedColor::LightBlue => NamedColor::BrightBlue,
        RioNamedColor::LightMagenta => NamedColor::BrightMagenta,
        RioNamedColor::LightCyan => NamedColor::BrightCyan,
        RioNamedColor::LightWhite => NamedColor::BrightWhite,
        RioNamedColor::Foreground => NamedColor::Foreground,
        RioNamedColor::Background => NamedColor::Background,
        RioNamedColor::Cursor => NamedColor::Cursor,
        RioNamedColor::DimBlack => NamedColor::DimBlack,
        RioNamedColor::DimRed => NamedColor::DimRed,
        RioNamedColor::DimGreen => NamedColor::DimGreen,
        RioNamedColor::DimYellow => NamedColor::DimYellow,
        RioNamedColor::DimBlue => NamedColor::DimBlue,
        RioNamedColor::DimMagenta => NamedColor::DimMagenta,
        RioNamedColor::DimCyan => NamedColor::DimCyan,
        RioNamedColor::DimWhite => NamedColor::DimWhite,
        RioNamedColor::LightForeground => NamedColor::BrightForeground,
        RioNamedColor::DimForeground => NamedColor::DimForeground,
    }
}

impl Cell {
    #[inline]
    pub fn character(&self) -> char {
        self.cell.c
    }

    #[cfg(test)]
    pub(crate) fn set_character(&mut self, character: char) {
        self.cell.c = character;
    }

    #[inline]
    pub fn foreground(&self) -> Color {
        self.cell.fg
    }

    #[inline]
    pub fn background(&self) -> Color {
        self.cell.bg
    }

    #[inline]
    pub fn zerowidth(&self) -> Option<&[char]> {
        self.cell.zerowidth.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn push_zerowidth(&mut self, character: char) {
        let mut zerowidth = self.cell.zerowidth.as_deref().unwrap_or(&[]).to_vec();
        zerowidth.push(character);
        self.cell.zerowidth = Some(Arc::from(zerowidth.as_slice()));
    }

    #[inline]
    pub fn hyperlink(&self) -> Option<Hyperlink> {
        self.cell.hyperlink.clone()
    }

    #[inline]
    pub fn is_inverse(&self) -> bool {
        self.cell.flags.contains(StyleFlags::INVERSE)
    }

    #[inline]
    pub fn is_wide_char_spacer(&self) -> bool {
        self.cell.is_wide_char_spacer
    }

    #[inline]
    pub fn is_dim(&self) -> bool {
        self.cell.flags.intersects(StyleFlags::DIM)
    }

    #[inline]
    pub fn has_underline(&self) -> bool {
        self.cell.flags.intersects(StyleFlags::ALL_UNDERLINES)
    }

    #[inline]
    pub fn has_undercurl(&self) -> bool {
        self.cell.flags.contains(StyleFlags::UNDERCURL)
    }

    #[inline]
    pub fn has_strikeout(&self) -> bool {
        self.cell.flags.intersects(StyleFlags::STRIKEOUT)
    }

    #[inline]
    pub fn is_bold(&self) -> bool {
        self.cell.flags.intersects(StyleFlags::BOLD)
    }

    #[inline]
    pub fn is_italic(&self) -> bool {
        self.cell.flags.intersects(StyleFlags::ITALIC)
    }

    #[inline]
    pub fn has_visible_style_modifier(&self) -> bool {
        self.cell.flags.intersects(
            StyleFlags::ALL_UNDERLINES | StyleFlags::INVERSE | StyleFlags::STRIKEOUT,
        )
    }
}

pub(super) fn renderable_cells(term: &RioTerm) -> RenderableCells<'_> {
    RenderableCells {
        cells: term.grid.display_iter(),
        grid: &term.grid,
    }
}

impl Iterator for RenderableCells<'_> {
    type Item = IndexedCell;

    fn next(&mut self) -> Option<Self::Item> {
        self.cells.next().map(|indexed| IndexedCell {
            point: terminal_point_from_rio(indexed.pos),
            cell: Cell {
                cell: resolve_cell(self.grid, indexed.square),
            },
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cells.size_hint()
    }
}

fn terminal_modes_from_rio(mode: RioMode) -> Modes {
    let mut terminal_modes = Modes::empty();
    let mut add = |rio_mode: RioMode, terminal_mode: Modes| {
        if mode.contains(rio_mode) {
            terminal_modes.insert(terminal_mode);
        }
    };
    add(RioMode::APP_CURSOR, Modes::APP_CURSOR);
    add(RioMode::APP_KEYPAD, Modes::APP_KEYPAD);
    add(RioMode::SHOW_CURSOR, Modes::SHOW_CURSOR);
    add(RioMode::LINE_WRAP, Modes::LINE_WRAP);
    add(RioMode::ORIGIN, Modes::ORIGIN);
    add(RioMode::INSERT, Modes::INSERT);
    add(RioMode::LINE_FEED_NEW_LINE, Modes::LINE_FEED_NEW_LINE);
    add(RioMode::FOCUS_IN_OUT, Modes::FOCUS_IN_OUT);
    add(RioMode::ALTERNATE_SCROLL, Modes::ALTERNATE_SCROLL);
    add(RioMode::BRACKETED_PASTE, Modes::BRACKETED_PASTE);
    add(RioMode::SGR_MOUSE, Modes::SGR_MOUSE);
    add(RioMode::UTF8_MOUSE, Modes::UTF8_MOUSE);
    add(RioMode::ALT_SCREEN, Modes::ALT_SCREEN);
    add(RioMode::MOUSE_REPORT_CLICK, Modes::MOUSE_REPORT_CLICK);
    add(RioMode::MOUSE_DRAG, Modes::MOUSE_DRAG);
    add(RioMode::MOUSE_MOTION, Modes::MOUSE_MOTION);
    add(RioMode::VI, Modes::VI);
    terminal_modes
}

impl Point {
    fn to_rio(self) -> Pos {
        Pos::new(Line(self.line), Column(self.column))
    }
}

fn terminal_point_from_rio(pos: Pos) -> Point {
    Point {
        line: pos.row.0,
        column: pos.col.0,
    }
}

impl Range {
    #[cfg(test)]
    pub(crate) fn to_rio(self) -> RangeInclusive<Pos> {
        self.start.to_rio()..=self.end.to_rio()
    }

    fn from_rio(range: RangeInclusive<Pos>) -> Self {
        Self {
            start: terminal_point_from_rio(*range.start()),
            end: terminal_point_from_rio(*range.end()),
        }
    }
}

fn terminal_selection_range_from_rio(
    range: rio_vt::selection::SelectionRange,
) -> SelectionRange {
    SelectionRange {
        start: terminal_point_from_rio(range.start),
        end: terminal_point_from_rio(range.end),
        is_block: range.is_block,
    }
}

fn terminal_cursor_shape_from_rio(shape: RioCursorShape) -> CursorShape {
    match shape {
        RioCursorShape::Block => CursorShape::Block,
        RioCursorShape::Underline => CursorShape::Underline,
        RioCursorShape::Beam => CursorShape::Bar,
        RioCursorShape::Hidden => CursorShape::Hidden,
    }
}

pub(super) fn clear_saved_screen(term: &mut RioTerm) {
    term.clear_screen(ClearMode::Saved);

    let cursor = term.grid.cursor.pos;

    term.grid.reset_region(..cursor.row);

    let columns = term.grid.columns();
    let line = (0..columns)
        .map(|column| term.grid[cursor.row][Column(column)])
        .collect::<Vec<Square>>();

    for (index, square) in line.into_iter().enumerate() {
        term.grid[Line(0)][Column(index)] = square;
    }

    term.grid.cursor.pos = Pos::new(Line(0), term.grid.cursor.pos.col);
    let new_cursor = term.grid.cursor.pos;

    if (new_cursor.row.0 as usize) < term.screen_lines() - 1 {
        term.grid.reset_region((new_cursor.row + 1)..);
    }
    term.mark_fully_damaged();
}

pub(super) fn shrink_to_used(term: &mut RioTerm) {
    term.grid.truncate();
}

pub(super) fn make_content(
    term: &RioTerm,
    last_content: &Content,
    config: &RioTermConfig,
) -> Content {
    let grid = &term.grid;
    let display_iter = grid.display_iter();

    let estimated_size = display_iter.size_hint().0;
    let mut cells = Vec::with_capacity(estimated_size);
    cells.extend(display_iter.map(|indexed| IndexedCell {
        point: terminal_point_from_rio(indexed.pos),
        cell: Cell {
            cell: resolve_cell(grid, indexed.square),
        },
    }));

    let selection = term
        .selection
        .as_ref()
        .and_then(|selection| selection.to_range(term));
    let selection_text = if selection.is_some() {
        term.selection_to_string()
    } else {
        None
    };

    let display_offset = term.display_offset();
    let cursor = renderable_cursor(term, config);
    let cursor_point = cursor.point;

    let bottom_line = term.screen_lines() as i32 - 1 - display_offset as i32;
    let bottom_row_occupied = cursor_point.line >= bottom_line
        || cells
            .iter()
            .rev()
            .take_while(|cell| cell.point.line >= bottom_line)
            .any(|cell| cell.cell.character() != ' ');

    Content {
        cells,
        mode: terminal_modes_from_rio(term.mode()),
        display_offset,
        selection_text,
        selection: selection.map(terminal_selection_range_from_rio),
        cursor,
        cursor_char: match grid[cursor_point.to_rio()].c() {
            '\0' => ' ',
            c => c,
        },
        terminal_bounds: last_content.terminal_bounds,
        last_hovered_word: last_content.last_hovered_word.clone(),
        scrolled_to_top: display_offset == term.history_size(),
        scrolled_to_bottom: display_offset == 0,
        bottom_row_occupied,
    }
}

fn renderable_cursor(term: &RioTerm, config: &RioTermConfig) -> Cursor {
    let vi_mode = term.mode().contains(RioMode::VI);
    let mut pos = if vi_mode {
        term.vi_mode_cursor.pos
    } else {
        term.grid.cursor.pos
    };
    if term.grid[pos].is_spacer() && pos.col.0 > 0 {
        pos.col -= 1;
    }

    let shape = if !vi_mode && !term.mode().contains(RioMode::SHOW_CURSOR) {
        CursorShape::Hidden
    } else if term.cursor_shape == term.default_cursor_shape {
        // The application hasn't overridden the default cursor shape, so
        // apply the configured one; this also restores shapes rio's VT layer
        // cannot represent (hollow block).
        CursorShape::from(config.default_cursor_shape)
    } else {
        terminal_cursor_shape_from_rio(term.cursor_shape)
    };

    Cursor {
        shape,
        point: terminal_point_from_rio(pos),
    }
}

pub(super) fn content_text(term: &RioTerm) -> String {
    let start = Pos::new(term.grid.topmost_line(), Column(0));
    let end = Pos::new(term.grid.bottommost_line(), term.grid.last_column());
    term.bounds_to_string(start, end)
}

pub(super) fn total_lines(term: &RioTerm) -> usize {
    term.grid.total_lines()
}

pub(super) fn screen_lines(term: &RioTerm) -> usize {
    term.screen_lines()
}

pub(super) fn full_content_range(term: &RioTerm) -> Range {
    let start = Pos::new(term.grid.topmost_line(), Column(0));
    let end = Pos::new(term.grid.bottommost_line(), term.grid.last_column());
    Range::from_rio(start..=end)
}

pub(super) fn last_non_empty_lines(term: &RioTerm, line_count: usize) -> Vec<String> {
    let grid = &term.grid;
    let mut lines = Vec::new();

    let mut current_line = grid.bottommost_line().0;
    let topmost_line = grid.topmost_line().0;

    while current_line >= topmost_line && lines.len() < line_count {
        let (logical_line_start, logical_line) =
            logical_line_for_row(grid, current_line, topmost_line);

        if let Some(line) = process_line(logical_line) {
            lines.push(line);
        }

        current_line = logical_line_start - 1;
    }

    lines.reverse();
    lines
}

pub(super) fn update_vi_cursor_for_scroll(term: &mut RioTerm, scroll: Scroll) {
    match scroll {
        Scroll::Delta(delta) => {
            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, delta);
        }
        Scroll::PageUp => {
            let lines = term.screen_lines() as i32;
            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, lines);
        }
        Scroll::PageDown => {
            let lines = -(term.screen_lines() as i32);
            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, lines);
        }
        Scroll::Top => {
            let pos = Pos::new(term.grid.topmost_line(), Column(0));
            term.vi_mode_cursor = ViModeCursor::new(pos);
        }
        Scroll::Bottom => {
            let pos = Pos::new(term.grid.bottommost_line(), Column(0));
            term.vi_mode_cursor = ViModeCursor::new(pos);
        }
    }
}

pub(super) fn update_selection_to_vi_cursor(term: &mut RioTerm) -> Option<Point> {
    let mut selection = term.selection.take()?;
    let pos = term.vi_mode_cursor.pos;
    selection.update(pos, Side::Right);
    term.selection = Some(selection);
    Some(terminal_point_from_rio(pos))
}

pub(super) fn find_from_terminal_point(
    term: &RioTerm,
    point: Point,
    regex_searches: &mut RegexSearches,
    path_style: PathStyle,
) -> Option<HyperlinkMatch> {
    let pos = point.to_rio().grid_clamp(term, Boundary::Grid);
    hyperlinks::find_from_grid_point(term, pos, regex_searches, path_style)
}

fn logical_line_for_row(grid: &Grid<Square>, current: i32, topmost: i32) -> (i32, String) {
    let start = find_logical_line_start(grid, current, topmost);
    let mut line = String::new();
    for row in start..=current {
        line.push_str(&row_to_string(&grid[Line(row)]));
    }
    (start, line)
}

fn find_logical_line_start(grid: &Grid<Square>, current: i32, topmost: i32) -> i32 {
    let mut line_start = current;
    while line_start > topmost {
        let previous_line = Line(line_start - 1);
        let last_square = &grid[previous_line][Column(grid.columns() - 1)];
        if !last_square.wrapline() {
            break;
        }
        line_start -= 1;
    }
    line_start
}

fn row_to_string(row: &Row<Square>) -> String {
    (0..row.len())
        .map(|column| match row[Column(column)].c() {
            '\0' => ' ',
            c => c,
        })
        .collect::<String>()
}

fn process_line(line: String) -> Option<String> {
    let trimmed = line.trim_end().to_string();
    if !trimmed.is_empty() {
        Some(trimmed)
    } else {
        None
    }
}

/// Appends a stringified task summary to the terminal, after its output.
///
/// This should only be called after the terminal's PTY is no longer alive:
/// the text is fed straight through the VT parser, as if the child process
/// had written it, so a live PTY would interleave its own output with it.
pub(super) fn append_text_to_term(term: &mut RioTerm, text_lines: &[&str]) {
    let mut processor = Processor::default();
    let mut bytes = Vec::with_capacity(
        text_lines
            .iter()
            .map(|line| line.len() + 2)
            .sum::<usize>()
            + 2,
    );
    bytes.extend_from_slice(b"\r\n");
    for line in text_lines {
        bytes.extend_from_slice(line.as_bytes());
        bytes.extend_from_slice(b"\r\n");
    }
    processor.advance(term, &bytes);
}

pub(super) fn search_matches(term: &RioTerm, searcher: Search) -> Vec<Range> {
    let mut searcher = searcher.into_rio();
    all_search_matches(term, &mut searcher)
        .map(Range::from_rio)
        .collect()
}

fn all_search_matches<'a, T: EventListener>(
    term: &'a Crosswords<T>,
    regex: &'a mut RegexSearch,
) -> impl Iterator<Item = Match> + 'a {
    let start = Pos::new(term.grid.topmost_line(), Column(0));
    let end = Pos::new(term.grid.bottommost_line(), term.grid.last_column());
    RegexIter::new(start, end, RioDirection::Right, term, regex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyperlink_from_rio_keeps_id() {
        let hyperlink = RioHyperlink::new(Some("id"), "https://example.com");
        let hyperlink = Hyperlink::from_rio(&hyperlink);

        assert_eq!(hyperlink.id(), Some("id"));
        assert_eq!(hyperlink.uri(), "https://example.com");
    }

    #[test]
    fn terminal_modes_from_rio_flags() {
        let rio_modes = RioMode::APP_CURSOR
            | RioMode::BRACKETED_PASTE
            | RioMode::ALT_SCREEN
            | RioMode::MOUSE_DRAG
            | RioMode::SGR_MOUSE
            | RioMode::VI;

        let terminal_modes = terminal_modes_from_rio(rio_modes);
        assert!(terminal_modes.contains(Modes::APP_CURSOR));
        assert!(terminal_modes.contains(Modes::BRACKETED_PASTE));
        assert!(terminal_modes.contains(Modes::ALT_SCREEN));
        assert!(terminal_modes.contains(Modes::MOUSE_DRAG));
        assert!(terminal_modes.intersects(Modes::MOUSE_MODE));
        assert!(terminal_modes.contains(Modes::SGR_MOUSE));
        assert!(terminal_modes.contains(Modes::VI));
        assert!(!terminal_modes.contains(Modes::MOUSE_REPORT_CLICK));
    }

    #[test]
    fn terminal_selection_range_from_rio_range() {
        let rio_range = rio_vt::selection::SelectionRange {
            start: Pos::new(Line(-2), Column(3)),
            end: Pos::new(Line(4), Column(8)),
            is_block: true,
        };

        let terminal_range = terminal_selection_range_from_rio(rio_range);
        assert_eq!(
            terminal_range,
            SelectionRange {
                start: Point {
                    line: -2,
                    column: 3
                },
                end: Point { line: 4, column: 8 },
                is_block: true,
            }
        );
    }

    #[test]
    fn named_colors_map_to_vte_equivalents() {
        assert_eq!(
            color_from_rio(AnsiColor::Named(RioNamedColor::LightBlack)),
            Color::Named(NamedColor::BrightBlack)
        );
        assert_eq!(
            color_from_rio(AnsiColor::Named(RioNamedColor::Foreground)),
            Color::Named(NamedColor::Foreground)
        );
        assert_eq!(
            color_from_rio(AnsiColor::Spec(rio_vt::config::colors::ColorRgb {
                r: 1,
                g: 2,
                b: 3
            })),
            Color::Spec(Rgb { r: 1, g: 2, b: 3 })
        );
        assert_eq!(color_from_rio(AnsiColor::Indexed(42)), Color::Indexed(42));
    }
}
