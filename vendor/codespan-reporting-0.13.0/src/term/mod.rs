//! Terminal back-end for emitting diagnostics.

use crate::diagnostic::Diagnostic;
use crate::files::Files;

mod config;
mod renderer;
mod views;

#[cfg(not(feature = "std"))]
use alloc::string::String;

pub use config::{Chars, Config, DisplayStyle};

// re-export
#[cfg(feature = "termcolor")]
pub use config::styles::{termcolor, Styles, StylesWriter};

pub use renderer::{GeneralWrite, GeneralWriteResult, Renderer, WriteStyle};
pub use views::{RichDiagnostic, ShortDiagnostic};

pub fn emit_into_string<'files, F: Files<'files> + ?Sized>(
    config: &Config,
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<String, super::files::Error> {
    let mut writer = String::new();
    emit_to_string(&mut writer, config, files, diagnostic)?;
    Ok(writer)
}

pub fn emit_to_string<'files, F: Files<'files> + ?Sized>(
    writer: &mut String,
    config: &Config,
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<(), super::files::Error> {
    // std::io::Write used under `feature = "std"`
    #[cfg(feature = "std")]
    {
        let mut buffer = Vec::new();
        emit_with_style(
            &mut renderer::PlainWriter::new(&mut buffer),
            config,
            files,
            diagnostic,
        )?;
        let buffer_str: &str = str::from_utf8(&buffer).expect("buffer not utf8");
        writer.push_str(buffer_str);
        Ok(())
    }

    // core::fmt::Write used not `not(feature = "std")`
    #[cfg(not(feature = "std"))]
    {
        emit_with_style(
            &mut renderer::PlainWriter::new(writer),
            config,
            files,
            diagnostic,
        )
    }
}

#[cfg(feature = "std")]
pub fn emit_to_io_write<'files, F: Files<'files> + ?Sized, W: std::io::Write>(
    writer: &mut W,
    config: &Config,
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<(), super::files::Error> {
    emit_with_style(
        &mut renderer::PlainWriter::new(writer),
        config,
        files,
        diagnostic,
    )
}

pub fn emit_to_write_style<'files, F: Files<'files> + ?Sized, W: WriteStyle>(
    writer: &mut W,
    config: &Config,
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<(), super::files::Error> {
    emit_with_style(writer, config, files, diagnostic)
}

#[deprecated(
    since = "0.13.0",
    note = "Use `emit_to_write_style` instead or depending on the writer use `emit_to_io_write` or `emit_to_string`"
)]
/// Emit a diagnostic using the given writer, context, config, and files.
///
/// The return value covers all error cases. These error case can arise if:
/// * a file was removed from the file database.
/// * a file was changed so that it is too small to have an index
/// * IO fails
pub fn emit<'files, F: Files<'files> + ?Sized, W: renderer::GeneralWrite>(
    writer: &mut W,
    config: &Config,
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<(), super::files::Error> {
    emit_with_style(
        &mut renderer::PlainWriter::new(writer),
        config,
        files,
        diagnostic,
    )
}

fn emit_with_style<'files, F: Files<'files> + ?Sized, W: WriteStyle>(
    writer: &mut W,
    config: &Config,
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<(), super::files::Error> {
    let mut renderer = Renderer::new(writer, config);
    match config.display_style {
        DisplayStyle::Rich => RichDiagnostic::new(diagnostic, config).render(files, &mut renderer),
        DisplayStyle::Medium => ShortDiagnostic::new(diagnostic, true).render(files, &mut renderer),
        DisplayStyle::Short => ShortDiagnostic::new(diagnostic, false).render(files, &mut renderer),
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::*;

    use crate::diagnostic::Label;
    use crate::files::SimpleFiles;

    /// Test range of 0 to 0 and check does not crash
    #[test]
    fn unsized_emit() {
        let mut files = SimpleFiles::new();

        let id = files.add("test", "");
        let zero_range = 0..0;
        let diagnostic = Diagnostic::bug().with_label(Label::primary(id, zero_range));
        emit_into_string(&Config::default(), &files, &diagnostic).unwrap();
    }
}
