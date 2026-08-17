use alloc::sync::Arc;
use core::fmt::{Debug, Display};
use std::io::{self, Read, Write};
#[cfg(any(unix, all(target_os = "wasi", target_env = "p1")))]
use std::os::fd::{AsRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, RawHandle};
use std::sync::{Mutex, RwLock};

use crate::{kb::Key, utils::Style};

#[cfg(unix)]
trait TermWrite: Write + Debug + AsRawFd + Send {}
#[cfg(unix)]
impl<T: Write + Debug + AsRawFd + Send> TermWrite for T {}

#[cfg(unix)]
trait TermRead: Read + Debug + AsRawFd + Send {}
#[cfg(unix)]
impl<T: Read + Debug + AsRawFd + Send> TermRead for T {}

#[cfg(unix)]
#[derive(Debug, Clone)]
pub struct ReadWritePair {
    #[allow(unused)]
    read: Arc<Mutex<dyn TermRead>>,
    write: Arc<Mutex<dyn TermWrite>>,
    style: Style,
}

/// Where the term is writing.
#[derive(Debug, Clone)]
pub enum TermTarget {
    Stdout,
    Stderr,
    #[cfg(unix)]
    ReadWritePair(ReadWritePair),
}

#[derive(Debug)]
struct TermInner {
    target: TermTarget,
    buffer: Option<Mutex<Vec<u8>>>,
    prompt: RwLock<String>,
    prompt_guard: Mutex<()>,
}

/// The family of the terminal.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TermFamily {
    /// Redirected to a file or file like thing.
    File,
    /// A standard unix terminal.
    UnixTerm,
    /// A cmd.exe like windows console.
    WindowsConsole,
    /// A dummy terminal (for instance on wasm)
    Dummy,
}

/// Gives access to the terminal features.
#[derive(Debug, Clone)]
pub struct TermFeatures<'a>(&'a Term);

impl TermFeatures<'_> {
    /// Check if this is a real user attended terminal (`isatty`)
    #[inline]
    pub fn is_attended(&self) -> bool {
        is_a_terminal(self.0)
    }

    /// Check if colors are supported by this terminal.
    ///
    /// This does not check if colors are enabled.  Currently all terminals
    /// are considered to support colors
    #[inline]
    pub fn colors_supported(&self) -> bool {
        is_a_color_terminal(self.0)
    }

    /// Check if true colors are supported by this terminal.
    pub fn true_colors_supported(&self) -> bool {
        is_a_true_color_terminal(self.0)
    }

    /// Check if this terminal is an msys terminal.
    ///
    /// This is sometimes useful to disable features that are known to not
    /// work on msys terminals or require special handling.
    #[inline]
    pub fn is_msys_tty(&self) -> bool {
        #[cfg(windows)]
        {
            msys_tty_on(self.0)
        }
        #[cfg(not(windows))]
        {
            false
        }
    }

    /// Check if this terminal wants emojis.
    #[inline]
    pub fn wants_emoji(&self) -> bool {
        self.is_attended() && wants_emoji()
    }

    /// Return the family of the terminal.
    #[inline]
    pub fn family(&self) -> TermFamily {
        if !self.is_attended() {
            return TermFamily::File;
        }
        #[cfg(windows)]
        {
            TermFamily::WindowsConsole
        }
        #[cfg(all(unix, not(target_arch = "wasm32")))]
        {
            TermFamily::UnixTerm
        }
        #[cfg(target_arch = "wasm32")]
        {
            TermFamily::Dummy
        }
    }
}

/// Abstraction around a terminal.
///
/// A terminal can be cloned.  If a buffer is used it's shared across all
/// clones which means it largely acts as a handle.
#[derive(Clone, Debug)]
pub struct Term {
    inner: Arc<TermInner>,
    pub(crate) is_msys_tty: bool,
    pub(crate) is_tty: bool,
}

