use alloc::borrow::Cow;
use core::{
    fmt::{self, Debug, Formatter},
    sync::atomic::{AtomicBool, Ordering},
};
use std::env;

use std::sync::OnceLock;

use crate::term::{wants_emoji, Term};

#[cfg(feature = "ansi-parsing")]
use crate::ansi::AnsiCodeIterator;

fn default_colors_enabled(out: &Term) -> bool {
    (out.features().colors_supported()
        && &env::var("CLICOLOR").unwrap_or_else(|_| "1".into()) != "0")
        || &env::var("CLICOLOR_FORCE").unwrap_or_else(|_| "0".into()) != "0"
}

fn default_true_colors_enabled(out: &Term) -> bool {
    out.features().true_colors_supported()
}

fn stdout_colors() -> &'static AtomicBool {
    static ENABLED: OnceLock<AtomicBool> = OnceLock::new();
    ENABLED.get_or_init(|| AtomicBool::new(default_colors_enabled(&Term::stdout())))
}
fn stdout_true_colors() -> &'static AtomicBool {
    static ENABLED: OnceLock<AtomicBool> = OnceLock::new();
    ENABLED.get_or_init(|| AtomicBool::new(default_true_colors_enabled(&Term::stdout())))
}
fn stderr_colors() -> &'static AtomicBool {
    static ENABLED: OnceLock<AtomicBool> = OnceLock::new();
    ENABLED.get_or_init(|| AtomicBool::new(default_colors_enabled(&Term::stderr())))
}
fn stderr_true_colors() -> &'static AtomicBool {
    static ENABLED: OnceLock<AtomicBool> = OnceLock::new();
    ENABLED.get_or_init(|| AtomicBool::new(default_true_colors_enabled(&Term::stderr())))
}

/// Returns `true` if colors should be enabled for stdout.
///
/// This honors the [clicolors spec](http://bixense.com/clicolors/).
///
/// * `CLICOLOR != 0`: ANSI colors are supported and should be used when the program isn't piped.
/// * `CLICOLOR == 0`: Don't output ANSI color escape codes.
/// * `CLICOLOR_FORCE != 0`: ANSI colors should be enabled no matter what.
#[inline]
pub fn colors_enabled() -> bool {
    stdout_colors().load(Ordering::Relaxed)
}

/// Returns `true` if true colors should be enabled for stdout.
#[inline]
pub fn true_colors_enabled() -> bool {
    stdout_true_colors().load(Ordering::Relaxed)
}

/// Forces colorization on or off for stdout.
///
/// This overrides the default for the current process and changes the return value of the
/// `colors_enabled` function.
#[inline]
pub fn set_colors_enabled(val: bool) {
    stdout_colors().store(val, Ordering::Relaxed)
}

/// Forces true colorization on or off for stdout.
///
/// This overrides the default for the current process and changes the return value of the
/// `true_colors_enabled` function.
#[inline]
pub fn set_true_colors_enabled(val: bool) {
    stdout_true_colors().store(val, Ordering::Relaxed)
}

/// Returns `true` if colors should be enabled for stderr.
///
/// This honors the [clicolors spec](http://bixense.com/clicolors/).
///
/// * `CLICOLOR != 0`: ANSI colors are supported and should be used when the program isn't piped.
/// * `CLICOLOR == 0`: Don't output ANSI color escape codes.
/// * `CLICOLOR_FORCE != 0`: ANSI colors should be enabled no matter what.
#[inline]
pub fn colors_enabled_stderr() -> bool {
    stderr_colors().load(Ordering::Relaxed)
}

/// Returns `true` if true colors should be enabled for stderr.
#[inline]
pub fn true_colors_enabled_stderr() -> bool {
    stderr_true_colors().load(Ordering::Relaxed)
}

/// Forces colorization on or off for stderr.
///
/// This overrides the default for the current process and changes the return value of the
/// `colors_enabled_stderr` function.
#[inline]
pub fn set_colors_enabled_stderr(val: bool) {
    stderr_colors().store(val, Ordering::Relaxed)
}

