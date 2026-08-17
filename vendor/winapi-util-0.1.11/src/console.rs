use std::{io, mem};

use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{GetConsoleMode, SetConsoleMode};
use windows_sys::Win32::System::Console::{
    GetConsoleScreenBufferInfo, SetConsoleTextAttribute,
    CONSOLE_SCREEN_BUFFER_INFO, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    FOREGROUND_BLUE, FOREGROUND_GREEN, FOREGROUND_INTENSITY, FOREGROUND_RED,
};

use crate::{AsHandleRef, HandleRef};

use FOREGROUND_BLUE as FG_BLUE;
use FOREGROUND_GREEN as FG_GREEN;
use FOREGROUND_INTENSITY as FG_INTENSITY;
use FOREGROUND_RED as FG_RED;

const FG_CYAN: u16 = FG_BLUE | FG_GREEN;
const FG_MAGENTA: u16 = FG_BLUE | FG_RED;
const FG_YELLOW: u16 = FG_GREEN | FG_RED;
const FG_WHITE: u16 = FG_BLUE | FG_GREEN | FG_RED;

/// Query the given handle for information about the console's screen buffer.
///
/// The given handle should represent a console. Otherwise, an error is
/// returned.
///
/// This corresponds to calling [`GetConsoleScreenBufferInfo`].
///
/// [`GetConsoleScreenBufferInfo`]: https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfo
pub fn screen_buffer_info<H: AsHandleRef>(
    h: H,
) -> io::Result<ScreenBufferInfo> {
    unsafe {
        let mut info: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
        let rc = GetConsoleScreenBufferInfo(h.as_raw() as HANDLE, &mut info);
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
pub fn set_text_attributes<H: AsHandleRef>(
    h: H,
    attributes: u16,
) -> io::Result<()> {
    if unsafe { SetConsoleTextAttribute(h.as_raw() as HANDLE, attributes) }
        == 0
    {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Query the mode of the console represented by the given handle.
///
/// This corresponds to calling [`GetConsoleMode`], which describes the return
/// value.
///
/// [`GetConsoleMode`]: https://docs.microsoft.com/en-us/windows/console/getconsolemode
pub fn mode<H: AsHandleRef>(h: H) -> io::Result<u32> {
    let mut mode = 0;
    if unsafe { GetConsoleMode(h.as_raw() as HANDLE, &mut mode) } == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(mode)
    }
}

/// Set the mode of the console represented by the given handle.
///
/// This corresponds to calling [`SetConsoleMode`], which describes the format
/// of the mode parameter.
///
/// [`SetConsoleMode`]: https://docs.microsoft.com/en-us/windows/console/setconsolemode
pub fn set_mode<H: AsHandleRef>(h: H, mode: u32) -> io::Result<()> {
    if unsafe { SetConsoleMode(h.as_raw() as HANDLE, mode) } == 0 {
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
pub struct ScreenBufferInfo(CONSOLE_SCREEN_BUFFER_INFO);

impl ScreenBufferInfo {
    /// Returns the size of the console screen buffer, in character columns and
    /// rows.
    ///
    /// This corresponds to `dwSize`.
    pub fn size(&self) -> (i16, i16) {
        (self.0.dwSize.X, self.0.dwSize.Y)
    }

    /// Returns the position of the cursor in terms of column and row
    /// coordinates of the console screen buffer.
    ///
    /// This corresponds to `dwCursorPosition`.
    pub fn cursor_position(&self) -> (i16, i16) {
        (self.0.dwCursorPosition.X, self.0.dwCursorPosition.Y)
    }

    /// Returns the character attributes associated with this console.
    ///
    /// This corresponds to `wAttributes`.
    ///
    /// See [`char info`] for more details.
    ///
    /// [`char info`]: https://docs.microsoft.com/en-us/windows/console/char-info-str
    pub fn attributes(&self) -> u16 {
        self.0.wAttributes
    }

    /// Returns the maximum size of the console window, in character columns
    /// and rows, given the current screen buffer size and font and the screen
    /// size.
    pub fn max_window_size(&self) -> (i16, i16) {
        (self.0.dwMaximumWindowSize.X, self.0.dwMaximumWindowSize.Y)
    }

    /// Returns the console screen buffer coordinates of the upper-left and
    /// lower-right corners of the display window.
    ///
    /// This corresponds to `srWindow`.
    pub fn window_rect(&self) -> SmallRect {
        SmallRect {
            left: self.0.srWindow.Left,
            top: self.0.srWindow.Top,
            right: self.0.srWindow.Right,
            bottom: self.0.srWindow.Bottom,
        }
    }
}

/// Defines the coordinates of the upper left and lower right corners of a rectangle.
///
/// This corresponds to [`SMALL_RECT`].
///
/// [`SMALL_RECT`]: https://docs.microsoft.com/en-us/windows/console/small-rect-str
pub struct SmallRect {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
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
///
/// # Example
/// ```no_run
/// # #[cfg(windows)]
/// # {
/// use winapi_util::console::{Console, Color, Intense};
///
/// let mut con = Console::stdout().unwrap();
/// con.fg(Intense::Yes, Color::Cyan).unwrap();
/// println!("This text will be intense cyan.");
/// con.reset().unwrap();
/// println!("This text will be normal.");
/// # }
/// ```
#[derive(Debug)]
pub struct Console {
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
    fn handle(&self) -> HandleRef {
        match *self {
            HandleKind::Stdout => HandleRef::stdout(),
            HandleKind::Stderr => HandleRef::stderr(),
        }
    }
}

impl Console {
    /// Get a console for a standard I/O stream.
    fn create_for_stream(kind: HandleKind) -> io::Result<Console> {
        let h = kind.handle();
        let info = screen_buffer_info(&h)?;
        let attr = TextAttributes::from_word(info.attributes());
        Ok(Console { kind, start_attr: attr, cur_attr: attr })
    }

    /// Create a new Console to stdout.
    ///
    /// If there was a problem creating the console, then an error is returned.
    pub fn stdout() -> io::Result<Console> {
        Self::create_for_stream(HandleKind::Stdout)
    }

    /// Create a new Console to stderr.
    ///
    /// If there was a problem creating the console, then an error is returned.
    pub fn stderr() -> io::Result<Console> {
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
    pub fn fg(&mut self, intense: Intense, color: Color) -> io::Result<()> {
        self.cur_attr.fg_color = color;
        self.cur_attr.fg_intense = intense;
        self.set()
    }

    /// Apply the given intensity and color attributes to the console
    /// background.
    ///
    /// If there was a problem setting attributes on the console, then an error
    /// is returned.
    pub fn bg(&mut self, intense: Intense, color: Color) -> io::Result<()> {
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
    pub fn reset(&mut self) -> io::Result<()> {
        self.cur_attr = self.start_attr;
        self.set()
    }

    /// Toggle virtual terminal processing.
    ///
    /// This method attempts to toggle virtual terminal processing for this
    /// console. If there was a problem toggling it, then an error returned.
    /// On success, the caller may assume that toggling it was successful.
    ///
    /// When virtual terminal processing is enabled, characters emitted to the
    /// console are parsed for VT100 and similar control character sequences
    /// that control color and other similar operations.
    pub fn set_virtual_terminal_processing(
        &mut self,
        yes: bool,
    ) -> io::Result<()> {
        let vt = ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        let handle = self.kind.handle();
        let old_mode = mode(&handle)?;
        let new_mode = if yes { old_mode | vt } else { old_mode & !vt };
        if old_mode == new_mode {
            return Ok(());
        }
        set_mode(&handle, new_mode)
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
    fn to_word(&self) -> u16 {
        let mut w = 0;
        w |= self.fg_color.to_fg();
        w |= self.fg_intense.to_fg();
        w |= self.bg_color.to_bg();
        w |= self.bg_intense.to_bg();
        w
    }

    fn from_word(word: u16) -> TextAttributes {
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
pub enum Intense {
    Yes,
    No,
}

impl Intense {
    fn to_bg(&self) -> u16 {
        self.to_fg() << 4
    }

    fn from_bg(word: u16) -> Intense {
        Intense::from_fg(word >> 4)
    }

    fn to_fg(&self) -> u16 {
        match *self {
            Intense::No => 0,
            Intense::Yes => FG_INTENSITY,
        }
    }

    fn from_fg(word: u16) -> Intense {
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
pub enum Color {
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
    fn to_bg(&self) -> u16 {
        self.to_fg() << 4
    }

    fn from_bg(word: u16) -> Color {
        Color::from_fg(word >> 4)
    }

    fn to_fg(&self) -> u16 {
        match *self {
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

    fn from_fg(word: u16) -> Color {
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
