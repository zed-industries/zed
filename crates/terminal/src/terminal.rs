mod mappings;

mod ghostty_backend;
mod ghostty_pty;
mod ghostty_worker;
mod pty_info;
mod terminal_hyperlinks;
pub mod terminal_settings;

#[cfg(not(windows))]
use anyhow::Context as _;
use anyhow::{Result, bail};
use futures_lite::future::yield_now;

use futures::{
    FutureExt,
    channel::mpsc::{UnboundedReceiver, unbounded},
};

use itertools::Itertools as _;
use mappings::mouse::{alt_scroll, grid_point, grid_point_and_side};

use async_channel::{Receiver, Sender};
use collections::{HashMap, VecDeque};
use futures::StreamExt;
use ghostty_backend::{FullContentBuilder, GhosttyOsc52};
use ghostty_pty::{GhosttyPtyEventLoop, GhosttyPtyNotifier, portable_pty_size};
use ghostty_worker::GhosttyBackendWorker;
use pty_info::{ProcessIdGetter, PtyProcessInfo};
use serde::{Deserialize, Serialize};
use settings::Settings;
use task::{HideStrategy, Shell, SpawnInTerminal};
use terminal_hyperlinks::{HyperlinkMatch, RegexSearches};
use terminal_settings::{AlternateScroll, CursorShape as SettingsCursorShape, TerminalSettings};
use theme::{ActiveTheme, Appearance, GlobalTheme, Theme};
use urlencoding;
use util::{paths::PathStyle, truncate_and_trailoff};

