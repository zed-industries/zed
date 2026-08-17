use alloc::string::String;

/// Configures how a diagnostic is rendered.
#[derive(Clone, Debug)]
pub struct Config {
    /// The display style to use when rendering diagnostics.
    /// Defaults to: [`DisplayStyle::Rich`].
    ///
    /// [`DisplayStyle::Rich`]: DisplayStyle::Rich
    pub display_style: DisplayStyle,
    /// Column width of tabs.
    /// Defaults to: `4`.
    pub tab_width: usize,

    /// Characters to use when rendering the diagnostic.
    pub chars: Chars,
    /// The minimum number of lines to be shown after the line on which a multiline [`Label`] begins.
    ///
    /// Defaults to: `3`.
    ///
    /// [`Label`]: crate::diagnostic::Label
    pub start_context_lines: usize,
    /// The minimum number of lines to be shown before the line on which a multiline [`Label`] ends.
    ///
    /// Defaults to: `1`.
    ///
    /// [`Label`]: crate::diagnostic::Label
    pub end_context_lines: usize,
    /// The minimum number of lines before a label that should be included for context.
    ///
    /// Defaults to: `0`.
    pub before_label_lines: usize,
    /// The minimum number of lines after a label that should be included for context.
    ///
    /// Defaults to: `0`.
    pub after_label_lines: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            display_style: DisplayStyle::Rich,
            tab_width: 4,
            chars: Chars::default(),
            start_context_lines: 3,
            end_context_lines: 1,
            before_label_lines: 0,
            after_label_lines: 0,
        }
    }
}

/// The display style to use when rendering diagnostics.
#[derive(Clone, Debug)]
pub enum DisplayStyle {
    /// Output a richly formatted diagnostic, with source code previews.
    ///
    /// ```text
    /// error[E0001]: unexpected type in `+` application
    ///   ┌─ test:2:9
    ///   │
    /// 2 │ (+ test "")
    ///   │         ^^ expected `Int` but found `String`
    ///   │
    ///   = expected type `Int`
    ///        found type `String`
    ///
    /// error[E0002]: Bad config found
    ///
    /// ```
    Rich,
    /// Output a condensed diagnostic, with a line number, severity, message and notes (if any).
    ///
    /// ```text
    /// test:2:9: error[E0001]: unexpected type in `+` application
    /// = expected type `Int`
    ///      found type `String`
    ///
    /// error[E0002]: Bad config found
    /// ```
    Medium,
    /// Output a short diagnostic, with a line number, severity, and message.
    ///
    /// ```text
    /// test:2:9: error[E0001]: unexpected type in `+` application
    /// error[E0002]: Bad config found
    /// ```
    Short,
}

#[cfg(feature = "termcolor")]
pub mod styles {
    use super::super::renderer::{self, WriteStyle};

    use crate::diagnostic::{LabelStyle, Severity};
    use termcolor::{Color, ColorSpec, WriteColor};

    // re-export
    pub use termcolor;

    /// Styles to use when rendering the diagnostic.
    #[derive(Clone, Debug)]
    pub struct Styles {
        /// The style to use when rendering bug headers.
        /// Defaults to `fg:red bold intense`.
        pub header_bug: ColorSpec,
        /// The style to use when rendering error headers.
        /// Defaults to `fg:red bold intense`.
        pub header_error: ColorSpec,
        /// The style to use when rendering warning headers.
        /// Defaults to `fg:yellow bold intense`.
        pub header_warning: ColorSpec,
        /// The style to use when rendering note headers.
        /// Defaults to `fg:green bold intense`.
        pub header_note: ColorSpec,
        /// The style to use when rendering help headers.
        /// Defaults to `fg:cyan bold intense`.
        pub header_help: ColorSpec,
        /// The style to use when the main diagnostic message.
        /// Defaults to `bold intense`.
        pub header_message: ColorSpec,