/// Forces true colorization on or off for stderr.
///
/// This overrides the default for the current process and changes the return value of the
/// `true_colors_enabled_stderr` function.
#[inline]
pub fn set_true_colors_enabled_stderr(val: bool) {
    stderr_true_colors().store(val, Ordering::Relaxed)
}

/// Measure the width of a string in terminal characters.
pub fn measure_text_width(s: &str) -> usize {
    #[cfg(feature = "ansi-parsing")]
    {
        AnsiCodeIterator::new(s)
            .filter_map(|(s, is_ansi)| match is_ansi {
                false => Some(str_width(s)),
                true => None,
            })
            .sum()
    }
    #[cfg(not(feature = "ansi-parsing"))]
    {
        str_width(s)
    }
}

/// A terminal color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Color256(u8),
    TrueColor(u8, u8, u8),
}

impl Color {
    #[inline]
    fn ansi_num(self) -> usize {
        match self {
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
            Color::Color256(x) => x as usize,
            Color::TrueColor(_, _, _) => panic!("RGB colors must be handled separately"),
        }
    }

    #[inline]
    fn is_color256(self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Color::Color256(_) => true,
            _ => false,
        }
    }
}

/// A terminal style attribute.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u16)]
pub enum Attribute {
    // This mapping is important, it exactly matches ansi_num = (x as u16 + 1)
    // See `ATTRIBUTES_LOOKUP` as well
    Bold = 0,
    Dim = 1,
    Italic = 2,
    Underlined = 3,
    Blink = 4,
    BlinkFast = 5,
    Reverse = 6,
    Hidden = 7,
    StrikeThrough = 8,
}

impl Attribute {
    const MAP: [Attribute; 9] = [
        Attribute::Bold,
        Attribute::Dim,
        Attribute::Italic,
        Attribute::Underlined,
        Attribute::Blink,
        Attribute::BlinkFast,
        Attribute::Reverse,
        Attribute::Hidden,
        Attribute::StrikeThrough,
    ];
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Attributes(u16);

impl Attributes {
    #[inline]
    const fn new() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    const fn insert(mut self, attr: Attribute) -> Self {
        let bit = attr as u16;
        self.0 |= 1 << bit;
        self
    }

    #[inline]
    const fn bits(self) -> BitsIter {
        BitsIter(self.0)
    }

    #[inline]
    fn attrs(self) -> impl Iterator<Item = Attribute> {
        self.bits().map(|bit| Attribute::MAP[bit as usize])
    }

    #[inline]
    fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ansi in self.bits().map(|bit| bit + 1) {
            write!(f, "\x1b[{ansi}m")?;
        }
        Ok(())
    }
}

struct BitsIter(u16);

impl Iterator for BitsIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = self.0.trailing_zeros();
        self.0 ^= (1 << bit) as u16;
        Some(bit as u16)
    }
}

impl Debug for Attributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.attrs()).finish()
    }
}

