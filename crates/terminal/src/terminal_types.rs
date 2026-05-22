use alacritty_terminal::{
    grid::Scroll as AlacScroll,
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
use regex::Regex;
use std::{
    ops::{BitOr, BitOrAssign, Deref, RangeInclusive},
    sync::Arc,
};
use vte::ansi::{Color as VteColor, NamedColor as VteNamedColor, Rgb as VteRgb};

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
    pattern: Arc<str>,
}

impl TerminalSearch {
    pub fn new(search: &str) -> Option<Self> {
        Regex::new(search).ok()?;

        Some(Self {
            pattern: Arc::from(search),
        })
    }

    fn compile_alacritty(search: &str) -> Option<RegexSearch> {
        RegexSearch::new(search).ok()
    }

    pub(super) fn alacritty(&self) -> Option<RegexSearch> {
        Self::compile_alacritty(self.pattern.as_ref())
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

        match (&cell.extra, &clone.extra) {
            (Some(extra), Some(clone_extra)) => assert!(Arc::ptr_eq(extra, clone_extra)),
            _ => panic!("expected extra storage on both cells"),
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct TerminalCellExtra {
    zerowidth: Vec<char>,
    underline_color: Option<TerminalColor>,
    hyperlink: Option<TerminalHyperlink>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct TerminalCellFlags(u16);

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TerminalCell {
    pub c: char,
    pub fg: TerminalColor,
    pub bg: TerminalColor,
    flags: TerminalCellFlags,
    extra: Option<Arc<TerminalCellExtra>>,
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
    fn extra_mut(&mut self) -> &mut TerminalCellExtra {
        Arc::make_mut(
            self.extra
                .get_or_insert_with(|| Arc::new(TerminalCellExtra::default())),
        )
    }

    #[inline]
    pub fn zerowidth(&self) -> Option<&[char]> {
        self.extra.as_ref().map(|extra| extra.zerowidth.as_slice())
    }

    #[inline]
    pub fn push_zerowidth(&mut self, character: char) {
        self.extra_mut().zerowidth.push(character);
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
            self.extra_mut().underline_color = color;
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
            self.extra_mut().hyperlink = hyperlink;
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

pub(super) fn terminal_cell_from_alacritty(cell: AlacCell) -> TerminalCell {
    let zerowidth = cell.zerowidth().unwrap_or_default().to_vec();
    let underline_color = cell.underline_color().map(Into::into);
    let hyperlink = cell.hyperlink().map(terminal_hyperlink_from_alacritty);
    let extra = if zerowidth.is_empty() && underline_color.is_none() && hyperlink.is_none() {
        None
    } else {
        Some(Arc::new(TerminalCellExtra {
            zerowidth,
            underline_color,
            hyperlink,
        }))
    };

    TerminalCell {
        c: cell.c,
        fg: cell.fg.into(),
        bg: cell.bg.into(),
        flags: terminal_cell_flags_from_alacritty(cell.flags),
        extra,
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
