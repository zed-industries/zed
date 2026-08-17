pub use anyhow::Result;
use std::ffi::c_void;

pub(crate) fn pipeline_flush_mt() -> Result<()> {
    Ok(())
}

pub(crate) fn clear_cache(_ptr: *const c_void, _len: usize) -> Result<()> {
    Ok(())
}
