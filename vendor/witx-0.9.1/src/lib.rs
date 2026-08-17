/// Map witx types to core (wasm standard) types
mod abi;
/// Types describing a validated witx document
mod ast;
/// Render documentation
mod docs;
/// Interface for filesystem or mock IO
mod io;
/// Calculate memory layout of types
mod layout;
/// Witx syntax parsing from SExprs
pub mod parser;
/// Paths to witx documents for various proposal phases
pub mod phases;
/// Calculate required polyfill between interfaces
pub mod polyfill;
/// Render ast to text
mod render;
/// Representational equality of types
mod representation;
/// Resolve toplevel `use` declarations across files
mod toplevel;
/// Validate declarations into ast
mod validate;

pub use abi::*;
pub use ast::*;
pub use docs::Documentation;
pub use io::{Filesystem, MockFs, WitxIo};
pub use layout::{Layout, RecordMemberLayout, SizeAlign};
pub use render::SExpr;
pub use representation::{RepEquality, Representable};
pub use validate::{DocValidation, ValidationError};

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Load a witx document from the filesystem
pub fn load<P: AsRef<Path>>(paths: &[P]) -> Result<Document, WitxError> {
    toplevel::parse_witx(paths)
}

/// Parse a witx document from a str. `(use ...)` directives are not permitted.
pub fn parse(source: &str) -> Result<Document, WitxError> {
    let mockfs = MockFs::new(&[("-", source)]);
    toplevel::parse_witx_with(&[Path::new("-")], &mockfs)
}

/// Location in the source text
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Location {
    pub path: PathBuf,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Error)]
pub enum WitxError {
    #[error("IO error with file {0:?}")]
    Io(PathBuf, #[source] ::std::io::Error),
    #[error("Parse error")]
    Parse(#[from] wast::Error),
    #[error("Validation error")]
    Validation(#[from] ValidationError),
}

impl WitxError {
    pub fn report_with(&self, witxio: &dyn WitxIo) -> String {
        use WitxError::*;
        match self {
            Io(path, ioerr) => format!("with file {:?}: {}", path, ioerr),
            Parse(parse) => parse.to_string(),
            Validation(validation) => validation.report_with(witxio),
        }
    }
    pub fn report(&self) -> String {
        self.report_with(&Filesystem)
    }
}

impl Location {
    pub fn highlight_source_with(&self, witxio: &dyn WitxIo) -> String {
        let mut msg = format!("in {:?}:\n", self.path);
        if let Ok(src_line) = witxio.fget_line(&self.path, self.line) {
            msg += &format!(
                "{line_num: >5} | {src_line}\n{blank: >5}    {caret: >column$}",
                line_num = self.line,
                src_line = src_line,
                blank = " ",
                caret = "^",
                column = self.column,
            );
        }
        msg
    }
    pub fn highlight_source(&self) -> String {
        self.highlight_source_with(&Filesystem)
    }
}
