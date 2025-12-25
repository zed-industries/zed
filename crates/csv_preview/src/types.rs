pub use coordinates::*;
mod coordinates;
pub use table_row::*;
mod table_row;
pub use table_cell::*;
pub mod data_table;
mod table_cell;

/// Line number information for CSV rows
#[derive(Debug, Clone, Copy)]
pub enum LineNumber {
    /// Single line row
    Line(usize),
    /// Multi-line row spanning from start to end line. Incluisive
    LineRange(usize, usize),
}
