use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::Buffer;

/// A set of styles to apply to the terminal output.
///
/// Call [`Formatter::style`] to get a `Style` and use the builder methods to
/// set styling properties, like [color] and [weight].
/// To print a value using the style, wrap it in a call to [`value`] when the log
/// record is formatted.
///
/// # Examples
///
/// Create a bold, red colored style and use it to print the log level:
///
/// ```
/// use std::io::Write;
/// use env_logger::fmt::Color;
///
/// let mut builder = env_logger::Builder::new();
///
/// builder.format(|buf, record| {
///     let mut level_style = buf.style();
///
///     level_style.set_color(Color::Red).set_bold(true);
///
///     writeln!(buf, "{}: {}",
///         level_style.value(record.level()),
///         record.args())
/// });
/// ```
///
/// Styles can be re-used to output multiple values:
///
/// ```
/// use std::io::Write;
/// use env_logger::fmt::Color;
///
/// let mut builder = env_logger::Builder::new();
///
/// builder.format(|buf, record| {
///     let mut bold = buf.style();
///
///     bold.set_bold(true);
///
///     writeln!(buf, "{}: {} {}",
///         bold.value(record.level()),
///         bold.value("some bold text"),
///         record.args())
/// });
/// ```
///
/// [`Formatter::style`]: struct.Formatter.html#method.style
/// [color]: #method.set_color
/// [weight]: #method.set_bold
/// [`value`]: #method.value
#[derive(Clone)]
pub struct Style {
    pub(in crate::fmt) buf: Rc<RefCell<Buffer>>,
    pub(in crate::fmt) spec: termcolor::ColorSpec,
}

impl Style {
    /// Set the text color.
    ///
    /// # Examples
    ///
    /// Create a style with red text:
    ///
    /// ```
    /// use std::io::Write;
    /// use env_logger::fmt::Color;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_color(Color::Red);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_color(&mut self, color: Color) -> &mut Style {
        self.spec.set_fg(Some(color.into_termcolor()));
        self
    }

    /// Set the text weight.
    ///
    /// If `yes` is true then text will be written in bold.
    /// If `yes` is false then text will be written in the default weight.
    ///
    /// # Examples
    ///
    /// Create a style with bold text:
    ///
    /// ```
    /// use std::io::Write;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_bold(true);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_bold(&mut self, yes: bool) -> &mut Style {
        self.spec.set_bold(yes);
        self
    }

    /// Set the text intensity.
    ///
    /// If `yes` is true then text will be written in a brighter color.
    /// If `yes` is false then text will be written in the default color.
    ///
    /// # Examples
    ///
    /// Create a style with intense text:
    ///
    /// ```
    /// use std::io::Write;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_intense(true);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_intense(&mut self, yes: bool) -> &mut Style {
        self.spec.set_intense(yes);
        self
    }

    /// Set whether the text is dimmed.
    ///
    /// If `yes` is true then text will be written in a dimmer color.
    /// If `yes` is false then text will be written in the default color.
    ///
    /// # Examples
    ///
    /// Create a style with dimmed text:
    ///
    /// ```
    /// use std::io::Write;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_dimmed(true);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_dimmed(&mut self, yes: bool) -> &mut Style {
        self.spec.set_dimmed(yes);
        self
    }

    /// Set the background color.
    ///
    /// # Examples
    ///
    /// Create a style with a yellow background:
    ///
    /// ```
    /// use std::io::Write;
    /// use env_logger::fmt::Color;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_bg(Color::Yellow);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_bg(&mut self, color: Color) -> &mut Style {
        self.spec.set_bg(Some(color.into_termcolor()));
        self
    }

    /// Wrap a value in the style.
    ///
    /// The same `Style` can be used to print multiple different values.
    ///
    /// # Examples
    ///
    /// Create a bold, red colored style and use it to print the log level:
    ///
    /// ```
    /// use std::io::Write;
    /// use env_logger::fmt::Color;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_color(Color::Red).set_bold(true);
    ///
    ///     writeln!(buf, "{}: {}",
    ///         style.value(record.level()),
    ///         record.args())
    /// });
    /// ```
    pub fn value<T>(&self, value: T) -> StyledValue<T> {
        StyledValue {
            style: Cow::Borrowed(self),
            value,
        }
    }

    /// Wrap a value in the style by taking ownership of it.
    pub(crate) fn into_value<T>(self, value: T) -> StyledValue<'static, T> {
        StyledValue {
            style: Cow::Owned(self),
            value,
        }
    }
}

impl fmt::Debug for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Style").field("spec", &self.spec).finish()
    }
}

/// A value that can be printed using the given styles.
///
/// It is the result of calling [`Style::value`].
///
/// [`Style::value`]: struct.Style.html#method.value
pub struct StyledValue<'a, T> {
    style: Cow<'a, Style>,
    value: T,
}

impl<'a, T> StyledValue<'a, T> {
    fn write_fmt<F>(&self, f: F) -> fmt::Result
    where
        F: FnOnce() -> fmt::Result,
    {
        self.style
            .buf
            .borrow_mut()
            .set_color(&self.style.spec)
            .map_err(|_| fmt::Error)?;

        // Always try to reset the terminal style, even if writing failed
        let write = f();
        let reset = self.style.buf.borrow_mut().reset().map_err(|_| fmt::Error);

        write.and(reset)
    }
}

macro_rules! impl_styled_value_fmt {
    ($($fmt_trait:path),*) => {
        $(
            impl<'a, T: $fmt_trait> $fmt_trait for StyledValue<'a, T> {
                fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
                    self.write_fmt(|| T::fmt(&self.value, f))
                }
            }
        )*
    };
}

impl_styled_value_fmt!(
    fmt::Debug,
    fmt::Display,
    fmt::Pointer,
    fmt::Octal,
    fmt::Binary,
    fmt::UpperHex,
    fmt::LowerHex,
    fmt::UpperExp,
    fmt::LowerExp
);

// The `Color` type is copied from https://github.com/BurntSushi/termcolor

/// The set of available colors for the terminal foreground/background.
///
/// The `Ansi256` and `Rgb` colors will only output the correct codes when
/// paired with the `Ansi` `WriteColor` implementation.
///
/// The `Ansi256` and `Rgb` color types are not supported when writing colors
/// on Windows using the console. If they are used on Windows, then they are
/// silently ignored and no colors will be emitted.
///
/// This set may expand over time.
///
/// This type has a `FromStr` impl that can parse colors from their human
/// readable form. The format is as follows:
///
/// 1. Any of the explicitly listed colors in English. They are matched
///    case insensitively.
/// 2. A single 8-bit integer, in either decimal or hexadecimal format.
/// 3. A triple of 8-bit integers separated by a comma, where each integer is
///    in decimal or hexadecimal format.
///
/// Hexadecimal numbers are written with a `0x` prefix.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    fn into_termcolor(self) -> termcolor::Color {
        match self {
            Color::Black => termcolor::Color::Black,
            Color::Blue => termcolor::Color::Blue,
            Color::Green => termcolor::Color::Green,
            Color::Red => termcolor::Color::Red,
            Color::Cyan => termcolor::Color::Cyan,
            Color::Magenta => termcolor::Color::Magenta,
            Color::Yellow => termcolor::Color::Yellow,
            Color::White => termcolor::Color::White,
            Color::Ansi256(value) => termcolor::Color::Ansi256(value),
            Color::Rgb(r, g, b) => termcolor::Color::Rgb(r, g, b),
        }
    }
}