/// Defines the alignment for padding operations.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// A stored style that can be applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    fg_bright: bool,
    bg_bright: bool,
    attrs: Attributes,
    force: Option<bool>,
    for_stderr: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Style {
    /// Returns an empty default style.
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            fg_bright: false,
            bg_bright: false,
            attrs: Attributes::new(),
            force: None,
            for_stderr: false,
        }
    }

    /// Creates a style from a dotted string.
    ///
    /// Effectively the string is split at each dot and then the
    /// terms in between are applied.  For instance `red.on_blue` will
    /// create a string that is red on blue background. `9.on_12` is
    /// the same, but using 256 color numbers. Unknown terms are
    /// ignored.
    pub fn from_dotted_str(s: &str) -> Self {
        let mut rv = Self::new();
        for part in s.split('.') {
            rv = match part {
                "black" => rv.black(),
                "red" => rv.red(),
                "green" => rv.green(),
                "yellow" => rv.yellow(),
                "blue" => rv.blue(),
                "magenta" => rv.magenta(),
                "cyan" => rv.cyan(),
                "white" => rv.white(),
                "bright" => rv.bright(),
                "on_black" => rv.on_black(),
                "on_red" => rv.on_red(),
                "on_green" => rv.on_green(),
                "on_yellow" => rv.on_yellow(),
                "on_blue" => rv.on_blue(),
                "on_magenta" => rv.on_magenta(),
                "on_cyan" => rv.on_cyan(),
                "on_white" => rv.on_white(),
                "on_bright" => rv.on_bright(),
                "bold" => rv.bold(),
                "dim" => rv.dim(),
                "underlined" => rv.underlined(),
                "blink" => rv.blink(),
                "blink_fast" => rv.blink_fast(),
                "reverse" => rv.reverse(),
                "hidden" => rv.hidden(),
                "strikethrough" => rv.strikethrough(),
                on_true_color if on_true_color.starts_with("on_#") && on_true_color.len() == 10 => {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&on_true_color[4..6], 16),
                        u8::from_str_radix(&on_true_color[6..8], 16),
                        u8::from_str_radix(&on_true_color[8..10], 16),
                    ) {
                        rv.on_true_color(r, g, b)
                    } else {
                        continue;
                    }
                }
                true_color if true_color.starts_with('#') && true_color.len() == 7 => {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&true_color[1..3], 16),
                        u8::from_str_radix(&true_color[3..5], 16),
                        u8::from_str_radix(&true_color[5..7], 16),
                    ) {
                        rv.true_color(r, g, b)
                    } else {
                        continue;
                    }
                }
                on_c if on_c.starts_with("on_") => {
                    if let Ok(n) = on_c[3..].parse::<u8>() {
                        rv.on_color256(n)
                    } else {
                        continue;
                    }
                }
                c => {
                    if let Ok(n) = c.parse::<u8>() {
                        rv.color256(n)
                    } else {
                        continue;
                    }
                }
            };
        }
        rv
    }

    /// Apply the style to something that can be displayed.
    pub fn apply_to<D>(&self, val: D) -> StyledObject<D> {
        StyledObject {
            style: self.clone(),
            val,
        }
    }

    /// Forces styling on or off.
    ///
    /// This overrides the automatic detection.
    #[inline]
    pub const fn force_styling(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Specifies that style is applying to something being written on stderr.
    #[inline]
    pub const fn for_stderr(mut self) -> Self {
        self.for_stderr = true;
        self
    }

    /// Specifies that style is applying to something being written on stdout.
    ///
    /// This is the default behaviour.
    #[inline]
    pub const fn for_stdout(mut self) -> Self {
        self.for_stderr = false;
        self
    }

    /// Sets a foreground color.
    #[inline]
    pub const fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets a background color.
    #[inline]
    pub const fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Adds a attr.
    #[inline]
    pub const fn attr(mut self, attr: Attribute) -> Self {
        self.attrs = self.attrs.insert(attr);
        self
    }

    #[inline]
    pub const fn black(self) -> Self {
        self.fg(Color::Black)
    }
    #[inline]
    pub const fn red(self) -> Self {
        self.fg(Color::Red)
    }
    #[inline]
    pub const fn green(self) -> Self {
        self.fg(Color::Green)
    }
    #[inline]
    pub const fn yellow(self) -> Self {
        self.fg(Color::Yellow)
    }
    #[inline]
    pub const fn blue(self) -> Self {
        self.fg(Color::Blue)
    }
    #[inline]
    pub const fn magenta(self) -> Self {
        self.fg(Color::Magenta)
    }
    #[inline]
    pub const fn cyan(self) -> Self {
        self.fg(Color::Cyan)
    }
    #[inline]
    pub const fn white(self) -> Self {
        self.fg(Color::White)
    }
    #[inline]
    pub const fn color256(self, color: u8) -> Self {
        self.fg(Color::Color256(color))
    }
    #[inline]
    pub const fn true_color(self, r: u8, g: u8, b: u8) -> Self {
        self.fg(Color::TrueColor(r, g, b))
    }

    #[inline]
    pub const fn bright(mut self) -> Self {
        self.fg_bright = true;
        self
    }

    #[inline]
    pub const fn on_black(self) -> Self {
        self.bg(Color::Black)
    }
    #[inline]
    pub const fn on_red(self) -> Self {
        self.bg(Color::Red)
    }
    #[inline]
    pub const fn on_green(self) -> Self {
        self.bg(Color::Green)
    }
    #[inline]
    pub const fn on_yellow(self) -> Self {
        self.bg(Color::Yellow)
    }
    #[inline]
    pub const fn on_blue(self) -> Self {
        self.bg(Color::Blue)
    }
    #[inline]
    pub const fn on_magenta(self) -> Self {
        self.bg(Color::Magenta)
    }
    #[inline]
    pub const fn on_cyan(self) -> Self {
        self.bg(Color::Cyan)
    }
    #[inline]
    pub const fn on_white(self) -> Self {
        self.bg(Color::White)
    }
    #[inline]
    pub const fn on_color256(self, color: u8) -> Self {
        self.bg(Color::Color256(color))
    }
    #[inline]
    pub const fn on_true_color(self, r: u8, g: u8, b: u8) -> Self {
        self.bg(Color::TrueColor(r, g, b))
    }

    #[inline]
    pub const fn on_bright(mut self) -> Self {
        self.bg_bright = true;
        self
    }

    #[inline]
    pub const fn bold(self) -> Self {
        self.attr(Attribute::Bold)
    }
    #[inline]
    pub const fn dim(self) -> Self {
        self.attr(Attribute::Dim)
    }
    #[inline]
    pub const fn italic(self) -> Self {
        self.attr(Attribute::Italic)
    }
    #[inline]
    pub const fn underlined(self) -> Self {
        self.attr(Attribute::Underlined)
    }
    #[inline]
    pub const fn blink(self) -> Self {
        self.attr(Attribute::Blink)
    }
    #[inline]
    pub const fn blink_fast(self) -> Self {
        self.attr(Attribute::BlinkFast)
    }
    #[inline]
    pub const fn reverse(self) -> Self {
        self.attr(Attribute::Reverse)
    }
    #[inline]
    pub const fn hidden(self) -> Self {
        self.attr(Attribute::Hidden)
    }
    #[inline]
    pub const fn strikethrough(self) -> Self {
        self.attr(Attribute::StrikeThrough)
    }
}

