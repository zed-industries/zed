use std::io;
use std::io::prelude::*;
use termcolor::{ColorSpec, WriteColor};

// Color tester from:
// https://github.com/wycats/language-reporting/blob/b021c87e0d4916b5f32756151bf215c220eee52d/crates/render-tree/src/stylesheet/accumulator.rs

/// A facility for creating visually inspectable representations of colored output
/// so they can be easily tested.
///
/// A new color is represented as `{style}` and a reset is represented by `{/}`.
///
/// Attributes are printed in this order:
///
/// - Foreground color as `fg:Color`
/// - Background color as `bg:Color`
/// - Bold as `bold`
/// - Underline as `underline`
/// - Intense as `bright`
///
/// For example, the style "intense, bold red foreground" would be printed as:
///
/// ```text
/// {fg:Red bold intense}
/// ```
///
/// Since this implementation attempts to make it possible to faithfully
/// understand what real WriteColor implementations would do, it tries
/// to approximate the contract in the WriteColor trait: "Subsequent
/// writes to this write will use these settings until either reset is
/// called or new color settings are set.")
///
/// - If set_color is called with a style, `{...}` is emitted containing the
///   color attributes.
/// - If set_color is called with no style, `{/}` is emitted
/// - If reset is called, `{/}` is emitted.
pub struct ColorBuffer {
    buf: Vec<u8>,
    color: ColorSpec,
}

impl ColorBuffer {
    pub fn new() -> ColorBuffer {
        ColorBuffer {
            buf: Vec::new(),
            color: ColorSpec::new(),
        }
    }

    pub fn into_string(self) -> String {
        String::from_utf8(self.buf).unwrap()
    }
}

impl io::Write for ColorBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl WriteColor for ColorBuffer {
    fn supports_color(&self) -> bool {
        true
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        #![allow(unused_assignments)]

        if self.color == *spec {
            return Ok(());
        } else {
            self.color = spec.clone();
        }

        if spec.is_none() {
            write!(self, "{{/}}")?;
            return Ok(());
        } else {
            write!(self, "{{")?;
        }

        let mut first = true;

        fn write_first(first: bool, write: &mut ColorBuffer) -> io::Result<bool> {
            if !first {
                write!(write, " ")?;
            }

            Ok(false)
        }

        if let Some(fg) = spec.fg() {
            first = write_first(first, self)?;
            write!(self, "fg:{:?}", fg)?;
        }

        if let Some(bg) = spec.bg() {
            first = write_first(first, self)?;
            write!(self, "bg:{:?}", bg)?;
        }

        if spec.bold() {
            first = write_first(first, self)?;
            write!(self, "bold")?;
        }

        if spec.underline() {
            first = write_first(first, self)?;
            write!(self, "underline")?;
        }

        if spec.intense() {
            first = write_first(first, self)?;
            write!(self, "bright")?;
        }

        write!(self, "}}")?;

        Ok(())
    }

    fn reset(&mut self) -> io::Result<()> {
        let color = self.color.clone();

        if color != ColorSpec::new() {
            write!(self, "{{/}}")?;
            self.color = ColorSpec::new();
        }

        Ok(())
    }
}
