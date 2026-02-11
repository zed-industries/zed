pub mod mappings;

pub use alacritty_terminal;

mod pty_info;
mod terminal_hyperlinks;
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
use anyhow::{Context as _, Result, bail};
use log::trace;

use futures::{
    FutureExt,
    channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded},
};

use itertools::Itertools as _;
use mappings::mouse::{
    alt_scroll, grid_point, grid_point_and_side, mouse_button_report, mouse_moved_report,
    scroll_report,
};

use collections::{HashMap, VecDeque};
use futures::StreamExt;
use pty_info::{ProcessIdGetter, PtyProcessInfo};
use serde::{Deserialize, Serialize};
use settings::Settings;
use smol::channel::{Receiver, Sender};
use task::{HideStrategy, Shell, SpawnInTerminal};
use terminal_hyperlinks::RegexSearches;
use terminal_settings::{AlternateScroll, CursorShape, TerminalSettings};
use theme::{ActiveTheme, Theme};
use urlencoding;
use util::{paths::PathStyle, truncate_and_trailoff};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::{
    borrow::Cow,
    cmp::{self, min},
    fmt::Display,
    ops::{Deref, RangeInclusive},
    path::PathBuf,
    process::ExitStatus,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;

use gpui::{
    App, AppContext as _, BackgroundExecutor, Bounds, ClipboardItem, Context, EventEmitter, Hsla,
    Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point,
    Rgba, ScrollWheelEvent, Size, Task, TouchPhase, Window, actions, black, px,
};

use crate::mappings::{colors::to_alac_rgb, keys::to_esc_str};

actions!(
    terminal,
    [
        /// Clears the terminal screen.
        Clear,
        /// Copies selected text to the clipboard.
        Copy,
        /// Pastes from the clipboard.
        Paste,
        /// Shows the character palette for special characters.
        ShowCharacterPalette,
        /// Searches for text in the terminal.
        SearchTest,
        /// Scrolls up by one line.
        ScrollLineUp,
        /// Scrolls down by one line.
        ScrollLineDown,
        /// Scrolls up by one page.
        ScrollPageUp,
        /// Scrolls down by one page.
        ScrollPageDown,
        /// Scrolls up by half a page.
        ScrollHalfPageUp,
        /// Scrolls down by half a page.
        ScrollHalfPageDown,
        /// Scrolls to the top of the terminal buffer.
        ScrollToTop,
        /// Scrolls to the bottom of the terminal buffer.
        ScrollToBottom,
        /// Toggles vi mode in the terminal.
        ToggleViMode,
        /// Selects all text in the terminal.
        SelectAll,
    ]
);

const DEBUG_TERMINAL_WIDTH: Pixels = px(500.);
const DEBUG_TERMINAL_HEIGHT: Pixels = px(30.);
const DEBUG_CELL_WIDTH: Pixels = px(5.);
const DEBUG_LINE_HEIGHT: Pixels = px(5.);

/// Inserts Zed-specific environment variables for terminal sessions.
/// Used by both local terminals and remote terminals (via SSH).
pub fn insert_zed_terminal_env(
    env: &mut HashMap<String, String>,
    version: &impl std::fmt::Display,
) {
    env.insert("ZED_TERM".to_string(), "true".to_string());
    env.insert("TERM_PROGRAM".to_string(), "zed".to_string());
    env.insert("TERM".to_string(), "xterm-256color".to_string());
    env.insert("COLORTERM".to_string(), "truecolor".to_string());
    env.insert("TERM_PROGRAM_VERSION".to_string(), version.to_string());
}

///Upward flowing events, for changing the title and such
#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathLikeTarget {
    /// File system path, absolute or relative, existing or not.
    /// Might have line and column number(s) attached as `file.rs:1:23`
    pub maybe_path: String,
    /// Current working directory of the terminal
    pub terminal_dir: Option<PathBuf>,
}

/// A string inside terminal, potentially useful as a URI that can be opened.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    FindHyperlink(Point<Pixels>, bool),
    ProcessHyperlink((String, bool, Match), bool),
    // Whether keep selection when copy
    Copy(Option<bool>),
    // Vi mode events
    ToggleViMode,
    ViMotion(ViMotion),
    MoveViCursorToAlacPoint(AlacPoint),
}

///A translation struct for Alacritty to communicate with us from their event loop
#[derive(Clone)]
pub struct ZedListener(pub UnboundedSender<AlacTermEvent>);

impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(event).ok();
    }
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
    pub program: Option<String>,
    pub args: Option<Vec<String>>,
    pub title_override: Option<String>,
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
            .unwrap_or_else(|| "<none specified>".to_string())
    }

    pub fn fmt_shell(&self) -> String {
        if let Some(title_override) = &self.title_override {
            format!(
                "{} {} ({})",
                self.program.as_deref().unwrap_or("<system defined shell>"),
                self.args.as_ref().into_iter().flatten().format(" "),
                title_override
            )
        } else {
            format!(
                "{} {}",
                self.program.as_deref().unwrap_or("<system defined shell>"),
                self.args.as_ref().into_iter().flatten().format(" ")
            )
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
pub const MAX_SCROLL_HISTORY_LINES: usize = 100_000;

pub struct TerminalBuilder {
    terminal: Terminal,
    events_rx: UnboundedReceiver<AlacTermEvent>,
}

impl TerminalBuilder {
    pub fn new_display_only(
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
    ) -> Result<TerminalBuilder> {
        // Create a display-only terminal (no actual PTY).
        let default_cursor_style = AlacCursorStyle::from(cursor_shape);
        let scrolling_history = max_scroll_history_lines
            .unwrap_or(DEFAULT_SCROLL_HISTORY_LINES)
            .min(MAX_SCROLL_HISTORY_LINES);
        let config = Config {
            scrolling_history,
            default_cursor_style,
            ..Config::default()
        };

        let (events_tx, events_rx) = unbounded();
        let mut term = Term::new(
            config.clone(),
            &TerminalBounds::default(),
            ZedListener(events_tx),
        );

        if let AlternateScroll::Off = alternate_scroll {
            term.unset_private_mode(PrivateMode::Named(NamedPrivateMode::AlternateScroll));
        }

        let term = Arc::new(FairMutex::new(term));

        let terminal = Terminal {
            task: None,
            terminal_type: TerminalType::DisplayOnly,
            completion_tx: None,
            term,
            term_config: config,
            title_override: None,
            events: VecDeque::with_capacity(10),
            last_content: Default::default(),
            last_mouse: None,
            matches: Vec::new(),

            selection_head: None,
            breadcrumb_text: String::new(),
            scroll_px: px(0.),
            next_link_id: 0,
            selection_phase: SelectionPhase::Ended,
            hyperlink_regex_searches: RegexSearches::default(),
            vi_mode_enabled: false,
            is_remote_terminal: false,
            last_mouse_move_time: Instant::now(),
            last_hyperlink_search_position: None,
            mouse_down_hyperlink: None,
            #[cfg(windows)]
            shell_program: None,
            activation_script: Vec::new(),
            template: CopyTemplate {
                shell: Shell::System,
                env: HashMap::default(),
                cursor_shape,
                alternate_scroll,
                max_scroll_history_lines,
                path_hyperlink_regexes: Vec::default(),
                path_hyperlink_timeout_ms: 0,
                window_id,
            },
            child_exited: None,
            event_loop_task: Task::ready(Ok(())),
            background_executor: background_executor.clone(),
            path_style,
        };

        Ok(TerminalBuilder {
            terminal,
            events_rx,
        })
    }

    pub fn new(
        working_directory: Option<PathBuf>,
        task: Option<TaskState>,
        shell: Shell,
        mut env: HashMap<String, String>,
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        path_hyperlink_regexes: Vec<String>,
        path_hyperlink_timeout_ms: u64,
        is_remote_terminal: bool,
        window_id: u64,
        completion_tx: Option<Sender<Option<ExitStatus>>>,
        cx: &App,
        activation_script: Vec<String>,
        path_style: PathStyle,
    ) -> Task<Result<TerminalBuilder>> {
        let version = release_channel::AppVersion::global(cx);
        let background_executor = cx.background_executor().clone();
        let fut = async move {
            // Remove SHLVL so the spawned shell initializes it to 1, matching
            // the behavior of standalone terminal emulators like iTerm2/Kitty/Alacritty.
            env.remove("SHLVL");

            // If the parent environment doesn't have a locale set
            // (As is the case when launched from a .app on MacOS),
            // and the Project doesn't have a locale set, then
            // set a fallback for our child environment to use.
            if std::env::var("LANG").is_err() {
                env.entry("LANG".to_string())
                    .or_insert_with(|| "en_US.UTF-8".to_string());
            }

            insert_zed_terminal_env(&mut env, &version);

            #[derive(Default)]
            struct ShellParams {
                program: String,
                args: Option<Vec<String>>,
                title_override: Option<String>,
            }

            impl ShellParams {
                fn new(
                    program: String,
                    args: Option<Vec<String>>,
                    title_override: Option<String>,
                ) -> Self {
                    log::debug!("Using {program} as shell");
                    Self {
                        program,
                        args,
                        title_override,
                    }
                }
            }

            let shell_params = match shell.clone() {
                Shell::System => {
                    if cfg!(windows) {
                        Some(ShellParams::new(
                            util::shell::get_windows_system_shell(),
                            None,
                            None,
                        ))
                    } else {
                        None
                    }
                }
                Shell::Program(program) => Some(ShellParams::new(program, None, None)),
                Shell::WithArguments {
                    program,
                    args,
                    title_override,
                } => Some(ShellParams::new(program, Some(args), title_override)),
            };
            let terminal_title_override =
                shell_params.as_ref().and_then(|e| e.title_override.clone());

            #[cfg(windows)]
            let shell_program = shell_params.as_ref().map(|params| {
                use util::ResultExt;

                Self::resolve_path(&params.program)
                    .log_err()
                    .unwrap_or(params.program.clone())
            });

            // Note: when remoting, this shell_kind will scrutinize `ssh` or
            // `wsl.exe` as a shell and fall back to posix or powershell based on
            // the compilation target. This is fine right now due to the restricted
            // way we use the return value, but would become incorrect if we
            // supported remoting into windows.
            let shell_kind = shell.shell_kind(cfg!(windows));

            let pty_options = {
                let alac_shell = shell_params.as_ref().map(|params| {
                    alacritty_terminal::tty::Shell::new(
                        params.program.clone(),
                        params.args.clone().unwrap_or_default(),
                    )
                });

                alacritty_terminal::tty::Options {
                    shell: alac_shell,
                    working_directory: working_directory.clone(),
                    drain_on_exit: true,
                    env: env.clone().into_iter().collect(),
                    #[cfg(windows)]
                    escape_args: shell_kind.tty_escape_args(),
                }
            };

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

            //Setup the pty...
            let pty = match tty::new(&pty_options, TerminalBounds::default().into(), window_id) {
                Ok(pty) => pty,
                Err(error) => {
                    bail!(TerminalError {
                        directory: working_directory,
                        program: shell_params.as_ref().map(|params| params.program.clone()),
                        args: shell_params.as_ref().and_then(|params| params.args.clone()),
                        title_override: terminal_title_override,
                        source: error,
                    });
                }
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

            let pty_info = PtyProcessInfo::new(&pty);

            //And connect them together
            let event_loop = EventLoop::new(
                term.clone(),
                ZedListener(events_tx),
                pty,
                pty_options.drain_on_exit,
                false,
            )
            .context("failed to create event loop")?;

            let pty_tx = event_loop.channel();
            let _io_thread = event_loop.spawn(); // DANGER

            let no_task = task.is_none();
            let terminal = Terminal {
                task,
                terminal_type: TerminalType::Pty {
                    pty_tx: Notifier(pty_tx),
                    info: Arc::new(pty_info),
                },
                completion_tx,
                term,
                term_config: config,
                title_override: terminal_title_override,
                events: VecDeque::with_capacity(10), //Should never get this high.
                last_content: Default::default(),
                last_mouse: None,
                matches: Vec::new(),

                selection_head: None,
                breadcrumb_text: String::new(),
                scroll_px: px(0.),
                next_link_id: 0,
                selection_phase: SelectionPhase::Ended,
                hyperlink_regex_searches: RegexSearches::new(
                    &path_hyperlink_regexes,
                    path_hyperlink_timeout_ms,
                ),
                vi_mode_enabled: false,
                is_remote_terminal,
                last_mouse_move_time: Instant::now(),
                last_hyperlink_search_position: None,
                mouse_down_hyperlink: None,
                #[cfg(windows)]
                shell_program,
                activation_script: activation_script.clone(),
                template: CopyTemplate {
                    shell,
                    env,
                    cursor_shape,
                    alternate_scroll,
                    max_scroll_history_lines,
                    path_hyperlink_regexes,
                    path_hyperlink_timeout_ms,
                    window_id,
                },
                child_exited: None,
                event_loop_task: Task::ready(Ok(())),
                background_executor,
                path_style,
            };

            if !activation_script.is_empty() && no_task {
                for activation_script in activation_script {
                    terminal.write_to_pty(activation_script.into_bytes());
                    // Simulate enter key press
                    // NOTE(PowerShell): using `\r\n` will put PowerShell in a continuation mode (infamous >> character)
                    // and generally mess up the rendering.
                    terminal.write_to_pty(b"\x0d");
                }
                // In order to clear the screen at this point, we have two options:
                // 1. We can send a shell-specific command such as "clear" or "cls"
                // 2. We can "echo" a marker message that we will then catch when handling a Wakeup event
                //    and clear the screen using `terminal.clear()` method
                // We cannot issue a `terminal.clear()` command at this point as alacritty is evented
                // and while we have sent the activation script to the pty, it will be executed asynchronously.
                // Therefore, we somehow need to wait for the activation script to finish executing before we
                // can proceed with clearing the screen.
                terminal.write_to_pty(shell_kind.clear_screen_command().as_bytes());
                // Simulate enter key press
                terminal.write_to_pty(b"\x0d");
            }

            Ok(TerminalBuilder {
                terminal,
                events_rx,
            })
        };
        // the thread we spawn things on has an effect on signal handling
        if !cfg!(target_os = "windows") {
            cx.spawn(async move |_| fut.await)
        } else {
            cx.background_spawn(fut)
        }
    }

    pub fn subscribe(mut self, cx: &Context<Terminal>) -> Terminal {
        //Event loop
        self.terminal.event_loop_task = cx.spawn(async move |terminal, cx| {
            while let Some(event) = self.events_rx.next().await {
                terminal.update(cx, |terminal, cx| {
                    //Process the first event immediately for lowered latency
                    terminal.process_event(event, cx);
                })?;

                'outer: loop {
                    let mut events = Vec::new();

                    #[cfg(any(test, feature = "test-support"))]
                    let mut timer = cx.background_executor().simulate_random_delay().fuse();
                    #[cfg(not(any(test, feature = "test-support")))]
                    let mut timer = cx
                        .background_executor()
                        .timer(std::time::Duration::from_millis(4))
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
                            this.process_event(AlacTermEvent::Wakeup, cx);
                        }

                        for event in events {
                            this.process_event(event, cx);
                        }
                    })?;
                    smol::future::yield_now().await;
                }
            }
            anyhow::Ok(())
        });
        self.terminal
    }

    #[cfg(windows)]
    fn resolve_path(path: &str) -> Result<String> {
        use windows::Win32::Storage::FileSystem::SearchPathW;
        use windows::core::HSTRING;

        let path = if path.starts_with(r"\\?\") || !path.contains(&['/', '\\']) {
            path.to_string()
        } else {
            r"\\?\".to_string() + path
        };

        let required_length = unsafe { SearchPathW(None, &HSTRING::from(&path), None, None, None) };
        let mut buf = vec![0u16; required_length as usize];
        let size = unsafe { SearchPathW(None, &HSTRING::from(&path), None, Some(&mut buf), None) };

        Ok(String::from_utf16(&buf[..size as usize])?)
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
    pub scrolled_to_top: bool,
    pub scrolled_to_bottom: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
            scrolled_to_top: false,
            scrolled_to_bottom: false,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum SelectionPhase {
    Selecting,
    Ended,
}

enum TerminalType {
    Pty {
        pty_tx: Notifier,
        info: Arc<PtyProcessInfo>,
    },
    DisplayOnly,
}

pub struct Terminal {
    terminal_type: TerminalType,
    completion_tx: Option<Sender<Option<ExitStatus>>>,
    term: Arc<FairMutex<Term<ZedListener>>>,
    term_config: Config,
    events: VecDeque<InternalEvent>,
    /// This is only used for mouse mode cell change detection
    last_mouse: Option<(AlacPoint, AlacDirection)>,
    pub matches: Vec<RangeInclusive<AlacPoint>>,
    pub last_content: TerminalContent,
    pub selection_head: Option<AlacPoint>,

    pub breadcrumb_text: String,
    title_override: Option<String>,
    scroll_px: Pixels,
    next_link_id: usize,
    selection_phase: SelectionPhase,
    hyperlink_regex_searches: RegexSearches,
    task: Option<TaskState>,
    vi_mode_enabled: bool,
    is_remote_terminal: bool,
    last_mouse_move_time: Instant,
    last_hyperlink_search_position: Option<Point<Pixels>>,
    mouse_down_hyperlink: Option<(String, bool, Match)>,
    #[cfg(windows)]
    shell_program: Option<String>,
    template: CopyTemplate,
    activation_script: Vec<String>,
    child_exited: Option<ExitStatus>,
    event_loop_task: Task<Result<(), anyhow::Error>>,
    background_executor: BackgroundExecutor,
    path_style: PathStyle,
}

struct CopyTemplate {
    shell: Shell,
    env: HashMap<String, String>,
    cursor_shape: CursorShape,
    alternate_scroll: AlternateScroll,
    max_scroll_history_lines: Option<usize>,
    path_hyperlink_regexes: Vec<String>,
    path_hyperlink_timeout_ms: u64,
    window_id: u64,
}

#[derive(Debug)]
pub struct TaskState {
    pub status: TaskStatus,
    pub completion_rx: Receiver<Option<ExitStatus>>,
    pub spawned_task: SpawnInTerminal,
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

const FIND_HYPERLINK_THROTTLE_PX: Pixels = px(5.0);

impl Terminal {
    fn process_event(&mut self, event: AlacTermEvent, cx: &mut Context<Self>) {
        match event {
            AlacTermEvent::Title(title) => {
                // ignore default shell program title change as windows always sends those events
                // and it would end up showing the shell executable path in breadcrumbs
                #[cfg(windows)]
                {
                    if self
                        .shell_program
                        .as_ref()
                        .map(|e| *e == title)
                        .unwrap_or(false)
                    {
                        return;
                    }
                }

                self.breadcrumb_text = title;
                cx.emit(Event::BreadcrumbsChanged);
            }
            AlacTermEvent::ResetTitle => {
                self.breadcrumb_text = String::new();
                cx.emit(Event::BreadcrumbsChanged);
            }
            AlacTermEvent::ClipboardStore(_, data) => {
                cx.write_to_clipboard(ClipboardItem::new_string(data))
            }
            AlacTermEvent::ClipboardLoad(_, format) => {
                self.write_to_pty(
                    match &cx.read_from_clipboard().and_then(|item| item.text()) {
                        // The terminal only supports pasting strings, not images.
                        Some(text) => format(text),
                        _ => format(""),
                    }
                    .into_bytes(),
                )
            }
            AlacTermEvent::PtyWrite(out) => self.write_to_pty(out.into_bytes()),
            AlacTermEvent::TextAreaSizeRequest(format) => {
                self.write_to_pty(format(self.last_content.terminal_bounds.into()).into_bytes())
            }
            AlacTermEvent::CursorBlinkingChange => {
                let terminal = self.term.lock();
                let blinking = terminal.cursor_style().blinking;
                cx.emit(Event::BlinkChanged(blinking));
            }
            AlacTermEvent::Bell => {
                cx.emit(Event::Bell);
            }
            AlacTermEvent::Exit => self.register_task_finished(Some(9), cx),
            AlacTermEvent::MouseCursorDirty => {
                //NOOP, Handled in render
            }
            AlacTermEvent::Wakeup => {
                cx.emit(Event::Wakeup);

                if let TerminalType::Pty { info, .. } = &self.terminal_type {
                    info.emit_title_changed_if_changed(cx);
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
                let color = self.term.lock().colors()[index]
                    .unwrap_or_else(|| to_alac_rgb(get_color_at_index(index, cx.theme().as_ref())));
                self.write_to_pty(format(color).into_bytes());
            }
            AlacTermEvent::ChildExit(raw_status) => {
                self.register_task_finished(Some(raw_status), cx);
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
                trace!("Resizing: new_bounds={new_bounds:?}");
                new_bounds.bounds.size.height =
                    cmp::max(new_bounds.line_height, new_bounds.height());
                new_bounds.bounds.size.width = cmp::max(new_bounds.cell_width, new_bounds.width());

                self.last_content.terminal_bounds = new_bounds;

                if let TerminalType::Pty { pty_tx, .. } = &self.terminal_type {
                    pty_tx.0.send(Msg::Resize(new_bounds.into())).ok();
                }

                term.resize(new_bounds);
                // If there are matches we need to emit a wake up event to
                // invalidate the matches and recalculate their locations
                // in the new terminal layout
                if !self.matches.is_empty() {
                    cx.emit(Event::Wakeup);
                }
            }
            InternalEvent::Clear => {
                trace!("Clearing");
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
                trace!("Scrolling: scroll={scroll:?}");
                term.scroll_display(*scroll);
                self.refresh_hovered_word(window);

                if self.vi_mode_enabled {
                    match *scroll {
                        AlacScroll::Delta(delta) => {
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, delta);
                        }
                        AlacScroll::PageUp => {
                            let lines = term.screen_lines() as i32;
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, lines);
                        }
                        AlacScroll::PageDown => {
                            let lines = -(term.screen_lines() as i32);
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, lines);
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
                trace!("Setting selection: selection={selection:?}");
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
                trace!("Updating selection: position={position:?}");
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

            InternalEvent::Copy(keep_selection) => {
                trace!("Copying selection: keep_selection={keep_selection:?}");
                if let Some(txt) = term.selection_to_string() {
                    cx.write_to_clipboard(ClipboardItem::new_string(txt));
                    if !keep_selection.unwrap_or_else(|| {
                        let settings = TerminalSettings::get_global(cx);
                        settings.keep_selection_on_copy
                    }) {
                        self.events.push_back(InternalEvent::SetSelection(None));
                    }
                }
            }
            InternalEvent::ScrollToAlacPoint(point) => {
                trace!("Scrolling to point: point={point:?}");
                term.scroll_to_point(*point);
                self.refresh_hovered_word(window);
            }
            InternalEvent::MoveViCursorToAlacPoint(point) => {
                trace!("Move vi cursor to point: point={point:?}");
                term.vi_goto_point(*point);
                self.refresh_hovered_word(window);
            }
            InternalEvent::ToggleViMode => {
                trace!("Toggling vi mode");
                self.vi_mode_enabled = !self.vi_mode_enabled;
                term.toggle_vi_mode();
            }
            InternalEvent::ViMotion(motion) => {
                trace!("Performing vi motion: motion={motion:?}");
                term.vi_motion(*motion);
            }
            InternalEvent::FindHyperlink(position, open) => {
                trace!("Finding hyperlink at position: position={position:?}, open={open:?}");

                let point = grid_point(
                    *position,
                    self.last_content.terminal_bounds,
                    term.grid().display_offset(),
                )
                .grid_clamp(term, Boundary::Grid);

                match terminal_hyperlinks::find_from_grid_point(
                    term,
                    point,
                    &mut self.hyperlink_regex_searches,
                    self.path_style,
                ) {
                    Some(hyperlink) => {
                        self.process_hyperlink(hyperlink, *open, cx);
                    }
                    None => {
                        self.last_content.last_hovered_word = None;
                        cx.emit(Event::NewNavigationTarget(None));
                    }
                }
            }
            InternalEvent::ProcessHyperlink(hyperlink, open) => {
                self.process_hyperlink(hyperlink.clone(), *open, cx);
            }
        }
    }

    fn process_hyperlink(
        &mut self,
        hyperlink: (String, bool, Match),
        open: bool,
        cx: &mut Context<Self>,
    ) {
        let (maybe_url_or_path, is_url, url_match) = hyperlink;
        let prev_hovered_word = self.last_content.last_hovered_word.take();

        let target = if is_url {
            if let Some(path) = maybe_url_or_path.strip_prefix("file://") {
                let decoded_path = urlencoding::decode(path)
                    .map(|decoded| decoded.into_owned())
                    .unwrap_or(path.to_owned());

                MaybeNavigationTarget::PathLike(PathLikeTarget {
                    maybe_path: decoded_path,
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

        if open {
            cx.emit(Event::Open(target));
        } else {
            self.update_selected_word(prev_hovered_word, url_match, maybe_url_or_path, target, cx);
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
        if let Some(prev_word) = prev_word
            && prev_word.word == word
            && prev_word.word_match == word_match
        {
            self.last_content.last_hovered_word = Some(HoveredWord {
                word,
                word_match,
                id: prev_word.id,
            });
            return;
        }

        self.last_content.last_hovered_word = Some(HoveredWord {
            word,
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

    pub fn write_output(&mut self, bytes: &[u8], cx: &mut Context<Self>) {
        // Inject bytes directly into the terminal emulator and refresh the UI.
        // This bypasses the PTY/event loop for display-only terminals.
        //
        // We first convert LF to CRLF, to get the expected line wrapping in Alacritty.
        // When output comes from piped commands (not a PTY) such as codex-acp, and that
        // output only contains LF (\n) without a CR (\r) after it, such as the output
        // of the `ls` command when running outside a PTY, Alacritty moves the cursor
        // cursor down a line but does not move it back to the initial column. This makes
        // the rendered output look ridiculous. To prevent this, we insert a CR (\r) before
        // each LF that didn't already have one. (Alacritty doesn't have a setting for this.)
        let mut converted = Vec::with_capacity(bytes.len());
        let mut prev_byte = 0u8;
        for &byte in bytes {
            if byte == b'\n' && prev_byte != b'\r' {
                converted.push(b'\r');
            }
            converted.push(byte);
            prev_byte = byte;
        }

        let mut processor = alacritty_terminal::vte::ansi::Processor::<
            alacritty_terminal::vte::ansi::StdSyncHandler,
        >::new();
        {
            let mut term = self.term.lock();
            processor.advance(&mut *term, &converted);
        }
        cx.emit(Event::Wakeup);
    }

    pub fn total_lines(&self) -> usize {
        self.term.lock_unfair().total_lines()
    }

    pub fn viewport_lines(&self) -> usize {
        self.term.lock_unfair().screen_lines()
    }

    //To test:
    //- Activate match on terminal (scrolling and selection)
    //- Editor search snapping behavior

    pub fn activate_match(&mut self, index: usize) {
        if let Some(search_match) = self.matches.get(index).cloned() {
            self.set_selection(Some((make_selection(&search_match), *search_match.end())));
            if self.vi_mode_enabled {
                self.events
                    .push_back(InternalEvent::MoveViCursorToAlacPoint(*search_match.end()));
            } else {
                self.events
                    .push_back(InternalEvent::ScrollToAlacPoint(*search_match.start()));
            }
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

    pub fn copy(&mut self, keep_selection: Option<bool>) {
        self.events.push_back(InternalEvent::Copy(keep_selection));
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

    pub fn scrolled_to_top(&self) -> bool {
        self.last_content.scrolled_to_top
    }

    pub fn scrolled_to_bottom(&self) -> bool {
        self.last_content.scrolled_to_bottom
    }

    ///Resize the terminal and the PTY.
    pub fn set_size(&mut self, new_bounds: TerminalBounds) {
        if self.last_content.terminal_bounds != new_bounds {
            self.events.push_back(InternalEvent::Resize(new_bounds))
        }
    }

    /// Write the Input payload to the PTY, if applicable.
    /// (This is a no-op for display-only terminals.)
    fn write_to_pty(&self, input: impl Into<Cow<'static, [u8]>>) {
        if let TerminalType::Pty { pty_tx, .. } = &self.terminal_type {
            let input = input.into();
            if log::log_enabled!(log::Level::Debug) {
                if let Ok(str) = str::from_utf8(&input) {
                    log::debug!("Writing to PTY: {:?}", str);
                } else {
                    log::debug!("Writing to PTY: {:?}", input);
                }
            }
            pty_tx.notify(input);
        }
    }

    pub fn input(&mut self, input: impl Into<Cow<'static, [u8]>>) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Bottom));
        self.events.push_back(InternalEvent::SetSelection(None));

        self.write_to_pty(input);
    }

    pub fn toggle_vi_mode(&mut self) {
        self.events.push_back(InternalEvent::ToggleViMode);
    }

    pub fn vi_motion(&mut self, keystroke: &Keystroke) {
        if !self.vi_mode_enabled {
            return;
        }

        let key: Cow<'_, str> = if keystroke.modifiers.shift {
            Cow::Owned(keystroke.key.to_uppercase())
        } else {
            Cow::Borrowed(keystroke.key.as_str())
        };

        let motion: Option<ViMotion> = match key.as_ref() {
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

        let scroll_motion = match key.as_ref() {
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

        match key.as_ref() {
            "v" => {
                let point = self.last_content.cursor.point;
                let selection_type = SelectionType::Simple;
                let side = AlacDirection::Right;
                let selection = Selection::new(selection_type, point, side);
                self.events
                    .push_back(InternalEvent::SetSelection(Some((selection, point))));
            }

            "escape" => {
                self.events.push_back(InternalEvent::SetSelection(None));
            }

            "y" => {
                self.copy(Some(false));
            }

            "i" => {
                self.scroll_to_bottom();
                self.toggle_vi_mode();
            }
            _ => {}
        }
    }

    pub fn try_keystroke(&mut self, keystroke: &Keystroke, option_as_meta: bool) -> bool {
        if self.vi_mode_enabled {
            self.vi_motion(keystroke);
            return true;
        }

        // Keep default terminal behavior
        let esc = to_esc_str(keystroke, &self.last_content.mode, option_as_meta);
        if let Some(esc) = esc {
            match esc {
                Cow::Borrowed(string) => self.input(string.as_bytes()),
                Cow::Owned(string) => self.input(string.into_bytes()),
            };
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

        self.input(paste_text.into_bytes());
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

        // Pre-allocate with estimated size to reduce reallocations
        let estimated_size = content.display_iter.size_hint().0;
        let mut cells = Vec::with_capacity(estimated_size);

        cells.extend(content.display_iter.map(|ic| IndexedCell {
            point: ic.point,
            cell: ic.cell.clone(),
        }));

        let selection_text = if content.selection.is_some() {
            term.selection_to_string()
        } else {
            None
        };

        TerminalContent {
            cells,
            mode: content.mode,
            display_offset: content.display_offset,
            selection_text,
            selection: content.selection,
            cursor: content.cursor,
            cursor_char: term.grid()[content.cursor.point].c,
            terminal_bounds: last_content.terminal_bounds,
            last_hovered_word: last_content.last_hovered_word.clone(),
            scrolled_to_top: content.display_offset == term.history_size(),
            scrolled_to_bottom: content.display_offset == 0,
        }
    }

    pub fn get_content(&self) -> String {
        let term = self.term.lock_unfair();
        let start = AlacPoint::new(term.topmost_line(), Column(0));
        let end = AlacPoint::new(term.bottommost_line(), term.last_column());
        term.bounds_to_string(start, end)
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
            self.write_to_pty("\x1b[I".as_bytes());
        }
    }

    pub fn focus_out(&mut self) {
        if self.last_content.mode.contains(TermMode::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[O".as_bytes());
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

            if self.mouse_changed(point, side)
                && let Some(bytes) =
                    mouse_moved_report(point, e.pressed_button, e.modifiers, self.last_content.mode)
            {
                self.write_to_pty(bytes);
            }
        } else {
            self.schedule_find_hyperlink(e.modifiers, e.position);
        }
        cx.notify();
    }

    fn schedule_find_hyperlink(&mut self, modifiers: Modifiers, position: Point<Pixels>) {
        if self.selection_phase == SelectionPhase::Selecting
            || !modifiers.secondary()
            || !self.last_content.terminal_bounds.bounds.contains(&position)
        {
            self.last_content.last_hovered_word = None;
            return;
        }

        // Throttle hyperlink searches to avoid excessive processing
        let now = Instant::now();
        if self
            .last_hyperlink_search_position
            .map_or(true, |last_pos| {
                // Only search if mouse moved significantly or enough time passed
                let distance_moved = ((position.x - last_pos.x).abs()
                    + (position.y - last_pos.y).abs())
                    > FIND_HYPERLINK_THROTTLE_PX;
                let time_elapsed = now.duration_since(self.last_mouse_move_time).as_millis() > 100;
                distance_moved || time_elapsed
            })
        {
            self.last_mouse_move_time = now;
            self.last_hyperlink_search_position = Some(position);
            self.events.push_back(InternalEvent::FindHyperlink(
                position - self.last_content.terminal_bounds.bounds.origin,
                false,
            ));
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
            if let Some((.., hyperlink_range)) = &self.mouse_down_hyperlink {
                let point = grid_point(
                    position,
                    self.last_content.terminal_bounds,
                    self.last_content.display_offset,
                );

                if !hyperlink_range.contains(&point) {
                    self.mouse_down_hyperlink = None;
                } else {
                    return;
                }
            }

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

        if e.button == MouseButton::Left
            && e.modifiers.secondary()
            && !self.mouse_mode(e.modifiers.shift)
        {
            let term_lock = self.term.lock();
            self.mouse_down_hyperlink = terminal_hyperlinks::find_from_grid_point(
                &term_lock,
                point,
                &mut self.hyperlink_regex_searches,
                self.path_style,
            );
            drop(term_lock);

            if self.mouse_down_hyperlink.is_some() {
                return;
            }
        }

        if self.mouse_mode(e.modifiers.shift) {
            if let Some(bytes) =
                mouse_button_report(point, e.button, e.modifiers, true, self.last_content.mode)
            {
                self.write_to_pty(bytes);
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
                        let text = item.text().unwrap_or_default();
                        self.input(text.into_bytes());
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
                self.write_to_pty(bytes);
            }
        } else {
            if e.button == MouseButton::Left && setting.copy_on_select {
                self.copy(Some(true));
            }

            if let Some(mouse_down_hyperlink) = self.mouse_down_hyperlink.take() {
                let point = grid_point(
                    position,
                    self.last_content.terminal_bounds,
                    self.last_content.display_offset,
                );

                if let Some(mouse_up_hyperlink) = {
                    let term_lock = self.term.lock();
                    terminal_hyperlinks::find_from_grid_point(
                        &term_lock,
                        point,
                        &mut self.hyperlink_regex_searches,
                        self.path_style,
                    )
                } {
                    if mouse_down_hyperlink == mouse_up_hyperlink {
                        self.events
                            .push_back(InternalEvent::ProcessHyperlink(mouse_up_hyperlink, true));
                        self.selection_phase = SelectionPhase::Ended;
                        self.last_mouse = None;
                        return;
                    }
                }
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
    pub fn scroll_wheel(&mut self, e: &ScrollWheelEvent, scroll_multiplier: f32) {
        let mouse_mode = self.mouse_mode(e.shift);
        let scroll_multiplier = if mouse_mode { 1. } else { scroll_multiplier };

        if let Some(scroll_lines) = self.determine_scroll_lines(e, scroll_multiplier)
            && scroll_lines != 0
        {
            if mouse_mode {
                let point = grid_point(
                    e.position - self.last_content.terminal_bounds.bounds.origin,
                    self.last_content.terminal_bounds,
                    self.last_content.display_offset,
                );

                if let Some(scrolls) = scroll_report(point, scroll_lines, e, self.last_content.mode)
                {
                    for scroll in scrolls {
                        self.write_to_pty(scroll);
                    }
                };
            } else if self
                .last_content
                .mode
                .contains(TermMode::ALT_SCREEN | TermMode::ALTERNATE_SCROLL)
                && !e.shift
            {
                self.write_to_pty(alt_scroll(scroll_lines));
            } else {
                let scroll = AlacScroll::Delta(scroll_lines);

                self.events.push_back(InternalEvent::Scroll(scroll));
            }
        }
    }

    fn refresh_hovered_word(&mut self, window: &Window) {
        self.schedule_find_hyperlink(window.modifiers(), window.mouse_position());
    }

    fn determine_scroll_lines(
        &mut self,
        e: &ScrollWheelEvent,
        scroll_multiplier: f32,
    ) -> Option<i32> {
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
        if self.is_remote_terminal {
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
        match &self.terminal_type {
            TerminalType::Pty { info, .. } => info
                .current
                .read()
                .as_ref()
                .map(|process| process.cwd.clone()),
            TerminalType::DisplayOnly => None,
        }
    }

    pub fn title(&self, truncate: bool) -> String {
        const MAX_CHARS: usize = 25;
        match &self.task {
            Some(task_state) => {
                if truncate {
                    truncate_and_trailoff(&task_state.spawned_task.label, MAX_CHARS)
                } else {
                    task_state.spawned_task.full_label.clone()
                }
            }
            None => self
                .title_override
                .as_ref()
                .map(|title_override| title_override.to_string())
                .unwrap_or_else(|| match &self.terminal_type {
                    TerminalType::Pty { info, .. } => info
                        .current
                        .read()
                        .as_ref()
                        .map(|fpi| {
                            let process_file = fpi
                                .cwd
                                .file_name()
                                .map(|name| name.to_string_lossy().into_owned())
                                .unwrap_or_default();

                            let argv = fpi.argv.as_slice();
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
                        .unwrap_or_else(|| "Terminal".to_string()),
                    TerminalType::DisplayOnly => "Terminal".to_string(),
                }),
        }
    }

    pub fn kill_active_task(&mut self) {
        if let Some(task) = self.task()
            && task.status == TaskStatus::Running
        {
            if let TerminalType::Pty { info, .. } = &self.terminal_type {
                // First kill the foreground process group (the command running in the shell)
                info.kill_current_process();
                // Then kill the shell itself so that the terminal exits properly
                // and wait_for_completed_task can complete
                info.kill_child_process();
            }
        }
    }

    pub fn pid(&self) -> Option<sysinfo::Pid> {
        match &self.terminal_type {
            TerminalType::Pty { info, .. } => info.pid(),
            TerminalType::DisplayOnly => None,
        }
    }

    pub fn pid_getter(&self) -> Option<&ProcessIdGetter> {
        match &self.terminal_type {
            TerminalType::Pty { info, .. } => Some(info.pid_getter()),
            TerminalType::DisplayOnly => None,
        }
    }

    pub fn task(&self) -> Option<&TaskState> {
        self.task.as_ref()
    }

    pub fn wait_for_completed_task(&self, cx: &App) -> Task<Option<ExitStatus>> {
        if let Some(task) = self.task() {
            if task.status == TaskStatus::Running {
                let completion_receiver = task.completion_rx.clone();
                return cx.spawn(async move |_| completion_receiver.recv().await.ok().flatten());
            } else if let Ok(status) = task.completion_rx.try_recv() {
                return Task::ready(status);
            }
        }
        Task::ready(None)
    }

    fn register_task_finished(&mut self, raw_status: Option<i32>, cx: &mut Context<Terminal>) {
        let exit_status: Option<ExitStatus> = raw_status.map(|value| {
            #[cfg(unix)]
            {
                std::os::unix::process::ExitStatusExt::from_raw(value)
            }
            #[cfg(windows)]
            {
                std::os::windows::process::ExitStatusExt::from_raw(value as u32)
            }
        });

        if let Some(tx) = &self.completion_tx {
            tx.try_send(exit_status).ok();
        }
        if let Some(e) = exit_status {
            self.child_exited = Some(e);
        }
        let task = match &mut self.task {
            Some(task) => task,
            None => {
                if self.child_exited.is_none_or(|e| e.code() == Some(0)) {
                    cx.emit(Event::CloseTerminal);
                }
                return;
            }
        };
        if task.status != TaskStatus::Running {
            return;
        }
        match exit_status.and_then(|e| e.code()) {
            Some(error_code) => {
                task.status.register_task_exit(error_code);
            }
            None => {
                task.status.register_terminal_exit();
            }
        };

        let (finished_successfully, task_line, command_line) = task_summary(task, exit_status);
        let mut lines_to_show = Vec::new();
        if task.spawned_task.show_summary {
            lines_to_show.push(task_line.as_str());
        }
        if task.spawned_task.show_command {
            lines_to_show.push(command_line.as_str());
        }

        if !lines_to_show.is_empty() {
            // SAFETY: the invocation happens on non `TaskStatus::Running` tasks, once,
            // after either `AlacTermEvent::Exit` or `AlacTermEvent::ChildExit` events that are spawned
            // when Zed task finishes and no more output is made.
            // After the task summary is output once, no more text is appended to the terminal.
            unsafe { append_text_to_term(&mut self.term.lock(), &lines_to_show) };
        }

        match task.spawned_task.hide {
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

    pub fn clone_builder(&self, cx: &App, cwd: Option<PathBuf>) -> Task<Result<TerminalBuilder>> {
        let working_directory = self.working_directory().or_else(|| cwd);
        TerminalBuilder::new(
            working_directory,
            None,
            self.template.shell.clone(),
            self.template.env.clone(),
            self.template.cursor_shape,
            self.template.alternate_scroll,
            self.template.max_scroll_history_lines,
            self.template.path_hyperlink_regexes.clone(),
            self.template.path_hyperlink_timeout_ms,
            self.is_remote_terminal,
            self.template.window_id,
            None,
            cx,
            self.activation_script.clone(),
            self.path_style,
        )
    }
}

// Helper function to convert a grid row to a string
pub fn row_to_string(row: &Row<Cell>) -> String {
    row[..Column(row.len())]
        .iter()
        .map(|cell| cell.c)
        .collect::<String>()
}

const TASK_DELIMITER: &str = "⏵ ";
fn task_summary(task: &TaskState, exit_status: Option<ExitStatus>) -> (bool, String, String) {
    let escaped_full_label = task
        .spawned_task
        .full_label
        .replace("\r\n", "\r")
        .replace('\n', "\r");
    let task_label = |suffix: &str| format!("{TASK_DELIMITER}Task `{escaped_full_label}` {suffix}");
    let (success, task_line) = match exit_status {
        Some(status) => {
            let code = status.code();
            #[cfg(unix)]
            let signal = status.signal();
            #[cfg(not(unix))]
            let signal: Option<i32> = None;

            match (code, signal) {
                (Some(0), _) => (true, task_label("finished successfully")),
                (Some(code), _) => (
                    false,
                    task_label(&format!("finished with exit code: {code}")),
                ),
                (None, Some(signal)) => (
                    false,
                    task_label(&format!("terminated by signal: {signal}")),
                ),
                (None, None) => (false, task_label("finished")),
            }
        }
        None => (false, task_label("finished")),
    };
    let escaped_command_label = task
        .spawned_task
        .command_label
        .replace("\r\n", "\r")
        .replace('\n', "\r");
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
        if let TerminalType::Pty { pty_tx, info } =
            std::mem::replace(&mut self.terminal_type, TerminalType::DisplayOnly)
        {
            pty_tx.0.send(Msg::Shutdown).ok();

            let timer = self.background_executor.timer(Duration::from_millis(100));
            self.background_executor
                .spawn(async move {
                    timer.await;
                    info.kill_child_process();
                })
                .detach();
        }
    }
}

impl EventEmitter<Event> for Terminal {}

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
        // 16-231 are a 6x6x6 RGB color cube, mapped to 0-255 using steps defined by XTerm.
        // See: https://github.com/xterm-x11/xterm-snapshots/blob/master/256colres.pl
        16..=231 => {
            let (r, g, b) = rgb_for_index(index as u8);
            rgba_color(
                if r == 0 { 0 } else { r * 40 + 55 },
                if g == 0 { 0 } else { g * 40 + 55 },
                if b == 0 { 0 } else { b * 40 + 55 },
            )
        }
        // 232-255 are a 24-step grayscale ramp from (8, 8, 8) to (238, 238, 238).
        232..=255 => {
            let i = index as u8 - 232; // Align index to 0..24
            let value = i * 10 + 8;
            rgba_color(value, value, value)
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
///
/// See: [8 bit ANSI color](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit).
///
/// Wikipedia gives a formula for calculating the index for a given color:
///
/// ```text
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
    use std::time::Duration;

    use super::*;
    use crate::{
        IndexedCell, TerminalBounds, TerminalBuilder, TerminalContent, content_index_for_mouse,
        rgb_for_index,
    };
    use alacritty_terminal::{
        index::{Column, Line, Point as AlacPoint},
        term::cell::Cell,
    };
    use collections::HashMap;
    use gpui::{
        Entity, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels,
        Point, TestAppContext, bounds, point, size,
    };
    use parking_lot::Mutex;
    use rand::{Rng, distr, rngs::ThreadRng};
    use smol::channel::Receiver;
    use task::{Shell, ShellBuilder};

    #[cfg(target_os = "macos")]
    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }

    /// Helper to build a test terminal running a shell command.
    /// Returns the terminal entity and a receiver for the completion signal.
    async fn build_test_terminal(
        cx: &mut TestAppContext,
        command: &str,
        args: &[&str],
    ) -> (Entity<Terminal>, Receiver<Option<ExitStatus>>) {
        let (completion_tx, completion_rx) = smol::channel::unbounded();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let (program, args) =
            ShellBuilder::new(&Shell::System, false).build(Some(command.to_owned()), &args);
        let builder = cx
            .update(|cx| {
                TerminalBuilder::new(
                    None,
                    None,
                    task::Shell::WithArguments {
                        program,
                        args,
                        title_override: None,
                    },
                    HashMap::default(),
                    CursorShape::default(),
                    AlternateScroll::On,
                    None,
                    vec![],
                    0,
                    false,
                    0,
                    Some(completion_tx),
                    cx,
                    vec![],
                    PathStyle::local(),
                )
            })
            .await
            .unwrap();
        let terminal = cx.new(|cx| builder.subscribe(cx));
        (terminal, completion_rx)
    }

    fn init_ctrl_click_hyperlink_test(cx: &mut TestAppContext, output: &[u8]) -> Entity<Terminal> {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        let terminal = cx.new(|cx| {
            TerminalBuilder::new_display_only(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .unwrap()
            .subscribe(cx)
        });

        terminal.update(cx, |terminal, cx| {
            terminal.write_output(output, cx);
        });

        cx.run_until_parked();

        terminal.update(cx, |terminal, _cx| {
            let term_lock = terminal.term.lock();
            terminal.last_content = Terminal::make_content(&term_lock, &terminal.last_content);
            drop(term_lock);

            let terminal_bounds = TerminalBounds::new(
                px(20.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(400.0), px(400.0))),
            );
            terminal.last_content.terminal_bounds = terminal_bounds;
            terminal.events.clear();
        });

        terminal
    }

    fn ctrl_mouse_down_at(
        terminal: &mut Terminal,
        position: Point<Pixels>,
        cx: &mut Context<Terminal>,
    ) {
        let mouse_down = MouseDownEvent {
            button: MouseButton::Left,
            position,
            modifiers: Modifiers::secondary_key(),
            click_count: 1,
            first_mouse: true,
        };
        terminal.mouse_down(&mouse_down, cx);
    }

    fn ctrl_mouse_move_to(
        terminal: &mut Terminal,
        position: Point<Pixels>,
        cx: &mut Context<Terminal>,
    ) {
        let terminal_bounds = terminal.last_content.terminal_bounds.bounds;
        let drag_event = MouseMoveEvent {
            position,
            pressed_button: Some(MouseButton::Left),
            modifiers: Modifiers::secondary_key(),
        };
        terminal.mouse_drag(&drag_event, terminal_bounds, cx);
    }

    fn ctrl_mouse_up_at(
        terminal: &mut Terminal,
        position: Point<Pixels>,
        cx: &mut Context<Terminal>,
    ) {
        let mouse_up = MouseUpEvent {
            button: MouseButton::Left,
            position,
            modifiers: Modifiers::secondary_key(),
            click_count: 1,
        };
        terminal.mouse_up(&mouse_up, cx);
    }

    #[gpui::test]
    async fn test_basic_terminal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let (terminal, completion_rx) = build_test_terminal(cx, "echo", &["hello"]).await;
        assert_eq!(
            completion_rx.recv().await.unwrap(),
            Some(ExitStatus::default())
        );
        assert_eq!(
            terminal.update(cx, |term, _| term.get_content()).trim(),
            "hello"
        );

        // Inject additional output directly into the emulator (display-only path)
        terminal.update(cx, |term, cx| {
            term.write_output(b"\nfrom_injection", cx);
        });

        let content_after = terminal.update(cx, |term, _| term.get_content());
        assert!(
            content_after.contains("from_injection"),
            "expected injected output to appear, got: {content_after}"
        );
    }

    // TODO should be tested on Linux too, but does not work there well
    #[cfg(target_os = "macos")]
    #[gpui::test(iterations = 10)]
    async fn test_terminal_eof(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let (completion_tx, completion_rx) = smol::channel::unbounded();
        let builder = cx
            .update(|cx| {
                TerminalBuilder::new(
                    None,
                    None,
                    task::Shell::System,
                    HashMap::default(),
                    CursorShape::default(),
                    AlternateScroll::On,
                    None,
                    vec![],
                    0,
                    false,
                    0,
                    Some(completion_tx),
                    cx,
                    Vec::new(),
                    PathStyle::local(),
                )
            })
            .await
            .unwrap();
        // Build an empty command, which will result in a tty shell spawned.
        let terminal = cx.new(|cx| builder.subscribe(cx));

        let (event_tx, event_rx) = smol::channel::unbounded::<Event>();
        cx.update(|cx| {
            cx.subscribe(&terminal, move |_, e, _| {
                event_tx.send_blocking(e.clone()).unwrap();
            })
        })
        .detach();
        cx.background_spawn(async move {
            assert_eq!(
                completion_rx.recv().await.unwrap(),
                Some(ExitStatus::default()),
                "EOF should result in the tty shell exiting successfully",
            );
        })
        .detach();

        let first_event = event_rx.recv().await.expect("No wakeup event received");

        terminal.update(cx, |terminal, _| {
            let success = terminal.try_keystroke(&Keystroke::parse("ctrl-c").unwrap(), false);
            assert!(success, "Should have registered ctrl-c sequence");
        });
        terminal.update(cx, |terminal, _| {
            let success = terminal.try_keystroke(&Keystroke::parse("ctrl-d").unwrap(), false);
            assert!(success, "Should have registered ctrl-d sequence");
        });

        let mut all_events = vec![first_event];
        while let Ok(new_event) = event_rx.recv().await {
            all_events.push(new_event.clone());
            if new_event == Event::CloseTerminal {
                break;
            }
        }
        assert!(
            all_events.contains(&Event::CloseTerminal),
            "EOF command sequence should have triggered a TTY terminal exit, but got events: {all_events:?}",
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_terminal_no_exit_on_spawn_failure(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let (completion_tx, completion_rx) = smol::channel::unbounded();
        let (program, args) = ShellBuilder::new(&Shell::System, false)
            .build(Some("asdasdasdasd".to_owned()), &["@@@@@".to_owned()]);
        let builder = cx
            .update(|cx| {
                TerminalBuilder::new(
                    None,
                    None,
                    task::Shell::WithArguments {
                        program,
                        args,
                        title_override: None,
                    },
                    HashMap::default(),
                    CursorShape::default(),
                    AlternateScroll::On,
                    None,
                    Vec::new(),
                    0,
                    false,
                    0,
                    Some(completion_tx),
                    cx,
                    Vec::new(),
                    PathStyle::local(),
                )
            })
            .await
            .unwrap();
        let terminal = cx.new(|cx| builder.subscribe(cx));

        let all_events: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(Vec::new()));
        cx.update({
            let all_events = all_events.clone();
            |cx| {
                cx.subscribe(&terminal, move |_, e, _| {
                    all_events.lock().push(e.clone());
                })
            }
        })
        .detach();
        let completion_check_task = cx.background_spawn(async move {
            // The channel may be closed if the terminal is dropped before sending
            // the completion signal, which can happen with certain task scheduling orders.
            let exit_status = completion_rx.recv().await.ok().flatten();
            if let Some(exit_status) = exit_status {
                assert!(
                    !exit_status.success(),
                    "Wrong shell command should result in a failure"
                );
                #[cfg(target_os = "windows")]
                assert_eq!(exit_status.code(), Some(1));
                #[cfg(not(target_os = "windows"))]
                assert_eq!(exit_status.code(), Some(127)); // code 127 means "command not found" on Unix
            }
        });

        completion_check_task.await;
        cx.executor().timer(Duration::from_millis(500)).await;

        assert!(
            !all_events
                .lock()
                .iter()
                .any(|event| event == &Event::CloseTerminal),
            "Wrong shell command should update the title but not should not close the terminal to show the error message, but got events: {all_events:?}",
        );
    }

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
        let mut rng = rand::rng();
        const ITERATIONS: usize = 10;
        const PRECISION: usize = 1000;

        for _ in 0..ITERATIONS {
            let viewport_cells = rng.random_range(15..20);
            let cell_size =
                rng.random_range(5 * PRECISION..20 * PRECISION) as f32 / PRECISION as f32;

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

                    let row_offset = rng.random_range(0..PRECISION) as f32 / PRECISION as f32;
                    let col_offset = rng.random_range(0..PRECISION) as f32 / PRECISION as f32;

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
        let mut rng = rand::rng();

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
                let cell_char = rng.sample(distr::Alphanumeric) as char;
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

    #[gpui::test]
    async fn test_write_output_converts_lf_to_crlf(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            TerminalBuilder::new_display_only(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .unwrap()
            .subscribe(cx)
        });

        // Test simple LF conversion
        terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"line1\nline2\n", cx);
        });

        // Get the content by directly accessing the term
        let content = terminal.update(cx, |terminal, _cx| {
            let term = terminal.term.lock_unfair();
            Terminal::make_content(&term, &terminal.last_content)
        });

        // If LF is properly converted to CRLF, each line should start at column 0
        // The diagonal staircase bug would cause increasing column positions

        // Get the cells and check that lines start at column 0
        let cells = &content.cells;
        let mut line1_col0 = false;
        let mut line2_col0 = false;

        for cell in cells {
            if cell.c == 'l' && cell.point.column.0 == 0 {
                if cell.point.line.0 == 0 && !line1_col0 {
                    line1_col0 = true;
                } else if cell.point.line.0 == 1 && !line2_col0 {
                    line2_col0 = true;
                }
            }
        }

        assert!(line1_col0, "First line should start at column 0");
        assert!(line2_col0, "Second line should start at column 0");
    }

    #[gpui::test]
    async fn test_write_output_preserves_existing_crlf(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            TerminalBuilder::new_display_only(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .unwrap()
            .subscribe(cx)
        });

        // Test that existing CRLF doesn't get doubled
        terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"line1\r\nline2\r\n", cx);
        });

        // Get the content by directly accessing the term
        let content = terminal.update(cx, |terminal, _cx| {
            let term = terminal.term.lock_unfair();
            Terminal::make_content(&term, &terminal.last_content)
        });

        let cells = &content.cells;

        // Check that both lines start at column 0
        let mut found_lines_at_column_0 = 0;
        for cell in cells {
            if cell.c == 'l' && cell.point.column.0 == 0 {
                found_lines_at_column_0 += 1;
            }
        }

        assert!(
            found_lines_at_column_0 >= 2,
            "Both lines should start at column 0"
        );
    }

    #[gpui::test]
    async fn test_write_output_preserves_bare_cr(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            TerminalBuilder::new_display_only(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .unwrap()
            .subscribe(cx)
        });

        // Test that bare CR (without LF) is preserved
        terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"hello\rworld", cx);
        });

        // Get the content by directly accessing the term
        let content = terminal.update(cx, |terminal, _cx| {
            let term = terminal.term.lock_unfair();
            Terminal::make_content(&term, &terminal.last_content)
        });

        let cells = &content.cells;

        // Check that we have "world" at the beginning of the line
        let mut text = String::new();
        for cell in cells.iter().take(5) {
            if cell.point.line.0 == 0 {
                text.push(cell.c);
            }
        }

        assert!(
            text.starts_with("world"),
            "Bare CR should allow overwriting: got '{}'",
            text
        );
    }

    #[gpui::test]
    async fn test_hyperlink_ctrl_click_same_position(cx: &mut TestAppContext) {
        let terminal = init_ctrl_click_hyperlink_test(cx, b"Visit https://zed.dev/ for more\r\n");

        terminal.update(cx, |terminal, cx| {
            let click_position = point(px(80.0), px(10.0));
            ctrl_mouse_down_at(terminal, click_position, cx);
            ctrl_mouse_up_at(terminal, click_position, cx);

            assert!(
                terminal
                    .events
                    .iter()
                    .any(|event| matches!(event, InternalEvent::ProcessHyperlink(_, true))),
                "Should have ProcessHyperlink event when ctrl+clicking on same hyperlink position"
            );
        });
    }

    #[gpui::test]
    async fn test_hyperlink_ctrl_click_drag_outside_bounds(cx: &mut TestAppContext) {
        let terminal = init_ctrl_click_hyperlink_test(
            cx,
            b"Visit https://zed.dev/ for more\r\nThis is another line\r\n",
        );

        terminal.update(cx, |terminal, cx| {
            let down_position = point(px(80.0), px(10.0));
            let up_position = point(px(10.0), px(50.0));

            ctrl_mouse_down_at(terminal, down_position, cx);
            ctrl_mouse_move_to(terminal, up_position, cx);
            ctrl_mouse_up_at(terminal, up_position, cx);

            assert!(
                !terminal
                    .events
                    .iter()
                    .any(|event| matches!(event, InternalEvent::ProcessHyperlink(_, _))),
                "Should NOT have ProcessHyperlink event when dragging outside the hyperlink"
            );
        });
    }

    #[gpui::test]
    async fn test_hyperlink_ctrl_click_drag_within_bounds(cx: &mut TestAppContext) {
        let terminal = init_ctrl_click_hyperlink_test(cx, b"Visit https://zed.dev/ for more\r\n");

        terminal.update(cx, |terminal, cx| {
            let down_position = point(px(70.0), px(10.0));
            let up_position = point(px(130.0), px(10.0));

            ctrl_mouse_down_at(terminal, down_position, cx);
            ctrl_mouse_move_to(terminal, up_position, cx);
            ctrl_mouse_up_at(terminal, up_position, cx);

            assert!(
                terminal
                    .events
                    .iter()
                    .any(|event| matches!(event, InternalEvent::ProcessHyperlink(_, true))),
                "Should have ProcessHyperlink event when dragging within hyperlink bounds"
            );
        });
    }

    /// Test that kill_active_task properly terminates both the foreground process
    /// and the shell, allowing wait_for_completed_task to complete and output to be captured.
    #[cfg(unix)]
    #[gpui::test]
    async fn test_kill_active_task_completes_and_captures_output(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        // Run a command that prints output then sleeps for a long time
        // The echo ensures we have output to capture before killing
        let (terminal, completion_rx) =
            build_test_terminal(cx, "echo", &["test_output_before_kill; sleep 60"]).await;

        // Wait a bit for the echo to execute and produce output
        cx.background_executor
            .timer(Duration::from_millis(200))
            .await;

        // Kill the active task
        terminal.update(cx, |term, _cx| {
            term.kill_active_task();
        });

        // wait_for_completed_task should complete within a reasonable time (not hang)
        let completion_result = completion_rx.recv().await;
        assert!(
            completion_result.is_ok(),
            "wait_for_completed_task should complete after kill_active_task, but it timed out"
        );

        // The exit status should indicate the process was killed (not a clean exit)
        let exit_status = completion_result.unwrap();
        assert!(
            exit_status.is_some(),
            "Should have received an exit status after killing"
        );

        // Verify that output captured before killing is still available
        let content = terminal.update(cx, |term, _| term.get_content());
        assert!(
            content.contains("test_output_before_kill"),
            "Output from before kill should be captured, got: {content}"
        );
    }

    /// Test that kill_active_task on a task that's not running is a no-op
    #[gpui::test]
    async fn test_kill_active_task_on_completed_task_is_noop(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        // Run a command that exits immediately
        let (terminal, completion_rx) = build_test_terminal(cx, "echo", &["done"]).await;

        // Wait for the command to complete naturally
        let exit_status = completion_rx
            .recv()
            .await
            .expect("Should receive exit status");
        assert_eq!(exit_status, Some(ExitStatus::default()));

        // Now try to kill - should be a no-op since task already completed
        terminal.update(cx, |term, _cx| {
            term.kill_active_task();
        });

        // Content should still be there
        let content = terminal.update(cx, |term, _| term.get_content());
        assert!(
            content.contains("done"),
            "Output should still be present after no-op kill, got: {content}"
        );
    }

    mod perf {
        use super::super::*;
        use gpui::{
            Entity, Point, ScrollDelta, ScrollWheelEvent, TestAppContext, VisualContext,
            VisualTestContext, point,
        };
        use util::default;
        use util_macros::perf;

        async fn init_scroll_perf_test(
            cx: &mut TestAppContext,
        ) -> (Entity<Terminal>, &mut VisualTestContext) {
            cx.update(|cx| {
                let settings_store = settings::SettingsStore::test(cx);
                cx.set_global(settings_store);
            });

            cx.executor().allow_parking();

            let window = cx.add_empty_window();
            let builder = window
                .update(|window, cx| {
                    let settings = TerminalSettings::get_global(cx);
                    let test_path_hyperlink_timeout_ms = 100;
                    TerminalBuilder::new(
                        None,
                        None,
                        task::Shell::System,
                        HashMap::default(),
                        CursorShape::default(),
                        AlternateScroll::On,
                        None,
                        settings.path_hyperlink_regexes.clone(),
                        test_path_hyperlink_timeout_ms,
                        false,
                        window.window_handle().window_id().as_u64(),
                        None,
                        cx,
                        vec![],
                        PathStyle::local(),
                    )
                })
                .await
                .unwrap();
            let terminal = window.new(|cx| builder.subscribe(cx));

            terminal.update(window, |term, cx| {
                term.write_output("long line ".repeat(1000).as_bytes(), cx);
            });

            (terminal, window)
        }

        #[perf]
        #[gpui::test]
        async fn scroll_long_line_benchmark(cx: &mut TestAppContext) {
            let (terminal, window) = init_scroll_perf_test(cx).await;
            let wobble = point(FIND_HYPERLINK_THROTTLE_PX, px(0.0));
            let mut scroll_by = |lines: i32| {
                window.update_window_entity(&terminal, |terminal, window, cx| {
                    let bounds = terminal.last_content.terminal_bounds.bounds;
                    let center = bounds.origin + bounds.center();
                    let position = center + wobble * lines as f32;

                    terminal.mouse_move(
                        &MouseMoveEvent {
                            position,
                            ..default()
                        },
                        cx,
                    );

                    terminal.scroll_wheel(
                        &ScrollWheelEvent {
                            position,
                            delta: ScrollDelta::Lines(Point::new(0.0, lines as f32)),
                            ..default()
                        },
                        1.0,
                    );

                    assert!(
                        terminal
                            .events
                            .iter()
                            .any(|event| matches!(event, InternalEvent::Scroll(_))),
                        "Should have Scroll event when scrolling within terminal bounds"
                    );
                    terminal.sync(window, cx);
                });
            };

            for _ in 0..20000 {
                scroll_by(1);
                scroll_by(-1);
            }
        }
    }
}
