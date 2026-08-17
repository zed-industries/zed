//! This crate provides utilities for instruction cache maintenance for JIT authors.
//!
//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.
//!
//! In self modifying codes such as when writing a JIT, special care must be taken when marking the
//! code as ready for execution. On fully coherent architectures (X86, S390X) the data cache (D-Cache)
//! and the instruction cache (I-Cache) are always in sync. However this is not guaranteed for all
//! architectures such as AArch64 where these caches are not coherent with each other.
//!
//! When writing new code there may be a I-cache entry for that same address which causes the
//! processor to execute whatever was in the cache instead of the new code.
//!
//! See the [ARM Community - Caches and Self-Modifying Code] blog post that contains a great
//! explanation of the above. (It references AArch32 but it has a high level overview of this problem).
//!
//! ## Usage
//!
//! You should call [clear_cache] on any pages that you write with the new code that you're intending
//! to execute. You can do this at any point in the code from the moment that you write the page up to
//! the moment where the code is executed.
//!
//! You also need to call [pipeline_flush_mt] to ensure that there isn't any invalid instruction currently
//! in the pipeline if you are running in a multi threaded environment.
//!
//! For single threaded programs you are free to omit [pipeline_flush_mt], otherwise you need to
//! call both [clear_cache] and [pipeline_flush_mt] in that order.
//!
//! ### Example:
//! ```
//! # use std::ffi::c_void;
//! # use std::io;
//! # use wasmtime_internal_jit_icache_coherence::*;
//! #
//! # struct Page {
//! #   addr: *const c_void,
//! #   len: usize,
//! # }
//! #
//! # fn main() -> anyhow::Result<()> {
//! #
//! # let run_code = || {};
//! # let code = vec![0u8; 64];
//! # let newly_written_pages = vec![Page {
//! #    addr: &code[0] as *const u8 as *const c_void,
//! #    len: code.len(),
//! # }];
//! # unsafe {
//! // Invalidate the cache for all the newly written pages where we wrote our new code.
//! for page in newly_written_pages {
//!     clear_cache(page.addr, page.len)?;
//! }
//!
//! // Once those are invalidated we also need to flush the pipeline
//! pipeline_flush_mt()?;
//!
//! // We can now safely execute our new code.
//! run_code();
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
//!
//!  **Warning**: In order to correctly use this interface you should always call [clear_cache].
//!  A followup call to [pipeline_flush_mt] is required if you are running in a multi-threaded environment.
//!
//! </pre></div>
//!
//! [ARM Community - Caches and Self-Modifying Code]: https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/caches-and-self-modifying-code

use std::ffi::c_void;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod win;
        use win as imp;
    } else if #[cfg(miri)] {
        mod miri;
        use crate::miri as imp;
    } else {
        mod libc;
        use crate::libc as imp;
    }
}

/// Flushes instructions in the processor pipeline
///
/// This pipeline flush is broadcast to all processors that are executing threads in the current process.
///
/// Calling [pipeline_flush_mt] is only required for multi-threaded programs and it *must* be called
/// after all calls to [clear_cache].
///
/// If the architecture does not require a pipeline flush, this function does nothing.
pub fn pipeline_flush_mt() -> imp::Result<()> {
    imp::pipeline_flush_mt()
}

/// Flushes the instruction cache for a region of memory.
///
/// If the architecture does not require an instruction cache flush, this function does nothing.
///
/// # Unsafe
///
/// It is necessary to call [pipeline_flush_mt] after this function if you are running in a multi-threaded
/// environment.
pub unsafe fn clear_cache(ptr: *const c_void, len: usize) -> imp::Result<()> {
    imp::clear_cache(ptr, len)
}
