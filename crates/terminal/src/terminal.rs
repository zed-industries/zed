#![cfg_attr(
    not(any(feature = "alacritty-backend", feature = "libghostty-vt")),
    allow(unused)
)]

pub mod mappings;

#[cfg(feature = "libghostty-vt")]
mod ghostty_backend;
#[cfg(feature = "libghostty-vt")]
mod ghostty_pty;
mod pty_info;
mod terminal_hyperlinks;
pub mod terminal_settings;

#[cfg(feature = "libghostty-vt")]
use ghostty_backend::{GhosttyBackend, GhosttyOsc52};
#[cfg(feature = "libghostty-vt")]
use ghostty_pty::{GhosttyPtyEventLoop, GhosttyPtyNotifier, portable_pty_size};

#[cfg(feature = "alacritty-backend")]
use alacritty_terminal::event::Notify;
#[cfg(feature = "alacritty-backend")]
use alacritty_terminal::event_loop::{EventLoop, Msg, Notifier};
#[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
use alacritty_terminal::term::Osc52 as AlacOsc52;
#[cfg(feature = "alacritty-backend")]
use alacritty_terminal::{
    Term,
    event::{Event as AlacTermEvent, EventListener, WindowSize},
    grid::{Dimensions, Grid, Row, Scroll as AlacScroll},
    index::{Boundary, Column, Direction as AlacDirection, Line, Point as AlacPoint},
    selection::{Selection, SelectionRange, SelectionType as AlacSelectionType},
    sync::FairMutex,
    term::{
        Config, RenderableCursor, TermMode,
        cell::{Cell as AlacCell, Flags, Hyperlink as AlacHyperlink},
        search::{Match, RegexIter, RegexSearch},
    },
    tty::{self},
    vi_mode::{ViModeCursor, ViMotion},
    vte::ansi::{
        ClearMode, CursorShape as AlacCursorShape, CursorStyle as AlacCursorStyle, Handler,
        NamedPrivateMode, PrivateMode,
    },
};
use anyhow::{Context as _, Result, bail};
use futures_lite::future::yield_now;
#[cfg(feature = "alacritty-backend")]
use log::trace;

#[cfg(feature = "alacritty-backend")]
use futures::channel::mpsc::UnboundedSender;
use futures::{
    FutureExt,
    channel::mpsc::{UnboundedReceiver, unbounded},
};

use itertools::Itertools as _;
use mappings::mouse::{
    alt_scroll, grid_point, grid_point_and_side, mouse_button_report, mouse_moved_report,
    scroll_report,
};

use async_channel::{Receiver, Sender};
use collections::{HashMap, VecDeque};
#[cfg(all(
    any(test, feature = "test-support"),
    feature = "libghostty-vt",
    feature = "alacritty-backend"
))]
use feature_flags::{FeatureFlag as _, FeatureFlagStore};
#[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
use feature_flags::{FeatureFlagAppExt as _, GhosttyTerminalFeatureFlag};
use futures::StreamExt;
use pty_info::{ProcessIdGetter, PtyProcessInfo};
use regex::Regex;
#[cfg(feature = "libghostty-vt")]
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use settings::Settings;
use task::{HideStrategy, Shell, SpawnInTerminal};
use terminal_hyperlinks::{HyperlinkMatch, RegexSearches};
use terminal_settings::{AlternateScroll, CursorShape, TerminalSettings};
use theme::{ActiveTheme, Theme};
#[cfg(feature = "libghostty-vt")]
use theme::{Appearance, GlobalTheme};
use urlencoding;
use util::{paths::PathStyle, truncate_and_trailoff};

