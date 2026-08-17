//! x86-specific (also x86-64) definitions of architecture-specific functions in
//! Wasmtime.

#[inline]
pub fn get_stack_pointer() -> usize {
    let stack_pointer: usize;
    unsafe {
        core::arch::asm!(
            "mov {}, rsp",
            out(reg) stack_pointer,
            options(nostack,nomem),
        );
    }
    stack_pointer
}

pub unsafe fn get_next_older_pc_from_fp(fp: usize) -> usize {
    // The calling convention always pushes the return pointer (aka the PC of
    // the next older frame) just before this frame.
    unsafe { *(fp as *mut usize).offset(1) }
}

pub unsafe fn resume_to_exception_handler(
    pc: usize,
    sp: usize,
    fp: usize,
    payload1: usize,
    payload2: usize,
) -> ! {
    unsafe {
        core::arch::asm!(
            "mov rsp, {}",
            "mov rbp, {}",
            "jmp {}",
            in(reg) sp,
            in(reg) fp,
            in(reg) pc,
            in("rax") payload1,
            in("rdx") payload2,
            options(nostack, nomem, noreturn),
        );
    }
}

// And the current frame pointer points to the next older frame pointer.
pub const NEXT_OLDER_FP_FROM_FP_OFFSET: usize = 0;

// SP of caller is FP in callee plus size of FP/return address pair.
pub const NEXT_OLDER_SP_FROM_FP_OFFSET: usize = 16;

/// Frame pointers are aligned if they're aligned to twice the size of a
/// pointer.
pub fn assert_fp_is_aligned(fp: usize) {
    let align = 2 * size_of::<usize>();
    assert_eq!(fp % align, 0, "stack should always be aligned to {align}");
}
