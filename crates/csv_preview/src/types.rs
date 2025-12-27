use std::fmt::Debug;

pub use coordinates::*;
mod coordinates;
pub use table_row::*;
mod table_row;
pub use table_cell::*;
pub mod data_table;
mod table_cell;
pub use table_like_content::*;
mod table_like_content;

/// Line number information for CSV rows
#[derive(Debug, Clone, Copy)]
pub enum LineNumber {
    /// Single line row
    Line(usize),
    /// Multi-line row spanning from start to end line. Incluisive
    LineRange(usize, usize),
}

pub trait ResultExt<T, E: Debug> {
    /// Syntactic sugar for `.unwrap_or_else(|e|panic!("{msg}: {e:?}"))`
    fn expect_lazy(self, f: impl FnOnce() -> String) -> T;
}
impl<T, E: Debug> ResultExt<T, E> for Result<T, E> {
    fn expect_lazy(self, f: impl FnOnce() -> String) -> T {
        self.unwrap_or_else(|e| panic!("{}: {e:?}", f()))
    }
}

pub trait OptionExt<T> {
    /// Syntactic sugar for `.unwrap_or_else(|| panic!("{msg}"))`
    fn expect_lazy(self, f: impl FnOnce() -> String) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    fn expect_lazy(self, f: impl FnOnce() -> String) -> T {
        self.unwrap_or_else(|| panic!("{}", f()))
    }
}
