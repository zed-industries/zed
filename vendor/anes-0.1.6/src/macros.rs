/// Creates a control sequence.
///
/// This macro prepends provided sequence with the control sequence introducer `ESC [` (`\x1B[`).
///
/// # Examples
///
/// ```
/// use anes::csi;
///
/// assert_eq!(csi!("?1049h"), "\x1B[?1049h");
/// ```
#[macro_export]
macro_rules! csi {
    ($($arg:expr),*) => { concat!("\x1B[", $($arg),*) };
}

/// Creates an escape sequence.
///
/// This macro prepends provided sequence with the `ESC` (`\x1B`) character.
///
/// # Examples
///
/// ```
/// use anes::esc;
///
/// assert_eq!(esc!("7"), "\x1B7");
/// ```
#[macro_export]
macro_rules! esc {
    ($($arg:expr),*) => { concat!("\x1B", $($arg),*) };
}

/// Creates a select graphic rendition sequence.
///
/// This macro prepends provided sequence with the `ESC[` (`\x1B[`) character and appends `m` character.
///
/// Also known as Set Graphics Rendition on Linux.
///
/// # Examples
///
/// ```
/// use anes::sgr;
///
/// assert_eq!(sgr!("0"), "\x1B[0m");
/// ```
#[macro_export]
macro_rules! sgr {
    ($($arg:expr),*) => { concat!("\x1B[", $($arg),* , "m") };
}

/// Creates an ANSI sequence.
///
/// You can use this macro to create your own ANSI sequence. All `anes` sequences are
/// created with this macro.
///
/// # Examples
///
/// An unit struct:
///
/// ```
/// use anes::{esc, sequence};
///
/// sequence!(
///   /// Saves the cursor position.    
///   struct SaveCursorPosition => esc!("7")    
/// );
///
/// assert_eq!(&format!("{}", SaveCursorPosition), "\x1B7");
/// ```
///
/// An enum:
///
/// ```
/// use anes::{csi, sequence};
///
/// sequence!(
///     /// Clears part of the buffer.
///     enum ClearBuffer {
///         /// Clears from the cursor position to end of the screen.
///         Below => csi!("J"),
///         /// Clears from the cursor position to beginning of the screen.
///         Above => csi!("1J"),
///         /// Clears the entire buffer.
///         All => csi!("2J"),
///         /// Clears the entire buffer and all saved lines in the scrollback buffer.
///         SavedLines => csi!("3J"),
///     }
/// );
///
/// assert_eq!(&format!("{}", ClearBuffer::Below), "\x1B[J");
/// assert_eq!(&format!("{}", ClearBuffer::Above), "\x1B[1J");
/// assert_eq!(&format!("{}", ClearBuffer::All), "\x1B[2J");
/// assert_eq!(&format!("{}", ClearBuffer::SavedLines), "\x1B[3J");
/// ```
///
/// A struct:
///
/// ```
/// use anes::{csi, sequence};
///
/// sequence!(
///     /// Moves the cursor to the given location (column, row).
///     ///
///     /// # Notes
///     ///
///     /// Top/left cell is represented as `1, 1`.
///     struct MoveCursorTo(u16, u16) =>
///     |this, f| write!(f, csi!("{};{}H"), this.0, this.1)
/// );
///
/// assert_eq!(&format!("{}", MoveCursorTo(10, 5)), "\x1B[10;5H");
/// ```
#[macro_export]
macro_rules! sequence {
    // Static unit struct
    (
        $(#[$meta:meta])*
        struct $name:ident => $value:expr
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        pub struct $name;

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, $value)
            }
        }
    };
    // Static enum
    (
        $(#[$meta:meta])*
        enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $variant_value:expr
            ),*
            $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )*
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", match self {
                    $(
                        $name::$variant => $variant_value,
                    )*
                })
            }
        }
    };
    // Dynamic struct
    (
        $(#[$meta:meta])*
        struct $type:ident(
            $($fields:ty),*
            $(,)?
        )
        =>
        $write:expr
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        pub struct $type($(pub $fields),*);

        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let write: &dyn Fn(&Self, &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result =
                    &$write;
                write(self, f)
            }
        }
    };
}

/// Queues ANSI escape sequence(s).
///
/// What does queue mean exactly? All sequences are queued with the
/// `write!($dst, "{}", $sequence)` macro without calling the
/// [`flush`](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush) method.
///
/// Check the [`execute!`](macro.execute.html) macro if you'd like execute them
/// immediately (call the `flush` method after all sequences were queued).
///
/// # Examples
///
/// ```no_run
/// use std::io::{Result, Write};
///
/// use anes::queue;
///
/// fn main() -> Result<()> {
///     let mut stdout = std::io::stdout();
///     queue!(
///         &mut stdout,
///         anes::SaveCursorPosition,
///         anes::MoveCursorTo(10, 10)
///     )?;
///
///     queue!(&mut stdout, anes::RestoreCursorPosition,)?;
///
///     // ANSI sequences are not executed until you flush it!
///     stdout.flush()
/// }
/// ```
#[macro_export]
macro_rules! queue {
    ($dst:expr, $($sequence:expr),* $(,)?) => {{
        let mut error = None;

        $(
            if let Err(e) = write!($dst, "{}", $sequence) {
                error = Some(e);
            }
        )*

        if let Some(error) = error {
            Err(error)
        } else {
            Ok(())
        }
    }}
}

