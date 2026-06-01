use alacritty_terminal::{
    grid::{GridIterator, Scroll as AlacScroll},
    index::{Column, Direction as AlacDirection, Line, Point as AlacPoint},
    selection::{Selection, SelectionRange, SelectionType as AlacSelectionType},
    term::{
        RenderableCursor, TermMode,
        cell::{Cell as AlacCell, Flags, Hyperlink as AlacHyperlink},
        search::RegexSearch,
    },
    vi_mode::ViMotion,
    vte::ansi::CursorShape as AlacCursorShape,
};
use std::{
    ops::{BitOr, BitOrAssign, Deref, RangeInclusive},
    sync::Arc,
};
pub use vte::ansi::{Color as TerminalColor, NamedColor as TerminalNamedColor, Rgb as TerminalRgb};

use crate::{TerminalBounds, terminal_settings::CursorShape};

#[derive(Clone, Copy, Debug)]
pub(super) enum TerminalScroll {
    Delta(i32),
    PageUp,
    PageDown,
    Top,
    Bottom,
}

impl TerminalScroll {
    pub(super) fn to_alacritty(self) -> AlacScroll {
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
pub(super) enum TerminalViMotion {
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
    pub(super) fn to_alacritty(self) -> ViMotion {
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
    search: RegexSearch,
}

impl TerminalSearch {
    pub fn new(search: &str) -> Option<Self> {
        Some(Self {
            search: RegexSearch::new(search).ok()?,
        })
    }

    pub(super) fn into_alacritty(self) -> RegexSearch {
        self.search
    }
}

#[derive(Clone, Debug)]
pub(super) struct TerminalSelection {
    ty: TerminalSelectionType,
    start: TerminalSelectionAnchor,
    end: TerminalSelectionAnchor,
    pub(super) head: TerminalPoint,
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
    pub(super) fn to_alacritty(self) -> AlacDirection {
        match self {
            Self::Left => AlacDirection::Left,
            Self::Right => AlacDirection::Right,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TerminalSelectionType {
    Simple,
    Semantic,
    Lines,
}

impl TerminalSelectionType {
    fn to_alacritty(self) -> AlacSelectionType {
        match self {
            Self::Simple => AlacSelectionType::Simple,
            Self::Semantic => AlacSelectionType::Semantic,
            Self::Lines => AlacSelectionType::Lines,
        }
    }
}

impl TerminalSelection {
    pub(super) fn new(
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

    pub(super) fn simple_range(range: TerminalRange) -> Self {
        let mut selection = Self::new(
            TerminalSelectionType::Simple,
            range.start(),
            TerminalSelectionSide::Left,
        );
        selection.update(range.end(), TerminalSelectionSide::Right);
        selection
    }

    pub(super) fn update(&mut self, point: TerminalPoint, side: TerminalSelectionSide) {
        self.end = TerminalSelectionAnchor { point, side };
        self.head = point;
    }

    pub(super) fn to_alacritty(&self) -> Selection {
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
}

pub fn is_default_background_color(color: TerminalColor) -> bool {
    matches!(color, TerminalColor::Named(TerminalNamedColor::Background))
}

pub fn is_app_chosen_exact_color(color: TerminalColor) -> bool {
    matches!(
        color,
        TerminalColor::Spec(_) | TerminalColor::Indexed(16..=255)
    )
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TerminalHyperlink {
    data: TerminalHyperlinkData,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TerminalHyperlinkData {
    Alacritty(AlacHyperlink),
    Owned { id: Option<Arc<str>>, uri: Arc<str> },
}

impl TerminalHyperlink {
    pub fn new<T: ToString>(id: Option<T>, uri: String) -> Self {
        Self {
            data: TerminalHyperlinkData::Owned {
                id: id.map(|id| Arc::from(id.to_string())),
                uri: Arc::from(uri),
            },
        }
    }

    pub fn id(&self) -> Option<&str> {
        match &self.data {
            TerminalHyperlinkData::Alacritty(hyperlink) => Some(hyperlink.id()),
            TerminalHyperlinkData::Owned { id, .. } => id.as_deref(),
        }
    }

    pub fn uri(&self) -> &str {
        match &self.data {
            TerminalHyperlinkData::Alacritty(hyperlink) => hyperlink.uri(),
            TerminalHyperlinkData::Owned { uri, .. } => uri,
        }
    }

    fn from_alacritty(hyperlink: AlacHyperlink) -> Self {
        Self {
            data: TerminalHyperlinkData::Alacritty(hyperlink),
        }
    }
}

fn terminal_hyperlink_from_alacritty(hyperlink: AlacHyperlink) -> TerminalHyperlink {
    TerminalHyperlink::from_alacritty(hyperlink)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_hyperlink_from_alacritty_keeps_alacritty_storage() {
        let hyperlink = AlacHyperlink::new(Some("id"), "https://example.com".to_string());
        let hyperlink = terminal_hyperlink_from_alacritty(hyperlink);

        assert!(matches!(
            &hyperlink.data,
            TerminalHyperlinkData::Alacritty(_)
        ));
        assert_eq!(hyperlink.id(), Some("id"));
        assert_eq!(hyperlink.uri(), "https://example.com");
    }

    #[test]
    fn terminal_cell_clone_shares_extra_storage() {
        let mut cell = TerminalCell::default();
        cell.push_zerowidth('a');

        let clone = cell.clone();

        match (&cell.cell.extra, &clone.cell.extra) {
            (Some(extra), Some(clone_extra)) => assert!(Arc::ptr_eq(extra, clone_extra)),
            _ => panic!("expected extra storage on both cells"),
        }
    }

    #[test]
    fn terminal_cell_from_alacritty_shares_extra_storage() {
        let mut cell = AlacCell::default();
        cell.push_zerowidth('a');

        let converted = terminal_cell_from_alacritty(&cell);

        match (&cell.extra, &converted.cell.extra) {
            (Some(extra), Some(converted_extra)) => assert!(Arc::ptr_eq(extra, converted_extra)),
            _ => panic!("expected extra storage on both cells"),
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct TerminalCell {
    cell: AlacCell,
}

impl TerminalCell {
    #[inline]
    pub fn character(&self) -> char {
        self.cell.c
    }

    #[inline]
    pub fn set_character(&mut self, character: char) {
        self.cell.c = character;
    }

    #[inline]
    pub fn foreground(&self) -> TerminalColor {
        self.cell.fg
    }

    #[inline]
    pub fn background(&self) -> TerminalColor {
        self.cell.bg
    }

    #[inline]
    pub fn zerowidth(&self) -> Option<&[char]> {
        self.cell.zerowidth()
    }

    #[inline]
    pub fn push_zerowidth(&mut self, character: char) {
        self.cell.push_zerowidth(character);
    }

    pub fn set_underline_color(&mut self, color: Option<TerminalColor>) {
        self.cell.set_underline_color(color);
    }

    #[inline]
    pub fn underline_color(&self) -> Option<TerminalColor> {
        self.cell.underline_color()
    }

    pub fn set_hyperlink(&mut self, hyperlink: Option<TerminalHyperlink>) {
        self.cell.set_hyperlink(hyperlink.map(Into::into));
    }

    #[inline]
    pub fn hyperlink(&self) -> Option<TerminalHyperlink> {
        self.cell.hyperlink().map(terminal_hyperlink_from_alacritty)
    }

    #[inline]
    pub fn is_inverse(&self) -> bool {
        self.cell.flags.contains(Flags::INVERSE)
    }

    #[inline]
    pub fn is_wide_char_spacer(&self) -> bool {
        self.cell.flags.contains(Flags::WIDE_CHAR_SPACER)
    }

    #[inline]
    pub fn is_dim(&self) -> bool {
        self.cell.flags.intersects(Flags::DIM)
    }

    #[inline]
    pub fn has_underline(&self) -> bool {
        self.cell.flags.intersects(Flags::ALL_UNDERLINES)
    }

    #[inline]
    pub fn has_undercurl(&self) -> bool {
        self.cell.flags.contains(Flags::UNDERCURL)
    }

    #[inline]
    pub fn has_strikeout(&self) -> bool {
        self.cell.flags.intersects(Flags::STRIKEOUT)
    }

    #[inline]
    pub fn is_bold(&self) -> bool {
        self.cell.flags.intersects(Flags::BOLD)
    }

    #[inline]
    pub fn is_italic(&self) -> bool {
        self.cell.flags.intersects(Flags::ITALIC)
    }

    #[inline]
    pub fn has_visible_style_modifier(&self) -> bool {
        self.cell
            .flags
            .intersects(Flags::ALL_UNDERLINES | Flags::INVERSE | Flags::STRIKEOUT)
    }
}

impl From<TerminalHyperlink> for AlacHyperlink {
    fn from(hyperlink: TerminalHyperlink) -> Self {
        match hyperlink.data {
            TerminalHyperlinkData::Alacritty(hyperlink) => hyperlink,
            TerminalHyperlinkData::Owned { id, uri } => Self::new(id.as_deref(), uri.to_string()),
        }
    }
}

pub(super) fn terminal_cell_from_alacritty(cell: &AlacCell) -> TerminalCell {
    TerminalCell { cell: cell.clone() }
}

pub struct RenderableCells<'a> {
    cells: GridIterator<'a, AlacCell>,
}

impl<'a> RenderableCells<'a> {
    pub(super) fn new(cells: GridIterator<'a, AlacCell>) -> Self {
        Self { cells }
    }
}

impl Iterator for RenderableCells<'_> {
    type Item = IndexedCell;

    fn next(&mut self) -> Option<Self::Item> {
        self.cells.next().map(|cell| IndexedCell {
            point: terminal_point_from_alacritty(cell.point),
            cell: terminal_cell_from_alacritty(cell.cell),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cells.size_hint()
    }
}

#[derive(Debug, Clone)]
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

    #[cfg(test)]
    pub(crate) fn to_alacritty(self) -> TermMode {
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

pub(super) fn terminal_modes_from_alacritty(mode: TermMode) -> TerminalModes {
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

#[cfg(test)]
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
    pub(super) fn from_alacritty(cursor: RenderableCursor) -> Self {
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TerminalPoint {
    pub line: i32,
    pub column: usize,
}

impl TerminalPoint {
    pub fn new(line: i32, column: usize) -> Self {
        Self { line, column }
    }

    pub(super) fn to_alacritty(self) -> AlacPoint {
        AlacPoint::new(Line(self.line), Column(self.column))
    }
}

pub(super) fn terminal_point_from_alacritty(point: AlacPoint) -> TerminalPoint {
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

    #[cfg(test)]
    pub(crate) fn to_alacritty(self) -> RangeInclusive<AlacPoint> {
        self.start.to_alacritty()..=self.end.to_alacritty()
    }

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

pub(super) fn terminal_selection_range_from_alacritty(
    range: SelectionRange,
) -> TerminalSelectionRange {
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
