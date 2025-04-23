pub mod mappings;

pub use alacritty_terminal;

mod pty_info;
pub mod terminal_settings;

use alacritty_terminal::{
    Term,
    event::{Event as AlacTermEvent, EventListener, Notify, WindowSize},
    event_loop::{EventLoop, Msg, Notifier},
    grid::{Dimensions, Grid, Row, Scroll as AlacScroll},
    index::{Boundary, Column, Direction as AlacDirection, Line, Point as AlacPoint},
    selection::{Selection, SelectionRange, SelectionType},
    sync::FairMutex,
    term::{
        Config, RenderableCursor, TermMode,
        cell::{Cell, Flags},
        search::{Match, RegexIter, RegexSearch},
    },
    tty::{self},
    vi_mode::{ViModeCursor, ViMotion},
    vte::ansi::{
        ClearMode, CursorStyle as AlacCursorStyle, Handler, NamedPrivateMode, PrivateMode,
    },
};
use anyhow::{Result, bail};

use futures::{
    FutureExt,
    channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded},
};

use mappings::mouse::{
    alt_scroll, grid_point, grid_point_and_side, mouse_button_report, mouse_moved_report,
    scroll_report,
};

use collections::{HashMap, VecDeque};
use futures::StreamExt;
use pty_info::PtyProcessInfo;
use regex::Regex;
use serde::{Deserialize, Serialize};
use settings::Settings;
use smol::channel::{Receiver, Sender};
use task::{HideStrategy, Shell, TaskId};
use terminal_settings::{AlternateScroll, CursorShape, TerminalSettings};
use theme::{ActiveTheme, Theme};
use util::{ResultExt, paths::home_dir, truncate_and_trailoff};

use std::{
    cmp::{self, min},
    fmt::Display,
    ops::{Deref, Index, RangeInclusive},
    path::PathBuf,
    process::ExitStatus,
    sync::{Arc, LazyLock},
    time::Duration,
};
use thiserror::Error;

use gpui::{
    AnyWindowHandle, App, AppContext as _, Bounds, ClipboardItem, Context, EventEmitter, Hsla,
    KeyboardMapper, Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Point, Rgba, ScrollWheelEvent, SharedString, Size, Task, TouchPhase,
    Window, actions, black, px,
};

use crate::mappings::{colors::to_alac_rgb, keys::to_esc_str};

actions!(
    terminal,
    [
        Clear,
        Copy,
        Paste,
        ShowCharacterPalette,
        SearchTest,
        ScrollLineUp,
        ScrollLineDown,
        ScrollPageUp,
        ScrollPageDown,
        ScrollToTop,
        ScrollToBottom,
        ToggleViMode,
    ]
);

///Scrolling is unbearably sluggish by default. Alacritty supports a configurable
///Scroll multiplier that is set to 3 by default. This will be removed when I
///Implement scroll bars.
#[cfg(target_os = "macos")]
const SCROLL_MULTIPLIER: f32 = 4.;
#[cfg(not(target_os = "macos"))]
const SCROLL_MULTIPLIER: f32 = 1.;
const MAX_SEARCH_LINES: usize = 100;
const DEBUG_TERMINAL_WIDTH: Pixels = px(500.);
const DEBUG_TERMINAL_HEIGHT: Pixels = px(30.);
const DEBUG_CELL_WIDTH: Pixels = px(5.);
const DEBUG_LINE_HEIGHT: Pixels = px(5.);

///Upward flowing events, for changing the title and such
#[derive(Clone, Debug)]
pub enum Event {
    TitleChanged,
    BreadcrumbsChanged,
    CloseTerminal,
    Bell,
    Wakeup,
    BlinkChanged(bool),
    SelectionsChanged,
    NewNavigationTarget(Option<MaybeNavigationTarget>),
    Open(MaybeNavigationTarget),
}

#[derive(Clone, Debug)]
pub struct PathLikeTarget {
    /// File system path, absolute or relative, existing or not.
    /// Might have line and column number(s) attached as `file.rs:1:23`
    pub maybe_path: String,
    /// Current working directory of the terminal
    pub terminal_dir: Option<PathBuf>,
}

/// A string inside terminal, potentially useful as a URI that can be opened.
#[derive(Clone, Debug)]
pub enum MaybeNavigationTarget {
    /// HTTP, git, etc. string determined by the `URL_REGEX` regex.
    Url(String),
    /// File system path, absolute or relative, existing or not.
    /// Might have line and column number(s) attached as `file.rs:1:23`
    PathLike(PathLikeTarget),
}

#[derive(Clone)]
enum InternalEvent {
    Resize(TerminalBounds),
    Clear,
    // FocusNextMatch,
    Scroll(AlacScroll),
    ScrollToAlacPoint(AlacPoint),
    SetSelection(Option<(Selection, AlacPoint)>),
    UpdateSelection(Point<Pixels>),
    // Adjusted mouse position, should open
    FindHyperlink(Point<Pixels>, bool),
    Copy,
    // Vi mode events
    ToggleViMode,
    ViMotion(ViMotion),
}

///A translation struct for Alacritty to communicate with us from their event loop
#[derive(Clone)]
pub struct ZedListener(pub UnboundedSender<AlacTermEvent>);

impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(event).ok();
    }
}

pub fn init(cx: &mut App) {
    TerminalSettings::register(cx);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalBounds {
    pub cell_width: Pixels,
    pub line_height: Pixels,
    pub bounds: Bounds<Pixels>,
}

impl TerminalBounds {
    pub fn new(line_height: Pixels, cell_width: Pixels, bounds: Bounds<Pixels>) -> Self {
        TerminalBounds {
            cell_width,
            line_height,
            bounds,
        }
    }

    pub fn num_lines(&self) -> usize {
        (self.bounds.size.height / self.line_height).floor() as usize
    }

    pub fn num_columns(&self) -> usize {
        (self.bounds.size.width / self.cell_width).floor() as usize
    }

    pub fn height(&self) -> Pixels {
        self.bounds.size.height
    }

    pub fn width(&self) -> Pixels {
        self.bounds.size.width
    }

    pub fn cell_width(&self) -> Pixels {
        self.cell_width
    }

    pub fn line_height(&self) -> Pixels {
        self.line_height
    }
}

impl Default for TerminalBounds {
    fn default() -> Self {
        TerminalBounds::new(
            DEBUG_LINE_HEIGHT,
            DEBUG_CELL_WIDTH,
            Bounds {
                origin: Point::default(),
                size: Size {
                    width: DEBUG_TERMINAL_WIDTH,
                    height: DEBUG_TERMINAL_HEIGHT,
                },
            },
        )
    }
}

impl From<TerminalBounds> for WindowSize {
    fn from(val: TerminalBounds) -> Self {
        WindowSize {
            num_lines: val.num_lines() as u16,
            num_cols: val.num_columns() as u16,
            cell_width: f32::from(val.cell_width()) as u16,
            cell_height: f32::from(val.line_height()) as u16,
        }
    }
}

impl Dimensions for TerminalBounds {
    /// Note: this is supposed to be for the back buffer's length,
    /// but we exclusively use it to resize the terminal, which does not
    /// use this method. We still have to implement it for the trait though,
    /// hence, this comment.
    fn total_lines(&self) -> usize {
        self.screen_lines()
    }

    fn screen_lines(&self) -> usize {
        self.num_lines()
    }

    fn columns(&self) -> usize {
        self.num_columns()
    }
}

#[derive(Error, Debug)]
pub struct TerminalError {
    pub directory: Option<PathBuf>,
    pub shell: Shell,
    pub source: std::io::Error,
}

impl TerminalError {
    pub fn fmt_directory(&self) -> String {
        self.directory
            .clone()
            .map(|path| {
                match path
                    .into_os_string()
                    .into_string()
                    .map_err(|os_str| format!("<non-utf8 path> {}", os_str.to_string_lossy()))
                {
                    Ok(s) => s,
                    Err(s) => s,
                }
            })
            .unwrap_or_else(|| {
                let default_dir =
                    dirs::home_dir().map(|buf| buf.into_os_string().to_string_lossy().to_string());
                match default_dir {
                    Some(dir) => format!("<none specified, using home directory> {}", dir),
                    None => "<none specified, could not find home directory>".to_string(),
                }
            })
    }

    pub fn fmt_shell(&self) -> String {
        match &self.shell {
            Shell::System => "<system defined shell>".to_string(),
            Shell::Program(s) => s.to_string(),
            Shell::WithArguments {
                program,
                args,
                title_override,
            } => {
                if let Some(title_override) = title_override {
                    format!("{} {} ({})", program, args.join(" "), title_override)
                } else {
                    format!("{} {}", program, args.join(" "))
                }
            }
        }
    }
}

impl Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_string: String = self.fmt_directory();
        let shell = self.fmt_shell();

        write!(
            f,
            "Working directory: {} Shell command: `{}`, IOError: {}",
            dir_string, shell, self.source
        )
    }
}