/// Wraps an object for formatting for styling.
///
/// Example:
///
/// ```rust,no_run
/// # use console::style;
/// format!("Hello {}", style("World").cyan());
/// ```
///
/// This is a shortcut for making a new style and applying it
/// to a value:
///
/// ```rust,no_run
/// # use console::Style;
/// format!("Hello {}", Style::new().cyan().apply_to("World"));
/// ```
pub fn style<D>(val: D) -> StyledObject<D> {
    Style::new().apply_to(val)
}

/// A formatting wrapper that can be styled for a terminal.
#[derive(Clone)]
pub struct StyledObject<D> {
    style: Style,
    val: D,
}

impl<D> StyledObject<D> {
    /// Forces styling on or off.
    ///
    /// This overrides the automatic detection.
    #[inline]
    pub fn force_styling(mut self, value: bool) -> StyledObject<D> {
        self.style = self.style.force_styling(value);
        self
    }

    /// Specifies that style is applying to something being written on stderr
    #[inline]
    pub fn for_stderr(mut self) -> StyledObject<D> {
        self.style = self.style.for_stderr();
        self
    }

    /// Specifies that style is applying to something being written on stdout
    ///
    /// This is the default
    #[inline]
    pub const fn for_stdout(mut self) -> StyledObject<D> {
        self.style = self.style.for_stdout();
        self
    }

    /// Sets a foreground color.
    #[inline]
    pub const fn fg(mut self, color: Color) -> StyledObject<D> {
        self.style = self.style.fg(color);
        self
    }

    /// Sets a background color.
    #[inline]
    pub const fn bg(mut self, color: Color) -> StyledObject<D> {
        self.style = self.style.bg(color);
        self
    }

    /// Adds a attr.
    #[inline]
    pub const fn attr(mut self, attr: Attribute) -> StyledObject<D> {
        self.style = self.style.attr(attr);
        self
    }

