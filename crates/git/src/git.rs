use std::ffi::OsStr;

pub use git2 as libgit;
pub use lazy_static::lazy_static;

pub mod diff;

lazy_static! {
    pub static ref DOT_GIT: &'static OsStr = OsStr::new(".git");
    pub static ref GITIGNORE: &'static OsStr = OsStr::new(".gitignore");
}
