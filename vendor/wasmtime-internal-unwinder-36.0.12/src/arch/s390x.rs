//! s390x-specific definitions of architecture-specific functions in Wasmtime.

#[inline]
pub fn get_stack_pointer() -> usize {
    let mut sp;
    unsafe {
        core::arch::asm!(
            "lgr {}, %r15",
            out(reg) sp,
            options(nostack, nomem),
        );
    }
    sp
}

pub unsafe fn get_next_older_pc_from_fp(fp: usize) -> usize {
    // The next older PC can be found in register %r14 at function entry, which
    // was saved into slot 14 of the register save area pointed to by "FP" (the
    // backchain pointer).
    unsafe { *(fp as *mut usize).offset(14) }
}

pub unsafe fn resume_to_exception_handler(
    pc: usize,
    sp: usize,
    _fp: usize,
    payload1: usize,
    payload2: usize,
) -> ! {
    unsafe {
        core::arch::asm!(
            "lgr %r15, {}",
            "br {}",
            in(reg) sp,
            in(reg_addr) pc,
            in("r6") payload1,
            in("r7") payload2,
            options(nostack, nomem, noreturn),
        );
    }
}

// The next older "FP" (backchain pointer) was saved in the slot pointed to
// by the current "FP".
pub const NEXT_OLDER_FP_FROM_FP_OFFSET: usize = 0;

// SP of caller is "FP" (backchain pointer) in callee.
pub const NEXT_OLDER_SP_FROM_FP_OFFSET: usize = 0;

pub fn assert_fp_is_aligned(fp: usize) {
    assert_eq!(fp % 8, 0, "stack should always be aligned to 8");
}
