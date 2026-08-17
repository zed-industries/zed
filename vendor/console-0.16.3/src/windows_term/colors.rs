use crate::ansi::AnsiCodeIterator;
use core::mem;
use core::str::{from_utf8, Bytes};
use std::io;
use std::os::windows::io::AsRawHandle;

use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{
    GetConsoleScreenBufferInfo, SetConsoleTextAttribute, CONSOLE_SCREEN_BUFFER_INFO,
    FOREGROUND_BLUE as FG_BLUE, FOREGROUND_GREEN as FG_GREEN, FOREGROUND_INTENSITY as FG_INTENSITY,
    FOREGROUND_RED as FG_RED,
};

use crate::Term;

#[allow(clippy::upper_case_acronyms)]
type WORD = u16;

const FG_CYAN: WORD = FG_BLUE | FG_GREEN;
const FG_MAGENTA: WORD = FG_BLUE | FG_RED;
const FG_YELLOW: WORD = FG_GREEN | FG_RED;
const FG_WHITE: WORD = FG_BLUE | FG_GREEN | FG_RED;

/// Query the given handle for information about the console's screen buffer.
///
/// The given handle should represent a console. Otherwise, an error is
/// returned.
///
/// This corresponds to calling [`GetConsoleScreenBufferInfo`].
///
/// [`GetConsoleScreenBufferInfo`]: https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfo
pub(crate) fn screen_buffer_info(h: HANDLE) -> io::Result<ScreenBufferInfo> {
    unsafe {
        let mut info: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
        let rc = GetConsoleScreenBufferInfo(h, &mut info);
        if rc == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(ScreenBufferInfo(info))
    }
}

/// Set the text attributes of the console represented by the given handle.
///
/// This corresponds to calling [`SetConsoleTextAttribute`].
///
/// [`SetConsoleTextAttribute`]: https://docs.microsoft.com/en-us/windows/console/setconsoletextattribute
pub(crate) fn set_text_attributes(h: HANDLE, attributes: u16) -> io::Result<()> {
    if unsafe { SetConsoleTextAttribute(h, attributes) } == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Represents console screen buffer information such as size, cursor position
/// and styling attributes.
///
/// This wraps a [`CONSOLE_SCREEN_BUFFER_INFO`].
///
/// [`CONSOLE_SCREEN_BUFFER_INFO`]: https://docs.microsoft.com/en-us/windows/console/console-screen-buffer-info-str
#[derive(Clone)]
pub(crate) struct ScreenBufferInfo(CONSOLE_SCREEN_BUFFER_INFO);

impl ScreenBufferInfo {
    /// Returns the character attributes associated with this console.
    ///
    /// This corresponds to `wAttributes`.
    ///
    /// See [`char info`] for more details.
    ///
    /// [`char info`]: https://docs.microsoft.com/en-us/windows/console/char-info-str
    pub(crate) fn attributes(&self) -> u16 {
        self.0.wAttributes
    }
}

/// A Windows console.
///
/// This represents a very limited set of functionality available to a Windows
/// console. In particular, it can only change text attributes such as color
/// and intensity. This may grow over time. If you need more routines, please
/// file an issue and/or PR.
///
/// There is no way to "write" to this console. Simply write to
/// stdout or stderr instead, while interleaving instructions to the console
/// to change text attributes.
///
/// A common pitfall when using a console is to forget to flush writes to
/// stdout before setting new text attributes.
#[derive(Debug)]
pub(crate) struct Console {
    kind: HandleKind,
    start_attr: TextAttributes,
    cur_attr: TextAttributes,
}

#[derive(Clone, Copy, Debug)]
enum HandleKind {
    Stdout,
    Stderr,
}

impl HandleKind {
    fn handle(&self) -> HANDLE {
        match *self {
            HandleKind::Stdout => io::stdout().as_raw_handle() as HANDLE,
            HandleKind::Stderr => io::stderr().as_raw_handle() as HANDLE,
        }
    }
}

impl Console {
    /// Get a console for a standard I/O stream.
    fn create_for_stream(kind: HandleKind) -> io::Result<Console> {
        let h = kind.handle();
        let info = screen_buffer_info(h)?;
        let attr = TextAttributes::from_word(info.attributes());
        Ok(Console {
            kind,
            start_attr: attr,
            cur_attr: attr,
        })
    }

    /// Create a new Console to stdout.
    ///
    /// If there was a problem creating the console, then an error is returned.
    pub(crate) fn stdout() -> io::Result<Console> {
        Self::create_for_stream(HandleKind::Stdout)
    }

    /// Create a new Console to stderr.
    ///
    /// If there was a problem creating the console, then an error is returned.
    pub(crate) fn stderr() -> io::Result<Console> {
        Self::create_for_stream(HandleKind::Stderr)
    }

    /// Applies the current text attributes.
    fn set(&mut self) -> io::Result<()> {
        set_text_attributes(self.kind.handle(), self.cur_attr.to_word())
    }

    /// Apply the given intensity and color attributes to the console
    /// foreground.
    ///
    /// If there was a problem setting attributes on the console, then an error
    /// is returned.
    pub(crate) fn fg(&mut self, intense: Intense, color: Color) -> io::Result<()> {
        self.cur_attr.fg_color = color;
        self.cur_attr.fg_intense = intense;
        self.set()
    }

    /// Apply the given intensity and color attributes to the console
    /// background.
    ///
    /// If there was a problem setting attributes on the console, then an error
    /// is returned.
    pub(crate) fn bg(&mut self, intense: Intense, color: Color) -> io::Result<()> {
        self.cur_attr.bg_color = color;
        self.cur_attr.bg_intense = intense;
        self.set()
    }

    /// Reset the console text attributes to their original settings.
    ///
    /// The original settings correspond to the text attributes on the console
    /// when this `Console` value was created.
    ///
    /// If there was a problem setting attributes on the console, then an error
    /// is returned.
    pub(crate) fn reset(&mut self) -> io::Result<()> {
        self.cur_attr = self.start_attr;
        self.set()
    }
}

/// A representation of text attributes for the Windows console.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct TextAttributes {
    fg_color: Color,
    fg_intense: Intense,
    bg_color: Color,
    bg_intense: Intense,
}