    #[inline]
    pub const fn black(self) -> StyledObject<D> {
        self.fg(Color::Black)
    }
    #[inline]
    pub const fn red(self) -> StyledObject<D> {
        self.fg(Color::Red)
    }
    #[inline]
    pub const fn green(self) -> StyledObject<D> {
        self.fg(Color::Green)
    }
    #[inline]
    pub const fn yellow(self) -> StyledObject<D> {
        self.fg(Color::Yellow)
    }
    #[inline]
    pub const fn blue(self) -> StyledObject<D> {
        self.fg(Color::Blue)
    }
    #[inline]
    pub const fn magenta(self) -> StyledObject<D> {
        self.fg(Color::Magenta)
    }
    #[inline]
    pub const fn cyan(self) -> StyledObject<D> {
        self.fg(Color::Cyan)
    }
    #[inline]
    pub const fn white(self) -> StyledObject<D> {
        self.fg(Color::White)
    }
    #[inline]
    pub const fn color256(self, color: u8) -> StyledObject<D> {
        self.fg(Color::Color256(color))
    }
    #[inline]
    pub const fn true_color(self, r: u8, g: u8, b: u8) -> StyledObject<D> {
        self.fg(Color::TrueColor(r, g, b))
    }

    #[inline]
    pub const fn bright(mut self) -> StyledObject<D> {
        self.style = self.style.bright();
        self
    }

    #[inline]
    pub const fn on_black(self) -> StyledObject<D> {
        self.bg(Color::Black)
    }
    #[inline]
    pub const fn on_red(self) -> StyledObject<D> {
        self.bg(Color::Red)
    }
    #[inline]
    pub const fn on_green(self) -> StyledObject<D> {
        self.bg(Color::Green)
    }
    #[inline]
    pub const fn on_yellow(self) -> StyledObject<D> {
        self.bg(Color::Yellow)
    }
    #[inline]
    pub const fn on_blue(self) -> StyledObject<D> {
        self.bg(Color::Blue)
    }
    #[inline]
    pub const fn on_magenta(self) -> StyledObject<D> {
        self.bg(Color::Magenta)
    }
    #[inline]
    pub const fn on_cyan(self) -> StyledObject<D> {
        self.bg(Color::Cyan)
    }
    #[inline]
    pub const fn on_white(self) -> StyledObject<D> {
        self.bg(Color::White)
    }
    #[inline]
    pub const fn on_color256(self, color: u8) -> StyledObject<D> {
        self.bg(Color::Color256(color))
    }
    #[inline]
    pub const fn on_true_color(self, r: u8, g: u8, b: u8) -> StyledObject<D> {
        self.bg(Color::TrueColor(r, g, b))
    }

    #[inline]
    pub const fn on_bright(mut self) -> StyledObject<D> {
        self.style = self.style.on_bright();
        self
    }

    #[inline]
    pub const fn bold(self) -> StyledObject<D> {
        self.attr(Attribute::Bold)
    }
    #[inline]
    pub const fn dim(self) -> StyledObject<D> {
        self.attr(Attribute::Dim)
    }
    #[inline]
    pub const fn italic(self) -> StyledObject<D> {
        self.attr(Attribute::Italic)
    }
    #[inline]
    pub const fn underlined(self) -> StyledObject<D> {
        self.attr(Attribute::Underlined)
    }
    #[inline]
    pub const fn blink(self) -> StyledObject<D> {
        self.attr(Attribute::Blink)
    }
    #[inline]
    pub const fn blink_fast(self) -> StyledObject<D> {
        self.attr(Attribute::BlinkFast)
    }
    #[inline]
    pub const fn reverse(self) -> StyledObject<D> {
        self.attr(Attribute::Reverse)
    }
    #[inline]
    pub const fn hidden(self) -> StyledObject<D> {
        self.attr(Attribute::Hidden)
    }
    #[inline]
    pub const fn strikethrough(self) -> StyledObject<D> {
        self.attr(Attribute::StrikeThrough)
    }
}

