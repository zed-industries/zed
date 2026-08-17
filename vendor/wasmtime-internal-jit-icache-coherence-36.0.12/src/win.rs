use std::ffi::c_void;
use std::io::Error;
use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::Threading::FlushProcessWriteBuffers;
use windows_sys::Win32::System::Threading::GetCurrentProcess;

pub use std::io::Result;

/// See docs on [crate::pipeline_flush_mt] for a description of what this function is trying to do.
#[inline]
pub(crate) fn pipeline_flush_mt() -> Result<()> {
    // If we are here, it means that the user has already called [cache_clear] for all buffers that
    // are going to be holding code. We don't really care about flushing the write buffers, but
    // the other guarantee that microsoft provides on this API. As documented:
    //
    // "The function generates an interprocessor interrupt (IPI) to all processors that are part of
    // the current process affinity. It guarantees the visibility of write operations performed on
    // one processor to the other processors."
    //
    // This all-core IPI acts as a core serializing operation, equivalent to a "broadcast" `ISB`
    // instruction that the architecture does not provide and which is what we really want.
    //
    // See: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushprocesswritebuffers
    if cfg!(target_arch = "aarch64") {
        unsafe {
            FlushProcessWriteBuffers();
        }
    }

    Ok(())
}

/// See docs on [crate::clear_cache] for a description of what this function is trying to do.
#[inline]
pub(crate) fn clear_cache(ptr: *const c_void, len: usize) -> Result<()> {
    // See:
    //   * https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache
    //   * https://devblogs.microsoft.com/oldnewthing/20190902-00/?p=102828
    unsafe {
        let res = FlushInstructionCache(GetCurrentProcess(), ptr, len);
        if res == 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(())
}