impl TextAttributes {
    fn to_word(self) -> WORD {
        let mut w = 0;
        w |= self.fg_color.to_fg();
        w |= self.fg_intense.to_fg();
        w |= self.bg_color.to_bg();
        w |= self.bg_intense.to_bg();
        w
    }

    fn from_word(word: WORD) -> TextAttributes {
        TextAttributes {
            fg_color: Color::from_fg(word),
            fg_intense: Intense::from_fg(word),
            bg_color: Color::from_bg(word),
            bg_intense: Intense::from_bg(word),
        }
    }
}

/// Whether to use intense colors or not.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Intense {
    Yes,
    No,
}

impl Intense {
    fn to_bg(self) -> WORD {
        self.to_fg() << 4
    }

    fn from_bg(word: WORD) -> Intense {
        Intense::from_fg(word >> 4)
    }

    fn to_fg(self) -> WORD {
        match self {
            Intense::No => 0,
            Intense::Yes => FG_INTENSITY,
        }
    }

    fn from_fg(word: WORD) -> Intense {
        if word & FG_INTENSITY > 0 {
            Intense::Yes
        } else {
            Intense::No
        }
    }
}

/// The set of available colors for use with a Windows console.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
}

impl Color {
    fn to_bg(self) -> WORD {
        self.to_fg() << 4
    }

    fn from_bg(word: WORD) -> Color {
        Color::from_fg(word >> 4)
    }

    fn to_fg(self) -> WORD {
        match self {
            Color::Black => 0,
            Color::Blue => FG_BLUE,
            Color::Green => FG_GREEN,
            Color::Red => FG_RED,
            Color::Cyan => FG_CYAN,
            Color::Magenta => FG_MAGENTA,
            Color::Yellow => FG_YELLOW,
            Color::White => FG_WHITE,
        }
    }

    fn from_fg(word: WORD) -> Color {
        match word & 0b111 {
            FG_BLUE => Color::Blue,
            FG_GREEN => Color::Green,
            FG_RED => Color::Red,
            FG_CYAN => Color::Cyan,
            FG_MAGENTA => Color::Magenta,
            FG_YELLOW => Color::Yellow,
            FG_WHITE => Color::White,
            _ => Color::Black,
        }
    }
}