macro_rules! impl_fmt {
    ($name:ident) => {
        impl<D: fmt::$name> fmt::$name for StyledObject<D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut reset = false;
                if self
                    .style
                    .force
                    .unwrap_or_else(|| match self.style.for_stderr {
                        true => colors_enabled_stderr(),
                        false => colors_enabled(),
                    })
                {
                    if let Some(fg) = self.style.fg {
                        if let Color::TrueColor(r, g, b) = fg {
                            write!(f, "\x1b[38;2;{};{};{}m", r, g, b)?;
                        } else if fg.is_color256() {
                            write!(f, "\x1b[38;5;{}m", fg.ansi_num())?;
                        } else if self.style.fg_bright {
                            write!(f, "\x1b[38;5;{}m", fg.ansi_num() + 8)?;
                        } else {
                            write!(f, "\x1b[{}m", fg.ansi_num() + 30)?;
                        }
                        reset = true;
                    }
                    if let Some(bg) = self.style.bg {
                        if let Color::TrueColor(r, g, b) = bg {
                            write!(f, "\x1b[48;2;{};{};{}m", r, g, b)?;
                        } else if bg.is_color256() {
                            write!(f, "\x1b[48;5;{}m", bg.ansi_num())?;
                        } else if self.style.bg_bright {
                            write!(f, "\x1b[48;5;{}m", bg.ansi_num() + 8)?;
                        } else {
                            write!(f, "\x1b[{}m", bg.ansi_num() + 40)?;
                        }
                        reset = true;
                    }
                    if !self.style.attrs.is_empty() {
                        write!(f, "{}", self.style.attrs)?;
                        reset = true;
                    }
                }
                fmt::$name::fmt(&self.val, f)?;
                if reset {
                    write!(f, "\x1b[0m")?;
                }
                Ok(())
            }
        }
    };
}

impl_fmt!(Binary);
impl_fmt!(Debug);
impl_fmt!(Display);
impl_fmt!(LowerExp);
impl_fmt!(LowerHex);
impl_fmt!(Octal);
impl_fmt!(Pointer);
impl_fmt!(UpperExp);
impl_fmt!(UpperHex);

/// "Intelligent" emoji formatter.
///
/// This struct intelligently wraps an emoji so that it is rendered
/// only on systems that want emojis and renders a fallback on others.
///
/// Example:
///
/// ```rust
/// use console::Emoji;
/// println!("[3/4] {}Downloading ...", Emoji("🚚 ", ""));
/// println!("[4/4] {} Done!", Emoji("✨", ":-)"));
/// ```
#[derive(Copy, Clone)]
pub struct Emoji<'a, 'b>(pub &'a str, pub &'b str);

impl<'a, 'b> Emoji<'a, 'b> {
    pub fn new(emoji: &'a str, fallback: &'b str) -> Emoji<'a, 'b> {
        Emoji(emoji, fallback)
    }
}

impl fmt::Display for Emoji<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if wants_emoji() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}", self.1)
        }
    }
}

fn str_width(s: &str) -> usize {
    #[cfg(feature = "unicode-width")]
    {
        use unicode_width::UnicodeWidthStr;
        s.width()
    }
    #[cfg(not(feature = "unicode-width"))]
    {
        s.chars().count()
    }
}

#[cfg(feature = "ansi-parsing")]
pub(crate) fn char_width(c: char) -> usize {
    #[cfg(feature = "unicode-width")]
    {
        use unicode_width::UnicodeWidthChar;
        c.width().unwrap_or(0)
    }
    #[cfg(not(feature = "unicode-width"))]
    {
        let _c = c;
        1
    }
}

#[cfg(not(feature = "ansi-parsing"))]
pub(crate) fn char_width(_c: char) -> usize {
    1
}

/// Truncates a string to a certain number of characters.
///
/// This ensures that escape codes are not screwed up in the process.
/// If the maximum length is hit the string will be truncated but
/// escapes code will still be honored.  If truncation takes place
/// the tail string will be appended.
pub fn truncate_str<'a>(s: &'a str, width: usize, tail: &str) -> Cow<'a, str> {
    if measure_text_width(s) <= width {
        return Cow::Borrowed(s);
    }

    #[cfg(feature = "ansi-parsing")]
    {
        use core::cmp::Ordering;
        let mut iter = AnsiCodeIterator::new(s);
        let mut length = 0;
        let mut rv = None;

        while let Some(item) = iter.next() {
            match item {
                (s, false) => {
                    if rv.is_none() {
                        if str_width(s) + length > width.saturating_sub(str_width(tail)) {
                            let ts = iter.current_slice();

                            let mut s_byte = 0;
                            let mut s_width = 0;
                            let rest_width =
                                width.saturating_sub(str_width(tail)).saturating_sub(length);
                            for c in s.chars() {
                                s_byte += c.len_utf8();
                                s_width += char_width(c);
                                match s_width.cmp(&rest_width) {
                                    Ordering::Equal => break,
                                    Ordering::Greater => {
                                        s_byte -= c.len_utf8();
                                        break;
                                    }
                                    Ordering::Less => continue,
                                }
                            }

                            let idx = ts.len() - s.len() + s_byte;
                            let mut buf = ts[..idx].to_string();
                            buf.push_str(tail);
                            rv = Some(buf);
                        }
                        length += str_width(s);
                    }
                }
                (s, true) => {
                    if let Some(ref mut rv) = rv {
                        rv.push_str(s);
                    }
                }
            }
        }

        if let Some(buf) = rv {
            Cow::Owned(buf)
        } else {
            Cow::Borrowed(s)
        }
    }

    #[cfg(not(feature = "ansi-parsing"))]
    {
        Cow::Owned(format!(
            "{}{}",
            &s[..width.saturating_sub(tail.len())],
            tail
        ))
    }
}

