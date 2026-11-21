//! Infrastructure shared by modal editing modes (Vim and Helix).

mod registers;

pub use registers::{Register, Registers, read_from_primary, write_to_primary};