        /// The style to use when rendering bug labels.
        /// Defaults to `fg:red`.
        pub primary_label_bug: ColorSpec,
        /// The style to use when rendering error labels.
        /// Defaults to `fg:red`.
        pub primary_label_error: ColorSpec,
        /// The style to use when rendering warning labels.
        /// Defaults to `fg:yellow`.
        pub primary_label_warning: ColorSpec,
        /// The style to use when rendering note labels.
        /// Defaults to `fg:green`.
        pub primary_label_note: ColorSpec,
        /// The style to use when rendering help labels.
        /// Defaults to `fg:cyan`.
        pub primary_label_help: ColorSpec,
        /// The style to use when rendering secondary labels.
        /// Defaults `fg:blue` (or `fg:cyan` on windows).
        pub secondary_label: ColorSpec,

        /// The style to use when rendering the line numbers.
        /// Defaults `fg:blue` (or `fg:cyan` on windows).
        pub line_number: ColorSpec,
        /// The style to use when rendering the source code borders.
        /// Defaults `fg:blue` (or `fg:cyan` on windows).
        pub source_border: ColorSpec,
        /// The style to use when rendering the note bullets.
        /// Defaults `fg:blue` (or `fg:cyan` on windows).
        pub note_bullet: ColorSpec,
    }

    impl Styles {
        /// The style used to mark a header at a given severity.
        pub fn header(&self, severity: Severity) -> &ColorSpec {
            match severity {
                Severity::Bug => &self.header_bug,
                Severity::Error => &self.header_error,
                Severity::Warning => &self.header_warning,
                Severity::Note => &self.header_note,
                Severity::Help => &self.header_help,
            }
        }

        pub fn header_message(&self) -> &ColorSpec {
            &self.header_message
        }

        pub fn line_number(&self) -> &ColorSpec {
            &self.line_number
        }

        pub fn note_bullet(&self) -> &ColorSpec {
            &self.note_bullet
        }

        pub fn source_border(&self) -> &ColorSpec {
            &self.source_border
        }

        /// The style used to mark a primary or secondary label at a given severity.
        pub fn label(&self, severity: Severity, label_style: LabelStyle) -> &ColorSpec {
            match (label_style, severity) {
                (LabelStyle::Primary, Severity::Bug) => &self.primary_label_bug,
                (LabelStyle::Primary, Severity::Error) => &self.primary_label_error,
                (LabelStyle::Primary, Severity::Warning) => &self.primary_label_warning,
                (LabelStyle::Primary, Severity::Note) => &self.primary_label_note,
                (LabelStyle::Primary, Severity::Help) => &self.primary_label_help,
                (LabelStyle::Secondary, _) => &self.secondary_label,
            }
        }
    }

    impl Styles {
        pub fn no_color() -> Styles {
            Styles {
                header_bug: ColorSpec::new(),
                header_error: ColorSpec::new(),
                header_warning: ColorSpec::new(),
                header_note: ColorSpec::new(),
                header_help: ColorSpec::new(),
                header_message: ColorSpec::new(),

                primary_label_bug: ColorSpec::new(),
                primary_label_error: ColorSpec::new(),
                primary_label_warning: ColorSpec::new(),
                primary_label_note: ColorSpec::new(),
                primary_label_help: ColorSpec::new(),
                secondary_label: ColorSpec::new(),

                line_number: ColorSpec::new(),
                source_border: ColorSpec::new(),
                note_bullet: ColorSpec::new(),
            }
        }
    }

    impl Default for Styles {
        fn default() -> Styles {
            // Default style
            let header = ColorSpec::new().set_bold(true).set_intense(true).clone();

            Styles {
                header_bug: header.clone().set_fg(Some(Color::Red)).clone(),
                header_error: header.clone().set_fg(Some(Color::Red)).clone(),
                header_warning: header.clone().set_fg(Some(Color::Yellow)).clone(),
                header_note: header.clone().set_fg(Some(Color::Green)).clone(),
                header_help: header.clone().set_fg(Some(Color::Cyan)).clone(),
                header_message: header,

                primary_label_bug: ColorSpec::new().set_fg(Some(Color::Red)).clone(),
                primary_label_error: ColorSpec::new().set_fg(Some(Color::Red)).clone(),
                primary_label_warning: ColorSpec::new().set_fg(Some(Color::Yellow)).clone(),
                primary_label_note: ColorSpec::new().set_fg(Some(Color::Green)).clone(),
                primary_label_help: ColorSpec::new().set_fg(Some(Color::Cyan)).clone(),
                secondary_label: ColorSpec::new().set_fg(Some(Color::Cyan)).clone(),

                line_number: ColorSpec::new().set_fg(Some(Color::Cyan)).clone(),
                source_border: ColorSpec::new().set_fg(Some(Color::Cyan)).clone(),
                note_bullet: ColorSpec::new().set_fg(Some(Color::Cyan)).clone(),
            }
        }
    }