use regex::{Regex, RegexBuilder};
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::{
    borrow::Cow,
    cmp::{self, min},
    fmt::{self, Display, Formatter},
    ops::{BitOr, BitOrAssign, Deref, Range as StdRange},
    path::{Path, PathBuf},
    process::ExitStatus,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use vte::ansi::{Attr, Handler, Processor, StdSyncHandler};
pub use vte::ansi::{Color, NamedColor, Rgb};

use gpui::{
    App, AppContext as _, BackgroundExecutor, Bounds, ClipboardItem, Context, EventEmitter, Hsla,
    Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels,
    Point as GpuiPoint, Rgba, ScrollWheelEvent, Size, Task, TouchPhase, Window, actions, black, px,
};

pub fn is_default_background_color(color: Color) -> bool {
    matches!(color, Color::Named(NamedColor::Background))
}

pub fn is_app_chosen_exact_color(color: Color) -> bool {
    matches!(color, Color::Spec(_) | Color::Indexed(16..=255))
}

pub type AnsiSpans = Vec<(StdRange<usize>, Option<Color>)>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ParsedAnsiText {
    pub text: String,
    pub foreground_spans: AnsiSpans,
    pub background_spans: AnsiSpans,
}

pub fn parse_ansi_text(input: &[u8]) -> ParsedAnsiText {
    let mut handler = StyledAnsiTextHandler::default();
    let mut processor = Processor::<StdSyncHandler>::default();
    processor.advance(&mut handler, input);
    handler.finish()
}

pub fn strip_ansi_text(input: &[u8]) -> String {
    let mut handler = PlainAnsiTextHandler::default();
    let mut processor = Processor::<StdSyncHandler>::default();
    processor.advance(&mut handler, input);
    handler.text
}

#[derive(Default)]
struct StyledAnsiTextHandler {
    text: String,
    foreground_spans: AnsiSpans,
    background_spans: AnsiSpans,
    current_foreground_range_start: usize,
    current_background_range_start: usize,
    current_foreground_color: Option<Color>,
    current_background_color: Option<Color>,
}

impl StyledAnsiTextHandler {
    fn finish(mut self) -> ParsedAnsiText {
        if self.current_foreground_range_start < self.text.len() {
            self.foreground_spans.push((
                self.current_foreground_range_start..self.text.len(),
                self.current_foreground_color,
            ));
        }

        if self.current_background_range_start < self.text.len() {
            self.background_spans.push((
                self.current_background_range_start..self.text.len(),
                self.current_background_color,
            ));
        }

        ParsedAnsiText {
            text: self.text,
            foreground_spans: self.foreground_spans,
            background_spans: self.background_spans,
        }
    }

    fn break_foreground_span(&mut self, color: Option<Color>) {
        self.foreground_spans.push((
            self.current_foreground_range_start..self.text.len(),
            self.current_foreground_color,
        ));
        self.current_foreground_color = color;
        self.current_foreground_range_start = self.text.len();
    }

    fn break_background_span(&mut self, color: Option<Color>) {
        self.background_spans.push((
            self.current_background_range_start..self.text.len(),
            self.current_background_color,
        ));
        self.current_background_color = color;
        self.current_background_range_start = self.text.len();
    }
}

impl Handler for StyledAnsiTextHandler {
    fn input(&mut self, c: char) {
        self.text.push(c);
    }

    fn linefeed(&mut self) {
        self.text.push('\n');
    }

    fn put_tab(&mut self, count: u16) {
        self.text.extend(std::iter::repeat_n('\t', count as usize));
    }

    fn terminal_attribute(&mut self, attr: Attr) {
        match attr {
            Attr::Foreground(color) => {
                self.break_foreground_span(Some(color));
            }
            Attr::Background(color) => {
                self.break_background_span(Some(color));
            }
            Attr::Reset => {
                self.break_foreground_span(None);
                self.break_background_span(None);
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct PlainAnsiTextHandler {
    text: String,
    line_start: usize,
}

impl Handler for PlainAnsiTextHandler {
    fn input(&mut self, c: char) {
        self.text.push(c);
    }

    fn linefeed(&mut self) {
        self.text.push('\n');
        self.line_start = self.text.len();
    }

    fn carriage_return(&mut self) {
        self.text.truncate(self.line_start);
    }

    fn put_tab(&mut self, count: u16) {
        self.text.extend(std::iter::repeat_n('\t', count as usize));
    }
}

const SEMANTIC_ESCAPE_CHARS: &str = ",│`|:\"' ()[]{}<>\t";

#[derive(Clone, Copy, Debug)]
pub(crate) enum Scroll {
    Delta(i32),
    PageUp,
    PageDown,
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum ViMotion {
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

#[derive(Clone, Debug)]
pub struct Search {
    pattern: Arc<str>,
}

impl Search {
    pub fn new(search: &str) -> Option<Self> {
        Regex::new(search).ok()?;

        Some(Self {
            pattern: Arc::from(search),
        })
    }

    pub(crate) fn regex(&self) -> Option<Regex> {
        let has_uppercase = self
            .pattern
            .chars()
            .any(|character| character.is_uppercase());
        RegexBuilder::new(self.pattern.as_ref())
            .case_insensitive(!has_uppercase)
            .build()
            .ok()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Selection {
    ty: SelectionType,
    start: SelectionAnchor,
    end: SelectionAnchor,
    pub(crate) head: Point,
}

#[derive(Clone, Copy, Debug)]
struct SelectionAnchor {
    point: Point,
    side: SelectionSide,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SelectionSide {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SelectionType {
    Simple,
    Semantic,
    Lines,
}

impl Selection {
    pub(crate) fn new(selection_type: SelectionType, point: Point, side: SelectionSide) -> Self {
        let anchor = SelectionAnchor { point, side };
        Self {
            ty: selection_type,
            start: anchor,
            end: anchor,
            head: point,
        }
    }

    pub(crate) fn simple_range(range: Range) -> Self {
        let mut selection = Self::new(SelectionType::Simple, range.start(), SelectionSide::Left);
        selection.update(range.end(), SelectionSide::Right);
        selection
    }

    pub(crate) fn update(&mut self, point: Point, side: SelectionSide) {
        self.end = SelectionAnchor { point, side };
        self.head = point;
    }

    pub(crate) fn update_vi(&mut self, point: Point) {
        self.start.side = SelectionSide::Left;
        self.end = SelectionAnchor {
            point,
            side: SelectionSide::Right,
        };
        self.head = point;
    }

    pub(crate) fn translate_lines(&mut self, delta: i32) {
        self.start.point.line = self.start.point.line.saturating_add(delta);
        self.end.point.line = self.end.point.line.saturating_add(delta);
        self.head.line = self.head.line.saturating_add(delta);
    }

    pub(crate) fn to_range(&self, content: &Content) -> Option<SelectionRange> {
        let (top_line, bottom_line) = content_line_bounds(content)?;
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

        start.point = clamp_selection_point(start.point, top_line, bottom_line, columns);
        end.point = clamp_selection_point(end.point, top_line, bottom_line, columns);

        match self.ty {
            SelectionType::Simple => self.range_simple(start, end, columns),
            SelectionType::Semantic => {
                Some(range_semantic_selection(content, start.point, end.point))
            }
            SelectionType::Lines => Some(SelectionRange {
                start: Point::new(start.point.line, 0),
                end: Point::new(end.point.line, columns.saturating_sub(1)),
                is_block: false,
            }),
        }
    }

    pub(crate) fn selected_text(&self, content: &Content, range: &SelectionRange) -> String {
        match self.ty {
            SelectionType::Lines => {
                let mut text = selection_bounds_text(content, range);
                text.push('\n');
                text
            }
            SelectionType::Simple | SelectionType::Semantic => {
                selection_bounds_text(content, range)
            }
        }
    }

    pub(crate) fn is_fully_within(&self, content: &Content) -> bool {
        let Some((top_line, bottom_line)) = content_line_bounds(content) else {
            return false;
        };
        let columns = content.terminal_bounds.num_columns();
        columns > 0
            && [self.start.point, self.end.point].iter().all(|point| {
                (top_line..=bottom_line).contains(&point.line) && point.column < columns
            })
    }

    fn range_simple(
        &self,
        mut start: SelectionAnchor,
        mut end: SelectionAnchor,
        columns: usize,
    ) -> Option<SelectionRange> {
        if self.is_empty() {
            return None;
        }

        if end.side == SelectionSide::Left && start.point != end.point {
            if end.point.column == 0 {
                end.point.column = columns - 1;
                end.point.line -= 1;
            } else {
                end.point.column -= 1;
            }
        }

        if start.side == SelectionSide::Right && start.point != end.point {
            start.point.column += 1;

            if start.point.column == columns {
                start.point.column = 0;
                start.point.line += 1;
            }
        }

        Some(SelectionRange {
            start: start.point,
            end: end.point,
            is_block: false,
        })
    }

    fn is_empty(&self) -> bool {
        match self.ty {
            SelectionType::Simple => {
                let (mut start, mut end) = (self.start, self.end);
                if start.point > end.point {
                    std::mem::swap(&mut start, &mut end);
                }

                start.point == end.point && start.side == end.side
                    || start.side == SelectionSide::Right
                        && end.side == SelectionSide::Left
                        && start.point.line == end.point.line
                        && start.point.column.checked_add(1) == Some(end.point.column)
            }
            SelectionType::Semantic | SelectionType::Lines => false,
        }
    }
}

pub(crate) fn content_line_bounds(content: &Content) -> Option<(i32, i32)> {
    Some((
        content.cells.first()?.point.line,
        content.cells.last()?.point.line,
    ))
}

fn clamp_selection_point(point: Point, top_line: i32, bottom_line: i32, columns: usize) -> Point {
    Point::new(
        point.line.max(top_line).min(bottom_line),
        point.column.min(columns.saturating_sub(1)),
    )
}

fn range_semantic_selection(content: &Content, start: Point, end: Point) -> SelectionRange {
    SelectionRange {
        start: semantic_search_left(content, start),
        end: semantic_search_right(content, end),
        is_block: false,
    }
}

fn last_column(content: &Content) -> usize {
    content.terminal_bounds.num_columns().saturating_sub(1)
}

fn semantic_search_left(content: &Content, mut point: Point) -> Point {
    while let Some(previous) = previous_point(content, point) {
        if selection_cell_is_wide_spacer(content, previous) {
            point = previous;
            continue;
        }
        if !selection_cell_is_semantic(content, previous) {
            break;
        }
        point = previous;
    }
    point
}

fn semantic_search_right(content: &Content, mut point: Point) -> Point {
    while let Some(next) = next_point(content, point) {
        if selection_cell_is_wide_spacer(content, next) {
            point = next;
            continue;
        }
        if !selection_cell_is_semantic(content, next) {
            break;
        }
        point = next;
    }
    point
}

fn selection_cell_is_semantic(content: &Content, point: Point) -> bool {
    selection_cell(content, point)
        .map(|cell| {
            !cell.is_wide_char_spacer_or_leading()
                && !SEMANTIC_ESCAPE_CHARS.contains(cell.character())
        })
        .unwrap_or(false)
}

fn selection_cell_is_wide_spacer(content: &Content, point: Point) -> bool {
    selection_cell(content, point)
        .map(|cell| cell.is_wide_char_spacer_or_leading())
        .unwrap_or(false)
}

fn selection_bounds_text(content: &Content, range: &SelectionRange) -> String {
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
            last_column(content)
        };

        text.push_str(&selection_line_text(
            content,
            line,
            start_column,
            end_column,
            end_column == last_column(content),
        ));

        if line != range.end.line && !content.is_soft_wrapped_line(line) {
            text.push('\n');
        }
    }
    text
}

fn selection_line_text(
    content: &Content,
    line: i32,
    start_column: usize,
    end_column: usize,
    trim_end: bool,
) -> String {
    let mut text = String::new();
    for column in start_column..=end_column {
        let Some(cell) = selection_cell(content, Point::new(line, column)) else {
            continue;
        };
        if cell.is_wide_char_spacer_or_leading() {
            continue;
        }

        text.push(cell.character());
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

pub(crate) fn selection_cell(content: &Content, point: Point) -> Option<&Cell> {
    content
        .cells
        .binary_search_by_key(&point, |cell| cell.point)
        .ok()
        .and_then(|index| content.cells.get(index))
        .map(|cell| &cell.cell)
}

pub(crate) fn clamp_content_point(content: &Content, point: Point) -> Point {
    let Some((top_line, bottom_line)) = content_line_bounds(content) else {
        return point;
    };

    Point::new(
        point.line.max(top_line).min(bottom_line),
        point.column.min(last_column(content)),
    )
}

pub(crate) fn vi_motion(content: &Content, cursor: Point, motion: ViMotion) -> Point {
    let Some((top_line, bottom_line)) = content_line_bounds(content) else {
        return cursor;
    };

    let cursor = clamp_content_point(content, cursor);
    let last_column = last_column(content);

    match motion {
        ViMotion::Up => Point::new(cursor.line.max(top_line + 1) - 1, cursor.column),
        ViMotion::Down => Point::new(cursor.line.min(bottom_line - 1) + 1, cursor.column),
        ViMotion::Left => previous_point(content, cursor).unwrap_or(cursor),
        ViMotion::Right => next_point(content, cursor).unwrap_or(cursor),
        ViMotion::First => Point::new(cursor.line, 0),
        ViMotion::Last => last_occupied_in_line(content, cursor.line)
            .unwrap_or_else(|| Point::new(cursor.line, last_column)),
        ViMotion::FirstOccupied => first_occupied_in_line(content, cursor.line)
            .unwrap_or_else(|| Point::new(cursor.line, 0)),
        ViMotion::High => line_start(content, top_line),
        ViMotion::Middle => {
            let line = top_line + (bottom_line - top_line) / 2;
            line_start(content, line)
        }
        ViMotion::Low => line_start(content, bottom_line),
        ViMotion::WordLeft => word_start_left(content, cursor).unwrap_or(cursor),
        ViMotion::WordRight => word_start_right(content, cursor).unwrap_or(cursor),
        ViMotion::WordRightEnd => word_end_right(content, cursor).unwrap_or(cursor),
        ViMotion::Bracket => matching_bracket(content, cursor).unwrap_or(cursor),
    }
}

fn line_start(content: &Content, line: i32) -> Point {
    first_occupied_in_line(content, line).unwrap_or_else(|| Point::new(line, 0))
}

fn first_occupied_in_line(content: &Content, line: i32) -> Option<Point> {
    (0..=last_column(content))
        .map(|column| Point::new(line, column))
        .find(|&point| !cell_is_space(content, point))
}

fn last_occupied_in_line(content: &Content, line: i32) -> Option<Point> {
    (0..=last_column(content))
        .rev()
        .map(|column| Point::new(line, column))
        .find(|&point| !cell_is_space(content, point))
}

fn next_point(content: &Content, point: Point) -> Option<Point> {
    let (_, bottom_line) = content_line_bounds(content)?;
    let last_column = last_column(content);

    if point.column < last_column {
        Some(Point::new(point.line, point.column + 1))
    } else if point.line < bottom_line && content.is_soft_wrapped_line(point.line) {
        Some(Point::new(point.line + 1, 0))
    } else {
        None
    }
}

fn previous_point(content: &Content, point: Point) -> Option<Point> {
    let (top_line, _) = content_line_bounds(content)?;
    let last_column = last_column(content);

    if point.column > 0 {
        Some(Point::new(point.line, point.column - 1))
    } else if point.line > top_line && content.is_soft_wrapped_line(point.line - 1) {
        Some(Point::new(point.line - 1, last_column))
    } else {
        None
    }
}

fn cell_is_space(content: &Content, point: Point) -> bool {
    selection_cell(content, point)
        .map(|cell| {
            cell.is_wide_char_spacer_or_leading()
                || cell.character() == ' '
                || cell.character() == '\t'
        })
        .unwrap_or(true)
}

fn word_start_right(content: &Content, mut point: Point) -> Option<Point> {
    if !cell_is_space(content, point) {
        while let Some(next) = next_point(content, point) {
            point = next;
            if cell_is_space(content, point) {
                break;
            }
        }
    }

    while cell_is_space(content, point) {
        point = next_point(content, point)?;
    }

    Some(point)
}

fn word_start_left(content: &Content, mut point: Point) -> Option<Point> {
    point = previous_point(content, point)?;

    while cell_is_space(content, point) {
        point = previous_point(content, point)?;
    }

    while let Some(previous) = previous_point(content, point) {
        if cell_is_space(content, previous) {
            break;
        }
        point = previous;
    }

    Some(point)
}

fn word_end_right(content: &Content, mut point: Point) -> Option<Point> {
    while cell_is_space(content, point) {
        point = next_point(content, point)?;
    }

    while let Some(next) = next_point(content, point) {
        if cell_is_space(content, next) {
            break;
        }
        point = next;
    }

    Some(point)
}

fn matching_bracket(content: &Content, point: Point) -> Option<Point> {
    let character = selection_cell(content, point)?.character();
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
        let cell = selection_cell(content, current)?;
        if cell.character() == character {
            depth = depth.saturating_add(1);
        } else if cell.character() == matching {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(current);
            }
        }

        next = if forward {
            next_point(content, current)
        } else {
            previous_point(content, current)
        };
    }

    None
}

pub(crate) fn content_search_matches(content: Content, regex: Regex) -> Vec<Range> {
    let Some((top_line, bottom_line)) = content_line_bounds(&content) else {
        return Vec::new();
    };

    let mut matches = Vec::new();
    let mut line = top_line;
    while line <= bottom_line {
        let (text, points, end_line) = search_logical_line_text(&content, line, bottom_line);
        if text.is_empty() {
            line = end_line.saturating_add(1);
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

            matches.push(Range::new(*start, *end));
        }
        line = end_line.saturating_add(1);
    }

    matches
}

fn search_logical_line_text(
    content: &Content,
    start_line: i32,
    bottom_line: i32,
) -> (String, Vec<(usize, Point)>, i32) {
    let mut text = String::new();
    let mut points = Vec::new();
    let mut line = start_line;
    loop {
        append_search_line_text(content, line, &mut text, &mut points);
        if line == bottom_line || !content.is_soft_wrapped_line(line) {
            break;
        }
        line += 1;
    }

    (text, points, line)
}

fn append_search_line_text(
    content: &Content,
    line: i32,
    text: &mut String,
    points: &mut Vec<(usize, Point)>,
) {
    for cell in cells_for_line(content, line) {
        if cell.is_wide_char_spacer_or_leading() {
            continue;
        }

        points.push((text.len(), cell.point));
        text.push(cell.character());
        if let Some(chars) = cell.zerowidth() {
            for character in chars {
                points.push((text.len(), cell.point));
                text.push(*character);
            }
        }
    }
}

fn cells_for_line(content: &Content, line: i32) -> &[IndexedCell] {
    let start = content.cells.partition_point(|cell| cell.point.line < line);
    let end = start + content.cells[start..].partition_point(|cell| cell.point.line == line);
    &content.cells[start..end]
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hyperlink {
    id: Option<Arc<str>>,
    uri: Arc<str>,
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
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct CellExtra {
    zerowidth: Vec<char>,
    underline_color: Option<Color>,
    hyperlink: Option<Hyperlink>,
}

impl CellExtra {
    fn is_empty(&self) -> bool {
        self.zerowidth.is_empty() && self.underline_color.is_none() && self.hyperlink.is_none()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CellFlags(u32);

impl CellFlags {
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

    pub(crate) const fn empty() -> Self {
        Self(0)
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

impl BitOr for CellFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for CellFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cell {
    character: char,
    foreground: Color,
    background: Color,
    flags: CellFlags,
    extra: Option<Arc<CellExtra>>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: ' ',
            background: Color::Named(NamedColor::Background),
            foreground: Color::Named(NamedColor::Foreground),
            flags: CellFlags::empty(),
            extra: None,
        }
    }
}

impl Cell {
    pub(crate) fn new(
        character: char,
        foreground: Color,
        background: Color,
        flags: CellFlags,
    ) -> Self {
        Self {
            character,
            foreground,
            background,
            flags,
            extra: None,
        }
    }

    #[inline]
    pub fn character(&self) -> char {
        self.character
    }

    #[inline]
    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }

    #[inline]
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    #[inline]
    pub fn background(&self) -> Color {
        self.background
    }

    #[inline]
    pub fn zerowidth(&self) -> Option<&[char]> {
        self.extra.as_ref().map(|extra| extra.zerowidth.as_slice())
    }

    #[inline]
    pub fn push_zerowidth(&mut self, character: char) {
        Arc::make_mut(
            self.extra
                .get_or_insert_with(|| Arc::new(CellExtra::default())),
        )
        .zerowidth
        .push(character);
    }

    pub fn set_underline_color(&mut self, color: Option<Color>) {
        let Some(color) = color else {
            if let Some(extra) = self.extra.as_mut() {
                let extra = Arc::make_mut(extra);
                extra.underline_color = None;
                if extra.is_empty() {
                    self.extra = None;
                }
            }
            return;
        };

        Arc::make_mut(
            self.extra
                .get_or_insert_with(|| Arc::new(CellExtra::default())),
        )
        .underline_color = Some(color);
    }

    #[inline]
    pub fn underline_color(&self) -> Option<Color> {
        self.extra.as_ref()?.underline_color
    }

    pub fn set_hyperlink(&mut self, hyperlink: Option<Hyperlink>) {
        let Some(hyperlink) = hyperlink else {
            if let Some(extra) = self.extra.as_mut() {
                let extra = Arc::make_mut(extra);
                extra.hyperlink = None;
                if extra.is_empty() {
                    self.extra = None;
                }
            }
            return;
        };

        Arc::make_mut(
            self.extra
                .get_or_insert_with(|| Arc::new(CellExtra::default())),
        )
        .hyperlink = Some(hyperlink);
    }

    #[inline]
    pub fn hyperlink(&self) -> Option<&Hyperlink> {
        self.extra.as_ref()?.hyperlink.as_ref()
    }

    #[inline]
    pub fn is_inverse(&self) -> bool {
        self.flags.contains(CellFlags::INVERSE)
    }

    #[inline]
    pub fn is_wide_char_spacer(&self) -> bool {
        self.flags.contains(CellFlags::WIDE_CHAR_SPACER)
    }

    #[inline]
    pub(crate) fn is_wide_char_spacer_or_leading(&self) -> bool {
        self.flags
            .intersects(CellFlags::WIDE_CHAR_SPACER | CellFlags::LEADING_WIDE_CHAR_SPACER)
    }

    #[inline]
    pub fn is_dim(&self) -> bool {
        self.flags.intersects(CellFlags::DIM)
    }

    #[inline]
    pub fn has_underline(&self) -> bool {
        self.flags.intersects(CellFlags::ALL_UNDERLINES)
    }

    #[inline]
    pub fn has_undercurl(&self) -> bool {
        self.flags.contains(CellFlags::UNDERCURL)
    }

    #[inline]
    pub fn has_strikeout(&self) -> bool {
        self.flags.intersects(CellFlags::STRIKEOUT)
    }

    #[inline]
    pub fn is_bold(&self) -> bool {
        self.flags.intersects(CellFlags::BOLD)
    }

    #[inline]
    pub fn is_italic(&self) -> bool {
        self.flags.intersects(CellFlags::ITALIC)
    }

    #[inline]
    pub fn has_visible_style_modifier(&self) -> bool {
        self.flags
            .intersects(CellFlags::ALL_UNDERLINES | CellFlags::INVERSE | CellFlags::STRIKEOUT)
    }
}

pub struct RenderableCells<'a> {
    cells: std::slice::Iter<'a, IndexedCell>,
}

impl<'a> RenderableCells<'a> {
    pub(crate) fn new(cells: &'a [IndexedCell]) -> Self {
        Self {
            cells: cells.iter(),
        }
    }
}

impl Iterator for RenderableCells<'_> {
    type Item = IndexedCell;

    fn next(&mut self) -> Option<Self::Item> {
        self.cells.next().cloned()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cells.size_hint()
    }
}

#[derive(Debug, Clone)]
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Modes(u32);

impl Modes {
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
}

impl BitOr for Modes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Modes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert(rhs);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cursor {
    pub shape: CursorShape,
    pub point: Point,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CursorShape {
    Block,
    Underline,
    Bar,
    HollowBlock,
    Hidden,
}

impl From<SettingsCursorShape> for CursorShape {
    fn from(shape: SettingsCursorShape) -> Self {
        match shape {
            SettingsCursorShape::Block => Self::Block,
            SettingsCursorShape::Underline => Self::Underline,
            SettingsCursorShape::Bar => Self::Bar,
            SettingsCursorShape::Hollow => Self::HollowBlock,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub line: i32,
    pub column: usize,
}

impl Point {
    pub fn new(line: i32, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Range {
    start: Point,
    end: Point,
}

impl Range {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }

    pub fn contains(&self, point: Point) -> bool {
        self.start <= point && point <= self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectionRange {
    pub start: Point,
    pub end: Point,
    pub is_block: bool,
}

impl SelectionRange {
    pub fn point_range(self) -> Range {
        Range::new(self.start, self.end)
    }
}

// TODO: Un-pub
#[derive(Clone)]
pub struct Content {
    pub cells: Vec<IndexedCell>,
    pub mode: Modes,
    pub display_offset: usize,
    pub soft_wrapped_lines: Vec<i32>,
    pub selection_text: Option<String>,
    pub selection: Option<SelectionRange>,
    pub cursor: Cursor,
    pub cursor_char: char,
    pub terminal_bounds: TerminalBounds,
    pub last_hovered_word: Option<HoveredWord>,
    pub scrolled_to_top: bool,
    pub scrolled_to_bottom: bool,
}

impl Content {
    fn is_soft_wrapped_line(&self, line: i32) -> bool {
        self.soft_wrapped_lines.binary_search(&line).is_ok()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HoveredWord {
    pub word: String,
    pub word_match: Range,
    pub id: usize,
}

impl Default for Content {
    fn default() -> Self {
        Content {
            cells: Default::default(),
            mode: Default::default(),
            display_offset: Default::default(),
            soft_wrapped_lines: Default::default(),
            selection_text: Default::default(),
            selection: Default::default(),
            cursor: Cursor {
                shape: CursorShape::Block,
                point: Point::new(0, 0),
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

#[cfg(test)]
mod domain_tests {
    use super::*;
    use gpui::{bounds, point, px, size};

    fn test_content(rows: &[&str], soft_wrapped_lines: Vec<i32>) -> Content {
        let columns = rows
            .iter()
            .map(|row| row.chars().count())
            .max()
            .unwrap_or(1);
        let cells = rows
            .iter()
            .enumerate()
            .flat_map(|(line, row)| {
                let characters = row.chars().chain(std::iter::repeat(' ')).take(columns);
                characters.enumerate().map(move |(column, character)| {
                    let point = Point::new(line as i32, column);
                    let cell = Cell::new(
                        character,
                        Color::Named(NamedColor::Foreground),
                        Color::Named(NamedColor::Background),
                        CellFlags::empty(),
                    );
                    IndexedCell { point, cell }
                })
            })
            .collect();

        Content {
            cells,
            terminal_bounds: TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(
                    point(px(0.0), px(0.0)),
                    size(px(columns as f32 * 10.0), px(rows.len() as f32 * 10.0)),
                ),
            ),
            soft_wrapped_lines,
            ..Default::default()
        }
    }

    fn test_content_with_cells(
        rows: &[Vec<(char, CellFlags)>],
        soft_wrapped_lines: Vec<i32>,
    ) -> Content {
        let columns = rows.iter().map(|row| row.len()).max().unwrap_or(1);
        let cells = rows
            .iter()
            .enumerate()
            .flat_map(|(line, row)| {
                row.iter()
                    .copied()
                    .chain(std::iter::repeat((' ', CellFlags::empty())))
                    .take(columns)
                    .enumerate()
                    .map(move |(column, (character, flags))| {
                        let point = Point::new(line as i32, column);
                        let cell = Cell::new(
                            character,
                            Color::Named(NamedColor::Foreground),
                            Color::Named(NamedColor::Background),
                            flags,
                        );
                        IndexedCell { point, cell }
                    })
            })
            .collect();

        Content {
            cells,
            terminal_bounds: TerminalBounds::new(
                px(10.0),
                px(10.0),
                bounds(
                    point(px(0.0), px(0.0)),
                    size(px(columns as f32 * 10.0), px(rows.len() as f32 * 10.0)),
                ),
            ),
            soft_wrapped_lines,
            ..Default::default()
        }
    }

    #[test]
    fn terminal_hyperlink_clone_shares_storage() {
        let hyperlink = Hyperlink::new(Some("id"), "https://example.com".to_string());
        let clone = hyperlink.clone();

        assert_eq!(clone.id(), Some("id"));
        assert_eq!(clone.uri(), "https://example.com");
        assert!(Arc::ptr_eq(&hyperlink.uri, &clone.uri));
    }

    #[test]
    fn terminal_cell_clone_shares_extra_storage() {
        let mut cell = Cell::default();
        cell.push_zerowidth('a');

        let clone = cell.clone();

        match (&cell.extra, &clone.extra) {
            (Some(extra), Some(clone_extra)) => assert!(Arc::ptr_eq(extra, clone_extra)),
            _ => panic!("expected extra storage on both cells"),
        }
    }

    #[test]
    fn semantic_selection_stops_at_default_escape_chars() {
        let content = test_content(&["error:42"], Vec::new());

        let word_selection = Selection::new(
            SelectionType::Semantic,
            Point::new(0, 1),
            SelectionSide::Left,
        );
        let word_range = word_selection
            .to_range(&content)
            .expect("word semantic range");
        assert_eq!(word_selection.selected_text(&content, &word_range), "error");

        let field_selection = Selection::new(
            SelectionType::Semantic,
            Point::new(0, 6),
            SelectionSide::Left,
        );
        let field_range = field_selection
            .to_range(&content)
            .expect("field semantic range");
        assert_eq!(field_selection.selected_text(&content, &field_range), "42");
    }

    #[test]
    fn semantic_selection_crosses_soft_wrapped_lines() {
        let content = test_content(&["abc", "def"], vec![0]);

        let first_row_selection = Selection::new(
            SelectionType::Semantic,
            Point::new(0, 1),
            SelectionSide::Left,
        );
        let first_row_range = first_row_selection
            .to_range(&content)
            .expect("first row semantic range");
        assert_eq!(
            first_row_selection.selected_text(&content, &first_row_range),
            "abcdef"
        );

        let continuation_selection = Selection::new(
            SelectionType::Semantic,
            Point::new(1, 1),
            SelectionSide::Left,
        );
        let continuation_range = continuation_selection
            .to_range(&content)
            .expect("continuation semantic range");
        assert_eq!(
            continuation_selection.selected_text(&content, &continuation_range),
            "abcdef"
        );
    }

    #[test]
    fn semantic_selection_crosses_soft_wrapped_wide_spacers() {
        let content = test_content_with_cells(
            &[
                vec![
                    ('a', CellFlags::empty()),
                    (' ', CellFlags::LEADING_WIDE_CHAR_SPACER),
                ],
                vec![
                    ('例', CellFlags::WIDE_CHAR),
                    (' ', CellFlags::WIDE_CHAR_SPACER),
                ],
            ],
            vec![0],
        );

        let first_row_selection = Selection::new(
            SelectionType::Semantic,
            Point::new(0, 0),
            SelectionSide::Left,
        );
        let first_row_range = first_row_selection
            .to_range(&content)
            .expect("first row semantic range");
        assert_eq!(
            first_row_selection.selected_text(&content, &first_row_range),
            "a例"
        );

        let continuation_selection = Selection::new(
            SelectionType::Semantic,
            Point::new(1, 0),
            SelectionSide::Left,
        );
        let continuation_range = continuation_selection
            .to_range(&content)
            .expect("continuation semantic range");
        assert_eq!(
            continuation_selection.selected_text(&content, &continuation_range),
            "a例"
        );
    }

    #[test]
    fn vi_motions_stop_at_hard_line_boundaries() {
        let content = test_content(&["abc", "def"], Vec::new());

        assert_eq!(
            vi_motion(&content, Point::new(0, 2), ViMotion::Right),
            Point::new(0, 2)
        );
        assert_eq!(
            vi_motion(&content, Point::new(1, 0), ViMotion::Left),
            Point::new(1, 0)
        );
        assert_eq!(
            vi_motion(&content, Point::new(0, 2), ViMotion::WordRight),
            Point::new(0, 2)
        );
        assert_eq!(
            vi_motion(&content, Point::new(1, 0), ViMotion::WordLeft),
            Point::new(1, 0)
        );
    }

    #[test]
    fn vi_motions_cross_soft_wrapped_lines() {
        let content = test_content(&["abc", "def"], vec![0]);

        assert_eq!(
            vi_motion(&content, Point::new(0, 2), ViMotion::Right),
            Point::new(1, 0)
        );
        assert_eq!(
            vi_motion(&content, Point::new(1, 0), ViMotion::Left),
            Point::new(0, 2)
        );
    }
}

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
    Scroll(Scroll),
    ScrollToPoint(Point),
    SetSelection(Option<Selection>),
    UpdateSelection(GpuiPoint<Pixels>),
    FindHyperlink(GpuiPoint<Pixels>, bool),
    ProcessHyperlink(HyperlinkMatch, bool),
    // Whether keep selection when copy
    Copy(Option<bool>),
    // Vi mode events
    ToggleViMode,
    ViMotion(ViMotion),
    MoveViCursorToPoint(Point),
}

type ClipboardFormatter = Arc<dyn Fn(&str) -> String + Sync + Send + 'static>;
type ColorFormatter = Arc<dyn Fn(Rgb) -> String + Sync + Send + 'static>;

#[derive(Clone)]
pub(crate) enum TerminalBackendEvent {
    Title(String),
    ClipboardStore(String),
    ClipboardLoad(ClipboardFormatter),
    ColorRequest(usize, ColorFormatter),
    PtyWrite(String),
    Wakeup,
    Bell,
    ChildExit(i32),
}

impl fmt::Debug for TerminalBackendEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Title(title) => write!(f, "Title({title})"),
            Self::ClipboardStore(data) => write!(f, "ClipboardStore({data})"),
            Self::ClipboardLoad(_) => f.write_str("ClipboardLoad"),
            Self::ColorRequest(index, _) => write!(f, "ColorRequest({index})"),
            Self::PtyWrite(output) => write!(f, "PtyWrite({output})"),
            Self::Wakeup => f.write_str("Wakeup"),
            Self::Bell => f.write_str("Bell"),
            Self::ChildExit(status) => write!(f, "ChildExit({status})"),
        }
    }
}

enum PtyEvent {
    Event(TerminalBackendEvent),
    OutputProcessed(Vec<TerminalBackendEvent>),
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
                origin: GpuiPoint::default(),
                size: Size {
                    width: DEBUG_TERMINAL_WIDTH,
                    height: DEBUG_TERMINAL_HEIGHT,
                },
            },
        )
    }
}

fn normalize_terminal_bounds(mut bounds: TerminalBounds) -> TerminalBounds {
    bounds.bounds.size.height = cmp::max(bounds.line_height, bounds.height());
    bounds.bounds.size.width = cmp::max(bounds.cell_width, bounds.width());
    bounds
}

fn vi_cursor_after_scroll(
    cursor: Point,
    scroll: Scroll,
    viewport_lines: usize,
    full_content_range: Option<Range>,
) -> (Point, bool) {
    let viewport_lines = i32::try_from(viewport_lines).unwrap_or(i32::MAX);
    let (cursor, use_first_occupied_column) = match scroll {
        Scroll::Delta(delta) => (
            Point::new(cursor.line.saturating_sub(delta), cursor.column),
            true,
        ),
        Scroll::PageUp => (
            Point::new(cursor.line.saturating_sub(viewport_lines), cursor.column),
            true,
        ),
        Scroll::PageDown => (
            Point::new(cursor.line.saturating_add(viewport_lines), cursor.column),
            true,
        ),
        Scroll::Top => (
            full_content_range
                .map(|range| Point::new(range.start().line, 0))
                .unwrap_or(cursor),
            false,
        ),
        Scroll::Bottom => (
            full_content_range
                .map(|range| Point::new(range.end().line, 0))
                .unwrap_or(cursor),
            false,
        ),
    };

    (
        full_content_range
            .map(|range| clamp_point_to_range(cursor, range))
            .unwrap_or(cursor),
        use_first_occupied_column,
    )
}

fn clamp_point_to_range(point: Point, range: Range) -> Point {
    if point.line < range.start().line {
        Point::new(range.start().line, 0)
    } else if point.line > range.end().line {
        Point::new(range.end().line, 0)
    } else {
        point
    }
}

fn selection_intersects_range(selection: &Selection, range: Range) -> bool {
    let (mut start, mut end) = (selection.start.point, selection.end.point);
    if start > end {
        std::mem::swap(&mut start, &mut end);
    }

    end.line >= range.start().line && start.line <= range.end().line
}

struct TerminalRowText {
    line: i32,
    text: String,
    non_empty: bool,
}

fn terminal_content_row_texts(content: &Content) -> Vec<TerminalRowText> {
    let mut rows = Vec::new();
    let mut current_line = None;
    let mut text = String::new();

    for cell in &content.cells {
        if current_line != Some(cell.point.line) {
            if let Some(line) = current_line {
                push_terminal_row_text(&mut rows, line, std::mem::take(&mut text));
            }
            current_line = Some(cell.point.line);
        }

        if cell.is_wide_char_spacer_or_leading() {
            continue;
        }

        text.push(cell.character());
        if let Some(zerowidth) = cell.zerowidth() {
            text.extend(zerowidth);
        }
    }

    if let Some(line) = current_line {
        push_terminal_row_text(&mut rows, line, text);
    }

    rows
}

fn push_terminal_row_text(rows: &mut Vec<TerminalRowText>, line: i32, text: String) {
    let non_empty = text.chars().any(|character| !character.is_whitespace());
    rows.push(TerminalRowText {
        line,
        text,
        non_empty,
    });
}

fn best_terminal_row_delta(
    previous_rows: &[TerminalRowText],
    current_rows: &[TerminalRowText],
) -> Option<i32> {
    let mut best_match: Option<(usize, usize, i32)> = None;

    for previous_start in 0..previous_rows.len() {
        for current_start in 0..current_rows.len() {
            let mut total_count = 0;
            let mut non_empty_count = 0;

            while let (Some(previous_row), Some(current_row)) = (
                previous_rows.get(previous_start + total_count),
                current_rows.get(current_start + total_count),
            ) {
                if previous_row.text != current_row.text {
                    break;
                }
                total_count += 1;
                if previous_row.non_empty {
                    non_empty_count += 1;
                }
            }

            if non_empty_count == 0 {
                continue;
            }

            let delta = current_rows[current_start]
                .line
                .saturating_sub(previous_rows[previous_start].line);
            let replace_best = match best_match {
                Some((best_non_empty_count, best_total_count, best_delta)) => {
                    non_empty_count > best_non_empty_count
                        || non_empty_count == best_non_empty_count
                            && (total_count > best_total_count
                                || total_count == best_total_count
                                    && delta.unsigned_abs() < best_delta.unsigned_abs())
                }
                None => true,
            };

            if replace_best {
                best_match = Some((non_empty_count, total_count, delta));
            }
        }
    }

    best_match.map(|(_, _, delta)| delta)
}

fn default_system_command() -> portable_pty::CommandBuilder {
    if cfg!(target_os = "macos") {
        portable_pty::CommandBuilder::new_default_prog()
    } else {
        portable_pty::CommandBuilder::new(util::shell::get_system_shell())
    }
}

fn apply_spawn_environment(
    command: &mut portable_pty::CommandBuilder,
    env: &HashMap<String, String>,
    window_id: u64,
) {
    const REMOVED_SPAWN_ENV: &[&str] = &["SHLVL", "XDG_ACTIVATION_TOKEN", "DESKTOP_STARTUP_ID"];

    for key in REMOVED_SPAWN_ENV {
        command.env_remove(key);
    }
    #[cfg(unix)]
    command.env("WINDOWID", window_id.to_string());
    #[cfg(not(unix))]
    let _ = window_id;
    for (key, value) in env {
        if REMOVED_SPAWN_ENV.contains(&key.as_str()) {
            continue;
        }
        command.env(key, value);
    }
}

#[cfg(any(
    target_os = "android",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos"
))]
fn enable_pty_utf8_input(master: &dyn portable_pty::MasterPty) -> std::io::Result<()> {
    let fd = master
        .as_raw_fd()
        .ok_or_else(|| std::io::Error::other("terminal PTY has no raw file descriptor"))?;
    let mut termios = std::mem::MaybeUninit::<libc::termios>::uninit();
    if unsafe { libc::tcgetattr(fd, termios.as_mut_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    let mut termios = unsafe { termios.assume_init() };
    termios.c_iflag |= libc::IUTF8;
    if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &termios) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(all(
    unix,
    not(any(
        target_os = "android",
        target_os = "ios",
        target_os = "linux",
        target_os = "macos"
    ))
))]
fn enable_pty_utf8_input(_master: &dyn portable_pty::MasterPty) -> std::io::Result<()> {
    Ok(())
}

fn validate_working_directory(working_directory: &Path) -> std::io::Result<()> {
    let metadata = std::fs::metadata(working_directory)?;
    if metadata.is_dir() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotADirectory,
            format!("not a directory: {}", working_directory.display()),
        ))
    }
}

#[cfg(unix)]
fn portable_process_id_getter(
    master: &dyn portable_pty::MasterPty,
    child: &dyn portable_pty::Child,
) -> ProcessIdGetter {
    ProcessIdGetter::new(
        master.as_raw_fd().unwrap_or(-1),
        child.process_id().unwrap_or(0),
    )
}

#[cfg(windows)]
fn portable_process_id_getter(
    _master: &dyn portable_pty::MasterPty,
    child: &dyn portable_pty::Child,
) -> ProcessIdGetter {
    ProcessIdGetter::new(
        child
            .as_raw_handle()
            .map(|handle| handle as i32)
            .unwrap_or_default(),
        child.process_id().unwrap_or(0),
    )
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

const DEFAULT_SCROLL_HISTORY_LINES: usize = 10_000;
pub const MAX_SCROLL_HISTORY_LINES: usize = 100_000;
const SEARCH_SNAPSHOT_ROWS_PER_TICK: usize = 512;

struct SearchSnapshotBuilder {
    builder: Option<FullContentBuilder>,
    content_revision: u64,
}

impl SearchSnapshotBuilder {
    fn new(terminal: &Terminal) -> Result<Self> {
        Ok(Self {
            builder: Some(
                terminal
                    .backend
                    .start_full_content(&terminal.last_content)?,
            ),
            content_revision: terminal.content_revision,
        })
    }

    fn append_rows(&mut self, terminal: &mut Terminal, row_count: usize) -> Result<bool> {
        if self.content_revision != terminal.content_revision {
            *self = Self::new(terminal)?;
        }

        let Some(builder) = self.builder.take() else {
            bail!("terminal search snapshot builder missing");
        };

        let (builder, done) = terminal
            .backend
            .append_full_content_rows(builder, row_count)?;
        self.builder = Some(builder);
        Ok(done)
    }

    fn finish(self) -> Option<Content> {
        self.builder.map(FullContentBuilder::finish)
    }
}

pub struct TerminalBuilder {
    terminal: Terminal,
    events_rx: UnboundedReceiver<PtyEvent>,
}

impl TerminalBuilder {
    pub fn new_display_only(
        cursor_shape: SettingsCursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
    ) -> Result<TerminalBuilder> {
        Self::new_display_only_with_bounds(
            cursor_shape,
            alternate_scroll,
            max_scroll_history_lines,
            window_id,
            background_executor,
            path_style,
            TerminalBounds::default(),
        )
    }

    pub fn new_display_only_with_bounds(
        cursor_shape: SettingsCursorShape,
        alternate_scroll: AlternateScroll,
        max_scroll_history_lines: Option<usize>,
        window_id: u64,
        background_executor: &BackgroundExecutor,
        path_style: PathStyle,
        terminal_bounds: TerminalBounds,
    ) -> Result<TerminalBuilder> {
        let terminal_bounds = normalize_terminal_bounds(terminal_bounds);
        let (_events_tx, events_rx) = unbounded();
        let backend = GhosttyBackendWorker::new(terminal_bounds, max_scroll_history_lines, None)?;
        backend.set_default_cursor_shape(cursor_shape.into());
        backend.set_osc52(GhosttyOsc52::Disabled);
        if let AlternateScroll::Off = alternate_scroll {
            backend.set_alternate_scroll(false)?;
        }

        let terminal = Terminal {
            task: None,
            terminal_type: TerminalType::DisplayOnly,
            completion_tx: None,
            backend,
            selection: None,
            vi_cursor: None,
            cursor_blinking: false,
            output_since_refresh: false,
            content_revision: 0,
            title_override: None,
            events: VecDeque::with_capacity(10),
            last_content: Content {
                terminal_bounds,
                ..Default::default()
            },
            last_full_content_range: None,
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
        cursor_shape: SettingsCursorShape,
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
        let initial_dark_color_scheme = cx
            .has_global::<GlobalTheme>()
            .then(|| cx.theme().appearance == Appearance::Dark);
        let fut = async move {
            // Remove SHLVL so the spawned shell initializes it to 1, matching
            // the behavior of standalone terminal emulators.
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

            let mut command = if let Some(params) = shell_params.as_ref() {
                let mut command = portable_pty::CommandBuilder::new(&params.program);
                if let Some(args) = params.args.as_ref() {
                    command.args(args);
                }
                command
            } else {
                default_system_command()
            };
            if let Some(working_directory) = working_directory.as_ref() {
                if let Err(error) = validate_working_directory(working_directory) {
                    bail!(TerminalError {
                        directory: Some(working_directory.clone()),
                        program: shell_params.as_ref().map(|params| params.program.clone()),
                        args: shell_params.as_ref().and_then(|params| params.args.clone()),
                        title_override: terminal_title_override,
                        source: error,
                    });
                }
                command.cwd(working_directory.as_os_str());
            };
            apply_spawn_environment(&mut command, &env, window_id);

            let (events_tx, events_rx) = unbounded();
            let pty_system = portable_pty::native_pty_system();
            let portable_pty::PtyPair { master, slave } =
                match pty_system.openpty(portable_pty_size(TerminalBounds::default())) {
                    Ok(pair) => pair,
                    Err(error) => {
                        bail!(TerminalError {
                            directory: working_directory.clone(),
                            program: shell_params.as_ref().map(|params| params.program.clone()),
                            args: shell_params.as_ref().and_then(|params| params.args.clone()),
                            title_override: terminal_title_override,
                            source: std::io::Error::other(error),
                        });
                    }
                };
            #[cfg(unix)]
            if let Err(error) = enable_pty_utf8_input(master.as_ref()) {
                log::warn!("failed to enable UTF-8 input mode on terminal PTY: {error}");
            }
            let child = match slave.spawn_command(command) {
                Ok(child) => child,
                Err(error) => {
                    bail!(TerminalError {
                        directory: working_directory.clone(),
                        program: shell_params.as_ref().map(|params| params.program.clone()),
                        args: shell_params.as_ref().and_then(|params| params.args.clone()),
                        title_override: terminal_title_override,
                        source: std::io::Error::other(error),
                    });
                }
            };
            drop(slave);

            let pty_info =
                PtyProcessInfo::new(portable_process_id_getter(master.as_ref(), child.as_ref()));
            let backend = GhosttyBackendWorker::new(
                TerminalBounds::default(),
                Some(scrolling_history),
                Some(events_tx.clone()),
            )?;
            backend.set_default_cursor_shape(cursor_shape.into());
            backend.set_osc52(GhosttyOsc52::default());
            if let Some(is_dark) = initial_dark_color_scheme {
                backend.set_dark_color_scheme(is_dark);
            }
            if let AlternateScroll::Off = alternate_scroll {
                backend.set_alternate_scroll(false)?;
            }

            let event_loop =
                GhosttyPtyEventLoop::new(events_tx, backend.clone(), master, child, true)
                    .context("failed to create terminal backend PTY event loop")?;
            let pty_tx = event_loop.channel();
            let _io_thread = event_loop.spawn();

            let no_task = task.is_none();
            let terminal = Terminal {
                task,
                terminal_type: TerminalType::Pty {
                    pty_tx: PtySender::Ghostty(GhosttyPtyNotifier::new(pty_tx)),
                    info: Arc::new(pty_info),
                },
                completion_tx,
                backend,
                selection: None,
                vi_cursor: None,
                cursor_blinking: false,
                output_since_refresh: false,
                content_revision: 0,
                title_override: terminal_title_override,
                events: VecDeque::with_capacity(10), //Should never get this high.
                last_content: Default::default(),
                last_full_content_range: None,
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
                keyboard_input_sent: false,
                event_loop_task: Task::ready(Ok(())),
                background_executor,
                path_style,
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
                // We cannot issue a `terminal.clear()` command at this point because shell output
                // is handled asynchronously, and while we have sent the activation script to the
                // pty, it will be executed asynchronously.
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
        cx.background_spawn(fut)
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

enum TerminalType {
    Pty {
        pty_tx: PtySender,
        info: Arc<PtyProcessInfo>,
    },
    DisplayOnly,
}

enum PtySender {
    Ghostty(GhosttyPtyNotifier),
}

impl PtySender {
    fn notify(&self, input: impl Into<Cow<'static, [u8]>>) {
        match self {
            Self::Ghostty(notifier) => notifier.notify(input),
        }
    }

    fn resize(&self, bounds: TerminalBounds) {
        match self {
            Self::Ghostty(notifier) => notifier.resize(bounds),
        }
    }

    fn shutdown(&self) {
        match self {
            Self::Ghostty(notifier) => notifier.shutdown(),
        }
    }
}

pub struct Terminal {
    terminal_type: TerminalType,
    completion_tx: Option<Sender<Option<ExitStatus>>>,
    backend: GhosttyBackendWorker,
    selection: Option<Selection>,
    vi_cursor: Option<Point>,
    cursor_blinking: bool,
    output_since_refresh: bool,
    content_revision: u64,
    events: VecDeque<InternalEvent>,
    /// This is only used for mouse mode cell change detection
    last_mouse: Option<(Point, SelectionSide)>,
    pub matches: Vec<Range>,
    pub last_content: Content,
    last_full_content_range: Option<Range>,
    pub selection_head: Option<Point>,

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
    last_hyperlink_search_position: Option<GpuiPoint<Pixels>>,
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
    #[cfg(any(test, feature = "test-support"))]
    input_log: Vec<Vec<u8>>,
}

struct CopyTemplate {
    shell: Shell,
    env: HashMap<String, String>,
    cursor_shape: SettingsCursorShape,
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
            PtyEvent::OutputProcessed(events) => {
                self.sync_backend_color_scheme(cx);
                self.process_backend_output_events(events, cx);
            }
        }
    }

    fn write_backend_output(&mut self, bytes: &[u8], cx: &mut Context<Self>) {
        self.sync_backend_color_scheme(cx);
        match self.backend.write_output(bytes) {
            Ok(events) => self.process_backend_output_events(events, cx),
            Err(error) => log::error!("failed to write terminal backend output: {error}"),
        }
    }

    fn process_backend_output_events(
        &mut self,
        events: Vec<TerminalBackendEvent>,
        cx: &mut Context<Self>,
    ) {
        self.bump_content_revision();
        self.output_since_refresh = true;
        for event in events {
            self.process_event(event, cx);
        }
        self.process_event(TerminalBackendEvent::Wakeup, cx);
    }

    fn sync_backend_color_scheme(&self, cx: &Context<Self>) {
        if cx.has_global::<GlobalTheme>() {
            self.backend
                .set_dark_color_scheme(cx.theme().appearance == Appearance::Dark);
        }
    }

    fn bump_content_revision(&mut self) {
        self.content_revision = self.content_revision.wrapping_add(1);
    }

    fn resize_backend(&mut self, bounds: TerminalBounds, cx: &mut Context<Self>) -> Result<()> {
        for event in self.backend.resize(bounds)? {
            self.process_event(event, cx);
        }
        self.bump_content_revision();
        Ok(())
    }

    fn clear_backend(&mut self, cx: &mut Context<Self>) -> Result<()> {
        for event in self.backend.clear()? {
            self.process_event(event, cx);
        }
        self.bump_content_revision();
        Ok(())
    }

    fn process_event(&mut self, event: TerminalBackendEvent, cx: &mut Context<Self>) {
        match event {
            TerminalBackendEvent::Title(title) => {
                // ignore default shell program title change as windows always sends those events
                // and it would end up showing the shell executable path in breadcrumbs
                #[cfg(windows)]
                if self
                    .shell_program
                    .as_ref()
                    .map(|e| *e == title)
                    .unwrap_or(false)
                {
                    return;
                }

                self.breadcrumb_text = title;
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
            TerminalBackendEvent::Bell => {
                cx.emit(Event::Bell);
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
                let color = terminal_rgb_from_color(get_color_at_index(index, cx.theme().as_ref()));
                self.write_to_pty(format(color).into_bytes());
            }
            TerminalBackendEvent::ChildExit(raw_status) => {
                self.register_task_finished(Some(raw_status), cx);
            }
        }
    }

    pub fn selection_started(&self) -> bool {
        self.selection_phase == SelectionPhase::Selecting
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

    fn find_hyperlink_at_point(&mut self, point: Point) -> Option<HyperlinkMatch> {
        terminal_hyperlinks::find_from_content_point(
            &self.last_content,
            point,
            &mut self.hyperlink_regex_searches,
            self.path_style,
        )
    }

    fn update_selected_word(
        &mut self,
        prev_word: Option<HoveredWord>,
        word_match: Range,
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

    pub fn last_content(&self) -> &Content {
        &self.last_content
    }

    pub fn set_cursor_shape(&mut self, cursor_shape: SettingsCursorShape) {
        self.backend.set_default_cursor_shape(cursor_shape.into());
    }

    pub fn write_output(&mut self, bytes: &[u8], cx: &mut Context<Self>) {
        // Inject bytes directly into the terminal emulator and refresh the UI.
        // This bypasses the PTY/event loop for display-only terminals.
        let mut converted = Vec::with_capacity(bytes.len());
        let mut prev_byte = 0u8;
        for &byte in bytes {
            if byte == b'\n' && prev_byte != b'\r' {
                converted.push(b'\r');
            }
            converted.push(byte);
            prev_byte = byte;
        }

        self.write_backend_output(&converted, cx);
        self.refresh_content(false, cx);
    }

    pub fn total_lines(&self) -> usize {
        self.backend.total_lines().unwrap_or_else(|error| {
            log::error!("failed to read terminal backend total lines: {error}");
            self.last_content.cells.len()
        })
    }

    pub fn viewport_lines(&self) -> usize {
        self.backend.viewport_lines().unwrap_or_else(|error| {
            log::error!("failed to read terminal backend viewport lines: {error}");
            self.last_content.terminal_bounds.num_lines()
        })
    }

    //To test:
    //- Activate match on terminal (scrolling and selection)
    //- Editor search snapping behavior

    pub fn activate_match(&mut self, index: usize) {
        if let Some(search_match) = self.matches.get(index).cloned() {
            self.set_selection(Some(Selection::simple_range(search_match)));
            if self.vi_mode_enabled {
                self.events
                    .push_back(InternalEvent::MoveViCursorToPoint(search_match.end()));
            } else {
                self.events
                    .push_back(InternalEvent::ScrollToPoint(search_match.start()));
            }
        }
    }

    pub fn select_matches(&mut self, matches: &[Range]) {
        let matches_to_select = self
            .matches
            .iter()
            .filter(|self_match| matches.contains(self_match))
            .cloned()
            .collect::<Vec<_>>();
        for match_to_select in matches_to_select {
            self.set_selection(Some(Selection::simple_range(match_to_select)));
        }
    }

    pub fn select_all(&mut self) {
        match self.backend.full_content_range() {
            Ok(Some(range)) => self.set_selection(Some(Selection::simple_range(range))),
            Ok(None) => {}
            Err(error) => log::error!("failed to read terminal backend content range: {error}"),
        }
    }

    fn set_selection(&mut self, selection: Option<Selection>) {
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
            .push_back(InternalEvent::Scroll(Scroll::Delta(1)));
    }

    pub fn scroll_up_by(&mut self, lines: usize) {
        self.events
            .push_back(InternalEvent::Scroll(Scroll::Delta(lines as i32)));
    }

    pub fn scroll_line_down(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(Scroll::Delta(-1)));
    }

    pub fn scroll_down_by(&mut self, lines: usize) {
        self.events
            .push_back(InternalEvent::Scroll(Scroll::Delta(-(lines as i32))));
    }

    pub fn scroll_page_up(&mut self) {
        self.events.push_back(InternalEvent::Scroll(Scroll::PageUp));
    }

    pub fn scroll_page_down(&mut self) {
        self.events
            .push_back(InternalEvent::Scroll(Scroll::PageDown));
    }

    pub fn scroll_to_top(&mut self) {
        self.events.push_back(InternalEvent::Scroll(Scroll::Top));
    }

    pub fn scroll_to_bottom(&mut self) {
        self.events.push_back(InternalEvent::Scroll(Scroll::Bottom));
    }

    pub fn scrolled_to_top(&self) -> bool {
        self.last_content.scrolled_to_top
    }

    pub fn scrolled_to_bottom(&self) -> bool {
        self.last_content.scrolled_to_bottom
    }

    ///Resize the terminal and the PTY.
    pub fn set_size(&mut self, new_bounds: TerminalBounds) {
        let new_bounds = normalize_terminal_bounds(new_bounds);

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
        self.events.push_back(InternalEvent::Scroll(Scroll::Bottom));
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
            let cursor_pos = GpuiPoint {
                x: cursor.column as f32 * self.last_content.terminal_bounds.cell_width,
                y: cursor.line as f32 * self.last_content.terminal_bounds.line_height,
            };
            self.events
                .push_back(InternalEvent::UpdateSelection(cursor_pos));
            self.events.push_back(InternalEvent::ViMotion(motion));
            return;
        }

        let scroll_motion = match key.as_ref() {
            "g" => Some(Scroll::Top),
            "G" => Some(Scroll::Bottom),
            "b" if keystroke.modifiers.control => Some(Scroll::PageUp),
            "f" if keystroke.modifiers.control => Some(Scroll::PageDown),
            "d" if keystroke.modifiers.control => {
                let amount = self.last_content.terminal_bounds.line_height().to_f64() as i32 / 2;
                Some(Scroll::Delta(-amount))
            }
            "u" if keystroke.modifiers.control => {
                let amount = self.last_content.terminal_bounds.line_height().to_f64() as i32 / 2;
                Some(Scroll::Delta(amount))
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
                let side = SelectionSide::Right;
                let selection = Selection::new(selection_type, point, side);
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

        match self.backend.encode_key(keystroke, option_as_meta) {
            Ok(Some(bytes)) => {
                self.input(bytes);
                true
            }
            Ok(None) => false,
            Err(error) => {
                log::error!("failed to encode terminal backend key input: {error}");
                false
            }
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
        let paste_text = if self.last_content.mode.contains(Modes::BRACKETED_PASTE) {
            format!("{}{}{}", "\x1b[200~", text.replace('\x1b', ""), "\x1b[201~")
        } else {
            text.replace("\r\n", "\r").replace('\n', "\r")
        };

        self.input(paste_text.into_bytes());
    }

    pub fn sync(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut selection_changed = false;
        let mut refresh_hovered_word_after_content_refresh = false;
        while let Some(event) = self.events.pop_front() {
            match event {
                InternalEvent::Resize(new_bounds) => {
                    let new_bounds = normalize_terminal_bounds(new_bounds);
                    self.last_content.terminal_bounds = new_bounds;
                    if let TerminalType::Pty { pty_tx, .. } = &self.terminal_type {
                        pty_tx.resize(new_bounds);
                    }
                    if let Err(error) = self.resize_backend(new_bounds, cx) {
                        log::error!("failed to resize terminal backend: {error}");
                    }
                    if !self.matches.is_empty() {
                        cx.emit(Event::Wakeup);
                    }
                }
                InternalEvent::Clear => {
                    if let Err(error) = self.clear_backend(cx) {
                        log::error!("failed to clear terminal backend: {error}");
                    }
                    cx.emit(Event::Wakeup);
                }
                InternalEvent::Scroll(scroll) => {
                    match scroll {
                        Scroll::Delta(delta) => {
                            for _ in 0..delta.unsigned_abs() {
                                if delta.is_positive() {
                                    self.backend.scroll_line_up();
                                } else {
                                    self.backend.scroll_line_down();
                                }
                            }
                        }
                        Scroll::PageUp => {
                            for _ in 0..self.last_content.terminal_bounds.num_lines() {
                                self.backend.scroll_line_up();
                            }
                        }
                        Scroll::PageDown => {
                            for _ in 0..self.last_content.terminal_bounds.num_lines() {
                                self.backend.scroll_line_down();
                            }
                        }
                        Scroll::Top => self.backend.scroll_to_top(),
                        Scroll::Bottom => self.backend.scroll_to_bottom(),
                    }
                    if self.update_vi_cursor_after_scroll(scroll) {
                        selection_changed = true;
                        cx.emit(Event::SelectionsChanged);
                    }
                    refresh_hovered_word_after_content_refresh = true;
                }
                InternalEvent::Copy(keep_selection) => {
                    let selection_text = self
                        .selection
                        .as_ref()
                        .and_then(|selection| {
                            self.selection_text_for_content(&self.last_content, selection)
                        })
                        .or_else(|| self.last_content.selection_text.clone());

                    if let Some(selection_text) = selection_text {
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
                    self.selection = None;
                    self.last_content.selection = None;
                    self.last_content.selection_text = None;
                    self.selection_head = None;
                    selection_changed = true;
                    cx.emit(Event::SelectionsChanged)
                }
                InternalEvent::SetSelection(Some(selection)) => {
                    self.selection_head = Some(selection.head);
                    self.selection = Some(selection);
                    selection_changed = true;
                    cx.emit(Event::SelectionsChanged)
                }
                InternalEvent::UpdateSelection(position) => {
                    if let Some(selection) = self.selection.as_mut() {
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
                    self.process_hyperlink_at_position(position, open, cx);
                }
                InternalEvent::ProcessHyperlink(hyperlink, open) => {
                    self.process_hyperlink(hyperlink, open, cx);
                }
                InternalEvent::ScrollToPoint(point) => {
                    self.backend.scroll_to_point(
                        point,
                        self.last_content.display_offset,
                        self.last_content.terminal_bounds.num_lines(),
                    );
                    refresh_hovered_word_after_content_refresh = true;
                }
                InternalEvent::MoveViCursorToPoint(point) => {
                    self.backend.scroll_to_point(
                        point,
                        self.last_content.display_offset,
                        self.last_content.terminal_bounds.num_lines(),
                    );
                    self.set_vi_cursor(point, &mut selection_changed, cx);
                    refresh_hovered_word_after_content_refresh = true;
                }
                InternalEvent::ToggleViMode => {
                    self.vi_mode_enabled = !self.vi_mode_enabled;
                    self.vi_cursor = self
                        .vi_mode_enabled
                        .then_some(self.last_content.cursor.point);
                    self.cursor_blinking = false;
                    cx.emit(Event::BlinkChanged(false));
                }
                InternalEvent::ViMotion(motion) => {
                    if self.vi_mode_enabled {
                        let cursor = self.vi_cursor.unwrap_or(self.last_content.cursor.point);
                        let cursor = vi_motion(&self.last_content, cursor, motion);
                        self.backend.scroll_to_point(
                            cursor,
                            self.last_content.display_offset,
                            self.last_content.terminal_bounds.num_lines(),
                        );
                        self.set_vi_cursor(cursor, &mut selection_changed, cx);
                    }
                }
            }
        }
        for event in self.backend.drain_events() {
            self.process_event(event, cx);
        }

        self.refresh_content(selection_changed, cx);
        if refresh_hovered_word_after_content_refresh {
            self.refresh_hovered_word_from_current_content(window, cx);
        }
    }

    fn refresh_content(&mut self, selection_changed: bool, cx: &mut Context<Self>) {
        let output_since_refresh = std::mem::take(&mut self.output_since_refresh);
        let previous_selection_text = self.last_content.selection_text.clone();

        match self.backend.content(&self.last_content) {
            Ok((mut content, backend_cursor_blinking, full_content_range)) => {
                let cursor_blinking = !self.vi_mode_enabled && backend_cursor_blinking;
                if cursor_blinking != self.cursor_blinking {
                    self.cursor_blinking = cursor_blinking;
                    cx.emit(Event::BlinkChanged(cursor_blinking));
                }
                let selection_remapped_after_output = output_since_refresh
                    && !selection_changed
                    && self.remap_selection_after_output(&content, full_content_range);
                if selection_remapped_after_output {
                    cx.emit(Event::SelectionsChanged);
                }
                self.apply_vi_mode(&mut content);
                self.apply_selection(
                    &mut content,
                    selection_remapped_after_output
                        .then_some(previous_selection_text)
                        .flatten(),
                );
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                if selection_changed && let Some(selection_text) = content.selection_text.clone() {
                    cx.write_to_primary(ClipboardItem::new_string(selection_text));
                }
                self.last_content = content;
                self.last_full_content_range = full_content_range;
            }
            Err(error) => log::error!("failed to build terminal backend content: {error}"),
        }
    }

    fn remap_selection_after_output(
        &mut self,
        content: &Content,
        full_content_range: Option<Range>,
    ) -> bool {
        if self.selection.is_none() {
            return false;
        }

        let Some(delta) = self.selection_line_delta_after_output(content, full_content_range)
        else {
            self.selection = None;
            self.selection_head = None;
            return true;
        };

        if delta == 0 {
            return false;
        }

        if let Some(selection) = self.selection.as_mut() {
            selection.translate_lines(delta);
            if let Some(full_content_range) = full_content_range
                && !selection_intersects_range(selection, full_content_range)
            {
                self.selection = None;
                self.selection_head = None;
                return true;
            }
        }
        if let Some(selection_head) = self.selection_head.as_mut() {
            selection_head.line = selection_head.line.saturating_add(delta);
        }
        true
    }

    fn selection_line_delta_after_output(
        &self,
        content: &Content,
        full_content_range: Option<Range>,
    ) -> Option<i32> {
        let previous_rows = terminal_content_row_texts(&self.last_content);
        if previous_rows.is_empty() {
            return Some(0);
        }

        if let Some(delta) =
            best_terminal_row_delta(&previous_rows, &terminal_content_row_texts(content))
        {
            return Some(delta);
        }

        let full_content_range = full_content_range?;
        let previous_full_content_range = self.last_full_content_range?;
        let range_delta = full_content_range
            .start()
            .line
            .saturating_sub(previous_full_content_range.start().line);
        (range_delta != 0).then_some(range_delta)
    }

    fn update_vi_cursor_after_scroll(&mut self, scroll: Scroll) -> bool {
        if !self.vi_mode_enabled {
            return false;
        }

        let cursor = self.vi_cursor.unwrap_or(self.last_content.cursor.point);
        let full_content_range = match self.backend.full_content_range() {
            Ok(range) => range,
            Err(error) => {
                log::error!("failed to read terminal backend content range: {error}");
                None
            }
        };
        let (mut cursor, use_first_occupied_column) = vi_cursor_after_scroll(
            cursor,
            scroll,
            self.last_content.terminal_bounds.num_lines(),
            full_content_range,
        );
        if use_first_occupied_column {
            cursor.column = match self.backend.first_occupied_column(cursor.line) {
                Ok(Some(column)) => column,
                Ok(None) => 0,
                Err(error) => {
                    log::error!("failed to read terminal backend row: {error}");
                    0
                }
            };
        }

        self.vi_cursor = Some(cursor);
        self.update_vi_selection(cursor)
    }

    fn set_vi_cursor(
        &mut self,
        point: Point,
        selection_changed: &mut bool,
        cx: &mut Context<Self>,
    ) {
        self.vi_cursor = Some(point);
        if self.update_vi_selection(point) {
            *selection_changed = true;
            cx.emit(Event::SelectionsChanged);
        }
    }

    fn update_vi_selection(&mut self, point: Point) -> bool {
        if let Some(selection) = self.selection.as_mut() {
            selection.update_vi(point);
            self.selection_head = Some(point);
            true
        } else {
            false
        }
    }

    fn apply_vi_mode(&mut self, content: &mut Content) {
        if self.vi_mode_enabled {
            content.mode.insert(Modes::VI);
            let cursor = self
                .vi_cursor
                .map(|cursor| clamp_content_point(content, cursor))
                .unwrap_or(content.cursor.point);
            self.vi_cursor = Some(cursor);
            content.cursor.point = cursor;
            content.cursor_char = selection_cell(content, cursor)
                .map(|cell| cell.character())
                .unwrap_or(' ');
        } else {
            content.mode.remove(Modes::VI);
            self.vi_cursor = None;
        }
    }

    fn apply_selection(&self, content: &mut Content, cached_selection_text: Option<String>) {
        let Some(selection) = &self.selection else {
            content.selection = None;
            content.selection_text = None;
            return;
        };

        let visible_range = selection.to_range(content);
        content.selection = visible_range;
        content.selection_text =
            cached_selection_text.or_else(|| self.selection_text_for_content(content, selection));
    }

    fn selection_text_for_content(
        &self,
        content: &Content,
        selection: &Selection,
    ) -> Option<String> {
        if selection.is_fully_within(content) {
            selection
                .to_range(content)
                .map(|range| selection.selected_text(content, &range))
        } else {
            match self.backend.full_content(content) {
                Ok(full_content) => selection
                    .to_range(&full_content)
                    .map(|range| selection.selected_text(&full_content, &range)),
                Err(error) => {
                    log::error!("failed to build terminal backend full content: {error}");
                    selection
                        .to_range(content)
                        .map(|range| selection.selected_text(content, &range))
                }
            }
        }
    }

    pub fn with_renderable_cells<R>(&self, f: impl for<'a> FnOnce(RenderableCells<'a>) -> R) -> R {
        f(RenderableCells::new(&self.last_content.cells))
    }

    pub fn get_content(&self) -> String {
        match self.backend.formatted_content() {
            Ok(content) => content,
            Err(error) => {
                log::error!("failed to format terminal backend content: {error}");
                Self::content_to_text(&self.last_content)
            }
        }
    }

    pub fn last_n_non_empty_lines(&self, n: usize) -> Vec<String> {
        match self.backend.formatted_content() {
            Ok(content) => Self::last_n_non_empty_lines_from_text(&content, n),
            Err(error) => {
                log::error!("failed to format terminal backend content: {error}");
                Self::last_n_non_empty_lines_from_text(
                    &Self::content_to_text(&self.last_content),
                    n,
                )
            }
        }
    }

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

    fn content_to_text(content: &Content) -> String {
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

            text.push(indexed_cell.cell.character());
            if let Some(chars) = indexed_cell.cell.zerowidth() {
                for character in chars {
                    text.push(*character);
                }
            }
        }
        text
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
        match self.backend.encode_focus(true) {
            Ok(Some(bytes)) => self.write_to_pty(bytes),
            Ok(None) => {}
            Err(error) => log::error!("failed to encode terminal backend focus-in input: {error}"),
        }
    }

    pub fn focus_out(&mut self) {
        match self.backend.encode_focus(false) {
            Ok(Some(bytes)) => self.write_to_pty(bytes),
            Ok(None) => {}
            Err(error) => log::error!("failed to encode terminal backend focus-out input: {error}"),
        }
    }

    fn mouse_changed(&mut self, point: Point, side: SelectionSide) -> bool {
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
        self.last_content.mode.intersects(Modes::MOUSE_MODE) && !shift
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
                match self.backend.encode_mouse_motion(
                    point,
                    self.last_content.terminal_bounds,
                    e.pressed_button,
                    e.modifiers,
                ) {
                    Ok(Some(bytes)) => self.write_to_pty(bytes),
                    Ok(None) => {}
                    Err(error) => {
                        log::error!("failed to encode terminal backend mouse-move input: {error}")
                    }
                }
            }
        } else {
            self.schedule_find_hyperlink(e.modifiers, e.position);
        }
        cx.notify();
    }

    fn schedule_find_hyperlink(&mut self, modifiers: Modifiers, position: GpuiPoint<Pixels>) {
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
            // Preserve the existing drag ordering: update the selection first,
            // then scroll 15ms later.
            self.events
                .push_back(InternalEvent::UpdateSelection(position));

            // Doesn't make sense to scroll the alt screen
            if !self.last_content.mode.contains(Modes::ALT_SCREEN) {
                let scroll_lines = match self.drag_line_delta(e, region) {
                    Some(value) => value,
                    None => return,
                };

                self.events
                    .push_back(InternalEvent::Scroll(Scroll::Delta(scroll_lines)));
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
            match self.backend.encode_mouse_button(
                point,
                self.last_content.terminal_bounds,
                e.button,
                e.modifiers,
                true,
            ) {
                Ok(Some(bytes)) => self.write_to_pty(bytes),
                Ok(None) => {}
                Err(error) => {
                    log::error!("failed to encode terminal backend mouse-down input: {error}")
                }
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

            match self.backend.encode_mouse_button(
                point,
                self.last_content.terminal_bounds,
                e.button,
                e.modifiers,
                false,
            ) {
                Ok(Some(bytes)) => self.write_to_pty(bytes),
                Ok(None) => {}
                Err(error) => {
                    log::error!("failed to encode terminal backend mouse-up input: {error}")
                }
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

                match self.backend.encode_mouse_scroll(
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
                        log::error!("failed to encode terminal backend mouse-scroll input: {error}")
                    }
                }
            } else if self
                .last_content
                .mode
                .contains(Modes::ALT_SCREEN | Modes::ALTERNATE_SCROLL)
                && !e.shift
            {
                self.write_to_pty(alt_scroll(scroll_lines));
            } else {
                self.events
                    .push_back(InternalEvent::Scroll(Scroll::Delta(scroll_lines)));
            }
        }
    }

    fn refresh_hovered_word(&mut self, window: &Window) {
        self.schedule_find_hyperlink(window.modifiers(), window.mouse_position());
    }

    fn refresh_hovered_word_from_current_content(
        &mut self,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        let position = window.mouse_position();
        if self.selection_phase == SelectionPhase::Selecting
            || !window.modifiers().secondary()
            || !self.last_content.terminal_bounds.bounds.contains(&position)
        {
            self.last_content.last_hovered_word = None;
            return;
        }

        self.last_mouse_move_time = Instant::now();
        self.last_hyperlink_search_position = Some(position);
        self.process_hyperlink_at_position(
            position - self.last_content.terminal_bounds.bounds.origin,
            false,
            cx,
        );
    }

    fn process_hyperlink_at_position(
        &mut self,
        position: GpuiPoint<Pixels>,
        open: bool,
        cx: &mut Context<Self>,
    ) {
        let point = grid_point(
            position,
            self.last_content.terminal_bounds,
            self.last_content.display_offset,
        );
        match self.find_hyperlink_at_point(point) {
            Some(hyperlink) => {
                self.process_hyperlink(hyperlink, open, cx);
            }
            None => {
                self.last_content.last_hovered_word = None;
                cx.emit(Event::NewNavigationTarget(None));
            }
        }
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

    pub fn find_matches(&self, searcher: Search, cx: &Context<Self>) -> Task<Vec<Range>> {
        let Some(regex) = searcher.regex() else {
            return Task::ready(Vec::new());
        };

        cx.spawn(async move |terminal, cx| {
            let mut builder = match terminal
                .update(cx, |terminal, _| SearchSnapshotBuilder::new(terminal))
            {
                Ok(Ok(builder)) => builder,
                Ok(Err(error)) => {
                    log::error!("failed to start terminal backend full content search: {error}");
                    return Vec::new();
                }
                Err(error) => {
                    log::error!("failed to access terminal for search: {error}");
                    return Vec::new();
                }
            };

            loop {
                let done = match terminal.update(cx, |terminal, _| {
                    builder.append_rows(terminal, SEARCH_SNAPSHOT_ROWS_PER_TICK)
                }) {
                    Ok(Ok(done)) => done,
                    Ok(Err(error)) => {
                        log::error!(
                            "failed to build terminal backend full content search: {error}"
                        );
                        return Vec::new();
                    }
                    Err(error) => {
                        log::error!("failed to access terminal for search: {error}");
                        return Vec::new();
                    }
                };

                if done {
                    break;
                }
                yield_now().await;
            }

            let Some(content) = builder.finish() else {
                log::error!("failed to finish terminal backend full content search");
                return Vec::new();
            };
            cx.background_spawn(async move { content_search_matches(content, regex) })
                .await
        })
    }

    pub fn working_directory(&self) -> Option<PathBuf> {
        if self.is_remote_terminal {
            // We can't yet reliably detect the working directory of a shell on the
            // SSH host. Until we can do that, it doesn't make sense to display
            // the working directory on the client and persist that.
            None
        } else {
            match self.backend.working_directory(self.path_style) {
                Ok(Some(working_directory)) => return Some(working_directory),
                Ok(None) => {}
                Err(error) => {
                    log::error!("failed to read terminal backend working directory: {error}")
                }
            }

            self.client_side_working_directory()
        }
    }

    /// Normalizes the command name of the foreground process, if one is known.
    pub fn foreground_process_command_name(&self) -> Option<String> {
        match &self.terminal_type {
            TerminalType::Pty { info, .. } => info
                .current
                .read()
                .as_ref()
                .and_then(|process| foreground_process_command_from_argv(&process.argv)),
            TerminalType::DisplayOnly => None,
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
            let mut summary = String::new();
            for line in lines_to_show {
                summary.push('\n');
                summary.push_str(line);
            }
            self.write_output(summary.as_bytes(), cx);
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

const TASK_DELIMITER: &str = "⏵ ";
fn task_summary(task: &TaskState, exit_status: Option<ExitStatus>) -> (bool, String, String) {
    let escaped_full_label = escape_task_summary_text(&task.spawned_task.full_label);
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
    let escaped_command_label = escape_task_summary_text(&task.spawned_task.command_label);
    let command_line = format!("{TASK_DELIMITER}Command: {escaped_command_label}");
    (success, task_line, command_line)
}

fn escape_task_summary_text(text: &str) -> String {
    let mut escaped_text = String::with_capacity(text.len());
    for character in text.chars() {
        if character.is_control() {
            escaped_text.extend(character.escape_default());
        } else {
            escaped_text.push(character);
        }
    }
    escaped_text
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

fn normalize_path_command_name(command: &str) -> Option<String> {
    const MAX_COMMAND_NAME_LENGTH: usize = 64;

    let command = command.trim();
    if command.is_empty()
        || command.len() > MAX_COMMAND_NAME_LENGTH
        || command.starts_with('.')
        || command.starts_with('-')
        || command.contains('/')
        || command.contains('\\')
    {
        return None;
    }

    let mut command = command.to_ascii_lowercase();
    for suffix in [".exe", ".cmd", ".bat", ".ps1"] {
        if command.ends_with(suffix) {
            command.truncate(command.len() - suffix.len());
            break;
        }
    }

    if command.is_empty()
        || !command.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
    {
        return None;
    }

    Some(command)
}

fn foreground_process_command_from_argv(argv: &[String]) -> Option<String> {
    let command = argv
        .first()
        .and_then(|command| normalize_path_command_name(command));

    if !matches!(
        command.as_deref(),
        Some("node" | "python" | "python3" | "bun" | "deno")
    ) {
        return command;
    }

    argv.iter()
        .skip(1)
        .filter_map(|argument| normalize_script_command_name(argument))
        .next()
        .or(command)
}

fn normalize_script_command_name(argument: &str) -> Option<String> {
    let path = Path::new(argument);
    let file_stem = path
        .file_stem()
        .and_then(|file_stem| file_stem.to_str())
        .and_then(normalize_path_command_name)?;

    if file_stem != "index" {
        return Some(file_stem);
    }

    path.parent()
        .and_then(|parent| parent.parent())
        .and_then(|package_path| package_path.file_name())
        .and_then(|package_name| package_name.to_str())
        .and_then(|package_name| package_name.strip_suffix("-cli").or(Some(package_name)))
        .and_then(normalize_path_command_name)
}

fn content_index_for_mouse(pos: GpuiPoint<Pixels>, terminal_bounds: &TerminalBounds) -> usize {
    let col = (pos.x / terminal_bounds.cell_width()).round() as usize;
    let clamped_col = min(col, terminal_bounds.num_columns().saturating_sub(1));
    let row = (pos.y / terminal_bounds.line_height()).round() as usize;
    let clamped_row = min(row, terminal_bounds.num_lines().saturating_sub(1));
    clamped_row * terminal_bounds.num_columns() + clamped_col
}

/// Converts an 8 bit ANSI color to its GPUI equivalent.
/// Indexes above 255 are internal extension values for default and dim colors.
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
        // Internal indexes for default and dim terminal colors.
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

fn terminal_rgb_from_color(color: impl Into<Rgba>) -> Rgb {
    let color = color.into();
    Rgb {
        r: ((color.r * color.a) * 255.) as u8,
        g: ((color.g * color.a) * 255.) as u8,
        b: ((color.b * color.a) * 255.) as u8,
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
        Cell, Content, IndexedCell, TerminalBounds, TerminalBuilder, content_index_for_mouse,
        rgb_for_index,
    };
    use async_channel::Receiver;
    use collections::HashMap;
    use gpui::MouseMoveEvent;
    use gpui::{
        ClipboardItem, Entity, Modifiers, MouseButton, MouseDownEvent, MouseUpEvent, Pixels,
        Point as GpuiPoint, TestAppContext, VisualContext, bounds, point, size,
    };
    use parking_lot::Mutex;
    use rand::{Rng, distr, rngs::StdRng};
    use task::{Shell, ShellBuilder};

    #[test]
    fn test_normalize_path_command_name() {
        assert_eq!(normalize_path_command_name("claude"), Some("claude".into()));
        assert_eq!(normalize_path_command_name("Cargo"), Some("cargo".into()));
        assert_eq!(normalize_path_command_name("node.exe"), Some("node".into()));
        assert_eq!(
            normalize_path_command_name("my-agent_cli.1"),
            Some("my-agent_cli.1".into())
        );
        assert_eq!(normalize_path_command_name("./local-agent"), None);
        assert_eq!(normalize_path_command_name("../local-agent"), None);
        assert_eq!(normalize_path_command_name("/usr/local/bin/cargo"), None);
        assert_eq!(
            normalize_path_command_name("target\\debug\\agent.exe"),
            None
        );
        assert_eq!(normalize_path_command_name(".hidden-agent"), None);
        assert_eq!(normalize_path_command_name("agent with spaces"), None);
        assert_eq!(normalize_path_command_name("zsh"), Some("zsh".into()));
        assert_eq!(normalize_path_command_name("-zsh"), None);
        assert_eq!(normalize_path_command_name("pwsh.exe"), Some("pwsh".into()));
    }

    #[test]
    fn test_foreground_process_command_from_interpreter_wrapper() {
        assert_eq!(
            foreground_process_command_from_argv(&[
                "node".to_string(),
                "/opt/homebrew/lib/node_modules/@google/gemini-cli/dist/index.js".to_string(),
            ]),
            Some("gemini".to_string())
        );
        assert_eq!(
            foreground_process_command_from_argv(&[
                "python3".to_string(),
                "/Users/me/.local/bin/codex.py".to_string(),
            ]),
            Some("codex".to_string())
        );
        assert_eq!(
            foreground_process_command_from_argv(&[
                "node".to_string(),
                "/Users/me/private-project/scripts/customer-data-export.js".to_string(),
            ]),
            Some("customer-data-export".to_string())
        );
    }

    #[test]
    fn test_apply_spawn_environment_applies_terminal_overrides() {
        let mut command = portable_pty::CommandBuilder::new("dummy");
        command.env("SHLVL", "42");
        command.env("XDG_ACTIVATION_TOKEN", "inherited-token");
        let mut env = HashMap::default();
        env.insert("TERM".to_string(), "xterm-256color".to_string());
        env.insert("SHLVL".to_string(), "43".to_string());
        env.insert(
            "XDG_ACTIVATION_TOKEN".to_string(),
            "explicit-token".to_string(),
        );
        env.insert(
            "DESKTOP_STARTUP_ID".to_string(),
            "explicit-startup-id".to_string(),
        );

        apply_spawn_environment(&mut command, &env, 123);

        assert!(command.get_env("SHLVL").is_none());
        assert!(command.get_env("XDG_ACTIVATION_TOKEN").is_none());
        assert!(command.get_env("DESKTOP_STARTUP_ID").is_none());
        #[cfg(unix)]
        assert_eq!(
            command.get_env("WINDOWID").and_then(|value| value.to_str()),
            Some("123")
        );
        assert_eq!(
            command.get_env("TERM").and_then(|value| value.to_str()),
            Some("xterm-256color")
        );
    }

    #[test]
    fn test_task_summary_escapes_control_characters() {
        let (_, completion_rx) = async_channel::unbounded();
        let task = TaskState {
            status: TaskStatus::Running,
            completion_rx,
            spawned_task: SpawnInTerminal {
                full_label: "build\x1b]2;owned\x07\nnext".to_string(),
                command_label: "cargo test\r\n\x1b[31mred".to_string(),
                ..Default::default()
            },
        };

        let (_, task_line, command_line) = task_summary(&task, None);

        assert!(!task_line.chars().any(char::is_control));
        assert!(!command_line.chars().any(char::is_control));
        assert!(task_line.contains("\\u{1b}"));
        assert!(task_line.contains("\\u{7}"));
        assert!(task_line.contains("\\n"));
        assert!(command_line.contains("\\r\\n"));
        assert!(command_line.contains("\\u{1b}"));
    }

    #[cfg(any(
        target_os = "android",
        target_os = "ios",
        target_os = "linux",
        target_os = "macos"
    ))]
    #[test]
    fn test_enable_pty_utf8_input_sets_iutf8() {
        let pty_system = portable_pty::native_pty_system();
        let portable_pty::PtyPair { master, .. } = pty_system
            .openpty(portable_pty_size(TerminalBounds::default()))
            .expect("failed to open PTY");

        enable_pty_utf8_input(master.as_ref()).expect("failed to enable UTF-8 input");

        let fd = master.as_raw_fd().expect("missing raw PTY file descriptor");
        let mut termios = std::mem::MaybeUninit::<libc::termios>::uninit();
        assert_eq!(unsafe { libc::tcgetattr(fd, termios.as_mut_ptr()) }, 0);
        let termios = unsafe { termios.assume_init() };
        assert_ne!(termios.c_iflag & libc::IUTF8, 0);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_default_system_command_uses_login_shell_on_macos() {
        assert!(default_system_command().is_default_prog());
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    #[test]
    fn test_default_system_command_is_non_login_on_unix() {
        assert!(!default_system_command().is_default_prog());
    }

    #[test]
    fn test_vi_cursor_after_scroll_preserves_terminal_scroll_semantics() {
        let full_content_range = Range::new(Point::new(-10, 0), Point::new(5, 79));
        let cursor = Point::new(0, 3);

        assert_eq!(
            vi_cursor_after_scroll(cursor, Scroll::Delta(2), 4, Some(full_content_range)),
            (Point::new(-2, 3), true)
        );
        assert_eq!(
            vi_cursor_after_scroll(cursor, Scroll::PageUp, 4, Some(full_content_range)),
            (Point::new(-4, 3), true)
        );
        assert_eq!(
            vi_cursor_after_scroll(cursor, Scroll::PageDown, 4, Some(full_content_range)),
            (Point::new(4, 3), true)
        );
        assert_eq!(
            vi_cursor_after_scroll(cursor, Scroll::Top, 4, Some(full_content_range)),
            (Point::new(-10, 0), false)
        );
        assert_eq!(
            vi_cursor_after_scroll(cursor, Scroll::Bottom, 4, Some(full_content_range)),
            (Point::new(5, 0), false)
        );
        assert_eq!(
            vi_cursor_after_scroll(
                Point::new(-9, 3),
                Scroll::PageUp,
                4,
                Some(full_content_range)
            ),
            (Point::new(-10, 0), true)
        );
    }

    #[gpui::test]
    async fn test_terminal_builder_rejects_invalid_working_directory(cx: &mut TestAppContext) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after UNIX epoch")
            .as_nanos();
        let missing_directory = std::env::temp_dir().join(format!(
            "zed-terminal-missing-cwd-{}-{timestamp}",
            std::process::id()
        ));

        let result = cx
            .update(|cx| {
                TerminalBuilder::new(
                    Some(missing_directory.clone()),
                    None,
                    task::Shell::WithArguments {
                        program: "definitely-not-run".to_string(),
                        args: Vec::new(),
                        title_override: None,
                    },
                    HashMap::default(),
                    SettingsCursorShape::default(),
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
            .await;

        let Err(error) = result else {
            panic!("expected invalid working directory to fail before spawn");
        };
        let terminal_error = error
            .downcast_ref::<TerminalError>()
            .expect("expected terminal error");
        assert_eq!(
            terminal_error.directory.as_deref(),
            Some(missing_directory.as_path())
        );
        assert_eq!(terminal_error.source.kind(), std::io::ErrorKind::NotFound);
    }

    #[cfg(not(target_os = "windows"))]
    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
    }

    fn display_only_bounds(columns: usize, rows: usize) -> TerminalBounds {
        TerminalBounds::new(
            px(10.0),
            px(10.0),
            bounds(
                point(px(0.0), px(0.0)),
                size(px(columns as f32 * 10.0), px(rows as f32 * 10.0)),
            ),
        )
    }

    fn display_only_terminal(
        cx: &mut TestAppContext,
        columns: usize,
        rows: usize,
        scrollback_lines: Option<usize>,
    ) -> Entity<Terminal> {
        let terminal_bounds = display_only_bounds(columns, rows);
        cx.new(|cx| {
            TerminalBuilder::new_display_only_with_bounds(
                SettingsCursorShape::default(),
                AlternateScroll::On,
                scrollback_lines,
                0,
                cx.background_executor(),
                PathStyle::local(),
                terminal_bounds,
            )
            .unwrap()
            .subscribe(cx)
        })
    }

    #[gpui::test]
    async fn test_vi_scroll_updates_cursor_and_selection(cx: &mut TestAppContext) {
        let terminal = display_only_terminal(cx, 8, 2, Some(100));

        terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"one\r\ntwo\r\nthree\r\nfour", cx);
            refresh_terminal_content(terminal, cx);

            let full_content_range = terminal
                .backend
                .full_content_range()
                .expect("full content range should load")
                .expect("terminal should have content");
            let bottom = full_content_range.end();
            let expected_top = Point::new(full_content_range.start().line, 0);

            terminal.vi_mode_enabled = true;
            terminal.vi_cursor = Some(bottom);
            terminal.selection = Some(Selection::new(
                SelectionType::Simple,
                bottom,
                SelectionSide::Right,
            ));

            assert!(terminal.update_vi_cursor_after_scroll(Scroll::Top));
            assert_eq!(terminal.vi_cursor, Some(expected_top));
            assert_eq!(terminal.selection_head, Some(expected_top));
            assert_eq!(
                terminal.selection.as_ref().map(|selection| selection.head),
                Some(expected_top)
            );
        });
    }

    fn apply_pending_selection_events(terminal: &mut Terminal) {
        while let Some(event) = terminal.events.pop_front() {
            match event {
                InternalEvent::SetSelection(selection) => {
                    terminal.selection_head = selection.as_ref().map(|selection| selection.head);
                    terminal.selection = selection;
                }
                _ => panic!("unexpected terminal event"),
            }
        }
    }

    /// Helper to build a test terminal running a shell command.
    /// Returns the terminal entity and a receiver for the completion signal.
    async fn build_test_terminal(
        cx: &mut TestAppContext,
        command: &str,
        args: &[&str],
    ) -> (Entity<Terminal>, Receiver<Option<ExitStatus>>) {
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let (program, args) =
            ShellBuilder::new(&Shell::System, false).build(Some(command.to_owned()), &args);
        build_test_terminal_with_arguments(cx, program, args).await
    }

    async fn build_test_terminal_with_arguments(
        cx: &mut TestAppContext,
        program: String,
        args: Vec<String>,
    ) -> (Entity<Terminal>, Receiver<Option<ExitStatus>>) {
        let (completion_tx, completion_rx) = async_channel::unbounded();
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
                    SettingsCursorShape::default(),
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
                SettingsCursorShape::default(),
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .unwrap()
            .subscribe(cx)
        });

        terminal.update(cx, |terminal, _cx| {
            let terminal_bounds = TerminalBounds::new(
                px(20.0),
                px(10.0),
                bounds(point(px(0.0), px(0.0)), size(px(400.0), px(400.0))),
            );
            terminal.last_content.terminal_bounds = terminal_bounds;
        });
        terminal.update(cx, |terminal, cx| {
            terminal
                .resize_backend(terminal.last_content.terminal_bounds, cx)
                .expect("failed to resize test terminal");
        });

        terminal.update(cx, |terminal, cx| {
            terminal.write_output(output, cx);
            refresh_terminal_content(terminal, cx);
            terminal.events.clear();
        });

        terminal
    }

    fn refresh_terminal_content(terminal: &mut Terminal, cx: &mut Context<Terminal>) {
        for event in terminal.backend.drain_events() {
            terminal.process_event(event, cx);
        }
        let (mut content, _, full_content_range) = terminal
            .backend
            .content(&terminal.last_content)
            .expect("failed to build test terminal content");
        terminal.apply_vi_mode(&mut content);
        terminal.apply_selection(&mut content, None);
        terminal.last_content = content;
        terminal.last_full_content_range = full_content_range;
    }

    fn ctrl_mouse_down_at(
        terminal: &mut Terminal,
        position: GpuiPoint<Pixels>,
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
        position: GpuiPoint<Pixels>,
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
        position: GpuiPoint<Pixels>,
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

    #[cfg(unix)]
    #[gpui::test]
    async fn test_foreground_process_command_tracks_path_command(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let (terminal, completion_rx) =
            build_test_terminal_with_arguments(cx, "sleep".to_string(), vec!["1".to_string()])
                .await;

        assert_foreground_process_command_eventually(&terminal, "sleep", cx).await;

        assert!(
            completion_rx.recv().await.is_ok(),
            "expected terminal completion after sleep exits"
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
                    SettingsCursorShape::default(),
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
                    SettingsCursorShape::default(),
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
                    SettingsCursorShape::default(),
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
    fn test_terminal_modes_flags() {
        let mut terminal_modes = Modes::empty();
        terminal_modes.insert(Modes::APP_CURSOR);
        terminal_modes.insert(Modes::BRACKETED_PASTE);
        terminal_modes.insert(Modes::ALT_SCREEN);
        terminal_modes.insert(Modes::MOUSE_DRAG);
        terminal_modes.insert(Modes::SGR_MOUSE);
        terminal_modes.insert(Modes::VI);

        assert!(terminal_modes.contains(Modes::APP_CURSOR));
        assert!(terminal_modes.contains(Modes::BRACKETED_PASTE));
        assert!(terminal_modes.contains(Modes::ALT_SCREEN));
        assert!(terminal_modes.contains(Modes::MOUSE_DRAG));
        assert!(terminal_modes.intersects(Modes::MOUSE_MODE));
        assert!(terminal_modes.contains(Modes::SGR_MOUSE));
        assert!(terminal_modes.contains(Modes::VI));
        assert!(!terminal_modes.contains(Modes::MOUSE_REPORT_CLICK));

        terminal_modes.remove(Modes::MOUSE_DRAG);
        assert!(!terminal_modes.contains(Modes::MOUSE_DRAG));
    }

    #[test]
    fn test_terminal_selection_range_point_range() {
        let terminal_range = SelectionRange {
            start: Point {
                line: -2,
                column: 3,
            },
            end: Point { line: 4, column: 8 },
            is_block: true,
        };
        assert_eq!(
            terminal_range.point_range(),
            Range::new(
                Point {
                    line: -2,
                    column: 3
                },
                Point { line: 4, column: 8 },
            )
        );
        assert!(terminal_range.is_block);
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
                    GpuiPoint::default(),
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
                    let mouse_cell = content.cells[content_index].character();
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
                GpuiPoint::default(),
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
            .character(),
            cells[0][0]
        );
        assert_eq!(
            content.cells[content_index_for_mouse(
                point(Pixels::from(1000.), Pixels::from(1000.)),
                &content.terminal_bounds,
            )]
            .character(),
            cells[9][9]
        );
    }

    #[gpui::test]
    async fn test_set_size_coalesces_pixel_only_changes(cx: &mut TestAppContext) {
        let builder = cx.update(|cx| {
            TerminalBuilder::new_display_only(
                SettingsCursorShape::Block,
                AlternateScroll::On,
                None,
                0,
                cx.background_executor(),
                PathStyle::local(),
            )
            .unwrap()
        });
        let mut terminal = builder.terminal;

        let base_bounds = TerminalBounds {
            cell_width: Pixels::from(10.),
            line_height: Pixels::from(10.),
            bounds: bounds(
                GpuiPoint::default(),
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

    fn convert_cells_to_content(terminal_bounds: TerminalBounds, cells: &[Vec<char>]) -> Content {
        let mut ic = Vec::new();

        for (index, row) in cells.iter().enumerate() {
            for (cell_index, cell_char) in row.iter().enumerate() {
                let mut cell = Cell::default();
                cell.set_character(*cell_char);
                ic.push(IndexedCell {
                    point: Point::new(index as i32, cell_index),
                    cell,
                });
            }
        }

        Content {
            cells: ic,
            terminal_bounds,
            ..Default::default()
        }
    }

    #[gpui::test]
    async fn test_write_output_converts_lf_to_crlf(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            TerminalBuilder::new_display_only(
                SettingsCursorShape::default(),
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

        let content = terminal.update(cx, |terminal, cx| {
            refresh_terminal_content(terminal, cx);
            terminal.last_content.clone()
        });

        // If LF is properly converted to CRLF, each line should start at column 0
        // The diagonal staircase bug would cause increasing column positions

        // Get the cells and check that lines start at column 0
        let cells = &content.cells;
        let mut line1_col0 = false;
        let mut line2_col0 = false;

        for cell in cells {
            if cell.character() == 'l' && cell.point.column == 0 {
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

    #[gpui::test]
    async fn test_write_output_refreshes_renderable_cells(cx: &mut TestAppContext) {
        let terminal = display_only_terminal(cx, 12, 2, Some(100));

        let rendered_text = terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"hello", cx);
            terminal.with_renderable_cells(|cells| {
                cells
                    .filter(|cell| cell.point.line == 0)
                    .map(|cell| cell.character())
                    .collect::<String>()
            })
        });

        assert!(
            rendered_text.starts_with("hello"),
            "expected renderable cells to update immediately after write_output, got {rendered_text:?}"
        );
    }

    #[gpui::test]
    async fn test_write_output_preserves_existing_crlf(cx: &mut TestAppContext) {
        let terminal = cx.new(|cx| {
            TerminalBuilder::new_display_only(
                SettingsCursorShape::default(),
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

        let content = terminal.update(cx, |terminal, cx| {
            refresh_terminal_content(terminal, cx);
            terminal.last_content.clone()
        });

        let cells = &content.cells;

        // Check that both lines start at column 0
        let mut found_lines_at_column_0 = 0;
        for cell in cells {
            if cell.character() == 'l' && cell.point.column == 0 {
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
                SettingsCursorShape::default(),
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

        let content = terminal.update(cx, |terminal, cx| {
            refresh_terminal_content(terminal, cx);
            terminal.last_content.clone()
        });

        let cells = &content.cells;

        // Check that we have "world" at the beginning of the line
        let mut text = String::new();
        for cell in cells.iter().take(5) {
            if cell.point.line == 0 {
                text.push(cell.character());
            }
        }

        assert!(
            text.starts_with("world"),
            "Bare CR should allow overwriting: got '{}'",
            text
        );
    }

    #[gpui::test]
    async fn test_display_only_write_output_ignores_osc52(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.write_to_clipboard(ClipboardItem::new_string("original".to_string()));
        });

        let terminal = cx.new(|cx| {
            TerminalBuilder::new_display_only(
                SettingsCursorShape::default(),
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
            terminal.write_output(b"\x1b]52;c;b3ZlcndyaXR0ZW4=\x07", cx);
        });
        cx.run_until_parked();

        let clipboard_text = cx.update(|cx| cx.read_from_clipboard().and_then(|item| item.text()));
        assert_eq!(clipboard_text.as_deref(), Some("original"));
    }

    #[gpui::test]
    async fn test_terminal_search_includes_scrollback(cx: &mut TestAppContext) {
        let terminal = display_only_terminal(cx, 12, 2, Some(100));

        let matches = terminal
            .update(cx, |terminal, cx| {
                terminal.write_output(b"first\nsecond\nthird\nfourth\n", cx);
                refresh_terminal_content(terminal, cx);
                terminal.find_matches(Search::new("first").unwrap(), cx)
            })
            .await;

        assert!(
            matches.iter().any(|range| range.start().line < 0),
            "expected search to find first line in scrollback, got {matches:?}"
        );
    }

    #[gpui::test]
    async fn test_search_snapshot_restarts_after_terminal_mutation(cx: &mut TestAppContext) {
        let terminal = display_only_terminal(cx, 32, 4, Some(2000));

        let (stale_matches, fresh_matches) = terminal.update(cx, |terminal, cx| {
            let row_count = SEARCH_SNAPSHOT_ROWS_PER_TICK + 20;
            let mut stale_output = String::new();
            let mut fresh_output = String::new();
            for row in 0..row_count {
                stale_output.push_str(&format!("stale-hit-{row}\n"));
                fresh_output.push_str(&format!("fresh-hit-{row}\n"));
            }

            terminal.write_output(stale_output.as_bytes(), cx);
            let mut snapshot =
                SearchSnapshotBuilder::new(terminal).expect("failed to start search snapshot");
            assert!(
                !snapshot
                    .append_rows(terminal, 1)
                    .expect("failed to append initial search snapshot row")
            );

            terminal
                .clear_backend(cx)
                .expect("failed to clear terminal during search snapshot");
            terminal.write_output(fresh_output.as_bytes(), cx);
            assert!(
                snapshot
                    .append_rows(terminal, usize::MAX)
                    .expect("failed to append restarted search snapshot")
            );

            let content = snapshot.finish().expect("search snapshot should finish");
            (
                content_search_matches(
                    content.clone(),
                    Search::new("stale-hit")
                        .expect("valid stale search")
                        .regex()
                        .expect("valid stale regex"),
                ),
                content_search_matches(
                    content,
                    Search::new("fresh-hit")
                        .expect("valid fresh search")
                        .regex()
                        .expect("valid fresh regex"),
                ),
            )
        });

        assert!(stale_matches.is_empty());
        assert!(!fresh_matches.is_empty());
    }

    #[gpui::test]
    async fn test_select_all_includes_scrollback(cx: &mut TestAppContext) {
        let terminal = display_only_terminal(cx, 12, 2, Some(100));

        let selection_text = terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"first\nsecond\nthird\nfourth\n", cx);
            refresh_terminal_content(terminal, cx);
            terminal.select_all();
            apply_pending_selection_events(terminal);
            refresh_terminal_content(terminal, cx);
            terminal
                .last_content
                .selection_text
                .clone()
                .unwrap_or_default()
        });

        assert!(
            selection_text.contains("first"),
            "expected select all to include scrollback, got {selection_text:?}"
        );
        assert!(
            selection_text.contains("fourth"),
            "expected select all to include visible output, got {selection_text:?}"
        );
    }

    #[gpui::test]
    async fn test_selection_follows_output_scrolling_into_scrollback(cx: &mut TestAppContext) {
        let terminal = display_only_terminal(cx, 12, 3, Some(100));

        let selection_text = terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"first\nsecond\nthird", cx);
            refresh_terminal_content(terminal, cx);

            terminal.set_selection(Some(Selection::simple_range(Range::new(
                Point::new(0, 0),
                Point::new(0, 4),
            ))));
            apply_pending_selection_events(terminal);
            refresh_terminal_content(terminal, cx);
            assert_eq!(
                terminal.last_content.selection_text.as_deref(),
                Some("first")
            );

            terminal.write_output(b"\nfourth\nfifth", cx);
            terminal
                .last_content
                .selection_text
                .clone()
                .unwrap_or_default()
        });

        assert_eq!(selection_text, "first");
    }

    #[gpui::test]
    async fn test_selection_text_preserves_soft_wrapped_lines(cx: &mut TestAppContext) {
        let terminal = display_only_terminal(cx, 5, 4, Some(100));

        let selection_text = terminal.update(cx, |terminal, cx| {
            terminal.write_output(b"abcdef", cx);
            refresh_terminal_content(terminal, cx);

            let content = terminal
                .backend
                .full_content(&terminal.last_content)
                .expect("failed to build full terminal content");
            let selection = Selection::simple_range(Range::new(Point::new(0, 0), Point::new(1, 0)));
            let range = selection
                .to_range(&content)
                .expect("expected wrapped selection range");
            selection.selected_text(&content, &range)
        });

        assert_eq!(selection_text, "abcdef");
    }

    #[gpui::test]
    async fn test_hovered_link_refreshes_after_scroll(cx: &mut TestAppContext) {
        let terminal_bounds = display_only_bounds(24, 2);
        let window = cx.add_empty_window();
        let terminal = window.new(|cx| {
            TerminalBuilder::new_display_only_with_bounds(
                SettingsCursorShape::default(),
                AlternateScroll::On,
                Some(100),
                0,
                cx.background_executor(),
                PathStyle::local(),
                terminal_bounds,
            )
            .expect("failed to build display-only terminal")
            .subscribe(cx)
        });

        window.update_window_entity(&terminal, |terminal, window, cx| {
            let hover_position = point(px(1.0), px(1.0));
            window.set_modifiers(Modifiers::secondary_key());
            window.simulate_mouse_move(hover_position, cx);

            terminal.write_output(b"https://old.dev/\nhttps://zed.dev/\ntail", cx);
            refresh_terminal_content(terminal, cx);
            terminal.refresh_hovered_word_from_current_content(window, cx);
            assert_eq!(
                terminal
                    .last_content
                    .last_hovered_word
                    .as_ref()
                    .map(|word| word.word.as_str()),
                Some("https://zed.dev/")
            );

            terminal
                .events
                .push_back(InternalEvent::Scroll(Scroll::Delta(1)));
            terminal.sync(window, cx);

            assert_eq!(
                terminal
                    .last_content
                    .last_hovered_word
                    .as_ref()
                    .map(|word| word.word.as_str()),
                Some("https://old.dev/")
            );
        });
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

    #[cfg(unix)]
    async fn assert_foreground_process_command_eventually(
        terminal: &Entity<Terminal>,
        expected: &str,
        cx: &mut TestAppContext,
    ) {
        let mut command_name = None;
        for _ in 0..100 {
            terminal.update(cx, |terminal, _| {
                if let TerminalType::Pty { info, .. } = &terminal.terminal_type {
                    info.load_for_test();
                }
            });
            command_name =
                terminal.update(cx, |terminal, _| terminal.foreground_process_command_name());
            if command_name.as_deref() == Some(expected) {
                return;
            }
            cx.background_executor
                .timer(Duration::from_millis(10))
                .await;
        }
        let process_info = terminal.update(cx, |terminal, _| match &terminal.terminal_type {
            TerminalType::Pty { info, .. } => format!(
                "pid={:?}, fallback_pid={:?}, has_current_info={}",
                info.pid(),
                info.pid_getter().fallback_pid(),
                info.current.read().is_some()
            ),
            TerminalType::DisplayOnly => "display-only".to_string(),
        });
        panic!(
            "Expected foreground process command name to be {expected:?}, got {command_name:?}; process info: {process_info:?}"
        );
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
            Entity, ScrollDelta, ScrollWheelEvent, TestAppContext, VisualContext,
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
                        SettingsCursorShape::default(),
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
                            delta: ScrollDelta::Lines(GpuiPoint::new(0.0, lines as f32)),
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
                            origin: GpuiPoint::default(),
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
                            origin: GpuiPoint::default(),
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
