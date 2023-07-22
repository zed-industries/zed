pub mod mappings;
pub use alacritty_terminal;

use alacritty_terminal::{
    ansi::{ClearMode, Handler},
    config::{Config, Program, PtyConfig, Scrolling},
    event::{Event as AlacTermEvent, EventListener, Notify, WindowSize},
    event_loop::{EventLoop, Msg, Notifier},
    grid::{Dimensions, Scroll as AlacScroll},
    index::{Column, Direction as AlacDirection, Line, Point},
    selection::{Selection, SelectionRange, SelectionType},
    sync::FairMutex,
    term::{
        cell::Cell,
        color::Rgb,
        search::{Match, RegexIter, RegexSearch},
        RenderableCursor, TermMode,
    },
    tty::{self, setup_env},
    Term,
};
use anyhow::{bail, Result};

use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    FutureExt,
};

use mappings::mouse::{
    alt_scroll, grid_point, mouse_button_report, mouse_moved_report, mouse_side, scroll_report,
};

use procinfo::LocalProcessInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::truncate_and_trailoff;

use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
    fmt::Display,
    ops::{Deref, Index, RangeInclusive, Sub},
    os::unix::prelude::AsRawFd,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;

use gpui::{
    fonts,
    geometry::vector::{vec2f, Vector2F},
    keymap_matcher::Keystroke,
    platform::{Modifiers, MouseButton, MouseMovedEvent, TouchPhase},
    scene::{MouseDown, MouseDrag, MouseScrollWheel, MouseUp},
    AppContext, ClipboardItem, Entity, ModelContext, Task,
};

use crate::mappings::{
    colors::{get_color_at_index, to_alac_rgb},
    keys::to_esc_str,
};
use lazy_static::lazy_static;

///Scrolling is unbearably sluggish by default. Alacritty supports a configurable
///Scroll multiplier that is set to 3 by default. This will be removed when I
///Implement scroll bars.
const SCROLL_MULTIPLIER: f32 = 4.;
const MAX_SEARCH_LINES: usize = 100;
const DEBUG_TERMINAL_WIDTH: f32 = 500.;
const DEBUG_TERMINAL_HEIGHT: f32 = 30.;
const DEBUG_CELL_WIDTH: f32 = 5.;
const DEBUG_LINE_HEIGHT: f32 = 5.;

lazy_static! {
    // Regex Copied from alacritty's ui_config.rs and modified its declaration slightly:
    // * avoid Rust-specific escaping.
    // * use more strict regex for `file://` protocol matching: original regex has `file:` inside, but we want to avoid matching `some::file::module` strings.
    static ref URL_REGEX: RegexSearch = RegexSearch::new(r#"(ipfs:|ipns:|magnet:|mailto:|gemini://|gopher://|https://|http://|news:|file://|git://|ssh:|ftp://)[^\u{0000}-\u{001F}\u{007F}-\u{009F}<>"\s{-}\^⟨⟩`]+"#).unwrap();

    static ref WORD_REGEX: RegexSearch = RegexSearch::new("[\\w.:/@-~]+").unwrap();
}

///Upward flowing events, for changing the title and such
#[derive(Clone, Debug)]
pub enum Event {
    TitleChanged,
    BreadcrumbsChanged,
    CloseTerminal,
    Bell,
    Wakeup,
    BlinkChanged,
    SelectionsChanged,
    NewNavigationTarget(Option<MaybeNavigationTarget>),
    Open(MaybeNavigationTarget),
}

/// A string inside terminal, potentially useful as a URI that can be opened.
#[derive(Clone, Debug)]
pub enum MaybeNavigationTarget {
    /// HTTP, git, etc. string determined by the [`URL_REGEX`] regex.
    Url(String),
    /// File system path, absolute or relative, existing or not.
    /// Might have line and column number(s) attached as `file.rs:1:23`
    PathLike(String),
}

#[derive(Clone)]
enum InternalEvent {
    ColorRequest(usize, Arc<dyn Fn(Rgb) -> String + Sync + Send + 'static>),
    Resize(TerminalSize),
    Clear,
    // FocusNextMatch,
    Scroll(AlacScroll),
    ScrollToPoint(Point),
    SetSelection(Option<(Selection, Point)>),
    UpdateSelection(Vector2F),
    // Adjusted mouse position, should open
    FindHyperlink(Vector2F, bool),
    Copy,
}

///A translation struct for Alacritty to communicate with us from their event loop
#[derive(Clone)]
pub struct ZedListener(UnboundedSender<AlacTermEvent>);

impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(event).ok();
    }
}