    /// A [`WriteStyle`](crate::term::renderer::WriteStyle) implementation that applies custom [`Styles`]
    /// using termcolor.
    ///
    /// Use this to render diagnostics with custom colors
    pub struct StylesWriter<'a, W> {
        writer: W,
        style: &'a Styles,
    }

    impl<'a, W> StylesWriter<'a, W> {
        /// Creates a new `StylesWriter` with the given writer and styles.
        pub fn new(writer: W, style: &'a Styles) -> Self {
            Self { writer, style }
        }
    }

    // Always true here #[cfg(feature = "std")]
    impl<'a, W: WriteColor> std::io::Write for StylesWriter<'a, W> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.writer.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.writer.flush()
        }
    }

    impl<W: WriteColor> WriteStyle for StylesWriter<'_, W> {
        fn set_header(&mut self, severity: Severity) -> renderer::GeneralWriteResult {
            self.writer.set_color(self.style.header(severity))
        }

        fn set_header_message(&mut self) -> renderer::GeneralWriteResult {
            self.writer.set_color(&self.style.header_message)
        }

        fn set_line_number(&mut self) -> renderer::GeneralWriteResult {
            self.writer.set_color(&self.style.line_number)
        }

        fn set_note_bullet(&mut self) -> renderer::GeneralWriteResult {
            self.writer.set_color(&self.style.note_bullet)
        }

        fn set_source_border(&mut self) -> renderer::GeneralWriteResult {
            self.writer.set_color(&self.style.source_border)
        }

        fn set_label(
            &mut self,
            severity: Severity,
            label_style: LabelStyle,
        ) -> renderer::GeneralWriteResult {
            let spec = self.style.label(severity, label_style);
            self.writer.set_color(spec)
        }

        fn reset(&mut self) -> renderer::GeneralWriteResult {
            self.writer.reset()
        }
    }
}

#[cfg(feature = "termcolor")]
impl<T> super::renderer::WriteStyle for T
where
    T: termcolor::WriteColor,
{
    fn set_header(
        &mut self,
        severity: crate::diagnostic::Severity,
    ) -> super::renderer::GeneralWriteResult {
        self.set_color(styles::Styles::default().header(severity))
    }

    fn set_header_message(&mut self) -> super::renderer::GeneralWriteResult {
        self.set_color(&styles::Styles::default().header_message)
    }

    fn set_line_number(&mut self) -> super::renderer::GeneralWriteResult {
        self.set_color(&styles::Styles::default().line_number)
    }

    fn set_note_bullet(&mut self) -> super::renderer::GeneralWriteResult {
        self.set_color(&styles::Styles::default().note_bullet)
    }

    fn set_source_border(&mut self) -> super::renderer::GeneralWriteResult {
        self.set_color(&styles::Styles::default().source_border)
    }

    fn set_label(
        &mut self,
        severity: crate::diagnostic::Severity,
        label_style: crate::diagnostic::LabelStyle,
    ) -> super::renderer::GeneralWriteResult {
        let styles = styles::Styles::default();
        let spec = styles.label(severity, label_style);
        self.set_color(spec)
    }

    fn reset(&mut self) -> super::renderer::GeneralWriteResult {
        self.reset()
    }
}

