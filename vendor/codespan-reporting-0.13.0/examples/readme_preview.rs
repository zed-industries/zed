//! Renders the preview SVG for the README.
//!
//! To update the preview, execute the following command from the top level of
//! the repository:
//!
//! ```sh
//! cargo run --example readme_preview svg > codespan-reporting/assets/readme_preview.svg
//! ```

use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::{self, Config};
use codespan_reporting::term::{GeneralWrite, GeneralWriteResult};

#[cfg(feature = "termcolor")]
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

#[cfg(not(feature = "std"))]
fn main() {
    panic!("example requires std feature");
}

#[cfg(feature = "std")]
fn main() -> anyhow::Result<()> {
    #[derive(Debug)]
    pub enum Opts {
        /// Render SVG output
        Svg,
        /// Render Stderr output
        #[cfg(feature = "termcolor")]
        Stderr {
            /// Configure coloring of output
            color: ColorChoice,
        },
    }

    fn parse_args() -> Result<Opts, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();
        match pargs.subcommand()? {
            Some(value) => match value.as_str() {
                "svg" => Ok(Opts::Svg),
                #[cfg(feature = "termcolor")]
                "stderr" => {
                    let color = pargs
                        .opt_value_from_str("--color")?
                        .unwrap_or(ColorChoice::Auto);
                    Ok(Opts::Stderr { color })
                }
                _ => Err(pico_args::Error::Utf8ArgumentParsingFailed {
                    value,
                    cause: "not a valid subcommand".to_owned(),
                }),
            },
            None => Err(pico_args::Error::MissingArgument),
        }
    }

    let file = SimpleFile::new(
        "FizzBuzz.fun",
        unindent::unindent(
            r#"
                module FizzBuzz where

                fizz₁ : Nat → String
                fizz₁ num = case (mod num 5) (mod num 3) of
                    0 0 => "FizzBuzz"
                    0 _ => "Fizz"
                    _ 0 => "Buzz"
                    _ _ => num

                fizz₂ : Nat → String
                fizz₂ num =
                    case (mod num 5) (mod num 3) of
                        0 0 => "FizzBuzz"
                        0 _ => "Fizz"
                        _ 0 => "Buzz"
                        _ _ => num
            "#,
        ),
    );

    let diagnostics = [Diagnostic::error()
        .with_message("`case` clauses have incompatible types")
        .with_code("E0308")
        .with_labels(vec![
            Label::primary((), 328..331).with_message("expected `String`, found `Nat`"),
            Label::secondary((), 211..331).with_message("`case` clauses have incompatible types"),
            Label::secondary((), 258..268).with_message("this is found to be of type `String`"),
            Label::secondary((), 284..290).with_message("this is found to be of type `String`"),
            Label::secondary((), 306..312).with_message("this is found to be of type `String`"),
            Label::secondary((), 186..192).with_message("expected type `String` found here"),
        ])
        .with_notes(vec![unindent::unindent(
            "
                expected type `String`
                   found type `Nat`
            ",
        )])];

    match parse_args()? {
        Opts::Svg => {
            let mut writer = SvgWriter::new();
            let config = Config::default();

            for diagnostic in &diagnostics {
                term::emit_to_write_style(&mut writer, &config, &file, diagnostic)?;
            }

            let num_lines = writer.line_count();

            let padding = 10;
            let font_size = 12;
            let line_spacing = 3;
            let width = 882;
            let height = padding + num_lines * (font_size + line_spacing) + padding;

            let content = writer.into_string();

            let stdout = std::io::stdout();
            let writer = &mut stdout.lock();

            write!(
                writer,
                r#"<svg viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">
  <style>
    /* https://github.com/aaron-williamson/base16-alacritty/blob/master/colors/base16-tomorrow-night-256.yml */
        pre {{
            background: #1d1f21;
            margin: 0;
            padding: {padding}px;
            border-radius: 6px;
            color: #ffffff;
            font: {font_size}px SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
        }}
        
        pre .bold {{
            font-weight: bold;
        }}
        
        pre .header-bug,
        pre .header-error {{
            color: #cc6666;
            font-weight: bold;
        }}
        
        pre .header-warning {{
            color: #f0c674;
            font-weight: bold;
        }}
        
        pre .header-note {{
            color: #b5bd68;
            font-weight: bold;
        }}
        
        pre .header-help {{
            color: #8abeb7;
            font-weight: bold;
        }}
        
        pre .header-message {{
            color: #c5c8c6;
            font-weight: bold;
        }}
        
        pre .line-number,
        pre .source-border,
        pre .note-bullet {{
            color: #81a2be;
        }}
        
        pre .label-primary-bug,
        pre .label-primary-error {{
            color: #cc6666;
        }}
        
        pre .label-primary-warning {{
            color: #f0c674;
        }}
        
        pre .label-primary-note {{
            color: #b5bd68;
        }}
        
        pre .label-primary-help {{
            color: #8abeb7;
        }}
        
        pre .label-secondary-bug,
        pre .label-secondary-error,
        pre .label-secondary-warning,
        pre .label-secondary-note,
        pre .label-secondary-help {{
            color: #81a2be;
        }}
  </style>

  <foreignObject x="0" y="0" width="{width}" height="{height}">
    <div xmlns="http://www.w3.org/1999/xhtml">
      <pre>
        {content}
      </pre>
    </div>
  </foreignObject>
</svg>"#
            )?;
        }
        #[cfg(feature = "termcolor")]
        Opts::Stderr { color } => {
            let writer = StandardStream::stderr(color);
            let config = Config::default();
            for diagnostic in &diagnostics {
                term::emit_to_write_style(&mut writer.lock(), &config, &file, diagnostic)?;
            }
        }
    }

    Ok(())
}