pub fn init(cx: &mut AppContext) {
    settings::register::<TerminalSettings>(cx);
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalDockPosition {
    Left,
    Bottom,
    Right,
}

#[derive(Deserialize)]
pub struct TerminalSettings {
    pub shell: Shell,
    pub working_directory: WorkingDirectory,
    font_size: Option<f32>,
    pub font_family: Option<String>,
    pub line_height: TerminalLineHeight,
    pub font_features: Option<fonts::Features>,
    pub env: HashMap<String, String>,
    pub blinking: TerminalBlink,
    pub alternate_scroll: AlternateScroll,
    pub option_as_meta: bool,
    pub copy_on_select: bool,
    pub dock: TerminalDockPosition,
    pub default_width: f32,
    pub default_height: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct TerminalSettingsContent {
    pub shell: Option<Shell>,
    pub working_directory: Option<WorkingDirectory>,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
    pub line_height: Option<TerminalLineHeight>,
    pub font_features: Option<fonts::Features>,
    pub env: Option<HashMap<String, String>>,
    pub blinking: Option<TerminalBlink>,
    pub alternate_scroll: Option<AlternateScroll>,
    pub option_as_meta: Option<bool>,
    pub copy_on_select: Option<bool>,
    pub dock: Option<TerminalDockPosition>,
    pub default_width: Option<f32>,
    pub default_height: Option<f32>,
}

impl TerminalSettings {
    pub fn font_size(&self, cx: &AppContext) -> Option<f32> {
        self.font_size
            .map(|size| theme::adjusted_font_size(size, cx))
    }
}

impl settings::Setting for TerminalSettings {
    const KEY: Option<&'static str> = Some("terminal");

    type FileContent = TerminalSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &AppContext,
    ) -> Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum TerminalLineHeight {
    #[default]
    Comfortable,
    Standard,
    Custom(f32),
}

impl TerminalLineHeight {
    pub fn value(&self) -> f32 {
        match self {
            TerminalLineHeight::Comfortable => 1.618,
            TerminalLineHeight::Standard => 1.3,
            TerminalLineHeight::Custom(line_height) => f32::max(*line_height, 1.),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerminalBlink {
    Off,
    TerminalControlled,
    On,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Shell {
    System,
    Program(String),
    WithArguments { program: String, args: Vec<String> },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlternateScroll {
    On,
    Off,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkingDirectory {
    CurrentProjectDirectory,
    FirstProjectDirectory,
    AlwaysHome,
    Always { directory: String },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TerminalSize {
    pub cell_width: f32,
    pub line_height: f32,
    pub height: f32,
    pub width: f32,
}

impl TerminalSize {
    pub fn new(line_height: f32, cell_width: f32, size: Vector2F) -> Self {
        TerminalSize {
            cell_width,
            line_height,
            width: size.x(),
            height: size.y(),
        }
    }

    pub fn num_lines(&self) -> usize {
        (self.height / self.line_height).floor() as usize
    }

    pub fn num_columns(&self) -> usize {
        (self.width / self.cell_width).floor() as usize
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }

    pub fn line_height(&self) -> f32 {
        self.line_height
    }
}
impl Default for TerminalSize {
    fn default() -> Self {
        TerminalSize::new(
            DEBUG_LINE_HEIGHT,
            DEBUG_CELL_WIDTH,
            vec2f(DEBUG_TERMINAL_WIDTH, DEBUG_TERMINAL_HEIGHT),
        )
    }
}

impl From<TerminalSize> for WindowSize {
    fn from(val: TerminalSize) -> Self {
        WindowSize {
            num_lines: val.num_lines() as u16,
            num_cols: val.num_columns() as u16,
            cell_width: val.cell_width() as u16,
            cell_height: val.line_height() as u16,
        }
    }
}

impl Dimensions for TerminalSize {
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

    pub fn shell_to_string(&self) -> String {
        match &self.shell {
            Shell::System => "<system shell>".to_string(),
            Shell::Program(p) => p.to_string(),
            Shell::WithArguments { program, args } => format!("{} {}", program, args.join(" ")),
        }
    }

    pub fn fmt_shell(&self) -> String {
        match &self.shell {
            Shell::System => "<system defined shell>".to_string(),
            Shell::Program(s) => s.to_string(),
            Shell::WithArguments { program, args } => format!("{} {}", program, args.join(" ")),
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

pub struct TerminalBuilder {
    terminal: Terminal,
    events_rx: UnboundedReceiver<AlacTermEvent>,
}

impl TerminalBuilder {
    pub fn new(
        working_directory: Option<PathBuf>,
        shell: Shell,
        mut env: HashMap<String, String>,
        blink_settings: Option<TerminalBlink>,
        alternate_scroll: AlternateScroll,
        window_id: usize,
    ) -> Result<TerminalBuilder> {
        let pty_config = {
            let alac_shell = match shell.clone() {
                Shell::System => None,
                Shell::Program(program) => Some(Program::Just(program)),
                Shell::WithArguments { program, args } => Some(Program::WithArgs { program, args }),
            };

            PtyConfig {
                shell: alac_shell,
                working_directory: working_directory.clone(),
                hold: false,
            }
        };

        //TODO: Properly set the current locale,
        env.insert("LC_ALL".to_string(), "en_US.UTF-8".to_string());
        env.insert("ZED_TERM".to_string(), true.to_string());

        let alac_scrolling = Scrolling::default();
        // alac_scrolling.set_history((BACK_BUFFER_SIZE * 2) as u32);

        let config = Config {
            pty_config: pty_config.clone(),
            env,
            scrolling: alac_scrolling,
            ..Default::default()
        };

        setup_env(&config);

        //Spawn a task so the Alacritty EventLoop can communicate with us in a view context
        //TODO: Remove with a bounded sender which can be dispatched on &self
        let (events_tx, events_rx) = unbounded();
        //Set up the terminal...
        let mut term = Term::new(
            &config,
            &TerminalSize::default(),
            ZedListener(events_tx.clone()),
        );

        //Start off blinking if we need to
        if let Some(TerminalBlink::On) = blink_settings {
            term.set_mode(alacritty_terminal::ansi::Mode::BlinkingCursor)
        }

        //Alacritty defaults to alternate scrolling being on, so we just need to turn it off.
        if let AlternateScroll::Off = alternate_scroll {
            term.unset_mode(alacritty_terminal::ansi::Mode::AlternateScroll)
        }

        let term = Arc::new(FairMutex::new(term));

        //Setup the pty...
        let pty = match tty::new(
            &pty_config,
            TerminalSize::default().into(),
            window_id as u64,
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

        let fd = pty.file().as_raw_fd();
        let shell_pid = pty.child().id();

        //And connect them together
        let event_loop = EventLoop::new(
            term.clone(),
            ZedListener(events_tx.clone()),
            pty,
            pty_config.hold,
            false,
        );

        //Kick things off
        let pty_tx = event_loop.channel();
        let _io_thread = event_loop.spawn();

        let terminal = Terminal {
            pty_tx: Notifier(pty_tx),
            term,
            events: VecDeque::with_capacity(10), //Should never get this high.
            last_content: Default::default(),
            last_mouse: None,
            matches: Vec::new(),
            last_synced: Instant::now(),
            sync_task: None,
            selection_head: None,
            shell_fd: fd as u32,
            shell_pid,
            foreground_process_info: None,
            breadcrumb_text: String::new(),
            scroll_px: 0.,
            last_mouse_position: None,
            next_link_id: 0,
            selection_phase: SelectionPhase::Ended,
            cmd_pressed: false,
            hovered_word: false,
        };

        Ok(TerminalBuilder {
            terminal,
            events_rx,
        })
    }

    pub fn subscribe(mut self, cx: &mut ModelContext<Terminal>) -> Terminal {
        //Event loop
        cx.spawn_weak(|this, mut cx| async move {
            use futures::StreamExt;

            while let Some(event) = self.events_rx.next().await {
                this.upgrade(&cx)?.update(&mut cx, |this, cx| {
                    //Process the first event immediately for lowered latency
                    this.process_event(&event, cx);
                });

                'outer: loop {
                    let mut events = vec![];
                    let mut timer = cx.background().timer(Duration::from_millis(4)).fuse();
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

                    if events.is_empty() && wakeup == false {
                        smol::future::yield_now().await;
                        break 'outer;
                    } else {
                        this.upgrade(&cx)?.update(&mut cx, |this, cx| {
                            if wakeup {
                                this.process_event(&AlacTermEvent::Wakeup, cx);
                            }

                            for event in events {
                                this.process_event(&event, cx);
                            }
                        });
                        smol::future::yield_now().await;
                    }
                }
            }

            Some(())
        })
        .detach();

        self.terminal
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexedCell {
    pub point: Point,
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
    pub size: TerminalSize,
    pub last_hovered_word: Option<HoveredWord>,
}

#[derive(Clone)]
pub struct HoveredWord {
    pub word: String,
    pub word_match: RangeInclusive<Point>,
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
                shape: alacritty_terminal::ansi::CursorShape::Block,
                point: Point::new(Line(0), Column(0)),
            },
            cursor_char: Default::default(),
            size: Default::default(),
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
    term: Arc<FairMutex<Term<ZedListener>>>,
    events: VecDeque<InternalEvent>,
    /// This is only used for mouse mode cell change detection
    last_mouse: Option<(Point, AlacDirection)>,
    /// This is only used for terminal hovered word checking
    last_mouse_position: Option<Vector2F>,
    pub matches: Vec<RangeInclusive<Point>>,
    pub last_content: TerminalContent,
    last_synced: Instant,
    sync_task: Option<Task<()>>,
    pub selection_head: Option<Point>,
    pub breadcrumb_text: String,
    shell_pid: u32,
    shell_fd: u32,
    pub foreground_process_info: Option<LocalProcessInfo>,
    scroll_px: f32,
    next_link_id: usize,
    selection_phase: SelectionPhase,
    cmd_pressed: bool,
    hovered_word: bool,
}

impl Terminal {
    fn process_event(&mut self, event: &AlacTermEvent, cx: &mut ModelContext<Self>) {
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
                cx.write_to_clipboard(ClipboardItem::new(data.to_string()))
            }
            AlacTermEvent::ClipboardLoad(_, format) => self.write_to_pty(format(
                &cx.read_from_clipboard()
                    .map(|ci| ci.text().to_string())
                    .unwrap_or_else(|| "".to_string()),
            )),
            AlacTermEvent::PtyWrite(out) => self.write_to_pty(out.clone()),
            AlacTermEvent::TextAreaSizeRequest(format) => {
                self.write_to_pty(format(self.last_content.size.into()))
            }
            AlacTermEvent::CursorBlinkingChange => {
                cx.emit(Event::BlinkChanged);
            }
            AlacTermEvent::Bell => {
                cx.emit(Event::Bell);
            }
            AlacTermEvent::Exit => cx.emit(Event::CloseTerminal),
            AlacTermEvent::MouseCursorDirty => {
                //NOOP, Handled in render
            }
            AlacTermEvent::Wakeup => {
                cx.emit(Event::Wakeup);

                if self.update_process_info() {
                    cx.emit(Event::TitleChanged);
                }
            }
            AlacTermEvent::ColorRequest(idx, fun_ptr) => {
                self.events
                    .push_back(InternalEvent::ColorRequest(*idx, fun_ptr.clone()));
            }
        }
    }

    /// Update the cached process info, returns whether the Zed-relevant info has changed
    fn update_process_info(&mut self) -> bool {
        let mut pid = unsafe { libc::tcgetpgrp(self.shell_fd as i32) };
        if pid < 0 {
            pid = self.shell_pid as i32;
        }

        if let Some(process_info) = LocalProcessInfo::with_root_pid(pid as u32) {
            let res = self
                .foreground_process_info
                .as_ref()
                .map(|old_info| {
                    process_info.cwd != old_info.cwd || process_info.name != old_info.name
                })
                .unwrap_or(true);

            self.foreground_process_info = Some(process_info.clone());

            res
        } else {
            false
        }
    }

    ///Takes events from Alacritty and translates them to behavior on this view
    fn process_terminal_event(
        &mut self,
        event: &InternalEvent,
        term: &mut Term<ZedListener>,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            InternalEvent::ColorRequest(index, format) => {
                let color = term.colors()[*index].unwrap_or_else(|| {
                    let term_style = &theme::current(cx).terminal;
                    to_alac_rgb(get_color_at_index(index, &term_style))
                });
                self.write_to_pty(format(color))
            }
            InternalEvent::Resize(mut new_size) => {
                new_size.height = f32::max(new_size.line_height, new_size.height);
                new_size.width = f32::max(new_size.cell_width, new_size.width);

                self.last_content.size = new_size.clone();

                self.pty_tx.0.send(Msg::Resize((new_size).into())).ok();

                term.resize(new_size);
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
                    Point::new(Line(0), term.grid_mut().cursor.point.column);
                let new_cursor = term.grid().cursor.point;

                // Clear the lines below the new cursor
                if (new_cursor.line.0 as usize) < term.screen_lines() - 1 {
                    term.grid_mut().reset_region((new_cursor.line + 1)..);
                }

                cx.emit(Event::Wakeup);
            }
            InternalEvent::Scroll(scroll) => {
                term.scroll_display(*scroll);
                self.refresh_hovered_word();
            }
            InternalEvent::SetSelection(selection) => {
                term.selection = selection.as_ref().map(|(sel, _)| sel.clone());

                if let Some((_, head)) = selection {
                    self.selection_head = Some(*head);
                }
                cx.emit(Event::SelectionsChanged)
            }
            InternalEvent::UpdateSelection(position) => {
                if let Some(mut selection) = term.selection.take() {
                    let point = grid_point(
                        *position,
                        self.last_content.size,
                        term.grid().display_offset(),
                    );

                    let side = mouse_side(*position, self.last_content.size);

                    selection.update(point, side);
                    term.selection = Some(selection);

                    self.selection_head = Some(point);
                    cx.emit(Event::SelectionsChanged)
                }
            }

            InternalEvent::Copy => {
                if let Some(txt) = term.selection_to_string() {
                    cx.write_to_clipboard(ClipboardItem::new(txt))
                }
            }
            InternalEvent::ScrollToPoint(point) => {
                term.scroll_to_point(*point);
                self.refresh_hovered_word();
            }
            InternalEvent::FindHyperlink(position, open) => {
                let prev_hovered_word = self.last_content.last_hovered_word.take();

                let point = grid_point(
                    *position,
                    self.last_content.size,
                    term.grid().display_offset(),
                )
                .grid_clamp(term, alacritty_terminal::index::Boundary::Grid);

                let link = term.grid().index(point).hyperlink();
                let found_word = if link.is_some() {
                    let mut min_index = point;
                    loop {
                        let new_min_index =
                            min_index.sub(term, alacritty_terminal::index::Boundary::Cursor, 1);
                        if new_min_index == min_index {
                            break;
                        } else if term.grid().index(new_min_index).hyperlink() != link {
                            break;
                        } else {
                            min_index = new_min_index
                        }
                    }

                    let mut max_index = point;
                    loop {
                        let new_max_index =
                            max_index.add(term, alacritty_terminal::index::Boundary::Cursor, 1);
                        if new_max_index == max_index {
                            break;
                        } else if term.grid().index(new_max_index).hyperlink() != link {
                            break;
                        } else {
                            max_index = new_max_index
                        }
                    }

                    let url = link.unwrap().uri().to_owned();
                    let url_match = min_index..=max_index;

                    Some((url, true, url_match))
                } else if let Some(word_match) = regex_match_at(term, point, &WORD_REGEX) {
                    let maybe_url_or_path =
                        term.bounds_to_string(*word_match.start(), *word_match.end());
                    let is_url = match regex_match_at(term, point, &URL_REGEX) {
                        Some(url_match) => url_match == word_match,
                        None => false,
                    };
                    Some((maybe_url_or_path, is_url, word_match))
                } else {
                    None
                };

                match found_word {
                    Some((maybe_url_or_path, is_url, url_match)) => {
                        if *open {
                            let target = if is_url {
                                MaybeNavigationTarget::Url(maybe_url_or_path)
                            } else {
                                MaybeNavigationTarget::PathLike(maybe_url_or_path)
                            };
                            cx.emit(Event::Open(target));
                        } else {
                            self.update_selected_word(
                                prev_hovered_word,
                                url_match,
                                maybe_url_or_path,
                                is_url,
                                cx,
                            );
                        }
                        self.hovered_word = true;
                    }
                    None => {
                        if self.hovered_word {
                            cx.emit(Event::NewNavigationTarget(None));
                        }
                        self.hovered_word = false;
                    }
                }
            }
        }
    }

    fn update_selected_word(
        &mut self,
        prev_word: Option<HoveredWord>,
        word_match: RangeInclusive<Point>,
        word: String,
        is_url: bool,
        cx: &mut ModelContext<Self>,
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
        let navigation_target = if is_url {
            MaybeNavigationTarget::Url(word)
        } else {
            MaybeNavigationTarget::PathLike(word)
        };
        cx.emit(Event::NewNavigationTarget(Some(navigation_target)));
    }

    fn next_link_id(&mut self) -> usize {
        let res = self.next_link_id;
        self.next_link_id = self.next_link_id.wrapping_add(1);
        res
    }

    pub fn last_content(&self) -> &TerminalContent {
        &self.last_content
    }

    //To test:
    //- Activate match on terminal (scrolling and selection)
    //- Editor search snapping behavior

    pub fn activate_match(&mut self, index: usize) {
        if let Some(search_match) = self.matches.get(index).cloned() {
            self.set_selection(Some((make_selection(&search_match), *search_match.end())));

            self.events
                .push_back(InternalEvent::ScrollToPoint(*search_match.start()));
        }
    }

    pub fn select_matches(&mut self, matches: Vec<RangeInclusive<Point>>) {
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

    fn set_selection(&mut self, selection: Option<(Selection, Point)>) {
        self.events
            .push_back(InternalEvent::SetSelection(selection));
    }

    pub fn copy(&mut self) {
        self.events.push_back(InternalEvent::Copy);
    }

    pub fn clear(&mut self) {
        self.events.push_back(InternalEvent::Clear)
    }

    ///Resize the terminal and the PTY.
    pub fn set_size(&mut self, new_size: TerminalSize) {
        self.events.push_back(InternalEvent::Resize(new_size))
    }

    ///Write the Input payload to the tty.
    fn write_to_pty(&self, input: String) {
        self.pty_tx.notify(input.into_bytes());
    }

    pub fn input(&mut self, input: String) {
        self.events
            .push_back(InternalEvent::Scroll(AlacScroll::Bottom));
        self.events.push_back(InternalEvent::SetSelection(None));

        self.write_to_pty(input);
    }

    pub fn try_keystroke(&mut self, keystroke: &Keystroke, alt_is_meta: bool) -> bool {
        let esc = to_esc_str(keystroke, &self.last_content.mode, alt_is_meta);
        if let Some(esc) = esc {
            self.input(esc);
            true
        } else {
            false
        }
    }

    pub fn try_modifiers_change(&mut self, modifiers: &Modifiers) -> bool {
        let changed = self.cmd_pressed != modifiers.cmd;
        if !self.cmd_pressed && modifiers.cmd {
            self.refresh_hovered_word();
        }
        self.cmd_pressed = modifiers.cmd;
        changed
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

    pub fn try_sync(&mut self, cx: &mut ModelContext<Self>) {
        let term = self.term.clone();

        let mut terminal = if let Some(term) = term.try_lock_unfair() {
            term
        } else if self.last_synced.elapsed().as_secs_f32() > 0.25 {
            term.lock_unfair() //It's been too long, force block
        } else if let None = self.sync_task {
            //Skip this frame
            let delay = cx.background().timer(Duration::from_millis(16));
            self.sync_task = Some(cx.spawn_weak(|weak_handle, mut cx| async move {
                delay.await;
                cx.update(|cx| {
                    if let Some(handle) = weak_handle.upgrade(cx) {
                        handle.update(cx, |terminal, cx| {
                            terminal.sync_task.take();
                            cx.notify();
                        });
                    }
                });
            }));
            return;
        } else {
            //No lock and delayed rendering already scheduled, nothing to do
            return;
        };

        //Note that the ordering of events matters for event processing
        while let Some(e) = self.events.pop_front() {
            self.process_terminal_event(&e, &mut terminal, cx)
        }

        self.last_content = Self::make_content(&terminal, &self.last_content);
        self.last_synced = Instant::now();
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
            size: last_content.size,
            last_hovered_word: last_content.last_hovered_word.clone(),
        }
    }

    pub fn focus_in(&self) {
        if self.last_content.mode.contains(TermMode::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[I".to_string());
        }
    }

    pub fn focus_out(&mut self) {
        self.last_mouse_position = None;
        if self.last_content.mode.contains(TermMode::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[O".to_string());
        }
    }

    pub fn mouse_changed(&mut self, point: Point, side: AlacDirection) -> bool {
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

    pub fn mouse_move(&mut self, e: &MouseMovedEvent, origin: Vector2F) {
        let position = e.position.sub(origin);
        self.last_mouse_position = Some(position);
        if self.mouse_mode(e.shift) {
            let point = grid_point(
                position,
                self.last_content.size,
                self.last_content.display_offset,
            );
            let side = mouse_side(position, self.last_content.size);

            if self.mouse_changed(point, side) {
                if let Some(bytes) = mouse_moved_report(point, e, self.last_content.mode) {
                    self.pty_tx.notify(bytes);
                }
            }
        } else if self.cmd_pressed {
            self.word_from_position(Some(position));
        }
    }

    fn word_from_position(&mut self, position: Option<Vector2F>) {
        if self.selection_phase == SelectionPhase::Selecting {
            self.last_content.last_hovered_word = None;
        } else if let Some(position) = position {
            self.events
                .push_back(InternalEvent::FindHyperlink(position, false));
        }
    }

    pub fn mouse_drag(&mut self, e: MouseDrag, origin: Vector2F) {
        let position = e.position.sub(origin);
        self.last_mouse_position = Some(position);

        if !self.mouse_mode(e.shift) {
            self.selection_phase = SelectionPhase::Selecting;
            // Alacritty has the same ordering, of first updating the selection
            // then scrolling 15ms later
            self.events
                .push_back(InternalEvent::UpdateSelection(position));

            // Doesn't make sense to scroll the alt screen
            if !self.last_content.mode.contains(TermMode::ALT_SCREEN) {
                let scroll_delta = match self.drag_line_delta(e) {
                    Some(value) => value,
                    None => return,
                };

                let scroll_lines = (scroll_delta / self.last_content.size.line_height) as i32;

                self.events
                    .push_back(InternalEvent::Scroll(AlacScroll::Delta(scroll_lines)));
            }
        }
    }

    fn drag_line_delta(&mut self, e: MouseDrag) -> Option<f32> {
        //TODO: Why do these need to be doubled? Probably the same problem that the IME has
        let top = e.region.origin_y() + (self.last_content.size.line_height * 2.);
        let bottom = e.region.lower_left().y() - (self.last_content.size.line_height * 2.);
        let scroll_delta = if e.position.y() < top {
            (top - e.position.y()).powf(1.1)
        } else if e.position.y() > bottom {
            -((e.position.y() - bottom).powf(1.1))
        } else {
            return None; //Nothing to do
        };
        Some(scroll_delta)
    }

    pub fn mouse_down(&mut self, e: &MouseDown, origin: Vector2F) {
        let position = e.position.sub(origin);
        let point = grid_point(
            position,
            self.last_content.size,
            self.last_content.display_offset,
        );

        if self.mouse_mode(e.shift) {
            if let Some(bytes) = mouse_button_report(point, e, true, self.last_content.mode) {
                self.pty_tx.notify(bytes);
            }
        } else if e.button == MouseButton::Left {
            let position = e.position.sub(origin);
            let point = grid_point(
                position,
                self.last_content.size,
                self.last_content.display_offset,
            );

            // Use .opposite so that selection is inclusive of the cell clicked.
            let side = mouse_side(position, self.last_content.size);

            let selection_type = match e.click_count {
                0 => return, //This is a release
                1 => Some(SelectionType::Simple),
                2 => Some(SelectionType::Semantic),
                3 => Some(SelectionType::Lines),
                _ => None,
            };

            let selection =
                selection_type.map(|selection_type| Selection::new(selection_type, point, side));

            if let Some(sel) = selection {
                self.events
                    .push_back(InternalEvent::SetSelection(Some((sel, point))));
            }
        }
    }

    pub fn mouse_up(&mut self, e: &MouseUp, origin: Vector2F, cx: &mut ModelContext<Self>) {
        let setting = settings::get::<TerminalSettings>(cx);

        let position = e.position.sub(origin);
        if self.mouse_mode(e.shift) {
            let point = grid_point(
                position,
                self.last_content.size,
                self.last_content.display_offset,
            );

            if let Some(bytes) = mouse_button_report(point, e, false, self.last_content.mode) {
                self.pty_tx.notify(bytes);
            }
        } else {
            if e.button == MouseButton::Left && setting.copy_on_select {
                self.copy();
            }

            //Hyperlinks
            if self.selection_phase == SelectionPhase::Ended {
                let mouse_cell_index = content_index_for_mouse(position, &self.last_content.size);
                if let Some(link) = self.last_content.cells[mouse_cell_index].hyperlink() {
                    cx.platform().open_url(link.uri());
                } else if self.cmd_pressed {
                    self.events
                        .push_back(InternalEvent::FindHyperlink(position, true));
                }
            }
        }

        self.selection_phase = SelectionPhase::Ended;
        self.last_mouse = None;
    }

    ///Scroll the terminal
    pub fn scroll_wheel(&mut self, e: MouseScrollWheel, origin: Vector2F) {
        let mouse_mode = self.mouse_mode(e.shift);

        if let Some(scroll_lines) = self.determine_scroll_lines(&e, mouse_mode) {
            if mouse_mode {
                let point = grid_point(
                    e.position.sub(origin),
                    self.last_content.size,
                    self.last_content.display_offset,
                );

                if let Some(scrolls) =
                    scroll_report(point, scroll_lines as i32, &e, self.last_content.mode)
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
            } else {
                if scroll_lines != 0 {
                    let scroll = AlacScroll::Delta(scroll_lines);

                    self.events.push_back(InternalEvent::Scroll(scroll));
                }
            }
        }
    }

    fn refresh_hovered_word(&mut self) {
        self.word_from_position(self.last_mouse_position);
    }

    fn determine_scroll_lines(&mut self, e: &MouseScrollWheel, mouse_mode: bool) -> Option<i32> {
        let scroll_multiplier = if mouse_mode { 1. } else { SCROLL_MULTIPLIER };
        let line_height = self.last_content.size.line_height;
        match e.phase {
            /* Reset scroll state on started */
            Some(TouchPhase::Started) => {
                self.scroll_px = 0.;
                None
            }
            /* Calculate the appropriate scroll lines */
            Some(gpui::platform::TouchPhase::Moved) => {
                let old_offset = (self.scroll_px / line_height) as i32;

                self.scroll_px += e.delta.pixel_delta(line_height).y() * scroll_multiplier;

                let new_offset = (self.scroll_px / line_height) as i32;

                // Whenever we hit the edges, reset our stored scroll to 0
                // so we can respond to changes in direction quickly
                self.scroll_px %= self.last_content.size.height;

                Some(new_offset - old_offset)
            }
            /* Fall back to delta / line_height */
            None => Some(
                ((e.delta.pixel_delta(line_height).y() * scroll_multiplier) / line_height) as i32,
            ),
            _ => None,
        }
    }

    pub fn find_matches(
        &mut self,
        searcher: RegexSearch,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<RangeInclusive<Point>>> {
        let term = self.term.clone();
        cx.background().spawn(async move {
            let term = term.lock();

            all_search_matches(&term, &searcher).collect()
        })
    }

    pub fn title(&self) -> String {
        self.foreground_process_info
            .as_ref()
            .map(|fpi| {
                format!(
                    "{} — {}",
                    truncate_and_trailoff(
                        &fpi.cwd
                            .file_name()
                            .map(|name| name.to_string_lossy().to_string())
                            .unwrap_or_default(),
                        25
                    ),
                    truncate_and_trailoff(
                        &{
                            format!(
                                "{}{}",
                                fpi.name,
                                if fpi.argv.len() >= 1 {
                                    format!(" {}", (&fpi.argv[1..]).join(" "))
                                } else {
                                    "".to_string()
                                }
                            )
                        },
                        25
                    )
                )
            })
            .unwrap_or_else(|| "Terminal".to_string())
    }

    pub fn can_navigate_to_selected_word(&self) -> bool {
        self.cmd_pressed && self.hovered_word
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.pty_tx.0.send(Msg::Shutdown).ok();
    }
}

impl Entity for Terminal {
    type Event = Event;
}

/// Based on alacritty/src/display/hint.rs > regex_match_at
/// Retrieve the match, if the specified point is inside the content matching the regex.
fn regex_match_at<T>(term: &Term<T>, point: Point, regex: &RegexSearch) -> Option<Match> {
    visible_regex_match_iter(term, regex).find(|rm| rm.contains(&point))
}

/// Copied from alacritty/src/display/hint.rs:
/// Iterate over all visible regex matches.
pub fn visible_regex_match_iter<'a, T>(
    term: &'a Term<T>,
    regex: &'a RegexSearch,
) -> impl Iterator<Item = Match> + 'a {
    let viewport_start = Line(-(term.grid().display_offset() as i32));
    let viewport_end = viewport_start + term.bottommost_line();
    let mut start = term.line_search_left(Point::new(viewport_start, Column(0)));
    let mut end = term.line_search_right(Point::new(viewport_end, Column(0)));
    start.line = start.line.max(viewport_start - MAX_SEARCH_LINES);
    end.line = end.line.min(viewport_end + MAX_SEARCH_LINES);

    RegexIter::new(start, end, AlacDirection::Right, term, regex)
        .skip_while(move |rm| rm.end().line < viewport_start)
        .take_while(move |rm| rm.start().line <= viewport_end)
}

fn make_selection(range: &RangeInclusive<Point>) -> Selection {
    let mut selection = Selection::new(SelectionType::Simple, *range.start(), AlacDirection::Left);
    selection.update(*range.end(), AlacDirection::Right);
    selection
}

fn all_search_matches<'a, T>(
    term: &'a Term<T>,
    regex: &'a RegexSearch,
) -> impl Iterator<Item = Match> + 'a {
    let start = Point::new(term.grid().topmost_line(), Column(0));
    let end = Point::new(term.grid().bottommost_line(), term.grid().last_column());
    RegexIter::new(start, end, AlacDirection::Right, term, regex)
}

fn content_index_for_mouse(pos: Vector2F, size: &TerminalSize) -> usize {
    let col = (pos.x() / size.cell_width()).round() as usize;

    let clamped_col = min(col, size.columns() - 1);

    let row = (pos.y() / size.line_height()).round() as usize;

    let clamped_row = min(row, size.screen_lines() - 1);

    clamped_row * size.columns() + clamped_col
}

#[cfg(test)]
mod tests {
    use alacritty_terminal::{
        index::{Column, Line, Point},
        term::cell::Cell,
    };
    use gpui::geometry::vector::vec2f;
    use rand::{distributions::Alphanumeric, rngs::ThreadRng, thread_rng, Rng};

    use crate::{content_index_for_mouse, IndexedCell, TerminalContent, TerminalSize};

    #[test]
    fn test_mouse_to_cell_test() {
        let mut rng = thread_rng();
        const ITERATIONS: usize = 10;
        const PRECISION: usize = 1000;

        for _ in 0..ITERATIONS {
            let viewport_cells = rng.gen_range(15..20);
            let cell_size = rng.gen_range(5 * PRECISION..20 * PRECISION) as f32 / PRECISION as f32;

            let size = crate::TerminalSize {
                cell_width: cell_size,
                line_height: cell_size,
                height: cell_size * (viewport_cells as f32),
                width: cell_size * (viewport_cells as f32),
            };

            let cells = get_cells(size, &mut rng);
            let content = convert_cells_to_content(size, &cells);

            for row in 0..(viewport_cells - 1) {
                let row = row as usize;
                for col in 0..(viewport_cells - 1) {
                    let col = col as usize;

                    let row_offset = rng.gen_range(0..PRECISION) as f32 / PRECISION as f32;
                    let col_offset = rng.gen_range(0..PRECISION) as f32 / PRECISION as f32;

                    let mouse_pos = vec2f(
                        col as f32 * cell_size + col_offset,
                        row as f32 * cell_size + row_offset,
                    );

                    let content_index = content_index_for_mouse(mouse_pos, &content.size);
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

        let size = crate::TerminalSize {
            cell_width: 10.,
            line_height: 10.,
            height: 100.,
            width: 100.,
        };

        let cells = get_cells(size, &mut rng);
        let content = convert_cells_to_content(size, &cells);

        assert_eq!(
            content.cells[content_index_for_mouse(vec2f(-10., -10.), &content.size)].c,
            cells[0][0]
        );
        assert_eq!(
            content.cells[content_index_for_mouse(vec2f(1000., 1000.), &content.size)].c,
            cells[9][9]
        );
    }

    fn get_cells(size: TerminalSize, rng: &mut ThreadRng) -> Vec<Vec<char>> {
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

    fn convert_cells_to_content(size: TerminalSize, cells: &Vec<Vec<char>>) -> TerminalContent {
        let mut ic = Vec::new();

        for row in 0..cells.len() {
            for col in 0..cells[row].len() {
                let cell_char = cells[row][col];
                ic.push(IndexedCell {
                    point: Point::new(Line(row as i32), Column(col)),
                    cell: Cell {
                        c: cell_char,
                        ..Default::default()
                    },
                });
            }
        }

        TerminalContent {
            cells: ic,
            size,
            ..Default::default()
        }
    }
}
