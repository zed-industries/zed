use alloc::string::String;
use core::ops::Range;

use crate::diagnostic::{LabelStyle, Severity};
use crate::files::{Error, Location};
use crate::term::{Chars, Config};

#[cfg(feature = "std")]
pub use std::io::Write as GeneralWrite;

#[cfg(feature = "std")]
pub type GeneralWriteResult = std::io::Result<()>;

#[cfg(not(feature = "std"))]
pub use core::fmt::Write as GeneralWrite;

#[cfg(not(feature = "std"))]
pub use core::fmt::Result as GeneralWriteResult;

/// A writer that can apply styling for different parts of a diagnostic renderer.
///
/// # Implementations
///
/// - [`PlainWriter<W>`](PlainWriter) - no-op styling, plain text output
/// - [`StylesWriter<W>`](crate::term::StylesWriter) - custom styles (requires `termcolor` feature)
/// - `T: WriteColor` - blanket impl using default styles (requires `termcolor` feature)
pub trait WriteStyle: GeneralWrite {
    fn set_header(&mut self, severity: Severity) -> GeneralWriteResult;

    fn set_header_message(&mut self) -> GeneralWriteResult;

    fn set_line_number(&mut self) -> GeneralWriteResult;

    fn set_note_bullet(&mut self) -> GeneralWriteResult;

    fn set_source_border(&mut self) -> GeneralWriteResult;

    fn set_label(&mut self, severity: Severity, label_style: LabelStyle) -> GeneralWriteResult;

    fn reset(&mut self) -> GeneralWriteResult;
}

/// A [`WriteStyle`] implementation that ignores all styling calls. useful for non color output.
/// Available on all targets
pub struct PlainWriter<W: GeneralWrite> {
    w: W,
}

impl<W: GeneralWrite> PlainWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { w: writer }
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> std::io::Write for PlainWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.w.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

#[cfg(not(feature = "std"))]
impl<W: core::fmt::Write> core::fmt::Write for PlainWriter<W> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.w.write_str(s)
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.w.write_char(c)
    }

    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
        self.w.write_fmt(args)
    }
}

impl<W: GeneralWrite> WriteStyle for PlainWriter<W> {
    fn set_header(&mut self, _severity: Severity) -> GeneralWriteResult {
        Ok(())
    }

    fn set_header_message(&mut self) -> GeneralWriteResult {
        Ok(())
    }

    fn set_line_number(&mut self) -> GeneralWriteResult {
        Ok(())
    }

    fn set_note_bullet(&mut self) -> GeneralWriteResult {
        Ok(())
    }

    fn set_source_border(&mut self) -> GeneralWriteResult {
        Ok(())
    }

    fn set_label(&mut self, _severity: Severity, _label_style: LabelStyle) -> GeneralWriteResult {
        Ok(())
    }

    fn reset(&mut self) -> GeneralWriteResult {
        Ok(())
    }
}

/// The 'location focus' of a source code snippet.
pub struct Locus {
    /// The user-facing name of the file.
    pub name: String,
    /// The location.
    pub location: Location,
}

/// Single-line label, with an optional message.
///
/// ```text
/// ^^^^^^^^^ blah blah
/// ```
pub type SingleLabel<'diagnostic> = (LabelStyle, Range<usize>, &'diagnostic str);

/// A multi-line label to render.
///
/// Locations are relative to the start of where the source code is rendered.
pub enum MultiLabel<'diagnostic> {
    /// Multi-line label top.
    /// The contained value indicates where the label starts.
    ///
    /// ```text
    /// ╭────────────^
    /// ```
    ///
    /// Can also be rendered at the beginning of the line
    /// if there is only whitespace before the label starts.
    ///
    /// ```text
    /// ╭
    /// ```
    Top(usize),
    /// Left vertical labels for multi-line labels.
    ///
    /// ```text
    /// │
    /// ```
    Left,
    /// Multi-line label bottom, with an optional message.
    /// The first value indicates where the label ends.
    ///
    /// ```text
    /// ╰────────────^ blah blah
    /// ```
    Bottom(usize, &'diagnostic str),
}

#[derive(Copy, Clone)]
enum VerticalBound {
    Top,
    Bottom,
}

