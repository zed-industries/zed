use crate::cfg::CFG;
use crate::gen::fs;
use std::error::Error as StdError;
use std::ffi::OsString;
use std::fmt::{self, Display};
use std::path::Path;

pub(super) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub(super) enum Error {
    NoEnv(OsString),
    Fs(fs::Error),
    ExportedDirNotAbsolute(&'static Path),
    ExportedEmptyPrefix,
    ExportedDirsWithoutLinks,
    ExportedPrefixesWithoutLinks,
    ExportedLinksWithoutLinks,
    UnusedExportedPrefix(&'static str),
    UnusedExportedLinks(&'static str),
}

macro_rules! expr {
    ($expr:expr) => {{
        let _ = $expr; // ensure it doesn't fall out of sync with CFG definition
        stringify!($expr)
    }};
}

const LINKS_DOCUMENTATION: &str =
    "https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key";

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoEnv(var) => {
                write!(f, "missing {} environment variable", var.to_string_lossy())
            }
            Error::Fs(err) => err.fmt(f),
            Error::ExportedDirNotAbsolute(path) => write!(
                f,
                "element of {} must be absolute path, but was: `{}`",
                expr!(CFG.exported_header_dirs),
                path.display(),
            ),
            Error::ExportedEmptyPrefix => write!(
                f,
                "element of {} must not be empty string",
                expr!(CFG.exported_header_prefixes),
            ),
            Error::ExportedDirsWithoutLinks => write!(
                f,
                "if {} is nonempty then `links` needs to be set in Cargo.toml; see {}",
                expr!(CFG.exported_header_dirs),
                LINKS_DOCUMENTATION,
            ),
            Error::ExportedPrefixesWithoutLinks => write!(
                f,
                "if {} is nonempty then `links` needs to be set in Cargo.toml; see {}",
                expr!(CFG.exported_header_prefixes),
                LINKS_DOCUMENTATION,
            ),
            Error::ExportedLinksWithoutLinks => write!(
                f,
                "if {} is nonempty then `links` needs to be set in Cargo.toml; see {}",
                expr!(CFG.exported_header_links),
                LINKS_DOCUMENTATION,
            ),
            Error::UnusedExportedPrefix(unused) => write!(
                f,
                "unused element in {}: {:?} does not match the include prefix of any direct dependency",
                expr!(CFG.exported_header_prefixes),
                unused,
            ),
            Error::UnusedExportedLinks(unused) => write!(
                f,
                "unused element in {}: {:?} does not match the `links` attribute any direct dependency",
                expr!(CFG.exported_header_links),
                unused,
            ),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Fs(err) => err.source(),
            _ => None,
        }
    }
}

impl From<fs::Error> for Error {
    fn from(err: fs::Error) -> Self {
        Error::Fs(err)
    }
}
