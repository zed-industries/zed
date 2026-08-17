use super::super::Properties;
use super::Boundary;

use core::fmt;

/// Information about a character including unicode properties and boundary
/// analysis.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct CharInfo(pub(crate) Properties);

impl CharInfo {
    /// Creates new character information from Unicode properties and
    /// boundary analysis.
    pub fn new(properties: Properties, boundary: Boundary) -> Self {
        Self(properties.with_boundary(boundary as u16))
    }

    /// Returns the unicode properties for the character.
    pub fn properties(self) -> Properties {
        self.0
    }

    /// Returns the boundary state.
    pub fn boundary(self) -> Boundary {
        Boundary::from_raw(self.0.boundary())
    }

    pub(crate) fn with_properties(self, props: Properties) -> Self {
        Self(props.with_boundary(self.0.boundary()))
    }
}

impl core::ops::Deref for CharInfo {
    type Target = Properties;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<char> for CharInfo {
    fn from(c: char) -> Self {
        Self(Properties::from(c))
    }
}

impl From<CharInfo> for Properties {
    fn from(a: CharInfo) -> Self {
        a.0
    }
}

impl From<&CharInfo> for Properties {
    fn from(a: &CharInfo) -> Self {
        a.0
    }
}

impl From<Properties> for CharInfo {
    fn from(p: Properties) -> Self {
        Self(p)
    }
}

impl From<&Properties> for CharInfo {
    fn from(p: &Properties) -> Self {
        Self(*p)
    }
}

const BOUND_SHIFT: u16 = 14;
const SPACE_SHIFT: u16 = 1;
const EMOJI_SHIFT: u16 = 8;
const SPACE_MASK: u16 = 0b111;
const EMOJI_MASK: u16 = 0b11;

/// Information about a cluster including content properties and boundary analysis.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct ClusterInfo(pub u16);

impl ClusterInfo {
    /// Returns true if the cluster is missing an appropriate base
    /// character.
    pub fn is_broken(self) -> bool {
        self.0 & 1 != 0
    }

    /// Returns true if the cluster is an emoji.
    pub fn is_emoji(self) -> bool {
        (self.0 >> EMOJI_SHIFT & EMOJI_MASK) != 0
    }

    /// Returns the emoji presentation mode of the cluster.
    pub fn emoji(self) -> Emoji {
        Emoji::from_raw(self.0 >> EMOJI_SHIFT & EMOJI_MASK)
    }

    /// Returns true if the cluster is whitespace.
    pub fn is_whitespace(self) -> bool {
        (self.0 >> SPACE_SHIFT & SPACE_MASK) != 0
    }

    /// Returns the whitespace content of the cluster.
    pub fn whitespace(self) -> Whitespace {
        Whitespace::from_raw(self.0 >> SPACE_SHIFT & SPACE_MASK)
    }

    /// Returns true if the cluster is a boundary.
    pub fn is_boundary(self) -> bool {
        (self.0 >> BOUND_SHIFT) != 0
    }

    /// Returns the boundary state of the cluster.
    pub fn boundary(self) -> Boundary {
        Boundary::from_raw(self.0 >> BOUND_SHIFT)
    }

    pub(super) fn set_broken(&mut self) {
        self.0 |= 1;
    }

    pub(super) fn set_emoji(&mut self, emoji: Emoji) {
        self.0 = self.0 & !(EMOJI_MASK << EMOJI_SHIFT) | (emoji as u16) << EMOJI_SHIFT;
    }

    pub(super) fn set_space(&mut self, space: Whitespace) {
        self.0 = self.0 & !(SPACE_MASK << SPACE_SHIFT) | (space as u16) << SPACE_SHIFT;
    }

    #[inline]
    pub(super) fn set_space_from_char(&mut self, ch: char) {
        match ch {
            ' ' => self.set_space(Whitespace::Space),
            '\u{a0}' => self.set_space(Whitespace::NoBreakSpace),
            '\t' => self.set_space(Whitespace::Tab),
            _ => {}
        }
    }

    pub(super) fn merge_boundary(&mut self, boundary: u16) {
        let bits = (self.0 >> BOUND_SHIFT).max(boundary) << BOUND_SHIFT;
        self.0 = ((self.0 << 2) >> 2) | bits;
    }
}

impl fmt::Debug for ClusterInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
        let e = match self.emoji() {
            Emoji::None => " ",
            Emoji::Default => "E",
            Emoji::Text => "T",
            Emoji::Color => "C",
        };
        let s = match self.whitespace() {
            Whitespace::None => " ",
            Whitespace::Space => "s",
            Whitespace::NoBreakSpace => "b",
            Whitespace::Tab => "t",
            Whitespace::Newline => "n",
            Whitespace::Other => "o",
        };
        write!(f, "{}", if self.is_broken() { "!" } else { " " })?;
        let b = match self.boundary() {
            Boundary::Mandatory => "L",
            Boundary::Line => "l",
            Boundary::Word => "w",
            _ => " ",
        };
        write!(f, "{}{}{}", e, s, b)
    }
}

/// Presentation mode for an emoji cluster.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Emoji {
    /// Not an emoji.
    None = 0,
    /// Emoji with default presentation.
    Default = 1,
    /// Emoji with text presentation.
    Text = 2,
    /// Emoji with color presentation.
    Color = 3,
}

impl Emoji {
    #[inline]
    fn from_raw(bits: u16) -> Self {
        match bits & 0b11 {
            0 => Self::None,
            1 => Self::Default,
            2 => Self::Text,
            3 => Self::Color,
            _ => Self::None,
        }
    }
}

/// Whitespace content of a cluster.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Whitespace {
    /// Not a space.
    None = 0,
    /// Standard space.
    Space = 1,
    /// Non-breaking space (U+00A0).
    NoBreakSpace = 2,
    /// Horizontal tab.
    Tab = 3,
    /// Newline (CR, LF, or CRLF).
    Newline = 4,
    /// Other space.
    Other = 5,
}

impl Whitespace {
    /// Returns true for space or no break space.
    pub fn is_space_or_nbsp(self) -> bool {
        matches!(self, Self::Space | Self::NoBreakSpace)
    }

    #[inline]
    fn from_raw(bits: u16) -> Self {
        match bits & 0b111 {
            0 => Self::None,
            1 => Self::Space,
            2 => Self::NoBreakSpace,
            3 => Self::Tab,
            4 => Self::Newline,
            5 => Self::Other,
            _ => Self::None,
        }
    }
}