type Underline = (LabelStyle, VerticalBound);

/// A renderer of display list entries.
///
/// The following diagram gives an overview of each of the parts of the renderer's output:
///
/// ```text
///                     ┌ outer gutter
///                     │ ┌ left border
///                     │ │ ┌ inner gutter
///                     │ │ │   ┌─────────────────────────── source ─────────────────────────────┐
///                     │ │ │   │                                                                │
///                  ┌────────────────────────────────────────────────────────────────────────────
///        header ── │ error[0001]: oh noes, a cupcake has occurred!
/// snippet start ── │    ┌─ test:9:0
/// snippet empty ── │    │
///  snippet line ── │  9 │   ╭ Cupcake ipsum dolor. Sit amet marshmallow topping cheesecake
///  snippet line ── │ 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
///                  │    │ ╭─│─────────^
/// snippet break ── │    · │ │
///  snippet line ── │ 33 │ │ │ Muffin danish chocolate soufflé pastry icing bonbon oat cake.
///  snippet line ── │ 34 │ │ │ Powder cake jujubes oat cake. Lemon drops tootsie roll marshmallow
///                  │    │ │ ╰─────────────────────────────^ blah blah
/// snippet break ── │    · │
///  snippet line ── │ 38 │ │   Brownie lemon drops chocolate jelly-o candy canes. Danish marzipan
///  snippet line ── │ 39 │ │   jujubes soufflé carrot cake marshmallow tiramisu caramels candy canes.
///                  │    │ │           ^^^^^^^^^^^^^^^^^^^ -------------------- blah blah
///                  │    │ │           │
///                  │    │ │           blah blah
///                  │    │ │           note: this is a note
///  snippet line ── │ 40 │ │   Fruitcake jelly-o danish toffee. Tootsie roll pastry cheesecake
///  snippet line ── │ 41 │ │   soufflé marzipan. Chocolate bar oat cake jujubes lollipop pastry
///  snippet line ── │ 42 │ │   cupcake. Candy canes cupcake toffee gingerbread candy canes muffin
///                  │    │ │                                ^^^^^^^^^^^^^^^^^^ blah blah
///                  │    │ ╰──────────^ blah blah
/// snippet break ── │    ·
///  snippet line ── │ 82 │     gingerbread toffee chupa chups chupa chups jelly-o cotton candy.
///                  │    │                 ^^^^^^                         ------- blah blah
/// snippet empty ── │    │
///  snippet note ── │    = blah blah
///  snippet note ── │    = blah blah blah
///                  │      blah blah
///  snippet note ── │    = blah blah blah
///                  │      blah blah
///         empty ── │
/// ```
///
/// > Filler text from <http://www.cupcakeipsum.com>
pub struct Renderer<'writer, 'config> {
    writer: &'writer mut dyn WriteStyle,
    config: &'config Config,
}

impl<'writer, 'config> Renderer<'writer, 'config> {
    /// Construct a renderer from the given writer and config.
    pub fn new(
        writer: &'writer mut dyn WriteStyle,
        config: &'config Config,
    ) -> Renderer<'writer, 'config> {
        Renderer { writer, config }
    }