#[cfg(feature = "alacritty-backend")]
use std::ops::RangeInclusive;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::{
    borrow::Cow,
    cmp::{self, min},
    fmt::{self, Display, Formatter},
    ops::{BitOr, BitOrAssign, Deref},
    path::PathBuf,
    process::ExitStatus,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use vte::ansi::{Color as VteColor, NamedColor as VteNamedColor, Rgb as VteRgb};

use gpui::{
    App, AppContext as _, BackgroundExecutor, Bounds, ClipboardItem, Context, EventEmitter, Hsla,
    Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point,
    Rgba, ScrollWheelEvent, Size, Task, TouchPhase, Window, actions, black, px,
};

#[cfg(feature = "alacritty-backend")]
use crate::mappings::colors::to_vte_rgb;
use crate::mappings::keys::to_esc_str;

actions!(
    terminal,
    [
        /// Clears the terminal screen.
        Clear,
        /// Copies selected text to the clipboard.
        Copy,
        /// Pastes from the clipboard.
        Paste,
        /// Pastes the text from the clipboard.
        PasteText,
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
    Scroll(TerminalScroll),
    ScrollToPoint(TerminalPoint),
    SetSelection(Option<TerminalSelection>),
    UpdateSelection(Point<Pixels>),
    FindHyperlink(Point<Pixels>, bool),
    ProcessHyperlink(HyperlinkMatch, bool),
    // Whether keep selection when copy
    Copy(Option<bool>),
    // Vi mode events
    ToggleViMode,
    ViMotion(TerminalViMotion),
    MoveViCursorToPoint(TerminalPoint),
}

#[derive(Clone, Copy, Debug)]
enum TerminalScroll {
    Delta(i32),
    PageUp,
    PageDown,
    Top,
    Bottom,
}

impl TerminalScroll {
    #[cfg(feature = "alacritty-backend")]
    fn to_alacritty(self) -> AlacScroll {
        match self {
            Self::Delta(delta) => AlacScroll::Delta(delta),
            Self::PageUp => AlacScroll::PageUp,
            Self::PageDown => AlacScroll::PageDown,
            Self::Top => AlacScroll::Top,
            Self::Bottom => AlacScroll::Bottom,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum TerminalViMotion {
    Up,
    Down,
    Left,
    Right,
    First,
    Last,
    FirstOccupied,
    High,
    Middle,
    Low,
    WordLeft,
    WordRight,
    WordRightEnd,
    Bracket,
}

impl TerminalViMotion {
    #[cfg(feature = "alacritty-backend")]
    fn to_alacritty(self) -> ViMotion {
        match self {
            Self::Up => ViMotion::Up,
            Self::Down => ViMotion::Down,
            Self::Left => ViMotion::Left,
            Self::Right => ViMotion::Right,
            Self::First => ViMotion::First,
            Self::Last => ViMotion::Last,
            Self::FirstOccupied => ViMotion::FirstOccupied,
            Self::High => ViMotion::High,
            Self::Middle => ViMotion::Middle,
            Self::Low => ViMotion::Low,
            Self::WordLeft => ViMotion::WordLeft,
            Self::WordRight => ViMotion::WordRight,
            Self::WordRightEnd => ViMotion::WordRightEnd,
            Self::Bracket => ViMotion::Bracket,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TerminalSearch {
    pattern: Arc<str>,
}

impl TerminalSearch {
    pub fn new(search: &str) -> Option<Self> {
        Regex::new(search).ok()?;

        Some(Self {
            pattern: Arc::from(search),
        })
    }

    #[cfg(feature = "alacritty-backend")]
    fn compile_alacritty(search: &str) -> Option<RegexSearch> {
        RegexSearch::new(search).ok()
    }

    #[cfg(feature = "alacritty-backend")]
    fn alacritty(&self) -> Option<RegexSearch> {
        Self::compile_alacritty(self.pattern.as_ref())
    }

    #[cfg(feature = "libghostty-vt")]
    fn compile_ghostty(search: &str) -> Option<Regex> {
        let has_uppercase = search.chars().any(|character| character.is_uppercase());
        RegexBuilder::new(search)
            .case_insensitive(!has_uppercase)
            .build()
            .ok()
    }

    #[cfg(feature = "libghostty-vt")]
    fn ghostty(&self) -> Option<Regex> {
        Self::compile_ghostty(self.pattern.as_ref())
    }
}

#[derive(Clone, Debug)]
struct TerminalSelection {
    ty: TerminalSelectionType,
    start: TerminalSelectionAnchor,
    end: TerminalSelectionAnchor,
    head: TerminalPoint,
}

#[derive(Clone, Copy, Debug)]
struct TerminalSelectionAnchor {
    point: TerminalPoint,
    side: TerminalSelectionSide,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TerminalSelectionSide {
    Left,
    Right,
}

impl TerminalSelectionSide {
    #[cfg(feature = "alacritty-backend")]
    fn to_alacritty(self) -> AlacDirection {
        match self {
            Self::Left => AlacDirection::Left,
            Self::Right => AlacDirection::Right,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerminalSelectionType {
    Simple,
    Semantic,
    Lines,
}

impl TerminalSelectionType {
    #[cfg(feature = "alacritty-backend")]
    fn to_alacritty(self) -> AlacSelectionType {
        match self {
            Self::Simple => AlacSelectionType::Simple,
            Self::Semantic => AlacSelectionType::Semantic,
            Self::Lines => AlacSelectionType::Lines,
        }
    }
}

impl TerminalSelection {
    fn new(
        selection_type: TerminalSelectionType,
        point: TerminalPoint,
        side: TerminalSelectionSide,
    ) -> Self {
        let anchor = TerminalSelectionAnchor { point, side };
        Self {
            ty: selection_type,
            start: anchor,
            end: anchor,
            head: point,
        }
    }

    fn simple_range(range: TerminalRange) -> Self {
        let mut selection = Self::new(
            TerminalSelectionType::Simple,
            range.start(),
            TerminalSelectionSide::Left,
        );
        selection.update(range.end(), TerminalSelectionSide::Right);
        selection
    }

    fn update(&mut self, point: TerminalPoint, side: TerminalSelectionSide) {
        self.end = TerminalSelectionAnchor { point, side };
        self.head = point;
    }

    #[cfg(feature = "alacritty-backend")]
    fn to_alacritty(&self) -> Selection {
        let mut selection = Selection::new(
            self.ty.to_alacritty(),
            self.start.point.to_alacritty(),
            self.start.side.to_alacritty(),
        );
        if self.start.point != self.end.point || self.start.side != self.end.side {
            selection.update(self.end.point.to_alacritty(), self.end.side.to_alacritty());
        }
        selection
    }

    #[cfg(feature = "libghostty-vt")]
    fn update_vi(&mut self, point: TerminalPoint) {
        self.start.side = TerminalSelectionSide::Left;
        self.end = TerminalSelectionAnchor {
            point,
            side: TerminalSelectionSide::Right,
        };
        self.head = point;
    }

    #[cfg(feature = "libghostty-vt")]
    fn to_range(&self, content: &TerminalContent) -> Option<TerminalSelectionRange> {
        let (top_line, bottom_line) = ghostty_content_line_bounds(content)?;
        let columns = content.terminal_bounds.num_columns();
        if columns == 0 {
            return None;
        }

        let mut start = self.start;
        let mut end = self.end;
        if start.point > end.point {
            std::mem::swap(&mut start, &mut end);
        }

        if end.point.line < top_line || start.point.line > bottom_line {
            return None;
        }

        start.point = clamp_ghostty_selection_point(start.point, top_line, bottom_line, columns);
        end.point = clamp_ghostty_selection_point(end.point, top_line, bottom_line, columns);

        match self.ty {
            TerminalSelectionType::Simple => self.range_simple(start, end, columns),
            TerminalSelectionType::Semantic => Some(range_ghostty_semantic_selection(
                content,
                start.point,
                end.point,
            )),
            TerminalSelectionType::Lines => Some(TerminalSelectionRange {
                start: TerminalPoint::new(start.point.line, 0),
                end: TerminalPoint::new(end.point.line, columns.saturating_sub(1)),
                is_block: false,
            }),
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn selected_text(&self, content: &TerminalContent, range: &TerminalSelectionRange) -> String {
        match self.ty {
            TerminalSelectionType::Lines => {
                let mut text = ghostty_selection_bounds_text(content, range);
                text.push('\n');
                text
            }
            TerminalSelectionType::Simple | TerminalSelectionType::Semantic => {
                ghostty_selection_bounds_text(content, range)
            }
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn range_simple(
        &self,
        mut start: TerminalSelectionAnchor,
        mut end: TerminalSelectionAnchor,
        columns: usize,
    ) -> Option<TerminalSelectionRange> {
        if self.is_empty() {
            return None;
        }

        if end.side == TerminalSelectionSide::Left && start.point != end.point {
            if end.point.column == 0 {
                end.point.column = columns - 1;
                end.point.line -= 1;
            } else {
                end.point.column -= 1;
            }
        }

        if start.side == TerminalSelectionSide::Right && start.point != end.point {
            start.point.column += 1;

            if start.point.column == columns {
                start.point.column = 0;
                start.point.line += 1;
            }
        }

        Some(TerminalSelectionRange {
            start: start.point,
            end: end.point,
            is_block: false,
        })
    }

    #[cfg(feature = "libghostty-vt")]
    fn is_empty(&self) -> bool {
        match self.ty {
            TerminalSelectionType::Simple => {
                let (mut start, mut end) = (self.start, self.end);
                if start.point > end.point {
                    std::mem::swap(&mut start, &mut end);
                }

                start.point == end.point && start.side == end.side
                    || start.side == TerminalSelectionSide::Right
                        && end.side == TerminalSelectionSide::Left
                        && start.point.line == end.point.line
                        && start.point.column.checked_add(1) == Some(end.point.column)
            }
            TerminalSelectionType::Semantic | TerminalSelectionType::Lines => false,
        }
    }
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_content_line_bounds(content: &TerminalContent) -> Option<(i32, i32)> {
    Some((
        content.cells.first()?.point.line,
        content.cells.last()?.point.line,
    ))
}

#[cfg(feature = "libghostty-vt")]
fn clamp_ghostty_selection_point(
    point: TerminalPoint,
    top_line: i32,
    bottom_line: i32,
    columns: usize,
) -> TerminalPoint {
    TerminalPoint::new(
        point.line.max(top_line).min(bottom_line),
        point.column.min(columns.saturating_sub(1)),
    )
}

#[cfg(feature = "libghostty-vt")]
fn range_ghostty_semantic_selection(
    content: &TerminalContent,
    start: TerminalPoint,
    end: TerminalPoint,
) -> TerminalSelectionRange {
    TerminalSelectionRange {
        start: ghostty_semantic_search_left(content, start),
        end: ghostty_semantic_search_right(content, end),
        is_block: false,
    }
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_last_column(content: &TerminalContent) -> usize {
    content.terminal_bounds.num_columns().saturating_sub(1)
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_semantic_search_left(
    content: &TerminalContent,
    mut point: TerminalPoint,
) -> TerminalPoint {
    while point.column > 0 {
        let previous = TerminalPoint::new(point.line, point.column - 1);
        if !ghostty_selection_cell_is_semantic(content, previous) {
            break;
        }
        point = previous;
    }
    point
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_semantic_search_right(
    content: &TerminalContent,
    mut point: TerminalPoint,
) -> TerminalPoint {
    let last_column = ghostty_last_column(content);
    while point.column < last_column {
        let next = TerminalPoint::new(point.line, point.column + 1);
        if !ghostty_selection_cell_is_semantic(content, next) {
            break;
        }
        point = next;
    }
    point
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_selection_cell_is_semantic(content: &TerminalContent, point: TerminalPoint) -> bool {
    ghostty_selection_cell(content, point)
        .map(|cell| !cell.c.is_whitespace())
        .unwrap_or(false)
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_selection_bounds_text(
    content: &TerminalContent,
    range: &TerminalSelectionRange,
) -> String {
    let mut text = String::new();
    for line in range.start.line..=range.end.line {
        let start_column = if line == range.start.line {
            range.start.column
        } else {
            0
        };
        let end_column = if line == range.end.line {
            range.end.column
        } else {
            ghostty_last_column(content)
        };

        text.push_str(&ghostty_selection_line_text(
            content,
            line,
            start_column,
            end_column,
            end_column == ghostty_last_column(content),
        ));

        if line != range.end.line {
            text.push('\n');
        }
    }
    text
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_selection_line_text(
    content: &TerminalContent,
    line: i32,
    start_column: usize,
    end_column: usize,
    trim_end: bool,
) -> String {
    let mut text = String::new();
    for column in start_column..=end_column {
        let Some(cell) = ghostty_selection_cell(content, TerminalPoint::new(line, column)) else {
            continue;
        };
        if cell.is_wide_char_spacer_or_leading() {
            continue;
        }

        text.push(cell.c);
        if let Some(chars) = cell.zerowidth() {
            for character in chars {
                text.push(*character);
            }
        }
    }

    if trim_end {
        text.truncate(text.trim_end_matches(' ').len());
    }

    text
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_selection_cell(
    content: &TerminalContent,
    point: TerminalPoint,
) -> Option<&TerminalCell> {
    content
        .cells
        .iter()
        .find(|cell| cell.point == point)
        .map(|cell| &cell.cell)
}

#[cfg(feature = "libghostty-vt")]
fn clamp_ghostty_content_point(content: &TerminalContent, point: TerminalPoint) -> TerminalPoint {
    let Some((top_line, bottom_line)) = ghostty_content_line_bounds(content) else {
        return point;
    };

    TerminalPoint::new(
        point.line.max(top_line).min(bottom_line),
        point.column.min(ghostty_last_column(content)),
    )
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_vi_motion(
    content: &TerminalContent,
    cursor: TerminalPoint,
    motion: TerminalViMotion,
) -> TerminalPoint {
    let Some((top_line, bottom_line)) = ghostty_content_line_bounds(content) else {
        return cursor;
    };

    let cursor = clamp_ghostty_content_point(content, cursor);
    let last_column = ghostty_last_column(content);

    match motion {
        TerminalViMotion::Up => {
            TerminalPoint::new(cursor.line.max(top_line + 1) - 1, cursor.column)
        }
        TerminalViMotion::Down => {
            TerminalPoint::new(cursor.line.min(bottom_line - 1) + 1, cursor.column)
        }
        TerminalViMotion::Left => ghostty_previous_point(content, cursor).unwrap_or(cursor),
        TerminalViMotion::Right => ghostty_next_point(content, cursor).unwrap_or(cursor),
        TerminalViMotion::First => TerminalPoint::new(cursor.line, 0),
        TerminalViMotion::Last => ghostty_last_occupied_in_line(content, cursor.line)
            .unwrap_or_else(|| TerminalPoint::new(cursor.line, last_column)),
        TerminalViMotion::FirstOccupied => ghostty_first_occupied_in_line(content, cursor.line)
            .unwrap_or_else(|| TerminalPoint::new(cursor.line, 0)),
        TerminalViMotion::High => ghostty_line_start(content, top_line),
        TerminalViMotion::Middle => {
            let line = top_line + (bottom_line - top_line) / 2;
            ghostty_line_start(content, line)
        }
        TerminalViMotion::Low => ghostty_line_start(content, bottom_line),
        TerminalViMotion::WordLeft => ghostty_word_start_left(content, cursor).unwrap_or(cursor),
        TerminalViMotion::WordRight => ghostty_word_start_right(content, cursor).unwrap_or(cursor),
        TerminalViMotion::WordRightEnd => ghostty_word_end_right(content, cursor).unwrap_or(cursor),
        TerminalViMotion::Bracket => ghostty_matching_bracket(content, cursor).unwrap_or(cursor),
    }
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_line_start(content: &TerminalContent, line: i32) -> TerminalPoint {
    ghostty_first_occupied_in_line(content, line).unwrap_or_else(|| TerminalPoint::new(line, 0))
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_first_occupied_in_line(content: &TerminalContent, line: i32) -> Option<TerminalPoint> {
    (0..=ghostty_last_column(content))
        .map(|column| TerminalPoint::new(line, column))
        .find(|&point| !ghostty_cell_is_space(content, point))
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_last_occupied_in_line(content: &TerminalContent, line: i32) -> Option<TerminalPoint> {
    (0..=ghostty_last_column(content))
        .rev()
        .map(|column| TerminalPoint::new(line, column))
        .find(|&point| !ghostty_cell_is_space(content, point))
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_next_point(content: &TerminalContent, point: TerminalPoint) -> Option<TerminalPoint> {
    let (_, bottom_line) = ghostty_content_line_bounds(content)?;
    let last_column = ghostty_last_column(content);

    if point.column < last_column {
        Some(TerminalPoint::new(point.line, point.column + 1))
    } else if point.line < bottom_line {
        Some(TerminalPoint::new(point.line + 1, 0))
    } else {
        None
    }
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_previous_point(
    content: &TerminalContent,
    point: TerminalPoint,
) -> Option<TerminalPoint> {
    let (top_line, _) = ghostty_content_line_bounds(content)?;
    let last_column = ghostty_last_column(content);

    if point.column > 0 {
        Some(TerminalPoint::new(point.line, point.column - 1))
    } else if point.line > top_line {
        Some(TerminalPoint::new(point.line - 1, last_column))
    } else {
        None
    }
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_cell_is_space(content: &TerminalContent, point: TerminalPoint) -> bool {
    ghostty_selection_cell(content, point)
        .map(|cell| cell.is_wide_char_spacer_or_leading() || cell.c == ' ' || cell.c == '\t')
        .unwrap_or(true)
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_word_start_right(
    content: &TerminalContent,
    mut point: TerminalPoint,
) -> Option<TerminalPoint> {
    if !ghostty_cell_is_space(content, point) {
        while let Some(next) = ghostty_next_point(content, point) {
            point = next;
            if ghostty_cell_is_space(content, point) {
                break;
            }
        }
    }

    while ghostty_cell_is_space(content, point) {
        point = ghostty_next_point(content, point)?;
    }

    Some(point)
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_word_start_left(
    content: &TerminalContent,
    mut point: TerminalPoint,
) -> Option<TerminalPoint> {
    point = ghostty_previous_point(content, point)?;

    while ghostty_cell_is_space(content, point) {
        point = ghostty_previous_point(content, point)?;
    }

    while let Some(previous) = ghostty_previous_point(content, point) {
        if ghostty_cell_is_space(content, previous) {
            break;
        }
        point = previous;
    }

    Some(point)
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_word_end_right(
    content: &TerminalContent,
    mut point: TerminalPoint,
) -> Option<TerminalPoint> {
    while ghostty_cell_is_space(content, point) {
        point = ghostty_next_point(content, point)?;
    }

    while let Some(next) = ghostty_next_point(content, point) {
        if ghostty_cell_is_space(content, next) {
            break;
        }
        point = next;
    }

    Some(point)
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_matching_bracket(
    content: &TerminalContent,
    point: TerminalPoint,
) -> Option<TerminalPoint> {
    let character = ghostty_selection_cell(content, point)?.c;
    let (matching, forward) = match character {
        '(' => (')', true),
        '[' => (']', true),
        '{' => ('}', true),
        '<' => ('>', true),
        ')' => ('(', false),
        ']' => ('[', false),
        '}' => ('{', false),
        '>' => ('<', false),
        _ => return None,
    };

    let mut depth = 0usize;
    let mut next = Some(point);
    while let Some(current) = next {
        let cell = ghostty_selection_cell(content, current)?;
        if cell.c == character {
            depth = depth.saturating_add(1);
        } else if cell.c == matching {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(current);
            }
        }

        next = if forward {
            ghostty_next_point(content, current)
        } else {
            ghostty_previous_point(content, current)
        };
    }

    None
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_content_search_matches(content: TerminalContent, regex: Regex) -> Vec<TerminalRange> {
    let Some((top_line, bottom_line)) = ghostty_content_line_bounds(&content) else {
        return Vec::new();
    };

    let mut matches = Vec::new();
    for line in top_line..=bottom_line {
        let (text, points) = ghostty_search_line_text(&content, line);
        if text.is_empty() {
            continue;
        }

        for regex_match in regex.find_iter(&text) {
            if regex_match.is_empty() {
                continue;
            }

            let start_index =
                points.partition_point(|(byte_index, _)| *byte_index < regex_match.start());
            let Some(end_index) = points
                .partition_point(|(byte_index, _)| *byte_index < regex_match.end())
                .checked_sub(1)
            else {
                continue;
            };

            let Some((_, start)) = points.get(start_index) else {
                continue;
            };
            let Some((_, end)) = points.get(end_index) else {
                continue;
            };

            matches.push(TerminalRange::new(*start, *end));
        }
    }

    matches
}

#[cfg(feature = "libghostty-vt")]
fn ghostty_search_line_text(
    content: &TerminalContent,
    line: i32,
) -> (String, Vec<(usize, TerminalPoint)>) {
    let mut text = String::new();
    let mut points = Vec::new();
    for cell in content.cells.iter().filter(|cell| cell.point.line == line) {
        if cell.is_wide_char_spacer_or_leading() {
            continue;
        }

        points.push((text.len(), cell.point));
        text.push(cell.c);
        if let Some(chars) = cell.zerowidth() {
            for character in chars {
                points.push((text.len(), cell.point));
                text.push(*character);
            }
        }
    }

    (text, points)
}

type ClipboardFormatter = Arc<dyn Fn(&str) -> String + Sync + Send + 'static>;
type ColorFormatter = Arc<dyn Fn(TerminalRgb) -> String + Sync + Send + 'static>;
#[cfg(feature = "alacritty-backend")]
type TextAreaSizeFormatter = Arc<dyn Fn(TerminalBounds) -> String + Sync + Send + 'static>;

#[derive(Clone)]
pub(crate) enum TerminalBackendEvent {
    #[cfg(feature = "alacritty-backend")]
    MouseCursorDirty,
    Title(String),
    #[cfg(feature = "alacritty-backend")]
    ResetTitle,
    ClipboardStore(String),
    ClipboardLoad(ClipboardFormatter),
    ColorRequest(usize, ColorFormatter),
    PtyWrite(String),
    #[cfg(feature = "alacritty-backend")]
    TextAreaSizeRequest(TextAreaSizeFormatter),
    #[cfg(feature = "alacritty-backend")]
    CursorBlinkingChange,
    Wakeup,
    Bell,
    #[cfg(feature = "alacritty-backend")]
    Exit,
    ChildExit(i32),
}

#[cfg(feature = "alacritty-backend")]
impl From<AlacTermEvent> for TerminalBackendEvent {
    fn from(event: AlacTermEvent) -> Self {
        match event {
            AlacTermEvent::MouseCursorDirty => Self::MouseCursorDirty,
            AlacTermEvent::Title(title) => Self::Title(title),
            AlacTermEvent::ResetTitle => Self::ResetTitle,
            AlacTermEvent::ClipboardStore(_, data) => Self::ClipboardStore(data),
            AlacTermEvent::ClipboardLoad(_, format) => Self::ClipboardLoad(format),
            AlacTermEvent::ColorRequest(index, format) => {
                Self::ColorRequest(index, Arc::new(move |color| format(VteRgb::from(color))))
            }
            AlacTermEvent::PtyWrite(output) => Self::PtyWrite(output),
            AlacTermEvent::TextAreaSizeRequest(format) => {
                Self::TextAreaSizeRequest(Arc::new(move |bounds| {
                    format(window_size_from_terminal_bounds(bounds))
                }))
            }
            AlacTermEvent::CursorBlinkingChange => Self::CursorBlinkingChange,
            AlacTermEvent::Wakeup => Self::Wakeup,
            AlacTermEvent::Bell => Self::Bell,
            AlacTermEvent::Exit => Self::Exit,
            AlacTermEvent::ChildExit(status) => Self::ChildExit(status),
        }
    }
}

impl fmt::Debug for TerminalBackendEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "alacritty-backend")]
            Self::MouseCursorDirty => f.write_str("MouseCursorDirty"),
            Self::Title(title) => write!(f, "Title({title})"),
            #[cfg(feature = "alacritty-backend")]
            Self::ResetTitle => f.write_str("ResetTitle"),
            Self::ClipboardStore(data) => write!(f, "ClipboardStore({data})"),
            Self::ClipboardLoad(_) => f.write_str("ClipboardLoad"),
            Self::ColorRequest(index, _) => write!(f, "ColorRequest({index})"),
            Self::PtyWrite(output) => write!(f, "PtyWrite({output})"),
            #[cfg(feature = "alacritty-backend")]
            Self::TextAreaSizeRequest(_) => f.write_str("TextAreaSizeRequest"),
            #[cfg(feature = "alacritty-backend")]
            Self::CursorBlinkingChange => f.write_str("CursorBlinkingChange"),
            Self::Wakeup => f.write_str("Wakeup"),
            Self::Bell => f.write_str("Bell"),
            #[cfg(feature = "alacritty-backend")]
            Self::Exit => f.write_str("Exit"),
            Self::ChildExit(status) => write!(f, "ChildExit({status})"),
        }
    }
}

enum PtyEvent {
    Event(TerminalBackendEvent),
    #[cfg(feature = "libghostty-vt")]
    Output(Vec<u8>),
}

///A translation struct for Alacritty to communicate with us from their event loop
#[derive(Clone)]
#[cfg(feature = "alacritty-backend")]
pub struct ZedListener(UnboundedSender<PtyEvent>);

#[cfg(feature = "alacritty-backend")]
impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(PtyEvent::Event(event.into())).ok();
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
        // Tolerance to prevent f32 precision from losing a row:
        // `N * line_height / line_height` can be N-epsilon, which floor()
        // would round down, pushing the first line into invisible scrollback.
        let raw = self.bounds.size.height / self.line_height;
        raw.next_up().floor() as usize
    }

    pub fn num_columns(&self) -> usize {
        let raw = self.bounds.size.width / self.cell_width;
        raw.next_up().floor() as usize
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

#[cfg(feature = "alacritty-backend")]
fn window_size_from_terminal_bounds(bounds: TerminalBounds) -> WindowSize {
    WindowSize {
        num_lines: bounds.num_lines() as u16,
        num_cols: bounds.num_columns() as u16,
        cell_width: f32::from(bounds.cell_width()) as u16,
        cell_height: f32::from(bounds.line_height()) as u16,
    }
}

#[cfg(feature = "alacritty-backend")]
fn alacritty_cursor_shape(cursor_shape: CursorShape) -> AlacCursorShape {
    match cursor_shape {
        CursorShape::Block => AlacCursorShape::Block,
        CursorShape::Underline => AlacCursorShape::Underline,
        CursorShape::Bar => AlacCursorShape::Beam,
        CursorShape::Hollow => AlacCursorShape::HollowBlock,
    }
}

#[cfg(feature = "alacritty-backend")]
fn alacritty_cursor_style(cursor_shape: CursorShape) -> AlacCursorStyle {
    AlacCursorStyle {
        shape: alacritty_cursor_shape(cursor_shape),
        blinking: false,
    }
}

#[cfg(feature = "alacritty-backend")]
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
    events_rx: UnboundedReceiver<PtyEvent>,
}

#[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerminalBackendKind {
    Alacritty,
    Ghostty,
}

#[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
impl TerminalBackendKind {
    fn selected(cx: &App) -> Self {
        if cx.has_flag::<GhosttyTerminalFeatureFlag>() {
            Self::Ghostty
        } else {
            Self::Alacritty
        }
    }
}

#[cfg(all(
    any(test, feature = "test-support"),
    feature = "libghostty-vt",
    feature = "alacritty-backend"
))]
pub fn enable_ghostty_terminal_feature_flag_for_tests(cx: &mut App) {
    FeatureFlagStore::init(cx);
    cx.update_flags(false, vec![GhosttyTerminalFeatureFlag::NAME.to_string()]);
}

#[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
fn ghostty_osc52_from_alacritty(osc52: AlacOsc52) -> GhosttyOsc52 {
    match osc52 {
        AlacOsc52::Disabled => GhosttyOsc52::Disabled,
        AlacOsc52::OnlyCopy => GhosttyOsc52::OnlyCopy,
        AlacOsc52::OnlyPaste => GhosttyOsc52::OnlyPaste,
        AlacOsc52::CopyPaste => GhosttyOsc52::CopyPaste,
    }
}

#[cfg(feature = "libghostty-vt")]
fn default_ghostty_osc52() -> GhosttyOsc52 {
    #[cfg(feature = "alacritty-backend")]
    {
        ghostty_osc52_from_alacritty(Config::default().osc52)
    }
    #[cfg(not(feature = "alacritty-backend"))]
    {
        GhosttyOsc52::default()
    }
}

#[cfg(feature = "libghostty-vt")]
fn new_default_portable_pty_command() -> portable_pty::CommandBuilder {
    portable_pty::CommandBuilder::new_default_prog()
}

impl TerminalBuilder {
    #[cfg(any(feature = "alacritty-backend", feature = "libghostty-vt"))]
    pub fn new_display_only(
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
        cx: &App,
    ) -> Result<TerminalBuilder> {
        #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
        {
            match TerminalBackendKind::selected(cx) {
                TerminalBackendKind::Alacritty => Self::new_display_only_alacritty(
                    cursor_shape,
                    alternate_scroll,
                    max_scroll_history_lines,
                    window_id,
                    background_executor,
                    path_style,
                ),
                TerminalBackendKind::Ghostty => Self::new_display_only_ghostty(
                    cursor_shape,
                    alternate_scroll,
                    max_scroll_history_lines,
                    window_id,
                    background_executor,
                    path_style,
                ),
            }
        }
        #[cfg(not(all(feature = "libghostty-vt", feature = "alacritty-backend")))]
        {
            let _ = cx;
            Self::new_display_only_default_backend(
                cursor_shape,
                alternate_scroll,
                max_scroll_history_lines,
                window_id,
                background_executor,
                path_style,
            )
        }
    }

    #[cfg(not(any(feature = "alacritty-backend", feature = "libghostty-vt")))]
    pub fn new_display_only(
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
        cx: &App,
    ) -> Result<TerminalBuilder> {
        let _ = (
            cursor_shape,
            alternate_scroll,
            max_scroll_history_lines,
            window_id,
            background_executor,
            path_style,
            cx,
        );
        Err(anyhow::anyhow!("no terminal backend compiled"))
    }

    #[cfg(all(feature = "alacritty-backend", not(feature = "libghostty-vt")))]
    fn new_display_only_default_backend(
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
    ) -> Result<TerminalBuilder> {
        Self::new_display_only_alacritty(
            cursor_shape,
            alternate_scroll,
            max_scroll_history_lines,
            window_id,
            background_executor,
            path_style,
        )
    }

    #[cfg(all(not(feature = "alacritty-backend"), feature = "libghostty-vt"))]
    fn new_display_only_default_backend(
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
    ) -> Result<TerminalBuilder> {
        Self::new_display_only_ghostty(
            cursor_shape,
            alternate_scroll,
            max_scroll_history_lines,
            window_id,
            background_executor,
            path_style,
        )
    }

    #[cfg(feature = "alacritty-backend")]
    fn new_display_only_alacritty(
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
    ) -> Result<TerminalBuilder> {
        // Create a display-only terminal (no actual PTY).
        let default_cursor_style = alacritty_cursor_style(cursor_shape);
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
            term: Some(term),
            term_config: Some(config),
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
            keyboard_input_sent: false,
            event_loop_task: Task::ready(Ok(())),
            background_executor: background_executor.clone(),
            path_style,
            #[cfg(feature = "libghostty-vt")]
            ghostty: None,
            #[cfg(feature = "libghostty-vt")]
            ghostty_selection: None,
            #[cfg(feature = "libghostty-vt")]
            ghostty_vi_cursor: None,
            #[cfg(feature = "libghostty-vt")]
            ghostty_cursor_blinking: false,
            #[cfg(any(test, feature = "test-support"))]
            input_log: Vec::new(),
        };

        Ok(TerminalBuilder {
            terminal,
            events_rx,
        })
    }

    #[cfg(feature = "libghostty-vt")]
    fn new_display_only_ghostty(
        cursor_shape: CursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
    ) -> Result<TerminalBuilder> {
        let mut ghostty = GhosttyBackend::new(TerminalBounds::default(), max_scroll_history_lines)?;
        ghostty.set_default_cursor_shape(cursor_shape.into());
        ghostty.set_osc52(default_ghostty_osc52());
        if let AlternateScroll::Off = alternate_scroll {
            ghostty.set_mode(libghostty_vt::terminal::Mode::ALT_SCROLL, false)?;
        }

        let (_events_tx, events_rx) = unbounded();
        let terminal = Terminal {
            task: None,
            terminal_type: TerminalType::DisplayOnly,
            completion_tx: None,
            #[cfg(feature = "alacritty-backend")]
            term: None,
            #[cfg(feature = "alacritty-backend")]
            term_config: None,
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
            hyperlink_regex_searches: RegexSearches::new_ghostty(Vec::<String>::new(), 0),
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
            keyboard_input_sent: false,
            event_loop_task: Task::ready(Ok(())),
            background_executor: background_executor.clone(),
            path_style,
            ghostty: Some(ghostty),
            ghostty_selection: None,
            ghostty_vi_cursor: None,
            ghostty_cursor_blinking: false,
            #[cfg(any(test, feature = "test-support"))]
            input_log: Vec::new(),
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
        #[cfg(not(any(feature = "alacritty-backend", feature = "libghostty-vt")))]
        {
            let _ = (
                working_directory,
                task,
                shell,
                env,
                cursor_shape,
                alternate_scroll,
                max_scroll_history_lines,
                path_hyperlink_regexes,
                path_hyperlink_timeout_ms,
                is_remote_terminal,
                window_id,
                completion_tx,
                cx,
                activation_script,
                path_style,
            );
            return Task::ready(Err(anyhow::anyhow!("no terminal backend compiled")));
        }

        #[cfg(any(feature = "alacritty-backend", feature = "libghostty-vt"))]
        {
            let version = release_channel::AppVersion::global(cx);
            let background_executor = cx.background_executor().clone();
            #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
            let terminal_backend_kind = TerminalBackendKind::selected(cx);
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

                #[cfg(feature = "alacritty-backend")]
                let new_alacritty_pty_options = || {
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
                #[cfg(feature = "libghostty-vt")]
                let new_portable_pty_command = || {
                    let mut command = if let Some(params) = shell_params.as_ref() {
                        let mut command = portable_pty::CommandBuilder::new(&params.program);
                        if let Some(args) = params.args.as_ref() {
                            command.args(args);
                        }
                        command
                    } else {
                        new_default_portable_pty_command()
                    };

                    if let Some(working_directory) = working_directory.as_ref() {
                        command.cwd(working_directory.as_os_str());
                    }
                    for (key, value) in &env {
                        command.env(key, value);
                    }

                    command
                };

                #[cfg(feature = "alacritty-backend")]
                let default_cursor_style = alacritty_cursor_style(cursor_shape);
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
                #[cfg(feature = "alacritty-backend")]
                let config = Config {
                    scrolling_history,
                    default_cursor_style,
                    ..Config::default()
                };

                #[cfg(feature = "alacritty-backend")]
                //Spawn a task so the Alacritty EventLoop can communicate with us
                //TODO: Remove with a bounded sender which can be dispatched on &self
                let (events_tx, events_rx) = unbounded();
                #[cfg(feature = "alacritty-backend")]
                let new_alacritty_term = |events_tx| {
                    let mut term = Term::new(
                        config.clone(),
                        &TerminalBounds::default(),
                        ZedListener(events_tx),
                    );

                    //Alacritty defaults to alternate scrolling being on, so we just need to turn it off.
                    if let AlternateScroll::Off = alternate_scroll {
                        term.unset_private_mode(PrivateMode::Named(
                            NamedPrivateMode::AlternateScroll,
                        ));
                    }

                    Arc::new(FairMutex::new(term))
                };

                #[cfg(all(feature = "alacritty-backend", not(feature = "libghostty-vt")))]
                let (pty_sender, pty_info, term, term_config) = {
                    let pty_options = new_alacritty_pty_options();
                    let pty = match tty::new(
                        &pty_options,
                        window_size_from_terminal_bounds(TerminalBounds::default()),
                        window_id,
                    ) {
                        Ok(pty) => pty,
                        Err(error) => {
                            bail!(TerminalError {
                                directory: working_directory.clone(),
                                program: shell_params.as_ref().map(|params| params.program.clone()),
                                args: shell_params.as_ref().and_then(|params| params.args.clone()),
                                title_override: terminal_title_override.clone(),
                                source: error,
                            });
                        }
                    };
                    let pty_info = PtyProcessInfo::new(&pty);
                    let term = new_alacritty_term(events_tx.clone());
                    let event_loop = EventLoop::new(
                        term.clone(),
                        ZedListener(events_tx),
                        pty,
                        pty_options.drain_on_exit,
                        false,
                    )
                    .context("failed to create event loop")?;
                    let pty_tx = event_loop.channel();
                    let _io_thread = event_loop.spawn();
                    (
                        PtySender::Alacritty(Notifier(pty_tx)),
                        pty_info,
                        Some(term),
                        Some(config.clone()),
                    )
                };

                #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
                let (pty_sender, pty_info, ghostty, term, term_config) = {
                    if terminal_backend_kind == TerminalBackendKind::Ghostty {
                        let pty_system = portable_pty::native_pty_system();
                        let portable_pty::PtyPair { master, slave } = match pty_system
                            .openpty(portable_pty_size(TerminalBounds::default()))
                        {
                            Ok(pair) => pair,
                            Err(error) => {
                                bail!(TerminalError {
                                    directory: working_directory.clone(),
                                    program: shell_params
                                        .as_ref()
                                        .map(|params| params.program.clone()),
                                    args: shell_params
                                        .as_ref()
                                        .and_then(|params| params.args.clone()),
                                    title_override: terminal_title_override,
                                    source: std::io::Error::other(error),
                                });
                            }
                        };
                        let child = match slave.spawn_command(new_portable_pty_command()) {
                            Ok(child) => child,
                            Err(error) => {
                                bail!(TerminalError {
                                    directory: working_directory.clone(),
                                    program: shell_params
                                        .as_ref()
                                        .map(|params| params.program.clone()),
                                    args: shell_params
                                        .as_ref()
                                        .and_then(|params| params.args.clone()),
                                    title_override: terminal_title_override,
                                    source: std::io::Error::other(error),
                                });
                            }
                        };
                        drop(slave);
                        let pty_info =
                            PtyProcessInfo::new_portable(master.as_ref(), child.as_ref());
                        let mut ghostty = GhosttyBackend::new(
                            TerminalBounds::default(),
                            Some(scrolling_history),
                        )?;
                        ghostty.set_default_cursor_shape(cursor_shape.into());
                        ghostty.set_osc52(ghostty_osc52_from_alacritty(config.osc52));
                        if let AlternateScroll::Off = alternate_scroll {
                            ghostty.set_mode(libghostty_vt::terminal::Mode::ALT_SCROLL, false)?;
                        }
                        let event_loop = GhosttyPtyEventLoop::new(events_tx, master, child, true)
                            .context("failed to create ghostty pty event loop")?;
                        let pty_tx = event_loop.channel();
                        let _io_thread = event_loop.spawn();
                        (
                            PtySender::Ghostty(GhosttyPtyNotifier::new(pty_tx)),
                            pty_info,
                            Some(ghostty),
                            None,
                            None,
                        )
                    } else {
                        let pty_options = new_alacritty_pty_options();
                        let pty = match tty::new(
                            &pty_options,
                            window_size_from_terminal_bounds(TerminalBounds::default()),
                            window_id,
                        ) {
                            Ok(pty) => pty,
                            Err(error) => {
                                bail!(TerminalError {
                                    directory: working_directory.clone(),
                                    program: shell_params
                                        .as_ref()
                                        .map(|params| params.program.clone()),
                                    args: shell_params
                                        .as_ref()
                                        .and_then(|params| params.args.clone()),
                                    title_override: terminal_title_override,
                                    source: error,
                                });
                            }
                        };
                        let pty_info = PtyProcessInfo::new(&pty);
                        let term = new_alacritty_term(events_tx.clone());
                        let event_loop = EventLoop::new(
                            term.clone(),
                            ZedListener(events_tx),
                            pty,
                            pty_options.drain_on_exit,
                            false,
                        )
                        .context("failed to create event loop")?;
                        let pty_tx = event_loop.channel();
                        let _io_thread = event_loop.spawn();
                        (
                            PtySender::Alacritty(Notifier(pty_tx)),
                            pty_info,
                            None,
                            Some(term),
                            Some(config.clone()),
                        )
                    }
                };

                #[cfg(all(feature = "libghostty-vt", not(feature = "alacritty-backend")))]
                let (pty_sender, pty_info, ghostty, events_rx) = {
                    let (events_tx, events_rx) = unbounded();
                    let pty_system = portable_pty::native_pty_system();
                    let portable_pty::PtyPair { master, slave } = match pty_system
                        .openpty(portable_pty_size(TerminalBounds::default()))
                    {
                        Ok(pair) => pair,
                        Err(error) => {
                            bail!(TerminalError {
                                directory: working_directory.clone(),
                                program: shell_params.as_ref().map(|params| params.program.clone()),
                                args: shell_params.as_ref().and_then(|params| params.args.clone()),
                                title_override: terminal_title_override.clone(),
                                source: std::io::Error::other(error),
                            });
                        }
                    };
                    let child = match slave.spawn_command(new_portable_pty_command()) {
                        Ok(child) => child,
                        Err(error) => {
                            bail!(TerminalError {
                                directory: working_directory.clone(),
                                program: shell_params.as_ref().map(|params| params.program.clone()),
                                args: shell_params.as_ref().and_then(|params| params.args.clone()),
                                title_override: terminal_title_override.clone(),
                                source: std::io::Error::other(error),
                            });
                        }
                    };
                    drop(slave);
                    let pty_info = PtyProcessInfo::new_portable(master.as_ref(), child.as_ref());
                    let mut ghostty =
                        GhosttyBackend::new(TerminalBounds::default(), Some(scrolling_history))?;
                    ghostty.set_default_cursor_shape(cursor_shape.into());
                    ghostty.set_osc52(default_ghostty_osc52());
                    if let AlternateScroll::Off = alternate_scroll {
                        ghostty.set_mode(libghostty_vt::terminal::Mode::ALT_SCROLL, false)?;
                    }
                    let event_loop = GhosttyPtyEventLoop::new(events_tx, master, child, true)
                        .context("failed to create ghostty pty event loop")?;
                    let pty_tx = event_loop.channel();
                    let _io_thread = event_loop.spawn();
                    (
                        PtySender::Ghostty(GhosttyPtyNotifier::new(pty_tx)),
                        pty_info,
                        Some(ghostty),
                        events_rx,
                    )
                };

                let no_task = task.is_none();
                #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
                let hyperlink_regex_searches = if terminal_backend_kind
                    == TerminalBackendKind::Ghostty
                {
                    RegexSearches::new_ghostty(&path_hyperlink_regexes, path_hyperlink_timeout_ms)
                } else {
                    RegexSearches::new(&path_hyperlink_regexes, path_hyperlink_timeout_ms)
                };
                #[cfg(all(feature = "libghostty-vt", not(feature = "alacritty-backend")))]
                let hyperlink_regex_searches =
                    RegexSearches::new_ghostty(&path_hyperlink_regexes, path_hyperlink_timeout_ms);
                #[cfg(not(feature = "libghostty-vt"))]
                let hyperlink_regex_searches =
                    RegexSearches::new(&path_hyperlink_regexes, path_hyperlink_timeout_ms);
                let terminal = Terminal {
                    task,
                    terminal_type: TerminalType::Pty {
                        pty_tx: pty_sender,
                        info: Arc::new(pty_info),
                    },
                    completion_tx,
                    #[cfg(feature = "alacritty-backend")]
                    term,
                    #[cfg(feature = "alacritty-backend")]
                    term_config,
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
                    hyperlink_regex_searches,
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
                    keyboard_input_sent: false,
                    event_loop_task: Task::ready(Ok(())),
                    background_executor,
                    path_style,
                    #[cfg(feature = "libghostty-vt")]
                    ghostty,
                    #[cfg(feature = "libghostty-vt")]
                    ghostty_selection: None,
                    #[cfg(feature = "libghostty-vt")]
                    ghostty_vi_cursor: None,
                    #[cfg(feature = "libghostty-vt")]
                    ghostty_cursor_blinking: false,
                    #[cfg(any(test, feature = "test-support"))]
                    input_log: Vec::new(),
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
            #[cfg(not(target_os = "windows"))]
            {
                cx.spawn(async move |_| fut.await)
            }
            #[cfg(target_os = "windows")]
            {
                cx.background_spawn(fut)
            }
        }
    }

    pub fn subscribe(mut self, cx: &Context<Terminal>) -> Terminal {
        //Event loop
        self.terminal.event_loop_task = cx.spawn(async move |terminal, cx| {
            while let Some(event) = self.events_rx.next().await {
                terminal.update(cx, |terminal, cx| {
                    //Process the first event immediately for lowered latency
                    terminal.process_pty_event(event, cx);
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
                                    if matches!(event, PtyEvent::Event(TerminalBackendEvent::Wakeup))
                                    {
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
                        yield_now().await;
                        break 'outer;
                    }

                    terminal.update(cx, |this, cx| {
                        if wakeup {
                            this.process_event(TerminalBackendEvent::Wakeup, cx);
                        }

                        for event in events {
                            this.process_pty_event(event, cx);
                        }
                    })?;
                    yield_now().await;
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TerminalColor {
    Named(TerminalNamedColor),
    Spec(TerminalRgb),
    Indexed(u8),
}

impl TerminalColor {
    #[inline]
    pub fn is_default_background(self) -> bool {
        self == Self::Named(TerminalNamedColor::Background)
    }

    #[inline]
    pub fn is_app_chosen_exact(self) -> bool {
        matches!(self, Self::Spec(_) | Self::Indexed(16..=255))
    }
}

impl From<VteColor> for TerminalColor {
    fn from(color: VteColor) -> Self {
        match color {
            VteColor::Named(color) => Self::Named(color.into()),
            VteColor::Spec(color) => Self::Spec(color.into()),
            VteColor::Indexed(index) => Self::Indexed(index),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminalRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[cfg(feature = "libghostty-vt")]
fn terminal_rgb_from_color(color: impl Into<Rgba>) -> TerminalRgb {
    let color = color.into();
    TerminalRgb {
        r: ((color.r * color.a) * 255.) as u8,
        g: ((color.g * color.a) * 255.) as u8,
        b: ((color.b * color.a) * 255.) as u8,
    }
}

impl From<VteRgb> for TerminalRgb {
    fn from(color: VteRgb) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

impl From<TerminalRgb> for VteRgb {
    fn from(color: TerminalRgb) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TerminalNamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Foreground,
    Background,
    Cursor,
    DimBlack,
    DimRed,
    DimGreen,
    DimYellow,
    DimBlue,
    DimMagenta,
    DimCyan,
    DimWhite,
    BrightForeground,
    DimForeground,
}

impl From<VteNamedColor> for TerminalNamedColor {
    fn from(color: VteNamedColor) -> Self {
        match color {
            VteNamedColor::Black => Self::Black,
            VteNamedColor::Red => Self::Red,
            VteNamedColor::Green => Self::Green,
            VteNamedColor::Yellow => Self::Yellow,
            VteNamedColor::Blue => Self::Blue,
            VteNamedColor::Magenta => Self::Magenta,
            VteNamedColor::Cyan => Self::Cyan,
            VteNamedColor::White => Self::White,
            VteNamedColor::BrightBlack => Self::BrightBlack,
            VteNamedColor::BrightRed => Self::BrightRed,
            VteNamedColor::BrightGreen => Self::BrightGreen,
            VteNamedColor::BrightYellow => Self::BrightYellow,
            VteNamedColor::BrightBlue => Self::BrightBlue,
            VteNamedColor::BrightMagenta => Self::BrightMagenta,
            VteNamedColor::BrightCyan => Self::BrightCyan,
            VteNamedColor::BrightWhite => Self::BrightWhite,
            VteNamedColor::Foreground => Self::Foreground,
            VteNamedColor::Background => Self::Background,
            VteNamedColor::Cursor => Self::Cursor,
            VteNamedColor::DimBlack => Self::DimBlack,
            VteNamedColor::DimRed => Self::DimRed,
            VteNamedColor::DimGreen => Self::DimGreen,
            VteNamedColor::DimYellow => Self::DimYellow,
            VteNamedColor::DimBlue => Self::DimBlue,
            VteNamedColor::DimMagenta => Self::DimMagenta,
            VteNamedColor::DimCyan => Self::DimCyan,
            VteNamedColor::DimWhite => Self::DimWhite,
            VteNamedColor::BrightForeground => Self::BrightForeground,
            VteNamedColor::DimForeground => Self::DimForeground,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminalHyperlink {
    id: Option<String>,
    uri: String,
}

impl TerminalHyperlink {
    pub fn new<T: ToString>(id: Option<T>, uri: String) -> Self {
        Self {
            id: id.map(|id| id.to_string()),
            uri,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}

#[cfg(feature = "alacritty-backend")]
fn terminal_hyperlink_from_alacritty(hyperlink: AlacHyperlink) -> TerminalHyperlink {
    TerminalHyperlink::new(Some(hyperlink.id().to_owned()), hyperlink.uri().to_owned())
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
struct TerminalCellExtra {
    zerowidth: Vec<char>,
    underline_color: Option<TerminalColor>,
    hyperlink: Option<TerminalHyperlink>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct TerminalCellFlags(u32);

impl TerminalCellFlags {
    pub(crate) const BOLD: Self = Self(1 << 0);
    pub(crate) const ITALIC: Self = Self(1 << 1);
    pub(crate) const DIM: Self = Self(1 << 2);
    pub(crate) const INVERSE: Self = Self(1 << 3);
    pub(crate) const HIDDEN: Self = Self(1 << 4);
    pub(crate) const STRIKEOUT: Self = Self(1 << 5);
    pub(crate) const UNDERLINE: Self = Self(1 << 6);
    pub(crate) const DOUBLE_UNDERLINE: Self = Self(1 << 7);
    pub(crate) const UNDERCURL: Self = Self(1 << 8);
    pub(crate) const DOTTED_UNDERLINE: Self = Self(1 << 9);
    pub(crate) const DASHED_UNDERLINE: Self = Self(1 << 10);
    pub(crate) const WIDE_CHAR: Self = Self(1 << 11);
    pub(crate) const WIDE_CHAR_SPACER: Self = Self(1 << 12);
    pub(crate) const LEADING_WIDE_CHAR_SPACER: Self = Self(1 << 13);
    const ALL_UNDERLINES: Self = Self(
        Self::UNDERLINE.0
            | Self::DOUBLE_UNDERLINE.0
            | Self::UNDERCURL.0
            | Self::DOTTED_UNDERLINE.0
            | Self::DASHED_UNDERLINE.0,
    );

    pub(crate) fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, flag: Self) {
        self.0 |= flag.0;
    }

    fn contains(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }

    fn intersects(self, flags: Self) -> bool {
        self.0 & flags.0 != 0
    }
}

impl BitOr for TerminalCellFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for TerminalCellFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

#[cfg(feature = "alacritty-backend")]
fn terminal_cell_flags_from_alacritty(flags: Flags) -> TerminalCellFlags {
    let mut terminal_flags = TerminalCellFlags::empty();
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::BOLD,
        TerminalCellFlags::BOLD,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::ITALIC,
        TerminalCellFlags::ITALIC,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::DIM,
        TerminalCellFlags::DIM,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::INVERSE,
        TerminalCellFlags::INVERSE,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::HIDDEN,
        TerminalCellFlags::HIDDEN,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::STRIKEOUT,
        TerminalCellFlags::STRIKEOUT,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::UNDERLINE,
        TerminalCellFlags::UNDERLINE,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::DOUBLE_UNDERLINE,
        TerminalCellFlags::DOUBLE_UNDERLINE,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::UNDERCURL,
        TerminalCellFlags::UNDERCURL,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::DOTTED_UNDERLINE,
        TerminalCellFlags::DOTTED_UNDERLINE,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::DASHED_UNDERLINE,
        TerminalCellFlags::DASHED_UNDERLINE,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::WIDE_CHAR,
        TerminalCellFlags::WIDE_CHAR,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::WIDE_CHAR_SPACER,
        TerminalCellFlags::WIDE_CHAR_SPACER,
    );
    add_terminal_cell_flag(
        &mut terminal_flags,
        flags,
        Flags::LEADING_WIDE_CHAR_SPACER,
        TerminalCellFlags::LEADING_WIDE_CHAR_SPACER,
    );
    terminal_flags
}

#[cfg(feature = "alacritty-backend")]
fn add_terminal_cell_flag(
    terminal_flags: &mut TerminalCellFlags,
    alacritty_flags: Flags,
    alacritty_flag: Flags,
    terminal_flag: TerminalCellFlags,
) {
    if alacritty_flags.contains(alacritty_flag) {
        terminal_flags.insert(terminal_flag);
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminalCell {
    pub c: char,
    pub fg: TerminalColor,
    pub bg: TerminalColor,
    flags: TerminalCellFlags,
    extra: Option<TerminalCellExtra>,
}

impl Default for TerminalCell {
    fn default() -> Self {
        Self {
            c: ' ',
            bg: TerminalColor::Named(TerminalNamedColor::Background),
            fg: TerminalColor::Named(TerminalNamedColor::Foreground),
            flags: TerminalCellFlags::empty(),
            extra: None,
        }
    }
}

impl TerminalCell {
    #[cfg(feature = "libghostty-vt")]
    pub(crate) fn new(
        c: char,
        fg: TerminalColor,
        bg: TerminalColor,
        flags: TerminalCellFlags,
    ) -> Self {
        Self {
            c,
            fg,
            bg,
            flags,
            extra: None,
        }
    }

    #[inline]
    pub fn zerowidth(&self) -> Option<&[char]> {
        self.extra.as_ref().map(|extra| extra.zerowidth.as_slice())
    }

    #[inline]
    pub fn push_zerowidth(&mut self, character: char) {
        self.extra
            .get_or_insert_with(TerminalCellExtra::default)
            .zerowidth
            .push(character);
    }

    pub fn set_underline_color(&mut self, color: Option<TerminalColor>) {
        let should_drop = color.is_none()
            && self
                .extra
                .as_ref()
                .is_none_or(|extra| extra.zerowidth.is_empty() && extra.hyperlink.is_none());

        if should_drop {
            self.extra = None;
        } else {
            self.extra
                .get_or_insert_with(TerminalCellExtra::default)
                .underline_color = color;
        }
    }

    #[inline]
    pub fn underline_color(&self) -> Option<TerminalColor> {
        self.extra.as_ref()?.underline_color
    }

    pub fn set_hyperlink(&mut self, hyperlink: Option<TerminalHyperlink>) {
        let should_drop = hyperlink.is_none()
            && self
                .extra
                .as_ref()
                .is_none_or(|extra| extra.zerowidth.is_empty() && extra.underline_color.is_none());

        if should_drop {
            self.extra = None;
        } else {
            self.extra
                .get_or_insert_with(TerminalCellExtra::default)
                .hyperlink = hyperlink;
        }
    }

    #[inline]
    pub fn hyperlink(&self) -> Option<&TerminalHyperlink> {
        self.extra.as_ref()?.hyperlink.as_ref()
    }

    #[inline]
    pub fn is_inverse(&self) -> bool {
        self.flags.contains(TerminalCellFlags::INVERSE)
    }

    #[inline]
    pub fn is_wide_char_spacer(&self) -> bool {
        self.flags.contains(TerminalCellFlags::WIDE_CHAR_SPACER)
    }

    #[inline]
    #[cfg(feature = "libghostty-vt")]
    pub(crate) fn is_wide_char_spacer_or_leading(&self) -> bool {
        self.flags.intersects(
            TerminalCellFlags::WIDE_CHAR_SPACER | TerminalCellFlags::LEADING_WIDE_CHAR_SPACER,
        )
    }

    #[inline]
    pub fn is_dim(&self) -> bool {
        self.flags.intersects(TerminalCellFlags::DIM)
    }

    #[inline]
    pub fn has_underline(&self) -> bool {
        self.flags.intersects(TerminalCellFlags::ALL_UNDERLINES)
    }

    #[inline]
    pub fn has_undercurl(&self) -> bool {
        self.flags.contains(TerminalCellFlags::UNDERCURL)
    }

    #[inline]
    pub fn has_strikeout(&self) -> bool {
        self.flags.intersects(TerminalCellFlags::STRIKEOUT)
    }

    #[inline]
    pub fn is_bold(&self) -> bool {
        self.flags.intersects(TerminalCellFlags::BOLD)
    }

    #[inline]
    pub fn is_italic(&self) -> bool {
        self.flags.intersects(TerminalCellFlags::ITALIC)
    }

    #[inline]
    pub fn has_visible_style_modifier(&self) -> bool {
        self.flags.intersects(
            TerminalCellFlags::ALL_UNDERLINES
                | TerminalCellFlags::INVERSE
                | TerminalCellFlags::STRIKEOUT,
        )
    }
}

#[cfg(feature = "alacritty-backend")]
fn terminal_cell_from_alacritty(cell: AlacCell) -> TerminalCell {
    let zerowidth = cell.zerowidth().unwrap_or_default().to_vec();
    let underline_color = cell.underline_color().map(Into::into);
    let hyperlink = cell.hyperlink().map(terminal_hyperlink_from_alacritty);
    let extra = if zerowidth.is_empty() && underline_color.is_none() && hyperlink.is_none() {
        None
    } else {
        Some(TerminalCellExtra {
            zerowidth,
            underline_color,
            hyperlink,
        })
    };

    TerminalCell {
        c: cell.c,
        fg: cell.fg.into(),
        bg: cell.bg.into(),
        flags: terminal_cell_flags_from_alacritty(cell.flags),
        extra,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexedCell {
    pub point: TerminalPoint,
    pub cell: TerminalCell,
}

impl Deref for IndexedCell {
    type Target = TerminalCell;

    #[inline]
    fn deref(&self) -> &TerminalCell {
        &self.cell
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminalModes(u32);

impl TerminalModes {
    pub const NONE: Self = Self(0);
    pub const APP_CURSOR: Self = Self(1 << 0);
    pub const APP_KEYPAD: Self = Self(1 << 1);
    pub const SHOW_CURSOR: Self = Self(1 << 2);
    pub const LINE_WRAP: Self = Self(1 << 3);
    pub const ORIGIN: Self = Self(1 << 4);
    pub const INSERT: Self = Self(1 << 5);
    pub const LINE_FEED_NEW_LINE: Self = Self(1 << 6);
    pub const FOCUS_IN_OUT: Self = Self(1 << 7);
    pub const ALTERNATE_SCROLL: Self = Self(1 << 8);
    pub const BRACKETED_PASTE: Self = Self(1 << 9);
    pub const SGR_MOUSE: Self = Self(1 << 10);
    pub const UTF8_MOUSE: Self = Self(1 << 11);
    pub const ALT_SCREEN: Self = Self(1 << 12);
    pub const MOUSE_REPORT_CLICK: Self = Self(1 << 13);
    pub const MOUSE_DRAG: Self = Self(1 << 14);
    pub const MOUSE_MOTION: Self = Self(1 << 15);
    pub const VI: Self = Self(1 << 16);
    pub const MOUSE_MODE: Self =
        Self(Self::MOUSE_REPORT_CLICK.0 | Self::MOUSE_DRAG.0 | Self::MOUSE_MOTION.0);

    pub const fn empty() -> Self {
        Self::NONE
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    #[cfg(all(test, feature = "alacritty-backend"))]
    fn to_alacritty(self) -> TermMode {
        let mut mode = TermMode::empty();
        add_alacritty_mode(&mut mode, self, Self::APP_CURSOR, TermMode::APP_CURSOR);
        add_alacritty_mode(&mut mode, self, Self::APP_KEYPAD, TermMode::APP_KEYPAD);
        add_alacritty_mode(&mut mode, self, Self::SHOW_CURSOR, TermMode::SHOW_CURSOR);
        add_alacritty_mode(&mut mode, self, Self::LINE_WRAP, TermMode::LINE_WRAP);
        add_alacritty_mode(&mut mode, self, Self::ORIGIN, TermMode::ORIGIN);
        add_alacritty_mode(&mut mode, self, Self::INSERT, TermMode::INSERT);
        add_alacritty_mode(
            &mut mode,
            self,
            Self::LINE_FEED_NEW_LINE,
            TermMode::LINE_FEED_NEW_LINE,
        );
        add_alacritty_mode(&mut mode, self, Self::FOCUS_IN_OUT, TermMode::FOCUS_IN_OUT);
        add_alacritty_mode(
            &mut mode,
            self,
            Self::ALTERNATE_SCROLL,
            TermMode::ALTERNATE_SCROLL,
        );
        add_alacritty_mode(
            &mut mode,
            self,
            Self::BRACKETED_PASTE,
            TermMode::BRACKETED_PASTE,
        );
        add_alacritty_mode(&mut mode, self, Self::SGR_MOUSE, TermMode::SGR_MOUSE);
        add_alacritty_mode(&mut mode, self, Self::UTF8_MOUSE, TermMode::UTF8_MOUSE);
        add_alacritty_mode(&mut mode, self, Self::ALT_SCREEN, TermMode::ALT_SCREEN);
        add_alacritty_mode(
            &mut mode,
            self,
            Self::MOUSE_REPORT_CLICK,
            TermMode::MOUSE_REPORT_CLICK,
        );
        add_alacritty_mode(&mut mode, self, Self::MOUSE_DRAG, TermMode::MOUSE_DRAG);
        add_alacritty_mode(&mut mode, self, Self::MOUSE_MOTION, TermMode::MOUSE_MOTION);
        add_alacritty_mode(&mut mode, self, Self::VI, TermMode::VI);
        mode
    }
}

#[cfg(feature = "alacritty-backend")]
fn terminal_modes_from_alacritty(mode: TermMode) -> TerminalModes {
    let mut terminal_modes = TerminalModes::empty();
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::APP_CURSOR,
        TerminalModes::APP_CURSOR,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::APP_KEYPAD,
        TerminalModes::APP_KEYPAD,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::SHOW_CURSOR,
        TerminalModes::SHOW_CURSOR,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::LINE_WRAP,
        TerminalModes::LINE_WRAP,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::ORIGIN,
        TerminalModes::ORIGIN,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::INSERT,
        TerminalModes::INSERT,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::LINE_FEED_NEW_LINE,
        TerminalModes::LINE_FEED_NEW_LINE,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::FOCUS_IN_OUT,
        TerminalModes::FOCUS_IN_OUT,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::ALTERNATE_SCROLL,
        TerminalModes::ALTERNATE_SCROLL,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::BRACKETED_PASTE,
        TerminalModes::BRACKETED_PASTE,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::SGR_MOUSE,
        TerminalModes::SGR_MOUSE,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::UTF8_MOUSE,
        TerminalModes::UTF8_MOUSE,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::ALT_SCREEN,
        TerminalModes::ALT_SCREEN,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::MOUSE_REPORT_CLICK,
        TerminalModes::MOUSE_REPORT_CLICK,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::MOUSE_DRAG,
        TerminalModes::MOUSE_DRAG,
    );
    add_terminal_mode(
        &mut terminal_modes,
        mode,
        TermMode::MOUSE_MOTION,
        TerminalModes::MOUSE_MOTION,
    );
    add_terminal_mode(&mut terminal_modes, mode, TermMode::VI, TerminalModes::VI);
    terminal_modes
}

impl BitOr for TerminalModes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for TerminalModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

#[cfg(feature = "alacritty-backend")]
fn add_terminal_mode(
    terminal_modes: &mut TerminalModes,
    alacritty_modes: TermMode,
    alacritty_mode: TermMode,
    terminal_mode: TerminalModes,
) {
    if alacritty_modes.contains(alacritty_mode) {
        terminal_modes.insert(terminal_mode);
    }
}

#[cfg(all(test, feature = "alacritty-backend"))]
fn add_alacritty_mode(
    alacritty_modes: &mut TermMode,
    terminal_modes: TerminalModes,
    terminal_mode: TerminalModes,
    alacritty_mode: TermMode,
) {
    if terminal_modes.contains(terminal_mode) {
        alacritty_modes.insert(alacritty_mode);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalCursor {
    pub shape: TerminalCursorShape,
    pub point: TerminalPoint,
}

impl TerminalCursor {
    #[cfg(feature = "alacritty-backend")]
    fn from_alacritty(cursor: RenderableCursor) -> Self {
        Self {
            shape: terminal_cursor_shape_from_alacritty(cursor.shape),
            point: terminal_point_from_alacritty(cursor.point),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalCursorShape {
    Block,
    Underline,
    Bar,
    HollowBlock,
    Hidden,
}

#[cfg(feature = "alacritty-backend")]
fn terminal_cursor_shape_from_alacritty(shape: AlacCursorShape) -> TerminalCursorShape {
    match shape {
        AlacCursorShape::Block => TerminalCursorShape::Block,
        AlacCursorShape::Underline => TerminalCursorShape::Underline,
        AlacCursorShape::Beam => TerminalCursorShape::Bar,
        AlacCursorShape::HollowBlock => TerminalCursorShape::HollowBlock,
        AlacCursorShape::Hidden => TerminalCursorShape::Hidden,
    }
}

impl From<CursorShape> for TerminalCursorShape {
    fn from(shape: CursorShape) -> Self {
        match shape {
            CursorShape::Block => Self::Block,
            CursorShape::Underline => Self::Underline,
            CursorShape::Bar => Self::Bar,
            CursorShape::Hollow => Self::HollowBlock,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TerminalPoint {
    pub line: i32,
    pub column: usize,
}

impl TerminalPoint {
    pub fn new(line: i32, column: usize) -> Self {
        Self { line, column }
    }

    #[cfg(feature = "alacritty-backend")]
    fn to_alacritty(self) -> AlacPoint {
        AlacPoint::new(Line(self.line), Column(self.column))
    }
}

#[cfg(feature = "alacritty-backend")]
fn terminal_point_from_alacritty(point: AlacPoint) -> TerminalPoint {
    TerminalPoint {
        line: point.line.0,
        column: point.column.0,
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TerminalRange {
    start: TerminalPoint,
    end: TerminalPoint,
}

impl TerminalRange {
    pub fn new(start: TerminalPoint, end: TerminalPoint) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> TerminalPoint {
        self.start
    }

    pub fn end(&self) -> TerminalPoint {
        self.end
    }

    pub fn contains(&self, point: TerminalPoint) -> bool {
        self.start <= point && point <= self.end
    }

    #[cfg(all(test, feature = "alacritty-backend"))]
    pub(crate) fn to_alacritty(self) -> RangeInclusive<AlacPoint> {
        self.start.to_alacritty()..=self.end.to_alacritty()
    }

    #[cfg(feature = "alacritty-backend")]
    pub(crate) fn from_alacritty(range: RangeInclusive<AlacPoint>) -> Self {
        Self {
            start: terminal_point_from_alacritty(*range.start()),
            end: terminal_point_from_alacritty(*range.end()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalSelectionRange {
    pub start: TerminalPoint,
    pub end: TerminalPoint,
    pub is_block: bool,
}

impl TerminalSelectionRange {
    pub fn point_range(self) -> TerminalRange {
        TerminalRange::new(self.start, self.end)
    }
}

#[cfg(feature = "alacritty-backend")]
fn terminal_selection_range_from_alacritty(range: SelectionRange) -> TerminalSelectionRange {
    TerminalSelectionRange {
        start: terminal_point_from_alacritty(range.start),
        end: terminal_point_from_alacritty(range.end),
        is_block: range.is_block,
    }
}

// TODO: Un-pub
#[derive(Clone)]
pub struct TerminalContent {
    pub cells: Vec<IndexedCell>,
    pub mode: TerminalModes,
    pub display_offset: usize,
    pub selection_text: Option<String>,
    pub selection: Option<TerminalSelectionRange>,
    pub cursor: TerminalCursor,
    pub cursor_char: char,
    pub terminal_bounds: TerminalBounds,
    pub last_hovered_word: Option<HoveredWord>,
    pub scrolled_to_top: bool,
    pub scrolled_to_bottom: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HoveredWord {
    pub word: String,
    pub word_match: TerminalRange,
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
            cursor: TerminalCursor {
                shape: TerminalCursorShape::Block,
                point: TerminalPoint::new(0, 0),
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
        pty_tx: PtySender,
        info: Arc<PtyProcessInfo>,
    },
    DisplayOnly,
}

enum PtySender {
    #[cfg(feature = "alacritty-backend")]
    Alacritty(Notifier),
    #[cfg(feature = "libghostty-vt")]
    Ghostty(GhosttyPtyNotifier),
}

impl PtySender {
    fn notify(&self, input: impl Into<Cow<'static, [u8]>>) {
        match self {
            #[cfg(feature = "alacritty-backend")]
            Self::Alacritty(notifier) => notifier.notify(input),
            #[cfg(feature = "libghostty-vt")]
            Self::Ghostty(notifier) => notifier.notify(input),
            #[cfg(not(any(feature = "alacritty-backend", feature = "libghostty-vt")))]
            _ => unreachable!("no terminal pty backend compiled"),
        }
    }

    fn resize(&self, bounds: TerminalBounds) {
        match self {
            #[cfg(feature = "alacritty-backend")]
            Self::Alacritty(notifier) => {
                if let Err(error) = notifier
                    .0
                    .send(Msg::Resize(window_size_from_terminal_bounds(bounds)))
                {
                    log::error!("failed to resize alacritty pty: {error}");
                }
            }
            #[cfg(feature = "libghostty-vt")]
            Self::Ghostty(notifier) => notifier.resize(bounds),
            #[cfg(not(any(feature = "alacritty-backend", feature = "libghostty-vt")))]
            _ => unreachable!("no terminal pty backend compiled"),
        }
    }

    fn shutdown(&self) {
        match self {
            #[cfg(feature = "alacritty-backend")]
            Self::Alacritty(notifier) => {
                if let Err(error) = notifier.0.send(Msg::Shutdown) {
                    log::debug!("failed to shut down alacritty pty loop: {error}");
                }
            }
            #[cfg(feature = "libghostty-vt")]
            Self::Ghostty(notifier) => notifier.shutdown(),
            #[cfg(not(any(feature = "alacritty-backend", feature = "libghostty-vt")))]
            _ => unreachable!("no terminal pty backend compiled"),
        }
    }
}

pub struct Terminal {
    terminal_type: TerminalType,
    completion_tx: Option<Sender<Option<ExitStatus>>>,
    #[cfg(feature = "alacritty-backend")]
    term: Option<Arc<FairMutex<Term<ZedListener>>>>,
    #[cfg(feature = "alacritty-backend")]
    term_config: Option<Config>,
    events: VecDeque<InternalEvent>,
    /// This is only used for mouse mode cell change detection
    last_mouse: Option<(TerminalPoint, TerminalSelectionSide)>,
    pub matches: Vec<TerminalRange>,
    pub last_content: TerminalContent,
    pub selection_head: Option<TerminalPoint>,

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
    mouse_down_hyperlink: Option<HyperlinkMatch>,
    #[cfg(windows)]
    shell_program: Option<String>,
    template: CopyTemplate,
    activation_script: Vec<String>,
    child_exited: Option<ExitStatus>,
    keyboard_input_sent: bool,
    event_loop_task: Task<Result<(), anyhow::Error>>,
    background_executor: BackgroundExecutor,
    path_style: PathStyle,
    #[cfg(feature = "libghostty-vt")]
    ghostty: Option<GhosttyBackend>,
    #[cfg(feature = "libghostty-vt")]
    ghostty_selection: Option<TerminalSelection>,
    #[cfg(feature = "libghostty-vt")]
    ghostty_vi_cursor: Option<TerminalPoint>,
    #[cfg(feature = "libghostty-vt")]
    ghostty_cursor_blinking: bool,
    #[cfg(any(test, feature = "test-support"))]
    input_log: Vec<Vec<u8>>,
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
    fn process_pty_event(&mut self, event: PtyEvent, cx: &mut Context<Self>) {
        match event {
            PtyEvent::Event(event) => self.process_event(event, cx),
            #[cfg(feature = "libghostty-vt")]
            PtyEvent::Output(output) => self.write_ghostty_output(&output, cx),
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn write_ghostty_output(&mut self, bytes: &[u8], cx: &mut Context<Self>) {
        let Some(ghostty) = self.ghostty.as_mut() else {
            log::error!("received ghostty pty output without a ghostty backend");
            return;
        };

        if cx.has_global::<GlobalTheme>() {
            ghostty.set_dark_color_scheme(cx.theme().appearance == Appearance::Dark);
        }
        ghostty.write_output(bytes);
        for event in ghostty.drain_events() {
            self.process_event(event, cx);
        }
        self.process_event(TerminalBackendEvent::Wakeup, cx);
    }

    fn process_event(&mut self, event: TerminalBackendEvent, cx: &mut Context<Self>) {
        match event {
            TerminalBackendEvent::Title(title) => {
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
            #[cfg(feature = "alacritty-backend")]
            TerminalBackendEvent::ResetTitle => {
                self.breadcrumb_text = String::new();
                cx.emit(Event::BreadcrumbsChanged);
            }
            TerminalBackendEvent::ClipboardStore(data) => {
                cx.write_to_clipboard(ClipboardItem::new_string(data))
            }
            TerminalBackendEvent::ClipboardLoad(format) => {
                self.write_to_pty(
                    match &cx.read_from_clipboard().and_then(|item| item.text()) {
                        // The terminal only supports pasting strings, not images.
                        Some(text) => format(text),
                        _ => format(""),
                    }
                    .into_bytes(),
                )
            }
            TerminalBackendEvent::PtyWrite(out) => self.write_to_pty(out.into_bytes()),
            #[cfg(feature = "alacritty-backend")]
            TerminalBackendEvent::TextAreaSizeRequest(format) => {
                self.write_to_pty(format(self.last_content.terminal_bounds).into_bytes())
            }
            #[cfg(feature = "alacritty-backend")]
            TerminalBackendEvent::CursorBlinkingChange => {
                #[cfg(feature = "libghostty-vt")]
                if self.ghostty.is_some() {
                    cx.emit(Event::BlinkChanged(false));
                    return;
                }

                #[cfg(feature = "alacritty-backend")]
                {
                    let Some(term) = self.term.as_ref() else {
                        log::error!("received cursor blinking change without an alacritty backend");
                        return;
                    };
                    let terminal = term.lock();
                    let blinking = terminal.cursor_style().blinking;
                    cx.emit(Event::BlinkChanged(blinking));
                }
            }
            TerminalBackendEvent::Bell => {
                cx.emit(Event::Bell);
            }
            #[cfg(feature = "alacritty-backend")]
            TerminalBackendEvent::Exit => self.register_task_finished(Some(9), cx),
            #[cfg(feature = "alacritty-backend")]
            TerminalBackendEvent::MouseCursorDirty => {
                //NOOP, Handled in render
            }
            TerminalBackendEvent::Wakeup => {
                cx.emit(Event::Wakeup);

                if let TerminalType::Pty { info, .. } = &self.terminal_type {
                    info.emit_title_changed_if_changed(cx);
                }
            }
            TerminalBackendEvent::ColorRequest(index, format) => {
                // It's important that the color request is processed here to retain relative order
                // with other PTY writes. Otherwise applications might witness out-of-order
                // responses to requests. For example: An application sending `OSC 11 ; ? ST`
                // (color request) followed by `CSI c` (request device attributes) would receive
                // the response to `CSI c` first.
                // Instead of locking, we could store the colors in `self.last_content`. But then
                // we might respond with out of date value if a "set color" sequence is immediately
                // followed by a color request sequence.
                #[cfg(feature = "libghostty-vt")]
                if self.ghostty.is_some() {
                    let color =
                        terminal_rgb_from_color(get_color_at_index(index, cx.theme().as_ref()));
                    self.write_to_pty(format(color).into_bytes());
                    return;
                }

                #[cfg(feature = "alacritty-backend")]
                {
                    let color = self
                        .term
                        .as_ref()
                        .and_then(|term| term.lock().colors()[index])
                        .unwrap_or_else(|| {
                            to_vte_rgb(get_color_at_index(index, cx.theme().as_ref()))
                        })
                        .into();
                    self.write_to_pty(format(color).into_bytes());
                }
            }
            TerminalBackendEvent::ChildExit(raw_status) => {
                self.register_task_finished(Some(raw_status), cx);
            }
        }
    }

    pub fn selection_started(&self) -> bool {
        self.selection_phase == SelectionPhase::Selecting
    }

    #[cfg(feature = "alacritty-backend")]
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
                    pty_tx.resize(new_bounds);
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
                    .collect::<Vec<(usize, AlacCell)>>();

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
                term.scroll_display(scroll.to_alacritty());
                self.refresh_hovered_word(window);

                if self.vi_mode_enabled {
                    match *scroll {
                        TerminalScroll::Delta(delta) => {
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, delta);
                        }
                        TerminalScroll::PageUp => {
                            let lines = term.screen_lines() as i32;
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, lines);
                        }
                        TerminalScroll::PageDown => {
                            let lines = -(term.screen_lines() as i32);
                            term.vi_mode_cursor = term.vi_mode_cursor.scroll(term, lines);
                        }
                        TerminalScroll::Top => {
                            let point = AlacPoint::new(term.topmost_line(), Column(0));
                            term.vi_mode_cursor = ViModeCursor::new(point);
                        }
                        TerminalScroll::Bottom => {
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

                        self.selection_head = Some(terminal_point_from_alacritty(point));
                        cx.emit(Event::SelectionsChanged)
                    }
                }
            }
            InternalEvent::SetSelection(selection) => {
                trace!("Setting selection: selection={selection:?}");
                term.selection = selection.as_ref().map(TerminalSelection::to_alacritty);

                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                if let Some(selection_text) = term.selection_to_string() {
                    cx.write_to_primary(ClipboardItem::new_string(selection_text));
                }

                if let Some(selection) = selection {
                    self.selection_head = Some(selection.head);
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

                    selection.update(point.to_alacritty(), side.to_alacritty());
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
            InternalEvent::ScrollToPoint(point) => {
                trace!("Scrolling to point: point={point:?}");
                term.scroll_to_point(point.to_alacritty());
                self.refresh_hovered_word(window);
            }
            InternalEvent::MoveViCursorToPoint(point) => {
                trace!("Move vi cursor to point: point={point:?}");
                term.vi_goto_point(point.to_alacritty());
                self.refresh_hovered_word(window);
            }
            InternalEvent::ToggleViMode => {
                trace!("Toggling vi mode");
                self.vi_mode_enabled = !self.vi_mode_enabled;
                term.toggle_vi_mode();
            }
            InternalEvent::ViMotion(motion) => {
                trace!("Performing vi motion: motion={motion:?}");
                term.vi_motion(motion.to_alacritty());
            }
            InternalEvent::FindHyperlink(position, open) => {
                trace!("Finding hyperlink at position: position={position:?}, open={open:?}");

                let point = grid_point(
                    *position,
                    self.last_content.terminal_bounds,
                    term.grid().display_offset(),
                )
                .to_alacritty()
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

    fn process_hyperlink(&mut self, hyperlink: HyperlinkMatch, open: bool, cx: &mut Context<Self>) {
        let HyperlinkMatch {
            text: maybe_url_or_path,
            is_url,
            range,
        } = hyperlink;
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
            self.update_selected_word(prev_hovered_word, range, maybe_url_or_path, target, cx);
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn find_ghostty_hyperlink(&mut self, point: TerminalPoint) -> Option<HyperlinkMatch> {
        terminal_hyperlinks::find_from_content_point(
            &self.last_content,
            point,
            &mut self.hyperlink_regex_searches,
            self.path_style,
        )
    }

    fn find_hyperlink_at_point(&mut self, point: TerminalPoint) -> Option<HyperlinkMatch> {
        #[cfg(feature = "libghostty-vt")]
        if self.ghostty.is_some() {
            return self.find_ghostty_hyperlink(point);
        }

        #[cfg(feature = "alacritty-backend")]
        {
            let Some(term) = self.term.as_ref() else {
                log::error!("tried to find an alacritty hyperlink without an alacritty backend");
                return None;
            };
            let term_lock = term.lock();
            terminal_hyperlinks::find_from_grid_point(
                &term_lock,
                point.to_alacritty(),
                &mut self.hyperlink_regex_searches,
                self.path_style,
            )
        }
        #[cfg(not(feature = "alacritty-backend"))]
        {
            let _ = point;
            None
        }
    }

    fn update_selected_word(
        &mut self,
        prev_word: Option<HoveredWord>,
        word_match: TerminalRange,
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
        #[cfg(feature = "libghostty-vt")]
        if let Some(ghostty) = self.ghostty.as_mut() {
            ghostty.set_default_cursor_shape(cursor_shape.into());
            return;
        }

        #[cfg(feature = "alacritty-backend")]
        {
            let (Some(term_config), Some(term)) = (self.term_config.as_mut(), self.term.as_ref())
            else {
                log::error!("tried to update alacritty cursor shape without an alacritty backend");
                return;
            };
            term_config.default_cursor_style = alacritty_cursor_style(cursor_shape);
            term.lock().set_options(term_config.clone());
        }
        #[cfg(not(feature = "alacritty-backend"))]
        let _ = cursor_shape;
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

        #[cfg(feature = "libghostty-vt")]
        if self.ghostty.is_some() {
            self.write_ghostty_output(&converted, cx);
            return;
        }

        #[cfg(feature = "alacritty-backend")]
        {
            let mut processor = vte::ansi::Processor::<vte::ansi::StdSyncHandler>::new();
            {
                let Some(alacritty_term) = self.term.as_ref() else {
                    log::error!("tried to write output without a terminal backend");
                    return;
                };
                let mut term = alacritty_term.lock();
                processor.advance(&mut *term, &converted);
            }
            cx.emit(Event::Wakeup);
        }
        #[cfg(not(feature = "alacritty-backend"))]
        let _ = (converted, cx);
    }

    pub fn total_lines(&self) -> usize {
        #[cfg(feature = "libghostty-vt")]
        if let Some(ghostty) = &self.ghostty {
            match ghostty.total_lines() {
                Ok(total_lines) => return total_lines,
                Err(error) => {
                    log::error!("failed to read ghostty terminal total rows: {error}");
                    return self.last_content.terminal_bounds.num_lines();
                }
            }
        }

        #[cfg(feature = "alacritty-backend")]
        {
            self.term
                .as_ref()
                .map(|term| term.lock_unfair().total_lines())
                .unwrap_or_else(|| self.last_content.terminal_bounds.num_lines())
        }
        #[cfg(not(feature = "alacritty-backend"))]
        {
            self.last_content.terminal_bounds.num_lines()
        }
    }

    pub fn viewport_lines(&self) -> usize {
        #[cfg(feature = "libghostty-vt")]
        if let Some(ghostty) = &self.ghostty {
            match ghostty.viewport_lines() {
                Ok(viewport_lines) => return viewport_lines,
                Err(error) => {
                    log::error!("failed to read ghostty terminal viewport rows: {error}");
                    return self.last_content.terminal_bounds.num_lines();
                }
            }
        }

        #[cfg(feature = "alacritty-backend")]
        {
            self.term
                .as_ref()
                .map(|term| term.lock_unfair().screen_lines())
                .unwrap_or_else(|| self.last_content.terminal_bounds.num_lines())
        }
        #[cfg(not(feature = "alacritty-backend"))]
        {
            self.last_content.terminal_bounds.num_lines()
        }
    }

    //To test:
    //- Activate match on terminal (scrolling and selection)
    //- Editor search snapping behavior

    pub fn activate_match(&mut self, index: usize) {
        if let Some(search_match) = self.matches.get(index).cloned() {
            self.set_selection(Some(TerminalSelection::simple_range(search_match)));
            if self.vi_mode_enabled {
                self.events
                    .push_back(InternalEvent::MoveViCursorToPoint(search_match.end()));
            } else {
                self.events
                    .push_back(InternalEvent::ScrollToPoint(search_match.start()));
            }
        }
    }

    pub fn select_matches(&mut self, matches: &[TerminalRange]) {
        let matches_to_select = self
            .matches
            .iter()
            .filter(|self_match| matches.contains(self_match))
            .cloned()
            .collect::<Vec<_>>();
        for match_to_select in matches_to_select {
            self.set_selection(Some(TerminalSelection::simple_range(match_to_select)));
        }
    }

    pub fn select_all(&mut self) {
        #[cfg(feature = "libghostty-vt")]
        if self.ghostty.is_some() {
            let start = self
                .last_content
                .cells
                .first()
                .map(|cell| TerminalPoint::new(cell.point.line, 0))
                .unwrap_or_else(|| TerminalPoint::new(0, 0));
            let end = self
                .last_content
                .cells
                .last()
                .map(|cell| {
                    TerminalPoint::new(
                        cell.point.line,
                        self.last_content
                            .terminal_bounds
                            .num_columns()
                            .saturating_sub(1),
                    )
                })
                .unwrap_or_else(|| {
                    TerminalPoint::new(
                        0,
                        self.last_content
                            .terminal_bounds
                            .num_columns()
                            .saturating_sub(1),
                    )
                });
            self.set_selection(Some(TerminalSelection::simple_range(TerminalRange::new(
                start, end,
            ))));
            return;
        }

        #[cfg(feature = "alacritty-backend")]
        {
            let Some(alacritty_term) = self.term.as_ref() else {
                self.set_selection(None);
                return;
            };
            let term = alacritty_term.lock();
            let start = AlacPoint::new(term.topmost_line(), Column(0));
            let end = AlacPoint::new(term.bottommost_line(), term.last_column());
            drop(term);
            self.set_selection(Some(TerminalSelection::simple_range(
                TerminalRange::from_alacritty(start..=end),
            )));
        }
        #[cfg(not(feature = "alacritty-backend"))]
        self.set_selection(None);
    }

    fn set_selection(&mut self, selection: Option<TerminalSelection>) {
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
            .push_back(InternalEvent::Scroll(TerminalScroll::Delta(1)));
    }

    pub fn scroll_up_by(&mut self, lines: usize) {
        self.events
            .push_back(InternalEvent::Scroll(TerminalScroll::Delta(lines as i32)));
    }

    pub fn scroll_line_down(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(TerminalScroll::Delta(-1)));
    }

    pub fn scroll_down_by(&mut self, lines: usize) {
        self.events
            .push_back(InternalEvent::Scroll(TerminalScroll::Delta(
                -(lines as i32),
            )));
    }

    pub fn scroll_page_up(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(TerminalScroll::PageUp));
    }

    pub fn scroll_page_down(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(TerminalScroll::PageDown));
    }

    pub fn scroll_to_top(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(TerminalScroll::Top));
    }

    pub fn scroll_to_bottom(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(TerminalScroll::Bottom));
    }

    pub fn scrolled_to_top(&self) -> bool {
        self.last_content.scrolled_to_top
    }

    pub fn scrolled_to_bottom(&self) -> bool {
        self.last_content.scrolled_to_bottom
    }

    ///Resize the terminal and the PTY.
    pub fn set_size(&mut self, new_bounds: TerminalBounds) {
        let mut new_bounds = new_bounds;
        new_bounds.bounds.size.height = cmp::max(new_bounds.line_height, new_bounds.height());
        new_bounds.bounds.size.width = cmp::max(new_bounds.cell_width, new_bounds.width());

        let old_bounds = self.last_content.terminal_bounds;
        self.last_content.terminal_bounds = new_bounds;

        // Avoid spamming PTY resizes on pixel-level size changes (e.g. while dragging edges),
        // since those can generate excessive SIGWINCH/reflows and cause visible flicker.
        let requires_resize = old_bounds.num_lines() != new_bounds.num_lines()
            || old_bounds.num_columns() != new_bounds.num_columns()
            || old_bounds.cell_width != new_bounds.cell_width
            || old_bounds.line_height != new_bounds.line_height;

        if !requires_resize {
            return;
        }

        match self.events.back_mut() {
            Some(InternalEvent::Resize(pending_bounds)) => *pending_bounds = new_bounds,
            _ => self.events.push_back(InternalEvent::Resize(new_bounds)),
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
            .push_back(InternalEvent::Scroll(TerminalScroll::Bottom));
        self.events.push_back(InternalEvent::SetSelection(None));

        self.keyboard_input_sent = true;
        let input = input.into();
        #[cfg(any(test, feature = "test-support"))]
        self.input_log.push(input.to_vec());

        self.write_to_pty(input);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn take_input_log(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.input_log)
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

        let motion: Option<TerminalViMotion> = match key.as_ref() {
            "h" | "left" => Some(TerminalViMotion::Left),
            "j" | "down" => Some(TerminalViMotion::Down),
            "k" | "up" => Some(TerminalViMotion::Up),
            "l" | "right" => Some(TerminalViMotion::Right),
            "w" => Some(TerminalViMotion::WordRight),
            "b" if !keystroke.modifiers.control => Some(TerminalViMotion::WordLeft),
            "e" => Some(TerminalViMotion::WordRightEnd),
            "%" => Some(TerminalViMotion::Bracket),
            "$" => Some(TerminalViMotion::Last),
            "0" => Some(TerminalViMotion::First),
            "^" => Some(TerminalViMotion::FirstOccupied),
            "H" => Some(TerminalViMotion::High),
            "M" => Some(TerminalViMotion::Middle),
            "L" => Some(TerminalViMotion::Low),
            _ => None,
        };

        if let Some(motion) = motion {
            let cursor = self.last_content.cursor.point;
            let cursor_pos = Point {
                x: cursor.column as f32 * self.last_content.terminal_bounds.cell_width,
                y: cursor.line as f32 * self.last_content.terminal_bounds.line_height,
            };
            self.events
                .push_back(InternalEvent::UpdateSelection(cursor_pos));
            self.events.push_back(InternalEvent::ViMotion(motion));
            return;
        }

        let scroll_motion = match key.as_ref() {
            "g" => Some(TerminalScroll::Top),
            "G" => Some(TerminalScroll::Bottom),
            "b" if keystroke.modifiers.control => Some(TerminalScroll::PageUp),
            "f" if keystroke.modifiers.control => Some(TerminalScroll::PageDown),
            "d" if keystroke.modifiers.control => {
                let amount = self.last_content.terminal_bounds.line_height().to_f64() as i32 / 2;
                Some(TerminalScroll::Delta(-amount))
            }
            "u" if keystroke.modifiers.control => {
                let amount = self.last_content.terminal_bounds.line_height().to_f64() as i32 / 2;
                Some(TerminalScroll::Delta(amount))
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
                let selection_type = TerminalSelectionType::Simple;
                let side = TerminalSelectionSide::Right;
                let selection = TerminalSelection::new(selection_type, point, side);
                self.events
                    .push_back(InternalEvent::SetSelection(Some(selection)));
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

        #[cfg(feature = "libghostty-vt")]
        if let Some(ghostty) = self.ghostty.as_mut() {
            return match ghostty.encode_key(keystroke, option_as_meta) {
                Ok(Some(bytes)) => {
                    self.input(bytes);
                    true
                }
                Ok(None) => false,
                Err(error) => {
                    log::error!("failed to encode ghostty key input: {error}");
                    false
                }
            };
        }

        // Keep default terminal behavior
        let esc = to_esc_str(keystroke, self.last_content.mode, option_as_meta);
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
        let paste_text = if self
            .last_content
            .mode
            .contains(TerminalModes::BRACKETED_PASTE)
        {
            format!("{}{}{}", "\x1b[200~", text.replace('\x1b', ""), "\x1b[201~")
        } else {
            text.replace("\r\n", "\r").replace('\n', "\r")
        };

        self.input(paste_text.into_bytes());
    }

    pub fn sync(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        #[cfg(feature = "libghostty-vt")]
        if self.ghostty.is_some() {
            self.sync_ghostty(window, cx);
            return;
        }

        #[cfg(feature = "alacritty-backend")]
        {
            let Some(term) = self.term.clone() else {
                log::error!("tried to sync alacritty terminal without an alacritty backend");
                return;
            };
            let mut terminal = term.lock_unfair();
            //Note that the ordering of events matters for event processing
            while let Some(e) = self.events.pop_front() {
                self.process_terminal_event(&e, &mut terminal, window, cx)
            }

            self.last_content = Self::make_content(&terminal, &self.last_content);
        }

        #[cfg(not(feature = "alacritty-backend"))]
        {
            self.events.clear();
            let _ = (window, cx);
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn sync_ghostty(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut selection_changed = false;
        while let Some(event) = self.events.pop_front() {
            match event {
                InternalEvent::Resize(new_bounds) => {
                    self.last_content.terminal_bounds = new_bounds;
                    if let TerminalType::Pty { pty_tx, .. } = &self.terminal_type {
                        pty_tx.resize(new_bounds);
                    }
                    if let Some(ghostty) = self.ghostty.as_mut()
                        && let Err(error) = ghostty.resize(new_bounds)
                    {
                        log::error!("failed to resize ghostty terminal: {error}");
                    }
                    if !self.matches.is_empty() {
                        cx.emit(Event::Wakeup);
                    }
                }
                InternalEvent::Clear => {
                    if let Some(ghostty) = self.ghostty.as_mut() {
                        if let Err(error) = ghostty.clear() {
                            log::error!("failed to clear ghostty terminal: {error}");
                        }
                    }
                    cx.emit(Event::Wakeup);
                }
                InternalEvent::Scroll(scroll) => {
                    if let Some(ghostty) = self.ghostty.as_mut() {
                        match scroll {
                            TerminalScroll::Delta(delta) => {
                                for _ in 0..delta.unsigned_abs() {
                                    if delta.is_positive() {
                                        ghostty.scroll_line_up();
                                    } else {
                                        ghostty.scroll_line_down();
                                    }
                                }
                            }
                            TerminalScroll::PageUp => {
                                for _ in 0..self.last_content.terminal_bounds.num_lines() {
                                    ghostty.scroll_line_up();
                                }
                            }
                            TerminalScroll::PageDown => {
                                for _ in 0..self.last_content.terminal_bounds.num_lines() {
                                    ghostty.scroll_line_down();
                                }
                            }
                            TerminalScroll::Top => ghostty.scroll_to_top(),
                            TerminalScroll::Bottom => ghostty.scroll_to_bottom(),
                        }
                    }
                    self.refresh_hovered_word(window);
                }
                InternalEvent::Copy(keep_selection) => {
                    if let Some(selection_text) = self.last_content.selection_text.clone() {
                        cx.write_to_clipboard(ClipboardItem::new_string(selection_text));
                        if !keep_selection.unwrap_or_else(|| {
                            let settings = TerminalSettings::get_global(cx);
                            settings.keep_selection_on_copy
                        }) {
                            self.events.push_back(InternalEvent::SetSelection(None));
                        }
                    }
                }
                InternalEvent::SetSelection(None) => {
                    self.ghostty_selection = None;
                    self.last_content.selection = None;
                    self.last_content.selection_text = None;
                    self.selection_head = None;
                    selection_changed = true;
                    cx.emit(Event::SelectionsChanged)
                }
                InternalEvent::SetSelection(Some(selection)) => {
                    self.selection_head = Some(selection.head);
                    self.ghostty_selection = Some(selection);
                    selection_changed = true;
                    cx.emit(Event::SelectionsChanged)
                }
                InternalEvent::UpdateSelection(position) => {
                    if let Some(selection) = self.ghostty_selection.as_mut() {
                        let (point, side) = grid_point_and_side(
                            position,
                            self.last_content.terminal_bounds,
                            self.last_content.display_offset,
                        );
                        selection.update(point, side);
                        self.selection_head = Some(point);
                        selection_changed = true;
                    }
                    cx.emit(Event::SelectionsChanged)
                }
                InternalEvent::FindHyperlink(position, open) => {
                    let point = grid_point(
                        position,
                        self.last_content.terminal_bounds,
                        self.last_content.display_offset,
                    );
                    match self.find_ghostty_hyperlink(point) {
                        Some(hyperlink) => {
                            self.process_hyperlink(hyperlink, open, cx);
                        }
                        None => {
                            self.last_content.last_hovered_word = None;
                            cx.emit(Event::NewNavigationTarget(None));
                        }
                    }
                }
                InternalEvent::ProcessHyperlink(hyperlink, open) => {
                    self.process_hyperlink(hyperlink, open, cx);
                }
                InternalEvent::ScrollToPoint(point) => {
                    if let Some(ghostty) = self.ghostty.as_mut() {
                        ghostty.scroll_to_point(
                            point,
                            self.last_content.display_offset,
                            self.last_content.terminal_bounds.num_lines(),
                        );
                    }
                    self.refresh_hovered_word(window);
                }
                InternalEvent::MoveViCursorToPoint(point) => {
                    if let Some(ghostty) = self.ghostty.as_mut() {
                        ghostty.scroll_to_point(
                            point,
                            self.last_content.display_offset,
                            self.last_content.terminal_bounds.num_lines(),
                        );
                    }
                    self.set_ghostty_vi_cursor(point, &mut selection_changed, cx);
                    self.refresh_hovered_word(window);
                }
                InternalEvent::ToggleViMode => {
                    self.vi_mode_enabled = !self.vi_mode_enabled;
                    self.ghostty_vi_cursor = self
                        .vi_mode_enabled
                        .then_some(self.last_content.cursor.point);
                    self.ghostty_cursor_blinking = false;
                    cx.emit(Event::BlinkChanged(false));
                }
                InternalEvent::ViMotion(motion) => {
                    if self.vi_mode_enabled {
                        let cursor = self
                            .ghostty_vi_cursor
                            .unwrap_or(self.last_content.cursor.point);
                        let cursor = ghostty_vi_motion(&self.last_content, cursor, motion);
                        if let Some(ghostty) = self.ghostty.as_mut() {
                            ghostty.scroll_to_point(
                                cursor,
                                self.last_content.display_offset,
                                self.last_content.terminal_bounds.num_lines(),
                            );
                        }
                        self.set_ghostty_vi_cursor(cursor, &mut selection_changed, cx);
                    }
                }
            }
        }
        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
        let _ = selection_changed;

        let ghostty_events = self
            .ghostty
            .as_ref()
            .map(|ghostty| ghostty.drain_events())
            .unwrap_or_default();
        for event in ghostty_events {
            self.process_event(event, cx);
        }

        if let Some(ghostty) = self.ghostty.as_mut() {
            match ghostty.content(&self.last_content) {
                Ok(mut content) => {
                    let cursor_blinking = !self.vi_mode_enabled && ghostty.cursor_blinking();
                    if cursor_blinking != self.ghostty_cursor_blinking {
                        self.ghostty_cursor_blinking = cursor_blinking;
                        cx.emit(Event::BlinkChanged(cursor_blinking));
                    }
                    self.apply_ghostty_vi_mode(&mut content);
                    self.apply_ghostty_selection(&mut content);
                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                    if selection_changed
                        && let Some(selection_text) = content.selection_text.clone()
                    {
                        cx.write_to_primary(ClipboardItem::new_string(selection_text));
                    }
                    self.last_content = content;
                }
                Err(error) => log::error!("failed to build ghostty terminal content: {error}"),
            }
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn set_ghostty_vi_cursor(
        &mut self,
        point: TerminalPoint,
        selection_changed: &mut bool,
        cx: &mut Context<Self>,
    ) {
        self.ghostty_vi_cursor = Some(point);
        if let Some(selection) = self.ghostty_selection.as_mut() {
            selection.update_vi(point);
            self.selection_head = Some(point);
            *selection_changed = true;
            cx.emit(Event::SelectionsChanged);
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn apply_ghostty_vi_mode(&mut self, content: &mut TerminalContent) {
        if self.vi_mode_enabled {
            content.mode.insert(TerminalModes::VI);
            let cursor = self
                .ghostty_vi_cursor
                .map(|cursor| clamp_ghostty_content_point(content, cursor))
                .unwrap_or(content.cursor.point);
            self.ghostty_vi_cursor = Some(cursor);
            content.cursor.point = cursor;
            content.cursor_char = ghostty_selection_cell(content, cursor)
                .map(|cell| cell.c)
                .unwrap_or(' ');
        } else {
            content.mode.remove(TerminalModes::VI);
            self.ghostty_vi_cursor = None;
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn apply_ghostty_selection(&self, content: &mut TerminalContent) {
        if let Some(selection) = &self.ghostty_selection
            && let Some(range) = selection.to_range(content)
        {
            content.selection_text = Some(selection.selected_text(content, &range));
            content.selection = Some(range);
            return;
        }

        content.selection = None;
        content.selection_text = None;
    }

    #[cfg(feature = "alacritty-backend")]
    fn make_content(term: &Term<ZedListener>, last_content: &TerminalContent) -> TerminalContent {
        let content = term.renderable_content();

        // Pre-allocate with estimated size to reduce reallocations
        let estimated_size = content.display_iter.size_hint().0;
        let mut cells = Vec::with_capacity(estimated_size);

        cells.extend(content.display_iter.map(|ic| IndexedCell {
            point: terminal_point_from_alacritty(ic.point),
            cell: terminal_cell_from_alacritty(ic.cell.clone()),
        }));

        let selection_text = if content.selection.is_some() {
            term.selection_to_string()
        } else {
            None
        };

        TerminalContent {
            cells,
            mode: terminal_modes_from_alacritty(content.mode),
            display_offset: content.display_offset,
            selection_text,
            selection: content
                .selection
                .map(terminal_selection_range_from_alacritty),
            cursor: TerminalCursor::from_alacritty(content.cursor),
            cursor_char: term.grid()[content.cursor.point].c,
            terminal_bounds: last_content.terminal_bounds,
            last_hovered_word: last_content.last_hovered_word.clone(),
            scrolled_to_top: content.display_offset == term.history_size(),
            scrolled_to_bottom: content.display_offset == 0,
        }
    }

    pub fn get_content(&self) -> String {
        #[cfg(feature = "libghostty-vt")]
        if let Some(ghostty) = &self.ghostty {
            match ghostty.formatted_content() {
                Ok(content) => return content,
                Err(error) => {
                    log::error!("failed to format ghostty terminal content: {error}");
                    return Self::content_to_text(&self.last_content);
                }
            }
        }

        #[cfg(feature = "alacritty-backend")]
        {
            let Some(alacritty_term) = self.term.as_ref() else {
                return String::new();
            };
            let term = alacritty_term.lock_unfair();
            let start = AlacPoint::new(term.topmost_line(), Column(0));
            let end = AlacPoint::new(term.bottommost_line(), term.last_column());
            term.bounds_to_string(start, end)
        }
        #[cfg(not(feature = "alacritty-backend"))]
        {
            String::new()
        }
    }

    pub fn last_n_non_empty_lines(&self, n: usize) -> Vec<String> {
        #[cfg(feature = "libghostty-vt")]
        if let Some(ghostty) = &self.ghostty {
            match ghostty.formatted_content() {
                Ok(content) => return Self::last_n_non_empty_lines_from_text(&content, n),
                Err(error) => {
                    log::error!("failed to format ghostty terminal content: {error}");
                    return Self::last_n_non_empty_lines_from_text(
                        &Self::content_to_text(&self.last_content),
                        n,
                    );
                }
            }
        }

        #[cfg(feature = "alacritty-backend")]
        {
            let Some(term) = self.term.clone() else {
                return Vec::new();
            };
            let terminal = term.lock_unfair();
            let grid = terminal.grid();
            let mut lines = Vec::new();

            let mut current_line = grid.bottommost_line().0;
            let topmost_line = grid.topmost_line().0;

            while current_line >= topmost_line && lines.len() < n {
                let logical_line_start =
                    self.find_logical_line_start(grid, current_line, topmost_line);
                let logical_line =
                    self.construct_logical_line(grid, logical_line_start, current_line);

                if let Some(line) = Self::process_line(logical_line) {
                    lines.push(line);
                }

                // Move to the line above the start of the current logical line
                current_line = logical_line_start - 1;
            }

            lines.reverse();
            lines
        }
        #[cfg(not(feature = "alacritty-backend"))]
        {
            let _ = n;
            Vec::new()
        }
    }

    #[cfg(feature = "libghostty-vt")]
    fn last_n_non_empty_lines_from_text(content: &str, n: usize) -> Vec<String> {
        let mut lines = content
            .lines()
            .rev()
            .filter_map(|line| Self::process_line(line.to_string()))
            .take(n)
            .collect::<Vec<_>>();
        lines.reverse();
        lines
    }

    #[cfg(feature = "libghostty-vt")]
    fn content_to_text(content: &TerminalContent) -> String {
        let mut text = String::new();
        let mut current_line = None;
        for indexed_cell in &content.cells {
            if Some(indexed_cell.point.line) != current_line {
                if current_line.is_some() {
                    text.push('\n');
                }
                current_line = Some(indexed_cell.point.line);
            }

            if indexed_cell.cell.is_wide_char_spacer() {
                continue;
            }

            text.push(indexed_cell.cell.c);
            if let Some(chars) = indexed_cell.cell.zerowidth() {
                for character in chars {
                    text.push(*character);
                }
            }
        }
        text
    }

    #[cfg(feature = "alacritty-backend")]
    fn find_logical_line_start(&self, grid: &Grid<AlacCell>, current: i32, topmost: i32) -> i32 {
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

    #[cfg(feature = "alacritty-backend")]
    fn construct_logical_line(&self, grid: &Grid<AlacCell>, start: i32, end: i32) -> String {
        let mut logical_line = String::new();
        for row in start..=end {
            let grid_row = &grid[Line(row)];
            logical_line.push_str(&row_to_string(grid_row));
        }
        logical_line
    }

    fn process_line(line: String) -> Option<String> {
        let trimmed = line.trim_end().to_string();
        if !trimmed.is_empty() {
            Some(trimmed)
        } else {
            None
        }
    }

    pub fn focus_in(&self) {
        #[cfg(feature = "libghostty-vt")]
        if let Some(ghostty) = &self.ghostty {
            match ghostty.encode_focus(true) {
                Ok(Some(bytes)) => self.write_to_pty(bytes),
                Ok(None) => {}
                Err(error) => log::error!("failed to encode ghostty focus-in input: {error}"),
            }
            return;
        }

        if self.last_content.mode.contains(TerminalModes::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[I".as_bytes());
        }
    }

    pub fn focus_out(&mut self) {
        #[cfg(feature = "libghostty-vt")]
        if let Some(ghostty) = &self.ghostty {
            match ghostty.encode_focus(false) {
                Ok(Some(bytes)) => self.write_to_pty(bytes),
                Ok(None) => {}
                Err(error) => log::error!("failed to encode ghostty focus-out input: {error}"),
            }
            return;
        }

        if self.last_content.mode.contains(TerminalModes::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[O".as_bytes());
        }
    }

    fn mouse_changed(&mut self, point: TerminalPoint, side: TerminalSelectionSide) -> bool {
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
        self.last_content.mode.intersects(TerminalModes::MOUSE_MODE) && !shift
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
                #[cfg(feature = "libghostty-vt")]
                let bytes = if let Some(ghostty) = self.ghostty.as_mut() {
                    match ghostty.encode_mouse_motion(
                        point,
                        self.last_content.terminal_bounds,
                        e.pressed_button,
                        e.modifiers,
                    ) {
                        Ok(bytes) => bytes,
                        Err(error) => {
                            log::error!("failed to encode ghostty mouse-motion input: {error}");
                            None
                        }
                    }
                } else {
                    mouse_moved_report(point, e.pressed_button, e.modifiers, self.last_content.mode)
                };

                #[cfg(not(feature = "libghostty-vt"))]
                let bytes = mouse_moved_report(
                    point,
                    e.pressed_button,
                    e.modifiers,
                    self.last_content.mode,
                );

                if let Some(bytes) = bytes {
                    self.write_to_pty(bytes);
                }
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
        let selection = TerminalSelection::new(TerminalSelectionType::Semantic, point, side);
        self.events
            .push_back(InternalEvent::SetSelection(Some(selection)));
    }

    pub fn mouse_drag(
        &mut self,
        e: &MouseMoveEvent,
        region: Bounds<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let position = e.position - self.last_content.terminal_bounds.bounds.origin;
        if !self.mouse_mode(e.modifiers.shift) {
            if let Some(hyperlink) = &self.mouse_down_hyperlink {
                let point = grid_point(
                    position,
                    self.last_content.terminal_bounds,
                    self.last_content.display_offset,
                );

                if !hyperlink.range.contains(point) {
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
            if !self.last_content.mode.contains(TerminalModes::ALT_SCREEN) {
                let scroll_lines = match self.drag_line_delta(e, region) {
                    Some(value) => value,
                    None => return,
                };

                self.events
                    .push_back(InternalEvent::Scroll(TerminalScroll::Delta(scroll_lines)));
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
            self.mouse_down_hyperlink = self.find_hyperlink_at_point(point);

            if self.mouse_down_hyperlink.is_some() {
                return;
            }
        }

        if self.mouse_mode(e.modifiers.shift) {
            #[cfg(feature = "libghostty-vt")]
            let bytes = if let Some(ghostty) = self.ghostty.as_mut() {
                match ghostty.encode_mouse_button(
                    point,
                    self.last_content.terminal_bounds,
                    e.button,
                    e.modifiers,
                    true,
                ) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        log::error!("failed to encode ghostty mouse-down input: {error}");
                        None
                    }
                }
            } else {
                mouse_button_report(point, e.button, e.modifiers, true, self.last_content.mode)
            };

            #[cfg(not(feature = "libghostty-vt"))]
            let bytes =
                mouse_button_report(point, e.button, e.modifiers, true, self.last_content.mode);

            if let Some(bytes) = bytes {
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
                        1 => Some(TerminalSelectionType::Simple),
                        2 => Some(TerminalSelectionType::Semantic),
                        3 => Some(TerminalSelectionType::Lines),
                        _ => None,
                    };

                    if selection_type == Some(TerminalSelectionType::Simple) && e.modifiers.shift {
                        self.events
                            .push_back(InternalEvent::UpdateSelection(position));
                        return;
                    }

                    let selection = selection_type
                        .map(|selection_type| TerminalSelection::new(selection_type, point, side));

                    if let Some(selection) = selection {
                        self.events
                            .push_back(InternalEvent::SetSelection(Some(selection)));
                    }
                }
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                MouseButton::Middle => {
                    if let Some(item) = _cx.read_from_primary() {
                        let text = item.text().unwrap_or_default();
                        self.paste(&text);
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

            #[cfg(feature = "libghostty-vt")]
            let bytes = if let Some(ghostty) = self.ghostty.as_mut() {
                match ghostty.encode_mouse_button(
                    point,
                    self.last_content.terminal_bounds,
                    e.button,
                    e.modifiers,
                    false,
                ) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        log::error!("failed to encode ghostty mouse-up input: {error}");
                        None
                    }
                }
            } else {
                mouse_button_report(point, e.button, e.modifiers, false, self.last_content.mode)
            };

            #[cfg(not(feature = "libghostty-vt"))]
            let bytes =
                mouse_button_report(point, e.button, e.modifiers, false, self.last_content.mode);

            if let Some(bytes) = bytes {
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

                if let Some(mouse_up_hyperlink) = self.find_hyperlink_at_point(point) {
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
                if let Some(link) = self
                    .last_content
                    .cells
                    .get(mouse_cell_index)
                    .and_then(|cell| cell.hyperlink())
                {
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

                #[cfg(feature = "libghostty-vt")]
                if let Some(ghostty) = self.ghostty.as_mut() {
                    match ghostty.encode_mouse_scroll(
                        point,
                        self.last_content.terminal_bounds,
                        scroll_lines,
                        e,
                    ) {
                        Ok(scrolls) => {
                            for scroll in scrolls {
                                self.write_to_pty(scroll);
                            }
                        }
                        Err(error) => {
                            log::error!("failed to encode ghostty mouse-scroll input: {error}");
                        }
                    }
                } else if let Some(scrolls) =
                    scroll_report(point, scroll_lines, e, self.last_content.mode)
                {
                    for scroll in scrolls {
                        self.write_to_pty(scroll);
                    }
                }

                #[cfg(not(feature = "libghostty-vt"))]
                if let Some(scrolls) = scroll_report(point, scroll_lines, e, self.last_content.mode)
                {
                    for scroll in scrolls {
                        self.write_to_pty(scroll);
                    }
                };
            } else if self
                .last_content
                .mode
                .contains(TerminalModes::ALT_SCREEN | TerminalModes::ALTERNATE_SCROLL)
                && !e.shift
            {
                self.write_to_pty(alt_scroll(scroll_lines));
            } else {
                self.events
                    .push_back(InternalEvent::Scroll(TerminalScroll::Delta(scroll_lines)));
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
        searcher: TerminalSearch,
        cx: &Context<Self>,
    ) -> Task<Vec<TerminalRange>> {
        #[cfg(feature = "libghostty-vt")]
        if self.ghostty.is_some() {
            let content = self.last_content.clone();
            return cx.background_spawn(async move {
                let Some(regex) = searcher.ghostty() else {
                    return Vec::new();
                };
                ghostty_content_search_matches(content, regex)
            });
        }

        #[cfg(feature = "alacritty-backend")]
        {
            let Some(term) = self.term.clone() else {
                return Task::ready(Vec::new());
            };
            cx.background_spawn(async move {
                let Some(mut searcher) = searcher.alacritty() else {
                    return Vec::new();
                };
                let term = term.lock();

                all_search_matches(&term, &mut searcher)
                    .map(TerminalRange::from_alacritty)
                    .collect()
            })
        }
        #[cfg(not(feature = "alacritty-backend"))]
        {
            let _ = (searcher, cx);
            Task::ready(Vec::new())
        }
    }

    pub fn working_directory(&self) -> Option<PathBuf> {
        if self.is_remote_terminal {
            // We can't yet reliably detect the working directory of a shell on the
            // SSH host. Until we can do that, it doesn't make sense to display
            // the working directory on the client and persist that.
            None
        } else {
            #[cfg(feature = "libghostty-vt")]
            if let Some(ghostty) = &self.ghostty {
                match ghostty.working_directory(self.path_style) {
                    Ok(Some(working_directory)) => return Some(working_directory),
                    Ok(None) => {}
                    Err(error) => {
                        log::error!("failed to read ghostty terminal working directory: {error}")
                    }
                }
            }

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

    pub fn backend_name(&self) -> &'static str {
        #[cfg(feature = "libghostty-vt")]
        if self.ghostty.is_some() {
            return "Ghostty";
        }

        #[cfg(feature = "alacritty-backend")]
        {
            "Alacritty"
        }
        #[cfg(all(not(feature = "alacritty-backend"), feature = "libghostty-vt"))]
        {
            "Ghostty"
        }
        #[cfg(not(any(feature = "alacritty-backend", feature = "libghostty-vt")))]
        {
            "None"
        }
    }

    pub fn is_ghostty_backend(&self) -> bool {
        #[cfg(feature = "libghostty-vt")]
        {
            self.ghostty.is_some()
        }
        #[cfg(not(feature = "libghostty-vt"))]
        {
            false
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
                // For interactive shells (no task), we need to differentiate:
                // 1. User-initiated exits (typed "exit", Ctrl+D, etc.) - always close,
                //    even if the shell exits with a non-zero code (e.g. after `false`).
                // 2. Shell spawn failures (bad $SHELL) - don't close, so the user sees
                //    the error. Spawn failures never receive keyboard input.
                let should_close = if self.keyboard_input_sent {
                    true
                } else {
                    self.child_exited.is_none_or(|e| e.code() == Some(0))
                };
                if should_close {
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
        let hide = task.spawned_task.hide;

        if !lines_to_show.is_empty() {
            #[cfg(feature = "libghostty-vt")]
            if self.ghostty.is_some() {
                let mut summary = Vec::new();
                for line in &lines_to_show {
                    summary.extend_from_slice(b"\r\n");
                    summary.extend_from_slice(line.as_bytes());
                }
                self.write_ghostty_output(&summary, cx);
            }

            #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
            if self.ghostty.is_none() {
                // SAFETY: the invocation happens on non `TaskStatus::Running` tasks, once,
                // after either `AlacTermEvent::Exit` or `AlacTermEvent::ChildExit` events that are spawned
                // when Zed task finishes and no more output is made.
                // After the task summary is output once, no more text is appended to the terminal.
                if let Some(term) = self.term.as_ref() {
                    unsafe { append_text_to_term(&mut term.lock(), &lines_to_show) };
                }
            }

            #[cfg(all(feature = "alacritty-backend", not(feature = "libghostty-vt")))]
            // SAFETY: the invocation happens on non `TaskStatus::Running` tasks, once,
            // after either `AlacTermEvent::Exit` or `AlacTermEvent::ChildExit` events that are spawned
            // when Zed task finishes and no more output is made.
            // After the task summary is output once, no more text is appended to the terminal.
            if let Some(term) = self.term.as_ref() {
                unsafe { append_text_to_term(&mut term.lock(), &lines_to_show) };
            }
        }

        match hide {
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

#[cfg(feature = "alacritty-backend")]
fn row_to_string(row: &Row<AlacCell>) -> String {
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
#[cfg(feature = "alacritty-backend")]
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
            pty_tx.shutdown();
            info.terminate_child_process();

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

#[cfg(feature = "alacritty-backend")]
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
    let clamped_col = min(col, terminal_bounds.num_columns().saturating_sub(1));
    let row = (pos.y / terminal_bounds.line_height()).round() as usize;
    let clamped_row = min(row, terminal_bounds.num_lines().saturating_sub(1));
    clamped_row * terminal_bounds.num_columns() + clamped_col
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
        IndexedCell, TerminalBounds, TerminalBuilder, TerminalCell, TerminalContent,
        content_index_for_mouse, rgb_for_index,
    };
    #[cfg(feature = "alacritty-backend")]
    use alacritty_terminal::index::{Column, Line, Point as AlacPoint};
    use async_channel::Receiver;
    use collections::HashMap;
    #[cfg(feature = "alacritty-backend")]
    use gpui::MouseMoveEvent;
    #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
    use gpui::UpdateGlobal;
    #[cfg(feature = "libghostty-vt")]
    use gpui::VisualContext as _;
    use gpui::{
        Entity, Modifiers, MouseButton, MouseDownEvent, MouseUpEvent, Pixels, Point,
        TestAppContext, bounds, point, size,
    };
    use parking_lot::Mutex;
    use rand::{Rng, distr, rngs::StdRng};
    use task::{Shell, ShellBuilder};

    #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
    fn set_ghostty_terminal_feature_flag_override(cx: &mut App, override_key: &str) {
        settings::SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                content.feature_flags.get_or_insert_default().insert(
                    GhosttyTerminalFeatureFlag::NAME.to_string(),
                    override_key.to_string(),
                );
            });
        });
    }

    #[cfg(not(target_os = "windows"))]
    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
    }

    /// Helper to build a test terminal running a shell command.
    /// Returns the terminal entity and a receiver for the completion signal.
    async fn build_test_terminal(
        cx: &mut TestAppContext,
        command: &str,
        args: &[&str],
    ) -> (Entity<Terminal>, Receiver<Option<ExitStatus>>) {
        let (completion_tx, completion_rx) = async_channel::unbounded();
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

    #[cfg(feature = "alacritty-backend")]
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
                cx,
            )
            .unwrap()
            .subscribe(cx)
        });

        terminal.update(cx, |terminal, cx| {
            terminal.write_output(output, cx);
        });

        cx.run_until_parked();

        terminal.update(cx, |terminal, _cx| {
            let term = terminal.term.as_ref().expect("missing alacritty terminal");
            let term_lock = term.lock();
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

    #[cfg(feature = "alacritty-backend")]
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

    fn drain_terminal_events(receiver: &async_channel::Receiver<Event>) -> Vec<Event> {
        std::iter::from_fn(|| receiver.try_recv().ok()).collect()
    }

    #[gpui::test]
    async fn test_basic_terminal(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let (terminal, completion_rx) = build_test_terminal(cx, "echo", &["hello"]).await;
        assert_eq!(
            completion_rx.recv().await.unwrap(),
            Some(ExitStatus::default())
        );
        assert_content_eventually(&terminal, "hello", cx).await;

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

        let (completion_tx, completion_rx) = async_channel::unbounded();
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

        let (event_tx, event_rx) = async_channel::unbounded::<Event>();
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

    #[cfg(not(target_os = "windows"))]
    #[gpui::test(iterations = 10)]
    async fn test_terminal_closes_after_nonzero_exit(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

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
                    None,
                    cx,
                    Vec::new(),
                    PathStyle::local(),
                )
            })
            .await
            .unwrap();
        let terminal = cx.new(|cx| builder.subscribe(cx));

        let (event_tx, event_rx) = async_channel::unbounded::<Event>();
        cx.update(|cx| {
            cx.subscribe(&terminal, move |_, e, _| {
                event_tx.send_blocking(e.clone()).unwrap();
            })
        })
        .detach();

        let first_event = event_rx.recv().await.expect("No wakeup event received");

        terminal.update(cx, |terminal, _| {
            terminal.input(b"false\r".to_vec());
        });
        cx.executor().timer(Duration::from_millis(500)).await;
        terminal.update(cx, |terminal, _| {
            terminal.input(b"exit\r".to_vec());
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
            "Shell exiting after `false && exit` should close terminal, but got events: {all_events:?}",
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_terminal_no_exit_on_spawn_failure(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let (completion_tx, completion_rx) = async_channel::unbounded();
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

    #[cfg(feature = "alacritty-backend")]
    #[test]
    fn test_terminal_modes_round_trip_alacritty_flags() {
        let alacritty_modes = TermMode::APP_CURSOR
            | TermMode::BRACKETED_PASTE
            | TermMode::ALT_SCREEN
            | TermMode::MOUSE_DRAG
            | TermMode::SGR_MOUSE
            | TermMode::VI;

        let terminal_modes = terminal_modes_from_alacritty(alacritty_modes);
        assert!(terminal_modes.contains(TerminalModes::APP_CURSOR));
        assert!(terminal_modes.contains(TerminalModes::BRACKETED_PASTE));
        assert!(terminal_modes.contains(TerminalModes::ALT_SCREEN));
        assert!(terminal_modes.contains(TerminalModes::MOUSE_DRAG));
        assert!(terminal_modes.intersects(TerminalModes::MOUSE_MODE));
        assert!(terminal_modes.contains(TerminalModes::SGR_MOUSE));
        assert!(terminal_modes.contains(TerminalModes::VI));
        assert!(!terminal_modes.contains(TerminalModes::MOUSE_REPORT_CLICK));

        let alacritty_modes = terminal_modes.to_alacritty();
        assert!(alacritty_modes.contains(TermMode::APP_CURSOR));
        assert!(alacritty_modes.contains(TermMode::BRACKETED_PASTE));
        assert!(alacritty_modes.contains(TermMode::ALT_SCREEN));
        assert!(alacritty_modes.contains(TermMode::MOUSE_DRAG));
        assert!(alacritty_modes.contains(TermMode::SGR_MOUSE));
        assert!(alacritty_modes.contains(TermMode::VI));
        assert!(!alacritty_modes.contains(TermMode::MOUSE_REPORT_CLICK));
    }

    #[cfg(feature = "alacritty-backend")]
    #[test]
    fn test_terminal_selection_range_round_trip_alacritty_range() {
        let alacritty_range = SelectionRange {
            start: AlacPoint::new(Line(-2), Column(3)),
            end: AlacPoint::new(Line(4), Column(8)),
            is_block: true,
        };

        let terminal_range = terminal_selection_range_from_alacritty(alacritty_range);
        assert_eq!(
            terminal_range,
            TerminalSelectionRange {
                start: TerminalPoint {
                    line: -2,
                    column: 3
                },
                end: TerminalPoint { line: 4, column: 8 },
                is_block: true,
            }
        );
    }

    #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
    #[gpui::test]
    fn test_ghostty_terminal_feature_flag_gates_backend(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            feature_flags::FeatureFlagStore::init(cx);

            set_ghostty_terminal_feature_flag_override(cx, "off");
            assert_eq!(
                TerminalBackendKind::selected(cx),
                TerminalBackendKind::Alacritty
            );
            set_ghostty_terminal_feature_flag_override(cx, "on");
            assert_eq!(
                TerminalBackendKind::selected(cx),
                TerminalBackendKind::Ghostty
            );
        });
    }

    #[cfg(all(feature = "libghostty-vt", feature = "alacritty-backend"))]
    #[gpui::test]
    fn test_ghostty_terminal_feature_flag_selects_display_only_backend(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            feature_flags::FeatureFlagStore::init(cx);

            set_ghostty_terminal_feature_flag_override(cx, "off");
            let alacritty_builder = TerminalBuilder::new_display_only(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
                cx,
            )
            .unwrap();
            assert_eq!(alacritty_builder.terminal.backend_name(), "Alacritty");
            assert!(!alacritty_builder.terminal.is_ghostty_backend());
            assert!(alacritty_builder.terminal.ghostty.is_none());
            assert!(alacritty_builder.terminal.term.is_some());

            set_ghostty_terminal_feature_flag_override(cx, "on");
            let ghostty_builder = TerminalBuilder::new_display_only(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
                cx,
            )
            .unwrap();
            assert_eq!(ghostty_builder.terminal.backend_name(), "Ghostty");
            assert!(ghostty_builder.terminal.is_ghostty_backend());
            assert!(ghostty_builder.terminal.ghostty.is_some());
            assert!(ghostty_builder.terminal.term.is_none());
            assert!(ghostty_builder.terminal.term_config.is_none());
        });
    }

    #[cfg(all(feature = "libghostty-vt", not(target_os = "windows")))]
    #[gpui::test]
    async fn test_ghostty_terminal_feature_flag_selects_real_pty_backend(cx: &mut TestAppContext) {
        init_test(cx);
        #[cfg(feature = "alacritty-backend")]
        cx.update(enable_ghostty_terminal_feature_flag_for_tests);
        cx.executor().allow_parking();

        let (terminal, completion_rx) =
            build_test_terminal(cx, "echo", &["ghostty feature flag"]).await;
        assert_eq!(
            completion_rx.recv().await.unwrap(),
            Some(ExitStatus::default())
        );
        terminal.update(cx, |terminal, _| {
            assert!(terminal.ghostty.is_some());
            #[cfg(feature = "alacritty-backend")]
            assert!(terminal.term.is_none());
            #[cfg(feature = "alacritty-backend")]
            assert!(terminal.term_config.is_none());
        });
        assert_content_eventually(&terminal, "ghostty feature flag", cx).await;
    }

    #[gpui::test]
    fn test_mouse_to_cell_test(mut rng: StdRng) {
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

    #[gpui::test]
    fn test_mouse_to_cell_clamp(mut rng: StdRng) {
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

    #[cfg(feature = "alacritty-backend")]
    #[gpui::test]
    async fn test_set_size_coalesces_pixel_only_changes(cx: &mut TestAppContext) {
        let builder = cx.update(|cx| {
            TerminalBuilder::new_display_only(
                CursorShape::Block,
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
                cx,
            )
            .unwrap()
        });
        let mut terminal = builder.terminal;

        let base_bounds = TerminalBounds {
            cell_width: Pixels::from(10.),
            line_height: Pixels::from(10.),
            bounds: bounds(
                Point::default(),
                size(Pixels::from(100.), Pixels::from(100.)),
            ),
        };

        terminal.set_size(base_bounds);
        terminal.events.clear();
        assert_eq!(terminal.last_content.terminal_bounds, base_bounds);

        // Pixel-only change: height grows by 1px but still the same number of rows/cols.
        let mut pixel_changed = base_bounds;
        pixel_changed.bounds.size.height = Pixels::from(101.);
        terminal.set_size(pixel_changed);
        assert!(terminal.events.is_empty());
        assert_eq!(terminal.last_content.terminal_bounds, pixel_changed);

        // Grid change: height increases enough to add a row.
        let mut grid_changed = base_bounds;
        grid_changed.bounds.size.height = Pixels::from(110.);
        terminal.set_size(grid_changed);
        assert!(matches!(
            terminal.events.back(),
            Some(InternalEvent::Resize(_))
        ));
    }

    fn get_cells(size: TerminalBounds, rng: &mut StdRng) -> Vec<Vec<char>> {
        let mut cells = Vec::new();

        for _ in 0..size.num_lines() {
            let mut row_vec = Vec::new();
            for _ in 0..size.num_columns() {
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
                    point: TerminalPoint::new(index as i32, cell_index),
                    cell: TerminalCell {
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

    #[cfg(feature = "alacritty-backend")]
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
                cx,
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
            let term = terminal
                .term
                .as_ref()
                .expect("missing alacritty terminal")
                .lock_unfair();
            Terminal::make_content(&term, &terminal.last_content)
        });

        // If LF is properly converted to CRLF, each line should start at column 0
        // The diagonal staircase bug would cause increasing column positions

        // Get the cells and check that lines start at column 0
        let cells = &content.cells;
        let mut line1_col0 = false;
        let mut line2_col0 = false;

        for cell in cells {
            if cell.c == 'l' && cell.point.column == 0 {
                if cell.point.line == 0 && !line1_col0 {
                    line1_col0 = true;
                } else if cell.point.line == 1 && !line2_col0 {
                    line2_col0 = true;
                }
            }
        }

        assert!(line1_col0, "First line should start at column 0");
        assert!(line2_col0, "Second line should start at column 0");
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_write_output(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        terminal.update(cx, |terminal, cx| {
            assert_eq!(terminal.backend_name(), "Ghostty");
            assert!(terminal.is_ghostty_backend());
            assert!(terminal.ghostty.is_some());
            #[cfg(feature = "alacritty-backend")]
            assert!(terminal.term.is_none());
            #[cfg(feature = "alacritty-backend")]
            assert!(terminal.term_config.is_none());
            terminal.write_output(b"ghostty\r\nbackend", cx);
        });

        let content = terminal.update(cx, |terminal, _cx| terminal.get_content());
        assert!(
            content.contains("ghostty") && content.contains("backend"),
            "expected ghostty display-only backend to format injected output, got: {content:?}",
        );
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_uses_key_encoder(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        terminal.update(cx, |terminal, _cx| {
            assert!(terminal.try_keystroke(&Keystroke::parse("ctrl-c").unwrap(), false));
            assert!(terminal.try_keystroke(&Keystroke::parse("up").unwrap(), false));
            assert!(terminal.try_keystroke(&Keystroke::parse("shift-enter").unwrap(), false));
            assert!(!terminal.try_keystroke(&Keystroke::parse("a").unwrap(), false));

            assert_eq!(
                terminal.take_input_log(),
                vec![b"\x03".to_vec(), b"\x1b[A".to_vec(), b"\n".to_vec()]
            );
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_uses_configured_cursor_shape(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::Bar,
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(120.0), px(30.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(b"cursor", cx);
            terminal.sync(window, cx);
            assert_eq!(terminal.last_content.cursor.shape, TerminalCursorShape::Bar);

            terminal.set_cursor_shape(CursorShape::Underline);
            terminal.sync(window, cx);
            assert_eq!(
                terminal.last_content.cursor.shape,
                TerminalCursorShape::Underline
            );
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_reports_terminal_cursor_blinking(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        let (event_tx, event_rx) = async_channel::unbounded::<Event>();
        window.update(|_, cx| {
            cx.subscribe(&terminal, move |_, event, _| {
                event_tx.send_blocking(event.clone()).unwrap();
            })
            .detach();
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(120.0), px(30.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(b"\x1b[2 q", cx);
            terminal.sync(window, cx);
        });
        drain_terminal_events(&event_rx);

        window.update_window_entity(&terminal, |terminal, window, cx| {
            terminal.write_output(b"\x1b[1 q", cx);
            terminal.sync(window, cx);
        });
        let events = drain_terminal_events(&event_rx);
        assert!(
            events.contains(&Event::BlinkChanged(true)),
            "expected Ghostty cursor blink enable event, got {events:?}",
        );

        window.update_window_entity(&terminal, |terminal, window, cx| {
            terminal.write_output(b"\x1b[2 q", cx);
            terminal.sync(window, cx);
        });
        let events = drain_terminal_events(&event_rx);
        assert!(
            events.contains(&Event::BlinkChanged(false)),
            "expected Ghostty cursor blink disable event, got {events:?}",
        );
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_clear_preserves_current_line_and_modes(
        cx: &mut TestAppContext,
    ) {
        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(200.0), px(30.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(
                b"\x1b[?2004hline 1\r\nline 2\r\nprompt> input\x1b]0;partial",
                cx,
            );
            terminal.sync(window, cx);

            assert!(
                terminal
                    .last_content
                    .mode
                    .contains(TerminalModes::BRACKETED_PASTE)
            );

            terminal.clear();
            terminal.sync(window, cx);

            let content = terminal.get_content();
            assert!(
                content.contains("prompt> input"),
                "expected current prompt line after clear, got: {content:?}",
            );
            assert!(
                !content.contains("line 1") && !content.contains("line 2"),
                "expected previous output to be cleared, got: {content:?}",
            );
            assert_eq!(terminal.last_content.cursor.point.line, 0);
            assert_eq!(
                terminal.last_content.cursor.point.column,
                "prompt> input".len()
            );
            assert!(
                terminal
                    .last_content
                    .mode
                    .contains(TerminalModes::BRACKETED_PASTE)
            );
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_last_n_non_empty_lines(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"first\r\n\r\n  second  \r\nthird\r\n", cx);
            assert_eq!(
                terminal.last_n_non_empty_lines(2),
                vec!["  second".to_string(), "third".to_string()]
            );
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_reports_osc7_working_directory(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        let working_directory = std::env::current_dir()
            .expect("current directory should be available for OSC7 test")
            .join("ghostty osc7 cwd");
        let uri = url::Url::from_directory_path(&working_directory)
            .expect("current directory should form a file URI");
        let sequence = format!("\x1b]7;file://localhost{}\x1b\\", uri.path());

        terminal.update(cx, |terminal, cx| {
            terminal.write_output(sequence.as_bytes(), cx);
            assert_eq!(terminal.working_directory(), Some(working_directory));
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_selection_and_copy(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(120.0), px(30.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(b"hello world\r\nsecond line", cx);
            terminal.sync(window, cx);

            let start = TerminalPoint::new(0, 0);
            let end = TerminalPoint::new(0, 4);
            terminal.set_selection(Some(TerminalSelection::simple_range(TerminalRange::new(
                start, end,
            ))));
            terminal.sync(window, cx);

            assert_eq!(
                terminal.last_content.selection_text.as_deref(),
                Some("hello")
            );
            assert_eq!(
                terminal.last_content.selection,
                Some(TerminalSelectionRange {
                    start,
                    end,
                    is_block: false,
                }),
            );

            terminal.copy(Some(true));
            terminal.sync(window, cx);
            assert_eq!(
                cx.read_from_clipboard().and_then(|item| item.text()),
                Some("hello".to_string())
            );
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_find_matches(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        let matches = window
            .update_window_entity(&terminal, |terminal, window, cx| {
                let terminal_bounds = TerminalBounds::new(
                    px(10.0),
                    px(10.0),
                    bounds(point(px(0.0), px(0.0)), size(px(120.0), px(30.0))),
                );

                terminal.set_size(terminal_bounds);
                terminal.write_output(b"hello WORLD\r\nsecond line", cx);
                terminal.sync(window, cx);

                terminal.find_matches(TerminalSearch::new("world").unwrap(), cx)
            })
            .await;

        let start = TerminalPoint::new(0, 6);
        let end = TerminalPoint::new(0, 10);
        assert_eq!(matches, vec![TerminalRange::new(start, end)]);

        window.update_window_entity(&terminal, |terminal, window, cx| {
            terminal.matches = matches;
            terminal.activate_match(0);
            terminal.sync(window, cx);

            assert_eq!(
                terminal.last_content.selection_text.as_deref(),
                Some("WORLD")
            );
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_activate_match_scrolls_to_point(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(120.0), px(30.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(
                b"line 1\r\nline 2\r\nline 3\r\nline 4\r\nline 5\r\nline 6",
                cx,
            );
            terminal.sync(window, cx);
            assert_eq!(terminal.last_content.display_offset, 0);

            let start = TerminalPoint::new(-2, 0);
            let end = TerminalPoint::new(-2, 5);
            terminal.matches = vec![TerminalRange::new(start, end)];
            terminal.activate_match(0);
            terminal.sync(window, cx);

            assert!(terminal.last_content.display_offset >= 2);
            assert_eq!(
                terminal.last_content.selection,
                Some(TerminalSelectionRange {
                    start,
                    end,
                    is_block: false,
                }),
            );
            assert!(
                terminal
                    .last_content
                    .selection_text
                    .as_deref()
                    .is_some_and(|selection_text| selection_text.starts_with("line "))
            );
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_vi_mode_moves_cursor(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(120.0), px(30.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(b"abcdef\r\nsecond line", cx);
            terminal.sync(window, cx);

            terminal.toggle_vi_mode();
            terminal
                .events
                .push_back(InternalEvent::MoveViCursorToPoint(TerminalPoint::new(0, 0)));
            terminal.sync(window, cx);

            assert!(terminal.last_content.mode.contains(TerminalModes::VI));
            assert_eq!(terminal.last_content.cursor.point, TerminalPoint::new(0, 0));

            assert!(terminal.try_keystroke(&Keystroke::parse("l").unwrap(), false));
            terminal.sync(window, cx);
            assert_eq!(terminal.last_content.cursor.point, TerminalPoint::new(0, 1));

            assert!(terminal.try_keystroke(&Keystroke::parse("$").unwrap(), false));
            terminal.sync(window, cx);
            assert_eq!(terminal.last_content.cursor.point, TerminalPoint::new(0, 5));

            assert!(terminal.try_keystroke(&Keystroke::parse("w").unwrap(), false));
            terminal.sync(window, cx);
            assert_eq!(terminal.last_content.cursor.point, TerminalPoint::new(1, 0));
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_vi_mode_visual_selection(cx: &mut TestAppContext) {
        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(120.0), px(30.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(b"abcdef", cx);
            terminal.sync(window, cx);

            terminal.toggle_vi_mode();
            terminal
                .events
                .push_back(InternalEvent::MoveViCursorToPoint(TerminalPoint::new(0, 0)));
            terminal.sync(window, cx);

            assert!(terminal.try_keystroke(&Keystroke::parse("v").unwrap(), false));
            terminal.sync(window, cx);
            assert!(terminal.try_keystroke(&Keystroke::parse("l").unwrap(), false));
            terminal.sync(window, cx);

            assert_eq!(terminal.last_content.selection_text.as_deref(), Some("ab"));
            assert_eq!(
                terminal.last_content.selection,
                Some(TerminalSelectionRange {
                    start: TerminalPoint::new(0, 0),
                    end: TerminalPoint::new(0, 1),
                    is_block: false,
                }),
            );
        });
    }

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_hyperlink_ctrl_click_same_position(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(20.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(400.0), px(400.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(b"Visit https://zed.dev/ for more\r\n", cx);
            terminal.sync(window, cx);
            terminal.events.clear();

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

    #[cfg(feature = "libghostty-vt")]
    #[gpui::test]
    async fn test_display_only_ghostty_osc8_hyperlink_ctrl_click(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            let builder = TerminalBuilder::new_display_only_ghostty(
                CursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            );
            match builder {
                Ok(builder) => builder.subscribe(cx),
                Err(error) => panic!("failed to build ghostty display-only terminal: {error}"),
            }
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let terminal_bounds = TerminalBounds::new(
                px(20.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(400.0), px(400.0))),
            );

            terminal.set_size(terminal_bounds);
            terminal.write_output(
                b"\x1b]8;;https://zed.dev/docs\x1b\\Docs\x1b]8;;\x1b\\ hidden\r\n",
                cx,
            );
            terminal.sync(window, cx);
            terminal.events.clear();

            let linked_cell = terminal
                .last_content
                .cells
                .iter()
                .find(|cell| cell.c == 'D')
                .expect("missing OSC8 linked cell");
            let hyperlink = linked_cell
                .hyperlink()
                .expect("missing OSC8 hyperlink metadata");
            assert_eq!(hyperlink.uri(), "https://zed.dev/docs");

            let click_position = point(px(10.0), px(5.0));
            ctrl_mouse_down_at(terminal, click_position, cx);
            ctrl_mouse_up_at(terminal, click_position, cx);

            assert!(
                terminal
                    .events
                    .iter()
                    .any(|event| matches!(event, InternalEvent::ProcessHyperlink(_, true))),
                "Should have ProcessHyperlink event when ctrl+clicking an OSC8 hyperlink"
            );
        });
    }

    #[cfg(feature = "alacritty-backend")]
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
                cx,
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
            let term = terminal
                .term
                .as_ref()
                .expect("missing alacritty terminal")
                .lock_unfair();
            Terminal::make_content(&term, &terminal.last_content)
        });

        let cells = &content.cells;

        // Check that both lines start at column 0
        let mut found_lines_at_column_0 = 0;
        for cell in cells {
            if cell.c == 'l' && cell.point.column == 0 {
                found_lines_at_column_0 += 1;
            }
        }

        assert!(
            found_lines_at_column_0 >= 2,
            "Both lines should start at column 0"
        );
    }

    #[cfg(feature = "alacritty-backend")]
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
                cx,
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
            let term = terminal
                .term
                .as_ref()
                .expect("missing alacritty terminal")
                .lock_unfair();
            Terminal::make_content(&term, &terminal.last_content)
        });

        let cells = &content.cells;

        // Check that we have "world" at the beginning of the line
        let mut text = String::new();
        for cell in cells.iter().take(5) {
            if cell.point.line == 0 {
                text.push(cell.c);
            }
        }

        assert!(
            text.starts_with("world"),
            "Bare CR should allow overwriting: got '{}'",
            text
        );
    }

    #[cfg(feature = "alacritty-backend")]
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

    #[cfg(feature = "alacritty-backend")]
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

    #[cfg(feature = "alacritty-backend")]
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

    /// Polls the terminal content until `expected` appears, or panics after ~1s.
    /// The PTY IO thread writes into the terminal grid independently of the
    /// GPUI executor, so we need a real-time polling loop to synchronize.
    async fn assert_content_eventually(
        terminal: &Entity<Terminal>,
        expected: &str,
        cx: &mut TestAppContext,
    ) {
        let mut content = String::new();
        for _ in 0..100 {
            content = terminal.update(cx, |term, _| term.get_content());
            if content.contains(expected) {
                return;
            }
            cx.background_executor
                .timer(Duration::from_millis(10))
                .await;
        }
        panic!("Expected terminal content to contain {expected:?}, got: {content}");
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

        assert_content_eventually(&terminal, "test_output_before_kill", cx).await;

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

        assert_content_eventually(&terminal, "done", cx).await;

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

        #[test]
        fn test_num_lines_float_precision() {
            let line_heights = [
                20.1f32, 16.7, 18.3, 22.9, 14.1, 15.6, 17.8, 19.4, 21.3, 23.7,
            ];
            for &line_height in &line_heights {
                for n in 1..=100 {
                    let height = n as f32 * line_height;
                    let bounds = TerminalBounds::new(
                        px(line_height),
                        px(8.0),
                        Bounds {
                            origin: Point::default(),
                            size: Size {
                                width: px(800.0),
                                height: px(height),
                            },
                        },
                    );
                    assert_eq!(
                        bounds.num_lines(),
                        n,
                        "num_lines() should be {n} for height={height}, line_height={line_height}"
                    );
                }
            }
        }

        #[test]
        fn test_num_columns_float_precision() {
            let cell_widths = [8.1f32, 7.3, 9.7, 6.9, 10.1];
            for &cell_width in &cell_widths {
                for n in 1..=200 {
                    let width = n as f32 * cell_width;
                    let bounds = TerminalBounds::new(
                        px(20.0),
                        px(cell_width),
                        Bounds {
                            origin: Point::default(),
                            size: Size {
                                width: px(width),
                                height: px(400.0),
                            },
                        },
                    );
                    assert_eq!(
                        bounds.num_columns(),
                        n,
                        "num_columns() should be {n} for width={width}, cell_width={cell_width}"
                    );
                }
            }
        }
    }
}
