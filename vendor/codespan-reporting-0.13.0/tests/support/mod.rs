use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::Files;
use codespan_reporting::term::{self, Config};

#[cfg(feature = "termcolor")]
mod color_buffer;

pub struct TestData<'files, F: Files<'files>> {
    pub files: F,
    pub diagnostics: Vec<Diagnostic<F::FileId>>,
}

impl<'files, F: Files<'files>> TestData<'files, F> {
    #[cfg(feature = "termcolor")]
    pub fn emit_color(&'files self, config: &Config) -> String {
        let mut writer = color_buffer::ColorBuffer::new();
        for diagnostic in &self.diagnostics {
            term::emit_to_write_style(&mut writer, config, &self.files, diagnostic).unwrap();
        }
        writer.into_string()
    }

    pub fn emit_no_color(&'files self, config: &Config) -> String {
        let mut writer = String::new();
        for diagnostic in &self.diagnostics {
            term::emit_to_string(&mut writer, config, &self.files, diagnostic).unwrap();
        }
        writer
    }
}