    fn chars(&self) -> &'config Chars {
        &self.config.chars
    }

    /// Diagnostic header, with severity, code, and message.
    ///
    /// ```text
    /// error[E0001]: unexpected type in `+` application
    /// ```
    pub fn render_header(
        &mut self,
        locus: Option<&Locus>,
        severity: Severity,
        code: Option<&str>,
        message: &str,
    ) -> Result<(), Error> {
        // Write locus
        //
        // ```text
        // test:2:9:
        // ```
        if let Some(locus) = locus {
            self.snippet_locus(locus)?;
            write!(self, ": ")?;
        }

        // Write severity name
        //
        // ```text
        // error
        // ```
        self.set_header(severity)?;
        match severity {
            Severity::Bug => write!(self, "bug")?,
            Severity::Error => write!(self, "error")?,
            Severity::Warning => write!(self, "warning")?,
            Severity::Help => write!(self, "help")?,
            Severity::Note => write!(self, "note")?,
        }

        // Write error code
        //
        // ```text
        // [E0001]
        // ```
        if let Some(code) = &code.filter(|code| !code.is_empty()) {
            write!(self, "[{code}]")?;
        }

        // Write diagnostic message
        //
        // ```text
        // : unexpected type in `+` application
        // ```
        self.set_header_message()?;
        write!(self, ": {message}")?;
        self.reset()?;

        writeln!(self)?;

        Ok(())
    }

    /// Empty line.
    pub fn render_empty(&mut self) -> Result<(), Error> {
        writeln!(self)?;
        Ok(())
    }

    /// Top left border and locus.
    ///
    /// ```text
    /// ┌─ test:2:9
    /// ```
    pub fn render_snippet_start(
        &mut self,
        outer_padding: usize,
        locus: &Locus,
    ) -> Result<(), Error> {
        self.outer_gutter(outer_padding)?;

        self.set_source_border()?;
        write!(self, "{}", self.chars().snippet_start)?;
        self.reset()?;

        write!(self, " ")?;
        self.snippet_locus(locus)?;

        writeln!(self)?;

        Ok(())
    }

    /// A line of source code.
    ///
    /// ```text
    /// 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
    ///    │ ╭─│─────────^
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn render_snippet_source(
        &mut self,
        outer_padding: usize,
        line_number: usize,
        source: &str,
        severity: Severity,
        single_labels: &[SingleLabel<'_>],
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        // Trim trailing newlines, linefeeds, and null chars from source, if they exist.
        // FIXME: Use the number of trimmed placeholders when rendering single line carets
        let source = source.trim_end_matches(['\n', '\r', '\0'].as_ref());

        // Write source line
        //
        // ```text
        // 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
        // ```
        {
            // Write outer gutter (with line number) and border
            self.outer_gutter_number(line_number, outer_padding)?;
            self.border_left()?;

            // Write inner gutter (with multi-line continuations on the left if necessary)
            let mut multi_labels_iter = multi_labels.iter().peekable();
            for label_column in 0..num_multi_labels {
                match multi_labels_iter.peek() {
                    Some((label_index, label_style, label)) if *label_index == label_column => {
                        match label {
                            MultiLabel::Top(start)
                                if *start <= source.len() - source.trim_start().len() =>
                            {
                                self.label_multi_top_left(severity, *label_style)?;
                            }
                            MultiLabel::Top(..) => self.inner_gutter_space()?,
                            MultiLabel::Left | MultiLabel::Bottom(..) => {
                                self.label_multi_left(severity, *label_style, None)?;
                            }
                        }
                        multi_labels_iter.next();
                    }
                    Some((_, _, _)) | None => self.inner_gutter_space()?,
                }
            }

            // Write source text
            write!(self, " ")?;
            let mut in_primary = false;
            for (metrics, ch) in self.char_metrics(source.char_indices()) {
                let column_range = metrics.byte_index..(metrics.byte_index + ch.len_utf8());

                // Check if we are overlapping a primary label
                let is_primary = single_labels.iter().any(|(ls, range, _)| {
                    *ls == LabelStyle::Primary && is_overlapping(range, &column_range)
                }) || multi_labels.iter().any(|(_, ls, label)| {
                    *ls == LabelStyle::Primary
                        && match label {
                            MultiLabel::Top(start) => column_range.start >= *start,
                            MultiLabel::Left => true,
                            MultiLabel::Bottom(start, _) => column_range.end <= *start,
                        }
                });

                // Set the source color if we are in a primary label
                if is_primary && !in_primary {
                    self.set_label(severity, LabelStyle::Primary)?;
                    in_primary = true;
                } else if !is_primary && in_primary {
                    self.reset()?;
                    in_primary = false;
                }

                match ch {
                    '\t' => (0..metrics.unicode_width).try_for_each(|_| write!(self, " "))?,
                    _ => write!(self, "{ch}")?,
                }
            }
            if in_primary {
                self.reset()?;
            }
            writeln!(self)?;
        }

        // Write single labels underneath source
        //
        // ```text
        //   │     - ---- ^^^ second mutable borrow occurs here
        //   │     │ │
        //   │     │ first mutable borrow occurs here
        //   │     first borrow later used by call
        //   │     help: some help here
        // ```
        if !single_labels.is_empty() {
            // Our plan is as follows:
            //
            // 1. Do an initial scan to find:
            //    - The number of non-empty messages.
            //    - The right-most start and end positions of labels.
            //    - A candidate for a trailing label (where the label's message
            //      is printed to the left of the caret).
            // 2. Check if the trailing label candidate overlaps another label -
            //    if so we print it underneath the carets with the other labels.
            // 3. Print a line of carets, and (possibly) the trailing message
            //    to the left.
            // 4. Print vertical lines pointing to the carets, and the messages
            //    for those carets.
            //
            // We try our best avoid introducing new dynamic allocations,
            // instead preferring to iterate over the labels multiple times. It
            // is unclear what the performance tradeoffs are however, so further
            // investigation may be required.

            // The number of non-empty messages to print.
            let mut num_messages = 0;
            // The right-most start position, eg:
            //
            // ```text
            // -^^^^---- ^^^^^^^
            //           │
            //           right-most start position
            // ```
            let mut max_label_start = 0;
            // The right-most end position, eg:
            //
            // ```text
            // -^^^^---- ^^^^^^^
            //                 │
            //                 right-most end position
            // ```
            let mut max_label_end = 0;
            // A trailing message, eg:
            //
            // ```text
            // ^^^ second mutable borrow occurs here
            // ```
            let mut trailing_label = None;

            for (label_index, label) in single_labels.iter().enumerate() {
                let (_, range, message) = label;
                if !message.is_empty() {
                    num_messages += 1;
                }
                max_label_start = core::cmp::max(max_label_start, range.start);
                max_label_end = core::cmp::max(max_label_end, range.end);
                // This is a candidate for the trailing label, so let's record it.
                if range.end == max_label_end {
                    if message.is_empty() {
                        trailing_label = None;
                    } else {
                        trailing_label = Some((label_index, label));
                    }
                }
            }
            if let Some((trailing_label_index, (_, trailing_range, _))) = trailing_label {
                // Check to see if the trailing label candidate overlaps any of
                // the other labels on the current line.
                if single_labels
                    .iter()
                    .enumerate()
                    .filter(|(label_index, _)| *label_index != trailing_label_index)
                    .any(|(_, (_, range, _))| is_overlapping(trailing_range, range))
                {
                    // If it does, we'll instead want to render it below the
                    // carets along with the other hanging labels.
                    trailing_label = None;
                }
            }

            // Write a line of carets
            //
            // ```text
            //   │ ^^^^^^  -------^^^^^^^^^-------^^^^^----- ^^^^ trailing label message
            // ```
            self.outer_gutter(outer_padding)?;
            self.border_left()?;
            self.inner_gutter(severity, num_multi_labels, multi_labels)?;
            write!(self, " ")?;

            let mut previous_label_style = None;
            let placeholder_metrics = Metrics {
                byte_index: source.len(),
                unicode_width: 1,
            };
            for (metrics, ch) in self
                .char_metrics(source.char_indices())
                // Add a placeholder source column at the end to allow for
                // printing carets at the end of lines, eg:
                //
                // ```text
                // 1 │ Hello world!
                //   │             ^
                // ```
                .chain(core::iter::once((placeholder_metrics, '\0')))
            {
                // Find the current label style at this column
                let column_range = metrics.byte_index..(metrics.byte_index + ch.len_utf8());
                let current_label_style = single_labels
                    .iter()
                    .filter(|(_, range, _)| is_overlapping(range, &column_range))
                    .map(|(label_style, _, _)| label_style.clone())
                    .max_by_key(label_priority_key);

                // Update writer style if necessary
                if previous_label_style != current_label_style {
                    match current_label_style {
                        None => {
                            self.reset()?;
                        }
                        Some(label_style) => {
                            self.set_label(severity, label_style)?;
                        }
                    }
                }

                let caret_ch = match current_label_style {
                    Some(LabelStyle::Primary) => Some(self.chars().single_primary_caret),
                    Some(LabelStyle::Secondary) => Some(self.chars().single_secondary_caret),
                    // Only print padding if we are before the end of the last single line caret
                    None if metrics.byte_index < max_label_end => Some(' '),
                    None => None,
                };
                if let Some(caret_ch) = caret_ch {
                    // FIXME: improve rendering of carets between character boundaries
                    (0..metrics.unicode_width).try_for_each(|_| write!(self, "{caret_ch}",))?;
                }

                previous_label_style = current_label_style;
            }
            // Reset style if it was previously set
            if previous_label_style.is_some() {
                self.reset()?;
            }
            // Write first trailing label message
            if let Some((_, (label_style, _, message))) = trailing_label {
                write!(self, " ")?;
                self.set_label(severity, *label_style)?;
                write!(self, "{message}",)?;
                self.reset()?;
            }
            writeln!(self)?;

            // Write hanging labels pointing to carets
            //
            // ```text
            //   │     │ │
            //   │     │ first mutable borrow occurs here
            //   │     first borrow later used by call
            //   │     help: some help here
            // ```
            if num_messages > trailing_label.iter().count() {
                // Write first set of vertical lines before hanging labels
                //
                // ```text
                //   │     │ │
                // ```
                self.outer_gutter(outer_padding)?;
                self.border_left()?;
                self.inner_gutter(severity, num_multi_labels, multi_labels)?;
                write!(self, " ")?;
                self.caret_pointers(
                    severity,
                    max_label_start,
                    single_labels,
                    trailing_label,
                    source.char_indices(),
                )?;
                writeln!(self)?;

                // Write hanging labels pointing to carets
                //
                // ```text
                //   │     │ first mutable borrow occurs here
                //   │     first borrow later used by call
                //   │     help: some help here
                // ```
                for (label_style, range, message) in
                    hanging_labels(single_labels, trailing_label).rev()
                {
                    self.outer_gutter(outer_padding)?;
                    self.border_left()?;
                    self.inner_gutter(severity, num_multi_labels, multi_labels)?;
                    write!(self, " ")?;
                    self.caret_pointers(
                        severity,
                        max_label_start,
                        single_labels,
                        trailing_label,
                        source
                            .char_indices()
                            .take_while(|(byte_index, _)| *byte_index < range.start),
                    )?;
                    self.set_label(severity, *label_style)?;
                    write!(self, "{message}",)?;
                    self.reset()?;
                    writeln!(self)?;
                }
            }
        }

        // Write top or bottom label carets underneath source
        //
        // ```text
        //     │ ╰───│──────────────────^ woops
        //     │   ╭─│─────────^
        // ```
        for (multi_label_index, (_, label_style, label)) in multi_labels.iter().enumerate() {
            let (label_style, range, bottom_message) = match label {
                MultiLabel::Left => continue, // no label caret needed
                // no label caret needed if this can be started in front of the line
                MultiLabel::Top(start) if *start <= source.len() - source.trim_start().len() => {
                    continue
                }
                MultiLabel::Top(range) => (*label_style, range, None),
                MultiLabel::Bottom(range, message) => (*label_style, range, Some(message)),
            };

            self.outer_gutter(outer_padding)?;
            self.border_left()?;

            // Write inner gutter.
            //
            // ```text
            //  │ ╭─│───│
            // ```
            let mut underline = None;
            let mut multi_labels_iter = multi_labels.iter().enumerate().peekable();
            for label_column in 0..num_multi_labels {
                match multi_labels_iter.peek() {
                    Some((i, (label_index, ls, label))) if *label_index == label_column => {
                        match label {
                            MultiLabel::Left => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Top(..) if multi_label_index > *i => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Bottom(..) if multi_label_index < *i => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Top(..) if multi_label_index == *i => {
                                underline = Some((*ls, VerticalBound::Top));
                                self.label_multi_top_left(severity, label_style)?;
                            }
                            MultiLabel::Bottom(..) if multi_label_index == *i => {
                                underline = Some((*ls, VerticalBound::Bottom));
                                self.label_multi_bottom_left(severity, label_style)?;
                            }
                            MultiLabel::Top(..) | MultiLabel::Bottom(..) => {
                                self.inner_gutter_column(severity, underline)?;
                            }
                        }
                        multi_labels_iter.next();
                    }
                    Some((_, _)) | None => self.inner_gutter_column(severity, underline)?,
                }
            }

            // Finish the top or bottom caret
            match bottom_message {
                None => self.label_multi_top_caret(severity, label_style, source, *range)?,
                Some(message) => {
                    self.label_multi_bottom_caret(severity, label_style, source, *range, message)?;
                }
            }
        }

        Ok(())
    }

    /// An empty source line, for providing additional whitespace to source snippets.
    ///
    /// ```text
    /// │ │ │
    /// ```
    pub fn render_snippet_empty(
        &mut self,
        outer_padding: usize,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        self.outer_gutter(outer_padding)?;
        self.border_left()?;
        self.inner_gutter(severity, num_multi_labels, multi_labels)?;
        writeln!(self)?;
        Ok(())
    }

    /// A broken source line, for labeling skipped sections of source.
    ///
    /// ```text
    /// · │ │
    /// ```
    pub fn render_snippet_break(
        &mut self,
        outer_padding: usize,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        self.outer_gutter(outer_padding)?;
        self.border_left_break()?;
        self.inner_gutter(severity, num_multi_labels, multi_labels)?;
        writeln!(self)?;
        Ok(())
    }

    /// Additional notes.
    ///
    /// ```text
    /// = expected type `Int`
    ///      found type `String`
    /// ```
    pub fn render_snippet_note(
        &mut self,
        outer_padding: usize,
        message: &str,
    ) -> Result<(), Error> {
        for (note_line_index, line) in message.lines().enumerate() {
            self.outer_gutter(outer_padding)?;
            match note_line_index {
                0 => {
                    self.set_note_bullet()?;
                    write!(self, "{}", self.chars().note_bullet)?;
                    self.reset()?;
                }
                _ => write!(self, " ")?,
            }
            // Write line of message
            writeln!(self, " {line}",)?;
        }

        Ok(())
    }

    /// Adds tab-stop aware unicode-width computations to an iterator over
    /// character indices. Assumes that the character indices begin at the start
    /// of the line.
    fn char_metrics(
        &self,
        char_indices: impl Iterator<Item = (usize, char)>,
    ) -> impl Iterator<Item = (Metrics, char)> {
        use unicode_width::UnicodeWidthChar;

        let tab_width = self.config.tab_width;
        let mut unicode_column = 0;

        char_indices.map(move |(byte_index, ch)| {
            let metrics = Metrics {
                byte_index,
                unicode_width: match (ch, tab_width) {
                    ('\t', 0) => 0, // Guard divide-by-zero
                    ('\t', _) => tab_width - (unicode_column % tab_width),
                    (ch, _) => ch.width().unwrap_or(0),
                },
            };
            unicode_column += metrics.unicode_width;

            (metrics, ch)
        })
    }

    /// Location focus.
    fn snippet_locus(&mut self, locus: &Locus) -> Result<(), Error> {
        write!(
            self,
            "{name}:{line_number}:{column_number}",
            name = locus.name,
            line_number = locus.location.line_number,
            column_number = locus.location.column_number,
        )?;
        Ok(())
    }

    /// The outer gutter of a source line.
    fn outer_gutter(&mut self, outer_padding: usize) -> Result<(), Error> {
        write!(self, "{space: >width$} ", space = "", width = outer_padding)?;
        Ok(())
    }

    /// The outer gutter of a source line, with line number.
    fn outer_gutter_number(
        &mut self,
        line_number: usize,
        outer_padding: usize,
    ) -> Result<(), Error> {
        self.set_line_number()?;
        write!(self, "{line_number: >outer_padding$}",)?;
        self.reset()?;
        write!(self, " ")?;
        Ok(())
    }

    /// The left-hand border of a source line.
    fn border_left(&mut self) -> Result<(), Error> {
        self.set_source_border()?;
        write!(self, "{}", self.chars().source_border_left)?;
        self.reset()?;
        Ok(())
    }

    /// The broken left-hand border of a source line.
    fn border_left_break(&mut self) -> Result<(), Error> {
        self.set_source_border()?;
        write!(self, "{}", self.chars().source_border_left_break)?;
        self.reset()?;
        Ok(())
    }

    /// Write vertical lines pointing to carets.
    fn caret_pointers(
        &mut self,
        severity: Severity,
        max_label_start: usize,
        single_labels: &[SingleLabel<'_>],
        trailing_label: Option<(usize, &SingleLabel<'_>)>,
        char_indices: impl Iterator<Item = (usize, char)>,
    ) -> Result<(), Error> {
        for (metrics, ch) in self.char_metrics(char_indices) {
            let column_range = metrics.byte_index..(metrics.byte_index + ch.len_utf8());
            let label_style = hanging_labels(single_labels, trailing_label)
                .filter(|(_, range, _)| column_range.contains(&range.start))
                .map(|(label_style, _, _)| label_style.clone())
                .max_by_key(label_priority_key);

            let mut spaces = match label_style {
                None => 0..metrics.unicode_width,
                Some(label_style) => {
                    self.set_label(severity, label_style)?;
                    write!(self, "{}", self.chars().pointer_left)?;
                    self.reset()?;
                    1..metrics.unicode_width
                }
            };
            // Only print padding if we are before the end of the last single line caret
            if metrics.byte_index <= max_label_start {
                spaces.try_for_each(|_| write!(self, " "))?;
            }
        }

        Ok(())
    }

    /// The left of a multi-line label.
    ///
    /// ```text
    ///  │
    /// ```
    fn label_multi_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        underline: Option<LabelStyle>,
    ) -> Result<(), Error> {
        match underline {
            None => write!(self, " ")?,
            // Continue an underline horizontally
            Some(label_style) => {
                self.set_label(severity, label_style)?;
                write!(self, "{}", self.chars().multi_top)?;
                self.reset()?;
            }
        }
        self.set_label(severity, label_style)?;
        write!(self, "{}", self.chars().multi_left)?;
        self.reset()?;
        Ok(())
    }

    /// The top-left of a multi-line label.
    ///
    /// ```text
    ///  ╭
    /// ```
    fn label_multi_top_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
    ) -> Result<(), Error> {
        write!(self, " ")?;
        self.set_label(severity, label_style)?;
        write!(self, "{}", self.chars().multi_top_left)?;
        self.reset()?;
        Ok(())
    }

    /// The bottom left of a multi-line label.
    ///
    /// ```text
    ///  ╰
    /// ```
    fn label_multi_bottom_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
    ) -> Result<(), Error> {
        write!(self, " ")?;
        self.set_label(severity, label_style)?;
        write!(self, "{}", self.chars().multi_bottom_left)?;
        self.reset()?;
        Ok(())
    }

    /// Multi-line label top.
    ///
    /// ```text
    /// ─────────────^
    /// ```
    fn label_multi_top_caret(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        source: &str,
        start: usize,
    ) -> Result<(), Error> {
        self.set_label(severity, label_style)?;

        for (metrics, _) in self
            .char_metrics(source.char_indices())
            .take_while(|(metrics, _)| metrics.byte_index < start + 1)
        {
            // FIXME: improve rendering of carets between character boundaries
            (0..metrics.unicode_width)
                .try_for_each(|_| write!(self, "{}", self.chars().multi_top))?;
        }

        let caret_start = match label_style {
            LabelStyle::Primary => self.config.chars.multi_primary_caret_start,
            LabelStyle::Secondary => self.config.chars.multi_secondary_caret_start,
        };
        write!(self, "{caret_start}",)?;
        self.reset()?;
        writeln!(self)?;
        Ok(())
    }

    /// Multi-line label bottom, with a message.
    ///
    /// ```text
    /// ─────────────^ expected `Int` but found `String`
    /// ```
    fn label_multi_bottom_caret(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        source: &str,
        start: usize,
        message: &str,
    ) -> Result<(), Error> {
        self.set_label(severity, label_style)?;

        for (metrics, _) in self
            .char_metrics(source.char_indices())
            .take_while(|(metrics, _)| metrics.byte_index < start)
        {
            // FIXME: improve rendering of carets between character boundaries
            (0..metrics.unicode_width)
                .try_for_each(|_| write!(self, "{}", self.chars().multi_bottom))?;
        }

        let caret_end = match label_style {
            LabelStyle::Primary => self.config.chars.multi_primary_caret_start,
            LabelStyle::Secondary => self.config.chars.multi_secondary_caret_start,
        };
        write!(self, "{caret_end}")?;
        if !message.is_empty() {
            write!(self, " {message}")?;
        }
        self.reset()?;
        writeln!(self)?;
        Ok(())
    }

    /// Writes an empty gutter space, or continues an underline horizontally.
    fn inner_gutter_column(
        &mut self,
        severity: Severity,
        underline: Option<Underline>,
    ) -> Result<(), Error> {
        match underline {
            None => self.inner_gutter_space(),
            Some((label_style, vertical_bound)) => {
                self.set_label(severity, label_style)?;
                let ch = match vertical_bound {
                    VerticalBound::Top => self.config.chars.multi_top,
                    VerticalBound::Bottom => self.config.chars.multi_bottom,
                };
                write!(self, "{ch}{ch}")?;
                self.reset()?;
                Ok(())
            }
        }
    }

    /// Writes an empty gutter space.
    fn inner_gutter_space(&mut self) -> Result<(), Error> {
        write!(self, "  ")?;
        Ok(())
    }

    /// Writes an inner gutter, with the left lines if necessary.
    fn inner_gutter(
        &mut self,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        let mut multi_labels_iter = multi_labels.iter().peekable();
        for label_column in 0..num_multi_labels {
            match multi_labels_iter.peek() {
                Some((label_index, ls, label)) if *label_index == label_column => match label {
                    MultiLabel::Left | MultiLabel::Bottom(..) => {
                        self.label_multi_left(severity, *ls, None)?;
                        multi_labels_iter.next();
                    }
                    MultiLabel::Top(..) => {
                        self.inner_gutter_space()?;
                        multi_labels_iter.next();
                    }
                },
                Some((_, _, _)) | None => self.inner_gutter_space()?,
            }
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Renderer<'_, '_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

#[cfg(not(feature = "std"))]
impl core::fmt::Write for Renderer<'_, '_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.writer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.writer.write_char(c)
    }

    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
        self.writer.write_fmt(args)
    }
}

