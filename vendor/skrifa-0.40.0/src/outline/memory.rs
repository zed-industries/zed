//! Support for temporary memory allocation, making use of the stack for
//! small sizes.

/// Invokes the callback with a memory buffer of the requested size.
pub(super) fn with_temporary_memory<R>(size: usize, mut f: impl FnMut(&mut [u8]) -> R) -> R {
    // Wrap in a function and prevent inlining to avoid stack allocation
    // and zeroing if we don't take this code path.
    #[inline(never)]
    fn stack_mem<const STACK_SIZE: usize, R>(size: usize, mut f: impl FnMut(&mut [u8]) -> R) -> R {
        f(&mut [0u8; STACK_SIZE][..size])
    }
    // Use bucketed stack allocations (up to 16k) to prevent excessive zeroing
    // of memory
    if size <= 512 {
        stack_mem::<512, _>(size, f)
    } else if size <= 1024 {
        stack_mem::<1024, _>(size, f)
    } else if size <= 2048 {
        stack_mem::<2048, _>(size, f)
    } else if size <= 4096 {
        stack_mem::<4096, _>(size, f)
    } else if size <= 8192 {
        stack_mem::<8192, _>(size, f)
    } else if size <= 16384 {
        stack_mem::<16384, _>(size, f)
    } else {
        f(&mut vec![0u8; size])
    }
}