/// Executes ANSI escape sequence(s).
///
/// What does execute mean exactly? All sequences are queued with the
/// `write!($dst, "{}", $sequence)` macro and then the
/// [`flush`](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush) method
/// is called.
///
/// Check the [`queue!`](macro.queue.html) macro if you'd like queue sequences
/// and execute them later.
///
/// ```no_run
/// use std::io::{Result, Write};
///
/// use anes::execute;
///
/// fn main() -> Result<()> {
///     let mut stdout = std::io::stdout();
///     execute!(
///         &mut stdout,
///         anes::SaveCursorPosition,
///         anes::MoveCursorTo(10, 10),
///         anes::RestoreCursorPosition
///     )?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! execute {
    ($dst:expr, $($sequence:expr),* $(,)?) => {{
        if let Err(e) = $crate::queue!($dst, $($sequence),*) {
            Err(e)
        } else {
            $dst.flush()
        }
    }}
}

#[cfg(test)]
macro_rules! test_sequences {
    (
        $(
            $name:ident(
                $($left:expr => $right:expr),*
                $(,)?
            )
        ),*
        $(,)?
    ) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            $(
                #[test]
                fn $name() {
                    $(
                        assert_eq!(&format!("{}", $left), $right);
                    )*
                }
            )*
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind, Write};

    #[test]
    fn csi() {
        assert_eq!(csi!("foo"), "\x1B[foo");
    }

    #[test]
    fn esc() {
        assert_eq!(esc!("bar"), "\x1Bbar");
    }

    #[test]
    fn sgr() {
        assert_eq!(sgr!("bar"), "\x1B[barm");
    }

    #[test]
    fn static_struct_sequence() {
        sequence!(
            struct TestSeq => csi!("foo")
        );

        assert_eq!(&format!("{}", TestSeq), "\x1B[foo");
    }

    #[test]
    fn static_enum_sequence() {
        sequence!(
            enum TestSeq {
                Foo => csi!("foo"),
                Bar => esc!("bar"),
            }
        );

        assert_eq!(&format!("{}", TestSeq::Foo), "\x1B[foo");
        assert_eq!(&format!("{}", TestSeq::Bar), "\x1Bbar");
    }

    #[test]
    fn dynamic_struct_sequence() {
        sequence!(
            struct TestSeq(u16) =>
            |this, f| write!(f, csi!("foo{}bar"), this.0)
        );

        assert_eq!(&format!("{}", TestSeq(10)), "\x1B[foo10bar");
    }

    #[test]
    fn queue_allows_trailing_comma() {
        let mut writer = Writer::default();

        assert!(queue!(&mut writer, "foo",).is_ok());
        assert_eq!(&writer.buffer, "foo");
    }

    #[test]
    fn queue_writes_single_sequence() {
        let mut writer = Writer::default();

        assert!(queue!(&mut writer, "foo").is_ok());
        assert_eq!(&writer.buffer, "foo");
    }

    #[test]
    fn queue_writes_multiple_sequences() {
        let mut writer = Writer::default();

        assert!(queue!(&mut writer, "foo", "bar", "baz").is_ok());
        assert_eq!(&writer.buffer, "foobarbaz");
    }

    #[test]
    fn queue_does_not_flush() {
        let mut writer = Writer::default();

        assert!(queue!(&mut writer, "foo").is_ok());
        assert!(!writer.flushed);
        assert!(writer.flushed_buffer.is_empty());
    }

    #[test]
    fn execute_allows_trailing_comma() {
        let mut writer = Writer::default();

        assert!(execute!(&mut writer, "foo",).is_ok());
        assert_eq!(&writer.flushed_buffer, "foo");
    }

    #[test]
    fn execute_writes_single_sequence() {
        let mut writer = Writer::default();

        assert!(execute!(&mut writer, "foo").is_ok());
        assert_eq!(&writer.flushed_buffer, "foo");
    }

    #[test]
    fn execute_writes_multiple_sequences() {
        let mut writer = Writer::default();

        assert!(execute!(&mut writer, "foo", "bar", "baz").is_ok());
        assert_eq!(&writer.flushed_buffer, "foobarbaz");
    }

    #[test]
    fn execute_does_flush() {
        let mut writer = Writer::default();

        assert!(execute!(&mut writer, "foo").is_ok());
        assert!(writer.flushed);
        assert_eq!(&writer.flushed_buffer, "foo");
        assert!(writer.buffer.is_empty());
    }

    #[derive(Default)]
    struct Writer {
        buffer: String,
        flushed_buffer: String,
        flushed: bool,
    }

    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            let s = std::str::from_utf8(buf).map_err(|_| ErrorKind::InvalidData)?;

            self.buffer.push_str(s);
            Ok(s.len())
        }

        fn flush(&mut self) -> Result<(), Error> {
            self.flushed_buffer = self.buffer.clone();
            self.buffer = String::new();
            self.flushed = true;
            Ok(())
        }
    }
}