impl WriteStyle for Renderer<'_, '_> {
    fn set_header(&mut self, severity: Severity) -> GeneralWriteResult {
        self.writer.set_header(severity)
    }

    fn set_header_message(&mut self) -> GeneralWriteResult {
        self.writer.set_header_message()
    }

    fn set_line_number(&mut self) -> GeneralWriteResult {
        self.writer.set_line_number()
    }

    fn set_note_bullet(&mut self) -> GeneralWriteResult {
        self.writer.set_note_bullet()
    }

    fn set_source_border(&mut self) -> GeneralWriteResult {
        self.writer.set_source_border()
    }

    fn set_label(&mut self, severity: Severity, label_style: LabelStyle) -> GeneralWriteResult {
        self.writer.set_label(severity, label_style)
    }
    fn reset(&mut self) -> GeneralWriteResult {
        self.writer.reset()
    }
}

struct Metrics {
    byte_index: usize,
    unicode_width: usize,
}

/// Check if two ranges overlap
fn is_overlapping(range0: &Range<usize>, range1: &Range<usize>) -> bool {
    let start = core::cmp::max(range0.start, range1.start);
    let end = core::cmp::min(range0.end, range1.end);
    start < end
}

/// For prioritizing primary labels over secondary labels when rendering carets.
fn label_priority_key(label_style: &LabelStyle) -> u8 {
    match label_style {
        LabelStyle::Secondary => 0,
        LabelStyle::Primary => 1,
    }
}

/// Return an iterator that yields the labels that require hanging messages
/// rendered underneath them.
fn hanging_labels<'labels, 'diagnostic>(
    single_labels: &'labels [SingleLabel<'diagnostic>],
    trailing_label: Option<(usize, &'labels SingleLabel<'diagnostic>)>,
) -> impl 'labels + DoubleEndedIterator<Item = &'labels SingleLabel<'diagnostic>> {
    single_labels
        .iter()
        .enumerate()
        .filter(|(_, (_, _, message))| !message.is_empty())
        .filter(move |(i, _)| trailing_label.map_or(true, |(j, _)| *i != j))
        .map(|(_, label)| label)
}
