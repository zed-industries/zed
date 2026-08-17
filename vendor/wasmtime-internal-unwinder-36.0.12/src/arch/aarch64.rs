//! Arm64-specific definitions of architecture-specific functions in Wasmtime.

#[inline]
pub fn get_stack_pointer() -> usize {
    let stack_pointer: usize;
    unsafe {
        core::arch::asm!(
            "mov {}, sp",
            out(reg) stack_pointer,
            options(nostack,nomem),
        );
    }
    stack_pointer
}

// The aarch64 calling conventions save the return PC one i64 above the FP and
// the previous FP is pointed to by the current FP:
//
// > Each frame shall link to the frame of its caller by means of a frame record
// > of two 64-bit values on the stack [...] The frame record for the innermost
// > frame [...] shall be pointed to by the frame pointer register (FP). The
// > lowest addressed double-word shall point to the previous frame record and the
// > highest addressed double-word shall contain the value passed in LR on entry
// > to the current function.
//
// - AAPCS64 section 6.2.3 The Frame Pointer[0]
pub unsafe fn get_next_older_pc_from_fp(fp: usize) -> usize {
    unsafe {
        let mut pc = *(fp as *mut usize).offset(1);

        // The return address might be signed, so we need to strip the highest bits
        // (where the authentication code might be located) in order to obtain a
        // valid address. We use the `XPACLRI` instruction, which is executed as a
        // no-op by processors that do not support pointer authentication, so that
        // the implementation is backward-compatible and there is no duplication.
        // However, this instruction requires the LR register for both its input and
        // output.
        core::arch::asm!(
            "mov lr, {pc}",
            "xpaclri",
            "mov {pc}, lr",
            pc = inout(reg) pc,
            out("lr") _,
            options(nomem, nostack, preserves_flags, pure),
        );

        pc
    }
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
            "mov sp, {}",
            "mov fp, {}",
            "br {}",
            in(reg) sp,
            in(reg) fp,
            in(reg) pc,
            in("x0") payload1,
            in("x1") payload2,
            options(nostack, nomem, noreturn),
        );
    }
}

// And the current frame pointer points to the next older frame pointer.
pub const NEXT_OLDER_FP_FROM_FP_OFFSET: usize = 0;

// SP of caller is FP in callee plus size of FP/return address pair.
pub const NEXT_OLDER_SP_FROM_FP_OFFSET: usize = 16;

pub fn assert_fp_is_aligned(_fp: usize) {
    // From AAPCS64, section 6.2.3 The Frame Pointer[0]:
    //
    // > The location of the frame record within a stack frame is not specified.
    //
    // So this presumably means that the FP can have any alignment, as its
    // location is not specified and nothing further is said about constraining
    // alignment.
    //
    // [0]: https://github.com/ARM-software/abi-aa/blob/2022Q1/aapcs64/aapcs64.rst#the-frame-pointer
}
