#![allow(missing_docs)]

use crate::prelude::*;

pub struct UnwindRegistration {}

impl UnwindRegistration {
    pub const SECTION_NAME: &'static str = ".eh_frame";

    pub unsafe fn new(
        _base_address: *const u8,
        _unwind_info: *const u8,
        _unwind_len: usize,
    ) -> Result<UnwindRegistration> {
        Ok(UnwindRegistration {})
    }
}
