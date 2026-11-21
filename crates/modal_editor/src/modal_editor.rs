//! Infrastructure shared by modal editing modes (Vim and Helix).

mod jump_labels;
mod mode_indicator;
mod registers;

pub use jump_labels::{
    HELIX_JUMP_LABEL_LIMIT, HELIX_JUMP_OVERLAY_KEY, HelixJumpLabel, HelixJumpUiData,
    collect_helix_jump_data,
};
#[cfg(any(test, feature = "test-support"))]
pub use jump_labels::{HelixJumpSkipData, build_helix_jump_ui_data, selection_skip_offsets};
pub use mode_indicator::{
    ModeIndicator, ModeIndicatorFocused, ModeIndicatorSource, ModeStatus,
    register_mode_indicator_source,
};
pub use registers::{Register, Registers, read_from_primary, write_to_primary};