impl Term {
    fn with_inner(inner: TermInner) -> Term {
        let mut term = Term {
            inner: Arc::new(inner),
            is_msys_tty: false,
            is_tty: false,
        };

        term.is_msys_tty = term.features().is_msys_tty();
        term.is_tty = term.features().is_attended();
        term
    }

    /// Return a new unbuffered terminal.
    #[inline]
    pub fn stdout() -> Term {
        Term::with_inner(TermInner {
            target: TermTarget::Stdout,
            buffer: None,
            prompt: RwLock::new(String::new()),
            prompt_guard: Mutex::new(()),
        })
    }

    /// Return a new unbuffered terminal to stderr.
    #[inline]
    pub fn stderr() -> Term {
        Term::with_inner(TermInner {
            target: TermTarget::Stderr,
            buffer: None,
            prompt: RwLock::new(String::new()),
            prompt_guard: Mutex::new(()),
        })
    }

    /// Return a new buffered terminal.
    pub fn buffered_stdout() -> Term {
        Term::with_inner(TermInner {
            target: TermTarget::Stdout,
            buffer: Some(Mutex::new(vec![])),
            prompt: RwLock::new(String::new()),
            prompt_guard: Mutex::new(()),
        })
    }

    /// Return a new buffered terminal to stderr.
    pub fn buffered_stderr() -> Term {
        Term::with_inner(TermInner {
            target: TermTarget::Stderr,
            buffer: Some(Mutex::new(vec![])),
            prompt: RwLock::new(String::new()),
            prompt_guard: Mutex::new(()),
        })
    }

    /// Return a terminal for the given Read/Write pair styled like stderr.
    #[cfg(unix)]
    pub fn read_write_pair<R, W>(read: R, write: W) -> Term
    where
        R: Read + Debug + AsRawFd + Send + 'static,
        W: Write + Debug + AsRawFd + Send + 'static,
    {
        Self::read_write_pair_with_style(read, write, Style::new().for_stderr())
    }

    /// Return a terminal for the given Read/Write pair.
    #[cfg(unix)]
    pub fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term
    where
        R: Read + Debug + AsRawFd + Send + 'static,
        W: Write + Debug + AsRawFd + Send + 'static,
    {
        Term::with_inner(TermInner {
            target: TermTarget::ReadWritePair(ReadWritePair {
                read: Arc::new(Mutex::new(read)),
                write: Arc::new(Mutex::new(write)),
                style,
            }),
            buffer: None,
            prompt: RwLock::new(String::new()),
            prompt_guard: Mutex::new(()),
        })
    }

    /// Return the style for this terminal.
    #[inline]
    pub fn style(&self) -> Style {
        match self.inner.target {
            TermTarget::Stderr => Style::new().for_stderr(),
            TermTarget::Stdout => Style::new().for_stdout(),
            #[cfg(unix)]
            TermTarget::ReadWritePair(ReadWritePair { ref style, .. }) => style.clone(),
        }
    }

    /// Return the target of this terminal.
    #[inline]
    pub fn target(&self) -> TermTarget {
        self.inner.target.clone()
    }

    #[doc(hidden)]
    pub fn write_str(&self, s: &str) -> io::Result<()> {
        match self.inner.buffer {
            Some(ref buffer) => buffer.lock().unwrap().write_all(s.as_bytes()),
            None => self.write_through(s.as_bytes()),
        }
    }

    /// Write a string to the terminal and add a newline.
    pub fn write_line(&self, s: &str) -> io::Result<()> {
        let prompt = self.inner.prompt.read().unwrap();
        if !prompt.is_empty() {
            self.clear_line()?;
        }
        match self.inner.buffer {
            Some(ref mutex) => {
                let mut buffer = mutex.lock().unwrap();
                buffer.extend_from_slice(s.as_bytes());
                buffer.push(b'\n');
                buffer.extend_from_slice(prompt.as_bytes());
                Ok(())
            }
            None => self.write_through(format!("{}\n{}", s, prompt.as_str()).as_bytes()),
        }
    }