pub(crate) fn console_colors(out: &Term, mut con: Console, bytes: &[u8]) -> io::Result<()> {
    let s = from_utf8(bytes).expect("data to be printed is not an ansi string");
    let mut iter = AnsiCodeIterator::new(s);

    while !iter.rest_slice().is_empty() {
        if let Some((part, is_esc)) = iter.next() {
            if !is_esc {
                out.write_through_common(part.as_bytes())?;
            } else if part == "\x1b[0m" {
                con.reset()?;
            } else if let Some((intense, color, fg_bg)) = driver(parse_color, part) {
                match fg_bg {
                    FgBg::Foreground => con.fg(intense, color),
                    FgBg::Background => con.bg(intense, color),
                }?;
            } else if driver(parse_attr, part).is_none() {
                out.write_through_common(part.as_bytes())?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum FgBg {
    Foreground,
    Background,
}

impl FgBg {
    fn new(byte: u8) -> Option<Self> {
        match byte {
            b'3' => Some(Self::Foreground),
            b'4' => Some(Self::Background),
            _ => None,
        }
    }
}

fn driver<Out>(parse: fn(Bytes<'_>) -> Option<Out>, part: &str) -> Option<Out> {
    let mut bytes = part.bytes();

    loop {
        while bytes.next()? != b'\x1b' {}

        if let ret @ Some(_) = (parse)(bytes.clone()) {
            return ret;
        }
    }
}

// `driver(parse_color, s)` parses the equivalent of the regex
// \x1b\[(3|4)8;5;(8|9|1[0-5])m
// for intense or
// \x1b\[(3|4)([0-7])m
// for normal
fn parse_color(mut bytes: Bytes<'_>) -> Option<(Intense, Color, FgBg)> {
    parse_prefix(&mut bytes)?;

    let fg_bg = FgBg::new(bytes.next()?)?;
    let (intense, color) = match bytes.next()? {
        b @ b'0'..=b'7' => (Intense::No, normal_color_ansi_from_byte(b)?),
        b'8' => {
            if &[bytes.next()?, bytes.next()?, bytes.next()?] != b";5;" {
                return None;
            }
            (Intense::Yes, parse_intense_color_ansi(&mut bytes)?)
        }
        _ => return None,
    };

    parse_suffix(&mut bytes)?;
    Some((intense, color, fg_bg))
}

// `driver(parse_attr, s)` parses the equivalent of the regex
// \x1b\[([1-8])m
fn parse_attr(mut bytes: Bytes<'_>) -> Option<u8> {
    parse_prefix(&mut bytes)?;
    let attr = match bytes.next()? {
        attr @ b'1'..=b'8' => attr,
        _ => return None,
    };
    parse_suffix(&mut bytes)?;
    Some(attr)
}

fn parse_prefix(bytes: &mut Bytes<'_>) -> Option<()> {
    if bytes.next()? == b'[' {
        Some(())
    } else {
        None
    }
}

fn parse_intense_color_ansi(bytes: &mut Bytes<'_>) -> Option<Color> {
    let color = match bytes.next()? {
        b'8' => Color::Black,
        b'9' => Color::Red,
        b'1' => match bytes.next()? {
            b'0' => Color::Green,
            b'1' => Color::Yellow,
            b'2' => Color::Blue,
            b'3' => Color::Magenta,
            b'4' => Color::Cyan,
            b'5' => Color::White,
            _ => return None,
        },
        _ => return None,
    };
    Some(color)
}

fn normal_color_ansi_from_byte(b: u8) -> Option<Color> {
    let color = match b {
        b'0' => Color::Black,
        b'1' => Color::Red,
        b'2' => Color::Green,
        b'3' => Color::Yellow,
        b'4' => Color::Blue,
        b'5' => Color::Magenta,
        b'6' => Color::Cyan,
        b'7' => Color::White,
        _ => return None,
    };
    Some(color)
}

fn parse_suffix(bytes: &mut Bytes<'_>) -> Option<()> {
    if bytes.next()? == b'm' {
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_parsing() {
        let intense_color = "leading bytes \x1b[38;5;10m trailing bytes";
        let parsed = driver(parse_color, intense_color).unwrap();
        assert_eq!(parsed, (Intense::Yes, Color::Green, FgBg::Foreground));

        let normal_color = "leading bytes \x1b[40m trailing bytes";
        let parsed = driver(parse_color, normal_color).unwrap();
        assert_eq!(parsed, (Intense::No, Color::Black, FgBg::Background));
    }

    #[test]
    fn attr_parsing() {
        let attr = "leading bytes \x1b[1m trailing bytes";
        let parsed = driver(parse_attr, attr).unwrap();
        assert_eq!(parsed, b'1');
    }
}