/// Pads a string to fill a certain number of characters.
///
/// This will honor ansi codes correctly and allows you to align a string
/// on the left, right or centered.  Additionally truncation can be enabled
/// by setting `truncate` to a string that should be used as a truncation
/// marker.
pub fn pad_str<'a>(
    s: &'a str,
    width: usize,
    align: Alignment,
    truncate: Option<&str>,
) -> Cow<'a, str> {
    pad_str_with(s, width, align, truncate, ' ')
}
/// Pads a string with specific padding to fill a certain number of characters.
///
/// This will honor ansi codes correctly and allows you to align a string
/// on the left, right or centered.  Additionally truncation can be enabled
/// by setting `truncate` to a string that should be used as a truncation
/// marker.
pub fn pad_str_with<'a>(
    s: &'a str,
    width: usize,
    align: Alignment,
    truncate: Option<&str>,
    pad: char,
) -> Cow<'a, str> {
    let cols = measure_text_width(s);

    if cols >= width {
        return match truncate {
            None => Cow::Borrowed(s),
            Some(tail) => truncate_str(s, width, tail),
        };
    }

    let diff = width - cols;

    let (left_pad, right_pad) = match align {
        Alignment::Left => (0, diff),
        Alignment::Right => (diff, 0),
        Alignment::Center => (diff / 2, diff - diff / 2),
    };

    let mut rv = String::new();
    for _ in 0..left_pad {
        rv.push(pad);
    }
    rv.push_str(s);
    for _ in 0..right_pad {
        rv.push(pad);
    }
    Cow::Owned(rv)
}

#[test]
fn test_text_width() {
    let s = style("foo")
        .red()
        .on_black()
        .bold()
        .force_styling(true)
        .to_string();

    assert_eq!(
        measure_text_width(&s),
        if cfg!(feature = "ansi-parsing") {
            3
        } else {
            21
        }
    );

    let s = style("🐶 <3").red().force_styling(true).to_string();

    assert_eq!(
        measure_text_width(&s),
        match (
            cfg!(feature = "ansi-parsing"),
            cfg!(feature = "unicode-width")
        ) {
            (true, true) => 5,    // "🐶 <3"
            (true, false) => 4,   // "🐶 <3", no unicode-aware width
            (false, true) => 14,  // full string
            (false, false) => 13, // full string, no unicode-aware width
        }
    );
}

#[test]
#[cfg(all(feature = "unicode-width", feature = "ansi-parsing"))]
fn test_truncate_str() {
    let s = format!("foo {}", style("bar").red().force_styling(true));
    assert_eq!(
        &truncate_str(&s, 5, ""),
        &format!("foo {}", style("b").red().force_styling(true))
    );
    let s = format!("foo {}", style("bar").red().force_styling(true));
    assert_eq!(
        &truncate_str(&s, 5, "!"),
        &format!("foo {}", style("!").red().force_styling(true))
    );
    let s = format!("foo {} baz", style("bar").red().force_styling(true));
    assert_eq!(
        &truncate_str(&s, 10, "..."),
        &format!("foo {}...", style("bar").red().force_styling(true))
    );
    let s = format!("foo {}", style("バー").red().force_styling(true));
    assert_eq!(
        &truncate_str(&s, 5, ""),
        &format!("foo {}", style("").red().force_styling(true))
    );
    let s = format!("foo {}", style("バー").red().force_styling(true));
    assert_eq!(
        &truncate_str(&s, 6, ""),
        &format!("foo {}", style("バ").red().force_styling(true))
    );
    let s = format!("foo {}", style("バー").red().force_styling(true));
    assert_eq!(
        &truncate_str(&s, 2, "!!!"),
        &format!("!!!{}", style("").red().force_styling(true))
    );
}