    /// Read a single character from the terminal.
    ///
    /// This does not echo the character and blocks until a single character
    /// or complete key chord is entered.  If the terminal is not user attended
    /// the return value will be an error.
    pub fn read_char(&self) -> io::Result<char> {
        if !self.is_tty {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Not a terminal",
            ));
        }
        loop {
            match self.read_key()? {
                Key::Char(c) => {
                    return Ok(c);
                }
                Key::Enter => {
                    return Ok('\n');
                }
                _ => {}
            }
        }
    }

    /// Read a single key from the terminal.
    ///
    /// This does not echo anything.  If the terminal is not user attended
    /// the return value will always be the unknown key.
    pub fn read_key(&self) -> io::Result<Key> {
        if !self.is_tty {
            Ok(Key::Unknown)
        } else {
            read_single_key(false)
        }
    }

    pub fn read_key_raw(&self) -> io::Result<Key> {
        if !self.is_tty {
            Ok(Key::Unknown)
        } else {
            read_single_key(true)
        }
    }

    /// Read one line of input.
    ///
    /// This does not include the trailing newline.  If the terminal is not
    /// user attended the return value will always be an empty string.
    pub fn read_line(&self) -> io::Result<String> {
        self.read_line_initial_text("")
    }

    /// Read one line of input with initial text.
    ///
    /// This method blocks until no other thread is waiting for this read_line
    /// before reading a line from the terminal.
    /// This does not include the trailing newline.  If the terminal is not
    /// user attended the return value will always be an empty string.
    pub fn read_line_initial_text(&self, initial: &str) -> io::Result<String> {
        if !self.is_tty {
            return Ok("".into());
        }
        *self.inner.prompt.write().unwrap() = initial.to_string();
        // use a guard in order to prevent races with other calls to read_line_initial_text
        let _guard = self.inner.prompt_guard.lock().unwrap();

        self.write_str(initial)?;

        fn read_line_internal(slf: &Term, initial: &str) -> io::Result<String> {
            let prefix_len = initial.len();

            let mut chars: Vec<char> = initial.chars().collect();

            loop {
                match slf.read_key()? {
                    Key::Backspace => {
                        if prefix_len < chars.len() {
                            if let Some(ch) = chars.pop() {
                                slf.clear_chars(crate::utils::char_width(ch))?;
                            }
                        }
                        slf.flush()?;
                    }
                    Key::Char(chr) => {
                        chars.push(chr);
                        let mut bytes_char = [0; 4];
                        chr.encode_utf8(&mut bytes_char);
                        slf.write_str(chr.encode_utf8(&mut bytes_char))?;
                        slf.flush()?;
                    }
                    Key::Enter => {
                        slf.write_through(format!("\n{initial}").as_bytes())?;
                        break;
                    }
                    _ => (),
                }
            }
            Ok(chars.iter().skip(prefix_len).collect::<String>())
        }
        let ret = read_line_internal(self, initial);

        *self.inner.prompt.write().unwrap() = String::new();
        ret
    }

    /// Read a line of input securely.
    ///
    /// This is similar to `read_line` but will not echo the output.  This
    /// also switches the terminal into a different mode where not all
    /// characters might be accepted.
    pub fn read_secure_line(&self) -> io::Result<String> {
        if !self.is_tty {
            return Ok("".into());
        }
        match read_secure() {
            Ok(rv) => {
                self.write_line("")?;
                Ok(rv)
            }
            Err(err) => Err(err),
        }
    }

    /// Flush internal buffers.
    ///
    /// This forces the contents of the internal buffer to be written to
    /// the terminal.  This is unnecessary for unbuffered terminals which
    /// will automatically flush.
    pub fn flush(&self) -> io::Result<()> {
        if let Some(ref buffer) = self.inner.buffer {
            let mut buffer = buffer.lock().unwrap();
            if !buffer.is_empty() {
                self.write_through(&buffer[..])?;
                buffer.clear();
            }
        }
        Ok(())
    }

    /// Check if the terminal is indeed a terminal.
    #[inline]
    pub fn is_term(&self) -> bool {
        self.is_tty
    }

    /// Check for common terminal features.
    #[inline]
    pub fn features(&self) -> TermFeatures<'_> {
        TermFeatures(self)
    }

    /// Return the terminal size in rows and columns or gets sensible defaults.
    #[inline]
    pub fn size(&self) -> (u16, u16) {
        self.size_checked().unwrap_or((24, DEFAULT_WIDTH))
    }

    /// Return the terminal size in rows and columns.
    ///
    /// If the size cannot be reliably determined `None` is returned.
    #[inline]
    pub fn size_checked(&self) -> Option<(u16, u16)> {
        terminal_size(self)
    }

    /// Move the cursor to row `x` and column `y`. Values are 0-based.
    #[inline]
    pub fn move_cursor_to(&self, x: usize, y: usize) -> io::Result<()> {
        move_cursor_to(self, x, y)
    }

    /// Move the cursor up by `n` lines, if possible.
    ///
    /// If there are less than `n` lines above the current cursor position,
    /// the cursor is moved to the top line of the terminal (i.e., as far up as possible).
    #[inline]
    pub fn move_cursor_up(&self, n: usize) -> io::Result<()> {
        move_cursor_up(self, n)
    }

    /// Move the cursor down by `n` lines, if possible.
    ///
    /// If there are less than `n` lines below the current cursor position,
    /// the cursor is moved to the bottom line of the terminal (i.e., as far down as possible).
    #[inline]
    pub fn move_cursor_down(&self, n: usize) -> io::Result<()> {
        move_cursor_down(self, n)
    }

    /// Move the cursor `n` characters to the left, if possible.
    ///
    /// If there are fewer than `n` characters to the left of the current cursor position,
    /// the cursor is moved to the beginning of the line (i.e., as far to the left as possible).
    #[inline]
    pub fn move_cursor_left(&self, n: usize) -> io::Result<()> {
        move_cursor_left(self, n)
    }

    /// Move the cursor `n` characters to the right.
    ///
    /// If there are fewer than `n` characters to the right of the current cursor position,
    /// the cursor is moved to the end of the current line (i.e., as far to the right as possible).
    #[inline]
    pub fn move_cursor_right(&self, n: usize) -> io::Result<()> {
        move_cursor_right(self, n)
    }

    /// Clear the current line.
    ///
    /// Position the cursor at the beginning of the current line.
    #[inline]
    pub fn clear_line(&self) -> io::Result<()> {
        clear_line(self)
    }

    /// Clear the last `n` lines before the current line.
    ///
    /// Position the cursor at the beginning of the first line that was cleared.
    pub fn clear_last_lines(&self, n: usize) -> io::Result<()> {
        self.move_cursor_up(n)?;
        for _ in 0..n {
            self.clear_line()?;
            self.move_cursor_down(1)?;
        }
        self.move_cursor_up(n)?;
        Ok(())
    }

    /// Clear the entire screen.
    ///
    /// Move the cursor to the upper left corner of the screen.
    #[inline]
    pub fn clear_screen(&self) -> io::Result<()> {
        clear_screen(self)
    }

    /// Clear everything from the current cursor position to the end of the screen.
    /// The cursor stays in its position.
    #[inline]
    pub fn clear_to_end_of_screen(&self) -> io::Result<()> {
        clear_to_end_of_screen(self)
    }

    /// Clear the last `n` characters of the current line.
    #[inline]
    pub fn clear_chars(&self, n: usize) -> io::Result<()> {
        clear_chars(self, n)
    }

    /// Set the terminal title.
    pub fn set_title<T: Display>(&self, title: T) {
        if !self.is_tty {
            return;
        }
        set_title(title);
    }

    /// Make the cursor visible again.
    #[inline]
    pub fn show_cursor(&self) -> io::Result<()> {
        show_cursor(self)
    }

    /// Hide the cursor.
    #[inline]
    pub fn hide_cursor(&self) -> io::Result<()> {
        hide_cursor(self)
    }

    // helpers

    #[cfg(all(windows, feature = "windows-console-colors"))]
    fn write_through(&self, bytes: &[u8]) -> io::Result<()> {
        if self.is_msys_tty || !self.is_tty {
            self.write_through_common(bytes)
        } else {
            match self.inner.target {
                TermTarget::Stdout => console_colors(self, Console::stdout()?, bytes),
                TermTarget::Stderr => console_colors(self, Console::stderr()?, bytes),
            }
        }
    }

    #[cfg(not(all(windows, feature = "windows-console-colors")))]
    fn write_through(&self, bytes: &[u8]) -> io::Result<()> {
        self.write_through_common(bytes)
    }

    pub(crate) fn write_through_common(&self, bytes: &[u8]) -> io::Result<()> {
        match self.inner.target {
            TermTarget::Stdout => {
                io::stdout().write_all(bytes)?;
                io::stdout().flush()?;
            }
            TermTarget::Stderr => {
                io::stderr().write_all(bytes)?;
                io::stderr().flush()?;
            }
            #[cfg(unix)]
            TermTarget::ReadWritePair(ReadWritePair { ref write, .. }) => {
                let mut write = write.lock().unwrap();
                write.write_all(bytes)?;
                write.flush()?;
            }
        }
        Ok(())
    }
}