// https://github.com/alacritty/alacritty/blob/cb3a79dbf6472740daca8440d5166c1d4af5029e/extra/man/alacritty.5.scd?plain=1#L207-L213
const DEFAULT_SCROLL_HISTORY_LINES: usize = 10_000;
const MAX_SCROLL_HISTORY_LINES: usize = 100_000;
const URL_REGEX: &str = r#"(ipfs:|ipns:|magnet:|mailto:|gemini://|gopher://|https://|http://|news:|file://|git://|ssh:|ftp://)[^\u{0000}-\u{001F}\u{007F}-\u{009F}<>"\s{-}\^⟨⟩`]+"#;
// Optional suffix matches MSBuild diagnostic suffixes for path parsing in PathLikeWithPosition
// https://learn.microsoft.com/en-us/visualstudio/msbuild/msbuild-diagnostic-format-for-tasks
const WORD_REGEX: &str =
    r#"[\$\+\w.\[\]:/\\@\-~()]+(?:\((?:\d+|\d+,\d+)\))|[\$\+\w.\[\]:/\\@\-~()]+"#;
const PYTHON_FILE_LINE_REGEX: &str = r#"File "(?P<file>[^"]+)", line (?P<line>\d+)"#;

static PYTHON_FILE_LINE_MATCHER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(PYTHON_FILE_LINE_REGEX).unwrap());

fn python_extract_path_and_line(input: &str) -> Option<(&str, u32)> {
    if let Some(captures) = PYTHON_FILE_LINE_MATCHER.captures(input) {
        let path_part = captures.name("file")?.as_str();

        let line_number: u32 = captures.name("line")?.as_str().parse().ok()?;
        return Some((path_part, line_number));
    }
    None
}

pub struct TerminalBuilder {
    terminal: Terminal,
    events_rx: UnboundedReceiver<AlacTermEvent>,
}

impl TerminalBuilder {
    pub fn new(
        working_directory: Option<PathBuf>,
        python_venv_directory: Option<PathBuf>,
        task: Option<TaskState>,
        shell: Shell,
        mut env: HashMap<String, String>,
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        is_ssh_terminal: bool,
        window: AnyWindowHandle,
        completion_tx: Sender<Option<ExitStatus>>,
        cx: &App,
    ) -> Result<TerminalBuilder> {
        // If the parent environment doesn't have a locale set
        // (As is the case when launched from a .app on MacOS),
        // and the Project doesn't have a locale set, then
        // set a fallback for our child environment to use.
        if std::env::var("LANG").is_err() {
            env.entry("LANG".to_string())
                .or_insert_with(|| "en_US.UTF-8".to_string());
        }

        env.insert("ZED_TERM".to_string(), "true".to_string());
        env.insert("TERM_PROGRAM".to_string(), "zed".to_string());
        env.insert("TERM".to_string(), "xterm-256color".to_string());
        env.insert(
            "TERM_PROGRAM_VERSION".to_string(),
            release_channel::AppVersion::global(cx).to_string(),
        );

        let mut terminal_title_override = None;

        let pty_options = {
            let alac_shell = match shell.clone() {
                Shell::System => {
                    #[cfg(target_os = "windows")]
                    {
                        Some(alacritty_terminal::tty::Shell::new(
                            util::get_windows_system_shell(),
                            Vec::new(),
                        ))
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        None
                    }
                }
                Shell::Program(program) => {
                    Some(alacritty_terminal::tty::Shell::new(program, Vec::new()))
                }
                Shell::WithArguments {
                    program,
                    args,
                    title_override,
                } => {
                    terminal_title_override = title_override;
                    Some(alacritty_terminal::tty::Shell::new(program, args))
                }
            };

            alacritty_terminal::tty::Options {
                shell: alac_shell,
                working_directory: working_directory
                    .clone()
                    .or_else(|| Some(home_dir().to_path_buf())),
                drain_on_exit: true,
                env: env.into_iter().collect(),
            }
        };

        // Setup Alacritty's env, which modifies the current process's environment
        alacritty_terminal::tty::setup_env();

        let default_cursor_style = AlacCursorStyle::from(cursor_shape);
        let scrolling_history = if task.is_some() {
            // Tasks like `cargo build --all` may produce a lot of output, ergo allow maximum scrolling.
            // After the task finishes, we do not allow appending to that terminal, so small tasks output should not
            // cause excessive memory usage over time.
            MAX_SCROLL_HISTORY_LINES
        } else {
            max_scroll_history_lines
                .unwrap_or(DEFAULT_SCROLL_HISTORY_LINES)
                .min(MAX_SCROLL_HISTORY_LINES)
        };
        let config = Config {
            scrolling_history,
            default_cursor_style,
            ..Config::default()
        };

        //Spawn a task so the Alacritty EventLoop can communicate with us
        //TODO: Remove with a bounded sender which can be dispatched on &self
        let (events_tx, events_rx) = unbounded();
        //Set up the terminal...
        let mut term = Term::new(
            config.clone(),
            &TerminalBounds::default(),
            ZedListener(events_tx.clone()),
        );

        //Alacritty defaults to alternate scrolling being on, so we just need to turn it off.
        if let AlternateScroll::Off = alternate_scroll {
            term.unset_private_mode(PrivateMode::Named(NamedPrivateMode::AlternateScroll));
        }

        let term = Arc::new(FairMutex::new(term));

        //Setup the pty...
        let pty = match tty::new(
            &pty_options,
            TerminalBounds::default().into(),
            window.window_id().as_u64(),
        ) {
            Ok(pty) => pty,
            Err(error) => {
                bail!(TerminalError {
                    directory: working_directory,
                    shell,
                    source: error,
                });
            }
        };

        let pty_info = PtyProcessInfo::new(&pty);

        //And connect them together
        let event_loop = EventLoop::new(
            term.clone(),
            ZedListener(events_tx.clone()),
            pty,
            pty_options.drain_on_exit,
            false,
        )?;

        //Kick things off
        let pty_tx = event_loop.channel();
        let _io_thread = event_loop.spawn(); // DANGER

        let terminal = Terminal {
            task,
            pty_tx: Notifier(pty_tx),
            completion_tx,
            term,
            term_config: config,
            title_override: terminal_title_override,
            events: VecDeque::with_capacity(10), //Should never get this high.
            last_content: Default::default(),
            last_mouse: None,
            matches: Vec::new(),
            selection_head: None,
            pty_info,
            breadcrumb_text: String::new(),
            scroll_px: px(0.),
            next_link_id: 0,
            selection_phase: SelectionPhase::Ended,
            // hovered_word: false,
            url_regex: RegexSearch::new(URL_REGEX).unwrap(),
            word_regex: RegexSearch::new(WORD_REGEX).unwrap(),
            python_file_line_regex: RegexSearch::new(PYTHON_FILE_LINE_REGEX).unwrap(),
            vi_mode_enabled: false,
            is_ssh_terminal,
            python_venv_directory,
        };

        Ok(TerminalBuilder {
            terminal,
            events_rx,
        })
    }

