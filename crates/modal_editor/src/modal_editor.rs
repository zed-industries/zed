//! Infrastructure shared by modal editing modes (Vim and Helix).

mod mode_indicator;
mod registers;

pub use mode_indicator::{
    ModeIndicator, ModeIndicatorFocused, ModeIndicatorSource, ModeStatus,
    register_mode_indicator_source,
};
pub use registers::{Register, Registers, read_from_primary, write_to_primary};