/// A fast way to check if the application has a user attended for stdout.
///
/// This means that stdout is connected to a terminal instead of a
/// file or redirected by other means. This is a shortcut for
/// checking the `is_attended` feature on the stdout terminal.
#[inline]
pub fn user_attended() -> bool {
    Term::stdout().features().is_attended()
}

/// A fast way to check if the application has a user attended for stderr.
///
/// This means that stderr is connected to a terminal instead of a
/// file or redirected by other means. This is a shortcut for
/// checking the `is_attended` feature on the stderr terminal.
#[inline]
pub fn user_attended_stderr() -> bool {
    Term::stderr().features().is_attended()
}

#[cfg(any(unix, all(target_os = "wasi", target_env = "p1")))]
impl AsRawFd for Term {
    fn as_raw_fd(&self) -> RawFd {
        match self.inner.target {
            TermTarget::Stdout => libc::STDOUT_FILENO,
            TermTarget::Stderr => libc::STDERR_FILENO,
            #[cfg(unix)]
            TermTarget::ReadWritePair(ReadWritePair { ref write, .. }) => {
                write.lock().unwrap().as_raw_fd()
            }
        }
    }
}

#[cfg(windows)]
impl AsRawHandle for Term {
    fn as_raw_handle(&self) -> RawHandle {
        use windows_sys::Win32::System::Console::{
            GetStdHandle, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE,
        };

        unsafe {
            GetStdHandle(match self.inner.target {
                TermTarget::Stdout => STD_OUTPUT_HANDLE,
                TermTarget::Stderr => STD_ERROR_HANDLE,
            }) as RawHandle
        }
    }
}

impl Write for Term {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner.buffer {
            Some(ref buffer) => buffer.lock().unwrap().write_all(buf),
            None => self.write_through(buf),
        }?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Term::flush(self)
    }
}

impl Write for &Term {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner.buffer {
            Some(ref buffer) => buffer.lock().unwrap().write_all(buf),
            None => self.write_through(buf),
        }?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Term::flush(self)
    }
}

impl Read for Term {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        io::stdin().read(buf)
    }
}

impl Read for &Term {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        io::stdin().read(buf)
    }
}

#[cfg(all(unix, not(target_arch = "wasm32")))]
pub(crate) use crate::unix_term::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::wasm_term::*;
#[cfg(windows)]
pub(crate) use crate::windows_term::*;