    pub fn subscribe(mut self, cx: &Context<Terminal>) -> Terminal {
        //Event loop
        cx.spawn(async move |terminal, cx| {
            while let Some(event) = self.events_rx.next().await {
                terminal.update(cx, |terminal, cx| {
                    //Process the first event immediately for lowered latency
                    terminal.process_event(&event, cx);
                })?;

                'outer: loop {
                    let mut events = Vec::new();
                    let mut timer = cx
                        .background_executor()
                        .timer(Duration::from_millis(4))
                        .fuse();
                    let mut wakeup = false;
                    loop {
                        futures::select_biased! {
                            _ = timer => break,
                            event = self.events_rx.next() => {
                                if let Some(event) = event {
                                    if matches!(event, AlacTermEvent::Wakeup) {
                                        wakeup = true;
                                    } else {
                                        events.push(event);
                                    }

                                    if events.len() > 100 {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            },
                        }
                    }

                    if events.is_empty() && !wakeup {
                        smol::future::yield_now().await;
                        break 'outer;
                    }

                    terminal.update(cx, |this, cx| {
                        if wakeup {
                            this.process_event(&AlacTermEvent::Wakeup, cx);
                        }

                        for event in events {
                            this.process_event(&event, cx);
                        }
                    })?;
                    smol::future::yield_now().await;
                }
            }

            anyhow::Ok(())
        })
        .detach();

        self.terminal
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexedCell {
    pub point: AlacPoint,
    pub cell: Cell,
}

impl Deref for IndexedCell {
    type Target = Cell;

    #[inline]
    fn deref(&self) -> &Cell {
        &self.cell
    }
}

// TODO: Un-pub
#[derive(Clone)]
pub struct TerminalContent {
    pub cells: Vec<IndexedCell>,
    pub mode: TermMode,
    pub display_offset: usize,
    pub selection_text: Option<String>,
    pub selection: Option<SelectionRange>,
    pub cursor: RenderableCursor,
    pub cursor_char: char,
    pub terminal_bounds: TerminalBounds,
    pub last_hovered_word: Option<HoveredWord>,
}

#[derive(Clone)]
pub struct HoveredWord {
    pub word: String,
    pub word_match: RangeInclusive<AlacPoint>,
    pub id: usize,
}

impl Default for TerminalContent {
    fn default() -> Self {
        TerminalContent {
            cells: Default::default(),
            mode: Default::default(),
            display_offset: Default::default(),
            selection_text: Default::default(),
            selection: Default::default(),
            cursor: RenderableCursor {
                shape: alacritty_terminal::vte::ansi::CursorShape::Block,
                point: AlacPoint::new(Line(0), Column(0)),
            },
            cursor_char: Default::default(),
            terminal_bounds: Default::default(),
            last_hovered_word: None,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum SelectionPhase {
    Selecting,
    Ended,
}

pub struct Terminal {
    pty_tx: Notifier,
    completion_tx: Sender<Option<ExitStatus>>,
    term: Arc<FairMutex<Term<ZedListener>>>,
    term_config: Config,
    events: VecDeque<InternalEvent>,
    /// This is only used for mouse mode cell change detection
    last_mouse: Option<(AlacPoint, AlacDirection)>,
    pub matches: Vec<RangeInclusive<AlacPoint>>,
    pub last_content: TerminalContent,
    pub selection_head: Option<AlacPoint>,
    pub breadcrumb_text: String,
    pub pty_info: PtyProcessInfo,
    title_override: Option<SharedString>,
    pub python_venv_directory: Option<PathBuf>,
    scroll_px: Pixels,
    next_link_id: usize,
    selection_phase: SelectionPhase,
    url_regex: RegexSearch,
    word_regex: RegexSearch,
    python_file_line_regex: RegexSearch,
    task: Option<TaskState>,
    vi_mode_enabled: bool,
    is_ssh_terminal: bool,
}

pub struct TaskState {
    pub id: TaskId,
    pub full_label: String,
    pub label: String,
    pub command_label: String,
    pub status: TaskStatus,
    pub completion_rx: Receiver<Option<ExitStatus>>,
    pub hide: HideStrategy,
    pub show_summary: bool,
    pub show_command: bool,
    pub show_rerun: bool,
}

/// A status of the current terminal tab's task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// The task had been started, but got cancelled or somehow otherwise it did not
    /// report its exit code before the terminal event loop was shut down.
    Unknown,
    /// The task is started and running currently.
    Running,
    /// After the start, the task stopped running and reported its error code back.
    Completed { success: bool },
}

impl TaskStatus {
    fn register_terminal_exit(&mut self) {
        if self == &Self::Running {
            *self = Self::Unknown;
        }
    }

    fn register_task_exit(&mut self, error_code: i32) {
        *self = TaskStatus::Completed {
            success: error_code == 0,
        };
    }
}

impl Terminal {
    fn process_event(&mut self, event: &AlacTermEvent, cx: &mut Context<Self>) {
        match event {
            AlacTermEvent::Title(title) => {
                self.breadcrumb_text = title.to_string();
                cx.emit(Event::BreadcrumbsChanged);
            }
            AlacTermEvent::ResetTitle => {
                self.breadcrumb_text = String::new();
                cx.emit(Event::BreadcrumbsChanged);
            }
            AlacTermEvent::ClipboardStore(_, data) => {
                cx.write_to_clipboard(ClipboardItem::new_string(data.to_string()))
            }
            AlacTermEvent::ClipboardLoad(_, format) => {
                self.write_to_pty(
                    match &cx.read_from_clipboard().and_then(|item| item.text()) {
                        // The terminal only supports pasting strings, not images.
                        Some(text) => format(text),
                        _ => format(""),
                    },
                )
            }
            AlacTermEvent::PtyWrite(out) => self.write_to_pty(out.clone()),
            AlacTermEvent::TextAreaSizeRequest(format) => {
                self.write_to_pty(format(self.last_content.terminal_bounds.into()))
            }
            AlacTermEvent::CursorBlinkingChange => {
                let terminal = self.term.lock();
                let blinking = terminal.cursor_style().blinking;
                cx.emit(Event::BlinkChanged(blinking));
            }
            AlacTermEvent::Bell => {
                cx.emit(Event::Bell);
            }
            AlacTermEvent::Exit => self.register_task_finished(None, cx),
            AlacTermEvent::MouseCursorDirty => {
                //NOOP, Handled in render
            }
            AlacTermEvent::Wakeup => {
                cx.emit(Event::Wakeup);

                if self.pty_info.has_changed() {
                    cx.emit(Event::TitleChanged);
                }
            }
            AlacTermEvent::ColorRequest(index, format) => {
                // It's important that the color request is processed here to retain relative order
                // with other PTY writes. Otherwise applications might witness out-of-order
                // responses to requests. For example: An application sending `OSC 11 ; ? ST`
                // (color request) followed by `CSI c` (request device attributes) would receive
                // the response to `CSI c` first.
                // Instead of locking, we could store the colors in `self.last_content`. But then
                // we might respond with out of date value if a "set color" sequence is immediately
                // followed by a color request sequence.
                let color = self.term.lock().colors()[*index].unwrap_or_else(|| {
                    to_alac_rgb(get_color_at_index(*index, cx.theme().as_ref()))
                });
                self.write_to_pty(format(color));
            }
            AlacTermEvent::ChildExit(error_code) => {
                self.register_task_finished(Some(*error_code), cx);
            }
        }
    }

    pub fn selection_started(&self) -> bool {
        self.selection_phase == SelectionPhase::Selecting
    }

    fn process_terminal_event(
        &mut self,
        event: &InternalEvent,
        term: &mut Term<ZedListener>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            &InternalEvent::Resize(mut new_bounds) => {
                new_bounds.bounds.size.height =
                    cmp::max(new_bounds.line_height, new_bounds.height());
                new_bounds.bounds.size.width = cmp::max(new_bounds.cell_width, new_bounds.width());

                self.last_content.terminal_bounds = new_bounds;

                self.pty_tx.0.send(Msg::Resize(new_bounds.into())).ok();

                term.resize(new_bounds);
            }
            InternalEvent::Clear => {
                // Clear back buffer
                term.clear_screen(ClearMode::Saved);

                let cursor = term.grid().cursor.point;

                // Clear the lines above
                term.grid_mut().reset_region(..cursor.line);

                // Copy the current line up
                let line = term.grid()[cursor.line][..Column(term.grid().columns())]
                    .iter()
                    .cloned()
                    .enumerate()
                    .collect::<Vec<(usize, Cell)>>();

                for (i, cell) in line {
                    term.grid_mut()[Line(0)][Column(i)] = cell;
                }

                // Reset the cursor
                term.grid_mut().cursor.point =
                    AlacPoint::new(Line(0), term.grid_mut().cursor.point.column);
                let new_cursor = term.grid().cursor.point;

                // Clear the lines below the new cursor
                if (new_cursor.line.0 as usize) < term.screen_lines() - 1 {
                    term.grid_mut().reset_region((new_cursor.line + 1)..);
                }

                cx.emit(Event::Wakeup);
            }
            InternalEvent::Scroll(scroll) => {
                term.scroll_display(*scroll);
                self.refresh_hovered_word(window);

                if self.vi_mode_enabled {
                    match *scroll {
                        AlacScroll::Delta(delta) => {
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(&term, delta);
                        }
                        AlacScroll::PageUp => {
                            let lines = term.screen_lines() as i32;
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(&term, lines);
                        }
                        AlacScroll::PageDown => {
                            let lines = -(term.screen_lines() as i32);
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(&term, lines);
                        }
                        AlacScroll::Top => {
                            let point = AlacPoint::new(term.topmost_line(), Column(0));
                            term.vi_mode_cursor = ViModeCursor::new(point);
                        }
                        AlacScroll::Bottom => {
                            let point = AlacPoint::new(term.bottommost_line(), Column(0));
                            term.vi_mode_cursor = ViModeCursor::new(point);
                        }
                    }
                    if let Some(mut selection) = term.selection.take() {
                        let point = term.vi_mode_cursor.point;
                        selection.update(point, AlacDirection::Right);
                        term.selection = Some(selection);

                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                        if let Some(selection_text) = term.selection_to_string() {
                            cx.write_to_primary(ClipboardItem::new_string(selection_text));
                        }

                        self.selection_head = Some(point);
                        cx.emit(Event::SelectionsChanged)
                    }
                }
            }
            InternalEvent::SetSelection(selection) => {
                term.selection = selection.as_ref().map(|(sel, _)| sel.clone());

                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                if let Some(selection_text) = term.selection_to_string() {
                    cx.write_to_primary(ClipboardItem::new_string(selection_text));
                }

                if let Some((_, head)) = selection {
                    self.selection_head = Some(*head);
                }
                cx.emit(Event::SelectionsChanged)
            }
            InternalEvent::UpdateSelection(position) => {
                if let Some(mut selection) = term.selection.take() {
                    let (point, side) = grid_point_and_side(
                        *position,
                        self.last_content.terminal_bounds,
                        term.grid().display_offset(),
                    );

                    selection.update(point, side);
                    term.selection = Some(selection);

                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                    if let Some(selection_text) = term.selection_to_string() {
                        cx.write_to_primary(ClipboardItem::new_string(selection_text));
                    }

                    self.selection_head = Some(point);
                    cx.emit(Event::SelectionsChanged)
                }
            }

            InternalEvent::Copy => {
                if let Some(txt) = term.selection_to_string() {
                    cx.write_to_clipboard(ClipboardItem::new_string(txt))
                }
            }
            InternalEvent::ScrollToAlacPoint(point) => {
                term.scroll_to_point(*point);
                self.refresh_hovered_word(window);
            }
            InternalEvent::ToggleViMode => {
                self.vi_mode_enabled = !self.vi_mode_enabled;
                term.toggle_vi_mode();
            }
            InternalEvent::ViMotion(motion) => {
                term.vi_motion(*motion);
            }
            InternalEvent::FindHyperlink(position, open) => {
                let prev_hovered_word = self.last_content.last_hovered_word.take();

                let point = grid_point(
                    *position,
                    self.last_content.terminal_bounds,
                    term.grid().display_offset(),
                )
                .grid_clamp(term, Boundary::Grid);

                let link = term.grid().index(point).hyperlink();
                let found_word = if link.is_some() {
                    let mut min_index = point;
                    loop {
                        let new_min_index = min_index.sub(term, Boundary::Cursor, 1);
                        if new_min_index == min_index
                            || term.grid().index(new_min_index).hyperlink() != link
                        {
                            break;
                        } else {
                            min_index = new_min_index
                        }
                    }

                    let mut max_index = point;
                    loop {
                        let new_max_index = max_index.add(term, Boundary::Cursor, 1);
                        if new_max_index == max_index
                            || term.grid().index(new_max_index).hyperlink() != link
                        {
                            break;
                        } else {
                            max_index = new_max_index
                        }
                    }

                    let url = link.unwrap().uri().to_owned();
                    let url_match = min_index..=max_index;

                    Some((url, true, url_match))
                } else if let Some(url_match) = regex_match_at(term, point, &mut self.url_regex) {
                    let url = term.bounds_to_string(*url_match.start(), *url_match.end());
                    Some((url, true, url_match))
                } else if let Some(python_match) =
                    regex_match_at(term, point, &mut self.python_file_line_regex)
                {
                    let matching_line =
                        term.bounds_to_string(*python_match.start(), *python_match.end());
                    python_extract_path_and_line(&matching_line).map(|(file_path, line_number)| {
                        (format!("{file_path}:{line_number}"), false, python_match)
                    })
                } else if let Some(word_match) = regex_match_at(term, point, &mut self.word_regex) {
                    let file_path = term.bounds_to_string(*word_match.start(), *word_match.end());

                    let (sanitized_match, sanitized_word) = 'sanitize: {
                        let mut word_match = word_match;
                        let mut file_path = file_path;

                        if is_path_surrounded_by_common_symbols(&file_path) {
                            word_match = Match::new(
                                word_match.start().add(term, Boundary::Grid, 1),
                                word_match.end().sub(term, Boundary::Grid, 1),
                            );
                            file_path = file_path[1..file_path.len() - 1].to_owned();
                        }

                        while file_path.ends_with(':') {
                            file_path.pop();
                            word_match = Match::new(
                                *word_match.start(),
                                word_match.end().sub(term, Boundary::Grid, 1),
                            );
                        }
                        let mut colon_count = 0;
                        for c in file_path.chars() {
                            if c == ':' {
                                colon_count += 1;
                            }
                        }
                        // strip trailing comment after colon in case of
                        // file/at/path.rs:row:column:description or error message
                        // so that the file path is `file/at/path.rs:row:column`
                        if colon_count > 2 {
                            let last_index = file_path.rfind(':').unwrap();
                            let prev_is_digit = last_index > 0
                                && file_path
                                    .chars()
                                    .nth(last_index - 1)
                                    .map_or(false, |c| c.is_ascii_digit());
                            let next_is_digit = last_index < file_path.len() - 1
                                && file_path
                                    .chars()
                                    .nth(last_index + 1)
                                    .map_or(true, |c| c.is_ascii_digit());
                            if prev_is_digit && !next_is_digit {
                                let stripped_len = file_path.len() - last_index;
                                word_match = Match::new(
                                    *word_match.start(),
                                    word_match.end().sub(term, Boundary::Grid, stripped_len),
                                );
                                file_path = file_path[0..last_index].to_owned();
                            }
                        }

                        break 'sanitize (word_match, file_path);
                    };

                    Some((sanitized_word, false, sanitized_match))
                } else {
                    None
                };

                match found_word {
                    Some((maybe_url_or_path, is_url, url_match)) => {
                        let target = if is_url {
                            // Treat "file://" URLs like file paths to ensure
                            // that line numbers at the end of the path are
                            // handled correctly
                            if let Some(path) = maybe_url_or_path.strip_prefix("file://") {
                                MaybeNavigationTarget::PathLike(PathLikeTarget {
                                    maybe_path: path.to_string(),
                                    terminal_dir: self.working_directory(),
                                })
                            } else {
                                MaybeNavigationTarget::Url(maybe_url_or_path.clone())
                            }
                        } else {
                            MaybeNavigationTarget::PathLike(PathLikeTarget {
                                maybe_path: maybe_url_or_path.clone(),
                                terminal_dir: self.working_directory(),
                            })
                        };
                        if *open {
                            cx.emit(Event::Open(target));
                        } else {
                            self.update_selected_word(
                                prev_hovered_word,
                                url_match,
                                maybe_url_or_path,
                                target,
                                cx,
                            );
                        }
                    }
                    None => {
                        cx.emit(Event::NewNavigationTarget(None));
                    }
                }
            }
        }
    }

    fn update_selected_word(
        &mut self,
        prev_word: Option<HoveredWord>,
        word_match: RangeInclusive<AlacPoint>,
        word: String,
        navigation_target: MaybeNavigationTarget,
        cx: &mut Context<Self>,
    ) {
        if let Some(prev_word) = prev_word {
            if prev_word.word == word && prev_word.word_match == word_match {
                self.last_content.last_hovered_word = Some(HoveredWord {
                    word,
                    word_match,
                    id: prev_word.id,
                });
                return;
            }
        }

        self.last_content.last_hovered_word = Some(HoveredWord {
            word: word.clone(),
            word_match,
            id: self.next_link_id(),
        });
        cx.emit(Event::NewNavigationTarget(Some(navigation_target)));
        cx.notify()
    }

    fn next_link_id(&mut self) -> usize {
        let res = self.next_link_id;
        self.next_link_id = self.next_link_id.wrapping_add(1);
        res
    }

    pub fn last_content(&self) -> &TerminalContent {
        &self.last_content
    }

    pub fn set_cursor_shape(&mut self, cursor_shape: CursorShape) {
        self.term_config.default_cursor_style = cursor_shape.into();
        self.term.lock().set_options(self.term_config.clone());
    }

    pub fn total_lines(&self) -> usize {
        let term = self.term.clone();
        let terminal = term.lock_unfair();
        terminal.total_lines()
    }

    pub fn viewport_lines(&self) -> usize {
        let term = self.term.clone();
        let terminal = term.lock_unfair();
        terminal.screen_lines()
    }

    //To test:
    //- Activate match on terminal (scrolling and selection)
    //- Editor search snapping behavior

    pub fn activate_match(&mut self, index: usize) {
        if let Some(search_match) = self.matches.get(index).cloned() {
            self.set_selection(Some((make_selection(&search_match), *search_match.end())));

            self.events
                .push_back(InternalEvent::ScrollToAlacPoint(*search_match.start()));
        }
    }

    pub fn select_matches(&mut self, matches: &[RangeInclusive<AlacPoint>]) {
        let matches_to_select = self
            .matches
            .iter()
            .filter(|self_match| matches.contains(self_match))
            .cloned()
            .collect::<Vec<_>>();
        for match_to_select in matches_to_select {
            self.set_selection(Some((
                make_selection(&match_to_select),
                *match_to_select.end(),
            )));
        }
    }

    pub fn select_all(&mut self) {
        let term = self.term.lock();
        let start = AlacPoint::new(term.topmost_line(), Column(0));
        let end = AlacPoint::new(term.bottommost_line(), term.last_column());
        drop(term);
        self.set_selection(Some((make_selection(&(start..=end)), end)));
    }

    fn set_selection(&mut self, selection: Option<(Selection, AlacPoint)>) {
        self.events
            .push_back(InternalEvent::SetSelection(selection));
    }

    pub fn copy(&mut self) {
        self.events.push_back(InternalEvent::Copy);
    }

    pub fn clear(&mut self) {
        self.events.push_back(InternalEvent::Clear)
    }

    pub fn scroll_line_up(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Delta(1)));
    }

    pub fn scroll_up_by(&mut self, lines: usize) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Delta(lines as i32)));
    }

    pub fn scroll_line_down(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Delta(-1)));
    }

    pub fn scroll_down_by(&mut self, lines: usize) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Delta(-(lines as i32))));
    }

    pub fn scroll_page_up(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::PageUp));
    }

    pub fn scroll_page_down(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::PageDown));
    }

    pub fn scroll_to_top(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Top));
    }

    pub fn scroll_to_bottom(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Bottom));
    }

    ///Resize the terminal and the PTY.
    pub fn set_size(&mut self, new_bounds: TerminalBounds) {
        if self.last_content.terminal_bounds != new_bounds {
            self.events.push_back(InternalEvent::Resize(new_bounds))
        }
    }

    ///Write the Input payload to the tty.
    fn write_to_pty(&self, input: String) {
        self.pty_tx.notify(input.into_bytes());
    }

    fn write_bytes_to_pty(&self, input: Vec<u8>) {
        self.pty_tx.notify(input);
    }

    pub fn input(&mut self, input: String) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Bottom));
        self.events.push_back(InternalEvent::SetSelection(None));

        self.write_to_pty(input);
    }

    pub fn input_bytes(&mut self, input: Vec<u8>) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Bottom));
        self.events.push_back(InternalEvent::SetSelection(None));

        self.write_bytes_to_pty(input);
    }

    pub fn toggle_vi_mode(&mut self) {
        self.events.push_back(InternalEvent::ToggleViMode);
    }

    pub fn vi_motion(&mut self, keystroke: &Keystroke, keyboard_mapper: &dyn KeyboardMapper) {
        if !self.vi_mode_enabled {
            return;
        }

        let keystroke = keyboard_mapper.to_vim_keystroke(keystroke);
        let key = keystroke.key.as_str();
        let motion: Option<ViMotion> = match key {
            "h" | "left" => Some(ViMotion::Left),
            "j" | "down" => Some(ViMotion::Down),
            "k" | "up" => Some(ViMotion::Up),
            "l" | "right" => Some(ViMotion::Right),
            "w" => Some(ViMotion::WordRight),
            "b" if !keystroke.modifiers.control => Some(ViMotion::WordLeft),
            "e" => Some(ViMotion::WordRightEnd),
            "%" => Some(ViMotion::Bracket),
            "$" => Some(ViMotion::Last),
            "0" => Some(ViMotion::First),
            "^" => Some(ViMotion::FirstOccupied),
            "H" => Some(ViMotion::High),
            "M" => Some(ViMotion::Middle),
            "L" => Some(ViMotion::Low),
            _ => None,
        };

        if let Some(motion) = motion {
            let cursor = self.last_content.cursor.point;
            let cursor_pos = Point {
                x: cursor.column.0 as f32 * self.last_content.terminal_bounds.cell_width,
                y: cursor.line.0 as f32 * self.last_content.terminal_bounds.line_height,
            };
            self.events
                .push_back(InternalEvent::UpdateSelection(cursor_pos));
            self.events.push_back(InternalEvent::ViMotion(motion));
            return;
        }

        let scroll_motion = match key {
            "g" => Some(AlacScroll::Top),
            "G" => Some(AlacScroll::Bottom),
            "b" if keystroke.modifiers.control => Some(AlacScroll::PageUp),
            "f" if keystroke.modifiers.control => Some(AlacScroll::PageDown),
            "d" if keystroke.modifiers.control => {
                let amount = self.last_content.terminal_bounds.line_height().to_f64() as i32 / 2;
                Some(AlacScroll::Delta(-amount))
            }
            "u" if keystroke.modifiers.control => {
                let amount = self.last_content.terminal_bounds.line_height().to_f64() as i32 / 2;
                Some(AlacScroll::Delta(amount))
            }
            _ => None,
        };

        if let Some(scroll_motion) = scroll_motion {
            self.events.push_back(InternalEvent::Scroll(scroll_motion));
            return;
        }

        match key {
            "v" => {
                let point = self.last_content.cursor.point;
                let selection_type = SelectionType::Simple;
                let side = AlacDirection::Right;
                let selection = Selection::new(selection_type, point, side);
                self.events
                    .push_back(InternalEvent::SetSelection(Some((selection, point))));
                return;
            }

            "escape" => {
                self.events.push_back(InternalEvent::SetSelection(None));
                return;
            }

            "y" => {
                self.events.push_back(InternalEvent::Copy);
                self.events.push_back(InternalEvent::SetSelection(None));
                return;
            }

            "i" => {
                self.scroll_to_bottom();
                self.toggle_vi_mode();
                return;
            }
            _ => {}
        }
    }

    pub fn try_keystroke(
        &mut self,
        keystroke: &Keystroke,
        alt_is_meta: bool,
        mapper: &dyn KeyboardMapper,
    ) -> bool {
        if self.vi_mode_enabled {
            self.vi_motion(keystroke, mapper);
            return true;
        }

        // Keep default terminal behavior
        let esc = to_esc_str(keystroke, &self.last_content.mode, alt_is_meta, mapper);
        if let Some(esc) = esc {
            self.input(esc);
            true
        } else {
            false
        }
    }

    pub fn try_modifiers_change(
        &mut self,
        modifiers: &Modifiers,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .last_content
            .terminal_bounds
            .bounds
            .contains(&window.mouse_position())
            && modifiers.secondary()
        {
            self.refresh_hovered_word(window);
        }
        cx.notify();
    }

    ///Paste text into the terminal
    pub fn paste(&mut self, text: &str) {
        let paste_text = if self.last_content.mode.contains(TermMode::BRACKETED_PASTE) {
            format!("{}{}{}", "\x1b[200~", text.replace('\x1b', ""), "\x1b[201~")
        } else {
            text.replace("\r\n", "\r").replace('\n', "\r")
        };

        self.input(paste_text);
    }

    pub fn sync(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let term = self.term.clone();
        let mut terminal = term.lock_unfair();
        //Note that the ordering of events matters for event processing
        while let Some(e) = self.events.pop_front() {
            self.process_terminal_event(&e, &mut terminal, window, cx)
        }

        self.last_content = Self::make_content(&terminal, &self.last_content);
    }

    fn make_content(term: &Term<ZedListener>, last_content: &TerminalContent) -> TerminalContent {
        let content = term.renderable_content();
        TerminalContent {
            cells: content
                .display_iter
                //TODO: Add this once there's a way to retain empty lines
                // .filter(|ic| {
                //     !ic.flags.contains(Flags::HIDDEN)
                //         && !(ic.bg == Named(NamedColor::Background)
                //             && ic.c == ' '
                //             && !ic.flags.contains(Flags::INVERSE))
                // })
                .map(|ic| IndexedCell {
                    point: ic.point,
                    cell: ic.cell.clone(),
                })
                .collect::<Vec<IndexedCell>>(),
            mode: content.mode,
            display_offset: content.display_offset,
            selection_text: term.selection_to_string(),
            selection: content.selection,
            cursor: content.cursor,
            cursor_char: term.grid()[content.cursor.point].c,
            terminal_bounds: last_content.terminal_bounds,
            last_hovered_word: last_content.last_hovered_word.clone(),
        }
    }

    pub fn last_n_non_empty_lines(&self, n: usize) -> Vec<String> {
        let term = self.term.clone();
        let terminal = term.lock_unfair();
        let grid = terminal.grid();
        let mut lines = Vec::new();

        let mut current_line = grid.bottommost_line().0;
        let topmost_line = grid.topmost_line().0;

        while current_line >= topmost_line && lines.len() < n {
            let logical_line_start = self.find_logical_line_start(grid, current_line, topmost_line);
            let logical_line = self.construct_logical_line(grid, logical_line_start, current_line);

            if let Some(line) = self.process_line(logical_line) {
                lines.push(line);
            }

            // Move to the line above the start of the current logical line
            current_line = logical_line_start - 1;
        }

        lines.reverse();
        lines
    }

    fn find_logical_line_start(&self, grid: &Grid<Cell>, current: i32, topmost: i32) -> i32 {
        let mut line_start = current;
        while line_start > topmost {
            let prev_line = Line(line_start - 1);
            let last_cell = &grid[prev_line][Column(grid.columns() - 1)];
            if !last_cell.flags.contains(Flags::WRAPLINE) {
                break;
            }
            line_start -= 1;
        }
        line_start
    }

    fn construct_logical_line(&self, grid: &Grid<Cell>, start: i32, end: i32) -> String {
        let mut logical_line = String::new();
        for row in start..=end {
            let grid_row = &grid[Line(row)];
            logical_line.push_str(&row_to_string(grid_row));
        }
        logical_line
    }

    fn process_line(&self, line: String) -> Option<String> {
        let trimmed = line.trim_end().to_string();
        if !trimmed.is_empty() {
            Some(trimmed)
        } else {
            None
        }
    }

    pub fn focus_in(&self) {
        if self.last_content.mode.contains(TermMode::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[I".to_string());
        }
    }

    pub fn focus_out(&mut self) {
        if self.last_content.mode.contains(TermMode::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[O".to_string());
        }
    }

    pub fn mouse_changed(&mut self, point: AlacPoint, side: AlacDirection) -> bool {
        match self.last_mouse {
            Some((old_point, old_side)) => {
                if old_point == point && old_side == side {
                    false
                } else {
                    self.last_mouse = Some((point, side));
                    true
                }
            }
            None => {
                self.last_mouse = Some((point, side));
                true
            }
        }
    }

    pub fn mouse_mode(&self, shift: bool) -> bool {
        self.last_content.mode.intersects(TermMode::MOUSE_MODE) && !shift
    }

    pub fn mouse_move(&mut self, e: &MouseMoveEvent, cx: &mut Context<Self>) {
        let position = e.position - self.last_content.terminal_bounds.bounds.origin;
        if self.mouse_mode(e.modifiers.shift) {
            let (point, side) = grid_point_and_side(
                position,
                self.last_content.terminal_bounds,
                self.last_content.display_offset,
            );

            if self.mouse_changed(point, side) {
                if let Some(bytes) =
                    mouse_moved_report(point, e.pressed_button, e.modifiers, self.last_content.mode)
                {
                    self.pty_tx.notify(bytes);
                }
            }
        } else if e.modifiers.secondary() {
            self.word_from_position(e.position);
        }
        cx.notify();
    }

    fn word_from_position(&mut self, position: Point<Pixels>) {
        if self.selection_phase == SelectionPhase::Selecting {
            self.last_content.last_hovered_word = None;
        } else if self.last_content.terminal_bounds.bounds.contains(&position) {
            self.events.push_back(InternalEvent::FindHyperlink(
                position - self.last_content.terminal_bounds.bounds.origin,
                false,
            ));
        } else {
            self.last_content.last_hovered_word = None;
        }
    }

    pub fn select_word_at_event_position(&mut self, e: &MouseDownEvent) {
        let position = e.position - self.last_content.terminal_bounds.bounds.origin;
        let (point, side) = grid_point_and_side(
            position,
            self.last_content.terminal_bounds,
            self.last_content.display_offset,
        );
        let selection = Selection::new(SelectionType::Semantic, point, side);
        self.events
            .push_back(InternalEvent::SetSelection(Some((selection, point))));
    }

    pub fn mouse_drag(
        &mut self,
        e: &MouseMoveEvent,
        region: Bounds<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let position = e.position - self.last_content.terminal_bounds.bounds.origin;
        if !self.mouse_mode(e.modifiers.shift) {
            self.selection_phase = SelectionPhase::Selecting;
            // Alacritty has the same ordering, of first updating the selection
            // then scrolling 15ms later
            self.events
                .push_back(InternalEvent::UpdateSelection(position));

            // Doesn't make sense to scroll the alt screen
            if !self.last_content.mode.contains(TermMode::ALT_SCREEN) {
                let scroll_lines = match self.drag_line_delta(e, region) {
                    Some(value) => value,
                    None => return,
                };

                self.events
                    .push_back(InternalEvent::Scroll(AlacScroll::Delta(scroll_lines)));
            }

            cx.notify();
        }
    }

    fn drag_line_delta(&self, e: &MouseMoveEvent, region: Bounds<Pixels>) -> Option<i32> {
        let top = region.origin.y;
        let bottom = region.bottom_left().y;

        let scroll_lines = if e.position.y < top {
            let scroll_delta = (top - e.position.y).pow(1.1);
            (scroll_delta / self.last_content.terminal_bounds.line_height).ceil() as i32
        } else if e.position.y > bottom {
            let scroll_delta = -((e.position.y - bottom).pow(1.1));
            (scroll_delta / self.last_content.terminal_bounds.line_height).floor() as i32
        } else {
            return None;
        };

        Some(scroll_lines.clamp(-3, 3))
    }

    pub fn mouse_down(&mut self, e: &MouseDownEvent, _cx: &mut Context<Self>) {
        let position = e.position - self.last_content.terminal_bounds.bounds.origin;
        let point = grid_point(
            position,
            self.last_content.terminal_bounds,
            self.last_content.display_offset,
        );

        if self.mouse_mode(e.modifiers.shift) {
            if let Some(bytes) =
                mouse_button_report(point, e.button, e.modifiers, true, self.last_content.mode)
            {
                self.pty_tx.notify(bytes);
            }
        } else {
            match e.button {
                MouseButton::Left => {
                    let (point, side) = grid_point_and_side(
                        position,
                        self.last_content.terminal_bounds,
                        self.last_content.display_offset,
                    );

                    let selection_type = match e.click_count {
                        0 => return, //This is a release
                        1 => Some(SelectionType::Simple),
                        2 => Some(SelectionType::Semantic),
                        3 => Some(SelectionType::Lines),
                        _ => None,
                    };

                    if selection_type == Some(SelectionType::Simple) && e.modifiers.shift {
                        self.events
                            .push_back(InternalEvent::UpdateSelection(position));
                        return;
                    }

                    let selection = selection_type
                        .map(|selection_type| Selection::new(selection_type, point, side));

                    if let Some(sel) = selection {
                        self.events
                            .push_back(InternalEvent::SetSelection(Some((sel, point))));
                    }
                }
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                MouseButton::Middle => {
                    if let Some(item) = _cx.read_from_primary() {
                        let text = item.text().unwrap_or_default().to_string();
                        self.input(text);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn mouse_up(&mut self, e: &MouseUpEvent, cx: &Context<Self>) {
        let setting = TerminalSettings::get_global(cx);

        let position = e.position - self.last_content.terminal_bounds.bounds.origin;
        if self.mouse_mode(e.modifiers.shift) {
            let point = grid_point(
                position,
                self.last_content.terminal_bounds,
                self.last_content.display_offset,
            );

            if let Some(bytes) =
                mouse_button_report(point, e.button, e.modifiers, false, self.last_content.mode)
            {
                self.pty_tx.notify(bytes);
            }
        } else {
            if e.button == MouseButton::Left && setting.copy_on_select {
                self.copy();
            }

            //Hyperlinks
            if self.selection_phase == SelectionPhase::Ended {
                let mouse_cell_index =
                    content_index_for_mouse(position, &self.last_content.terminal_bounds);
                if let Some(link) = self.last_content.cells[mouse_cell_index].hyperlink() {
                    cx.open_url(link.uri());
                } else if e.modifiers.secondary() {
                    self.events
                        .push_back(InternalEvent::FindHyperlink(position, true));
                }
            }
        }

        self.selection_phase = SelectionPhase::Ended;
        self.last_mouse = None;
    }

    ///Scroll the terminal
    pub fn scroll_wheel(&mut self, e: &ScrollWheelEvent) {
        let mouse_mode = self.mouse_mode(e.shift);

        if let Some(scroll_lines) = self.determine_scroll_lines(e, mouse_mode) {
            if mouse_mode {
                let point = grid_point(
                    e.position - self.last_content.terminal_bounds.bounds.origin,
                    self.last_content.terminal_bounds,
                    self.last_content.display_offset,
                );

                if let Some(scrolls) = scroll_report(point, scroll_lines, e, self.last_content.mode)
                {
                    for scroll in scrolls {
                        self.pty_tx.notify(scroll);
                    }
                };
            } else if self
                .last_content
                .mode
                .contains(TermMode::ALT_SCREEN | TermMode::ALTERNATE_SCROLL)
                && !e.shift
            {
                self.pty_tx.notify(alt_scroll(scroll_lines))
            } else if scroll_lines != 0 {
                let scroll = AlacScroll::Delta(scroll_lines);

                self.events.push_back(InternalEvent::Scroll(scroll));
            }
        }
    }

    fn refresh_hovered_word(&mut self, window: &Window) {
        self.word_from_position(window.mouse_position());
    }

    fn determine_scroll_lines(&mut self, e: &ScrollWheelEvent, mouse_mode: bool) -> Option<i32> {
        let scroll_multiplier = if mouse_mode { 1. } else { SCROLL_MULTIPLIER };
        let line_height = self.last_content.terminal_bounds.line_height;
        match e.touch_phase {
            /* Reset scroll state on started */
            TouchPhase::Started => {
                self.scroll_px = px(0.);
                None
            }
            /* Calculate the appropriate scroll lines */
            TouchPhase::Moved => {
                let old_offset = (self.scroll_px / line_height) as i32;

                self.scroll_px += e.delta.pixel_delta(line_height).y * scroll_multiplier;

                let new_offset = (self.scroll_px / line_height) as i32;

                // Whenever we hit the edges, reset our stored scroll to 0
                // so we can respond to changes in direction quickly
                self.scroll_px %= self.last_content.terminal_bounds.height();

                Some(new_offset - old_offset)
            }
            TouchPhase::Ended => None,
        }
    }

    pub fn find_matches(
        &self,
        mut searcher: RegexSearch,
        cx: &Context<Self>,
    ) -> Task<Vec<RangeInclusive<AlacPoint>>> {
        let term = self.term.clone();
        cx.background_spawn(async move {
            let term = term.lock();

            all_search_matches(&term, &mut searcher).collect()
        })
    }

    pub fn working_directory(&self) -> Option<PathBuf> {
        if self.is_ssh_terminal {
            // We can't yet reliably detect the working directory of a shell on the
            // SSH host. Until we can do that, it doesn't make sense to display
            // the working directory on the client and persist that.
            None
        } else {
            self.client_side_working_directory()
        }
    }

    /// Returns the working directory of the process that's connected to the PTY.
    /// That means it returns the working directory of the local shell or program
    /// that's running inside the terminal.
    ///
    /// This does *not* return the working directory of the shell that runs on the
    /// remote host, in case Zed is connected to a remote host.
    fn client_side_working_directory(&self) -> Option<PathBuf> {
        self.pty_info
            .current
            .as_ref()
            .map(|process| process.cwd.clone())
    }

    pub fn title(&self, truncate: bool) -> String {
        const MAX_CHARS: usize = 25;
        match &self.task {
            Some(task_state) => {
                if truncate {
                    truncate_and_trailoff(&task_state.label, MAX_CHARS)
                } else {
                    task_state.full_label.clone()
                }
            }
            None => self
                .title_override
                .as_ref()
                .map(|title_override| title_override.to_string())
                .unwrap_or_else(|| {
                    self.pty_info
                        .current
                        .as_ref()
                        .map(|fpi| {
                            let process_file = fpi
                                .cwd
                                .file_name()
                                .map(|name| name.to_string_lossy().to_string())
                                .unwrap_or_default();

                            let argv = fpi.argv.clone();
                            let process_name = format!(
                                "{}{}",
                                fpi.name,
                                if !argv.is_empty() {
                                    format!(" {}", (argv[1..]).join(" "))
                                } else {
                                    "".to_string()
                                }
                            );
                            let (process_file, process_name) = if truncate {
                                (
                                    truncate_and_trailoff(&process_file, MAX_CHARS),
                                    truncate_and_trailoff(&process_name, MAX_CHARS),
                                )
                            } else {
                                (process_file, process_name)
                            };
                            format!("{process_file} — {process_name}")
                        })
                        .unwrap_or_else(|| "Terminal".to_string())
                }),
        }
    }

    pub fn task(&self) -> Option<&TaskState> {
        self.task.as_ref()
    }

    pub fn wait_for_completed_task(&self, cx: &App) -> Task<Option<ExitStatus>> {
        if let Some(task) = self.task() {
            if task.status == TaskStatus::Running {
                let completion_receiver = task.completion_rx.clone();
                return cx
                    .spawn(async move |_| completion_receiver.recv().await.log_err().flatten());
            }
        }
        Task::ready(None)
    }

    fn register_task_finished(&mut self, error_code: Option<i32>, cx: &mut Context<Terminal>) {
        let e: Option<ExitStatus> = error_code.map(|code| {
            #[cfg(unix)]
            {
                return std::os::unix::process::ExitStatusExt::from_raw(code);
            }
            #[cfg(windows)]
            {
                return std::os::windows::process::ExitStatusExt::from_raw(code as u32);
            }
        });

        self.completion_tx.try_send(e).ok();
        let task = match &mut self.task {
            Some(task) => task,
            None => {
                if error_code.is_none() {
                    cx.emit(Event::CloseTerminal);
                }
                return;
            }
        };
        if task.status != TaskStatus::Running {
            return;
        }
        match error_code {
            Some(error_code) => {
                task.status.register_task_exit(error_code);
            }
            None => {
                task.status.register_terminal_exit();
            }
        };

        let (finished_successfully, task_line, command_line) = task_summary(task, error_code);
        let mut lines_to_show = Vec::new();
        if task.show_summary {
            lines_to_show.push(task_line.as_str());
        }
        if task.show_command {
            lines_to_show.push(command_line.as_str());
        }

        if !lines_to_show.is_empty() {
            // SAFETY: the invocation happens on non `TaskStatus::Running` tasks, once,
            // after either `AlacTermEvent::Exit` or `AlacTermEvent::ChildExit` events that are spawned
            // when Zed task finishes and no more output is made.
            // After the task summary is output once, no more text is appended to the terminal.
            unsafe { append_text_to_term(&mut self.term.lock(), &lines_to_show) };
        }

        match task.hide {
            HideStrategy::Never => {}
            HideStrategy::Always => {
                cx.emit(Event::CloseTerminal);
            }
            HideStrategy::OnSuccess => {
                if finished_successfully {
                    cx.emit(Event::CloseTerminal);
                }
            }
        }
    }

    pub fn vi_mode_enabled(&self) -> bool {
        self.vi_mode_enabled
    }
}

// Helper function to convert a grid row to a string
pub fn row_to_string(row: &Row<Cell>) -> String {
    row[..Column(row.len())]
        .iter()
        .map(|cell| cell.c)
        .collect::<String>()
}

fn is_path_surrounded_by_common_symbols(path: &str) -> bool {
    // Avoid detecting `[]` or `()` strings as paths, surrounded by common symbols
    path.len() > 2
        // The rest of the brackets and various quotes cannot be matched by the [`WORD_REGEX`] hence not checked for.
        && (path.starts_with('[') && path.ends_with(']')
            || path.starts_with('(') && path.ends_with(')'))
}

const TASK_DELIMITER: &str = "⏵ ";
fn task_summary(task: &TaskState, error_code: Option<i32>) -> (bool, String, String) {
    let escaped_full_label = task.full_label.replace("\r\n", "\r").replace('\n', "\r");
    let (success, task_line) = match error_code {
        Some(0) => (
            true,
            format!("{TASK_DELIMITER}Task `{escaped_full_label}` finished successfully"),
        ),
        Some(error_code) => (
            false,
            format!(
                "{TASK_DELIMITER}Task `{escaped_full_label}` finished with non-zero error code: {error_code}"
            ),
        ),
        None => (
            false,
            format!("{TASK_DELIMITER}Task `{escaped_full_label}` finished"),
        ),
    };
    let escaped_command_label = task.command_label.replace("\r\n", "\r").replace('\n', "\r");
    let command_line = format!("{TASK_DELIMITER}Command: {escaped_command_label}");
    (success, task_line, command_line)
}

/// Appends a stringified task summary to the terminal, after its output.
///
/// SAFETY: This function should only be called after terminal's PTY is no longer alive.
/// New text being added to the terminal here, uses "less public" APIs,
/// which are not maintaining the entire terminal state intact.
///
///
/// The library
///
/// * does not increment inner grid cursor's _lines_ on `input` calls
///   (but displaying the lines correctly and incrementing cursor's columns)
///
/// * ignores `\n` and \r` character input, requiring the `newline` call instead
///
/// * does not alter grid state after `newline` call
///   so its `bottommost_line` is always the same additions, and
///   the cursor's `point` is not updated to the new line and column values
///
/// * ??? there could be more consequences, and any further "proper" streaming from the PTY might bug and/or panic.
///   Still, subsequent `append_text_to_term` invocations are possible and display the contents correctly.
///
/// Despite the quirks, this is the simplest approach to appending text to the terminal: its alternative, `grid_mut` manipulations,
/// do not properly set the scrolling state and display odd text after appending; also those manipulations are more tedious and error-prone.
/// The function achieves proper display and scrolling capabilities, at a cost of grid state not properly synchronized.
/// This is enough for printing moderately-sized texts like task summaries, but might break or perform poorly for larger texts.
unsafe fn append_text_to_term(term: &mut Term<ZedListener>, text_lines: &[&str]) {
    term.newline();
    term.grid_mut().cursor.point.column = Column(0);
    for line in text_lines {
        for c in line.chars() {
            term.input(c);
        }
        term.newline();
        term.grid_mut().cursor.point.column = Column(0);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.pty_tx.0.send(Msg::Shutdown).ok();
    }
}

impl EventEmitter<Event> for Terminal {}

/// Based on alacritty/src/display/hint.rs > regex_match_at
/// Retrieve the match, if the specified point is inside the content matching the regex.
fn regex_match_at<T>(term: &Term<T>, point: AlacPoint, regex: &mut RegexSearch) -> Option<Match> {
    visible_regex_match_iter(term, regex).find(|rm| rm.contains(&point))
}

/// Copied from alacritty/src/display/hint.rs:
/// Iterate over all visible regex matches.
pub fn visible_regex_match_iter<'a, T>(
    term: &'a Term<T>,
    regex: &'a mut RegexSearch,
) -> impl Iterator<Item = Match> + 'a {
    let viewport_start = Line(-(term.grid().display_offset() as i32));
    let viewport_end = viewport_start + term.bottommost_line();
    let mut start = term.line_search_left(AlacPoint::new(viewport_start, Column(0)));
    let mut end = term.line_search_right(AlacPoint::new(viewport_end, Column(0)));
    start.line = start.line.max(viewport_start - MAX_SEARCH_LINES);
    end.line = end.line.min(viewport_end + MAX_SEARCH_LINES);

    RegexIter::new(start, end, AlacDirection::Right, term, regex)
        .skip_while(move |rm| rm.end().line < viewport_start)
        .take_while(move |rm| rm.start().line <= viewport_end)
}

fn make_selection(range: &RangeInclusive<AlacPoint>) -> Selection {
    let mut selection = Selection::new(SelectionType::Simple, *range.start(), AlacDirection::Left);
    selection.update(*range.end(), AlacDirection::Right);
    selection
}

fn all_search_matches<'a, T>(
    term: &'a Term<T>,
    regex: &'a mut RegexSearch,
) -> impl Iterator<Item = Match> + 'a {
    let start = AlacPoint::new(term.grid().topmost_line(), Column(0));
    let end = AlacPoint::new(term.grid().bottommost_line(), term.grid().last_column());
    RegexIter::new(start, end, AlacDirection::Right, term, regex)
}

fn content_index_for_mouse(pos: Point<Pixels>, terminal_bounds: &TerminalBounds) -> usize {
    let col = (pos.x / terminal_bounds.cell_width()).round() as usize;
    let clamped_col = min(col, terminal_bounds.columns() - 1);
    let row = (pos.y / terminal_bounds.line_height()).round() as usize;
    let clamped_row = min(row, terminal_bounds.screen_lines() - 1);
    clamped_row * terminal_bounds.columns() + clamped_col
}

/// Converts an 8 bit ANSI color to its GPUI equivalent.
/// Accepts `usize` for compatibility with the `alacritty::Colors` interface,
/// Other than that use case, should only be called with values in the `[0,255]` range
pub fn get_color_at_index(index: usize, theme: &Theme) -> Hsla {
    let colors = theme.colors();

    match index {
        // 0-15 are the same as the named colors above
        0 => colors.terminal_ansi_black,
        1 => colors.terminal_ansi_red,
        2 => colors.terminal_ansi_green,
        3 => colors.terminal_ansi_yellow,
        4 => colors.terminal_ansi_blue,
        5 => colors.terminal_ansi_magenta,
        6 => colors.terminal_ansi_cyan,
        7 => colors.terminal_ansi_white,
        8 => colors.terminal_ansi_bright_black,
        9 => colors.terminal_ansi_bright_red,
        10 => colors.terminal_ansi_bright_green,
        11 => colors.terminal_ansi_bright_yellow,
        12 => colors.terminal_ansi_bright_blue,
        13 => colors.terminal_ansi_bright_magenta,
        14 => colors.terminal_ansi_bright_cyan,
        15 => colors.terminal_ansi_bright_white,
        // 16-231 are mapped to their RGB colors on a 0-5 range per channel
        16..=231 => {
            let (r, g, b) = rgb_for_index(index as u8); // Split the index into its ANSI-RGB components
            let step = (u8::MAX as f32 / 5.).floor() as u8; // Split the RGB range into 5 chunks, with floor so no overflow
            rgba_color(r * step, g * step, b * step) // Map the ANSI-RGB components to an RGB color
        }
        // 232-255 are a 24 step grayscale from black to white
        232..=255 => {
            let i = index as u8 - 232; // Align index to 0..24
            let step = (u8::MAX as f32 / 24.).floor() as u8; // Split the RGB grayscale values into 24 chunks
            rgba_color(i * step, i * step, i * step) // Map the ANSI-grayscale components to the RGB-grayscale
        }
        // For compatibility with the alacritty::Colors interface
        // See: https://github.com/alacritty/alacritty/blob/master/alacritty_terminal/src/term/color.rs
        256 => colors.terminal_foreground,
        257 => colors.terminal_background,
        258 => theme.players().local().cursor,
        259 => colors.terminal_ansi_dim_black,
        260 => colors.terminal_ansi_dim_red,
        261 => colors.terminal_ansi_dim_green,
        262 => colors.terminal_ansi_dim_yellow,
        263 => colors.terminal_ansi_dim_blue,
        264 => colors.terminal_ansi_dim_magenta,
        265 => colors.terminal_ansi_dim_cyan,
        266 => colors.terminal_ansi_dim_white,
        267 => colors.terminal_bright_foreground,
        268 => colors.terminal_ansi_black, // 'Dim Background', non-standard color

        _ => black(),
    }
}

/// Generates the RGB channels in [0, 5] for a given index into the 6x6x6 ANSI color cube.
/// See: [8 bit ANSI color](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit).
///
/// Wikipedia gives a formula for calculating the index for a given color:
///
/// ```
/// index = 16 + 36 × r + 6 × g + b (0 ≤ r, g, b ≤ 5)
/// ```
///
/// This function does the reverse, calculating the `r`, `g`, and `b` components from a given index.
fn rgb_for_index(i: u8) -> (u8, u8, u8) {
    debug_assert!((16..=231).contains(&i));
    let i = i - 16;
    let r = (i - (i % 36)) / 36;
    let g = ((i % 36) - (i % 6)) / 6;
    let b = (i % 36) % 6;
    (r, g, b)
}

pub fn rgba_color(r: u8, g: u8, b: u8) -> Hsla {
    Rgba {
        r: (r as f32 / 255.),
        g: (g as f32 / 255.),
        b: (b as f32 / 255.),
        a: 1.,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use alacritty_terminal::{
        index::{Column, Line, Point as AlacPoint},
        term::cell::Cell,
    };
    use gpui::{Pixels, Point, bounds, point, size};
    use rand::{Rng, distributions::Alphanumeric, rngs::ThreadRng, thread_rng};

    use crate::{
        IndexedCell, TerminalBounds, TerminalContent, content_index_for_mouse,
        python_extract_path_and_line, rgb_for_index,
    };

    #[test]
    fn test_rgb_for_index() {
        // Test every possible value in the color cube.
        for i in 16..=231 {
            let (r, g, b) = rgb_for_index(i);
            assert_eq!(i, 16 + 36 * r + 6 * g + b);
        }
    }

    #[test]
    fn test_mouse_to_cell_test() {
        let mut rng = thread_rng();
        const ITERATIONS: usize = 10;
        const PRECISION: usize = 1000;

        for _ in 0..ITERATIONS {
            let viewport_cells = rng.gen_range(15..20);
            let cell_size = rng.gen_range(5 * PRECISION..20 * PRECISION) as f32 / PRECISION as f32;

            let size = crate::TerminalBounds {
                cell_width: Pixels::from(cell_size),
                line_height: Pixels::from(cell_size),
                bounds: bounds(
                    Point::default(),
                    size(
                        Pixels::from(cell_size * (viewport_cells as f32)),
                        Pixels::from(cell_size * (viewport_cells as f32)),
                    ),
                ),
            };

            let cells = get_cells(size, &mut rng);
            let content = convert_cells_to_content(size, &cells);

            for row in 0..(viewport_cells - 1) {
                let row = row as usize;
                for col in 0..(viewport_cells - 1) {
                    let col = col as usize;

                    let row_offset = rng.gen_range(0..PRECISION) as f32 / PRECISION as f32;
                    let col_offset = rng.gen_range(0..PRECISION) as f32 / PRECISION as f32;

                    let mouse_pos = point(
                        Pixels::from(col as f32 * cell_size + col_offset),
                        Pixels::from(row as f32 * cell_size + row_offset),
                    );

                    let content_index =
                        content_index_for_mouse(mouse_pos, &content.terminal_bounds);
                    let mouse_cell = content.cells[content_index].c;
                    let real_cell = cells[row][col];

                    assert_eq!(mouse_cell, real_cell);
                }
            }
        }
    }

    #[test]
    fn test_mouse_to_cell_clamp() {
        let mut rng = thread_rng();

        let size = crate::TerminalBounds {
            cell_width: Pixels::from(10.),
            line_height: Pixels::from(10.),
            bounds: bounds(
                Point::default(),
                size(Pixels::from(100.), Pixels::from(100.)),
            ),
        };

        let cells = get_cells(size, &mut rng);
        let content = convert_cells_to_content(size, &cells);

        assert_eq!(
            content.cells[content_index_for_mouse(
                point(Pixels::from(-10.), Pixels::from(-10.)),
                &content.terminal_bounds,
            )]
            .c,
            cells[0][0]
        );
        assert_eq!(
            content.cells[content_index_for_mouse(
                point(Pixels::from(1000.), Pixels::from(1000.)),
                &content.terminal_bounds,
            )]
            .c,
            cells[9][9]
        );
    }

    fn get_cells(size: TerminalBounds, rng: &mut ThreadRng) -> Vec<Vec<char>> {
        let mut cells = Vec::new();

        for _ in 0..((size.height() / size.line_height()) as usize) {
            let mut row_vec = Vec::new();
            for _ in 0..((size.width() / size.cell_width()) as usize) {
                let cell_char = rng.sample(Alphanumeric) as char;
                row_vec.push(cell_char)
            }
            cells.push(row_vec)
        }

        cells
    }

    fn convert_cells_to_content(
        terminal_bounds: TerminalBounds,
        cells: &[Vec<char>],
    ) -> TerminalContent {
        let mut ic = Vec::new();

        for (index, row) in cells.iter().enumerate() {
            for (cell_index, cell_char) in row.iter().enumerate() {
                ic.push(IndexedCell {
                    point: AlacPoint::new(Line(index as i32), Column(cell_index)),
                    cell: Cell {
                        c: *cell_char,
                        ..Default::default()
                    },
                });
            }
        }

        TerminalContent {
            cells: ic,
            terminal_bounds,
            ..Default::default()
        }
    }

    fn re_test(re: &str, hay: &str, expected: Vec<&str>) {
        let results: Vec<_> = regex::Regex::new(re)
            .unwrap()
            .find_iter(hay)
            .map(|m| m.as_str())
            .collect();
        assert_eq!(results, expected);
    }
    #[test]
    fn test_url_regex() {
        re_test(
            crate::URL_REGEX,
            "test http://example.com test mailto:bob@example.com train",
            vec!["http://example.com", "mailto:bob@example.com"],
        );
    }
    #[test]
    fn test_word_regex() {
        re_test(
            crate::WORD_REGEX,
            "hello, world! \"What\" is this?",
            vec!["hello", "world", "What", "is", "this"],
        );
    }
    #[test]
    fn test_word_regex_with_linenum() {
        // filename(line) and filename(line,col) as used in MSBuild output
        // should be considered a single "word", even though comma is
        // usually a word separator
        re_test(
            crate::WORD_REGEX,
            "a Main.cs(20) b",
            vec!["a", "Main.cs(20)", "b"],
        );
        re_test(
            crate::WORD_REGEX,
            "Main.cs(20,5) Error desc",
            vec!["Main.cs(20,5)", "Error", "desc"],
        );
        // filename:line:col is a popular format for unix tools
        re_test(
            crate::WORD_REGEX,
            "a Main.cs:20:5 b",
            vec!["a", "Main.cs:20:5", "b"],
        );
        // Some tools output "filename:line:col:message", which currently isn't
        // handled correctly, but might be in the future
        re_test(
            crate::WORD_REGEX,
            "Main.cs:20:5:Error desc",
            vec!["Main.cs:20:5:Error", "desc"],
        );
    }

    #[test]
    fn test_python_file_line_regex() {
        re_test(
            crate::PYTHON_FILE_LINE_REGEX,
            "hay File \"/zed/bad_py.py\", line 8 stack",
            vec!["File \"/zed/bad_py.py\", line 8"],
        );
        re_test(crate::PYTHON_FILE_LINE_REGEX, "unrelated", vec![]);
    }

    #[test]
    fn test_python_file_line() {
        let inputs: Vec<(&str, Option<(&str, u32)>)> = vec![
            (
                "File \"/zed/bad_py.py\", line 8",
                Some(("/zed/bad_py.py", 8u32)),
            ),
            ("File \"path/to/zed/bad_py.py\"", None),
            ("unrelated", None),
            ("", None),
        ];
        let actual = inputs
            .iter()
            .map(|input| python_extract_path_and_line(input.0))
            .collect::<Vec<_>>();
        let expected = inputs.iter().map(|(_, output)| *output).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