// This whole example requires the std feature, but below is feature agnostic for reference

#[cfg(feature = "std")]
type WriterBuffer = Vec<u8>;

#[cfg(not(feature = "std"))]
type WriterBuffer = String;

pub struct SvgWriter {
    buffer: WriterBuffer,
    span_open: bool,
}

impl SvgWriter {
    pub fn new() -> Self {
        SvgWriter {
            buffer: WriterBuffer::default(),
            span_open: false,
        }
    }

    #[cfg(feature = "std")]
    pub fn into_string(self) -> String {
        String::from_utf8(self.buffer).unwrap()
    }

    #[cfg(not(feature = "std"))]
    pub fn into_string(self) -> String {
        self.buffer
    }

    #[cfg(feature = "std")]
    pub fn line_count(&self) -> usize {
        self.buffer.iter().filter(|byte| **byte == b'\n').count() + 1
    }

    #[cfg(not(feature = "std"))]
    pub fn line_count(&self) -> usize {
        self.buffer.lines().count()
    }

    /// Close any open span
    fn close_span(&mut self) -> GeneralWriteResult {
        if self.span_open {
            write!(self.buffer, "</span>")?;
            self.span_open = false;
        }
        Ok(())
    }

    /// Open a new span with the given CSS class
    fn open_span(&mut self, class: &str) -> GeneralWriteResult {
        // close existing first
        self.close_span()?;
        write!(self.buffer, "<span class=\"{}\">", class)?;
        self.span_open = true;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::io::Write for SvgWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut last = 0;
        for (i, &b) in buf.iter().enumerate() {
            let escape = match b {
                b'<' => b"&lt;"[..].as_ref(),
                b'>' => b"&gt;"[..].as_ref(),
                b'&' => b"&amp;"[..].as_ref(),
                _ => continue,
            };
            self.buffer.write_all(&buf[last..i])?;
            self.buffer.write_all(escape)?;
            last = i + 1;
        }
        self.buffer.write_all(&buf[last..])?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

#[cfg(not(feature = "std"))]
impl core::fmt::Write for SvgWriter {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        let mut last = 0;
        // TODO match indices
        for (i, b) in s.chars().enumerate() {
            let escape = match b {
                '<' => "&lt;",
                '>' => "&gt;",
                '&' => "&amp;",
                _ => continue,
            };
            self.buffer.write_str(&s[last..i])?;
            self.buffer.write_str(escape)?;
            last = i + 1;
        }
        self.buffer.write_str(&s[last..])?;
        Ok(())
    }
}

impl codespan_reporting::term::WriteStyle for SvgWriter {
    fn set_header(&mut self, severity: Severity) -> GeneralWriteResult {
        let class = match severity {
            Severity::Bug => "header-bug",
            Severity::Error => "header-error",
            Severity::Warning => "header-warning",
            Severity::Note => "header-note",
            Severity::Help => "header-help",
        };
        self.open_span(class)
    }

    fn set_header_message(&mut self) -> GeneralWriteResult {
        self.open_span("header-message")
    }

    fn set_line_number(&mut self) -> GeneralWriteResult {
        self.open_span("line-number")
    }

    fn set_note_bullet(&mut self) -> GeneralWriteResult {
        self.open_span("note-bullet")
    }

    fn set_source_border(&mut self) -> GeneralWriteResult {
        self.open_span("source-border")
    }

    fn set_label(&mut self, severity: Severity, label_style: LabelStyle) -> GeneralWriteResult {
        let sev = match severity {
            Severity::Bug => "bug",
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
            Severity::Help => "help",
        };
        let typ = match label_style {
            LabelStyle::Primary => "primary",
            LabelStyle::Secondary => "secondary",
        };
        self.open_span(&format!("label-{}-{}", typ, sev))
    }

    fn reset(&mut self) -> GeneralWriteResult {
        self.close_span()
    }
}