#[test]
fn test_truncate_str_no_ansi() {
    assert_eq!(&truncate_str("foo bar", 7, "!"), "foo bar");
    assert_eq!(&truncate_str("foo bar", 5, ""), "foo b");
    assert_eq!(&truncate_str("foo bar", 5, "!"), "foo !");
    assert_eq!(&truncate_str("foo bar baz", 10, "..."), "foo bar...");
    assert_eq!(&truncate_str("foo bar", 0, ""), "");
    assert_eq!(&truncate_str("foo bar", 0, "!"), "!");
    assert_eq!(&truncate_str("foo bar", 2, "!!!"), "!!!");
    assert_eq!(&truncate_str("ab", 2, "!!!"), "ab");
}

#[test]
fn test_pad_str() {
    assert_eq!(pad_str("foo", 7, Alignment::Center, None), "  foo  ");
    assert_eq!(pad_str("foo", 7, Alignment::Left, None), "foo    ");
    assert_eq!(pad_str("foo", 7, Alignment::Right, None), "    foo");
    assert_eq!(pad_str("foo", 3, Alignment::Left, None), "foo");
    assert_eq!(pad_str("foobar", 3, Alignment::Left, None), "foobar");
    assert_eq!(pad_str("foobar", 3, Alignment::Left, Some("")), "foo");
    assert_eq!(
        pad_str("foobarbaz", 6, Alignment::Left, Some("...")),
        "foo..."
    );
}

#[test]
fn test_pad_str_with() {
    assert_eq!(
        pad_str_with("foo", 7, Alignment::Center, None, '#'),
        "##foo##"
    );
    assert_eq!(
        pad_str_with("foo", 7, Alignment::Left, None, '#'),
        "foo####"
    );
    assert_eq!(
        pad_str_with("foo", 7, Alignment::Right, None, '#'),
        "####foo"
    );
    assert_eq!(pad_str_with("foo", 3, Alignment::Left, None, '#'), "foo");
    assert_eq!(
        pad_str_with("foobar", 3, Alignment::Left, None, '#'),
        "foobar"
    );
    assert_eq!(
        pad_str_with("foobar", 3, Alignment::Left, Some(""), '#'),
        "foo"
    );
    assert_eq!(
        pad_str_with("foobarbaz", 6, Alignment::Left, Some("..."), '#'),
        "foo..."
    );
}

#[test]
fn test_attributes_single() {
    for attr in Attribute::MAP {
        let attrs = Attributes::new().insert(attr);
        assert_eq!(attrs.bits().collect::<Vec<_>>(), [attr as u16]);
        assert_eq!(attrs.attrs().collect::<Vec<_>>(), [attr]);
        assert_eq!(format!("{attrs:?}"), format!("{{{:?}}}", attr));
    }
}

#[test]
fn test_attributes_many() {
    let tests: [&[Attribute]; 3] = [
        &[
            Attribute::Bold,
            Attribute::Underlined,
            Attribute::BlinkFast,
            Attribute::Hidden,
        ],
        &[
            Attribute::Dim,
            Attribute::Italic,
            Attribute::Blink,
            Attribute::Reverse,
            Attribute::StrikeThrough,
        ],
        &Attribute::MAP,
    ];
    for test_attrs in tests {
        let mut attrs = Attributes::new();
        for attr in test_attrs {
            attrs = attrs.insert(*attr);
        }
        assert_eq!(
            attrs.bits().collect::<Vec<_>>(),
            test_attrs
                .iter()
                .map(|attr| *attr as u16)
                .collect::<Vec<_>>()
        );
        assert_eq!(&attrs.attrs().collect::<Vec<_>>(), test_attrs);
    }
}