/// Characters to use when rendering the diagnostic.
///
/// By using [`Chars::ascii()`] you can switch to an ASCII-only format suitable
/// for rendering on terminals that do not support box drawing characters.
#[derive(Clone, Debug)]
pub struct Chars {
    /// The characters to use for the top-left border of the snippet.
    /// Defaults to: `"┌─"` or `"-->"` with [`Chars::ascii()`].
    pub snippet_start: String,
    /// The character to use for the left border of the source.
    /// Defaults to: `'│'` or `'|'` with [`Chars::ascii()`].
    pub source_border_left: char,
    /// The character to use for the left border break of the source.
    /// Defaults to: `'·'` or `'.'` with [`Chars::ascii()`].
    pub source_border_left_break: char,

    /// The character to use for the note bullet.
    /// Defaults to: `'='`.
    pub note_bullet: char,

    /// The character to use for marking a single-line primary label.
    /// Defaults to: `'^'`.
    pub single_primary_caret: char,
    /// The character to use for marking a single-line secondary label.
    /// Defaults to: `'-'`.
    pub single_secondary_caret: char,

    /// The character to use for marking the start of a multi-line primary label.
    /// Defaults to: `'^'`.
    pub multi_primary_caret_start: char,
    /// The character to use for marking the end of a multi-line primary label.
    /// Defaults to: `'^'`.
    pub multi_primary_caret_end: char,
    /// The character to use for marking the start of a multi-line secondary label.
    /// Defaults to: `'\''`.
    pub multi_secondary_caret_start: char,
    /// The character to use for marking the end of a multi-line secondary label.
    /// Defaults to: `'\''`.
    pub multi_secondary_caret_end: char,
    /// The character to use for the top-left corner of a multi-line label.
    /// Defaults to: `'╭'` or `'/'` with [`Chars::ascii()`].
    pub multi_top_left: char,
    /// The character to use for the top of a multi-line label.
    /// Defaults to: `'─'` or `'-'` with [`Chars::ascii()`].
    pub multi_top: char,
    /// The character to use for the bottom-left corner of a multi-line label.
    /// Defaults to: `'╰'` or `'\'` with [`Chars::ascii()`].
    pub multi_bottom_left: char,
    /// The character to use when marking the bottom of a multi-line label.
    /// Defaults to: `'─'` or `'-'` with [`Chars::ascii()`].
    pub multi_bottom: char,
    /// The character to use for the left of a multi-line label.
    /// Defaults to: `'│'` or `'|'` with [`Chars::ascii()`].
    pub multi_left: char,

    /// The character to use for the left of a pointer underneath a caret.
    /// Defaults to: `'│'` or `'|'` with [`Chars::ascii()`].
    pub pointer_left: char,
}

impl Default for Chars {
    fn default() -> Chars {
        Chars::box_drawing()
    }
}

impl Chars {
    /// A character set that uses Unicode box drawing characters.
    pub fn box_drawing() -> Chars {
        Chars {
            snippet_start: "┌─".into(),
            source_border_left: '│',
            source_border_left_break: '·',

            note_bullet: '=',

            single_primary_caret: '^',
            single_secondary_caret: '-',

            multi_primary_caret_start: '^',
            multi_primary_caret_end: '^',
            multi_secondary_caret_start: '\'',
            multi_secondary_caret_end: '\'',
            multi_top_left: '╭',
            multi_top: '─',
            multi_bottom_left: '╰',
            multi_bottom: '─',
            multi_left: '│',

            pointer_left: '│',
        }
    }

    /// A character set that only uses ASCII characters.
    ///
    /// This is useful if your terminal's font does not support box drawing
    /// characters well and results in output that looks similar to rustc's
    /// diagnostic output.
    pub fn ascii() -> Chars {
        Chars {
            snippet_start: "-->".into(),
            source_border_left: '|',
            source_border_left_break: '.',

            note_bullet: '=',

            single_primary_caret: '^',
            single_secondary_caret: '-',

            multi_primary_caret_start: '^',
            multi_primary_caret_end: '^',
            multi_secondary_caret_start: '\'',
            multi_secondary_caret_end: '\'',
            multi_top_left: '/',
            multi_top: '-',
            multi_bottom_left: '\\',
            multi_bottom: '-',
            multi_left: '|',

            pointer_left: '|',
        }
    }
}
